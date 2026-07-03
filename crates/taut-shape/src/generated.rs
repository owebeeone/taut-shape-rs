#![allow(dead_code)]
// This inner attribute exempts the file from `cargo fmt` on stable (the
// `rustfmt.toml` `ignore` key is nightly-only), keeping it byte-identical to
// the tautc output. It is the ONLY line added above the original generator
// header line below.
#![cfg_attr(rustfmt, rustfmt::skip)]
// ============================================================================
// GENERATED — DO NOT EDIT BY HAND.
//
// tautc-generated `shape_log` message types + CBOR codec (D17). This file is
// vendored verbatim from the tautc `gen -l rust --api-only` output below the
// provenance block; regenerate + re-vendor on any schema bump, never
// hand-maintain (the gwz-core `protocol/generated.rs` pattern).
//
// NOTE: this file is EXEMPT from `cargo fmt` via the `#![rustfmt::skip]` inner
// attribute above, so it stays byte-identical to the tautc output; `fmt` would
// otherwise reflow it (expand `match` arms, split `#[default] Push,`, …).
//
// Source schema : taut-shape/ir/shape_log.taut.py
//                 (exported IR: taut-shape/ir/shape_log.ir.json)
// Generator     : taut  @ 70e17b7
// Schema repo   : taut-shape @ 7aa206b + diagnostics (D18/D19: LogSeverity,
//                 LogDiagCode, LogDiagnostic, LogMsgType.diagnostic=11; uncommitted)
//
// Regen (from taut-dev/taut-shape):
//   PYTHONPATH=../taut/src python3 -m taut.cli gen ir/shape_log.taut.py \
//       -o <out> -l rust --api-only
//   cp <out>/rust/api.rs crates/taut-shape/src/generated.rs   # then re-add this header
//
// no_std note: the codec below uses `Vec`/`String`/`vec!`; the core crate is
// `#![no_std]` + `alloc`, so the prelude glob below makes those names resolve
// without `std`. This is the only edit applied on top of the raw tautc output.
// ============================================================================

#[allow(unused_imports)]
use alloc::{string::String, vec, vec::Vec};

use crate::cbor::Cbor;

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
    pub fn from_wire(v: i64) -> Self { match v {
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
        _ => panic!("bad LogMsgType wire value {}", v),
    } }
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
    pub fn from_wire(v: i64) -> Self { match v {
        0 => Self::Data,
        1 => Self::WouldBlock,
        2 => Self::Eof,
        3 => Self::Closed,
        4 => Self::Failed,
        5 => Self::Expired,
        _ => panic!("bad LogState wire value {}", v),
    } }
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
    pub fn from_wire(v: i64) -> Self { match v {
        0 => Self::UnknownLog,
        1 => Self::ProducerError,
        2 => Self::Internal,
        _ => panic!("bad LogErrorCode wire value {}", v),
    } }
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
    pub fn from_wire(v: i64) -> Self { match v {
        0 => Self::LastReaderGone,
        1 => Self::Closed,
        2 => Self::Failed,
        _ => panic!("bad LogStopReason wire value {}", v),
    } }
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
    pub fn from_wire(v: i64) -> Self { match v {
        0 => Self::Warn,
        1 => Self::Error,
        _ => panic!("bad LogSeverity wire value {}", v),
    } }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LogDiagCode {
    #[default] PushAfterTerminal,
}
impl LogDiagCode {
    pub fn wire(self) -> i64 { match self {
        Self::PushAfterTerminal => 0,
    } }
    pub fn from_wire(v: i64) -> Self { match v {
        0 => Self::PushAfterTerminal,
        _ => panic!("bad LogDiagCode wire value {}", v),
    } }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogCursor {
    pub seq: i64,
}
impl LogCursor {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.seq)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            seq: c.get(1).int(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogRecord {
    pub seq: i64,
    pub payload: Vec<u8>,
}
impl LogRecord {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.seq)),
            (2, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            seq: c.get(1).int(),
            payload: c.get(2).bytes(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogError {
    pub code: LogErrorCode,
    pub message: Option<String>,
}
impl LogError {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.code.wire())),
            (2, match &self.message { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            code: LogErrorCode::from_wire(c.get(1).int()),
            message: { let v = c.get(2); if v.is_null() { None } else { Some(v.text()) } },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogPush {
    pub payload: Vec<u8>,
}
impl LogPush {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            payload: c.get(1).bytes(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogSeal {
}
impl LogSeal {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogClose {
    pub error: Option<LogError>,
}
impl LogClose {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, match &self.error { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            error: { let v = c.get(1); if v.is_null() { None } else { Some(LogError::from_cbor(v)) } },
        }
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
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            log_id: c.get(1).text(),
            stream_id: c.get(2).text(),
            cursor: { let v = c.get(3); if v.is_null() { None } else { Some(LogCursor::from_cbor(v)) } },
            max_records: { let v = c.get(4); if v.is_null() { None } else { Some(v.int()) } },
            max_bytes: { let v = c.get(5); if v.is_null() { None } else { Some(v.int()) } },
            timeout_ms: { let v = c.get(6); if v.is_null() { None } else { Some(v.int()) } },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogEndStream {
    pub log_id: String,
    pub stream_id: String,
}
impl LogEndStream {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.log_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            log_id: c.get(1).text(),
            stream_id: c.get(2).text(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogTimerExpired {
    pub token: i64,
}
impl LogTimerExpired {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.token)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            token: c.get(1).int(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogEvict {
    pub up_to_seq: i64,
}
impl LogEvict {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.up_to_seq)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            up_to_seq: c.get(1).int(),
        }
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
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            log_id: c.get(1).text(),
            stream_id: c.get(2).text(),
            records: c.get(3).array().iter().map(|x| LogRecord::from_cbor(x)).collect(),
            next_cursor: LogCursor::from_cbor(c.get(4)),
            state: LogState::from_wire(c.get(5).int()),
            error: { let v = c.get(6); if v.is_null() { None } else { Some(LogError::from_cbor(v)) } },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogSetTimer {
    pub token: i64,
    pub ms: i64,
}
impl LogSetTimer {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.token)),
            (2, Cbor::Int(self.ms)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            token: c.get(1).int(),
            ms: c.get(2).int(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogCancelTimer {
    pub token: i64,
}
impl LogCancelTimer {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.token)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            token: c.get(1).int(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogProducerStop {
    pub reason: LogStopReason,
}
impl LogProducerStop {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.reason.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            reason: LogStopReason::from_wire(c.get(1).int()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LogDiagnostic {
    pub severity: LogSeverity,
    pub code: LogDiagCode,
}
impl LogDiagnostic {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.severity.wire())),
            (2, Cbor::Int(self.code.wire())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Self {
        Self {
            severity: LogSeverity::from_wire(c.get(1).int()),
            code: LogDiagCode::from_wire(c.get(2).int()),
        }
    }
}
