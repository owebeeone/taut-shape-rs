//! Latest-state single-writer mailbox (`atom.oracle/v1`).

use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};

use crate::generated_atom::{
    AtomCancelTimer, AtomClose, AtomDiagCode, AtomDiagnostic, AtomEndStream, AtomProducerStop,
    AtomReadRequest, AtomReadResponse, AtomReplace, AtomSeal, AtomSetTimer, AtomSeverity,
    AtomState, AtomStopReason, AtomTimerExpired, AtomValue, AtomVersion,
};
use crate::StopWhen;

#[derive(Clone, Debug, PartialEq)]
pub enum AtomInput {
    Replace(AtomReplace),
    Seal(AtomSeal),
    Close(AtomClose),
    Read(AtomReadRequest),
    EndStream(AtomEndStream),
    TimerExpired(AtomTimerExpired),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AtomOutput {
    ReadResponse(AtomReadResponse),
    SetTimer(AtomSetTimer),
    CancelTimer(AtomCancelTimer),
    ProducerStop(AtomProducerStop),
    Diagnostic(AtomDiagnostic),
}

#[derive(Clone)]
struct HeldRead {
    atom_id: String,
    version: i64,
    timeout_ms: Option<i64>,
    timer_token: Option<i64>,
}

/// One retained value with versioned reads, holds, timers, and lifecycle.
pub struct AtomNode {
    stop_when: StopWhen,
    version: i64,
    payload: Option<Vec<u8>>,
    sealed: bool,
    closed: bool,
    close_error: Option<crate::generated_atom::AtomError>,
    streams: BTreeMap<String, Option<HeldRead>>,
    order: Vec<String>,
    next_token: i64,
}

impl AtomNode {
    pub const fn new(stop_when: StopWhen) -> Self {
        Self {
            stop_when,
            version: 0,
            payload: None,
            sealed: false,
            closed: false,
            close_error: None,
            streams: BTreeMap::new(),
            order: Vec::new(),
            next_token: 1,
        }
    }

    pub fn handle(&mut self, input: AtomInput) -> Vec<AtomOutput> {
        match input {
            AtomInput::Replace(message) => self.replace(message),
            AtomInput::Seal(_) => self.seal(),
            AtomInput::Close(message) => self.close(message),
            AtomInput::Read(message) => self.read(message),
            AtomInput::EndStream(message) => self.end_stream(message),
            AtomInput::TimerExpired(message) => self.timer_expired(message),
        }
    }

    pub const fn version(&self) -> i64 {
        self.version
    }

    fn replace(&mut self, message: AtomReplace) -> Vec<AtomOutput> {
        if self.sealed || self.closed {
            return vec![AtomOutput::Diagnostic(AtomDiagnostic {
                severity: AtomSeverity::Warn,
                code: AtomDiagCode::ReplaceAfterTerminal,
            })];
        }
        self.version += 1;
        self.payload = Some(message.payload);
        self.answer_all_held()
    }

    fn seal(&mut self) -> Vec<AtomOutput> {
        if self.sealed {
            return Vec::new();
        }
        self.sealed = true;
        self.answer_all_held()
    }

    fn close(&mut self, message: AtomClose) -> Vec<AtomOutput> {
        if self.closed {
            return Vec::new();
        }
        self.closed = true;
        self.close_error = message.error;
        let mut outputs = self.answer_all_held();
        outputs.push(AtomOutput::ProducerStop(AtomProducerStop {
            reason: if self.close_error.is_some() {
                AtomStopReason::Failed
            } else {
                AtomStopReason::Closed
            },
        }));
        outputs
    }

    fn ensure_stream(&mut self, stream_id: &str) {
        if !self.streams.contains_key(stream_id) {
            self.streams.insert(stream_id.into(), None);
            self.order.push(stream_id.into());
        }
    }

    fn read(&mut self, message: AtomReadRequest) -> Vec<AtomOutput> {
        self.ensure_stream(&message.stream_id);
        let mut outputs = Vec::new();
        if let Some(Some(prior)) = self.streams.get(&message.stream_id) {
            if let Some(token) = prior.timer_token {
                outputs.push(AtomOutput::CancelTimer(AtomCancelTimer { token }));
            }
        }
        self.streams.insert(message.stream_id.clone(), None);
        let version = message.version.map_or(0, |value| value.version);
        if let Some(response) = self.resolve(
            &message.atom_id,
            &message.stream_id,
            version,
            message.timeout_ms,
        ) {
            outputs.push(AtomOutput::ReadResponse(response));
            return outputs;
        }
        let timer_token = message.timeout_ms.filter(|ms| *ms > 0).map(|ms| {
            let token = self.next_token;
            self.next_token += 1;
            outputs.push(AtomOutput::SetTimer(AtomSetTimer { token, ms }));
            token
        });
        self.streams.insert(
            message.stream_id,
            Some(HeldRead {
                atom_id: message.atom_id,
                version,
                timeout_ms: message.timeout_ms,
                timer_token,
            }),
        );
        outputs
    }

