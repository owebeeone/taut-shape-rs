// Source: taut-shape/ir/shape_swmr.taut.py; regenerate with scripts/regenerate_foundation_bindings.py
// GENERATED native Rust types + codec — do not edit.
#![cfg_attr(rustfmt, rustfmt::skip)]
#![allow(dead_code, clippy::redundant_closure, clippy::needless_question_mark, clippy::collapsible_if)]
extern crate alloc;
#[allow(unused_imports)]
use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::cbor::{Cbor, DecodeError};
// The file's bounds, for a decode rooted at a type that is not a message:
// `cbor::try_decode_with(bytes, MAX_DEPTH, MAX_ENCODED_LEN)`.
pub const MAX_DEPTH: usize = 32;
pub const MAX_ENCODED_LEN: Option<usize> = None;
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrMsgType {
    #[default] SnapshotPush,
    DeltaPush,
    Reset,
    Seal,
    Close,
    Read,
    EndStream,
    TimerExpired,
    ReadResponse,
    SetTimer,
    CancelTimer,
    ProducerStop,
    Diagnostic,
}
impl SwmrMsgType {
    pub fn wire(self) -> i64 { match self {
        Self::SnapshotPush => 0,
        Self::DeltaPush => 1,
        Self::Reset => 2,
        Self::Seal => 3,
        Self::Close => 4,
        Self::Read => 5,
        Self::EndStream => 6,
        Self::TimerExpired => 7,
        Self::ReadResponse => 8,
        Self::SetTimer => 9,
        Self::CancelTimer => 10,
        Self::ProducerStop => 11,
        Self::Diagnostic => 12,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::SnapshotPush,
        1 => Self::DeltaPush,
        2 => Self::Reset,
        3 => Self::Seal,
        4 => Self::Close,
        5 => Self::Read,
        6 => Self::EndStream,
        7 => Self::TimerExpired,
        8 => Self::ReadResponse,
        9 => Self::SetTimer,
        10 => Self::CancelTimer,
        11 => Self::ProducerStop,
        12 => Self::Diagnostic,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "SwmrMsgType", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrState {
    #[default] Data,
    WouldBlock,
    Eof,
    Closed,
    Failed,
    Reset,
}
impl SwmrState {
    pub fn wire(self) -> i64 { match self {
        Self::Data => 0,
        Self::WouldBlock => 1,
        Self::Eof => 2,
        Self::Closed => 3,
        Self::Failed => 4,
        Self::Reset => 5,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Data,
        1 => Self::WouldBlock,
        2 => Self::Eof,
        3 => Self::Closed,
        4 => Self::Failed,
        5 => Self::Reset,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "SwmrState", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrErrorCode {
    #[default] UnknownSwmr,
    ProducerError,
    Internal,
}
impl SwmrErrorCode {
    pub fn wire(self) -> i64 { match self {
        Self::UnknownSwmr => 0,
        Self::ProducerError => 1,
        Self::Internal => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::UnknownSwmr,
        1 => Self::ProducerError,
        2 => Self::Internal,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "SwmrErrorCode", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrStopReason {
    #[default] LastReaderGone,
    Closed,
    Failed,
}
impl SwmrStopReason {
    pub fn wire(self) -> i64 { match self {
        Self::LastReaderGone => 0,
        Self::Closed => 1,
        Self::Failed => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::LastReaderGone,
        1 => Self::Closed,
        2 => Self::Failed,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "SwmrStopReason", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrResetReason {
    #[default] ProducerRequested,
    RetentionExceeded,
    InvalidResumeSeq,
}
impl SwmrResetReason {
    pub fn wire(self) -> i64 { match self {
        Self::ProducerRequested => 0,
        Self::RetentionExceeded => 1,
        Self::InvalidResumeSeq => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::ProducerRequested,
        1 => Self::RetentionExceeded,
        2 => Self::InvalidResumeSeq,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "SwmrResetReason", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrSeverity {
    #[default] Warn,
    Error,
}
impl SwmrSeverity {
    pub fn wire(self) -> i64 { match self {
        Self::Warn => 0,
        Self::Error => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Warn,
        1 => Self::Error,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "SwmrSeverity", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrDiagCode {
    #[default] PushAfterTerminal,
    DeltaBeforeSnapshot,
    WriterConflict,
    RetentionBoundExceeded,
}
impl SwmrDiagCode {
    pub fn wire(self) -> i64 { match self {
        Self::PushAfterTerminal => 0,
        Self::DeltaBeforeSnapshot => 1,
        Self::WriterConflict => 2,
        Self::RetentionBoundExceeded => 3,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::PushAfterTerminal,
        1 => Self::DeltaBeforeSnapshot,
        2 => Self::WriterConflict,
        3 => Self::RetentionBoundExceeded,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "SwmrDiagCode", value: v }),
    }) }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrCursor {
    pub seq: i64,
    pub epoch: i64,
}
impl SwmrCursor {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.seq)),
            (2, Cbor::Int(self.epoch)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            seq: c.try_get(1)?.try_int()?,
            epoch: c.try_get(2)?.try_int()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrSnapshot {
    pub seq: i64,
    pub payload: Vec<u8>,
}
impl SwmrSnapshot {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.seq)),
            (2, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            seq: c.try_get(1)?.try_int()?,
            payload: c.try_get(2)?.try_bytes()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrDelta {
    pub base_seq: i64,
    pub seq: i64,
    pub payload: Vec<u8>,
}
impl SwmrDelta {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.base_seq)),
            (2, Cbor::Int(self.seq)),
            (3, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            base_seq: c.try_get(1)?.try_int()?,
            seq: c.try_get(2)?.try_int()?,
            payload: c.try_get(3)?.try_bytes()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrError {
    pub code: SwmrErrorCode,
    pub message: Option<String>,
}
impl SwmrError {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.code.wire())),
            (2, match &self.message { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            code: SwmrErrorCode::from_wire(c.try_get(1)?.try_int()?)?,
            message: { let v = c.try_get(2)?; if v.is_null() { None } else { Some(v.try_text()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrSnapshotPush {
    pub writer_id: String,
    pub payload: Vec<u8>,
}
impl SwmrSnapshotPush {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.writer_id.clone())),
            (2, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            writer_id: c.try_get(1)?.try_text()?,
            payload: c.try_get(2)?.try_bytes()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrDeltaPush {
    pub writer_id: String,
    pub payload: Vec<u8>,
}
impl SwmrDeltaPush {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.writer_id.clone())),
            (2, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            writer_id: c.try_get(1)?.try_text()?,
            payload: c.try_get(2)?.try_bytes()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrReset {
    pub writer_id: String,
    pub reason: SwmrResetReason,
    pub detail: Option<Vec<u8>>,
}
impl SwmrReset {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.writer_id.clone())),
            (2, Cbor::Int(self.reason.wire())),
            (3, match &self.detail { Some(v) => Cbor::Bytes(v.clone()), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            writer_id: c.try_get(1)?.try_text()?,
            reason: SwmrResetReason::from_wire(c.try_get(2)?.try_int()?)?,
            detail: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(v.try_bytes()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrSeal {
}
impl SwmrSeal {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        if !c.is_map() { return Err(DecodeError::WrongType { expected: "map" }); }
        Ok(Self {
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrClose {
    pub error: Option<SwmrError>,
}
impl SwmrClose {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            error: { let v = c.try_get(1)?; if v.is_null() { None } else { Some(SwmrError::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrReadRequest {
    pub swmr_id: String,
    pub stream_id: String,
    pub cursor: Option<SwmrCursor>,
    pub timeout_ms: Option<i64>,
}
impl SwmrReadRequest {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.swmr_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, match &self.cursor { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (4, match &self.timeout_ms { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            swmr_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            cursor: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(SwmrCursor::from_cbor(v)?) } },
            timeout_ms: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_int()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrEndStream {
    pub swmr_id: String,
    pub stream_id: String,
}
impl SwmrEndStream {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.swmr_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            swmr_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrTimerExpired {
    pub token: i64,
}
impl SwmrTimerExpired {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.token)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            token: c.try_get(1)?.try_int()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrReadResponse {
    pub swmr_id: String,
    pub stream_id: String,
    pub snapshot: Option<SwmrSnapshot>,
    pub deltas: Vec<SwmrDelta>,
    pub next_cursor: Option<SwmrCursor>,
    pub state: SwmrState,
    pub reset_reason: Option<SwmrResetReason>,
    pub error: Option<SwmrError>,
    pub reset_detail: Option<Vec<u8>>,
}
impl SwmrReadResponse {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.swmr_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, match &self.snapshot { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (4, Cbor::Array(self.deltas.iter().map(|x| x.to_cbor()).collect())),
            (5, match &self.next_cursor { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (6, Cbor::Int(self.state.wire())),
            (7, match &self.reset_reason { Some(v) => Cbor::Int(v.wire()), None => Cbor::Null }),
            (8, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (9, match &self.reset_detail { Some(v) => Cbor::Bytes(v.clone()), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            swmr_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            snapshot: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(SwmrSnapshot::from_cbor(v)?) } },
            deltas: c.try_get(4)?.try_array()?.iter().map(|x| SwmrDelta::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
            next_cursor: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(SwmrCursor::from_cbor(v)?) } },
            state: SwmrState::from_wire(c.try_get(6)?.try_int()?)?,
            reset_reason: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(SwmrResetReason::from_wire(v.try_int()?)?) } },
            error: { let v = c.try_get(8)?; if v.is_null() { None } else { Some(SwmrError::from_cbor(v)?) } },
            reset_detail: { let v = c.try_get(9)?; if v.is_null() { None } else { Some(v.try_bytes()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrSetTimer {
    pub token: i64,
    pub ms: i64,
}
impl SwmrSetTimer {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.token)),
            (2, Cbor::Int(self.ms)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            token: c.try_get(1)?.try_int()?,
            ms: c.try_get(2)?.try_int()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrCancelTimer {
    pub token: i64,
}
impl SwmrCancelTimer {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.token)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            token: c.try_get(1)?.try_int()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrProducerStop {
    pub reason: SwmrStopReason,
}
impl SwmrProducerStop {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.reason.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            reason: SwmrStopReason::from_wire(c.try_get(1)?.try_int()?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrDiagnostic {
    pub severity: SwmrSeverity,
    pub code: SwmrDiagCode,
}
impl SwmrDiagnostic {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.severity.wire())),
            (2, Cbor::Int(self.code.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            severity: SwmrSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: SwmrDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
// Generator build rust-oneof-reference.1; descriptor=1; codec=1; owner=taut.rust.foundation
const _: () = assert!(crate::cbor::descriptors::DESCRIPTOR_CONTRACT == 1, "Taut descriptor contract mismatch: rebind runtime");
const _: () = assert!(crate::cbor::descriptors::CODEC_CONTRACT == 1, "Taut codec contract mismatch: rebind runtime");
impl SwmrMsgType {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "cdae1fa62506438fd2a922e8a22cff9b4b94403cd9f44d9ac5f39f492b2be2f2";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_SwmrMsgType; descriptor.check(); descriptor }
}
impl SwmrState {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "fc70beb746641bc7a9c7a2c3ea1e69d30fad2296c8a7f607df58f597060ca1ea";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_SwmrState; descriptor.check(); descriptor }
}
impl SwmrErrorCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "f408f7e17493360a9928e14e08a60ce5c4c46736033d844a77bdab6f00c2adbf";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_SwmrErrorCode; descriptor.check(); descriptor }
}
impl SwmrStopReason {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "7a7f928bb58c24cf5dae2ff82ffb5de18d0b77010376e8dd4c87dedccc8e4303";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_SwmrStopReason; descriptor.check(); descriptor }
}
impl SwmrResetReason {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "ea2c9d7b1f88014006c99aa69dfd3b59d9bb9ede058e2552d0d6d71b632916d6";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_SwmrResetReason; descriptor.check(); descriptor }
}
impl SwmrSeverity {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "2ff1ffff30b45099c556562922a71e3981e0784ec44c12a7c7fe3ae8a80e3fd8";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_SwmrSeverity; descriptor.check(); descriptor }
}
impl SwmrDiagCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "3f5ceeacb3a48d90861f4a7d8d6daf788c77f1b5081598676017a083b27be30e";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_SwmrDiagCode; descriptor.check(); descriptor }
}
impl SwmrCursor {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "408300de3a14c549b236199e3f4e1744875b93be86ccc940e3cc3714ee661271";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrCursor.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrCursor }
}
impl SwmrSnapshot {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "74c129130c15b3e8e4301127d0329219d2b639c990700a449701c880e19466de";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrSnapshot.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrSnapshot }
}
impl SwmrDelta {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "01586efc3381cd76e0bd1518e51b7dd509b5747c8508735992534aafd3674b92";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrDelta.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrDelta }
}
impl SwmrError {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "ef69c26054d65b410e7d0a279fd77c3f024721f96773f798a14ae072574ac98a";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrError.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrError }
}
impl SwmrSnapshotPush {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "017003bb6243413b2d28ccea98883be1412576b8507d8fd71f93c531be24ad56";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrSnapshotPush.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrSnapshotPush }
}
impl SwmrDeltaPush {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "8784e53c05e26a3ce9c9d2275b103d63fef800b63cc3987495a7fd5a82acee5a";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrDeltaPush.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrDeltaPush }
}
impl SwmrReset {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "9829324a0c939cefa3b4058f9192e84f5901fbaa23801088e8e3bcd69a392615";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrReset.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrReset }
}
impl SwmrSeal {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "5141e31c02009cc1e12ca45ef6fb13f7b171f4df1834972a2c2d92fa4bf49429";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrSeal.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrSeal }
}
impl SwmrClose {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "3106953daa4898dbfc92b898c6ce64197b17545bd4bfdb067698ea6a6adfb67a";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrClose.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrClose }
}
impl SwmrReadRequest {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "5086351195f184139ad316f3201f0916ac87fe018c1e264d55b53431814253a0";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrReadRequest.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrReadRequest }
}
impl SwmrEndStream {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "f3320f96c68aa6de8f34de69ff8b57b6c354a030993581cdd8ac4c15a54742ca";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrEndStream.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrEndStream }
}
impl SwmrTimerExpired {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "0b884a97b352b3cb3be5939d1b206120f74bb3fa5784b0bcc1636ed219fb17ac";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrTimerExpired.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrTimerExpired }
}
impl SwmrReadResponse {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "48a12c75b395d32cd9efa2583a5968a342b5ecc27064b46efe6c04ffba67ca72";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrReadResponse.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrReadResponse }
}
impl SwmrSetTimer {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "dc8e65a29e82d05f5f88392b1232104c9a229fe60304b285fcf745b96010e460";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrSetTimer.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrSetTimer }
}
impl SwmrCancelTimer {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "694355ed28b3b9d954c8ae08bf5fcbb5196a0844144f73f62afe6fc83fb505b2";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrCancelTimer.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrCancelTimer }
}
impl SwmrProducerStop {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "0a5be57dcb12dd7d97b6c36223bbf79d1cb096a66120e1bcc7e43e348c99ab91";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrProducerStop.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrProducerStop }
}
impl SwmrDiagnostic {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "fca23ef338bfe2ad50194c66a2d63021a26f0c446dea44e923d113d6071d2891";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_SwmrDiagnostic.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_SwmrDiagnostic }
}
pub static SCHEMA_DESCRIPTOR: &crate::cbor::descriptors::SchemaDescriptor = &crate::cbor::descriptors::SchemaDescriptor::new(1, 2, "schema_bddb5683508c8c40",
&[SwmrCursor::descriptor, SwmrSnapshot::descriptor, SwmrDelta::descriptor, SwmrError::descriptor, SwmrSnapshotPush::descriptor, SwmrDeltaPush::descriptor, SwmrReset::descriptor, SwmrSeal::descriptor, SwmrClose::descriptor, SwmrReadRequest::descriptor, SwmrEndStream::descriptor, SwmrTimerExpired::descriptor, SwmrReadResponse::descriptor, SwmrSetTimer::descriptor, SwmrCancelTimer::descriptor, SwmrProducerStop::descriptor, SwmrDiagnostic::descriptor],
&[SwmrMsgType::descriptor, SwmrState::descriptor, SwmrErrorCode::descriptor, SwmrStopReason::descriptor, SwmrResetReason::descriptor, SwmrSeverity::descriptor, SwmrDiagCode::descriptor],
_taut_metadata::FILE_OPTIONS, _taut_metadata::FILE_EFFECTIVE);
#[allow(non_upper_case_globals, unused_variables, unused_imports)]
mod _taut_metadata {
use super::*;
use crate::cbor::descriptors::*;
pub const FILE_OPTIONS: Options = &[];
pub const FILE_EFFECTIVE: Options = &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)];
pub fn check_external() {
require_contract("schema_bddb5683508c8c40", "taut.rust.foundation", 1, 1, &["descriptors.v1", "codec.v1"]);
}
pub static ENUM_SwmrMsgType: &EnumDescriptor = &EnumDescriptor::new("SwmrMsgType", "schema_bddb5683508c8c40.SwmrMsgType", &[EnumMemberDescriptor::new("snapshot_push", 0, &[]), EnumMemberDescriptor::new("delta_push", 1, &[]), EnumMemberDescriptor::new("reset", 2, &[]), EnumMemberDescriptor::new("seal", 3, &[]), EnumMemberDescriptor::new("close", 4, &[]), EnumMemberDescriptor::new("read", 5, &[]), EnumMemberDescriptor::new("end_stream", 6, &[]), EnumMemberDescriptor::new("timer_expired", 7, &[]), EnumMemberDescriptor::new("read_response", 8, &[]), EnumMemberDescriptor::new("set_timer", 9, &[]), EnumMemberDescriptor::new("cancel_timer", 10, &[]), EnumMemberDescriptor::new("producer_stop", 11, &[]), EnumMemberDescriptor::new("diagnostic", 12, &[])], &[]);
pub static ENUM_SwmrState: &EnumDescriptor = &EnumDescriptor::new("SwmrState", "schema_bddb5683508c8c40.SwmrState", &[EnumMemberDescriptor::new("data", 0, &[]), EnumMemberDescriptor::new("would_block", 1, &[]), EnumMemberDescriptor::new("eof", 2, &[]), EnumMemberDescriptor::new("closed", 3, &[]), EnumMemberDescriptor::new("failed", 4, &[]), EnumMemberDescriptor::new("reset", 5, &[])], &[]);
pub static ENUM_SwmrErrorCode: &EnumDescriptor = &EnumDescriptor::new("SwmrErrorCode", "schema_bddb5683508c8c40.SwmrErrorCode", &[EnumMemberDescriptor::new("unknown_swmr", 0, &[]), EnumMemberDescriptor::new("producer_error", 1, &[]), EnumMemberDescriptor::new("internal", 2, &[])], &[]);
pub static ENUM_SwmrStopReason: &EnumDescriptor = &EnumDescriptor::new("SwmrStopReason", "schema_bddb5683508c8c40.SwmrStopReason", &[EnumMemberDescriptor::new("last_reader_gone", 0, &[]), EnumMemberDescriptor::new("closed", 1, &[]), EnumMemberDescriptor::new("failed", 2, &[])], &[]);
pub static ENUM_SwmrResetReason: &EnumDescriptor = &EnumDescriptor::new("SwmrResetReason", "schema_bddb5683508c8c40.SwmrResetReason", &[EnumMemberDescriptor::new("producer_requested", 0, &[]), EnumMemberDescriptor::new("retention_exceeded", 1, &[]), EnumMemberDescriptor::new("invalid_resume_seq", 2, &[])], &[]);
pub static ENUM_SwmrSeverity: &EnumDescriptor = &EnumDescriptor::new("SwmrSeverity", "schema_bddb5683508c8c40.SwmrSeverity", &[EnumMemberDescriptor::new("warn", 0, &[]), EnumMemberDescriptor::new("error", 1, &[])], &[]);
pub static ENUM_SwmrDiagCode: &EnumDescriptor = &EnumDescriptor::new("SwmrDiagCode", "schema_bddb5683508c8c40.SwmrDiagCode", &[EnumMemberDescriptor::new("push_after_terminal", 0, &[]), EnumMemberDescriptor::new("delta_before_snapshot", 1, &[]), EnumMemberDescriptor::new("writer_conflict", 2, &[]), EnumMemberDescriptor::new("retention_bound_exceeded", 3, &[])], &[]);
pub static BINDING_SwmrCursor: &SchemaBinding<SwmrCursor> = &SchemaBinding::new(MessageDescriptor::new("SwmrCursor", "schema_bddb5683508c8c40.SwmrCursor", 1, &[
FieldDescriptor::new("seq", Some("seq"), 1, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("epoch", Some("epoch"), 2, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.seq)),
1 => FieldState::from_primitive(PrimitiveValue::Int(value.epoch)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrSnapshot: &SchemaBinding<SwmrSnapshot> = &SchemaBinding::new(MessageDescriptor::new("SwmrSnapshot", "schema_bddb5683508c8c40.SwmrSnapshot", 1, &[
FieldDescriptor::new("seq", Some("seq"), 1, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("payload", Some("payload"), 2, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.seq)),
1 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrDelta: &SchemaBinding<SwmrDelta> = &SchemaBinding::new(MessageDescriptor::new("SwmrDelta", "schema_bddb5683508c8c40.SwmrDelta", 1, &[
FieldDescriptor::new("base_seq", Some("base_seq"), 1, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("seq", Some("seq"), 2, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("payload", Some("payload"), 3, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.base_seq)),
1 => FieldState::from_primitive(PrimitiveValue::Int(value.seq)),
2 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrError: &SchemaBinding<SwmrError> = &SchemaBinding::new(MessageDescriptor::new("SwmrError", "schema_bddb5683508c8c40.SwmrError", 1, &[
FieldDescriptor::new("code", Some("code"), 1, TypeDescriptor::Enum(SwmrErrorCode::descriptor), "required", false, false, &[]),
FieldDescriptor::new("message", Some("message"), 2, TypeDescriptor::String, "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, Some(value.code.wire())),
1 => match &value.message { Some(v) => FieldState::from_primitive(PrimitiveValue::String(v)), None => FieldState::new(false, None) },
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrSnapshotPush: &SchemaBinding<SwmrSnapshotPush> = &SchemaBinding::new(MessageDescriptor::new("SwmrSnapshotPush", "schema_bddb5683508c8c40.SwmrSnapshotPush", 1, &[
FieldDescriptor::new("writer_id", Some("writer_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("payload", Some("payload"), 2, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.writer_id)),
1 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrDeltaPush: &SchemaBinding<SwmrDeltaPush> = &SchemaBinding::new(MessageDescriptor::new("SwmrDeltaPush", "schema_bddb5683508c8c40.SwmrDeltaPush", 1, &[
FieldDescriptor::new("writer_id", Some("writer_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("payload", Some("payload"), 2, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.writer_id)),
1 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrReset: &SchemaBinding<SwmrReset> = &SchemaBinding::new(MessageDescriptor::new("SwmrReset", "schema_bddb5683508c8c40.SwmrReset", 1, &[
FieldDescriptor::new("writer_id", Some("writer_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("reason", Some("reason"), 2, TypeDescriptor::Enum(SwmrResetReason::descriptor), "required", false, false, &[]),
FieldDescriptor::new("detail", Some("detail"), 3, TypeDescriptor::Bytes, "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.writer_id)),
1 => FieldState::new(true, Some(value.reason.wire())),
2 => match &value.detail { Some(v) => FieldState::from_primitive(PrimitiveValue::Bytes(v)), None => FieldState::new(false, None) },
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrSeal: &SchemaBinding<SwmrSeal> = &SchemaBinding::new(MessageDescriptor::new("SwmrSeal", "schema_bddb5683508c8c40.SwmrSeal", 1, &[
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
panic!("Taut typed binding field mismatch: regenerate artifact")
}, |value, index, visitor| {
true });
pub static BINDING_SwmrClose: &SchemaBinding<SwmrClose> = &SchemaBinding::new(MessageDescriptor::new("SwmrClose", "schema_bddb5683508c8c40.SwmrClose", 1, &[
FieldDescriptor::new("error", Some("error"), 1, TypeDescriptor::Message(SwmrError::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(value.error.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 0 {
if let Some(child) = &value.error {
if !visitor.record(&SwmrError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_SwmrReadRequest: &SchemaBinding<SwmrReadRequest> = &SchemaBinding::new(MessageDescriptor::new("SwmrReadRequest", "schema_bddb5683508c8c40.SwmrReadRequest", 1, &[
FieldDescriptor::new("swmr_id", Some("swmr_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("cursor", Some("cursor"), 3, TypeDescriptor::Message(SwmrCursor::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("timeout_ms", Some("timeout_ms"), 4, TypeDescriptor::Int, "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.swmr_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
2 => FieldState::new(value.cursor.is_some(), None),
3 => match &value.timeout_ms { Some(v) => FieldState::from_primitive(PrimitiveValue::Int(*v)), None => FieldState::new(false, None) },
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 2 {
if let Some(child) = &value.cursor {
if !visitor.record(&SwmrCursor::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_SwmrEndStream: &SchemaBinding<SwmrEndStream> = &SchemaBinding::new(MessageDescriptor::new("SwmrEndStream", "schema_bddb5683508c8c40.SwmrEndStream", 1, &[
FieldDescriptor::new("swmr_id", Some("swmr_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.swmr_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrTimerExpired: &SchemaBinding<SwmrTimerExpired> = &SchemaBinding::new(MessageDescriptor::new("SwmrTimerExpired", "schema_bddb5683508c8c40.SwmrTimerExpired", 1, &[
FieldDescriptor::new("token", Some("token"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.token)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrReadResponse: &SchemaBinding<SwmrReadResponse> = &SchemaBinding::new(MessageDescriptor::new("SwmrReadResponse", "schema_bddb5683508c8c40.SwmrReadResponse", 1, &[
FieldDescriptor::new("swmr_id", Some("swmr_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("snapshot", Some("snapshot"), 3, TypeDescriptor::Message(SwmrSnapshot::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("deltas", Some("deltas"), 4, TypeDescriptor::List(&TypeDescriptor::Message(SwmrDelta::descriptor)), "required", false, false, &[]),
FieldDescriptor::new("next_cursor", Some("next_cursor"), 5, TypeDescriptor::Message(SwmrCursor::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("state", Some("state"), 6, TypeDescriptor::Enum(SwmrState::descriptor), "required", false, false, &[]),
FieldDescriptor::new("reset_reason", Some("reset_reason"), 7, TypeDescriptor::Enum(SwmrResetReason::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("error", Some("error"), 8, TypeDescriptor::Message(SwmrError::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("reset_detail", Some("reset_detail"), 9, TypeDescriptor::Bytes, "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.swmr_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
2 => FieldState::new(value.snapshot.is_some(), None),
3 => FieldState::new(true, None),
4 => FieldState::new(value.next_cursor.is_some(), None),
5 => FieldState::new(true, Some(value.state.wire())),
6 => FieldState::new(value.reset_reason.is_some(), value.reset_reason.as_ref().map(|v| v.wire())),
7 => FieldState::new(value.error.is_some(), None),
8 => match &value.reset_detail { Some(v) => FieldState::from_primitive(PrimitiveValue::Bytes(v)), None => FieldState::new(false, None) },
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 2 {
if let Some(child) = &value.snapshot {
if !visitor.record(&SwmrSnapshot::taut_binding().bind(child)) { return false; }
}
}
if index == 3 {
for (index0, item0) in value.deltas.iter().enumerate() {
visitor.push_index(index0);
if !visitor.record(&SwmrDelta::taut_binding().bind(item0)) { return false; }
visitor.pop();
}
}
if index == 4 {
if let Some(child) = &value.next_cursor {
if !visitor.record(&SwmrCursor::taut_binding().bind(child)) { return false; }
}
}
if index == 7 {
if let Some(child) = &value.error {
if !visitor.record(&SwmrError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_SwmrSetTimer: &SchemaBinding<SwmrSetTimer> = &SchemaBinding::new(MessageDescriptor::new("SwmrSetTimer", "schema_bddb5683508c8c40.SwmrSetTimer", 1, &[
FieldDescriptor::new("token", Some("token"), 1, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("ms", Some("ms"), 2, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.token)),
1 => FieldState::from_primitive(PrimitiveValue::Int(value.ms)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrCancelTimer: &SchemaBinding<SwmrCancelTimer> = &SchemaBinding::new(MessageDescriptor::new("SwmrCancelTimer", "schema_bddb5683508c8c40.SwmrCancelTimer", 1, &[
FieldDescriptor::new("token", Some("token"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.token)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrProducerStop: &SchemaBinding<SwmrProducerStop> = &SchemaBinding::new(MessageDescriptor::new("SwmrProducerStop", "schema_bddb5683508c8c40.SwmrProducerStop", 1, &[
FieldDescriptor::new("reason", Some("reason"), 1, TypeDescriptor::Enum(SwmrStopReason::descriptor), "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, Some(value.reason.wire())),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_SwmrDiagnostic: &SchemaBinding<SwmrDiagnostic> = &SchemaBinding::new(MessageDescriptor::new("SwmrDiagnostic", "schema_bddb5683508c8c40.SwmrDiagnostic", 1, &[
FieldDescriptor::new("severity", Some("severity"), 1, TypeDescriptor::Enum(SwmrSeverity::descriptor), "required", false, false, &[]),
FieldDescriptor::new("code", Some("code"), 2, TypeDescriptor::Enum(SwmrDiagCode::descriptor), "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, Some(value.severity.wire())),
1 => FieldState::new(true, Some(value.code.wire())),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
}
