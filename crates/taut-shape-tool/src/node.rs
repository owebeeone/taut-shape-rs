//! `node` mode: drive a single [`LogNode`] behind the stdin/stdout framing.
//!
//! The pump is dead simple and deterministic (D15/D16): read one input frame,
//! decode it into an engine [`Input`], `handle` it, then write every resulting
//! [`Output`] as a frame in order and flush. EOF on stdin ⇒ exit 0. A malformed
//! or unknown frame ⇒ one stderr line + exit 3 (never a panic).
//!
//! Input↔wire mapping (S4.2). The generated wire structs carry an `i64`-typed,
//! `log_id`/`stream_id`-addressed shape; the engine's [`Input`]/[`Output`] use
//! the ergonomic §A.1 core types. This module is the sole place that converts
//! between the two (the "framing concern", InitialPlan §Type-mapping). Because
//! `node` mode is a single log, the read request's `log_id`/`stream_id` are
//! carried opaquely and echoed straight back onto the read-response frame.

use std::collections::HashMap;
use std::io::{self, BufWriter, Read, Write};

use taut_shape::cbor::Cbor;
use taut_shape::generated::{
    LogCancelTimer, LogCursor, LogEndStream, LogEvict, LogMsgType, LogProducerStop, LogPush,
    LogReadRequest, LogReadResponse, LogRecord, LogSetTimer, LogTimerExpired,
};
use taut_shape::{
    Config, Cursor, Input, Limits, LogNode, Output, Response, State, StopReason, StopWhen,
};

use crate::framing::{self, Frame};

/// Run the node pump against real stdin/stdout. Returns the process exit code
/// (0 on clean EOF, 3 on a malformed/unknown frame, 1 on an underlying I/O
/// fault on the streams themselves).
pub fn run(stop_when: StopWhen) -> u8 {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());
    match pump(&mut input, &mut output, stop_when) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("taut-shape-tool node: I/O error: {e}");
            1
        }
    }
}

/// The pump loop, generic over the byte streams so the integration test can
/// drive it directly (and so it is `#[cfg(test)]`-exercisable without a process
/// spawn, in addition to the real-pipe test).
pub fn pump<R: Read, W: Write>(
    input: &mut R,
    output: &mut W,
    stop_when: StopWhen,
) -> io::Result<u8> {
    let mut node = LogNode::new(Config { stop_when });
    // `log_id` is a service-level routing handle the node engine never sees
    // (D3): the engine's `Response` carries only `stream_id`. To echo a
    // consistent `log_id` back on every response addressed to a stream — the
    // direct `Read` answer *and* a later held-release triggered by
    // Push/Seal/Close/TimerExpired — we remember, per `stream_id`, the `log_id`
    // the stream was created with. Without this, a released read would echo an
    // empty `log_id` (no `Read` frame in scope), diverging from the same
    // stream's direct-read answer and freezing that inconsistency into the
    // oracle. A natural cross-language shell keeps this per-stream mapping too.
    let mut stream_log_ids: HashMap<String, String> = HashMap::new();

    loop {
        match framing::read_frame(input)? {
            Ok(None) => {
                // Clean EOF (incl. truncated tail): drain and exit 0.
                output.flush()?;
                return Ok(0);
            }
            Ok(Some(frame)) => {
                let engine_input = match decode_input(&frame) {
                    Ok(i) => i,
                    Err(msg) => {
                        eprintln!("taut-shape-tool node: {msg}");
                        output.flush()?;
                        return Ok(3);
                    }
                };
                // Remember this stream's `log_id` (a `Read` frame carries it),
                // so every response addressed to the stream — direct or a later
                // held-release — echoes the same handle.
                if let Some(echo) = read_echo(&frame) {
                    stream_log_ids.insert(echo.stream_id, echo.log_id);
                }
                for out in node.handle(engine_input) {
                    let (tag, body) = encode_output(&out, &stream_log_ids);
                    framing::write_frame(output, tag, &body)?;
                }
                output.flush()?;
            }
            Err(fe) => {
                eprintln!("taut-shape-tool node: {fe}");
                output.flush()?;
                return Ok(3);
            }
        }
    }
}

/// The `log_id`/`stream_id` pair to echo back onto a `read_response` — only a
/// `Read` frame carries them, so it is `None` for every other input.
struct ReadEcho {
    log_id: String,
    stream_id: String,
}

fn read_echo(frame: &Frame) -> Option<ReadEcho> {
    if !matches!(frame.tag, LogMsgType::Read) {
        return None;
    }
    let req = LogReadRequest::from_cbor(&frame.body);
    Some(ReadEcho {
        log_id: req.log_id,
        stream_id: req.stream_id,
    })
}

