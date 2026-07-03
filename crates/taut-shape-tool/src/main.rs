//! `taut-shape-tool` — the conformance/interop CLI (Phase 4).
//!
//! Modes (InitialPlan §4, S4.1/S4.2):
//!   * `node`   — run a [`taut_shape::LogNode`] behind the stdin/stdout framing
//!     (implemented here). One frame = `u32-LE len | tag byte | CBOR body`;
//!     each input is `handle`d and its outputs are written back as frames.
//!   * `gen` / `check` / `client` — stubs (each prints what it will do, exit 2).
//!
//! Exit codes: 0 = clean EOF on stdin; 2 = usage / unimplemented stub; 3 = a
//! malformed or unknown input frame; 1 = an underlying I/O fault.

use std::process::ExitCode;

use taut_shape::StopWhen;

mod framing;
mod node;

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

FRAMING (node data channel, stdin→stdout):
    one frame = u32-LE byte length, then 1 tag byte (LogMsgType wire value),
    then the message's CBOR body. `length` covers the tag byte + body.
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
        "gen" => stub("gen", "emit oracle vectors from scripted scenarios (reference impl only)"),
        "check" => stub(
            "check",
            "replay the committed oracle corpus through the engine and report pass/fail",
        ),
        "client" => stub("client", "run the reading side (the cursor loop) against a node"),
        other => {
            eprintln!("taut-shape-tool: unknown mode {other:?}\n");
            eprint!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

/// Parse `node`'s args and run the pump. Only `--stop-when` is recognised.
fn run_node(args: &[String]) -> ExitCode {
    let mut stop_when = StopWhen::LastReader; // task default: last_reader
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--stop-when" => {
                let val = match args.get(i + 1) {
                    Some(v) => v.as_str(),
                    None => {
                        eprintln!("taut-shape-tool node: --stop-when needs a value");
                        return ExitCode::from(2);
                    }
                };
                stop_when = match val {
                    "last_reader" => StopWhen::LastReader,
                    "explicit" => StopWhen::ExplicitOnly,
                    other => {
                        eprintln!(
                            "taut-shape-tool node: --stop-when expects last_reader|explicit, got {other:?}"
                        );
                        return ExitCode::from(2);
                    }
                };
                i += 2;
            }
            other => {
                eprintln!("taut-shape-tool node: unknown option {other:?}");
                return ExitCode::from(2);
            }
        }
    }
    ExitCode::from(node::run(stop_when))
}

/// A not-yet-implemented mode: name what it will do, exit 2.
fn stub(name: &str, what: &str) -> ExitCode {
    eprintln!("taut-shape-tool: `{name}` is not implemented yet — it will {what}.");
    ExitCode::from(2)
}
