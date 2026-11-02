// Source: taut-shape/ir/shape_atom.taut.py; regenerate with scripts/regenerate_foundation_bindings.py
// GENERATED native Rust types + codec — do not edit.
#![cfg_attr(rustfmt, rustfmt::skip)]
#![allow(dead_code, clippy::redundant_closure, clippy::needless_question_mark, clippy::collapsible_if)]
extern crate alloc;
#[allow(unused_imports)]
use alloc::{string::String, vec, vec::Vec};
use crate::cbor::{Cbor, DecodeError};
// The file's bounds, for a decode rooted at a type that is not a message:
// `cbor::try_decode_with(bytes, MAX_DEPTH, MAX_ENCODED_LEN)`.
pub const MAX_DEPTH: usize = 32;
pub const MAX_ENCODED_LEN: Option<usize> = None;
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomMsgType {
    #[default] Replace,
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
    pub fn wire(self) -> i64 { match self {
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
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
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
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AtomMsgType", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomState {
    #[default] Data,
    WouldBlock,
    Eof,
    Closed,
    Failed,
}
impl AtomState {
    pub fn wire(self) -> i64 { match self {
        Self::Data => 0,
        Self::WouldBlock => 1,
        Self::Eof => 2,
        Self::Closed => 3,
        Self::Failed => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Data,
        1 => Self::WouldBlock,
        2 => Self::Eof,
        3 => Self::Closed,
        4 => Self::Failed,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AtomState", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomErrorCode {
    #[default] UnknownAtom,
    ProducerError,
    Internal,
}
impl AtomErrorCode {
    pub fn wire(self) -> i64 { match self {
        Self::UnknownAtom => 0,
        Self::ProducerError => 1,
        Self::Internal => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::UnknownAtom,
        1 => Self::ProducerError,
        2 => Self::Internal,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AtomErrorCode", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomStopReason {
    #[default] LastReaderGone,
    Closed,
    Failed,
}
impl AtomStopReason {
    pub fn wire(self) -> i64 { match self {
        Self::LastReaderGone => 0,
        Self::Closed => 1,
        Self::Failed => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::LastReaderGone,
        1 => Self::Closed,
        2 => Self::Failed,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AtomStopReason", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomSeverity {
    #[default] Warn,
    Error,
}
impl AtomSeverity {
    pub fn wire(self) -> i64 { match self {
        Self::Warn => 0,
        Self::Error => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Warn,
        1 => Self::Error,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AtomSeverity", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AtomDiagCode {
    #[default] ReplaceAfterTerminal,
}
impl AtomDiagCode {
    pub fn wire(self) -> i64 { match self {
        Self::ReplaceAfterTerminal => 0,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::ReplaceAfterTerminal,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AtomDiagCode", value: v }),
    }) }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomVersion {
    pub version: i64,
}
impl AtomVersion {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.version)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            version: c.try_get(1)?.try_int()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomValue {
    pub version: i64,
    pub payload: Vec<u8>,
}
impl AtomValue {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
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
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomError {
    pub code: AtomErrorCode,
    pub message: Option<String>,
}
impl AtomError {
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
            code: AtomErrorCode::from_wire(c.try_get(1)?.try_int()?)?,
            message: { let v = c.try_get(2)?; if v.is_null() { None } else { Some(v.try_text()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomReplace {
    pub payload: Vec<u8>,
}
impl AtomReplace {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            payload: c.try_get(1)?.try_bytes()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomSeal {
}
impl AtomSeal {
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
pub struct AtomClose {
    pub error: Option<AtomError>,
}
impl AtomClose {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            error: { let v = c.try_get(1)?; if v.is_null() { None } else { Some(AtomError::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
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
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.atom_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, match &self.version { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (4, match &self.timeout_ms { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            atom_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            version: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(AtomVersion::from_cbor(v)?) } },
            timeout_ms: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_int()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomEndStream {
    pub atom_id: String,
    pub stream_id: String,
}
impl AtomEndStream {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
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
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomTimerExpired {
    pub token: i64,
}
impl AtomTimerExpired {
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
pub struct AtomReadResponse {
    pub atom_id: String,
    pub stream_id: String,
    pub value: Option<AtomValue>,
    pub next_version: AtomVersion,
    pub state: AtomState,
    pub error: Option<AtomError>,
}
impl AtomReadResponse {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.atom_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, match &self.value { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (4, self.next_version.to_cbor()),
            (5, Cbor::Int(self.state.wire())),
            (6, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            atom_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            value: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(AtomValue::from_cbor(v)?) } },
            next_version: AtomVersion::from_cbor(c.try_get(4)?)?,
            state: AtomState::from_wire(c.try_get(5)?.try_int()?)?,
            error: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(AtomError::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomSetTimer {
    pub token: i64,
    pub ms: i64,
}
impl AtomSetTimer {
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
pub struct AtomCancelTimer {
    pub token: i64,
}
impl AtomCancelTimer {
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
pub struct AtomProducerStop {
    pub reason: AtomStopReason,
}
impl AtomProducerStop {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.reason.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            reason: AtomStopReason::from_wire(c.try_get(1)?.try_int()?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AtomDiagnostic {
    pub severity: AtomSeverity,
    pub code: AtomDiagCode,
}
impl AtomDiagnostic {
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
            severity: AtomSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: AtomDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
// Generator build rust-oneof-reference.1; descriptor=1; codec=1; owner=taut.rust.foundation
const _: () = assert!(crate::cbor::descriptors::DESCRIPTOR_CONTRACT == 1, "Taut descriptor contract mismatch: rebind runtime");
const _: () = assert!(crate::cbor::descriptors::CODEC_CONTRACT == 1, "Taut codec contract mismatch: rebind runtime");
impl AtomMsgType {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "b7d2dc3df58aafc972fee368603903c5d9aadc74eee75f2087543f409a78516b";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_AtomMsgType; descriptor.check(); descriptor }
}
impl AtomState {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "2287bbfd23d8c141d802a3e28d74a344f71da2740ebdd6b4f4a4a80b81c89f91";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_AtomState; descriptor.check(); descriptor }
}
impl AtomErrorCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "449381a43adb5fe5b273289a34fda6af5f4441bdbc77d0935ea98b2963f2a3e1";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_AtomErrorCode; descriptor.check(); descriptor }
}
impl AtomStopReason {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "abe31222fc8540e9f24e34385201eef1ea5a14d32cfe0bf87ce975a4626427a4";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_AtomStopReason; descriptor.check(); descriptor }
}
impl AtomSeverity {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "c0487bdea37cb0dd13fdfecabf8a74f4d2dddbea2c6a5a0068c6348e4839449f";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_AtomSeverity; descriptor.check(); descriptor }
}
impl AtomDiagCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "2661770826cb1c6f34049a6cca19d441a77faf3e6f0c96e704ea78dec8db0a6f";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_AtomDiagCode; descriptor.check(); descriptor }
}
impl AtomVersion {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "c8f123f249d89514b435080c9bdaaa2b73503283295fb31415cc0fa1a047c39c";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomVersion.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomVersion }
}
impl AtomValue {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "b16359d6ab7b1a8f37ee85f0307598bfc5d6055fbdbdb20a3b211f662420b68c";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomValue.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomValue }
}
impl AtomError {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "45d68b0c870b0c27daf37d28a398a8e94d83a28092d34541bea43c506e3b75c2";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomError.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomError }
}
impl AtomReplace {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "07d0995a1d5a2ec1a663e9f8cb3dd4432a113fce840d875e7e16ad4f24dafb3b";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomReplace.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomReplace }
}
impl AtomSeal {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "089c29a0cb168b2fdb94869643b97e33c908f2b24534457353c1819df79c2a0e";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomSeal.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomSeal }
}
impl AtomClose {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "33f21b94c3bddbbc0551fabadbacb302c98cb60779ee5ff6a49b1112d54c6efb";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomClose.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomClose }
}
impl AtomReadRequest {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "63329ca2de59db38c56e593ae5b1fa65c074194ac7b8c1c2d16da8a1ac15c889";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomReadRequest.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomReadRequest }
}
impl AtomEndStream {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "ea55d64fc0aad0fb15adb2e48edf3fcd23720acef6c2b9660b9cff1acd755792";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomEndStream.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomEndStream }
}
impl AtomTimerExpired {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "6844092009f9a9177db0feb0b89329fc9202cd21c2fabb97e7f72f874c8c685f";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomTimerExpired.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomTimerExpired }
}
impl AtomReadResponse {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "cd3efdb7b39ff98e53aadb1af61c8f2148a2dbc79f027406ffb3996f1d40405f";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomReadResponse.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomReadResponse }
}
impl AtomSetTimer {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "1a3a003c3c972e59893dde2ebb6d30ed0c84f0073aeb797f981bfae7afe224ca";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomSetTimer.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomSetTimer }
}
impl AtomCancelTimer {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "e9b78133ab7390883c571cfb68dce857479f4f38ff8af6d29cb28821ba3f1b6c";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomCancelTimer.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomCancelTimer }
}
impl AtomProducerStop {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "615ab90494d5d775c6c50090be002bfae05a7cecf20a719dc18d301d729eb1bb";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomProducerStop.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomProducerStop }
}
impl AtomDiagnostic {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "5fd92706e4fc68fa54d0b2a94ee451bb62145b1d1adac114d319298c05cfd1e9";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_AtomDiagnostic.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_AtomDiagnostic }
}
pub static SCHEMA_DESCRIPTOR: &crate::cbor::descriptors::SchemaDescriptor = &crate::cbor::descriptors::SchemaDescriptor::new(1, 2, "schema_8e89e747dad3b048",
&[AtomVersion::descriptor, AtomValue::descriptor, AtomError::descriptor, AtomReplace::descriptor, AtomSeal::descriptor, AtomClose::descriptor, AtomReadRequest::descriptor, AtomEndStream::descriptor, AtomTimerExpired::descriptor, AtomReadResponse::descriptor, AtomSetTimer::descriptor, AtomCancelTimer::descriptor, AtomProducerStop::descriptor, AtomDiagnostic::descriptor],
&[AtomMsgType::descriptor, AtomState::descriptor, AtomErrorCode::descriptor, AtomStopReason::descriptor, AtomSeverity::descriptor, AtomDiagCode::descriptor],
_taut_metadata::FILE_OPTIONS, _taut_metadata::FILE_EFFECTIVE);
#[allow(non_upper_case_globals, unused_variables, unused_imports)]
mod _taut_metadata {
use super::*;
use crate::cbor::descriptors::*;
pub const FILE_OPTIONS: Options = &[];
pub const FILE_EFFECTIVE: Options = &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)];
pub fn check_external() {
require_contract("schema_8e89e747dad3b048", "taut.rust.foundation", 1, 1, &["descriptors.v1", "codec.v1"]);
}
pub static ENUM_AtomMsgType: &EnumDescriptor = &EnumDescriptor::new("AtomMsgType", "schema_8e89e747dad3b048.AtomMsgType", &[EnumMemberDescriptor::new("replace", 0, &[]), EnumMemberDescriptor::new("seal", 1, &[]), EnumMemberDescriptor::new("close", 2, &[]), EnumMemberDescriptor::new("read", 3, &[]), EnumMemberDescriptor::new("end_stream", 4, &[]), EnumMemberDescriptor::new("timer_expired", 5, &[]), EnumMemberDescriptor::new("read_response", 6, &[]), EnumMemberDescriptor::new("set_timer", 7, &[]), EnumMemberDescriptor::new("cancel_timer", 8, &[]), EnumMemberDescriptor::new("producer_stop", 9, &[]), EnumMemberDescriptor::new("diagnostic", 10, &[])], &[]);
pub static ENUM_AtomState: &EnumDescriptor = &EnumDescriptor::new("AtomState", "schema_8e89e747dad3b048.AtomState", &[EnumMemberDescriptor::new("data", 0, &[]), EnumMemberDescriptor::new("would_block", 1, &[]), EnumMemberDescriptor::new("eof", 2, &[]), EnumMemberDescriptor::new("closed", 3, &[]), EnumMemberDescriptor::new("failed", 4, &[])], &[]);
pub static ENUM_AtomErrorCode: &EnumDescriptor = &EnumDescriptor::new("AtomErrorCode", "schema_8e89e747dad3b048.AtomErrorCode", &[EnumMemberDescriptor::new("unknown_atom", 0, &[]), EnumMemberDescriptor::new("producer_error", 1, &[]), EnumMemberDescriptor::new("internal", 2, &[])], &[]);
pub static ENUM_AtomStopReason: &EnumDescriptor = &EnumDescriptor::new("AtomStopReason", "schema_8e89e747dad3b048.AtomStopReason", &[EnumMemberDescriptor::new("last_reader_gone", 0, &[]), EnumMemberDescriptor::new("closed", 1, &[]), EnumMemberDescriptor::new("failed", 2, &[])], &[]);
pub static ENUM_AtomSeverity: &EnumDescriptor = &EnumDescriptor::new("AtomSeverity", "schema_8e89e747dad3b048.AtomSeverity", &[EnumMemberDescriptor::new("warn", 0, &[]), EnumMemberDescriptor::new("error", 1, &[])], &[]);
pub static ENUM_AtomDiagCode: &EnumDescriptor = &EnumDescriptor::new("AtomDiagCode", "schema_8e89e747dad3b048.AtomDiagCode", &[EnumMemberDescriptor::new("replace_after_terminal", 0, &[])], &[]);
pub static BINDING_AtomVersion: &SchemaBinding<AtomVersion> = &SchemaBinding::new(MessageDescriptor::new("AtomVersion", "schema_8e89e747dad3b048.AtomVersion", 1, &[
FieldDescriptor::new("version", Some("version"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.version)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_AtomValue: &SchemaBinding<AtomValue> = &SchemaBinding::new(MessageDescriptor::new("AtomValue", "schema_8e89e747dad3b048.AtomValue", 1, &[
FieldDescriptor::new("version", Some("version"), 1, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("payload", Some("payload"), 2, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.version)),
1 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_AtomError: &SchemaBinding<AtomError> = &SchemaBinding::new(MessageDescriptor::new("AtomError", "schema_8e89e747dad3b048.AtomError", 1, &[
FieldDescriptor::new("code", Some("code"), 1, TypeDescriptor::Enum(AtomErrorCode::descriptor), "required", false, false, &[]),
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
pub static BINDING_AtomReplace: &SchemaBinding<AtomReplace> = &SchemaBinding::new(MessageDescriptor::new("AtomReplace", "schema_8e89e747dad3b048.AtomReplace", 1, &[
FieldDescriptor::new("payload", Some("payload"), 1, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_AtomSeal: &SchemaBinding<AtomSeal> = &SchemaBinding::new(MessageDescriptor::new("AtomSeal", "schema_8e89e747dad3b048.AtomSeal", 1, &[
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
panic!("Taut typed binding field mismatch: regenerate artifact")
}, |value, index, visitor| {
true });
pub static BINDING_AtomClose: &SchemaBinding<AtomClose> = &SchemaBinding::new(MessageDescriptor::new("AtomClose", "schema_8e89e747dad3b048.AtomClose", 1, &[
FieldDescriptor::new("error", Some("error"), 1, TypeDescriptor::Message(AtomError::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(value.error.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 0 {
if let Some(child) = &value.error {
if !visitor.record(&AtomError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_AtomReadRequest: &SchemaBinding<AtomReadRequest> = &SchemaBinding::new(MessageDescriptor::new("AtomReadRequest", "schema_8e89e747dad3b048.AtomReadRequest", 1, &[
FieldDescriptor::new("atom_id", Some("atom_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("version", Some("version"), 3, TypeDescriptor::Message(AtomVersion::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("timeout_ms", Some("timeout_ms"), 4, TypeDescriptor::Int, "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.atom_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
2 => FieldState::new(value.version.is_some(), None),
3 => match &value.timeout_ms { Some(v) => FieldState::from_primitive(PrimitiveValue::Int(*v)), None => FieldState::new(false, None) },
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 2 {
if let Some(child) = &value.version {
if !visitor.record(&AtomVersion::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_AtomEndStream: &SchemaBinding<AtomEndStream> = &SchemaBinding::new(MessageDescriptor::new("AtomEndStream", "schema_8e89e747dad3b048.AtomEndStream", 1, &[
FieldDescriptor::new("atom_id", Some("atom_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.atom_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_AtomTimerExpired: &SchemaBinding<AtomTimerExpired> = &SchemaBinding::new(MessageDescriptor::new("AtomTimerExpired", "schema_8e89e747dad3b048.AtomTimerExpired", 1, &[
FieldDescriptor::new("token", Some("token"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.token)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_AtomReadResponse: &SchemaBinding<AtomReadResponse> = &SchemaBinding::new(MessageDescriptor::new("AtomReadResponse", "schema_8e89e747dad3b048.AtomReadResponse", 1, &[
FieldDescriptor::new("atom_id", Some("atom_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("value", Some("value"), 3, TypeDescriptor::Message(AtomValue::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("next_version", Some("next_version"), 4, TypeDescriptor::Message(AtomVersion::descriptor), "required", false, false, &[]),
FieldDescriptor::new("state", Some("state"), 5, TypeDescriptor::Enum(AtomState::descriptor), "required", false, false, &[]),
FieldDescriptor::new("error", Some("error"), 6, TypeDescriptor::Message(AtomError::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.atom_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
2 => FieldState::new(value.value.is_some(), None),
3 => FieldState::new(true, None),
4 => FieldState::new(true, Some(value.state.wire())),
5 => FieldState::new(value.error.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 2 {
if let Some(child) = &value.value {
if !visitor.record(&AtomValue::taut_binding().bind(child)) { return false; }
}
}
if index == 3 {
if !visitor.record(&AtomVersion::taut_binding().bind(&value.next_version)) { return false; }
}
if index == 5 {
if let Some(child) = &value.error {
if !visitor.record(&AtomError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_AtomSetTimer: &SchemaBinding<AtomSetTimer> = &SchemaBinding::new(MessageDescriptor::new("AtomSetTimer", "schema_8e89e747dad3b048.AtomSetTimer", 1, &[
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
pub static BINDING_AtomCancelTimer: &SchemaBinding<AtomCancelTimer> = &SchemaBinding::new(MessageDescriptor::new("AtomCancelTimer", "schema_8e89e747dad3b048.AtomCancelTimer", 1, &[
FieldDescriptor::new("token", Some("token"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.token)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_AtomProducerStop: &SchemaBinding<AtomProducerStop> = &SchemaBinding::new(MessageDescriptor::new("AtomProducerStop", "schema_8e89e747dad3b048.AtomProducerStop", 1, &[
FieldDescriptor::new("reason", Some("reason"), 1, TypeDescriptor::Enum(AtomStopReason::descriptor), "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, Some(value.reason.wire())),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_AtomDiagnostic: &SchemaBinding<AtomDiagnostic> = &SchemaBinding::new(MessageDescriptor::new("AtomDiagnostic", "schema_8e89e747dad3b048.AtomDiagnostic", 1, &[
FieldDescriptor::new("severity", Some("severity"), 1, TypeDescriptor::Enum(AtomSeverity::descriptor), "required", false, false, &[]),
FieldDescriptor::new("code", Some("code"), 2, TypeDescriptor::Enum(AtomDiagCode::descriptor), "required", false, false, &[]),
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
