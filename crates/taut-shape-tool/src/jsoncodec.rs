//! The taut *jsoncodec form* of `shape_log` messages (Oracle §3): a `type`
//! discriminator plus proto3-JSON field conventions — **bytes are base64**,
//! **i64s are strings**, absent optionals are `null`/omitted.
//!
//! Two directions live here, so the node `--script`, the client `--script`, and
//! the client's OOB transcript all speak one dialect, byte-identical to
//! `corpus/log.v0.json`:
//!
//!   * [`input_from_json`] — a producer/stream message object → engine [`Input`]
//!     (used by `--script` injection). Only the producer-side kinds a script
//!     drives are accepted: `push`, `seal`, `close`, `evict` (plus `read` /
//!     `end_stream` for completeness, so a script could also drive readers).
//!   * [`response_to_json`] / [`output_to_json`] — a received `read_response`
//!     (or any engine [`Output`]) → a jsoncodec object, for the stderr
//!     transcript.

use taut_shape::generated::LogReadResponse;
use taut_shape::{
    Cursor, Error, ErrorCode, Input, Limits, Output, Response, State, StopReason, TimerToken,
};

use crate::json::{self, Json};

/// Parse one jsoncodec message object into an engine [`Input`]. Returns a human
/// string on an unknown/mis-shaped `type` (mapped by the caller to a usage/exit
/// error). Field decoding mirrors the corpus: base64 payloads, string i64s.
pub fn input_from_json(v: &Json) -> Result<Input, String> {
    let ty = v
        .get("type")
        .and_then(Json::as_str)
        .ok_or_else(|| "script message missing string `type`".to_string())?;
    Ok(match ty {
        "push" => {
            let payload = v
                .get("payload")
                .and_then(Json::as_str)
                .and_then(json::base64_decode)
                .ok_or_else(|| "push: missing/invalid base64 `payload`".to_string())?;
            Input::Push { payload }
        }
        "seal" => Input::Seal,
        "close" => {
            let error = match v.get("error") {
                None | Some(Json::Null) => None,
                Some(e) => Some(error_from_json(e)?),
            };
            Input::Close { error }
        }
        "evict" => {
            let up_to_seq = v
                .get("up_to_seq")
                .and_then(Json::as_i64)
                .ok_or_else(|| "evict: missing `up_to_seq`".to_string())?;
            Input::Evict {
                up_to_seq: up_to_seq as u64,
            }
        }
        "read" => {
            let stream_id = v
                .get("stream_id")
                .and_then(Json::as_str)
                .ok_or_else(|| "read: missing `stream_id`".to_string())?;
            let cursor = v
                .get("cursor")
                .and_then(|c| c.get("seq"))
                .and_then(Json::as_i64)
                .map(|seq| Cursor::new(seq as u64));
            Input::Read {
                stream_id: stream_id.into(),
                cursor,
                limits: Limits {
                    max_records: v.get("max_records").and_then(Json::as_i64).map(|n| n as u32),
                    max_bytes: v.get("max_bytes").and_then(Json::as_i64).map(|n| n as u64),
                },
                timeout_ms: v.get("timeout_ms").and_then(Json::as_i64).map(|n| n as u64),
            }
        }
        "end_stream" => {
            let stream_id = v
                .get("stream_id")
                .and_then(Json::as_str)
                .ok_or_else(|| "end_stream: missing `stream_id`".to_string())?;
            Input::EndStream {
                stream_id: stream_id.into(),
            }
        }
        "timer_expired" => {
            let token = v
                .get("token")
                .and_then(Json::as_i64)
                .ok_or_else(|| "timer_expired: missing `token`".to_string())?;
            Input::TimerExpired {
                token: TimerToken(token as u64),
            }
        }
        other => return Err(format!("unknown script message type {other:?}")),
    })
}

fn error_from_json(e: &Json) -> Result<Error, String> {
    let code = match e.get("code").and_then(Json::as_str) {
        Some("unknown_log") => ErrorCode::UnknownLog,
        Some("producer_error") => ErrorCode::ProducerError,
        Some("internal") => ErrorCode::Internal,
        Some(other) => return Err(format!("unknown error code {other:?}")),
        None => ErrorCode::ProducerError,
    };
    let message = e.get("message").and_then(Json::as_str).map(str::to_string);
    Ok(Error { code, message })
}

// ── engine → jsoncodec (transcript direction) ───────────────────────────────

