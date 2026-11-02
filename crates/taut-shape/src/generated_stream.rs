// Source: taut-shape/ir/shape_stream.taut.py; regenerate with scripts/regenerate_foundation_bindings.py
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
pub enum StreamMsgType {
    #[default] Push,
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
    pub fn wire(self) -> i64 { match self {
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
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
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
        _ => return Err(DecodeError::UnknownEnum { enum_name: "StreamMsgType", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamState {
    #[default] Data,
    WouldBlock,
    Eof,
    Closed,
    Failed,
    Dropped,
}
impl StreamState {
    pub fn wire(self) -> i64 { match self {
        Self::Data => 0,
        Self::WouldBlock => 1,
        Self::Eof => 2,
        Self::Closed => 3,
        Self::Failed => 4,
        Self::Dropped => 5,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Data,
        1 => Self::WouldBlock,
        2 => Self::Eof,
        3 => Self::Closed,
        4 => Self::Failed,
        5 => Self::Dropped,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "StreamState", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamErrorCode {
    #[default] UnknownStream,
    ProducerError,
    Internal,
    SlowConsumer,
}
impl StreamErrorCode {
    pub fn wire(self) -> i64 { match self {
        Self::UnknownStream => 0,
        Self::ProducerError => 1,
        Self::Internal => 2,
        Self::SlowConsumer => 3,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::UnknownStream,
        1 => Self::ProducerError,
        2 => Self::Internal,
        3 => Self::SlowConsumer,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "StreamErrorCode", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamStopReason {
    #[default] LastReaderGone,
    Closed,
    Failed,
}
impl StreamStopReason {
    pub fn wire(self) -> i64 { match self {
        Self::LastReaderGone => 0,
        Self::Closed => 1,
        Self::Failed => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::LastReaderGone,
        1 => Self::Closed,
        2 => Self::Failed,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "StreamStopReason", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamSeverity {
    #[default] Warn,
    Error,
}
impl StreamSeverity {
    pub fn wire(self) -> i64 { match self {
        Self::Warn => 0,
        Self::Error => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Warn,
        1 => Self::Error,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "StreamSeverity", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StreamDiagCode {
    #[default] PushAfterTerminal,
}
impl StreamDiagCode {
    pub fn wire(self) -> i64 { match self {
        Self::PushAfterTerminal => 0,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::PushAfterTerminal,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "StreamDiagCode", value: v }),
    }) }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamPosition {
    pub seq: i64,
}
impl StreamPosition {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.seq)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            seq: c.try_get(1)?.try_int()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamRecord {
    pub seq: i64,
    pub payload: Vec<u8>,
}
impl StreamRecord {
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
pub struct StreamError {
    pub code: StreamErrorCode,
    pub message: Option<String>,
}
impl StreamError {
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
            code: StreamErrorCode::from_wire(c.try_get(1)?.try_int()?)?,
            message: { let v = c.try_get(2)?; if v.is_null() { None } else { Some(v.try_text()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamPush {
    pub payload: Vec<u8>,
}
impl StreamPush {
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
pub struct StreamSeal {
}
impl StreamSeal {
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
pub struct StreamClose {
    pub error: Option<StreamError>,
}
impl StreamClose {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            error: { let v = c.try_get(1)?; if v.is_null() { None } else { Some(StreamError::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
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
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.stream_id.clone())),
            (2, match &self.max_records { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (3, match &self.max_bytes { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (4, match &self.timeout_ms { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            stream_id: c.try_get(1)?.try_text()?,
            max_records: { let v = c.try_get(2)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            max_bytes: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            timeout_ms: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_int()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamEndStream {
    pub stream_id: String,
}
impl StreamEndStream {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.stream_id.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            stream_id: c.try_get(1)?.try_text()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamTimerExpired {
    pub token: i64,
}
impl StreamTimerExpired {
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
pub struct StreamReadResponse {
    pub stream_id: String,
    pub records: Vec<StreamRecord>,
    pub next_position: StreamPosition,
    pub state: StreamState,
    pub error: Option<StreamError>,
}
impl StreamReadResponse {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.stream_id.clone())),
            (2, Cbor::Array(self.records.iter().map(|x| x.to_cbor()).collect())),
            (3, self.next_position.to_cbor()),
            (4, Cbor::Int(self.state.wire())),
            (5, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            stream_id: c.try_get(1)?.try_text()?,
            records: c.try_get(2)?.try_array()?.iter().map(|x| StreamRecord::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
            next_position: StreamPosition::from_cbor(c.try_get(3)?)?,
            state: StreamState::from_wire(c.try_get(4)?.try_int()?)?,
            error: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(StreamError::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamSetTimer {
    pub token: i64,
    pub ms: i64,
}
impl StreamSetTimer {
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
pub struct StreamCancelTimer {
    pub token: i64,
}
impl StreamCancelTimer {
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
pub struct StreamProducerStop {
    pub reason: StreamStopReason,
}
impl StreamProducerStop {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.reason.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            reason: StreamStopReason::from_wire(c.try_get(1)?.try_int()?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamDiagnostic {
    pub severity: StreamSeverity,
    pub code: StreamDiagCode,
}
impl StreamDiagnostic {
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
            severity: StreamSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: StreamDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
// Generator build rust-oneof-reference.1; descriptor=1; codec=1; owner=taut.rust.foundation
const _: () = assert!(crate::cbor::descriptors::DESCRIPTOR_CONTRACT == 1, "Taut descriptor contract mismatch: rebind runtime");
const _: () = assert!(crate::cbor::descriptors::CODEC_CONTRACT == 1, "Taut codec contract mismatch: rebind runtime");
impl StreamMsgType {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "65fb668de57455877c323c8cfa8fd2af6824a98b977621b6898b5520de1e208c";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_StreamMsgType; descriptor.check(); descriptor }
}
impl StreamState {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "2c6b5bbea0f7001c0f95e6be543a9fce13a7261d1623f24be39a3be218108bcf";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_StreamState; descriptor.check(); descriptor }
}
impl StreamErrorCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "bd54c822862a9ef3fb5cddf699f77b7d9a457170fe157f8f073dfd3bbd4f0e53";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_StreamErrorCode; descriptor.check(); descriptor }
}
impl StreamStopReason {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "6ee21cda017497886e5b48cd79fba9c3885683e9edb7fd08cd3dd7eef1477c0a";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_StreamStopReason; descriptor.check(); descriptor }
}
impl StreamSeverity {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "26dc1fe47bbec0d4f1bcd55cfc338bf3b77b656a81d1d585794c8fb42ca72ea3";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_StreamSeverity; descriptor.check(); descriptor }
}
impl StreamDiagCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "ecd7d7c9a46f36e923662bfc92c0ded0f2076375291fcff71e7075e9cb076365";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_StreamDiagCode; descriptor.check(); descriptor }
}
impl StreamPosition {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "53c5621928e9c40e348abd0480d9246bef4e778ed99b90dbfeeb94107936e469";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamPosition.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamPosition }
}
impl StreamRecord {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "dbcea3fb2c89f10bf3e0ae694d343798e436e2406bb1636db78c61cfa93fb9a8";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamRecord.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamRecord }
}
impl StreamError {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "a3103738ac1a7b66203c7f07bc6115a4b12f26ff47dbb4cd55ac9537de652506";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamError.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamError }
}
impl StreamPush {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "bc409d38eaab942104e4b302f2501766cf0605c40d137f8c8d6276393a30950b";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamPush.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamPush }
}
impl StreamSeal {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "4f183b44f8f308e701410d5cc451eff6f9872964273af07220fec7cc7c133c8d";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamSeal.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamSeal }
}
impl StreamClose {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "718e1af88458dfb453af4e976bae1870e93506455c816fea7edda2d961214a12";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamClose.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamClose }
}
impl StreamReadRequest {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "2df9a42e30e83ff09f43b5d9ea2cf5ac115d2c07cbfbf466eff1457ec0b52241";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamReadRequest.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamReadRequest }
}
impl StreamEndStream {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "9d2fdcf9768e9c86432d56303d50f7f4b19fd42551026bce171f423b155f305e";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamEndStream.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamEndStream }
}
impl StreamTimerExpired {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "5486236adac4f8b1a04e79506e885fbf6e78235950304a410b885ed21f0d7cc4";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamTimerExpired.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamTimerExpired }
}
impl StreamReadResponse {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "fe4a0febae04c5c521339813420b3a2c27ad2a7115ae748421712520d6dc5428";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamReadResponse.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamReadResponse }
}
impl StreamSetTimer {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "7636c26f41cf730a2248b849cde231dfe1eb4d8e8fbfe8e7a6ca914744d6a4ff";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamSetTimer.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamSetTimer }
}
impl StreamCancelTimer {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "227ed15cdf9393c08c8e947bdae89dbc75f7a8724689386a3ae1a2582332d87f";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamCancelTimer.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamCancelTimer }
}
impl StreamProducerStop {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "55539ca5b4524063afed976b09494227d9cd11404d60522da5e35ef55d302f61";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamProducerStop.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamProducerStop }
}
impl StreamDiagnostic {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "95e7686c74c22e11ae06aab93155b27dd01d670848b017a358473224406dfe84";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_StreamDiagnostic.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_StreamDiagnostic }
}
pub static SCHEMA_DESCRIPTOR: &crate::cbor::descriptors::SchemaDescriptor = &crate::cbor::descriptors::SchemaDescriptor::new(1, 2, "schema_8157b3edfad6c6b6",
&[StreamPosition::descriptor, StreamRecord::descriptor, StreamError::descriptor, StreamPush::descriptor, StreamSeal::descriptor, StreamClose::descriptor, StreamReadRequest::descriptor, StreamEndStream::descriptor, StreamTimerExpired::descriptor, StreamReadResponse::descriptor, StreamSetTimer::descriptor, StreamCancelTimer::descriptor, StreamProducerStop::descriptor, StreamDiagnostic::descriptor],
&[StreamMsgType::descriptor, StreamState::descriptor, StreamErrorCode::descriptor, StreamStopReason::descriptor, StreamSeverity::descriptor, StreamDiagCode::descriptor],
_taut_metadata::FILE_OPTIONS, _taut_metadata::FILE_EFFECTIVE);
#[allow(non_upper_case_globals, unused_variables, unused_imports)]
mod _taut_metadata {
use super::*;
use crate::cbor::descriptors::*;
pub const FILE_OPTIONS: Options = &[];
pub const FILE_EFFECTIVE: Options = &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)];
pub fn check_external() {
require_contract("schema_8157b3edfad6c6b6", "taut.rust.foundation", 1, 1, &["descriptors.v1", "codec.v1"]);
}
pub static ENUM_StreamMsgType: &EnumDescriptor = &EnumDescriptor::new("StreamMsgType", "schema_8157b3edfad6c6b6.StreamMsgType", &[EnumMemberDescriptor::new("push", 0, &[]), EnumMemberDescriptor::new("seal", 1, &[]), EnumMemberDescriptor::new("close", 2, &[]), EnumMemberDescriptor::new("read", 3, &[]), EnumMemberDescriptor::new("end_stream", 4, &[]), EnumMemberDescriptor::new("timer_expired", 5, &[]), EnumMemberDescriptor::new("read_response", 6, &[]), EnumMemberDescriptor::new("set_timer", 7, &[]), EnumMemberDescriptor::new("cancel_timer", 8, &[]), EnumMemberDescriptor::new("producer_stop", 9, &[]), EnumMemberDescriptor::new("diagnostic", 10, &[])], &[]);
pub static ENUM_StreamState: &EnumDescriptor = &EnumDescriptor::new("StreamState", "schema_8157b3edfad6c6b6.StreamState", &[EnumMemberDescriptor::new("data", 0, &[]), EnumMemberDescriptor::new("would_block", 1, &[]), EnumMemberDescriptor::new("eof", 2, &[]), EnumMemberDescriptor::new("closed", 3, &[]), EnumMemberDescriptor::new("failed", 4, &[]), EnumMemberDescriptor::new("dropped", 5, &[])], &[]);
pub static ENUM_StreamErrorCode: &EnumDescriptor = &EnumDescriptor::new("StreamErrorCode", "schema_8157b3edfad6c6b6.StreamErrorCode", &[EnumMemberDescriptor::new("unknown_stream", 0, &[]), EnumMemberDescriptor::new("producer_error", 1, &[]), EnumMemberDescriptor::new("internal", 2, &[]), EnumMemberDescriptor::new("slow_consumer", 3, &[])], &[]);
pub static ENUM_StreamStopReason: &EnumDescriptor = &EnumDescriptor::new("StreamStopReason", "schema_8157b3edfad6c6b6.StreamStopReason", &[EnumMemberDescriptor::new("last_reader_gone", 0, &[]), EnumMemberDescriptor::new("closed", 1, &[]), EnumMemberDescriptor::new("failed", 2, &[])], &[]);
pub static ENUM_StreamSeverity: &EnumDescriptor = &EnumDescriptor::new("StreamSeverity", "schema_8157b3edfad6c6b6.StreamSeverity", &[EnumMemberDescriptor::new("warn", 0, &[]), EnumMemberDescriptor::new("error", 1, &[])], &[]);
pub static ENUM_StreamDiagCode: &EnumDescriptor = &EnumDescriptor::new("StreamDiagCode", "schema_8157b3edfad6c6b6.StreamDiagCode", &[EnumMemberDescriptor::new("push_after_terminal", 0, &[])], &[]);
pub static BINDING_StreamPosition: &SchemaBinding<StreamPosition> = &SchemaBinding::new(MessageDescriptor::new("StreamPosition", "schema_8157b3edfad6c6b6.StreamPosition", 1, &[
FieldDescriptor::new("seq", Some("seq"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.seq)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_StreamRecord: &SchemaBinding<StreamRecord> = &SchemaBinding::new(MessageDescriptor::new("StreamRecord", "schema_8157b3edfad6c6b6.StreamRecord", 1, &[
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
pub static BINDING_StreamError: &SchemaBinding<StreamError> = &SchemaBinding::new(MessageDescriptor::new("StreamError", "schema_8157b3edfad6c6b6.StreamError", 1, &[
FieldDescriptor::new("code", Some("code"), 1, TypeDescriptor::Enum(StreamErrorCode::descriptor), "required", false, false, &[]),
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
pub static BINDING_StreamPush: &SchemaBinding<StreamPush> = &SchemaBinding::new(MessageDescriptor::new("StreamPush", "schema_8157b3edfad6c6b6.StreamPush", 1, &[
FieldDescriptor::new("payload", Some("payload"), 1, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_StreamSeal: &SchemaBinding<StreamSeal> = &SchemaBinding::new(MessageDescriptor::new("StreamSeal", "schema_8157b3edfad6c6b6.StreamSeal", 1, &[
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
panic!("Taut typed binding field mismatch: regenerate artifact")
}, |value, index, visitor| {
true });
pub static BINDING_StreamClose: &SchemaBinding<StreamClose> = &SchemaBinding::new(MessageDescriptor::new("StreamClose", "schema_8157b3edfad6c6b6.StreamClose", 1, &[
FieldDescriptor::new("error", Some("error"), 1, TypeDescriptor::Message(StreamError::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(value.error.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 0 {
if let Some(child) = &value.error {
if !visitor.record(&StreamError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_StreamReadRequest: &SchemaBinding<StreamReadRequest> = &SchemaBinding::new(MessageDescriptor::new("StreamReadRequest", "schema_8157b3edfad6c6b6.StreamReadRequest", 1, &[
FieldDescriptor::new("stream_id", Some("stream_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("max_records", Some("max_records"), 2, TypeDescriptor::Int, "nullable", false, false, &[]),
FieldDescriptor::new("max_bytes", Some("max_bytes"), 3, TypeDescriptor::Int, "nullable", false, false, &[]),
FieldDescriptor::new("timeout_ms", Some("timeout_ms"), 4, TypeDescriptor::Int, "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
1 => match &value.max_records { Some(v) => FieldState::from_primitive(PrimitiveValue::Int(*v)), None => FieldState::new(false, None) },
2 => match &value.max_bytes { Some(v) => FieldState::from_primitive(PrimitiveValue::Int(*v)), None => FieldState::new(false, None) },
3 => match &value.timeout_ms { Some(v) => FieldState::from_primitive(PrimitiveValue::Int(*v)), None => FieldState::new(false, None) },
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_StreamEndStream: &SchemaBinding<StreamEndStream> = &SchemaBinding::new(MessageDescriptor::new("StreamEndStream", "schema_8157b3edfad6c6b6.StreamEndStream", 1, &[
FieldDescriptor::new("stream_id", Some("stream_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_StreamTimerExpired: &SchemaBinding<StreamTimerExpired> = &SchemaBinding::new(MessageDescriptor::new("StreamTimerExpired", "schema_8157b3edfad6c6b6.StreamTimerExpired", 1, &[
FieldDescriptor::new("token", Some("token"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.token)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_StreamReadResponse: &SchemaBinding<StreamReadResponse> = &SchemaBinding::new(MessageDescriptor::new("StreamReadResponse", "schema_8157b3edfad6c6b6.StreamReadResponse", 1, &[
FieldDescriptor::new("stream_id", Some("stream_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("records", Some("records"), 2, TypeDescriptor::List(&TypeDescriptor::Message(StreamRecord::descriptor)), "required", false, false, &[]),
FieldDescriptor::new("next_position", Some("next_position"), 3, TypeDescriptor::Message(StreamPosition::descriptor), "required", false, false, &[]),
FieldDescriptor::new("state", Some("state"), 4, TypeDescriptor::Enum(StreamState::descriptor), "required", false, false, &[]),
FieldDescriptor::new("error", Some("error"), 5, TypeDescriptor::Message(StreamError::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
1 => FieldState::new(true, None),
2 => FieldState::new(true, None),
3 => FieldState::new(true, Some(value.state.wire())),
4 => FieldState::new(value.error.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 1 {
for (index0, item0) in value.records.iter().enumerate() {
visitor.push_index(index0);
if !visitor.record(&StreamRecord::taut_binding().bind(item0)) { return false; }
visitor.pop();
}
}
if index == 2 {
if !visitor.record(&StreamPosition::taut_binding().bind(&value.next_position)) { return false; }
}
if index == 4 {
if let Some(child) = &value.error {
if !visitor.record(&StreamError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_StreamSetTimer: &SchemaBinding<StreamSetTimer> = &SchemaBinding::new(MessageDescriptor::new("StreamSetTimer", "schema_8157b3edfad6c6b6.StreamSetTimer", 1, &[
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
pub static BINDING_StreamCancelTimer: &SchemaBinding<StreamCancelTimer> = &SchemaBinding::new(MessageDescriptor::new("StreamCancelTimer", "schema_8157b3edfad6c6b6.StreamCancelTimer", 1, &[
FieldDescriptor::new("token", Some("token"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.token)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_StreamProducerStop: &SchemaBinding<StreamProducerStop> = &SchemaBinding::new(MessageDescriptor::new("StreamProducerStop", "schema_8157b3edfad6c6b6.StreamProducerStop", 1, &[
FieldDescriptor::new("reason", Some("reason"), 1, TypeDescriptor::Enum(StreamStopReason::descriptor), "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, Some(value.reason.wire())),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_StreamDiagnostic: &SchemaBinding<StreamDiagnostic> = &SchemaBinding::new(MessageDescriptor::new("StreamDiagnostic", "schema_8157b3edfad6c6b6.StreamDiagnostic", 1, &[
FieldDescriptor::new("severity", Some("severity"), 1, TypeDescriptor::Enum(StreamSeverity::descriptor), "required", false, false, &[]),
FieldDescriptor::new("code", Some("code"), 2, TypeDescriptor::Enum(StreamDiagCode::descriptor), "required", false, false, &[]),
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
