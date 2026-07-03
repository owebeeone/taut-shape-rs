//! Interop integration test: `node` ⇄ `client` over real crossed pipes.
//!
//! Spawns the *real* built binary twice — once as `node` (owning the engine and
//! a `--script` producer injection), once as `client` (the cursor loop) — and
//! wires their stdio into the crossed-pipe topology the Oracle §7 matrix uses:
//!
//! ```text
//!     client.stdout ──frames──▶ node.stdin      (LogReadRequest → node)
//!     node.stdout   ──frames──▶ client.stdin    (LogReadResponse → client)
//! ```
//!
//! Because `std::process::Command` can't fuse two children's pipes at the OS
//! level portably, a pair of relay threads copies the bytes across. The producer
//! is driven entirely by the node's `--script`: a `push` after the client's 1st
//! request releases the first held read (`data`), and a `seal` after its 2nd
//! request releases the second (`eof`), driving the client to a terminal exit.
//!
//! Assertions are on the client's OOB JSONL transcript (stderr): it must observe
//! the released record and terminate on `eof`, exit 0 — proving the data-channel
//! framing round-trips end to end between two independent processes.

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;

/// Copy every byte from `src` to `dst` until EOF (a pipe relay). Ignores a
/// broken-pipe on the far side (the peer may exit first).
fn relay<R: Read + Send + 'static, W: Write + Send + 'static>(
    mut src: R,
    mut dst: W,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match src.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if dst.write_all(&buf[..n]).is_err() {
                        break;
                    }
                    let _ = dst.flush();
                }
                Err(_) => break,
            }
        }
    })
}

#[test]
fn node_serves_client_over_crossed_pipes_with_script_producer() {
    // A producer script: push "hello" after the client's 1st request, then seal
    // after its 2nd. Written to a temp file the node reads via `--script`.
    let script = r#"[
      { "after_frames": 1, "inputs": [ { "type": "push", "payload": "aGVsbG8=" } ] },
      { "after_frames": 2, "inputs": [ { "type": "seal" } ] }
    ]"#;
    let dir = std::env::temp_dir();
    let script_path = dir.join(format!(
        "taut-shape-interop-{}-{}.json",
        std::process::id(),
        line!()
    ));
    std::fs::write(&script_path, script).expect("write script");

    let bin = env!("CARGO_BIN_EXE_taut-shape-tool");

    // Node: owns the engine + the producer script.
    let mut node = Command::new(bin)
        .arg("node")
        .arg("--stop-when")
        .arg("last_reader")
        .arg("--script")
        .arg(&script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn node");

    // Client: the cursor loop, reading stream s1 from seq 0.
    let mut client = Command::new(bin)
        .arg("client")
        .arg("--stream-id")
        .arg("s1")
        .arg("--from")
        .arg("0")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn client");

    // Cross the data pipes with relay threads.
    let node_in = node.stdin.take().unwrap();
    let node_out = node.stdout.take().unwrap();
    let client_in = client.stdin.take().unwrap();
    let client_out = client.stdout.take().unwrap();

    let r1 = relay(client_out, node_in); // client → node (requests)
    let r2 = relay(node_out, client_in); // node → client (responses)

    // Collect the client's OOB transcript from a thread so we don't deadlock.
    let mut client_err = client.stderr.take().unwrap();
    let err_handle = thread::spawn(move || {
        let mut s = String::new();
        let _ = client_err.read_to_string(&mut s);
        s
    });
    // Drain the node's stderr too, so its transcript writes can never fill the
    // pipe buffer and block the node.
    let mut node_err = node.stderr.take().unwrap();
    let node_err_handle = thread::spawn(move || {
        let mut s = String::new();
        let _ = node_err.read_to_string(&mut s);
        s
    });

    // The client terminates on `eof` (seal releases its 2nd held read); the node
    // then sees its stdin close and exits 0.
    let client_status = client.wait().expect("wait client");
    let transcript = err_handle.join().expect("join client stderr");

    // Node's stdin is fed by the relay from the client's (now-closed) stdout, so
    // it drains and exits once the relays finish.
    let _ = r1.join();
    let _ = r2.join();
    let node_status = node.wait().expect("wait node");
    let _ = node_err_handle.join();

    let _ = std::fs::remove_file(&script_path);

    assert!(
        client_status.success(),
        "client exited {:?}",
        client_status.code()
    );
    assert!(
        node_status.success(),
        "node exited {:?}",
        node_status.code()
    );

    // The transcript is one JSONL object per received response, then a final
    // `client_final` line. It must contain the released "hello" record (base64
    // "aGVsbG8=") and terminate on `eof`.
    let lines: Vec<&str> = transcript.lines().collect();
    assert!(
        lines.iter().any(|l| l.contains("aGVsbG8=") && l.contains("\"state\":\"data\"")),
        "expected a data response carrying the pushed record; transcript:\n{transcript}"
    );
    let last = lines.last().copied().unwrap_or("");
    assert!(
        last.contains("\"type\":\"client_final\"") && last.contains("\"state\":\"eof\""),
        "expected a final eof line; transcript:\n{transcript}"
    );
}

