// Source: taut-shape/ir/shape_log.taut.py; regenerate with scripts/regenerate_foundation_bindings.py
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
pub enum LogMsgType {
    #[default] Push,
    Seal,
    Close,
    Read,
    EndStream,
    TimerExpired,
    Evict,
    ReadResponse,
    SetTimer,
    CancelTimer,
    ProducerStop,
    Diagnostic,
}
impl LogMsgType {
    pub fn wire(self) -> i64 { match self {
        Self::Push => 0,
        Self::Seal => 1,
        Self::Close => 2,
        Self::Read => 3,
        Self::EndStream => 4,
        Self::TimerExpired => 5,
        Self::Evict => 6,
        Self::ReadResponse => 7,
        Self::SetTimer => 8,
        Self::CancelTimer => 9,
        Self::ProducerStop => 10,
        Self::Diagnostic => 11,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Push,
        1 => Self::Seal,
        2 => Self::Close,
        3 => Self::Read,
        4 => Self::EndStream,
        5 => Self::TimerExpired,
        6 => Self::Evict,
        7 => Self::ReadResponse,
        8 => Self::SetTimer,
        9 => Self::CancelTimer,
        10 => Self::ProducerStop,
        11 => Self::Diagnostic,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "LogMsgType", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LogState {
    #[default] Data,
    WouldBlock,
    Eof,
    Closed,
    Failed,
    Expired,
}
impl LogState {
    pub fn wire(self) -> i64 { match self {
        Self::Data => 0,
        Self::WouldBlock => 1,
        Self::Eof => 2,
        Self::Closed => 3,
        Self::Failed => 4,
        Self::Expired => 5,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Data,
        1 => Self::WouldBlock,
        2 => Self::Eof,
        3 => Self::Closed,
        4 => Self::Failed,
        5 => Self::Expired,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "LogState", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LogErrorCode {
    #[default] UnknownLog,
    ProducerError,
    Internal,
}
impl LogErrorCode {
    pub fn wire(self) -> i64 { match self {
        Self::UnknownLog => 0,
        Self::ProducerError => 1,
        Self::Internal => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::UnknownLog,
        1 => Self::ProducerError,
        2 => Self::Internal,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "LogErrorCode", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LogStopReason {
    #[default] LastReaderGone,
    Closed,
    Failed,
}
impl LogStopReason {
    pub fn wire(self) -> i64 { match self {
        Self::LastReaderGone => 0,
        Self::Closed => 1,
        Self::Failed => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::LastReaderGone,
        1 => Self::Closed,
        2 => Self::Failed,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "LogStopReason", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LogSeverity {
    #[default] Warn,
    Error,
}
impl LogSeverity {
    pub fn wire(self) -> i64 { match self {
        Self::Warn => 0,
        Self::Error => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Warn,
        1 => Self::Error,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "LogSeverity", value: v }),
    }) }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LogDiagCode {
    #[default] PushAfterTerminal,
}
impl LogDiagCode {
    pub fn wire(self) -> i64 { match self {
        Self::PushAfterTerminal => 0,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::PushAfterTerminal,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "LogDiagCode", value: v }),
    }) }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogCursor {
    pub seq: i64,
}
impl LogCursor {
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
pub struct LogRecord {
    pub seq: i64,
    pub payload: Vec<u8>,
}
impl LogRecord {
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
pub struct LogError {
    pub code: LogErrorCode,
    pub message: Option<String>,
}
impl LogError {
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
            code: LogErrorCode::from_wire(c.try_get(1)?.try_int()?)?,
            message: { let v = c.try_get(2)?; if v.is_null() { None } else { Some(v.try_text()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogPush {
    pub payload: Vec<u8>,
}
impl LogPush {
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
pub struct LogSeal {
}
impl LogSeal {
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
pub struct LogClose {
    pub error: Option<LogError>,
}
impl LogClose {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            error: { let v = c.try_get(1)?; if v.is_null() { None } else { Some(LogError::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogReadRequest {
    pub log_id: String,
    pub stream_id: String,
    pub cursor: Option<LogCursor>,
    pub max_records: Option<i64>,
    pub max_bytes: Option<i64>,
    pub timeout_ms: Option<i64>,
}
impl LogReadRequest {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.log_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, match &self.cursor { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (4, match &self.max_records { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (5, match &self.max_bytes { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (6, match &self.timeout_ms { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            log_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            cursor: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(LogCursor::from_cbor(v)?) } },
            max_records: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            max_bytes: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            timeout_ms: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(v.try_int()?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogEndStream {
    pub log_id: String,
    pub stream_id: String,
}
impl LogEndStream {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.log_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            log_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogTimerExpired {
    pub token: i64,
}
impl LogTimerExpired {
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
pub struct LogEvict {
    pub up_to_seq: i64,
}
impl LogEvict {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.up_to_seq)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            up_to_seq: c.try_get(1)?.try_int()?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogReadResponse {
    pub log_id: String,
    pub stream_id: String,
    pub records: Vec<LogRecord>,
    pub next_cursor: LogCursor,
    pub state: LogState,
    pub error: Option<LogError>,
}
impl LogReadResponse {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.log_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, Cbor::Array(self.records.iter().map(|x| x.to_cbor()).collect())),
            (4, self.next_cursor.to_cbor()),
            (5, Cbor::Int(self.state.wire())),
            (6, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            log_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            records: c.try_get(3)?.try_array()?.iter().map(|x| LogRecord::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
            next_cursor: LogCursor::from_cbor(c.try_get(4)?)?,
            state: LogState::from_wire(c.try_get(5)?.try_int()?)?,
            error: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(LogError::from_cbor(v)?) } },
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogSetTimer {
    pub token: i64,
    pub ms: i64,
}
impl LogSetTimer {
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
pub struct LogCancelTimer {
    pub token: i64,
}
impl LogCancelTimer {
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
pub struct LogProducerStop {
    pub reason: LogStopReason,
}
impl LogProducerStop {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.reason.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            reason: LogStopReason::from_wire(c.try_get(1)?.try_int()?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogDiagnostic {
    pub severity: LogSeverity,
    pub code: LogDiagCode,
}
impl LogDiagnostic {
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
            severity: LogSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: LogDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
// Generator build rust-oneof-reference.1; descriptor=1; codec=1; owner=taut.rust.foundation
const _: () = assert!(crate::cbor::descriptors::DESCRIPTOR_CONTRACT == 1, "Taut descriptor contract mismatch: rebind runtime");
const _: () = assert!(crate::cbor::descriptors::CODEC_CONTRACT == 1, "Taut codec contract mismatch: rebind runtime");
impl LogMsgType {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "e36eb2f21e5fc54d9ed310b989720b43443b73e4fee8219c3000b4562108071c";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_LogMsgType; descriptor.check(); descriptor }
}
impl LogState {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "aeb72f31d9e878a1dc15d7afd543e381b22f6a314dfda56f87a7f8002fa76c87";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_LogState; descriptor.check(); descriptor }
}
impl LogErrorCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "01362ff2dc397c94918a76291b6a7733bf12b74e42bff460d7b3313324337f39";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_LogErrorCode; descriptor.check(); descriptor }
}
impl LogStopReason {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "d20533f9a56aebe6dbd064ecdbec800fcdec26d754529b6c4194ffa4446c2c3b";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_LogStopReason; descriptor.check(); descriptor }
}
impl LogSeverity {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "f9fc5607b32b0579589396ae0ffe9f4d761685bf61819c5b834e39d8543f2909";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_LogSeverity; descriptor.check(); descriptor }
}
impl LogDiagCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "045aae670820365ac3b99f1129a8480620a07421e86201205c15189c1fce64a4";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_LogDiagCode; descriptor.check(); descriptor }
}
impl LogCursor {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "e525234c99b176170d7ef4c106b1c6a33e1f239dab477578752341d4ec20fc4a";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogCursor.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogCursor }
}
impl LogRecord {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "24bd7d7bbfa0c271189320df3b72643bcc32afea09666f29559321a9201ede82";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogRecord.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogRecord }
}
impl LogError {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "620a9e454246995731939f072f13b502a34c76ca30016186cc2ffe72b75f6ef7";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogError.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogError }
}
impl LogPush {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "5a27a123718131434d156292dbac690519bcc1cb172e2c98b800ec55f88c77d3";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogPush.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogPush }
}
impl LogSeal {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "ed98d25458a375f51a27b45e63ae53ba5933ffaca89bb908bab9e818cb484bc2";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogSeal.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogSeal }
}
impl LogClose {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "1cc0f0d1c230eb79dd8660ac86cb703852178639279f5d9c5ed4eeccc741fa37";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogClose.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogClose }
}
impl LogReadRequest {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "1a102ada8f5a4a0598c06653519687ebaae0422aa893a0296500d50aa7574bef";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogReadRequest.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogReadRequest }
}
impl LogEndStream {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "7f66da4e3c1e5ce26311b8dda291bac05ca542ce3b5b84895f42dcf305f192a1";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogEndStream.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogEndStream }
}
impl LogTimerExpired {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "d68222af26390472e09a63532c1e37489a77eb0c229d5c1a4c8aeca0a0965ac4";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogTimerExpired.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogTimerExpired }
}
impl LogEvict {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "5b52047ebc0f443b3bf9458706790cd4069f1a52a039c845b6b3cce3cca080e1";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogEvict.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogEvict }
}
impl LogReadResponse {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "2bd2bfdc16ab728d66f7040d01e2cd001130bd143bfd41b3dbaa76688ac21ebf";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogReadResponse.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogReadResponse }
}
impl LogSetTimer {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "183f98ea1a6a780aa61f5e8f1074aa7714a1c6a4ea6f9a6d0fbc690549645403";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogSetTimer.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogSetTimer }
}
impl LogCancelTimer {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "f6bf8102a924bde7b4027909d4598c8d17ca43231d212a5326458aa001309865";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogCancelTimer.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogCancelTimer }
}
impl LogProducerStop {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "4faf3ecbec7f8b6ddc25d3ab604b8ee62e6256da56797f6e8bccbd34bbb62df7";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogProducerStop.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogProducerStop }
}
impl LogDiagnostic {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "8ff8c0e20e586b59e49d96c0c7588c94239f4c8a9f42d7fc060737cbad2e4d79";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_LogDiagnostic.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_LogDiagnostic }
}
pub static SCHEMA_DESCRIPTOR: &crate::cbor::descriptors::SchemaDescriptor = &crate::cbor::descriptors::SchemaDescriptor::new(1, 2, "schema_a1c5d286436a35d4",
&[LogCursor::descriptor, LogRecord::descriptor, LogError::descriptor, LogPush::descriptor, LogSeal::descriptor, LogClose::descriptor, LogReadRequest::descriptor, LogEndStream::descriptor, LogTimerExpired::descriptor, LogEvict::descriptor, LogReadResponse::descriptor, LogSetTimer::descriptor, LogCancelTimer::descriptor, LogProducerStop::descriptor, LogDiagnostic::descriptor],
&[LogMsgType::descriptor, LogState::descriptor, LogErrorCode::descriptor, LogStopReason::descriptor, LogSeverity::descriptor, LogDiagCode::descriptor],
_taut_metadata::FILE_OPTIONS, _taut_metadata::FILE_EFFECTIVE);
#[allow(non_upper_case_globals, unused_variables, unused_imports)]
mod _taut_metadata {
use super::*;
use crate::cbor::descriptors::*;
pub const FILE_OPTIONS: Options = &[];
pub const FILE_EFFECTIVE: Options = &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)];
pub fn check_external() {
require_contract("schema_a1c5d286436a35d4", "taut.rust.foundation", 1, 1, &["descriptors.v1", "codec.v1"]);
}
pub static ENUM_LogMsgType: &EnumDescriptor = &EnumDescriptor::new("LogMsgType", "schema_a1c5d286436a35d4.LogMsgType", &[EnumMemberDescriptor::new("push", 0, &[]), EnumMemberDescriptor::new("seal", 1, &[]), EnumMemberDescriptor::new("close", 2, &[]), EnumMemberDescriptor::new("read", 3, &[]), EnumMemberDescriptor::new("end_stream", 4, &[]), EnumMemberDescriptor::new("timer_expired", 5, &[]), EnumMemberDescriptor::new("evict", 6, &[]), EnumMemberDescriptor::new("read_response", 7, &[]), EnumMemberDescriptor::new("set_timer", 8, &[]), EnumMemberDescriptor::new("cancel_timer", 9, &[]), EnumMemberDescriptor::new("producer_stop", 10, &[]), EnumMemberDescriptor::new("diagnostic", 11, &[])], &[]);
pub static ENUM_LogState: &EnumDescriptor = &EnumDescriptor::new("LogState", "schema_a1c5d286436a35d4.LogState", &[EnumMemberDescriptor::new("data", 0, &[]), EnumMemberDescriptor::new("would_block", 1, &[]), EnumMemberDescriptor::new("eof", 2, &[]), EnumMemberDescriptor::new("closed", 3, &[]), EnumMemberDescriptor::new("failed", 4, &[]), EnumMemberDescriptor::new("expired", 5, &[])], &[]);
pub static ENUM_LogErrorCode: &EnumDescriptor = &EnumDescriptor::new("LogErrorCode", "schema_a1c5d286436a35d4.LogErrorCode", &[EnumMemberDescriptor::new("unknown_log", 0, &[]), EnumMemberDescriptor::new("producer_error", 1, &[]), EnumMemberDescriptor::new("internal", 2, &[])], &[]);
pub static ENUM_LogStopReason: &EnumDescriptor = &EnumDescriptor::new("LogStopReason", "schema_a1c5d286436a35d4.LogStopReason", &[EnumMemberDescriptor::new("last_reader_gone", 0, &[]), EnumMemberDescriptor::new("closed", 1, &[]), EnumMemberDescriptor::new("failed", 2, &[])], &[]);
pub static ENUM_LogSeverity: &EnumDescriptor = &EnumDescriptor::new("LogSeverity", "schema_a1c5d286436a35d4.LogSeverity", &[EnumMemberDescriptor::new("warn", 0, &[]), EnumMemberDescriptor::new("error", 1, &[])], &[]);
pub static ENUM_LogDiagCode: &EnumDescriptor = &EnumDescriptor::new("LogDiagCode", "schema_a1c5d286436a35d4.LogDiagCode", &[EnumMemberDescriptor::new("push_after_terminal", 0, &[])], &[]);
pub static BINDING_LogCursor: &SchemaBinding<LogCursor> = &SchemaBinding::new(MessageDescriptor::new("LogCursor", "schema_a1c5d286436a35d4.LogCursor", 1, &[
FieldDescriptor::new("seq", Some("seq"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.seq)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_LogRecord: &SchemaBinding<LogRecord> = &SchemaBinding::new(MessageDescriptor::new("LogRecord", "schema_a1c5d286436a35d4.LogRecord", 1, &[
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
pub static BINDING_LogError: &SchemaBinding<LogError> = &SchemaBinding::new(MessageDescriptor::new("LogError", "schema_a1c5d286436a35d4.LogError", 1, &[
FieldDescriptor::new("code", Some("code"), 1, TypeDescriptor::Enum(LogErrorCode::descriptor), "required", false, false, &[]),
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
pub static BINDING_LogPush: &SchemaBinding<LogPush> = &SchemaBinding::new(MessageDescriptor::new("LogPush", "schema_a1c5d286436a35d4.LogPush", 1, &[
FieldDescriptor::new("payload", Some("payload"), 1, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_LogSeal: &SchemaBinding<LogSeal> = &SchemaBinding::new(MessageDescriptor::new("LogSeal", "schema_a1c5d286436a35d4.LogSeal", 1, &[
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
panic!("Taut typed binding field mismatch: regenerate artifact")
}, |value, index, visitor| {
true });
pub static BINDING_LogClose: &SchemaBinding<LogClose> = &SchemaBinding::new(MessageDescriptor::new("LogClose", "schema_a1c5d286436a35d4.LogClose", 1, &[
FieldDescriptor::new("error", Some("error"), 1, TypeDescriptor::Message(LogError::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(value.error.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 0 {
if let Some(child) = &value.error {
if !visitor.record(&LogError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_LogReadRequest: &SchemaBinding<LogReadRequest> = &SchemaBinding::new(MessageDescriptor::new("LogReadRequest", "schema_a1c5d286436a35d4.LogReadRequest", 1, &[
FieldDescriptor::new("log_id", Some("log_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("cursor", Some("cursor"), 3, TypeDescriptor::Message(LogCursor::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("max_records", Some("max_records"), 4, TypeDescriptor::Int, "nullable", false, false, &[]),
FieldDescriptor::new("max_bytes", Some("max_bytes"), 5, TypeDescriptor::Int, "nullable", false, false, &[]),
FieldDescriptor::new("timeout_ms", Some("timeout_ms"), 6, TypeDescriptor::Int, "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.log_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
2 => FieldState::new(value.cursor.is_some(), None),
3 => match &value.max_records { Some(v) => FieldState::from_primitive(PrimitiveValue::Int(*v)), None => FieldState::new(false, None) },
4 => match &value.max_bytes { Some(v) => FieldState::from_primitive(PrimitiveValue::Int(*v)), None => FieldState::new(false, None) },
5 => match &value.timeout_ms { Some(v) => FieldState::from_primitive(PrimitiveValue::Int(*v)), None => FieldState::new(false, None) },
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 2 {
if let Some(child) = &value.cursor {
if !visitor.record(&LogCursor::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_LogEndStream: &SchemaBinding<LogEndStream> = &SchemaBinding::new(MessageDescriptor::new("LogEndStream", "schema_a1c5d286436a35d4.LogEndStream", 1, &[
FieldDescriptor::new("log_id", Some("log_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.log_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_LogTimerExpired: &SchemaBinding<LogTimerExpired> = &SchemaBinding::new(MessageDescriptor::new("LogTimerExpired", "schema_a1c5d286436a35d4.LogTimerExpired", 1, &[
FieldDescriptor::new("token", Some("token"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.token)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_LogEvict: &SchemaBinding<LogEvict> = &SchemaBinding::new(MessageDescriptor::new("LogEvict", "schema_a1c5d286436a35d4.LogEvict", 1, &[
FieldDescriptor::new("up_to_seq", Some("up_to_seq"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.up_to_seq)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_LogReadResponse: &SchemaBinding<LogReadResponse> = &SchemaBinding::new(MessageDescriptor::new("LogReadResponse", "schema_a1c5d286436a35d4.LogReadResponse", 1, &[
FieldDescriptor::new("log_id", Some("log_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("records", Some("records"), 3, TypeDescriptor::List(&TypeDescriptor::Message(LogRecord::descriptor)), "required", false, false, &[]),
FieldDescriptor::new("next_cursor", Some("next_cursor"), 4, TypeDescriptor::Message(LogCursor::descriptor), "required", false, false, &[]),
FieldDescriptor::new("state", Some("state"), 5, TypeDescriptor::Enum(LogState::descriptor), "required", false, false, &[]),
FieldDescriptor::new("error", Some("error"), 6, TypeDescriptor::Message(LogError::descriptor), "nullable", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.log_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
2 => FieldState::new(true, None),
3 => FieldState::new(true, None),
4 => FieldState::new(true, Some(value.state.wire())),
5 => FieldState::new(value.error.is_some(), None),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 2 {
for (index0, item0) in value.records.iter().enumerate() {
visitor.push_index(index0);
if !visitor.record(&LogRecord::taut_binding().bind(item0)) { return false; }
visitor.pop();
}
}
if index == 3 {
if !visitor.record(&LogCursor::taut_binding().bind(&value.next_cursor)) { return false; }
}
if index == 5 {
if let Some(child) = &value.error {
if !visitor.record(&LogError::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_LogSetTimer: &SchemaBinding<LogSetTimer> = &SchemaBinding::new(MessageDescriptor::new("LogSetTimer", "schema_a1c5d286436a35d4.LogSetTimer", 1, &[
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
pub static BINDING_LogCancelTimer: &SchemaBinding<LogCancelTimer> = &SchemaBinding::new(MessageDescriptor::new("LogCancelTimer", "schema_a1c5d286436a35d4.LogCancelTimer", 1, &[
FieldDescriptor::new("token", Some("token"), 1, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::Int(value.token)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_LogProducerStop: &SchemaBinding<LogProducerStop> = &SchemaBinding::new(MessageDescriptor::new("LogProducerStop", "schema_a1c5d286436a35d4.LogProducerStop", 1, &[
FieldDescriptor::new("reason", Some("reason"), 1, TypeDescriptor::Enum(LogStopReason::descriptor), "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::new(true, Some(value.reason.wire())),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_LogDiagnostic: &SchemaBinding<LogDiagnostic> = &SchemaBinding::new(MessageDescriptor::new("LogDiagnostic", "schema_a1c5d286436a35d4.LogDiagnostic", 1, &[
FieldDescriptor::new("severity", Some("severity"), 1, TypeDescriptor::Enum(LogSeverity::descriptor), "required", false, false, &[]),
FieldDescriptor::new("code", Some("code"), 2, TypeDescriptor::Enum(LogDiagCode::descriptor), "required", false, false, &[]),
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
