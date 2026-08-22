//! Payload-agnostic causal replicated-operation engine (`crdt.oracle/v1`).

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec,
    vec::Vec,
};

use crate::generated_crdt::{
    CrdtApply, CrdtBootstrap, CrdtClock, CrdtClockEntry, CrdtClose, CrdtDiagCode, CrdtDiagnostic,
    CrdtError, CrdtInstallBootstrap, CrdtOp, CrdtReadRequest, CrdtReadResponse, CrdtSeal,
    CrdtSeverity, CrdtState,
};

pub mod text;

#[derive(Clone, Debug, PartialEq)]
pub enum CrdtInput {
    Apply(CrdtApply),
    InstallBootstrap(CrdtInstallBootstrap),
    Seal(CrdtSeal),
    Close(CrdtClose),
    Read(CrdtReadRequest),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CrdtOutput {
    ReadResponse(CrdtReadResponse),
    Diagnostic(CrdtDiagnostic),
}

fn clock_map(clock: &CrdtClock) -> Option<BTreeMap<String, i64>> {
    let mut result = BTreeMap::new();
    let mut previous: Option<&str> = None;
    for entry in &clock.entries {
        if entry.origin.is_empty()
            || entry.seq <= 0
            || previous.is_some_and(|origin| entry.origin.as_str() <= origin)
        {
            return None;
        }
        previous = Some(&entry.origin);
        result.insert(entry.origin.clone(), entry.seq);
    }
    Some(result)
}

fn clock_value(clock: &BTreeMap<String, i64>) -> CrdtClock {
    CrdtClock {
        entries: clock
            .iter()
            .filter(|(_, seq)| **seq > 0)
            .map(|(origin, seq)| CrdtClockEntry {
                origin: origin.clone(),
                seq: *seq,
            })
            .collect(),
    }
}

fn key(op: &CrdtOp) -> (String, i64) {
    (op.origin.clone(), op.seq)
}

fn variant(op: &CrdtOp) -> (Vec<(String, i64)>, Vec<u8>) {
    (
        clock_map(&op.deps)
            .expect("validated operation")
            .into_iter()
            .collect(),
        op.payload.clone(),
    )
}

/// Immediate anti-entropy mailbox with bounded causal buffering.
pub struct CrdtNode {
    max_pending: usize,
    clock: BTreeMap<String, i64>,
    floor: BTreeMap<String, i64>,
    bootstrap: Option<CrdtBootstrap>,
    ops: BTreeMap<(String, i64), CrdtOp>,
    integrated: BTreeSet<(String, i64)>,
    equivocated: BTreeSet<(String, i64)>,
    sealed: bool,
    closed: bool,
    error: Option<CrdtError>,
}

impl Default for CrdtNode {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl CrdtNode {
    pub const fn new(max_pending: usize) -> Self {
        Self {
            max_pending,
            clock: BTreeMap::new(),
            floor: BTreeMap::new(),
            bootstrap: None,
            ops: BTreeMap::new(),
            integrated: BTreeSet::new(),
            equivocated: BTreeSet::new(),
            sealed: false,
            closed: false,
            error: None,
        }
    }

    pub fn clock(&self) -> CrdtClock {
        clock_value(&self.clock)
    }

    pub fn bootstrap(&self) -> Option<&CrdtBootstrap> {
        self.bootstrap.as_ref()
    }

    pub fn operations(&self) -> Vec<CrdtOp> {
        self.integrated
            .iter()
            .filter_map(|key| self.ops.get(key).cloned())
            .collect()
    }

    pub fn equivocations(&self) -> Vec<(String, i64)> {
        self.equivocated.iter().cloned().collect()
    }

    pub fn handle(&mut self, input: CrdtInput) -> Vec<CrdtOutput> {
        match input {
            CrdtInput::Apply(message) => self.apply(message.op),
            CrdtInput::InstallBootstrap(message) => self.install_bootstrap(message.bootstrap),
            CrdtInput::Seal(_) => {
                if !self.closed {
                    self.sealed = true;
                }
                Vec::new()
            }
            CrdtInput::Close(message) => {
                if !self.closed {
                    self.closed = true;
                    self.error = message.error;
                }
                Vec::new()
            }
            CrdtInput::Read(request) => vec![CrdtOutput::ReadResponse(self.read(request))],
        }
    }

