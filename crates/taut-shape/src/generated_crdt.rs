// GENERATED native Rust types + codec — do not edit.
// Source: taut-shape/ir/shape_crdt.taut.py
#![allow(dead_code)]
use crate::cbor::{Cbor, DecodeError};
use alloc::{string::String, vec, vec::Vec};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CrdtMsgType {
    #[default]
    Apply,
    InstallBootstrap,
    Seal,
    Close,
    Read,
    ReadResponse,
    Diagnostic,
}
impl CrdtMsgType {
    pub fn wire(self) -> i64 {
        match self {
            Self::Apply => 0,
            Self::InstallBootstrap => 1,
            Self::Seal => 2,
            Self::Close => 3,
            Self::Read => 4,
            Self::ReadResponse => 5,
            Self::Diagnostic => 6,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::Apply,
            1 => Self::InstallBootstrap,
            2 => Self::Seal,
            3 => Self::Close,
            4 => Self::Read,
            5 => Self::ReadResponse,
            6 => Self::Diagnostic,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "CrdtMsgType",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CrdtState {
    #[default]
    Data,
    Empty,
    Eof,
    Closed,
    Failed,
    BootstrapRequired,
    InvalidCursor,
}
impl CrdtState {
    pub fn wire(self) -> i64 {
        match self {
            Self::Data => 0,
            Self::Empty => 1,
            Self::Eof => 2,
            Self::Closed => 3,
            Self::Failed => 4,
            Self::BootstrapRequired => 5,
            Self::InvalidCursor => 6,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::Data,
            1 => Self::Empty,
            2 => Self::Eof,
            3 => Self::Closed,
            4 => Self::Failed,
            5 => Self::BootstrapRequired,
            6 => Self::InvalidCursor,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "CrdtState",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CrdtErrorCode {
    #[default]
    UnknownCrdt,
    ProducerError,
    Internal,
}
impl CrdtErrorCode {
    pub fn wire(self) -> i64 {
        match self {
            Self::UnknownCrdt => 0,
            Self::ProducerError => 1,
            Self::Internal => 2,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::UnknownCrdt,
            1 => Self::ProducerError,
            2 => Self::Internal,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "CrdtErrorCode",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CrdtSeverity {
    #[default]
    Warn,
    Error,
}
impl CrdtSeverity {
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
                    enum_name: "CrdtSeverity",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CrdtDiagCode {
    #[default]
    ApplyAfterTerminal,
    InvalidOperation,
    Equivocation,
    BootstrapConflict,
    PendingBoundExceeded,
}
impl CrdtDiagCode {
    pub fn wire(self) -> i64 {
        match self {
            Self::ApplyAfterTerminal => 0,
            Self::InvalidOperation => 1,
            Self::Equivocation => 2,
            Self::BootstrapConflict => 3,
            Self::PendingBoundExceeded => 4,
        }
    }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> {
        Ok(match v {
            0 => Self::ApplyAfterTerminal,
            1 => Self::InvalidOperation,
            2 => Self::Equivocation,
            3 => Self::BootstrapConflict,
            4 => Self::PendingBoundExceeded,
            _ => {
                return Err(DecodeError::UnknownEnum {
                    enum_name: "CrdtDiagCode",
                    value: v,
                })
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtClockEntry {
    pub origin: String,
    pub seq: i64,
}
impl CrdtClockEntry {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.origin.clone())),
            (2, Cbor::Int(self.seq)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            origin: c.try_get(1)?.try_text()?,
            seq: c.try_get(2)?.try_int()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtClock {
    pub entries: Vec<CrdtClockEntry>,
}
impl CrdtClock {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(
            1,
            Cbor::Array(self.entries.iter().map(|x| x.to_cbor()).collect()),
        )])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            entries: c
                .try_get(1)?
                .try_array()?
                .iter()
                .map(|x| CrdtClockEntry::from_cbor(x))
                .collect::<Result<Vec<_>, DecodeError>>()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtOp {
    pub origin: String,
    pub seq: i64,
    pub deps: CrdtClock,
    pub payload: Vec<u8>,
}
impl CrdtOp {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.origin.clone())),
            (2, Cbor::Int(self.seq)),
            (3, self.deps.to_cbor()),
            (4, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            origin: c.try_get(1)?.try_text()?,
            seq: c.try_get(2)?.try_int()?,
            deps: CrdtClock::from_cbor(c.try_get(3)?)?,
            payload: c.try_get(4)?.try_bytes()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtBootstrap {
    pub clock: CrdtClock,
    pub state: Vec<u8>,
}
impl CrdtBootstrap {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, self.clock.to_cbor()),
            (2, Cbor::Bytes(self.state.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            clock: CrdtClock::from_cbor(c.try_get(1)?)?,
            state: c.try_get(2)?.try_bytes()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtError {
    pub code: CrdtErrorCode,
    pub message: Option<String>,
}
impl CrdtError {
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
            code: CrdtErrorCode::from_wire(c.try_get(1)?.try_int()?)?,
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
pub struct CrdtApply {
    pub op: CrdtOp,
}
impl CrdtApply {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, self.op.to_cbor())])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            op: CrdtOp::from_cbor(c.try_get(1)?)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtInstallBootstrap {
    pub bootstrap: CrdtBootstrap,
}
impl CrdtInstallBootstrap {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![(1, self.bootstrap.to_cbor())])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            bootstrap: CrdtBootstrap::from_cbor(c.try_get(1)?)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtSeal {}
impl CrdtSeal {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtClose {
    pub error: Option<CrdtError>,
}
impl CrdtClose {
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
                    Some(CrdtError::from_cbor(v)?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtReadRequest {
    pub crdt_id: String,
    pub stream_id: String,
    pub cursor: Option<CrdtClock>,
}
impl CrdtReadRequest {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.crdt_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (
                3,
                match &self.cursor {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            crdt_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            cursor: {
                let v = c.try_get(3)?;
                if v.is_null() {
                    None
                } else {
                    Some(CrdtClock::from_cbor(v)?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtReadResponse {
    pub crdt_id: String,
    pub stream_id: String,
    pub bootstrap: Option<CrdtBootstrap>,
    pub ops: Vec<CrdtOp>,
    pub next_cursor: CrdtClock,
    pub state: CrdtState,
    pub error: Option<CrdtError>,
}
impl CrdtReadResponse {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.crdt_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (
                3,
                match &self.bootstrap {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
            (
                4,
                Cbor::Array(self.ops.iter().map(|x| x.to_cbor()).collect()),
            ),
            (5, self.next_cursor.to_cbor()),
            (6, Cbor::Int(self.state.wire())),
            (
                7,
                match &self.error {
                    Some(v) => v.to_cbor(),
                    None => Cbor::Null,
                },
            ),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            crdt_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            bootstrap: {
                let v = c.try_get(3)?;
                if v.is_null() {
                    None
                } else {
                    Some(CrdtBootstrap::from_cbor(v)?)
                }
            },
            ops: c
                .try_get(4)?
                .try_array()?
                .iter()
                .map(|x| CrdtOp::from_cbor(x))
                .collect::<Result<Vec<_>, DecodeError>>()?,
            next_cursor: CrdtClock::from_cbor(c.try_get(5)?)?,
            state: CrdtState::from_wire(c.try_get(6)?.try_int()?)?,
            error: {
                let v = c.try_get(7)?;
                if v.is_null() {
                    None
                } else {
                    Some(CrdtError::from_cbor(v)?)
                }
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtDiagnostic {
    pub severity: CrdtSeverity,
    pub code: CrdtDiagCode,
    pub origin: Option<String>,
    pub seq: Option<i64>,
}
impl CrdtDiagnostic {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.severity.wire())),
            (2, Cbor::Int(self.code.wire())),
            (
                3,
                match &self.origin {
                    Some(v) => Cbor::Text(v.clone()),
                    None => Cbor::Null,
                },
            ),
            (
                4,
                match &self.seq {
                    Some(v) => Cbor::Int(*v),
                    None => Cbor::Null,
                },
            ),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            severity: CrdtSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: CrdtDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
            origin: {
                let v = c.try_get(3)?;
                if v.is_null() {
                    None
                } else {
                    Some(v.try_text()?)
                }
            },
            seq: {
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
