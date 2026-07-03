//! `client` mode: the reading side — the cursor loop against a running node.
//!
//! The client owns the *node's stdin* (its own stdout, crossed-pipe): it sends
//! [`LogReadRequest`] frames toward the node and reads [`LogReadResponse`]
//! frames back on its own stdin. Because interop scenarios avoid real timers
//! (Oracle §7), every request is a **held read** (`timeout_ms` omitted): the
//! node parks it and answers only when data/lifecycle arrives.
//!
//! The loop (per the shared tool contract):
//!
//! 1. Send `LogReadRequest{log_id "log-A", stream_id S, cursor {seq}, …}`.
//! 2. Await a response frame on stdin. Log it to the OOB stderr transcript
//!    (jsoncodec form, one object per line).
//! 3. On `data`  → advance the cursor to `next_cursor.seq`, go to 1.
//!    On `would_block` → same (a probe/timeout answer; re-hold from next_cursor).
//!    On `expired` → resume from `next_cursor` (the earliest resumable position,
//!    D9) and re-hold — `expired` is a state, not a terminal, so an evict
//!    mid-stream is survivable.
//!    On the terminal `eof|closed|failed` → emit a final `state` line, exit 0.
//! 4. Clean EOF on stdin before a terminal answer → emit `state: "eof"`, exit 0.
//!
//! `--script` drives the producer from the client side: after the k-th request
//! the client has sent, it writes the scripted producer frames (push/seal/close/
//! evict) to its stdout too, interleaved with its own requests. This lets a
//! same-process or crossed-pipe scenario feed the peer node without a separate
//! producer.

use std::io::{self, BufWriter, Read, Write};

use taut_shape::generated::{LogCursor, LogMsgType, LogReadRequest, LogReadResponse, LogState};
use taut_shape::Input;

use crate::framing;
use crate::json::{self, Json};
use crate::jsoncodec;
use crate::script::{Pending, Script};

/// `client`-mode knobs from the CLI.
pub struct Opts {
    pub log_id: String,
    pub stream_id: String,
    pub from: u64,
    pub max_records: Option<u32>,
    /// Optional `--script`: producer frames written to stdout after the k-th
    /// request the client sends (client-side producer injection, §7).
    pub script: Option<Script>,
}

/// Run the client loop against real stdin/stdout. Exit codes match the node:
/// 0 on a terminal answer or clean EOF, 3 on a malformed response frame, 1 on an
/// underlying I/O fault.
pub fn run(opts: Opts) -> u8 {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut input = stdin.lock();
    let mut output = BufWriter::new(stdout.lock());
    let mut transcript = stderr.lock();
    match drive(&mut input, &mut output, &mut transcript, opts) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("taut-shape-tool client: I/O error: {e}");
            1
        }
    }
}

/// The cursor loop, generic over the streams so the integration test drives it
/// in-process over real OS pipes.
pub fn drive<R: Read, W: Write, T: Write>(
    input: &mut R,
    output: &mut W,
    transcript: &mut T,
    opts: Opts,
) -> io::Result<u8> {
    let mut cursor = opts.from;
    let mut pending = opts.script.map(Pending::new);
    // Count of request frames the client has sent (drives `--script`).
    let mut sent: u64 = 0;

    'sends: loop {
        // 1) Send one held LogReadRequest from the current cursor.
        let req = LogReadRequest {
            log_id: opts.log_id.clone(),
            stream_id: opts.stream_id.clone(),
            cursor: Some(LogCursor { seq: cursor as i64 }),
            max_records: opts.max_records.map(|n| n as i64),
            max_bytes: None,
            timeout_ms: None, // held read (no timers in interop, §7)
        };
        framing::write_frame(output, LogMsgType::Read, &req.to_cbor())?;

        // After the k-th request, fire any due producer injections to stdout.
        sent += 1;
        if let Some(p) = &mut pending {
            for inp in p.take_due(sent) {
                write_producer_frame(output, &inp)?;
            }
        }
        output.flush()?;

        // 2) Await *this request's* read answer. The node writes every engine
        //    output as a frame, so before the answer we may see control frames on
        //    the data channel; those are handled here WITHOUT re-sending a request
        //    (re-sending only happens when we advance the cursor and loop `'sends`).
        let resp = loop {
            let frame = match framing::read_frame(input)? {
                Ok(Some(f)) => f,
                Ok(None) => {
                    // Clean EOF before a terminal answer: the node hung up. Treat
                    // as eof and exit 0 (the producer went away without a
                    // lifecycle frame — a truncated but non-erroneous end).
                    emit_final(transcript, "eof")?;
                    return Ok(0);
                }
                Err(fe) => {
                    eprintln!("taut-shape-tool client: {fe}");
                    return Ok(3);
                }
            };
            match frame.tag {
                LogMsgType::ReadResponse => break LogReadResponse::from_cbor(&frame.body),
                // A terminal producer signal — the producer is done.
                LogMsgType::ProducerStop => {
                    emit_final(transcript, "producer_stop")?;
                    return Ok(0);
                }
                // Informational; keep awaiting the read answer (do NOT re-send).
                LogMsgType::SetTimer | LogMsgType::CancelTimer | LogMsgType::Diagnostic => continue,
                // An input-only tag the node would never emit ⇒ protocol error.
                other => {
                    eprintln!(
                        "taut-shape-tool client: unexpected input-only tag {} on the response channel",
                        other.wire()
                    );
                    return Ok(3);
                }
            }
        };

        // 3) OOB transcript: log every received response verbatim.
        let _ = writeln!(transcript, "{}", jsoncodec::read_response_to_json(&resp));

        // 4) Advance or terminate on the response state.
        match resp.state {
            LogState::Data | LogState::WouldBlock => {
                // Advance to the reported next position and re-hold.
                cursor = resp.next_cursor.seq.max(0) as u64;
                continue 'sends;
            }
            LogState::Eof => return terminal(transcript, "eof"),
            LogState::Closed => return terminal(transcript, "closed"),
            LogState::Failed => return terminal(transcript, "failed"),
            LogState::Expired => {
                // A cursor invalidated by eviction is a state, not an error: the
                // node's next_cursor is the earliest resumable position. Resume
                // from there rather than exiting, so an evict mid-stream is
                // survivable (matches the corpus `evict_then_expired` shape).
                cursor = resp.next_cursor.seq.max(0) as u64;
                continue 'sends;
            }
        }
    }
}

/// Serialize a scripted producer [`Input`] as one data-channel frame toward the
/// node (client-side `--script` injection).
fn write_producer_frame<W: Write>(output: &mut W, input: &Input) -> io::Result<()> {
    let (tag, body) = crate::node::encode_input(input);
    framing::write_frame(output, tag, &body)
}

/// Emit the final `{type: "client_final", state: …}` transcript line and return
/// exit 0.
fn terminal<T: Write>(transcript: &mut T, state: &str) -> io::Result<u8> {
    emit_final(transcript, state)?;
    Ok(0)
}

fn emit_final<T: Write>(transcript: &mut T, state: &str) -> io::Result<()> {
    let mut m = std::collections::BTreeMap::new();
    m.insert("type".to_string(), json::s("client_final"));
    m.insert("state".to_string(), json::s(state));
    writeln!(transcript, "{}", Json::Obj(m))
}