    fn diagnostic(code: CrdtDiagCode, op: Option<&CrdtOp>) -> CrdtOutput {
        CrdtOutput::Diagnostic(CrdtDiagnostic {
            severity: if code == CrdtDiagCode::ApplyAfterTerminal {
                CrdtSeverity::Warn
            } else {
                CrdtSeverity::Error
            },
            code,
            origin: op.map(|value| value.origin.clone()),
            seq: op.map(|value| value.seq),
        })
    }

    fn valid(op: &CrdtOp) -> bool {
        let Some(deps) = clock_map(&op.deps) else {
            return false;
        };
        !op.origin.is_empty() && op.seq > 0 && deps.get(&op.origin).copied().unwrap_or(0) < op.seq
    }

    fn apply(&mut self, op: CrdtOp) -> Vec<CrdtOutput> {
        if self.sealed || self.closed {
            return vec![Self::diagnostic(
                CrdtDiagCode::ApplyAfterTerminal,
                Some(&op),
            )];
        }
        if !Self::valid(&op) {
            return vec![Self::diagnostic(CrdtDiagCode::InvalidOperation, Some(&op))];
        }
        let op_key = key(&op);
        if op.seq <= self.floor.get(&op.origin).copied().unwrap_or(0) {
            return Vec::new();
        }
        if let Some(previous) = self.ops.get(&op_key) {
            if previous == &op {
                return Vec::new();
            }
            let mut outputs = Vec::new();
            if self.equivocated.insert(op_key.clone()) {
                outputs.push(Self::diagnostic(CrdtDiagCode::Equivocation, Some(&op)));
            }
            if variant(&op) < variant(previous) {
                self.ops.insert(op_key, op);
            }
            self.drain();
            return outputs;
        }
        if !self.ready(&op) && self.ops.len() - self.integrated.len() >= self.max_pending {
            return vec![Self::diagnostic(
                CrdtDiagCode::PendingBoundExceeded,
                Some(&op),
            )];
        }
        self.ops.insert(op_key, op);
        self.drain();
        Vec::new()
    }

    fn ready(&self, op: &CrdtOp) -> bool {
        if op.seq > self.clock.get(&op.origin).copied().unwrap_or(0) + 1 {
            return false;
        }
        clock_map(&op.deps)
            .expect("validated operation")
            .into_iter()
            .all(|(origin, seq)| self.clock.get(&origin).copied().unwrap_or(0) >= seq)
    }

    fn drain(&mut self) {
        loop {
            let ready: Vec<(String, i64)> = self
                .ops
                .iter()
                .filter(|(op_key, op)| !self.integrated.contains(*op_key) && self.ready(op))
                .map(|(op_key, _)| op_key.clone())
                .collect();
            if ready.is_empty() {
                return;
            }
            for op_key in ready {
                let op = &self.ops[&op_key];
                self.integrated.insert(op_key);
                self.clock
                    .entry(op.origin.clone())
                    .and_modify(|seq| *seq = (*seq).max(op.seq))
                    .or_insert(op.seq);
            }
        }
    }

    fn install_bootstrap(&mut self, bootstrap: CrdtBootstrap) -> Vec<CrdtOutput> {
        if self.sealed || self.closed {
            return vec![Self::diagnostic(CrdtDiagCode::ApplyAfterTerminal, None)];
        }
        let Some(parsed) = clock_map(&bootstrap.clock) else {
            return vec![Self::diagnostic(CrdtDiagCode::InvalidOperation, None)];
        };
        if self.bootstrap.as_ref() == Some(&bootstrap) {
            return Vec::new();
        }
        if self.bootstrap.is_some() || !self.ops.is_empty() {
            return vec![Self::diagnostic(CrdtDiagCode::BootstrapConflict, None)];
        }
        self.floor = parsed.clone();
        self.clock = parsed;
        self.bootstrap = Some(bootstrap);
        Vec::new()
    }

