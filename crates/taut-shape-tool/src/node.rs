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
use crate::jsoncodec;
use crate::runtime::{
    require_engine_shape, AdapterCode, AdapterDiagnostic, EngineAdapter, EngineEffect,
    EngineEmission, EngineRuntime, TeardownAction, TimerAction,
};
use crate::script::{Pending, Script};

/// `node`-mode construction knobs collected from the CLI.
pub struct Opts {
    pub stop_when: StopWhen,
    /// Optional `--script`: a producer-injection script fired against the local
    /// engine after the k-th client frame is processed (interop driver, §7).
    pub script: Option<Script<Input>>,
}

struct LogAdapter {
    node: LogNode,
    stream_log_ids: HashMap<String, String>,
}

impl LogAdapter {
    fn new(stop_when: StopWhen) -> Self {
        require_engine_shape("log").expect("the built-in log adapter must be registered");
        Self {
            node: LogNode::new(Config { stop_when }),
            stream_log_ids: HashMap::new(),
        }
    }

    fn injected_log_id(&self, out: &Output) -> String {
        match out {
            Output::Response(resp) => self
                .stream_log_ids
                .get(resp.stream_id.0.as_ref())
                .cloned()
                .unwrap_or_default(),
            _ => String::new(),
        }
    }
}

impl EngineAdapter for LogAdapter {
    type FrameIn = Frame;
    type Input = Input;
    type Output = Output;
    type FrameOut = (u8, Cbor);

    fn shape(&self) -> &'static str {
        "log"
    }

    fn decode_input(&mut self, frame: Frame) -> Result<Input, AdapterDiagnostic> {
        if let Some(echo) = read_echo(&frame) {
            self.stream_log_ids.insert(echo.stream_id, echo.log_id);
        }
        decode_input(&frame)
    }

    fn dispatch(&mut self, input: Input) -> Vec<Output> {
        self.node.handle(input)
    }

    fn encode_output(&self, output: &Output) -> EngineEffect<(u8, Cbor)> {
        let timer = match output {
            Output::SetTimer { token, ms } => Some(TimerAction::Set {
                token: token.0 as i64,
                delay_ms: *ms as i64,
            }),
            Output::CancelTimer { token } => Some(TimerAction::Cancel {
                token: token.0 as i64,
            }),
            _ => None,
        };
        let teardown = match output {
            Output::ProducerStop { reason } => Some(TeardownAction {
                reason: format!("{reason:?}"),
            }),
            _ => None,
        };
        let (tag, body) = encode_output(output, &self.stream_log_ids);
        EngineEffect {
            frame: (tag.wire() as u8, body),
            timer,
            teardown,
        }
    }

    fn finish(&mut self) -> Vec<Output> {
        Vec::new()
    }
}