/// Render a wire [`LogReadResponse`] as a `read_response` jsoncodec object —
/// the exact shape the corpus and the node's response frames carry. Used by the
/// client to log every received response to its OOB stderr transcript.
pub fn read_response_to_json(r: &LogReadResponse) -> Json {
    obj(vec![
        ("type", json::s("read_response")),
        ("log_id", json::s(r.log_id.clone())),
        ("stream_id", json::s(r.stream_id.clone())),
        (
            "records",
            Json::Arr(
                r.records
                    .iter()
                    .map(|rec| {
                        obj(vec![
                            ("seq", json::i64_str(rec.seq)),
                            ("payload", json::s(json::base64_encode(&rec.payload))),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "next_cursor",
            obj(vec![("seq", json::i64_str(r.next_cursor.seq))]),
        ),
        ("state", json::s(state_str(r.state))),
        (
            "error",
            match &r.error {
                None => Json::Null,
                Some(e) => error_to_json_wire(e),
            },
        ),
    ])
}

fn state_str(s: taut_shape::generated::LogState) -> &'static str {
    use taut_shape::generated::LogState as W;
    match s {
        W::Data => "data",
        W::WouldBlock => "would_block",
        W::Eof => "eof",
        W::Closed => "closed",
        W::Failed => "failed",
        W::Expired => "expired",
    }
}

fn error_to_json_wire(e: &taut_shape::generated::LogError) -> Json {
    use taut_shape::generated::LogErrorCode as W;
    let code = match e.code {
        W::UnknownLog => "unknown_log",
        W::ProducerError => "producer_error",
        W::Internal => "internal",
    };
    obj(vec![
        ("code", json::s(code)),
        (
            "message",
            match &e.message {
                None => Json::Null,
                Some(m) => json::s(m.clone()),
            },
        ),
    ])
}

/// Render an engine [`Output`] as a jsoncodec object (all variants), so the node
/// `--script` path can log injected-producer outputs to its OOB transcript in
/// the same dialect the corpus uses for `set_timer`/`cancel_timer`/etc.
pub fn output_to_json(out: &Output, log_id: &str) -> Json {
    match out {
        Output::Response(resp) => response_to_json(resp, log_id),
        Output::SetTimer { token, ms } => obj(vec![
            ("type", json::s("set_timer")),
            ("token", json::i64_str(token.0 as i64)),
            ("ms", json::i64_str(*ms as i64)),
        ]),
        Output::CancelTimer { token } => obj(vec![
            ("type", json::s("cancel_timer")),
            ("token", json::i64_str(token.0 as i64)),
        ]),
        Output::ProducerStop { reason } => obj(vec![
            ("type", json::s("producer_stop")),
            ("reason", json::s(stop_reason_str(*reason))),
        ]),
        Output::Diagnostic(d) => obj(vec![
            ("type", json::s("diagnostic")),
            ("severity", json::s(severity_str(d.severity))),
            ("code", json::s(diag_code_str(d.code))),
        ]),
    }
}

/// The engine's own [`Response`] rendered as a `read_response` object. `log_id`
/// is a framing-layer echo (D3) supplied by the caller.
pub fn response_to_json(resp: &Response, log_id: &str) -> Json {
    obj(vec![
        ("type", json::s("read_response")),
        ("log_id", json::s(log_id)),
        ("stream_id", json::s(resp.stream_id.0.to_string())),
        (
            "records",
            Json::Arr(
                resp.records
                    .iter()
                    .map(|rec| {
                        obj(vec![
                            ("seq", json::i64_str(rec.seq as i64)),
                            ("payload", json::s(json::base64_encode(&rec.payload))),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "next_cursor",
            obj(vec![("seq", json::i64_str(resp.next_cursor.seq as i64))]),
        ),
        ("state", json::s(core_state_str(resp.state))),
        (
            "error",
            match &resp.error {
                None => Json::Null,
                Some(e) => error_to_json_core(e),
            },
        ),
    ])
}

fn core_state_str(s: State) -> &'static str {
    match s {
        State::Data => "data",
        State::WouldBlock => "would_block",
        State::Eof => "eof",
        State::Closed => "closed",
        State::Failed => "failed",
        State::Expired => "expired",
    }
}

fn error_to_json_core(e: &Error) -> Json {
    let code = match e.code {
        ErrorCode::UnknownLog => "unknown_log",
        ErrorCode::ProducerError => "producer_error",
        ErrorCode::Internal => "internal",
        _ => "internal",
    };
    obj(vec![
        ("code", json::s(code)),
        (
            "message",
            match &e.message {
                None => Json::Null,
                Some(m) => json::s(m.clone()),
            },
        ),
    ])
}

fn stop_reason_str(r: StopReason) -> &'static str {
    match r {
        StopReason::LastReaderGone => "last_reader_gone",
        StopReason::Closed => "closed",
        StopReason::Failed => "failed",
    }
}

fn severity_str(s: taut_shape::generated::LogSeverity) -> &'static str {
    use taut_shape::generated::LogSeverity as W;
    match s {
        W::Warn => "warn",
        W::Error => "error",
    }
}

fn diag_code_str(c: taut_shape::generated::LogDiagCode) -> &'static str {
    use taut_shape::generated::LogDiagCode as W;
    match c {
        W::PushAfterTerminal => "push_after_terminal",
    }
}

fn obj(pairs: Vec<(&str, Json)>) -> Json {
    let mut m = std::collections::BTreeMap::new();
    for (k, v) in pairs {
        m.insert(k.to_string(), v);
    }
    Json::Obj(m)
}
