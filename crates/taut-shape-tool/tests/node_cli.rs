//! Integration test for `taut-shape-tool node` (S4.1/S4.2).
//!
//! Spawns the *real* built binary and drives its stdin/stdout through OS pipes,
//! proving the on-the-wire framing round-trips end to end: a `push(hello)`
//! followed by a probe `read(s1, cursor 0, timeout 0)` yields exactly one
//! `read_response` frame whose bytes decode to the expected records + state.
//!
//! Frame on the wire (pinned): `u32-LE length | tag byte | CBOR body`, where
//! `length` = 1 (tag) + body bytes.

use std::io::{Read, Write};
use std::process::{Command, Stdio};

use taut_shape::cbor::{self, Cbor};
use taut_shape::generated::{
    LogCursor, LogMsgType, LogPush, LogReadRequest, LogReadResponse, LogState,
};

/// Encode one frame: `u32-LE (1 + body.len()) | tag byte | CBOR body`.
fn frame(tag: LogMsgType, body: &Cbor) -> Vec<u8> {
    let encoded = cbor::encode(body);
    let mut out = Vec::new();
    let len = (encoded.len() + 1) as u32;
    out.extend_from_slice(&len.to_le_bytes());
    out.push(tag.wire() as u8);
    out.extend_from_slice(&encoded);
    out
}

/// Read exactly one frame from a byte slice, returning `(tag, body, rest)`.
fn parse_frame(buf: &[u8]) -> (LogMsgType, Cbor, &[u8]) {
    assert!(buf.len() >= 4, "frame shorter than its length prefix");
    let len = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
    assert!(len >= 1, "frame length must cover at least the tag byte");
    let frame_end = 4 + len;
    assert!(buf.len() >= frame_end, "frame body truncated");
    let tag = LogMsgType::from_wire(buf[4] as i64).expect("known frame tag");
    let body = cbor::decode(&buf[5..frame_end]);
    (tag, body, &buf[frame_end..])
}

#[test]
fn push_then_read_round_trips_through_real_pipes() {
    // Build the stdin byte stream: push(hello), then read(s1, cursor 0, tmo 0).
    let mut stdin_bytes = Vec::new();
    stdin_bytes.extend_from_slice(&frame(
        LogMsgType::Push,
        &LogPush {
            payload: b"hello".to_vec(),
        }
        .to_cbor(),
    ));
    stdin_bytes.extend_from_slice(&frame(
        LogMsgType::Read,
        &LogReadRequest {
            log_id: "log-A".to_string(),
            stream_id: "s1".to_string(),
            cursor: Some(LogCursor { seq: 0 }),
            max_records: None,
            max_bytes: None,
            timeout_ms: Some(0), // probe
        }
        .to_cbor(),
    ));

    // Spawn the real bin via `cargo`'s built artifact path.
    let bin = env!("CARGO_BIN_EXE_taut-shape-tool");
    let mut child = Command::new(bin)
        .arg("node")
        .arg("--shape")
        .arg("log")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn taut-shape-tool node");

    // Feed stdin then close it (drives EOF ⇒ exit 0).
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&stdin_bytes)
        .expect("write stdin");

    let out = child.wait_with_output().expect("collect output");
    assert!(
        out.status.success(),
        "node exited {:?}; stderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    // The push yields no output; the probe read yields exactly one
    // read_response frame. So stdout is exactly one frame.
    let (tag, body, rest) = parse_frame(&out.stdout);
    assert_eq!(tag.wire(), LogMsgType::ReadResponse.wire());
    assert!(rest.is_empty(), "unexpected extra output frames: {rest:?}");

    let resp = LogReadResponse::from_cbor(&body).expect("valid read response");
    assert_eq!(resp.log_id, "log-A");
    assert_eq!(resp.stream_id, "s1");
    assert_eq!(resp.records.len(), 1, "expected the one pushed record");
    assert_eq!(resp.records[0].seq, 1, "first record is seq 1 (D8)");
    assert_eq!(resp.records[0].payload, b"hello");
    assert_eq!(resp.next_cursor.seq, 1, "next_cursor advances to head");
    assert_eq!(
        resp.state.wire(),
        LogState::Data.wire(),
        "records available ⇒ data state"
    );
    assert!(resp.error.is_none());
}

