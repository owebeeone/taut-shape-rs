//! Attributed multi-writer LWW whole-value register (`value.oracle/v0`).

use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};

use crate::generated_value::{
    ValueDiagCode, ValueDiagnostic, ValueReadRequest, ValueReadResponse, ValueSet, ValueSeverity,
    ValueStamp, ValueState,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ValueInput {
    Set(ValueSet),
    Read(ValueReadRequest),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueOutput {
    ReadResponse(ValueReadResponse),
    Diagnostic(ValueDiagnostic),
}

/// Pure op-set fold. Exact re-delivery is idempotent; a forked identity emits
/// one diagnostic and cannot mutate the accepted set.
#[derive(Default)]
pub struct ValueNode {
    ops: BTreeMap<(String, i64), ValueSet>,
}

impl ValueNode {
    pub const fn new() -> Self {
        Self {
            ops: BTreeMap::new(),
        }
    }

    pub fn handle(&mut self, input: ValueInput) -> Vec<ValueOutput> {
        match input {
            ValueInput::Set(op) => self.set(op),
            ValueInput::Read(request) => vec![ValueOutput::ReadResponse(self.read(request))],
        }
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    fn set(&mut self, op: ValueSet) -> Vec<ValueOutput> {
        let key = (op.origin.clone(), op.seq);
        if let Some(prior) = self.ops.get(&key) {
            if prior.payload != op.payload || prior.prev != op.prev {
                return vec![ValueOutput::Diagnostic(ValueDiagnostic {
                    severity: ValueSeverity::Error,
                    code: ValueDiagCode::Equivocation,
                })];
            }
            return Vec::new();
        }
        self.ops.insert(key, op);
        Vec::new()
    }

    fn read(&self, request: ValueReadRequest) -> ValueReadResponse {
        let winner = self.ops.values().max_by(|left, right| {
            left.lamport
                .cmp(&right.lamport)
                .then_with(|| left.origin.cmp(&right.origin))
                .then_with(|| left.seq.cmp(&right.seq))
        });
        match winner {
            None => ValueReadResponse {
                value_id: request.value_id,
                stream_id: request.stream_id,
                value: None,
                winner: None,
                state: ValueState::Empty,
            },
            Some(op) => ValueReadResponse {
                value_id: request.value_id,
                stream_id: request.stream_id,
                value: Some(op.payload.clone()),
                winner: Some(ValueStamp {
                    origin: op.origin.clone(),
                    seq: op.seq,
                    lamport: op.lamport,
                }),
                state: ValueState::Data,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(origin: &str, seq: i64, lamport: i64, payload: &[u8]) -> ValueInput {
        ValueInput::Set(ValueSet {
            origin: origin.into(),
            seq,
            lamport,
            prev: None,
            payload: payload.to_vec(),
        })
    }

    fn read(node: &mut ValueNode) -> ValueReadResponse {
        match node
            .handle(ValueInput::Read(ValueReadRequest {
                value_id: "v-A".into(),
                stream_id: "s1".into(),
            }))
            .pop()
            .unwrap()
        {
            ValueOutput::ReadResponse(response) => response,
            ValueOutput::Diagnostic(_) => panic!("read returned a diagnostic"),
        }
    }

    #[test]
    fn lww_uses_lamport_then_origin() {
        let mut node = ValueNode::new();
        node.handle(set("z", 1, 1, b"old"));
        node.handle(set("a", 1, 2, b"new-a"));
        node.handle(set("b", 1, 2, b"new-b"));
        assert_eq!(read(&mut node).value.as_deref(), Some(&b"new-b"[..]));
    }

    #[test]
    fn reused_same_origin_lamport_uses_sequence() {
        let mut node = ValueNode::new();
        node.handle(set("a", 1, 5, b"old"));
        node.handle(set("a", 2, 5, b"new"));
        assert_eq!(read(&mut node).value.as_deref(), Some(&b"new"[..]));
    }

    #[test]
    fn duplicate_is_idempotent_and_equivocation_does_not_mutate() {
        let mut node = ValueNode::new();
        let original = set("a", 1, 1, b"ok");
        assert!(node.handle(original.clone()).is_empty());
        assert!(node.handle(original).is_empty());
        let diagnostic = node.handle(set("a", 1, 99, b"fork"));
        assert!(matches!(
            diagnostic.as_slice(),
            [ValueOutput::Diagnostic(_)]
        ));
        assert_eq!(node.len(), 1);
        assert_eq!(read(&mut node).value.as_deref(), Some(&b"ok"[..]));
    }
}
