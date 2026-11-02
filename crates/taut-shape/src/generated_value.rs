// Source: taut-shape/ir/shape_value.taut.py; regenerate with scripts/regenerate_foundation_bindings.py
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
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
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
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
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
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
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
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueReadRequest {
    pub value_id: String,
    pub stream_id: String,
}
impl ValueReadRequest {
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
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
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
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
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_ENCODED_LEN: Option<usize> = None;
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
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueDiagnostic {
    pub severity: ValueSeverity,
    pub code: ValueDiagCode,
}
impl ValueDiagnostic {
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
            severity: ValueSeverity::from_wire(c.try_get(1)?.try_int()?)?,
            code: ValueDiagCode::from_wire(c.try_get(2)?.try_int()?)?,
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
// Generator build rust-oneof-reference.1; descriptor=1; codec=1; owner=taut.rust.foundation
const _: () = assert!(crate::cbor::descriptors::DESCRIPTOR_CONTRACT == 1, "Taut descriptor contract mismatch: rebind runtime");
const _: () = assert!(crate::cbor::descriptors::CODEC_CONTRACT == 1, "Taut codec contract mismatch: rebind runtime");
impl ValueMsgType {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "f7368136bca27637724b0f4fc36b0491ded8a628190c3032ac8bdb79702cc0bd";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_ValueMsgType; descriptor.check(); descriptor }
}
impl ValueState {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "0c667731fce4692a0ca8bcc72c40dd1538c550882e82e929233dc1e033be646a";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_ValueState; descriptor.check(); descriptor }
}
impl ValueSeverity {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "21afca5ad2523212e5b875d07d3bdae736c77484f8c5449537ba3d112d128505";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_ValueSeverity; descriptor.check(); descriptor }
}
impl ValueDiagCode {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "3e74d7fc07c0364e768d1f95cf25958a7ef2d3bd8b98bf8098dc21235f4d1dc2";
pub fn descriptor() -> &'static crate::cbor::descriptors::EnumDescriptor { let descriptor = _taut_metadata::ENUM_ValueDiagCode; descriptor.check(); descriptor }
}
impl ValueStamp {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "b6b5f4c8b4fb6102bad51c7f7e4592bdae401454c9317d9ee181830026fc253a";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_ValueStamp.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_ValueStamp }
}
impl ValueSet {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "a6118e2a35f3525715a1506f5e3c6deb39e0e87355e2cfbdab1123dc96904279";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_ValueSet.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_ValueSet }
}
impl ValueReadRequest {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "f1267decb63ab2c34d1654a3d14155ea528cfab74fda8078ce2f29fe122d30e4";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_ValueReadRequest.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_ValueReadRequest }
}
impl ValueReadResponse {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "07389ea8d5535e9d7529deb38514a4287e7e4dd438b82952d597f5f21aa9c534";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_ValueReadResponse.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_ValueReadResponse }
}
impl ValueDiagnostic {
pub const TAUT_DESCRIPTOR_CONTRACT: u32 = 1;
pub const TAUT_CODEC_CONTRACT: u32 = 1;
pub const TAUT_FACTS_FINGERPRINT: &'static str = "43cc06b7e6cd99491038e2246c039f910bb8cb50da02c45d35a66e8014efef2a";
pub fn descriptor() -> &'static crate::cbor::descriptors::MessageDescriptor {
_taut_metadata::BINDING_ValueDiagnostic.descriptor()
}
pub fn taut_binding() -> &'static crate::cbor::descriptors::SchemaBinding<Self> { _taut_metadata::BINDING_ValueDiagnostic }
}
pub static SCHEMA_DESCRIPTOR: &crate::cbor::descriptors::SchemaDescriptor = &crate::cbor::descriptors::SchemaDescriptor::new(1, 2, "schema_8bd4caa26c8a13f3",
&[ValueStamp::descriptor, ValueSet::descriptor, ValueReadRequest::descriptor, ValueReadResponse::descriptor, ValueDiagnostic::descriptor],
&[ValueMsgType::descriptor, ValueState::descriptor, ValueSeverity::descriptor, ValueDiagCode::descriptor],
_taut_metadata::FILE_OPTIONS, _taut_metadata::FILE_EFFECTIVE);
#[allow(non_upper_case_globals, unused_variables, unused_imports)]
mod _taut_metadata {
use super::*;
use crate::cbor::descriptors::*;
pub const FILE_OPTIONS: Options = &[];
pub const FILE_EFFECTIVE: Options = &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)];
pub fn check_external() {
require_contract("schema_8bd4caa26c8a13f3", "taut.rust.foundation", 1, 1, &["descriptors.v1", "codec.v1"]);
}
pub static ENUM_ValueMsgType: &EnumDescriptor = &EnumDescriptor::new("ValueMsgType", "schema_8bd4caa26c8a13f3.ValueMsgType", &[EnumMemberDescriptor::new("set", 0, &[]), EnumMemberDescriptor::new("read", 1, &[]), EnumMemberDescriptor::new("read_response", 2, &[]), EnumMemberDescriptor::new("diagnostic", 3, &[])], &[]);
pub static ENUM_ValueState: &EnumDescriptor = &EnumDescriptor::new("ValueState", "schema_8bd4caa26c8a13f3.ValueState", &[EnumMemberDescriptor::new("data", 0, &[]), EnumMemberDescriptor::new("empty", 1, &[])], &[]);
pub static ENUM_ValueSeverity: &EnumDescriptor = &EnumDescriptor::new("ValueSeverity", "schema_8bd4caa26c8a13f3.ValueSeverity", &[EnumMemberDescriptor::new("warn", 0, &[]), EnumMemberDescriptor::new("error", 1, &[])], &[]);
pub static ENUM_ValueDiagCode: &EnumDescriptor = &EnumDescriptor::new("ValueDiagCode", "schema_8bd4caa26c8a13f3.ValueDiagCode", &[EnumMemberDescriptor::new("equivocation", 0, &[])], &[]);
pub static BINDING_ValueStamp: &SchemaBinding<ValueStamp> = &SchemaBinding::new(MessageDescriptor::new("ValueStamp", "schema_8bd4caa26c8a13f3.ValueStamp", 1, &[
FieldDescriptor::new("origin", Some("origin"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("seq", Some("seq"), 2, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("lamport", Some("lamport"), 3, TypeDescriptor::Int, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.origin)),
1 => FieldState::from_primitive(PrimitiveValue::Int(value.seq)),
2 => FieldState::from_primitive(PrimitiveValue::Int(value.lamport)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_ValueSet: &SchemaBinding<ValueSet> = &SchemaBinding::new(MessageDescriptor::new("ValueSet", "schema_8bd4caa26c8a13f3.ValueSet", 1, &[
FieldDescriptor::new("origin", Some("origin"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("seq", Some("seq"), 2, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("lamport", Some("lamport"), 3, TypeDescriptor::Int, "required", false, false, &[]),
FieldDescriptor::new("prev", Some("prev"), 4, TypeDescriptor::Bytes, "nullable", false, false, &[]),
FieldDescriptor::new("payload", Some("payload"), 5, TypeDescriptor::Bytes, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.origin)),
1 => FieldState::from_primitive(PrimitiveValue::Int(value.seq)),
2 => FieldState::from_primitive(PrimitiveValue::Int(value.lamport)),
3 => match &value.prev { Some(v) => FieldState::from_primitive(PrimitiveValue::Bytes(v)), None => FieldState::new(false, None) },
4 => FieldState::from_primitive(PrimitiveValue::Bytes(&value.payload)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_ValueReadRequest: &SchemaBinding<ValueReadRequest> = &SchemaBinding::new(MessageDescriptor::new("ValueReadRequest", "schema_8bd4caa26c8a13f3.ValueReadRequest", 1, &[
FieldDescriptor::new("value_id", Some("value_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.value_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
true });
pub static BINDING_ValueReadResponse: &SchemaBinding<ValueReadResponse> = &SchemaBinding::new(MessageDescriptor::new("ValueReadResponse", "schema_8bd4caa26c8a13f3.ValueReadResponse", 1, &[
FieldDescriptor::new("value_id", Some("value_id"), 1, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("stream_id", Some("stream_id"), 2, TypeDescriptor::String, "required", false, false, &[]),
FieldDescriptor::new("value", Some("value"), 3, TypeDescriptor::Bytes, "nullable", false, false, &[]),
FieldDescriptor::new("winner", Some("winner"), 4, TypeDescriptor::Message(ValueStamp::descriptor), "nullable", false, false, &[]),
FieldDescriptor::new("state", Some("state"), 5, TypeDescriptor::Enum(ValueState::descriptor), "required", false, false, &[]),
], &[
], &[], &[], &[], None, &[], &[("max_depth", OptionValue::Int(32)), ("max_encoded_len", OptionValue::Null)]), check_external, |value, index| {
match index {
0 => FieldState::from_primitive(PrimitiveValue::String(&value.value_id)),
1 => FieldState::from_primitive(PrimitiveValue::String(&value.stream_id)),
2 => match &value.value { Some(v) => FieldState::from_primitive(PrimitiveValue::Bytes(v)), None => FieldState::new(false, None) },
3 => FieldState::new(value.winner.is_some(), None),
4 => FieldState::new(true, Some(value.state.wire())),
_ => panic!("Taut typed binding field mismatch: regenerate artifact"),
}
}, |value, index, visitor| {
if index == 3 {
if let Some(child) = &value.winner {
if !visitor.record(&ValueStamp::taut_binding().bind(child)) { return false; }
}
}
true });
pub static BINDING_ValueDiagnostic: &SchemaBinding<ValueDiagnostic> = &SchemaBinding::new(MessageDescriptor::new("ValueDiagnostic", "schema_8bd4caa26c8a13f3.ValueDiagnostic", 1, &[
FieldDescriptor::new("severity", Some("severity"), 1, TypeDescriptor::Enum(ValueSeverity::descriptor), "required", false, false, &[]),
FieldDescriptor::new("code", Some("code"), 2, TypeDescriptor::Enum(ValueDiagCode::descriptor), "required", false, false, &[]),
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
