// GENERATED native Rust types + codec — do not edit.
// Source: taut-shape/ir/shape_stream.taut.py
#![allow(dead_code)]
use crate::cbor::{Cbor, DecodeError};
use alloc::{string::String, vec, vec::Vec};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamMsgType {
    #[default]
    Push,
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
impl StreamMsgType {
    pub fn wire(self) -> i64 {
        match self {
            Self::Push => 0,
            Self::Seal => 1,
            Self::Close => 2,
            Self::Read => 3,
            Self::EndStream => 4,
            Self::TimerExpired => 5,
            Self::ReadResponse => 6,
            Self::SetTimer => 7,
            Self::CancelTimer => 8,
            Self::ProducerStop => 9,
            Self::Diagnostic => 10,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::Push,
            1 => Self::Seal,
            2 => Self::Close,
            3 => Self::Read,
            4 => Self::EndStream,
            5 => Self::TimerExpired,
            6 => Self::ReadResponse,
            7 => Self::SetTimer,
            8 => Self::CancelTimer,
            9 => Self::ProducerStop,
            10 => Self::Diagnostic,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "StreamMsgType",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamState {
    #[default]
    Data,
    WouldBlock,
    Eof,
    Closed,
    Failed,
    Dropped,
}
impl StreamState {
    pub fn wire(self) -> i64 {
        match self {
            Self::Data => 0,
            Self::WouldBlock => 1,
            Self::Eof => 2,
            Self::Closed => 3,
            Self::Failed => 4,
            Self::Dropped => 5,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::Data,
            1 => Self::WouldBlock,
            2 => Self::Eof,
            3 => Self::Closed,
            4 => Self::Failed,
            5 => Self::Dropped,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "StreamState",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamErrorCode {
    #[default]
    UnknownStream,
    ProducerError,
    Internal,
    SlowConsumer,
}
impl StreamErrorCode {
    pub fn wire(self) -> i64 {
        match self {
            Self::UnknownStream => 0,
            Self::ProducerError => 1,
            Self::Internal => 2,
            Self::SlowConsumer => 3,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::UnknownStream,
            1 => Self::ProducerError,
            2 => Self::Internal,
            3 => Self::SlowConsumer,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "StreamErrorCode",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamStopReason {
    #[default]
    LastReaderGone,
    Closed,
    Failed,
}
impl StreamStopReason {
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
                    enum_name: "StreamStopReason",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamSeverity {
    #[default]
    Warn,
    Error,
}
impl StreamSeverity {
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
                    enum_name: "StreamSeverity",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamDiagCode {
    #[default]
    PushAfterTerminal,
}
impl StreamDiagCode {
    pub fn wire(self) -> i64 {
        match self {
            Self::PushAfterTerminal => 0,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::PushAfterTerminal,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "StreamDiagCode",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamPosition {
    pub seq: i64,
}
impl StreamPosition {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Int(self.seq))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            seq: c.try_get(1)?.try_int()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamRecord {
    pub seq: i64,
    pub payload: Vec<u8>,
}
impl StreamRecord {
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
pub struct StreamError {
    pub code: StreamErrorCode,
    pub message: Option<String>,
}
impl StreamError {
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
            code: StreamErrorCode::from_wire(c.try_get(1)?.try_int()?)?,
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
pub struct StreamPush {
    pub payload: Vec<u8>,
}
impl StreamPush {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Bytes(self.payload.clone()))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            payload: c.try_get(1)?.try_bytes()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamSeal {}
impl StreamSeal {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamClose {
    pub error: Option<StreamError>,
}
impl StreamClose {
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
                    Some(StreamError::from_cbor(v)?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamReadRequest {
    pub stream_id: String,
    pub max_records: Option<i64>,
    pub max_bytes: Option<i64>,
    pub timeout_ms: Option<i64>,
}
impl StreamReadRequest {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.stream_id.clone())),
            (
                2,
                match &self.max_records {
                    Some(v) => Cbor::Int(*v),
                    None => Cbor::Null,
                },
            ),
            (
                3,
                match &self.max_bytes {
                    Some(v) => Cbor::Int(*v),
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
            stream_id: c.try_get(1)?.try_text()?,
            max_records: {
                let v = c.try_get(2)?;
                if v.is_null() {
                    None
                } else {
                    Some(v.try_int()?)
                }
            },
            max_bytes: {
                let v = c.try_get(3)?;
                if v.is_null() {
                    None
                } else {
                    Some(v.try_int()?)
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
pub struct StreamEndStream {
    pub stream_id: String,
}
impl StreamEndStream {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Text(self.stream_id.clone()))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            stream_id: c.try_get(1)?.try_text()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamTimerExpired {
    pub token: i64,
}
impl StreamTimerExpired {
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
pub struct StreamReadResponse {
    pub stream_id: String,
    pub records: Vec<StreamRecord>,
    pub next_position: StreamPosition,
    pub state: StreamState,
    pub error: Option<StreamError>,
}
impl StreamReadResponse {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.stream_id.clone())),
            (
                2,
                Cbor::Array(self.records.iter().map(|x| x.to_cbor()).collect()),
            ),
            (3, self.next_position.to_cbor()),
            (4, Cbor::Int(self.state.wire())),
            (
                5,
                match &self.error {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            stream_id: c.try_get(1)?.try_text()?,
            records: c
                .try_get(2)?
                .try_array()?
                .iter()
                .map(|x| StreamRecord::from_cbor(x))
                .collect::<Result<Vec<_>, DecodeError>>()?,
            next_position: StreamPosition::from_cbor(c.try_get(3)?)?,
            state: StreamState::from_wire(c.try_get(4)?.try_int()?)?,
            error: {
                let v = c.try_get(5)?;
                if v.is_null() {
                    None
                } else {
                    Some(StreamError::from_cbor(v)?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamSetTimer {
    pub token: i64,
    pub ms: i64,
}
impl StreamSetTimer {
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
pub struct StreamCancelTimer {
    pub token: i64,
}
impl StreamCancelTimer {
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
pub struct StreamProducerStop {
    pub reason: StreamStopReason,
}
impl StreamProducerStop {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Int(self.reason.wire()))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            reason: StreamStopReason::from_wire(c.try_get(1)?.try_int()?)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamDiagnostic {
    pub severity: StreamSeverity,
    pub code: StreamDiagCode,
}
impl StreamDiagnostic {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.severity.wire())),
            (2, Cbor::Int(self.code.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            severity: StreamSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: StreamDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
        })
    }
}