    fn resolve(
        &self,
        atom_id: &str,
        stream_id: &str,
        version: i64,
        timeout_ms: Option<i64>,
    ) -> Option<AtomReadResponse> {
        if version < self.version {
            return Some(AtomReadResponse {
                atom_id: atom_id.into(),
                stream_id: stream_id.into(),
                value: Some(AtomValue {
                    version: self.version,
                    payload: self
                        .payload
                        .as_ref()
                        .expect("atom version advanced without payload")
                        .clone(),
                }),
                next_version: AtomVersion {
                    version: self.version,
                },
                state: AtomState::Data,
                error: None,
            });
        }
        let state = if self.closed {
            Some(if self.close_error.is_some() {
                AtomState::Failed
            } else {
                AtomState::Closed
            })
        } else if self.sealed {
            Some(AtomState::Eof)
        } else if timeout_ms == Some(0) {
            Some(AtomState::WouldBlock)
        } else {
            None
        }?;
        Some(AtomReadResponse {
            atom_id: atom_id.into(),
            stream_id: stream_id.into(),
            value: None,
            next_version: AtomVersion {
                version: self.version,
            },
            state,
            error: if state == AtomState::Failed {
                self.close_error.clone()
            } else {
                None
            },
        })
    }

    fn answer_all_held(&mut self) -> Vec<AtomOutput> {
        let mut outputs = Vec::new();
        for stream_id in self.order.clone() {
            let Some(Some(held)) = self.streams.get(&stream_id).cloned() else {
                continue;
            };
            let Some(response) =
                self.resolve(&held.atom_id, &stream_id, held.version, held.timeout_ms)
            else {
                continue;
            };
            if let Some(token) = held.timer_token {
                outputs.push(AtomOutput::CancelTimer(AtomCancelTimer { token }));
            }
            outputs.push(AtomOutput::ReadResponse(response));
            self.streams.insert(stream_id, None);
        }
        outputs
    }

    fn end_stream(&mut self, message: AtomEndStream) -> Vec<AtomOutput> {
        let Some(held) = self.streams.remove(&message.stream_id) else {
            return Vec::new();
        };
        self.order
            .retain(|stream_id| stream_id != &message.stream_id);
        let mut outputs = Vec::new();
        if let Some(token) = held.and_then(|held| held.timer_token) {
            outputs.push(AtomOutput::CancelTimer(AtomCancelTimer { token }));
        }
        if self.streams.is_empty() && self.stop_when == StopWhen::LastReader {
            outputs.push(AtomOutput::ProducerStop(AtomProducerStop {
                reason: AtomStopReason::LastReaderGone,
            }));
        }
        outputs
    }

    fn timer_expired(&mut self, message: AtomTimerExpired) -> Vec<AtomOutput> {
        let found = self.streams.iter().find_map(|(stream_id, held)| {
            held.as_ref()
                .filter(|held| held.timer_token == Some(message.token))
                .map(|held| (stream_id.clone(), held.clone()))
        });
        let Some((stream_id, held)) = found else {
            return Vec::new();
        };
        self.streams.insert(stream_id.clone(), None);
        vec![AtomOutput::ReadResponse(
            self.resolve(&held.atom_id, &stream_id, held.version, Some(0))
                .expect("atom probe unexpectedly held"),
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    fn held(stream_id: String) -> AtomInput {
        AtomInput::Read(AtomReadRequest {
            atom_id: "atom-A".into(),
            stream_id,
            version: Some(AtomVersion { version: 0 }),
            timeout_ms: None,
        })
    }

    #[test]
    fn replacement_releases_a_large_held_set_in_creation_order() {
        let mut node = AtomNode::new(StopWhen::LastReader);
        for index in 0..2_000 {
            assert!(node.handle(held(format!("s{index:04}"))).is_empty());
        }
        let outputs = node.handle(AtomInput::Replace(AtomReplace {
            payload: b"ready".to_vec(),
        }));
        assert_eq!(outputs.len(), 2_000);
        for (index, output) in outputs.iter().enumerate() {
            let AtomOutput::ReadResponse(response) = output else {
                panic!("expected response");
            };
            assert_eq!(response.stream_id, format!("s{index:04}"));
        }
    }
}