#[test]
fn held_release_echoes_the_streams_log_id() {
    // A held read (no timeout_ms) on an empty log parks; a later `push`
    // releases it. The releasing frame is a `Push`, not a `Read`, so the
    // read-response is emitted with no `Read` frame in scope. It must still echo
    // the same `log_id` the stream was created with ("log-A") — the framing
    // layer remembers it per stream (D3: the engine's `Response` carries only
    // `stream_id`). Regression guard against the empty-`log_id` echo.
    let mut stdin_bytes = Vec::new();
    stdin_bytes.extend_from_slice(&frame(
        LogMsgType::Read,
        &LogReadRequest {
            log_id: "log-A".to_string(),
            stream_id: "s1".to_string(),
            cursor: Some(LogCursor { seq: 0 }),
            max_records: None,
            max_bytes: None,
            timeout_ms: None, // hold indefinitely
        }
        .to_cbor(),
    ));
    stdin_bytes.extend_from_slice(&frame(
        LogMsgType::Push,
        &LogPush {
            payload: b"hello".to_vec(),
        }
        .to_cbor(),
    ));

    let bin = env!("CARGO_BIN_EXE_taut-shape-tool");
    let mut child = Command::new(bin)
        .arg("node")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn taut-shape-tool node");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&stdin_bytes)
        .expect("write stdin");
    let out = child.wait_with_output().expect("collect output");
    assert!(out.status.success(), "node exited {:?}", out.status.code());

    // The held read emits nothing; the push emits exactly one read_response.
    let (tag, body, rest) = parse_frame(&out.stdout);
    assert_eq!(tag.wire(), LogMsgType::ReadResponse.wire());
    assert!(rest.is_empty(), "unexpected extra frames: {rest:?}");
    let resp = LogReadResponse::from_cbor(&body).expect("valid read response");
    assert_eq!(
        resp.log_id, "log-A",
        "a held-release response must echo the stream's originating log_id"
    );
    assert_eq!(resp.stream_id, "s1");
    assert_eq!(resp.records.len(), 1);
    assert_eq!(resp.records[0].seq, 1);
    assert_eq!(resp.state.wire(), LogState::Data.wire());
}

#[test]
fn malformed_frame_exits_3_without_panicking() {
    // A frame with an out-of-range tag byte (99) — a fully-present but
    // malformed frame ⇒ exit 3, one stderr line, no panic.
    let body = cbor::encode(&Cbor::Map(vec![]));
    let mut bytes = Vec::new();
    let len = (body.len() + 1) as u32;
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.push(99); // unknown tag
    bytes.extend_from_slice(&body);

    let bin = env!("CARGO_BIN_EXE_taut-shape-tool");
    let mut child = Command::new(bin)
        .arg("node")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let out = child.wait_with_output().expect("collect");
    assert_eq!(out.status.code(), Some(3), "malformed frame ⇒ exit 3");
    assert!(
        !out.stderr.is_empty(),
        "a malformed frame should log one stderr line"
    );
}

#[test]
fn clean_eof_between_frames_exits_0() {
    // No input at all: immediate EOF ⇒ exit 0, no output.
    let bin = env!("CARGO_BIN_EXE_taut-shape-tool");
    let mut child = Command::new(bin)
        .arg("node")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");
    drop(child.stdin.take()); // immediate EOF
    let mut stdout = child.stdout.take().unwrap();
    let mut buf = Vec::new();
    stdout.read_to_end(&mut buf).unwrap();
    let status = child.wait().unwrap();
    assert!(status.success(), "clean EOF ⇒ exit 0");
    assert!(buf.is_empty(), "no input ⇒ no output frames");
}

#[test]
fn unsupported_shape_exits_before_node_or_client_startup() {
    let bin = env!("CARGO_BIN_EXE_taut-shape-tool");
    for mode in ["node", "client"] {
        let out = Command::new(bin)
            .arg(mode)
            .arg("--shape")
            .arg("window")
            .output()
            .expect("run taut-shape-tool");
        assert_eq!(out.status.code(), Some(2), "{mode} unsupported shape");
        assert!(
            out.stdout.is_empty(),
            "{mode} must not start the data channel"
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("TAUT_SHAPE_UNSUPPORTED_SHAPE") && stderr.contains("window"),
            "{mode} typed diagnostic: {stderr}"
        );
    }
}
