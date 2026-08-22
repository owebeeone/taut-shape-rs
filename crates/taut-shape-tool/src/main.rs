//! `taut-shape-tool` — the conformance/interop CLI (Phase 4).
//!
//! Modes (InitialPlan §4, S4.1/S4.2):
//!   * `node`   — run a selected shape engine behind stdin/stdout framing.
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

mod atom_json;
mod atom_tool;
mod client;
mod crdt_json;
mod crdt_tool;
mod framing;
mod json;
mod jsoncodec;
mod node;
mod runtime;
mod script;
#[cfg(test)]
mod snapshot_delta_json;
mod stream_json;
mod stream_tool;
mod swmr_json;
mod swmr_tool;
mod value_json;
mod value_tool;

use runtime::require_engine_shape;
use script::Script;

const USAGE: &str = "\
taut-shape-tool — Taut shape conformance/interop CLI

USAGE:
    taut-shape-tool <MODE> [OPTIONS]

MODES:
    gen       Emit oracle vectors from scripted scenarios (reference impl only)
    check     Replay the committed oracle corpus through the engine; report pass/fail
    node      Run a selected engine behind the stdin/stdout framing
    client    Run a selected engine's reading side against a node

`node` OPTIONS:
    --shape <NAME>                       Engine shape (default: log)
    --stop-when <last_reader|explicit>   ProducerStop policy (default: last_reader)
    --script <FILE>                      Producer-injection script (interop, §7)
    --capacity-records <N>               Stream ring capacity (default: 64)
    --max-deltas <N>                     SWMR retained-delta bound (optional)
    --max-pending <N>                    CRDT causal pending bound (default: 1024)

`client` OPTIONS:
    --shape <NAME>        Engine shape (default: log)
    --stream-id <S>       Stream id to read as (required)
    --from <SEQ>          Starting cursor seq (required)
    --max-records <N>     Per-read record cap (optional)
    --max-bytes <N>       Stream per-read byte cap (optional)
    --log-id <ID>         Log id on requests (default: log-A)
    --value-id <ID>       Value register id (default: v-A)
    --atom-id <ID>        Atom id (default: atom-A)
    --swmr-id <ID>        SWMR object id (default: swmr-A)
    --crdt-id <ID>        CRDT object id (default: crdt-A)
    --replica-script <FILE> Local CRDT bootstrap/operations to sync
    --epoch <N>           SWMR cursor epoch (requires --from; default: 0)
    --extra-stream-id <S> Add an atom reader (repeatable)
    --timeout-ms <MS>     Atom/stream read timeout (optional; 0 probes)
    --reconnect-dropped   Rejoin once after a stream slow-reader drop
    --pause-stream-id <S> Pause re-reading this stream after its first data
    --resume-after-data <N> Resume paused streams after N data responses
    --reads <N>           Immediate value reads to issue (default: 1)
    --script <FILE>       Producer-injection script written toward the node (§7)

SUPPORTED ENGINE SHAPES:
    atom, crdt, log, snapshot_delta, stream, swmr, text_crdt, value

