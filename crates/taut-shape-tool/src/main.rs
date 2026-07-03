//! `taut-shape-tool` — the conformance/interop CLI (Phase 4).
//!
//! Modes (InitialPlan §4, S4.1/S4.2):
//!   * `node`   — run a [`taut_shape::LogNode`] behind the stdin/stdout framing.
//!     One frame = `u32-LE len | tag byte | CBOR body`; each input is `handle`d
//!     and its outputs are written back as frames. `--script` drives the
//!     producer against the local engine after the k-th client frame (§7).
//!   * `client` — the reading side (the cursor loop) against a running node:
//!     held `LogReadRequest`s out, `LogReadResponse`s in, advance/terminate,
//!     with an OOB JSONL transcript on stderr. `--script` drives the producer
//!     from the client side (frames toward the peer node).
//!   * `gen` / `check` — stubs (each prints what it will do, exit 2).
//!
//! Exit codes: 0 = clean EOF / terminal answer; 2 = usage / unimplemented stub;
//! 3 = a malformed or unknown frame; 1 = an underlying I/O fault.

use std::process::ExitCode;

use taut_shape::StopWhen;

mod client;
mod framing;
mod json;
mod jsoncodec;
mod node;
mod script;

use script::Script;

const USAGE: &str = "\
taut-shape-tool — Taut `log`-shape conformance/interop CLI

USAGE:
    taut-shape-tool <MODE> [OPTIONS]

MODES:
    gen       Emit oracle vectors from scripted scenarios (reference impl only)
    check     Replay the committed oracle corpus through the engine; report pass/fail
    node      Run a LogNode behind the stdin/stdout framing
    client    Run the reading side (the cursor loop) against a node

`node` OPTIONS:
    --stop-when <last_reader|explicit>   ProducerStop policy (default: last_reader)
    --script <FILE>                      Producer-injection script (interop, §7)

`client` OPTIONS:
    --stream-id <S>       Stream id to read as (required)
    --from <SEQ>          Starting cursor seq (required)
    --max-records <N>     Per-read record cap (optional)
    --log-id <ID>         Log id on requests (default: log-A)
    --script <FILE>       Producer-injection script written toward the node (§7)

FRAMING (data channel, stdin↔stdout):
    one frame = u32-LE byte length, then 1 tag byte (LogMsgType wire value),
    then the message's CBOR body. `length` covers the tag byte + body.

`--script` FORMAT (JSON):
    an ordered list of { \"after_frames\": k, \"inputs\": [ <jsoncodec msg>, … ] };
    after the k-th client frame is processed, the listed producer messages
    (push/seal/close/evict, taut jsoncodec form) are injected deterministically.
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = match args.first() {
        Some(m) => m.as_str(),
        None => {
            eprint!("{USAGE}");
            return ExitCode::from(2);
        }
    };

    match mode {
        "node" => run_node(&args[1..]),
        "client" => run_client(&args[1..]),
        "gen" => stub("gen", "emit oracle vectors from scripted scenarios (reference impl only)"),
        "check" => stub(
            "check",
            "replay the committed oracle corpus through the engine and report pass/fail",
        ),
        other => {
            eprintln!("taut-shape-tool: unknown mode {other:?}\n");
            eprint!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

/// Parse `node`'s args and run the pump. Recognises `--stop-when` and `--script`.
fn run_node(args: &[String]) -> ExitCode {
    let mut stop_when = StopWhen::LastReader; // task default: last_reader
    let mut script_path: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--stop-when" => {
                let val = match args.get(i + 1) {
                    Some(v) => v.as_str(),
                    None => return usage_err("node", "--stop-when needs a value"),
                };
                stop_when = match val {
                    "last_reader" => StopWhen::LastReader,
                    "explicit" => StopWhen::ExplicitOnly,
                    other => {
                        return usage_err(
                            "node",
                            &format!("--stop-when expects last_reader|explicit, got {other:?}"),
                        )
                    }
                };
                i += 2;
            }
            "--script" => {
                match args.get(i + 1) {
                    Some(v) => script_path = Some(v.clone()),
                    None => return usage_err("node", "--script needs a FILE"),
                }
                i += 2;
            }
            other => return usage_err("node", &format!("unknown option {other:?}")),
        }
    }

    let script = match load_script("node", script_path) {
        Ok(s) => s,
        Err(code) => return code,
    };
    ExitCode::from(node::run(node::Opts { stop_when, script }))
}

/// Parse `client`'s args and run the cursor loop.
fn run_client(args: &[String]) -> ExitCode {
    let mut stream_id: Option<String> = None;
    let mut from: Option<u64> = None;
    let mut max_records: Option<u32> = None;
    let mut log_id = "log-A".to_string();
    let mut script_path: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        let need = |i: usize| -> Result<&String, ExitCode> {
            args.get(i + 1)
                .ok_or_else(|| usage_err_code("client", &format!("{} needs a value", args[i])))
        };
        match args[i].as_str() {
            "--stream-id" => match need(i) {
                Ok(v) => {
                    stream_id = Some(v.clone());
                    i += 2;
                }
                Err(c) => return c,
            },
            "--from" => match need(i) {
                Ok(v) => match v.parse::<u64>() {
                    Ok(n) => {
                        from = Some(n);
                        i += 2;
                    }
                    Err(_) => return usage_err("client", "--from must be a non-negative integer"),
                },
                Err(c) => return c,
            },
            "--max-records" => match need(i) {
                Ok(v) => match v.parse::<u32>() {
                    Ok(n) => {
                        max_records = Some(n);
                        i += 2;
                    }
                    Err(_) => return usage_err("client", "--max-records must be a u32"),
                },
                Err(c) => return c,
            },
            "--log-id" => match need(i) {
                Ok(v) => {
                    log_id = v.clone();
                    i += 2;
                }
                Err(c) => return c,
            },
            "--script" => match need(i) {
                Ok(v) => {
                    script_path = Some(v.clone());
                    i += 2;
                }
                Err(c) => return c,
            },
            other => return usage_err("client", &format!("unknown option {other:?}")),
        }
    }

    let stream_id = match stream_id {
        Some(s) => s,
        None => return usage_err("client", "--stream-id is required"),
    };
    let from = match from {
        Some(n) => n,
        None => return usage_err("client", "--from is required"),
    };
    let script = match load_script("client", script_path) {
        Ok(s) => s,
        Err(code) => return code,
    };

    ExitCode::from(client::run(client::Opts {
        log_id,
        stream_id,
        from,
        max_records,
        script,
    }))
}

/// Load and parse an optional `--script` file, mapping any parse error to a
/// usage exit (2).
fn load_script(mode: &str, path: Option<String>) -> Result<Option<Script>, ExitCode> {
    match path {
        None => Ok(None),
        Some(p) => match Script::load(&p) {
            Ok(s) => Ok(Some(s)),
            Err(e) => Err(usage_err_code(mode, &format!("--script: {e}"))),
        },
    }
}

fn usage_err(mode: &str, msg: &str) -> ExitCode {
    eprintln!("taut-shape-tool {mode}: {msg}");
    ExitCode::from(2)
}

fn usage_err_code(mode: &str, msg: &str) -> ExitCode {
    eprintln!("taut-shape-tool {mode}: {msg}");
    ExitCode::from(2)
}

/// A not-yet-implemented mode: name what it will do, exit 2.
fn stub(name: &str, what: &str) -> ExitCode {
    eprintln!("taut-shape-tool: `{name}` is not implemented yet — it will {what}.");
    ExitCode::from(2)
}