/// Decode an input frame into the engine's [`Input`]. Returns a human string on
/// a frame whose tag is an *output* kind (or otherwise not a valid input) —
/// mapped by the caller to exit 3.
fn decode_input(frame: &Frame) -> Result<Input, String> {
    Ok(match frame.tag {
        LogMsgType::Push => {
            let m = LogPush::from_cbor(&frame.body);
            Input::Push { payload: m.payload }
        }
        LogMsgType::Seal => Input::Seal,
        LogMsgType::Close => {
            // `LogClose.error` is present on the wire but the engine's on_close
            // takes an `Option<Error>`; convert (message string is preserved).
            let m = taut_shape::generated::LogClose::from_cbor(&frame.body);
            Input::Close {
                error: m.error.map(from_wire_error),
            }
        }
        LogMsgType::Read => {
            let m = LogReadRequest::from_cbor(&frame.body);
            Input::Read {
                stream_id: m.stream_id.as_str().into(),
                cursor: m.cursor.map(|c| Cursor::new(c.seq as u64)),
                limits: Limits {
                    max_records: m.max_records.map(|n| n as u32),
                    max_bytes: m.max_bytes.map(|n| n as u64),
                },
                timeout_ms: m.timeout_ms.map(|n| n as u64),
            }
        }
        LogMsgType::EndStream => {
            let m = LogEndStream::from_cbor(&frame.body);
            Input::EndStream {
                stream_id: m.stream_id.as_str().into(),
            }
        }
        LogMsgType::TimerExpired => {
            let m = LogTimerExpired::from_cbor(&frame.body);
            Input::TimerExpired {
                token: taut_shape::TimerToken(m.token as u64),
            }
        }
        LogMsgType::Evict => {
            let m = LogEvict::from_cbor(&frame.body);
            Input::Evict {
                up_to_seq: m.up_to_seq as u64,
            }
        }
        // Output-only tags arriving on the input side are a protocol error.
        LogMsgType::ReadResponse
        | LogMsgType::SetTimer
        | LogMsgType::CancelTimer
        | LogMsgType::ProducerStop
        | LogMsgType::Diagnostic => {
            return Err(format!(
                "output-only tag {} on the input channel",
                frame.tag.wire()
            ))
        }
    })
}

/// Encode one engine [`Output`] into its `(tag, CBOR body)`. `stream_log_ids`
/// maps each live stream to the `log_id` it was created with, so the
/// `read_response` can echo the same `log_id` on both the direct answer and a
/// later held-release (the engine's `Response` only tracks `stream_id`; `log_id`
/// is a framing-layer echo — D3).
fn encode_output(out: &Output, stream_log_ids: &HashMap<String, String>) -> (LogMsgType, Cbor) {
    match out {
        Output::Response(resp) => (
            LogMsgType::ReadResponse,
            read_response_cbor(resp, stream_log_ids).to_cbor(),
        ),
        Output::SetTimer { token, ms } => (
            LogMsgType::SetTimer,
            LogSetTimer {
                token: token.0 as i64,
                ms: *ms as i64,
            }
            .to_cbor(),
        ),
        Output::CancelTimer { token } => (
            LogMsgType::CancelTimer,
            LogCancelTimer {
                token: token.0 as i64,
            }
            .to_cbor(),
        ),
        Output::ProducerStop { reason } => (
            LogMsgType::ProducerStop,
            LogProducerStop {
                reason: to_wire_stop_reason(*reason),
            }
            .to_cbor(),
        ),
        // D18/D19: the diagnostic wraps the generated type verbatim.
        Output::Diagnostic(diag) => (LogMsgType::Diagnostic, diag.to_cbor()),
    }
}

/// Build a wire [`LogReadResponse`] from the engine's [`Response`]. `stream_id`
/// comes from the engine's response; `log_id` is looked up from the per-stream
/// map populated by each `Read` frame, so it is consistent across the direct
/// answer and any later held-release (empty only if the stream is somehow
/// unknown, which cannot happen — a `Response` implies a prior `Read`).
fn read_response_cbor(
    resp: &Response,
    stream_log_ids: &HashMap<String, String>,
) -> LogReadResponse {
    let stream_id = resp.stream_id.0.to_string();
    let log_id = stream_log_ids.get(&stream_id).cloned().unwrap_or_default();
    LogReadResponse {
        log_id,
        stream_id,
        records: resp
            .records
            .iter()
            .map(|r| LogRecord {
                seq: r.seq as i64,
                payload: r.payload.clone(),
            })
            .collect(),
        next_cursor: LogCursor {
            seq: resp.next_cursor.seq as i64,
        },
        state: to_wire_state(resp.state),
        error: resp.error.as_ref().map(to_wire_error),
    }
}

// ── core ↔ generated conversions ────────────────────────────────────────────

fn to_wire_state(s: State) -> taut_shape::generated::LogState {
    use taut_shape::generated::LogState as W;
    match s {
        State::Data => W::Data,
        State::WouldBlock => W::WouldBlock,
        State::Eof => W::Eof,
        State::Closed => W::Closed,
        State::Failed => W::Failed,
        State::Expired => W::Expired,
    }
}

fn to_wire_stop_reason(r: StopReason) -> taut_shape::generated::LogStopReason {
    use taut_shape::generated::LogStopReason as W;
    match r {
        StopReason::LastReaderGone => W::LastReaderGone,
        StopReason::Closed => W::Closed,
        StopReason::Failed => W::Failed,
    }
}

fn to_wire_error(e: &taut_shape::Error) -> taut_shape::generated::LogError {
    use taut_shape::generated::LogErrorCode as W;
    use taut_shape::ErrorCode as C;
    taut_shape::generated::LogError {
        code: match e.code {
            C::UnknownLog => W::UnknownLog,
            C::ProducerError => W::ProducerError,
            C::Internal => W::Internal,
            // `ErrorCode` is #[non_exhaustive]; a future variant maps to the
            // generic `internal` wire code rather than failing to compile.
            _ => W::Internal,
        },
        message: e.message.clone(),
    }
}

fn from_wire_error(e: taut_shape::generated::LogError) -> taut_shape::Error {
    use taut_shape::generated::LogErrorCode as W;
    use taut_shape::ErrorCode as C;
    taut_shape::Error {
        code: match e.code {
            W::UnknownLog => C::UnknownLog,
            W::ProducerError => C::ProducerError,
            W::Internal => C::Internal,
        },
        message: e.message,
    }
}