FRAMING (data channel, stdin↔stdout):
    one frame = u32-LE byte length, then 1 shape-local tag byte,
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
        "gen" => stub(
            "gen",
            "emit oracle vectors from scripted scenarios (reference impl only)",
        ),
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
    let mut shape = "log".to_string();
    let mut stop_when = StopWhen::LastReader; // task default: last_reader
    let mut script_path: Option<String> = None;
    let mut capacity_records: usize = 64;
    let mut max_deltas: Option<usize> = None;
    let mut max_pending: usize = 1024;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--shape" => {
                match args.get(i + 1) {
                    Some(v) => shape = v.clone(),
                    None => return usage_err("node", "--shape needs a NAME"),
                }
                i += 2;
            }
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
            "--capacity-records" => {
                let Some(value) = args.get(i + 1) else {
                    return usage_err("node", "--capacity-records needs a value");
                };
                capacity_records = match value.parse::<usize>() {
                    Ok(value) if value > 0 => value,
                    _ => return usage_err("node", "--capacity-records must be positive"),
                };
                i += 2;
            }
            "--max-deltas" => {
                let Some(value) = args.get(i + 1) else {
                    return usage_err("node", "--max-deltas needs a value");
                };
                max_deltas = match value.parse::<usize>() {
                    Ok(value) => Some(value),
                    _ => return usage_err("node", "--max-deltas must be non-negative"),
                };
                i += 2;
            }
            "--max-pending" => {
                let Some(value) = args.get(i + 1) else {
                    return usage_err("node", "--max-pending needs a value");
                };
                max_pending = match value.parse::<usize>() {
                    Ok(value) => value,
                    _ => return usage_err("node", "--max-pending must be non-negative"),
                };
                i += 2;
            }
            other => return usage_err("node", &format!("unknown option {other:?}")),
        }
    }

    if let Err(code) = select_shape("node", &shape) {
        return code;
    }

    let script = match load_script("node", script_path) {
        Ok(s) => s,
        Err(code) => return code,
    };
    match shape.as_str() {
        "crdt" | "text_crdt" => {
            let script = match decode_script("node", script, crdt_json::input_from_json) {
                Ok(script) => script,
                Err(code) => return code,
            };
            ExitCode::from(crdt_tool::run_node(crdt_tool::NodeOpts {
                max_pending,
                script,
            }))
        }
        "atom" => {
            let script = match decode_script("node", script, atom_json::input_from_json) {
                Ok(script) => script,
                Err(code) => return code,
            };
            ExitCode::from(atom_tool::run_node(atom_tool::NodeOpts {
                stop_when,
                script,
            }))
        }
        "log" => {
            let script = match decode_script("node", script, jsoncodec::input_from_json) {
                Ok(script) => script,
                Err(code) => return code,
            };
            ExitCode::from(node::run(node::Opts { stop_when, script }))
        }
        "stream" => {
            let script = match decode_script("node", script, stream_json::input_from_json) {
                Ok(script) => script,
                Err(code) => return code,
            };
            ExitCode::from(stream_tool::run_node(stream_tool::NodeOpts {
                stop_when,
                capacity_records,
                script,
            }))
        }
        "swmr" | "snapshot_delta" => {
            let script = match decode_script("node", script, swmr_json::input_from_json) {
                Ok(script) => script,
                Err(code) => return code,
            };
            ExitCode::from(swmr_tool::run_node(swmr_tool::NodeOpts {
                stop_when,
                max_deltas: max_deltas.or_else(|| (shape == "snapshot_delta").then_some(64)),
                script,
                expire_profile: shape == "snapshot_delta",
            }))
        }
        "value" => {
            let script = match decode_script("node", script, value_json::input_from_json) {
                Ok(script) => script,
                Err(code) => return code,
            };
            ExitCode::from(value_tool::run_node(value_tool::NodeOpts { script }))
        }
        _ => unreachable!("shape was validated against the exact registry"),
    }
}