    fn read(&self, request: CrdtReadRequest) -> CrdtReadResponse {
        let cursor = match &request.cursor {
            None => BTreeMap::new(),
            Some(value) => match clock_map(value) {
                Some(value) => value,
                None => {
                    return self.response(
                        &request,
                        CrdtState::InvalidCursor,
                        None,
                        Vec::new(),
                        None,
                    )
                }
            },
        };
        if cursor
            .iter()
            .any(|(origin, seq)| *seq > self.clock.get(origin).copied().unwrap_or(0))
        {
            return self.response(&request, CrdtState::InvalidCursor, None, Vec::new(), None);
        }
        let below_floor = self
            .floor
            .iter()
            .any(|(origin, seq)| cursor.get(origin).copied().unwrap_or(0) < *seq);
        let (bootstrap, base, state) = if request.cursor.is_some() && below_floor {
            (
                self.bootstrap.clone(),
                &self.floor,
                CrdtState::BootstrapRequired,
            )
        } else {
            (
                if request.cursor.is_none() {
                    self.bootstrap.clone()
                } else {
                    None
                },
                &cursor,
                CrdtState::Data,
            )
        };
        let ops: Vec<CrdtOp> = self
            .integrated
            .iter()
            .filter(|(origin, seq)| *seq > base.get(origin).copied().unwrap_or(0))
            .filter_map(|op_key| self.ops.get(op_key).cloned())
            .collect();
        if bootstrap.is_some() || !ops.is_empty() {
            return self.response(&request, state, bootstrap, ops, None);
        }
        if self.closed {
            return self.response(
                &request,
                if self.error.is_some() {
                    CrdtState::Failed
                } else {
                    CrdtState::Closed
                },
                None,
                Vec::new(),
                self.error.clone(),
            );
        }
        if self.sealed {
            return self.response(&request, CrdtState::Eof, None, Vec::new(), None);
        }
        self.response(&request, CrdtState::Empty, None, Vec::new(), None)
    }

    fn response(
        &self,
        request: &CrdtReadRequest,
        state: CrdtState,
        bootstrap: Option<CrdtBootstrap>,
        ops: Vec<CrdtOp>,
        error: Option<CrdtError>,
    ) -> CrdtReadResponse {
        CrdtReadResponse {
            crdt_id: request.crdt_id.clone(),
            stream_id: request.stream_id.clone(),
            bootstrap,
            ops,
            next_cursor: self.clock(),
            state,
            error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn op(origin: &str, seq: i64, deps: &[(&str, i64)], payload: &[u8]) -> CrdtInput {
        CrdtInput::Apply(CrdtApply {
            op: CrdtOp {
                origin: origin.into(),
                seq,
                deps: CrdtClock {
                    entries: deps
                        .iter()
                        .map(|(origin, seq)| CrdtClockEntry {
                            origin: (*origin).into(),
                            seq: *seq,
                        })
                        .collect(),
                },
                payload: payload.to_vec(),
            },
        })
    }

    #[test]
    fn reverse_chain_drains_iteratively() {
        let mut node = CrdtNode::new(2000);
        for seq in (1..=2000).rev() {
            node.handle(op("a", seq, &[], b"x"));
        }
        assert_eq!(
            node.clock().entries,
            vec![CrdtClockEntry {
                origin: "a".into(),
                seq: 2000
            }]
        );
    }

    #[test]
    fn permutations_converge_and_equivocation_has_a_stable_winner() {
        let inputs = [
            op("a", 1, &[], b"B"),
            op("a", 1, &[], b"A"),
            op("b", 1, &[], b"C"),
        ];
        let mut left = CrdtNode::default();
        let mut right = CrdtNode::default();
        for index in [0, 1, 2] {
            left.handle(inputs[index].clone());
        }
        for index in [2, 1, 0] {
            right.handle(inputs[index].clone());
        }
        assert_eq!(left.clock(), right.clock());
        assert_eq!(left.operations(), right.operations());
        assert_eq!(left.equivocations(), right.equivocations());
    }
}
