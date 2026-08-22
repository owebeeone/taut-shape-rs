//! Bounded disposable ordered delivery (`stream.oracle/v1`).

use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};

use crate::generated_stream::{
    StreamCancelTimer, StreamClose, StreamDiagCode, StreamDiagnostic, StreamEndStream, StreamError,
    StreamErrorCode, StreamPosition, StreamProducerStop, StreamPush, StreamReadRequest,
    StreamReadResponse, StreamRecord, StreamSeal, StreamSetTimer, StreamSeverity, StreamState,
    StreamStopReason, StreamTimerExpired,
};
use crate::StopWhen;

#[derive(Clone, Debug, PartialEq)]
pub enum StreamInput {
    Push(StreamPush),
    Seal(StreamSeal),
    Close(StreamClose),
    Read(StreamReadRequest),
    EndStream(StreamEndStream),
    TimerExpired(StreamTimerExpired),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StreamOutput {
    ReadResponse(StreamReadResponse),
    SetTimer(StreamSetTimer),
    CancelTimer(StreamCancelTimer),
    ProducerStop(StreamProducerStop),
    Diagnostic(StreamDiagnostic),
}

#[derive(Clone)]
struct HeldRead {
    max_records: Option<i64>,
    max_bytes: Option<i64>,
    timeout_ms: Option<i64>,
    timer_token: Option<i64>,
}

#[derive(Clone)]
struct Reader {
    cursor: i64,
    held: Option<HeldRead>,
}

/// A live-only bounded ring with node-owned reader positions.
pub struct StreamNode {
    capacity_records: usize,
    stop_when: StopWhen,
    head: i64,
    payloads: BTreeMap<i64, Vec<u8>>,
    sealed: bool,
    closed: bool,
    close_error: Option<StreamError>,
    readers: BTreeMap<String, Reader>,
    order: Vec<String>,
    next_token: i64,
}

impl StreamNode {
    pub fn new(capacity_records: usize, stop_when: StopWhen) -> Self {
        assert!(capacity_records > 0, "capacity_records must be positive");
        Self {
            capacity_records,
            stop_when,
            head: 0,
            payloads: BTreeMap::new(),
            sealed: false,
            closed: false,
            close_error: None,
            readers: BTreeMap::new(),
            order: Vec::new(),
            next_token: 1,
        }
    }

    pub const fn head(&self) -> i64 {
        self.head
    }

    pub fn floor(&self) -> i64 {
        self.payloads
            .first_key_value()
            .map_or(self.head + 1, |(seq, _)| *seq)
    }

    pub fn handle(&mut self, input: StreamInput) -> Vec<StreamOutput> {
        match input {
            StreamInput::Push(message) => self.push(message),
            StreamInput::Seal(_) => self.seal(),
            StreamInput::Close(message) => self.close(message),
            StreamInput::Read(message) => self.read(message),
            StreamInput::EndStream(message) => self.end_stream(message),
            StreamInput::TimerExpired(message) => self.timer_expired(message),
        }
    }

    fn push(&mut self, message: StreamPush) -> Vec<StreamOutput> {
        if self.sealed || self.closed {
            return vec![StreamOutput::Diagnostic(StreamDiagnostic {
                severity: StreamSeverity::Warn,
                code: StreamDiagCode::PushAfterTerminal,
            })];
        }
        self.head += 1;
        self.payloads.insert(self.head, message.payload);
        if self.payloads.len() > self.capacity_records {
            self.payloads
                .remove(&(self.head - self.capacity_records as i64));
        }
        self.answer_all_held()
    }

    fn seal(&mut self) -> Vec<StreamOutput> {
        if self.sealed || self.closed {
            return Vec::new();
        }
        self.sealed = true;
        self.answer_all_held()
    }

    fn close(&mut self, message: StreamClose) -> Vec<StreamOutput> {
        if self.closed {
            return Vec::new();
        }
        self.closed = true;
        self.close_error = message.error;
        let mut outputs = self.answer_all_held();
        outputs.push(StreamOutput::ProducerStop(StreamProducerStop {
            reason: if self.close_error.is_some() {
                StreamStopReason::Failed
            } else {
                StreamStopReason::Closed
            },
        }));
        outputs
    }

