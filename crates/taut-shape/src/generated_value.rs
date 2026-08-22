// GENERATED native Rust types + codec — do not edit.
// Source: taut-shape/ir/shape_value.taut.py
// Schema sha256: 405278e2797ac5f0935b92c40b99a6c87b7edbfa288b640cca45c2c2ce1c3e8c
// Command: PYTHONPATH=../taut/src python3 -m taut.cli gen
//   ir/shape_value.taut.py -o <out> -l rust --api-only --fail-closed
#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt::skip)]
use alloc::{string::String, vec, vec::Vec};
use crate::cbor::{Cbor, DecodeError};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ValueMsgType {
    #[default] Set,
    Read,
    ReadResponse,
    Diagnostic,
}
impl ValueMsgType {
    pub fn wire(self) -> i64 { match self {
        Self::Set => 0,
        Self::Read => 1,
        Self::ReadResponse => 2,
        Self::Diagnostic => 3,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Set,
        1 => Self::Read,
        2 => Self::ReadResponse,
        3 => Self::Diagnostic,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ValueMsgType", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ValueState {
    #[default] Data,
    Empty,
}
impl ValueState {
    pub fn wire(self) -> i64 { match self {
        Self::Data => 0,
        Self::Empty => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Data,
        1 => Self::Empty,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ValueState", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ValueSeverity {
    #[default] Warn,
    Error,
}
impl ValueSeverity {
    pub fn wire(self) -> i64 { match self {
        Self::Warn => 0,
        Self::Error => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Warn,
        1 => Self::Error,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ValueSeverity", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ValueDiagCode {
    #[default] Equivocation,
}
impl ValueDiagCode {
    pub fn wire(self) -> i64 { match self {
        Self::Equivocation => 0,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Equivocation,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ValueDiagCode", value: v }),
    }) }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueStamp {
    pub origin: String,
    pub seq: i64,
    pub lamport: i64,
}
impl ValueStamp {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.origin.clone())),
            (2, Cbor::Int(self.seq)),
            (3, Cbor::Int(self.lamport)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            origin: c.try_get(1)?.try_text()?,
            seq: c.try_get(2)?.try_int()?,
            lamport: c.try_get(3)?.try_int()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueSet {
    pub origin: String,
    pub seq: i64,
    pub lamport: i64,
    pub prev: Option<Vec<u8>>,
    pub payload: Vec<u8>,
}
impl ValueSet {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.origin.clone())),
            (2, Cbor::Int(self.seq)),
            (3, Cbor::Int(self.lamport)),
            (4, match &self.prev { Some(v) => Cbor::Bytes(v.clone()), None => Cbor::Null }),
            (5, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            origin: c.try_get(1)?.try_text()?,
            seq: c.try_get(2)?.try_int()?,
            lamport: c.try_get(3)?.try_int()?,
            prev: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_bytes()?) } },
            payload: c.try_get(5)?.try_bytes()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueReadRequest {
    pub value_id: String,
    pub stream_id: String,
}
impl ValueReadRequest {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.value_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            value_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueReadResponse {
    pub value_id: String,
    pub stream_id: String,
    pub value: Option<Vec<u8>>,
    pub winner: Option<ValueStamp>,
    pub state: ValueState,
}
impl ValueReadResponse {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.value_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, match &self.value { Some(v) => Cbor::Bytes(v.clone()), None => Cbor::Null }),
            (4, match &self.winner { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (5, Cbor::Int(self.state.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            value_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            value: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(v.try_bytes()?) } },
            winner: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(ValueStamp::from_cbor(v)?) } },
            state: ValueState::from_wire(c.try_get(5)?.try_int()?)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueDiagnostic {
    pub severity: ValueSeverity,
    pub code: ValueDiagCode,
}
impl ValueDiagnostic {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.severity.wire())),
            (2, Cbor::Int(self.code.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            severity: ValueSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: ValueDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
        })
    }
}