/// Run the node pump against real stdin/stdout. Returns the process exit code
/// (0 on clean EOF, 3 on a malformed/unknown frame, 1 on an underlying I/O
/// fault on the streams themselves).
pub fn run(opts: Opts) -> u8 {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut input = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());
    let mut transcript = stderr.lock();
    match pump(&mut input, &mut output, &mut transcript, opts) {
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
///
/// `transcript` receives one OOB JSONL line per injected producer message and
/// its outputs (the interop control channel, §7); it is `sink()`-able when a
/// caller does not want a transcript. Only the `--script` path writes to it —
/// ordinary client-driven frames are already visible as the data-channel frames.
pub fn pump<R: Read, W: Write, T: Write>(
    input: &mut R,
    output: &mut W,
    transcript: &mut T,
    opts: Opts,
) -> io::Result<u8> {
    let mut runtime = EngineRuntime::new(LogAdapter::new(opts.stop_when));

    let mut pending = opts.script.map(Pending::new);
    // Count of client (stdin) frames processed so far; drives `after_frames`.
    let mut frames: u64 = 0;

    // `after_frames: 0` fires *before* any client frame (Oracle §7).
    if let Some(p) = &mut pending {
        let due = p.take_due(0);
        inject(&mut runtime, output, transcript, due)?;
        output.flush()?;
    }

    loop {
        match framing::read_frame(input)? {
            Ok(None) => {
                // Clean EOF (incl. truncated tail). Flush any remaining scripted
                // injections whose trigger count was never reached (a fallback so
                // a trailing seal/close is not silently dropped), then exit 0.
                if let Some(p) = &mut pending {
                    let rest = p.drain_remaining();
                    inject(&mut runtime, output, transcript, rest)?;
                }
                write_emissions(output, runtime.finish())?;
                output.flush()?;
                return Ok(0);
            }
            Ok(Some(frame)) => {
                let emissions = match runtime.process(frame) {
                    Ok(emissions) => emissions,
                    Err(diagnostic) => {
                        eprintln!("taut-shape-tool node: {diagnostic}");
                        output.flush()?;
                        return Ok(3);
                    }
                };
                write_emissions(output, emissions)?;
                // One client frame processed: fire any injections now due.
                frames += 1;
                if let Some(p) = &mut pending {
                    let due = p.take_due(frames);
                    inject(&mut runtime, output, transcript, due)?;
                }
                output.flush()?;
            }
            Err(fe) => {
                eprintln!("taut-shape-tool node: {}: {fe}", fe.code());
                output.flush()?;
                return Ok(3);
            }
        }
    }
}

/// Feed scripted producer [`Input`]s to the engine and write their outputs as
/// data-channel frames, mirroring each to the OOB transcript. `stream_log_ids`
/// is read-only here: producer inputs create no new streams, so a released
/// read's `log_id` was already recorded by the `Read` that parked it.
fn inject<W: Write, T: Write>(
    runtime: &mut EngineRuntime<LogAdapter>,
    output: &mut W,
    transcript: &mut T,
    inputs: Vec<Input>,
) -> io::Result<()> {
    for input in inputs {
        for emission in runtime.dispatch(input) {
            // Best-effort control-channel echo; the data channel is the pin.
            let log_id = runtime.adapter().injected_log_id(&emission.output);
            let _ = writeln!(
                transcript,
                "{}",
                jsoncodec::output_to_json(&emission.output, &log_id)
            );
            // The conformance CLI preserves these as wire frames; an embedded
            // host may route the same shape-neutral annotations to its runtime.
            let _runtime_actions = (&emission.effect.timer, &emission.effect.teardown);
            let EngineEffect {
                frame: (tag, body),
                timer: _,
                teardown: _,
            } = emission.effect;
            framing::write_frame(output, tag, &body)?;
        }
    }
    Ok(())
}

fn write_emissions<W: Write>(
    output: &mut W,
    emissions: Vec<EngineEmission<Output, (u8, Cbor)>>,
) -> io::Result<()> {
    for emission in emissions {
        let _runtime_actions = (&emission.effect.timer, &emission.effect.teardown);
        let EngineEffect {
            frame: (tag, body),
            timer: _,
            teardown: _,
        } = emission.effect;
        framing::write_frame(output, tag, &body)?;
    }
    Ok(())
}

/// The `log_id`/`stream_id` pair to echo back onto a `read_response` — only a
/// `Read` frame carries them, so it is `None` for every other input.
struct ReadEcho {
    log_id: String,
    stream_id: String,
}

fn read_echo(frame: &Frame) -> Option<ReadEcho> {
    if LogMsgType::from_wire(frame.tag as i64).ok()? != LogMsgType::Read {
        return None;
    }
    // Fail-closed decode: a malformed body yields None (no echo) rather than a
    // panic.
    let req = LogReadRequest::from_cbor(&frame.body).ok()?;
    Some(ReadEcho {
        log_id: req.log_id,
        stream_id: req.stream_id,
    })
}

/// Decode an input frame into the engine's [`Input`]. Returns a human string on
/// a frame whose tag is an *output* kind (or otherwise not a valid input) —
/// mapped by the caller to exit 3.
fn decode_input(frame: &Frame) -> Result<Input, AdapterDiagnostic> {
    let tag = LogMsgType::from_wire(frame.tag as i64).map_err(|_| {
        AdapterDiagnostic::new(
            AdapterCode::UnknownTag,
            "log",
            format!("unknown frame tag byte {}", frame.tag),
        )
    })?;
    // Fail-closed: a body that does not decode to the expected shape is mapped
    // to the `Err(String)` the caller turns into exit 3 — never a panic.
    let bad = |e: taut_shape::cbor::DecodeError| {
        AdapterDiagnostic::new(
            AdapterCode::MalformedMessage,
            "log",
            format!("malformed {tag:?} body: {e}"),
        )
    };
    Ok(match tag {
        LogMsgType::Push => {
            let m = LogPush::from_cbor(&frame.body).map_err(bad)?;
            Input::Push { payload: m.payload }
        }
        LogMsgType::Seal => Input::Seal,
        LogMsgType::Close => {
            // `LogClose.error` is present on the wire but the engine's on_close
            // takes an `Option<Error>`; convert (message string is preserved).
            let m = taut_shape::generated::LogClose::from_cbor(&frame.body).map_err(bad)?;
            Input::Close {
                error: m.error.map(from_wire_error),
            }
        }
        LogMsgType::Read => {
            let m = LogReadRequest::from_cbor(&frame.body).map_err(bad)?;
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
            let m = LogEndStream::from_cbor(&frame.body).map_err(bad)?;
            Input::EndStream {
                stream_id: m.stream_id.as_str().into(),
            }
        }
        LogMsgType::TimerExpired => {
            let m = LogTimerExpired::from_cbor(&frame.body).map_err(bad)?;
            Input::TimerExpired {
                token: taut_shape::TimerToken(m.token as u64),
            }
        }
        LogMsgType::Evict => {
            let m = LogEvict::from_cbor(&frame.body).map_err(bad)?;
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
            return Err(AdapterDiagnostic::new(
                AdapterCode::DirectionViolation,
                "log",
                format!("output-only tag {} on the input channel", tag.wire()),
            ))
        }
    })
}

/// Encode an engine [`Input`] back into its `(tag, CBOR body)` — the inverse of
/// [`decode_input`]. The producer-side inputs a `--script` drives carry no
/// `log_id`, so the `Read`/`EndStream` arms stamp the shared interop `"log-A"`
/// handle (the only log in single-node interop). Used by the client's `--script`
/// path to write producer frames toward the peer node.
pub fn encode_input(input: &Input) -> (LogMsgType, Cbor) {
    use taut_shape::generated::{LogClose, LogPush as WPush};
    match input {
        Input::Push { payload } => (
            LogMsgType::Push,
            WPush {
                payload: payload.clone(),
            }
            .to_cbor(),
        ),
        Input::Seal => (
            LogMsgType::Seal,
            taut_shape::generated::LogSeal {}.to_cbor(),
        ),
        Input::Close { error } => (
            LogMsgType::Close,
            LogClose {
                error: error.as_ref().map(to_wire_error),
            }
            .to_cbor(),
        ),
        Input::Evict { up_to_seq } => (
            LogMsgType::Evict,
            LogEvict {
                up_to_seq: *up_to_seq as i64,
            }
            .to_cbor(),
        ),
        Input::Read {
            stream_id,
            cursor,
            limits,
            timeout_ms,
        } => (
            LogMsgType::Read,
            LogReadRequest {
                log_id: "log-A".to_string(),
                stream_id: stream_id.0.to_string(),
                cursor: cursor.map(|c| LogCursor { seq: c.seq as i64 }),
                max_records: limits.max_records.map(|n| n as i64),
                max_bytes: limits.max_bytes.map(|n| n as i64),
                timeout_ms: timeout_ms.map(|n| n as i64),
            }
            .to_cbor(),
        ),
        Input::EndStream { stream_id } => (
            LogMsgType::EndStream,
            LogEndStream {
                log_id: "log-A".to_string(),
                stream_id: stream_id.0.to_string(),
            }
            .to_cbor(),
        ),
        Input::TimerExpired { token } => (
            LogMsgType::TimerExpired,
            LogTimerExpired {
                token: token.0 as i64,
            }
            .to_cbor(),
        ),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_adapter_exposes_timer_and_teardown_effects() {
        let mut runtime = EngineRuntime::new(LogAdapter::new(StopWhen::LastReader));
        let held = runtime.dispatch(Input::Read {
            stream_id: "s1".into(),
            cursor: Some(Cursor::new(0)),
            limits: Limits::default(),
            timeout_ms: Some(25),
        });
        assert_eq!(
            held[0].effect.timer,
            Some(TimerAction::Set {
                token: 1,
                delay_ms: 25,
            })
        );
        assert_eq!(held[0].effect.frame.0, LogMsgType::SetTimer.wire() as u8);

        let closed = runtime.dispatch(Input::Close { error: None });
        assert!(closed
            .iter()
            .any(|emission| emission.effect.teardown.is_some()));
    }
}
