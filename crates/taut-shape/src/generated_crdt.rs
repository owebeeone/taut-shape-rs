// Source: taut-shape/ir/shape_crdt.taut.py; regenerate with scripts/regenerate_foundation_bindings.py
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
pub enum CrdtMsgType {
    #[default] Apply,
    InstallBootstrap,
    Seal,
    Close,
    Read,
    ReadResponse,
    Diagnostic,
}
impl CrdtMsgType {
    pub fn wire(self) -> i64 { match self {
        Self::Apply => 0,
        Self::InstallBootstrap => 1,
        Self::Seal => 2,
        Self::Close => 3,
        Self::Read => 4,
        Self::ReadResponse => 5,
        Self::Diagnostic => 6,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Apply,
        1 => Self::InstallBootstrap,
        2 => Self::Seal,
        3 => Self::Close,
        4 => Self::Read,
        5 => Self::ReadResponse,
        6 => Self::Diagnostic,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "CrdtMsgType", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CrdtState {
    #[default] Data,
    Empty,
    Eof,
    Closed,
    Failed,
    BootstrapRequired,
    InvalidCursor,
}
impl CrdtState {
    pub fn wire(self) -> i64 { match self {
        Self::Data => 0,
        Self::Empty => 1,
        Self::Eof => 2,
        Self::Closed => 3,
        Self::Failed => 4,
        Self::BootstrapRequired => 5,
        Self::InvalidCursor => 6,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Data,
        1 => Self::Empty,
        2 => Self::Eof,
        3 => Self::Closed,
        4 => Self::Failed,
        5 => Self::BootstrapRequired,
        6 => Self::InvalidCursor,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "CrdtState", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CrdtErrorCode {
    #[default] UnknownCrdt,
    ProducerError,
    Internal,
}
impl CrdtErrorCode {
    pub fn wire(self) -> i64 { match self {
        Self::UnknownCrdt => 0,
        Self::ProducerError => 1,
        Self::Internal => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::UnknownCrdt,
        1 => Self::ProducerError,
        2 => Self::Internal,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "CrdtErrorCode", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CrdtSeverity {
    #[default] Warn,
    Error,
}
impl CrdtSeverity {
    pub fn wire(self) -> i64 { match self {
        Self::Warn => 0,
        Self::Error => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Warn,
        1 => Self::Error,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "CrdtSeverity", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CrdtDiagCode {
    #[default] ApplyAfterTerminal,
    InvalidOperation,
    Equivocation,
    BootstrapConflict,
    PendingBoundExceeded,
}
impl CrdtDiagCode {
    pub fn wire(self) -> i64 { match self {
        Self::ApplyAfterTerminal => 0,
        Self::InvalidOperation => 1,
        Self::Equivocation => 2,
        Self::BootstrapConflict => 3,
        Self::PendingBoundExceeded => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::ApplyAfterTerminal,
        1 => Self::InvalidOperation,
        2 => Self::Equivocation,
        3 => Self::BootstrapConflict,
        4 => Self::PendingBoundExceeded,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "CrdtDiagCode", value: v }),
    }) }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtClockEntry {
    pub origin: String,
    pub seq: i64,
}
impl CrdtClockEntry {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
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
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtClock {
    pub entries: Vec<CrdtClockEntry>,
}
impl CrdtClock {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Array(self.entries.iter().map(|x| x.to_cbor()).collect())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            entries: c.try_get(1)?.try_array()?.iter().map(|x| CrdtClockEntry::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
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
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
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
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtBootstrap {
    pub clock: CrdtClock,
    pub state: Vec<u8>,
}
impl CrdtBootstrap {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
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
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtError {
    pub code: CrdtErrorCode,
    pub message: Option<String>,
}
impl CrdtError {
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
            code: CrdtErrorCode::from_wire(c.try_get(1)?.try_int()?)?,
            message: { let v = c.try_get(2)?; if v.is_null() { None } else { Some(v.try_text()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtApply {
    pub op: CrdtOp,
}
impl CrdtApply {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, self.op.to_cbor()),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            op: CrdtOp::from_cbor(c.try_get(1)?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtInstallBootstrap {
    pub bootstrap: CrdtBootstrap,
}
impl CrdtInstallBootstrap {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, self.bootstrap.to_cbor()),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            bootstrap: CrdtBootstrap::from_cbor(c.try_get(1)?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtSeal {
}
impl CrdtSeal {
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
pub struct CrdtClose {
    pub error: Option<CrdtError>,
}
impl CrdtClose {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            error: { let v = c.try_get(1)?; if v.is_null() { None } else { Some(CrdtError::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CrdtReadRequest {
    pub crdt_id: String,
    pub stream_id: String,
    pub cursor: Option<CrdtClock>,
}
impl CrdtReadRequest {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.crdt_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, match &self.cursor { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            crdt_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            cursor: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(CrdtClock::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
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
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.crdt_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, match &self.bootstrap { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (4, Cbor::Array(self.ops.iter().map(|x| x.to_cbor()).collect())),
            (5, self.next_cursor.to_cbor()),
            (6, Cbor::Int(self.state.wire())),
            (7, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            crdt_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            bootstrap: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(CrdtBootstrap::from_cbor(v)?) } },
            ops: c.try_get(4)?.try_array()?.iter().map(|x| CrdtOp::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
            next_cursor: CrdtClock::from_cbor(c.try_get(5)?)?,
            state: CrdtState::from_wire(c.try_get(6)?.try_int()?)?,
            error: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(CrdtError::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
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
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.severity.wire())),
            (2, Cbor::Int(self.code.wire())),
            (3, match &self.origin { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (4, match &self.seq { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            severity: CrdtSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: CrdtDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
            origin: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            seq: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_int()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
// Generator build rust-oneof-reference.1; descriptor=1; codec=1; owner=taut.rust.foundation
const _: () = assert!(crate::cbor::descriptors::DESCRIPTOR_CONTRACT == 1, "Taut descriptor contract mismatch: rebind runtime");
const _: () = assert!(crate::cbor::descriptors::CODEC_CONTRACT == 1, "Taut codec contract mismatch: rebind runtime");
impl CrdtMsgType {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "9fe1d2722430b703fd7a56b44270714eca18f773bbbe01b2801aadacf3fe4fce";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_CrdtMsgType; descriptor.check(); descriptor }
}
impl CrdtState {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "30fde9df83a7463cc74f9dc09539e1b8dab34c0945cdd77b7a4fa8349394b0ac";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_CrdtState; descriptor.check(); descriptor }
}
impl CrdtErrorCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "86a4f236cb87eb83061bb4f4f2458fde7e619c4925df13789bb5bbb2a866b1eb";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_CrdtErrorCode; descriptor.check(); descriptor }
}
impl CrdtSeverity {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "dbb2960a58391603244a6ed756472b29787f69d6c8864f25845e3baae77d531e";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_CrdtSeverity; descriptor.check(); descriptor }
}
impl CrdtDiagCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "0f82455aed77b504987a8186dfcaad9a0ed86d0c102acd1f6247bc5e18c9e99e";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_CrdtDiagCode; descriptor.check(); descriptor }
}
impl CrdtClockEntry {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "11d3fe1e1b8acc93ec0cd3b22bb8ba2947a8e12a53849b6dbbb2617505925213";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtClockEntry.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtClockEntry }
}
impl CrdtClock {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "da012a03becb15fd9fe75a8f0b60ffc8572355d63c825abf1153226d050e0c6b";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtClock.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtClock }
}
impl CrdtOp {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "143cc0f5fa3f78adaf65783594c2c1732417b3e6eba004ba01d2771a00eba507";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtOp.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtOp }
}
impl CrdtBootstrap {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "25213c04dcd6ef1aa97863aa71c2915babcfb448e6f4f89388595b0c777454ab";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtBootstrap.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtBootstrap }
}
impl CrdtError {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "b3efc50d4fe460ec10c33475b77750fdce6f40a3d1a87ddb5049dfbd0fc6730d";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtError.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtError }
}
impl CrdtApply {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "368a63838515b66fc8508f2214f4cd534af432392e14b98d92a45764b09c1cad";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtApply.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtApply }
}
impl CrdtInstallBootstrap {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "d2c9933543051b824a76f5f9824a7b4e5691cd9e3e2d95498f54b6d560c03eb1";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtInstallBootstrap.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtInstallBootstrap }
}
impl CrdtSeal {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "5557a0f557673de8e4bd47df07880a93aee15f112df71c8820bd172ffa954538";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtSeal.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtSeal }
}
impl CrdtClose {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "f1d154e2376307e61296c114ff80c3117d643ca7c671ae9d3f10a018fbdcccfc";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtClose.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtClose }
}
impl CrdtReadRequest {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "1893944f10322207daada1660c3276f6f8de6948c8bd462f2183b39bdd58c21f";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtReadRequest.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtReadRequest }
}
impl CrdtReadResponse {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "8a38e91276526ac75a9d6609753747d9edaf7a365f21be199049ddb6d0914d9a";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtReadResponse.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtReadResponse }
}
impl CrdtDiagnostic {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "0fc361e790c3cff32ab2171d891f6e238eabc1a903cef12c11ea2a515db0e60c";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_CrdtDiagnostic.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_CrdtDiagnostic }
}
pub static SCHEMA_DESCRIPTOR: &crate::cbor::descriptors::SchemaDescriptor = &crate::cbor::descriptors::SchemaDescriptor::new(1, 2, "schema_5401f36f39b3d41a",
&[CrdtClockEntry::descriptor, CrdtClock::descriptor, CrdtOp::descriptor, CrdtBootstrap::descriptor, CrdtError::descriptor, CrdtApply::descriptor, CrdtInstallBootstrap::descriptor, CrdtSeal::descriptor, CrdtClose::descriptor, CrdtReadRequest::descriptor, CrdtReadResponse::descriptor, CrdtDiagnostic::descriptor],
&[CrdtMsgType::descriptor, CrdtState::descriptor, CrdtErrorCode::descriptor, CrdtSeverity::descriptor, CrdtDiagCode::descriptor],
_taut_metadata::FILE_OPTIONS, _taut_metadata::FILE_EFFECTIVE);
#[allow(non_upper_case_globals, unused_variables, unused_imports)]
mod _taut_metadata {
use super::*;
use crate::cbor::descriptors::*;
pub const FILE_OPTIONS: Options = &[];
pub const FILE_EFFECTIVE: Options = &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)];
pub fn check_external() {
require_contract("schema_5401f36f39b3d41a", "taut.rust.foundation", 1, 1, &["descriptors.v1", "codec.v1"]);
}
pub static ENUM_CrdtMsgType: &EnumDescriptor = &EnumDescriptor::new("CrdtMsgType", "schema_5401f36f39b3d41a.CrdtMsgType", &[EnumMemberDescriptor::new("apply", 0, &[]), EnumMemberDescriptor::new("install_bootstrap", 1, &[]), EnumMemberDescriptor::new("seal", 2, &[]), EnumMemberDescriptor::new("close", 3, &[]), EnumMemberDescriptor::new("read", 4, &[]), EnumMemberDescriptor::new("read_response", 5, &[]), EnumMemberDescriptor::new("diagnostic", 6, &[])], &[]);
pub static ENUM_CrdtState: &EnumDescriptor = &EnumDescriptor::new("CrdtState", "schema_5401f36f39b3d41a.CrdtState", &[EnumMemberDescriptor::new("data", 0, &[]), EnumMemberDescriptor::new("empty", 1, &[]), EnumMemberDescriptor::new("eof", 2, &[]), EnumMemberDescriptor::new("closed", 3, &[]), EnumMemberDescriptor::new("failed", 4, &[]), EnumMemberDescriptor::new("bootstrap_required", 5, &[]), EnumMemberDescriptor::new("invalid_cursor", 6, &[])], &[]);
pub static ENUM_CrdtErrorCode: &EnumDescriptor = &EnumDescriptor::new("CrdtErrorCode", "schema_5401f36f39b3d41a.CrdtErrorCode", &[EnumMemberDescriptor::new("unknown_crdt", 0, &[]), EnumMemberDescriptor::new("producer_error", 1, &[]), EnumMemberDescriptor::new("internal", 2, &[])], &[]);
pub static ENUM_CrdtSeverity: &EnumDescriptor = &EnumDescriptor::new("CrdtSeverity", "schema_5401f36f39b3d41a.CrdtSeverity", &[EnumMemberDescriptor::new("warn", 0, &[]), EnumMemberDescriptor::new("error", 1, &[])], &[]);
pub static ENUM_CrdtDiagCode: &EnumDescriptor = &EnumDescriptor::new("CrdtDiagCode", "schema_5401f36f39b3d41a.CrdtDiagCode", &[EnumMemberDescriptor::new("apply_after_terminal", 0, &[]), EnumMemberDescriptor::new("invalid_operation", 1, &[]), EnumMemberDescriptor::new("equivocation", 2, &[]), EnumMemberDescriptor::new("bootstrap_conflict", 3, &[]), EnumMemberDescriptor::new("pending_bound_exceeded", 4, &[])], &[]);
pub static BINDING_CrdtClockEntry: &SchemaBinding<CrdtClockEntry> = &SchemaBinding::new(MessageDescriptor::new("CrdtClockEntry", "schema_5401f36f39b3d41a.CrdtClockEntry", 1, &[
FieldDescriptor::new("origin", Some("origin"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("seq", Some("seq"), 2, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.origin)),
1 => FieldState::from_primitive(PrimitiveValue::Int(value.seq)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_CrdtClock: &SchemaBinding<CrdtClock> = &SchemaBinding::new(MessageDescriptor::new("CrdtClock", "schema_5401f36f39b3d41a.CrdtClock", 1, &[
FieldDescriptor::new("entries", Some("entries"), 1, TypeDescriptor::List(&TypeDescriptor::Message(CrdtClockEntry::descriptor)), "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 0 {
for (index0, item0) in value.entries.iter().enumerate() {
visitor.push_index(index0);
if !visitor.record(&CrdtClockEntry::taut_binding().bind(item0)) { return false; }
visitor.pop();
}
}
true });
pub static BINDING_CrdtOp: &SchemaBinding<CrdtOp> = &SchemaBinding::new(MessageDescriptor::new("CrdtOp", "schema_5401f36f39b3d41a.CrdtOp", 1, &[
FieldDescriptor::new("origin", Some("origin"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("seq", Some("seq"), 2, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("deps", Some("deps"), 3, TypeDescriptor::Message(CrdtClock::descriptor), "required", false, false, &[]),
FieldDescriptor::new("payload", Some("payload"), 4, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.origin)),
1 => FieldState::from_primitive(PrimitiveValue::Int(value.seq)),
2 => FieldState::new(true, None),
3 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 2 {
if !visitor.record(&CrdtClock::taut_binding().bind(&value.deps)) { return false; }
}
true });
pub static BINDING_CrdtBootstrap: &SchemaBinding<CrdtBootstrap> = &SchemaBinding::new(MessageDescriptor::new("CrdtBootstrap", "schema_5401f36f39b3d41a.CrdtBootstrap", 1, &[
FieldDescriptor::new("clock", Some("clock"), 1, TypeDescriptor::Message(CrdtClock::descriptor), "required", false, false, &[]),
FieldDescriptor::new("state", Some("state"), 2, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, None),
1 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.state)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 0 {
if !visitor.record(&CrdtClock::taut_binding().bind(&value.clock)) { return false; }
}
true });
pub static BINDING_CrdtError: &SchemaBinding<CrdtError> = &SchemaBinding::new(MessageDescriptor::new("CrdtError", "schema_5401f36f39b3d41a.CrdtError", 1, &[
FieldDescriptor::new("code", Some("code"), 1, TypeDescriptor::Enum(CrdtErrorCode::descriptor), "required", false, false, &[]),
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
pub static BINDING_CrdtApply: &SchemaBinding<CrdtApply> = &SchemaBinding::new(MessageDescriptor::new("CrdtApply", "schema_5401f36f39b3d41a.CrdtApply", 1, &[
FieldDescriptor::new("op", Some("op"), 1, TypeDescriptor::Message(CrdtOp::descriptor), "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 0 {
if !visitor.record(&CrdtOp::taut_binding().bind(&value.op)) { return false; }
}
true });
pub static BINDING_CrdtInstallBootstrap: &SchemaBinding<CrdtInstallBootstrap> = &SchemaBinding::new(MessageDescriptor::new("CrdtInstallBootstrap", "schema_5401f36f39b3d41a.CrdtInstallBootstrap", 1, &[
FieldDescriptor::new("bootstrap", Some("bootstrap"), 1, TypeDescriptor::Message(CrdtBootstrap::descriptor), "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 0 {
if !visitor.record(&CrdtBootstrap::taut_binding().bind(&value.bootstrap)) { return false; }
}
true });
pub static BINDING_CrdtSeal: &SchemaBinding<CrdtSeal> = &SchemaBinding::new(MessageDescriptor::new("CrdtSeal", "schema_5401f36f39b3d41a.CrdtSeal", 1, &[
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
panic!("Taut typed binding field mismatch: regenerate artifact")
}, |value, index, visitor| {
true });
pub static BINDING_CrdtClose: &SchemaBinding<CrdtClose> = &SchemaBinding::new(MessageDescriptor::new("CrdtClose", "schema_5401f36f39b3d41a.CrdtClose", 1, &[
FieldDescriptor::new("error", Some("error"), 1, TypeDescriptor::Message(CrdtError::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(value.error.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 0 {
if let Some(child) = &value.error {
if !visitor.record(&CrdtError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_CrdtReadRequest: &SchemaBinding<CrdtReadRequest> = &SchemaBinding::new(MessageDescriptor::new("CrdtReadRequest", "schema_5401f36f39b3d41a.CrdtReadRequest", 1, &[
FieldDescriptor::new("crdt_id", Some("crdt_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("cursor", Some("cursor"), 3, TypeDescriptor::Message(CrdtClock::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.crdt_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
2 => FieldState::new(value.cursor.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 2 {
if let Some(child) = &value.cursor {
if !visitor.record(&CrdtClock::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_CrdtReadResponse: &SchemaBinding<CrdtReadResponse> = &SchemaBinding::new(MessageDescriptor::new("CrdtReadResponse", "schema_5401f36f39b3d41a.CrdtReadResponse", 1, &[
FieldDescriptor::new("crdt_id", Some("crdt_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("bootstrap", Some("bootstrap"), 3, TypeDescriptor::Message(CrdtBootstrap::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("ops", Some("ops"), 4, TypeDescriptor::List(&TypeDescriptor::Message(CrdtOp::descriptor)), "required", false, false, &[]),
FieldDescriptor::new("next_cursor", Some("next_cursor"), 5, TypeDescriptor::Message(CrdtClock::descriptor), "required", false, false, &[]),
FieldDescriptor::new("state", Some("state"), 6, TypeDescriptor::Enum(CrdtState::descriptor), "required", false, false, &[]),
FieldDescriptor::new("error", Some("error"), 7, TypeDescriptor::Message(CrdtError::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.crdt_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
2 => FieldState::new(value.bootstrap.is_some(), None),
3 => FieldState::new(true, None),
4 => FieldState::new(true, None),
5 => FieldState::new(true, Some(value.state.wire())),
6 => FieldState::new(value.error.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 2 {
if let Some(child) = &value.bootstrap {
if !visitor.record(&CrdtBootstrap::taut_binding().bind(child)) { return false; }
}
}
if index == 3 {
for (index0, item0) in value.ops.iter().enumerate() {
visitor.push_index(index0);
if !visitor.record(&CrdtOp::taut_binding().bind(item0)) { return false; }
visitor.pop();
}
}
if index == 4 {
if !visitor.record(&CrdtClock::taut_binding().bind(&value.next_cursor)) { return false; }
}
if index == 6 {
if let Some(child) = &value.error {
if !visitor.record(&CrdtError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_CrdtDiagnostic: &SchemaBinding<CrdtDiagnostic> = &SchemaBinding::new(MessageDescriptor::new("CrdtDiagnostic", "schema_5401f36f39b3d41a.CrdtDiagnostic", 1, &[
FieldDescriptor::new("severity", Some("severity"), 1, TypeDescriptor::Enum(CrdtSeverity::descriptor), "required", false, false, &[]),
FieldDescriptor::new("code", Some("code"), 2, TypeDescriptor::Enum(CrdtDiagCode::descriptor), "required", false, false, &[]),
FieldDescriptor::new("origin", Some("origin"), 3, TypeDescriptor::String, "nullable", false, false, &[]),
FieldDescriptor::new("seq", Some("seq"), 4, TypeDescriptor::Int, "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, Some(value.severity.wire())),
1 => FieldState::new(true, Some(value.code.wire())),
2 => match &value.origin { Some(v) => FieldState::from_primitive(PrimitiveValue::String(v)), None => FieldState::new(false, None) },
3 => match &value.seq { Some(v) => FieldState::from_primitive(PrimitiveValue::Int(*v)), None => FieldState::new(false, None) },
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
}
