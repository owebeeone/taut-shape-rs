//! Single-writer snapshot plus retained deltas (`swmr.oracle/v1`).

use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};

use crate::generated_swmr::{
    SwmrCancelTimer, SwmrClose, SwmrCursor, SwmrDelta, SwmrDeltaPush, SwmrDiagCode, SwmrDiagnostic,
    SwmrEndStream, SwmrError, SwmrProducerStop, SwmrReadRequest, SwmrReadResponse, SwmrReset,
    SwmrResetReason, SwmrSeal, SwmrSetTimer, SwmrSeverity, SwmrSnapshot, SwmrSnapshotPush,
    SwmrState, SwmrStopReason, SwmrTimerExpired,
};
use crate::StopWhen;

#[derive(Clone, Debug, PartialEq)]
pub enum SwmrInput {
    SnapshotPush(SwmrSnapshotPush),
    DeltaPush(SwmrDeltaPush),
    Reset(SwmrReset),
    Seal(SwmrSeal),
    Close(SwmrClose),
    Read(SwmrReadRequest),
    EndStream(SwmrEndStream),
    TimerExpired(SwmrTimerExpired),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SwmrOutput {
    ReadResponse(SwmrReadResponse),
    SetTimer(SwmrSetTimer),
    CancelTimer(SwmrCancelTimer),
    ProducerStop(SwmrProducerStop),
    Diagnostic(SwmrDiagnostic),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwmrRecoveryPolicy {
    Repair,
    Expire,
}

#[derive(Clone)]
struct HeldRead {
    swmr_id: String,
    cursor: Option<SwmrCursor>,
    timeout_ms: Option<i64>,
    timer_token: Option<i64>,
}

/// Snapshot/delta store with durable reset epochs and disposable readers.
pub struct SwmrNode {
    stop_when: StopWhen,
    max_deltas: Option<usize>,
    recovery_policy: SwmrRecoveryPolicy,
    snapshot: Option<SwmrSnapshot>,
    deltas: Vec<SwmrDelta>,
    sealed: bool,
    closed: bool,
    close_error: Option<SwmrError>,
    writer_id: Option<String>,
    epoch: i64,
    reset_reason: Option<SwmrResetReason>,
    reset_detail: Option<Vec<u8>>,
    streams: BTreeMap<String, Option<HeldRead>>,
    order: Vec<String>,
    next_token: i64,
}

impl SwmrNode {
    pub const fn new(stop_when: StopWhen, max_deltas: Option<usize>) -> Self {
        Self::with_recovery(stop_when, max_deltas, SwmrRecoveryPolicy::Repair)
    }

    pub const fn with_recovery(
        stop_when: StopWhen,
        max_deltas: Option<usize>,
        recovery_policy: SwmrRecoveryPolicy,
    ) -> Self {
        Self {
            stop_when,
            max_deltas,
            recovery_policy,
            snapshot: None,
            deltas: Vec::new(),
            sealed: false,
            closed: false,
            close_error: None,
            writer_id: None,
            epoch: 0,
            reset_reason: None,
            reset_detail: None,
            streams: BTreeMap::new(),
            order: Vec::new(),
            next_token: 1,
        }
    }

    pub const fn recovery_policy(&self) -> SwmrRecoveryPolicy {
        self.recovery_policy
    }

    pub fn handle(&mut self, input: SwmrInput) -> Vec<SwmrOutput> {
        match input {
            SwmrInput::SnapshotPush(message) => self.snapshot_push(message),
            SwmrInput::DeltaPush(message) => self.delta_push(message),
            SwmrInput::Reset(message) => self.reset(message),
            SwmrInput::Seal(_) => self.seal(),
            SwmrInput::Close(message) => self.close(message),
            SwmrInput::Read(message) => self.read(message),
            SwmrInput::EndStream(message) => self.end_stream(message),
            SwmrInput::TimerExpired(message) => self.timer_expired(message),
        }
    }

    pub fn head(&self) -> i64 {
        self.snapshot
            .as_ref()
            .map_or(0, |snapshot| snapshot.seq + self.deltas.len() as i64)
    }

    pub const fn epoch(&self) -> i64 {
        self.epoch
    }

    const fn terminal(&self) -> bool {
        self.sealed || self.closed
    }

    fn check_writer(&mut self, writer_id: &str) -> bool {
        match &self.writer_id {
            Some(bound) => bound == writer_id,
            None => {
                self.writer_id = Some(writer_id.into());
                true
            }
        }
    }

    fn diagnostic(code: SwmrDiagCode) -> Vec<SwmrOutput> {
        vec![SwmrOutput::Diagnostic(SwmrDiagnostic {
            severity: if code == SwmrDiagCode::PushAfterTerminal {
                SwmrSeverity::Warn
            } else {
                SwmrSeverity::Error
            },
            code,
        })]
    }

    fn snapshot_push(&mut self, message: SwmrSnapshotPush) -> Vec<SwmrOutput> {
        if self.terminal() {
            return Self::diagnostic(SwmrDiagCode::PushAfterTerminal);
        }
        if !self.check_writer(&message.writer_id) {
            return Self::diagnostic(SwmrDiagCode::WriterConflict);
        }
        self.snapshot = Some(SwmrSnapshot {
            seq: self.head(),
            payload: message.payload,
        });
        self.deltas.clear();
        self.answer_all_held()
    }

    fn delta_push(&mut self, message: SwmrDeltaPush) -> Vec<SwmrOutput> {
        if self.terminal() {
            return Self::diagnostic(SwmrDiagCode::PushAfterTerminal);
        }
        if !self.check_writer(&message.writer_id) {
            return Self::diagnostic(SwmrDiagCode::WriterConflict);
        }
        if self.snapshot.is_none() {
            return Self::diagnostic(SwmrDiagCode::DeltaBeforeSnapshot);
        }
        if self
            .max_deltas
            .is_some_and(|limit| self.deltas.len() >= limit)
        {
            return Self::diagnostic(SwmrDiagCode::RetentionBoundExceeded);
        }
        let seq = self.head() + 1;
        self.deltas.push(SwmrDelta {
            base_seq: seq - 1,
            seq,
            payload: message.payload,
        });
        self.answer_all_held()
    }

    fn reset(&mut self, message: SwmrReset) -> Vec<SwmrOutput> {
        if self.terminal() {
            return Self::diagnostic(SwmrDiagCode::PushAfterTerminal);
        }
        if !self.check_writer(&message.writer_id) {
            return Self::diagnostic(SwmrDiagCode::WriterConflict);
        }
        let reason = SwmrResetReason::ProducerRequested;
        self.epoch += 1;
        self.snapshot = None;
        self.deltas.clear();
        self.reset_reason = Some(reason);
        self.reset_detail = message.detail;
        let mut outputs = Vec::new();
        for stream_id in self.order.clone() {
            let Some(Some(held)) = self.streams.get(&stream_id).cloned() else {
                continue;
            };
            if held.cursor.is_none() {
                if let Some(response) =
                    self.resolve(&held.swmr_id, &stream_id, None, held.timeout_ms)
                {
                    if let Some(token) = held.timer_token {
                        outputs.push(SwmrOutput::CancelTimer(SwmrCancelTimer { token }));
                    }
                    outputs.push(SwmrOutput::ReadResponse(response));
                    self.streams.insert(stream_id, None);
                }
                continue;
            }
            if let Some(token) = held.timer_token {
                outputs.push(SwmrOutput::CancelTimer(SwmrCancelTimer { token }));
            }
            outputs.push(SwmrOutput::ReadResponse(Self::response(
                &held.swmr_id,
                &stream_id,
                None,
                Vec::new(),
                None,
                SwmrState::Reset,
                Some(reason),
                None,
                self.reset_detail.clone(),
            )));
            self.streams.insert(stream_id, None);
        }
        outputs
    }

    fn seal(&mut self) -> Vec<SwmrOutput> {
        if self.sealed {
            return Vec::new();
        }
        self.sealed = true;
        self.answer_all_held()
    }

    fn close(&mut self, message: SwmrClose) -> Vec<SwmrOutput> {
        if self.closed {
            return Vec::new();
        }
        self.closed = true;
        self.close_error = message.error;
        let mut outputs = self.answer_all_held();
        outputs.push(SwmrOutput::ProducerStop(SwmrProducerStop {
            reason: if self.close_error.is_some() {
                SwmrStopReason::Failed
            } else {
                SwmrStopReason::Closed
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

    fn read(&mut self, message: SwmrReadRequest) -> Vec<SwmrOutput> {
        self.ensure_stream(&message.stream_id);
        let mut outputs = Vec::new();
        if let Some(Some(prior)) = self.streams.get(&message.stream_id) {
            if let Some(token) = prior.timer_token {
                outputs.push(SwmrOutput::CancelTimer(SwmrCancelTimer { token }));
            }
        }
        self.streams.insert(message.stream_id.clone(), None);
        if let Some(response) = self.resolve(
            &message.swmr_id,
            &message.stream_id,
            message.cursor.as_ref(),
            message.timeout_ms,
        ) {
            outputs.push(SwmrOutput::ReadResponse(response));
            return outputs;
        }
        let timer_token = message.timeout_ms.filter(|ms| *ms > 0).map(|ms| {
            let token = self.next_token;
            self.next_token += 1;
            outputs.push(SwmrOutput::SetTimer(SwmrSetTimer { token, ms }));
            token
        });
        self.streams.insert(
            message.stream_id,
            Some(HeldRead {
                swmr_id: message.swmr_id,
                cursor: message.cursor,
                timeout_ms: message.timeout_ms,
                timer_token,
            }),
        );
        outputs
    }

    fn resolve(
        &self,
        swmr_id: &str,
        stream_id: &str,
        cursor: Option<&SwmrCursor>,
        timeout_ms: Option<i64>,
    ) -> Option<SwmrReadResponse> {
        if cursor.is_some_and(|cursor| cursor.epoch < self.epoch) {
            return Some(Self::response(
                swmr_id,
                stream_id,
                self.snapshot.clone(),
                if self.snapshot.is_some() {
                    self.deltas.clone()
                } else {
                    Vec::new()
                },
                self.current_cursor(),
                SwmrState::Reset,
                Some(
                    self.reset_reason
                        .unwrap_or(SwmrResetReason::ProducerRequested),
                ),
                None,
                self.reset_detail.clone(),
            ));
        }
        if cursor.is_some_and(|cursor| cursor.epoch > self.epoch) {
            return Some(Self::response(
                swmr_id,
                stream_id,
                self.snapshot.clone(),
                if self.snapshot.is_some() {
                    self.deltas.clone()
                } else {
                    Vec::new()
                },
                self.current_cursor(),
                SwmrState::Reset,
                Some(SwmrResetReason::InvalidResumeSeq),
                None,
                None,
            ));
        }
        let Some(snapshot) = &self.snapshot else {
            return self.caught_up(swmr_id, stream_id, None, timeout_ms);
        };
        let Some(cursor) = cursor else {
            return Some(Self::response(
                swmr_id,
                stream_id,
                Some(snapshot.clone()),
                self.deltas.clone(),
                self.current_cursor(),
                SwmrState::Data,
                None,
                None,
                None,
            ));
        };
        if cursor.seq < snapshot.seq {
            return Some(Self::response(
                swmr_id,
                stream_id,
                Some(snapshot.clone()),
                self.deltas.clone(),
                self.current_cursor(),
                SwmrState::Reset,
                Some(SwmrResetReason::RetentionExceeded),
                None,
                None,
            ));
        }
        if cursor.seq == self.head() {
            return self.caught_up(
                swmr_id,
                stream_id,
                Some(SwmrCursor {
                    seq: self.head(),
                    epoch: self.epoch,
                }),
                timeout_ms,
            );
        }
        if cursor.seq > self.head() {
            return Some(Self::response(
                swmr_id,
                stream_id,
                Some(snapshot.clone()),
                self.deltas.clone(),
                self.current_cursor(),
                SwmrState::Reset,
                Some(SwmrResetReason::InvalidResumeSeq),
                None,
                None,
            ));
        }
        let offset = (cursor.seq - snapshot.seq) as usize;
        Some(Self::response(
            swmr_id,
            stream_id,
            None,
            self.deltas[offset..].to_vec(),
            self.current_cursor(),
            SwmrState::Data,
            None,
            None,
            None,
        ))
    }

    fn current_cursor(&self) -> Option<SwmrCursor> {
        self.snapshot.as_ref().map(|_| SwmrCursor {
            seq: self.head(),
            epoch: self.epoch,
        })
    }

    fn caught_up(
        &self,
        swmr_id: &str,
        stream_id: &str,
        next_cursor: Option<SwmrCursor>,
        timeout_ms: Option<i64>,
    ) -> Option<SwmrReadResponse> {
        let (state, error) = if self.closed {
            if self.close_error.is_some() {
                (SwmrState::Failed, self.close_error.clone())
            } else {
                (SwmrState::Closed, None)
            }
        } else if self.sealed {
            (SwmrState::Eof, None)
        } else if timeout_ms == Some(0) {
            (SwmrState::WouldBlock, None)
        } else {
            return None;
        };
        Some(Self::response(
            swmr_id,
            stream_id,
            None,
            Vec::new(),
            next_cursor,
            state,
            None,
            error,
            None,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn response(
        swmr_id: &str,
        stream_id: &str,
        snapshot: Option<SwmrSnapshot>,
        deltas: Vec<SwmrDelta>,
        next_cursor: Option<SwmrCursor>,
        state: SwmrState,
        reset_reason: Option<SwmrResetReason>,
        error: Option<SwmrError>,
        reset_detail: Option<Vec<u8>>,
    ) -> SwmrReadResponse {
        SwmrReadResponse {
            swmr_id: swmr_id.into(),
            stream_id: stream_id.into(),
            snapshot,
            deltas,
            next_cursor,
            state,
            reset_reason,
            error,
            reset_detail,
        }
    }

    fn answer_all_held(&mut self) -> Vec<SwmrOutput> {
        let mut outputs = Vec::new();
        for stream_id in self.order.clone() {
            let Some(Some(held)) = self.streams.get(&stream_id).cloned() else {
                continue;
            };
            let Some(response) = self.resolve(
                &held.swmr_id,
                &stream_id,
                held.cursor.as_ref(),
                held.timeout_ms,
            ) else {
                continue;
            };
            if let Some(token) = held.timer_token {
                outputs.push(SwmrOutput::CancelTimer(SwmrCancelTimer { token }));
            }
            outputs.push(SwmrOutput::ReadResponse(response));
            self.streams.insert(stream_id, None);
        }
        outputs
    }

    fn end_stream(&mut self, message: SwmrEndStream) -> Vec<SwmrOutput> {
        let Some(held) = self.streams.remove(&message.stream_id) else {
            return Vec::new();
        };
        self.order
            .retain(|stream_id| stream_id != &message.stream_id);
        let mut outputs = Vec::new();
        if let Some(token) = held.and_then(|held| held.timer_token) {
            outputs.push(SwmrOutput::CancelTimer(SwmrCancelTimer { token }));
        }
        if self.streams.is_empty() && self.stop_when == StopWhen::LastReader {
            outputs.push(SwmrOutput::ProducerStop(SwmrProducerStop {
                reason: SwmrStopReason::LastReaderGone,
            }));
        }
        outputs
    }

    fn timer_expired(&mut self, message: SwmrTimerExpired) -> Vec<SwmrOutput> {
        let found = self.streams.iter().find_map(|(stream_id, held)| {
            held.as_ref()
                .filter(|held| held.timer_token == Some(message.token))
                .map(|held| (stream_id.clone(), held.clone()))
        });
        let Some((stream_id, held)) = found else {
            return Vec::new();
        };
        self.streams.insert(stream_id.clone(), None);
        vec![SwmrOutput::ReadResponse(
            self.resolve(&held.swmr_id, &stream_id, held.cursor.as_ref(), Some(0))
                .expect("swmr probe unexpectedly held"),
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn snapshot_releases_a_large_held_set_in_creation_order() {
        let mut node = SwmrNode::new(StopWhen::LastReader, None);
        for index in 0..2_000 {
            assert!(node
                .handle(SwmrInput::Read(SwmrReadRequest {
                    swmr_id: "swmr-A".into(),
                    stream_id: format!("s{index:04}"),
                    cursor: None,
                    timeout_ms: None,
                }))
                .is_empty());
        }
        let outputs = node.handle(SwmrInput::SnapshotPush(SwmrSnapshotPush {
            writer_id: "writer-A".into(),
            payload: b"ready".to_vec(),
        }));
        assert_eq!(outputs.len(), 2_000);
        for (index, output) in outputs.iter().enumerate() {
            let SwmrOutput::ReadResponse(response) = output else {
                panic!("expected response");
            };
            assert_eq!(response.stream_id, format!("s{index:04}"));
        }
    }
}