    fn ensure_reader(&mut self, stream_id: &str) {
        if !self.readers.contains_key(stream_id) {
            self.readers.insert(
                stream_id.into(),
                Reader {
                    cursor: self.head,
                    held: None,
                },
            );
            self.order.push(stream_id.into());
        }
    }

    fn read(&mut self, message: StreamReadRequest) -> Vec<StreamOutput> {
        self.ensure_reader(&message.stream_id);
        let mut outputs = Vec::new();
        if let Some(token) = self
            .readers
            .get(&message.stream_id)
            .and_then(|reader| reader.held.as_ref())
            .and_then(|held| held.timer_token)
        {
            outputs.push(StreamOutput::CancelTimer(StreamCancelTimer { token }));
        }
        let mut reader = self
            .readers
            .remove(&message.stream_id)
            .expect("stream reader was just ensured");
        reader.held = None;
        if let Some(response) = self.resolve(
            &message.stream_id,
            &mut reader,
            message.max_records,
            message.max_bytes,
            message.timeout_ms,
        ) {
            let dropped = response.state == StreamState::Dropped;
            outputs.push(StreamOutput::ReadResponse(response));
            if dropped {
                self.order
                    .retain(|stream_id| stream_id != &message.stream_id);
                outputs.extend(self.stop_if_last_reader());
            } else {
                self.readers.insert(message.stream_id, reader);
            }
            return outputs;
        }
        let timer_token = message.timeout_ms.filter(|ms| *ms > 0).map(|ms| {
            let token = self.next_token;
            self.next_token += 1;
            outputs.push(StreamOutput::SetTimer(StreamSetTimer { token, ms }));
            token
        });
        reader.held = Some(HeldRead {
            max_records: message.max_records,
            max_bytes: message.max_bytes,
            timeout_ms: message.timeout_ms,
            timer_token,
        });
        self.readers.insert(message.stream_id, reader);
        outputs
    }

    fn resolve(
        &self,
        stream_id: &str,
        reader: &mut Reader,
        max_records: Option<i64>,
        max_bytes: Option<i64>,
        timeout_ms: Option<i64>,
    ) -> Option<StreamReadResponse> {
        if reader.cursor < self.floor() - 1 {
            return Some(Self::response(
                stream_id,
                Vec::new(),
                self.head,
                StreamState::Dropped,
                Some(StreamError {
                    code: StreamErrorCode::SlowConsumer,
                    message: None,
                }),
            ));
        }
        let mut records = Vec::new();
        let mut used = 0_i64;
        for (&seq, payload) in self.payloads.range((reader.cursor + 1)..) {
            if max_records.is_some_and(|limit| limit > 0 && records.len() as i64 >= limit) {
                break;
            }
            let payload_size = payload.len() as i64;
            if max_bytes.is_some_and(|limit| {
                limit >= 0 && !records.is_empty() && used + payload_size > limit
            }) {
                break;
            }
            records.push(StreamRecord {
                seq,
                payload: payload.clone(),
            });
            used += payload_size;
        }
        if let Some(last) = records.last() {
            reader.cursor = last.seq;
            return Some(Self::response(
                stream_id,
                records,
                reader.cursor,
                StreamState::Data,
                None,
            ));
        }
        let (state, error) = if self.closed {
            if self.close_error.is_some() {
                (StreamState::Failed, self.close_error.clone())
            } else {
                (StreamState::Closed, None)
            }
        } else if self.sealed {
            (StreamState::Eof, None)
        } else if timeout_ms == Some(0) {
            (StreamState::WouldBlock, None)
        } else {
            return None;
        };
        Some(Self::response(
            stream_id,
            Vec::new(),
            reader.cursor,
            state,
            error,
        ))
    }

    fn response(
        stream_id: &str,
        records: Vec<StreamRecord>,
        position: i64,
        state: StreamState,
        error: Option<StreamError>,
    ) -> StreamReadResponse {
        StreamReadResponse {
            stream_id: stream_id.into(),
            records,
            next_position: StreamPosition { seq: position },
            state,
            error,
        }
    }

