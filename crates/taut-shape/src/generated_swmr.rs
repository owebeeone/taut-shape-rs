// GENERATED native Rust types + codec — do not edit.
// Source: taut-shape/ir/shape_swmr.taut.py
#![allow(dead_code)]
use crate::cbor::{Cbor, DecodeError};
use alloc::{string::String, vec, vec::Vec};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrMsgType {
    #[default]
    SnapshotPush,
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
    pub fn wire(self) -> i64 {
        match self {
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
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
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
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "SwmrMsgType",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrState {
    #[default]
    Data,
    WouldBlock,
    Eof,
    Closed,
    Failed,
    Reset,
}
impl SwmrState {
    pub fn wire(self) -> i64 {
        match self {
            Self::Data => 0,
            Self::WouldBlock => 1,
            Self::Eof => 2,
            Self::Closed => 3,
            Self::Failed => 4,
            Self::Reset => 5,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::Data,
            1 => Self::WouldBlock,
            2 => Self::Eof,
            3 => Self::Closed,
            4 => Self::Failed,
            5 => Self::Reset,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "SwmrState",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrErrorCode {
    #[default]
    UnknownSwmr,
    ProducerError,
    Internal,
}
impl SwmrErrorCode {
    pub fn wire(self) -> i64 {
        match self {
            Self::UnknownSwmr => 0,
            Self::ProducerError => 1,
            Self::Internal => 2,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::UnknownSwmr,
            1 => Self::ProducerError,
            2 => Self::Internal,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "SwmrErrorCode",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrStopReason {
    #[default]
    LastReaderGone,
    Closed,
    Failed,
}
impl SwmrStopReason {
    pub fn wire(self) -> i64 {
        match self {
            Self::LastReaderGone => 0,
            Self::Closed => 1,
            Self::Failed => 2,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::LastReaderGone,
            1 => Self::Closed,
            2 => Self::Failed,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "SwmrStopReason",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrResetReason {
    #[default]
    ProducerRequested,
    RetentionExceeded,
    InvalidResumeSeq,
}
impl SwmrResetReason {
    pub fn wire(self) -> i64 {
        match self {
            Self::ProducerRequested => 0,
            Self::RetentionExceeded => 1,
            Self::InvalidResumeSeq => 2,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::ProducerRequested,
            1 => Self::RetentionExceeded,
            2 => Self::InvalidResumeSeq,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "SwmrResetReason",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrSeverity {
    #[default]
    Warn,
    Error,
}
impl SwmrSeverity {
    pub fn wire(self) -> i64 {
        match self {
            Self::Warn => 0,
            Self::Error => 1,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::Warn,
            1 => Self::Error,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "SwmrSeverity",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SwmrDiagCode {
    #[default]
    PushAfterTerminal,
    DeltaBeforeSnapshot,
    WriterConflict,
    RetentionBoundExceeded,
}
impl SwmrDiagCode {
    pub fn wire(self) -> i64 {
        match self {
            Self::PushAfterTerminal => 0,
            Self::DeltaBeforeSnapshot => 1,
            Self::WriterConflict => 2,
            Self::RetentionBoundExceeded => 3,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::PushAfterTerminal,
            1 => Self::DeltaBeforeSnapshot,
            2 => Self::WriterConflict,
            3 => Self::RetentionBoundExceeded,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "SwmrDiagCode",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrCursor {
    pub seq: i64,
    pub epoch: i64,
}
impl SwmrCursor {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Int(self.seq)), (2, Cbor::Int(self.epoch))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            seq: c.try_get(1)?.try_int()?,
            epoch: c.try_get(2)?.try_int()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrSnapshot {
    pub seq: i64,
    pub payload: Vec<u8>,
}
impl SwmrSnapshot {
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
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrDelta {
    pub base_seq: i64,
    pub seq: i64,
    pub payload: Vec<u8>,
}
impl SwmrDelta {
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
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrError {
    pub code: SwmrErrorCode,
    pub message: Option<String>,
}
impl SwmrError {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.code.wire())),
            (
                2,
                match &self.message {
                    Some(v) => Cbor::Text(v.clone()),
                    None => Cbor::Null,
                },
            ),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            code: SwmrErrorCode::from_wire(c.try_get(1)?.try_int()?)?,
            message: {
                let v = c.try_get(2)?;
                if v.is_null() {
                    None
                } else {
                    Some(v.try_text()?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrSnapshotPush {
    pub writer_id: String,
    pub payload: Vec<u8>,
}
impl SwmrSnapshotPush {
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
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrDeltaPush {
    pub writer_id: String,
    pub payload: Vec<u8>,
}
impl SwmrDeltaPush {
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
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrReset {
    pub writer_id: String,
    pub reason: SwmrResetReason,
    pub detail: Option<Vec<u8>>,
}
impl SwmrReset {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.writer_id.clone())),
            (2, Cbor::Int(self.reason.wire())),
            (
                3,
                match &self.detail {
                    Some(v) => Cbor::Bytes(v.clone()),
                    None => Cbor::Null,
                },
            ),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            writer_id: c.try_get(1)?.try_text()?,
            reason: SwmrResetReason::from_wire(c.try_get(2)?.try_int()?)?,
            detail: {
                let v = c.try_get(3)?;
                if v.is_null() {
                    None
                } else {
                    Some(v.try_bytes()?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrSeal {}
impl SwmrSeal {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrClose {
    pub error: Option<SwmrError>,
}
impl SwmrClose {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(
            1,
            match &self.error {
                Some(v) => v.to_cbor(),
                None => Cbor::Null,
            },
        )])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            error: {
                let v = c.try_get(1)?;
                if v.is_null() {
                    None
                } else {
                    Some(SwmrError::from_cbor(v)?)
                }
            },
        })
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
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.swmr_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (
                3,
                match &self.cursor {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
            (
                4,
                match &self.timeout_ms {
                    Some(v) => Cbor::Int(*v),
                    None => Cbor::Null,
                },
            ),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            swmr_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            cursor: {
                let v = c.try_get(3)?;
                if v.is_null() {
                    None
                } else {
                    Some(SwmrCursor::from_cbor(v)?)
                }
            },
            timeout_ms: {
                let v = c.try_get(4)?;
                if v.is_null() {
                    None
                } else {
                    Some(v.try_int()?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrEndStream {
    pub swmr_id: String,
    pub stream_id: String,
}
impl SwmrEndStream {
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
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrTimerExpired {
    pub token: i64,
}
impl SwmrTimerExpired {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Int(self.token))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            token: c.try_get(1)?.try_int()?,
        })
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
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.swmr_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (
                3,
                match &self.snapshot {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
            (
                4,
                Cbor::Array(self.deltas.iter().map(|x| x.to_cbor()).collect()),
            ),
            (
                5,
                match &self.next_cursor {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
            (6, Cbor::Int(self.state.wire())),
            (
                7,
                match &self.reset_reason {
                    Some(v) => Cbor::Int(v.wire()),
                    None => Cbor::Null,
                },
            ),
            (
                8,
                match &self.error {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
            (
                9,
                match &self.reset_detail {
                    Some(v) => Cbor::Bytes(v.clone()),
                    None => Cbor::Null,
                },
            ),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            swmr_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            snapshot: {
                let v = c.try_get(3)?;
                if v.is_null() {
                    None
                } else {
                    Some(SwmrSnapshot::from_cbor(v)?)
                }
            },
            deltas: c
                .try_get(4)?
                .try_array()?
                .iter()
                .map(|x| SwmrDelta::from_cbor(x))
                .collect::<Result<Vec<_>, DecodeError>>()?,
            next_cursor: {
                let v = c.try_get(5)?;
                if v.is_null() {
                    None
                } else {
                    Some(SwmrCursor::from_cbor(v)?)
                }
            },
            state: SwmrState::from_wire(c.try_get(6)?.try_int()?)?,
            reset_reason: {
                let v = c.try_get(7)?;
                if v.is_null() {
                    None
                } else {
                    Some(SwmrResetReason::from_wire(v.try_int()?)?)
                }
            },
            error: {
                let v = c.try_get(8)?;
                if v.is_null() {
                    None
                } else {
                    Some(SwmrError::from_cbor(v)?)
                }
            },
            reset_detail: {
                let v = c.try_get(9)?;
                if v.is_null() {
                    None
                } else {
                    Some(v.try_bytes()?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrSetTimer {
    pub token: i64,
    pub ms: i64,
}
impl SwmrSetTimer {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Int(self.token)), (2, Cbor::Int(self.ms))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            token: c.try_get(1)?.try_int()?,
            ms: c.try_get(2)?.try_int()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrCancelTimer {
    pub token: i64,
}
impl SwmrCancelTimer {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Int(self.token))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            token: c.try_get(1)?.try_int()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrProducerStop {
    pub reason: SwmrStopReason,
}
impl SwmrProducerStop {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Int(self.reason.wire()))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            reason: SwmrStopReason::from_wire(c.try_get(1)?.try_int()?)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SwmrDiagnostic {
    pub severity: SwmrSeverity,
    pub code: SwmrDiagCode,
}
impl SwmrDiagnostic {
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
}