#[test]
fn client_side_script_drives_a_plain_node_to_closed() {
    // The other `--script` direction: a *plain* node (no script), and the CLIENT
    // injects the producer. After its 1st request the client pushes "hello"
    // (releasing read #1 with `data`); after its 2nd it `close`s the log
    // (releasing read #2 with `closed` and emitting ProducerStop). The client
    // terminates on `closed`.
    let script = r#"[
      { "after_frames": 1, "inputs": [ { "type": "push", "payload": "aGVsbG8=" } ] },
      { "after_frames": 2, "inputs": [ { "type": "close" } ] }
    ]"#;
    let dir = std::env::temp_dir();
    let script_path = dir.join(format!(
        "taut-shape-interop-{}-{}.json",
        std::process::id(),
        line!()
    ));
    std::fs::write(&script_path, script).expect("write script");

    let bin = env!("CARGO_BIN_EXE_taut-shape-tool");

    let mut node = Command::new(bin)
        .arg("node")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn node");

    let mut client = Command::new(bin)
        .arg("client")
        .arg("--stream-id")
        .arg("s1")
        .arg("--from")
        .arg("0")
        .arg("--script")
        .arg(&script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn client");

    let node_in = node.stdin.take().unwrap();
    let node_out = node.stdout.take().unwrap();
    let client_in = client.stdin.take().unwrap();
    let client_out = client.stdout.take().unwrap();

    let r1 = relay(client_out, node_in);
    let r2 = relay(node_out, client_in);

    let mut client_err = client.stderr.take().unwrap();
    let err_handle = thread::spawn(move || {
        let mut s = String::new();
        let _ = client_err.read_to_string(&mut s);
        s
    });
    let mut node_err = node.stderr.take().unwrap();
    let node_err_handle = thread::spawn(move || {
        let mut s = String::new();
        let _ = node_err.read_to_string(&mut s);
        s
    });

    let client_status = client.wait().expect("wait client");
    let transcript = err_handle.join().expect("join client stderr");
    let _ = r1.join();
    let _ = r2.join();
    let node_status = node.wait().expect("wait node");
    let _ = node_err_handle.join();

    let _ = std::fs::remove_file(&script_path);

    assert!(
        client_status.success(),
        "client exited {:?}; transcript:\n{transcript}",
        client_status.code()
    );
    assert!(node_status.success(), "node exited {:?}", node_status.code());

    let lines: Vec<&str> = transcript.lines().collect();
    assert!(
        lines
            .iter()
            .any(|l| l.contains("aGVsbG8=") && l.contains("\"state\":\"data\"")),
        "expected the pushed record; transcript:\n{transcript}"
    );
    // Terminal is `closed` (from the read answer) — the client exits on that
    // before it would read the trailing ProducerStop frame.
    let last = lines.last().copied().unwrap_or("");
    assert!(
        last.contains("\"type\":\"client_final\"") && last.contains("\"state\":\"closed\""),
        "expected a final closed line; transcript:\n{transcript}"
    );
}