    fn answer_all_held(&mut self) -> Vec<StreamOutput> {
        let mut outputs = Vec::new();
        for stream_id in self.order.clone() {
            let Some(mut reader) = self.readers.remove(&stream_id) else {
                continue;
            };
            let Some(held) = reader.held.take() else {
                self.readers.insert(stream_id, reader);
                continue;
            };
            let Some(response) = self.resolve(
                &stream_id,
                &mut reader,
                held.max_records,
                held.max_bytes,
                held.timeout_ms,
            ) else {
                reader.held = Some(held);
                self.readers.insert(stream_id, reader);
                continue;
            };
            if let Some(token) = held.timer_token {
                outputs.push(StreamOutput::CancelTimer(StreamCancelTimer { token }));
            }
            let dropped = response.state == StreamState::Dropped;
            outputs.push(StreamOutput::ReadResponse(response));
            if dropped {
                self.order.retain(|candidate| candidate != &stream_id);
                outputs.extend(self.stop_if_last_reader());
            } else {
                self.readers.insert(stream_id, reader);
            }
        }
        outputs
    }

    fn remove_reader(&mut self, stream_id: &str) -> Vec<StreamOutput> {
        if self.readers.remove(stream_id).is_none() {
            return Vec::new();
        }
        self.order.retain(|candidate| candidate != stream_id);
        self.stop_if_last_reader()
    }

    fn stop_if_last_reader(&self) -> Vec<StreamOutput> {
        if self.readers.is_empty() && self.stop_when == StopWhen::LastReader {
            vec![StreamOutput::ProducerStop(StreamProducerStop {
                reason: StreamStopReason::LastReaderGone,
            })]
        } else {
            Vec::new()
        }
    }

    fn end_stream(&mut self, message: StreamEndStream) -> Vec<StreamOutput> {
        let Some(reader) = self.readers.get(&message.stream_id) else {
            return Vec::new();
        };
        let mut outputs = Vec::new();
        if let Some(token) = reader.held.as_ref().and_then(|held| held.timer_token) {
            outputs.push(StreamOutput::CancelTimer(StreamCancelTimer { token }));
        }
        outputs.extend(self.remove_reader(&message.stream_id));
        outputs
    }

    fn timer_expired(&mut self, message: StreamTimerExpired) -> Vec<StreamOutput> {
        let found = self.order.iter().find_map(|stream_id| {
            self.readers
                .get(stream_id)
                .and_then(|reader| reader.held.as_ref())
                .filter(|held| held.timer_token == Some(message.token))
                .map(|held| (stream_id.clone(), held.clone()))
        });
        let Some((stream_id, held)) = found else {
            return Vec::new();
        };
        let mut reader = self
            .readers
            .remove(&stream_id)
            .expect("timer belonged to an active reader");
        reader.held = None;
        let response = self
            .resolve(
                &stream_id,
                &mut reader,
                held.max_records,
                held.max_bytes,
                Some(0),
            )
            .expect("stream probe unexpectedly held");
        self.readers.insert(stream_id, reader);
        vec![StreamOutput::ReadResponse(response)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    fn held(stream_id: String) -> StreamInput {
        StreamInput::Read(StreamReadRequest {
            stream_id,
            max_records: None,
            max_bytes: None,
            timeout_ms: None,
        })
    }

    #[test]
    #[should_panic(expected = "capacity_records must be positive")]
    fn zero_capacity_is_rejected() {
        let _ = StreamNode::new(0, StopWhen::LastReader);
    }

    #[test]
    fn push_releases_a_large_held_set_in_creation_order() {
        let mut node = StreamNode::new(64, StopWhen::LastReader);
        for index in 0..2_000 {
            assert!(node.handle(held(format!("s{index:04}"))).is_empty());
        }
        let outputs = node.handle(StreamInput::Push(StreamPush {
            payload: b"ready".to_vec(),
        }));
        assert_eq!(outputs.len(), 2_000);
        for (index, output) in outputs.iter().enumerate() {
            let StreamOutput::ReadResponse(response) = output else {
                panic!("expected response");
            };
            assert_eq!(response.stream_id, format!("s{index:04}"));
        }
    }
}