/// Parse `client`'s args and run the cursor loop.
fn run_client(args: &[String]) -> ExitCode {
    let mut shape = "log".to_string();
    let mut stream_id: Option<String> = None;
    let mut from: Option<i64> = None;
    let mut max_records: Option<u32> = None;
    let mut log_id = "log-A".to_string();
    let mut value_id = "v-A".to_string();
    let mut atom_id = "atom-A".to_string();
    let mut swmr_id = "swmr-A".to_string();
    let mut crdt_id = "crdt-A".to_string();
    let mut replica_script: Option<String> = None;
    let mut epoch: Option<i64> = None;
    let mut extra_stream_ids: Vec<String> = Vec::new();
    let mut timeout_ms: Option<i64> = None;
    let mut max_bytes: Option<i64> = None;
    let mut reconnect_dropped = false;
    let mut pause_stream_ids: Vec<String> = Vec::new();
    let mut resume_after_data: Option<usize> = None;
    let mut reads: u32 = 1;
    let mut script_path: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        let need = |i: usize| -> Result<&String, ExitCode> {
            args.get(i + 1)
                .ok_or_else(|| usage_err_code("client", &format!("{} needs a value", args[i])))
        };
        match args[i].as_str() {
            "--shape" => match need(i) {
                Ok(v) => {
                    shape = v.clone();
                    i += 2;
                }
                Err(c) => return c,
            },
            "--stream-id" => match need(i) {
                Ok(v) => {
                    stream_id = Some(v.clone());
                    i += 2;
                }
                Err(c) => return c,
            },
            "--from" => match need(i) {
                Ok(v) => match v.parse::<i64>() {
                    Ok(n) => {
                        from = Some(n);
                        i += 2;
                    }
                    Err(_) => return usage_err("client", "--from must be an i64"),
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
            "--max-bytes" => match need(i) {
                Ok(v) => match v.parse::<i64>() {
                    Ok(n) if n >= 0 => {
                        max_bytes = Some(n);
                        i += 2;
                    }
                    _ => return usage_err("client", "--max-bytes must be non-negative"),
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
            "--value-id" => match need(i) {
                Ok(v) => {
                    value_id = v.clone();
                    i += 2;
                }
                Err(c) => return c,
            },
            "--atom-id" => match need(i) {
                Ok(v) => {
                    atom_id = v.clone();
                    i += 2;
                }
                Err(c) => return c,
            },
            "--swmr-id" => match need(i) {
                Ok(v) => {
                    swmr_id = v.clone();
                    i += 2;
                }
                Err(c) => return c,
            },
            "--crdt-id" => match need(i) {
                Ok(v) => {
                    crdt_id = v.clone();
                    i += 2;
                }
                Err(c) => return c,
            },
            "--replica-script" => match need(i) {
                Ok(v) => {
                    replica_script = Some(v.clone());
                    i += 2;
                }
                Err(c) => return c,
            },
            "--epoch" => match need(i) {
                Ok(v) => match v.parse::<i64>() {
                    Ok(n) if n >= 0 => {
                        epoch = Some(n);
                        i += 2;
                    }
                    _ => return usage_err("client", "--epoch must be a non-negative i64"),
                },
                Err(c) => return c,
            },
            "--extra-stream-id" => match need(i) {
                Ok(v) => {
                    extra_stream_ids.push(v.clone());
                    i += 2;
                }
                Err(c) => return c,
            },
            "--timeout-ms" => match need(i) {
                Ok(v) => match v.parse::<i64>() {
                    Ok(n) if n >= 0 => {
                        timeout_ms = Some(n);
                        i += 2;
                    }
                    _ => return usage_err("client", "--timeout-ms must be a non-negative i64"),
                },
                Err(c) => return c,
            },
            "--reconnect-dropped" => {
                reconnect_dropped = true;
                i += 1;
            }
            "--pause-stream-id" => match need(i) {
                Ok(v) => {
                    pause_stream_ids.push(v.clone());
                    i += 2;
                }
                Err(c) => return c,
            },
            "--resume-after-data" => match need(i) {
                Ok(v) => match v.parse::<usize>() {
                    Ok(n) if n > 0 => {
                        resume_after_data = Some(n);
                        i += 2;
                    }
                    _ => return usage_err("client", "--resume-after-data must be positive"),
                },
                Err(c) => return c,
            },
            "--reads" => match need(i) {
                Ok(v) => match v.parse::<u32>() {
                    Ok(n) if n > 0 => {
                        reads = n;
                        i += 2;
                    }
                    _ => return usage_err("client", "--reads must be a positive u32"),
                },
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

    if let Err(code) = select_shape("client", &shape) {
        return code;
    }

    let stream_id = match stream_id {
        Some(s) => s,
        None => return usage_err("client", "--stream-id is required"),
    };
    let script = match load_script("client", script_path) {
        Ok(s) => s,
        Err(code) => return code,
    };
    if epoch.is_some() && from.is_none() {
        return usage_err("client", "--epoch requires --from");
    }

    match shape.as_str() {
        "crdt" | "text_crdt" => {
            if script.is_some() {
                return usage_err("client", "--script is not supported by the CRDT client");
            }
            let Some(replica_script) = replica_script else {
                return usage_err("client", "--replica-script is required for CRDT");
            };
            ExitCode::from(crdt_tool::run_client(crdt_tool::ClientOpts {
                crdt_id,
                stream_id,
                replica_script,
                text_profile: shape == "text_crdt",
            }))
        }
        "atom" => {
            if script.is_some() {
                return usage_err("client", "--script is not supported by the atom client");
            }
            let from = from.unwrap_or(0);
            let mut stream_ids = vec![stream_id];
            stream_ids.extend(extra_stream_ids);
            ExitCode::from(atom_tool::run_client(atom_tool::ClientOpts {
                atom_id,
                stream_ids,
                from,
                timeout_ms,
            }))
        }
        "log" => {
            let from = match from {
                Some(n) if n >= 0 => n as u64,
                Some(_) => return usage_err("client", "--from must be non-negative for log"),
                None => return usage_err("client", "--from is required for log"),
            };
            let script = match decode_script("client", script, jsoncodec::input_from_json) {
                Ok(script) => script,
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
        "stream" => {
            if script.is_some() {
                return usage_err("client", "--script is not supported by the stream client");
            }
            let mut stream_ids = vec![stream_id];
            stream_ids.extend(extra_stream_ids);
            ExitCode::from(stream_tool::run_client(stream_tool::ClientOpts {
                stream_ids,
                max_records: max_records.map(i64::from),
                max_bytes,
                timeout_ms,
                reconnect_dropped,
                pause_stream_ids: pause_stream_ids.into_iter().collect(),
                resume_after_data,
            }))
        }
        "swmr" | "snapshot_delta" => {
            if script.is_some() {
                return usage_err("client", "--script is not supported by the swmr client");
            }
            let initial_cursor = match from {
                Some(seq) if seq >= 0 => Some(taut_shape::generated_swmr::SwmrCursor {
                    seq,
                    epoch: epoch.unwrap_or(0),
                }),
                Some(_) => return usage_err("client", "--from must be non-negative for swmr"),
                None => None,
            };
            let mut stream_ids = vec![stream_id];
            stream_ids.extend(extra_stream_ids);
            ExitCode::from(swmr_tool::run_client(swmr_tool::ClientOpts {
                swmr_id,
                stream_ids,
                initial_cursor,
                timeout_ms,
                expire_profile: shape == "snapshot_delta",
            }))
        }
        "value" => {
            if script.is_some() {
                return usage_err("client", "--script is not supported by the value client");
            }
            ExitCode::from(value_tool::run_client(value_tool::ClientOpts {
                value_id,
                stream_id,
                reads,
            }))
        }
        _ => unreachable!("shape was validated against the exact registry"),
    }
}

fn select_shape(mode: &str, shape: &str) -> Result<(), ExitCode> {
    match require_engine_shape(shape) {
        Ok(_) => Ok(()),
        Err(diagnostic) => {
            eprintln!("taut-shape-tool {mode}: {diagnostic}");
            Err(ExitCode::from(2))
        }
    }
}

/// Load and parse an optional `--script` file, mapping any parse error to a
/// usage exit (2).
fn load_script(mode: &str, path: Option<String>) -> Result<Option<Script<json::Json>>, ExitCode> {
    match path {
        None => Ok(None),
        Some(p) => match Script::load(&p) {
            Ok(s) => Ok(Some(s)),
            Err(e) => Err(usage_err_code(mode, &format!("--script: {e}"))),
        },
    }
}

fn decode_script<T>(
    mode: &str,
    script: Option<Script<json::Json>>,
    decode: impl FnMut(&json::Json) -> Result<T, String>,
) -> Result<Option<Script<T>>, ExitCode> {
    script
        .map(|script| script.decode(decode))
        .transpose()
        .map_err(|error| usage_err_code(mode, &format!("--script: {error}")))
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
