// GENERATED native Rust types + codec — do not edit.
// Source: taut-shape/ir/shape_atom.taut.py
// Regenerate with `tautc gen ... --api-only` and replace this file.
#![allow(dead_code)]
use crate::cbor::{Cbor, DecodeError};
use alloc::{string::String, vec, vec::Vec};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomMsgType {
    #[default]
    Replace,
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
impl AtomMsgType {
    pub fn wire(self) -> i64 {
        match self {
            Self::Replace => 0,
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
            0 => Self::Replace,
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
                    enum_name: "AtomMsgType",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomState {
    #[default]
    Data,
    WouldBlock,
    Eof,
    Closed,
    Failed,
}
impl AtomState {
    pub fn wire(self) -> i64 {
        match self {
            Self::Data => 0,
            Self::WouldBlock => 1,
            Self::Eof => 2,
            Self::Closed => 3,
            Self::Failed => 4,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::Data,
            1 => Self::WouldBlock,
            2 => Self::Eof,
            3 => Self::Closed,
            4 => Self::Failed,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "AtomState",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomErrorCode {
    #[default]
    UnknownAtom,
    ProducerError,
    Internal,
}
impl AtomErrorCode {
    pub fn wire(self) -> i64 {
        match self {
            Self::UnknownAtom => 0,
            Self::ProducerError => 1,
            Self::Internal => 2,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::UnknownAtom,
            1 => Self::ProducerError,
            2 => Self::Internal,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "AtomErrorCode",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomStopReason {
    #[default]
    LastReaderGone,
    Closed,
    Failed,
}
impl AtomStopReason {
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
                    enum_name: "AtomStopReason",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomSeverity {
    #[default]
    Warn,
    Error,
}
impl AtomSeverity {
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
                    enum_name: "AtomSeverity",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomDiagCode {
    #[default]
    ReplaceAfterTerminal,
}
impl AtomDiagCode {
    pub fn wire(self) -> i64 {
        match self {
            Self::ReplaceAfterTerminal => 0,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::ReplaceAfterTerminal,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "AtomDiagCode",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomVersion {
    pub version: i64,
}
impl AtomVersion {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Int(self.version))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            version: c.try_get(1)?.try_int()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomValue {
    pub version: i64,
    pub payload: Vec<u8>,
}
impl AtomValue {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.version)),
            (2, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            version: c.try_get(1)?.try_int()?,
            payload: c.try_get(2)?.try_bytes()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomError {
    pub code: AtomErrorCode,
    pub message: Option<String>,
}
impl AtomError {
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
            code: AtomErrorCode::from_wire(c.try_get(1)?.try_int()?)?,
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
pub struct AtomReplace {
    pub payload: Vec<u8>,
}
impl AtomReplace {
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
pub struct AtomSeal {}
impl AtomSeal {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomClose {
    pub error: Option<AtomError>,
}
impl AtomClose {
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
                    Some(AtomError::from_cbor(v)?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomReadRequest {
    pub atom_id: String,
    pub stream_id: String,
    pub version: Option<AtomVersion>,
    pub timeout_ms: Option<i64>,
}
impl AtomReadRequest {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.atom_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (
                3,
                match &self.version {
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
            atom_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            version: {
                let v = c.try_get(3)?;
                if v.is_null() {
                    None
                } else {
                    Some(AtomVersion::from_cbor(v)?)
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
pub struct AtomEndStream {
    pub atom_id: String,
    pub stream_id: String,
}
impl AtomEndStream {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.atom_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            atom_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomTimerExpired {
    pub token: i64,
}
impl AtomTimerExpired {
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
pub struct AtomReadResponse {
    pub atom_id: String,
    pub stream_id: String,
    pub value: Option<AtomValue>,
    pub next_version: AtomVersion,
    pub state: AtomState,
    pub error: Option<AtomError>,
}
impl AtomReadResponse {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.atom_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (
                3,
                match &self.value {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
            (4, self.next_version.to_cbor()),
            (5, Cbor::Int(self.state.wire())),
            (
                6,
                match &self.error {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            atom_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            value: {
                let v = c.try_get(3)?;
                if v.is_null() {
                    None
                } else {
                    Some(AtomValue::from_cbor(v)?)
                }
            },
            next_version: AtomVersion::from_cbor(c.try_get(4)?)?,
            state: AtomState::from_wire(c.try_get(5)?.try_int()?)?,
            error: {
                let v = c.try_get(6)?;
                if v.is_null() {
                    None
                } else {
                    Some(AtomError::from_cbor(v)?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomSetTimer {
    pub token: i64,
    pub ms: i64,
}
impl AtomSetTimer {
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
pub struct AtomCancelTimer {
    pub token: i64,
}
impl AtomCancelTimer {
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
pub struct AtomProducerStop {
    pub reason: AtomStopReason,
}
impl AtomProducerStop {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, Cbor::Int(self.reason.wire()))])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            reason: AtomStopReason::from_wire(c.try_get(1)?.try_int()?)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomDiagnostic {
    pub severity: AtomSeverity,
    pub code: AtomDiagCode,
}
impl AtomDiagnostic {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.severity.wire())),
            (2, Cbor::Int(self.code.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            severity: AtomSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: AtomDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
        })
    }
}
