//! Phase 1 unit tests — one focused test per pinned rule (shared §3.2–§3.4 +
//! D-numbers). The engine is `no_std`+`alloc`; the tests may use `std`.
//!
//! Naming: `<d_number>_<what>` so a failure names the violated decision.

use alloc::vec;
use alloc::vec::Vec;

use super::{
    Config, Cursor, Error, ErrorCode, Input, Limits, LogNode, Output, Record, Response, State,
    StopReason, StopWhen, StreamId, TimerToken,
};
use crate::generated::{LogDiagCode, LogDiagnostic, LogSeverity};

// ── helpers ─────────────────────────────────────────────────────────────────

fn node() -> LogNode {
    LogNode::new(Config::default())
}

fn node_last_reader() -> LogNode {
    LogNode::new(Config {
        stop_when: StopWhen::LastReader,
    })
}

fn sid(s: &str) -> StreamId {
    StreamId::new(s)
}

fn push(n: &mut LogNode, payload: &[u8]) -> Vec<Output> {
    n.handle(Input::Push {
        payload: payload.to_vec(),
    })
}

/// A read with explicit cursor + no timeout (hold indefinitely).
fn read_hold(n: &mut LogNode, s: &str, cursor: Option<Cursor>) -> Vec<Output> {
    n.handle(Input::Read {
        stream_id: sid(s),
        cursor,
        limits: Limits::default(),
        timeout_ms: None,
    })
}

/// A probe read (`timeout_ms = 0`).
fn read_probe(n: &mut LogNode, s: &str, cursor: Option<Cursor>) -> Vec<Output> {
    n.handle(Input::Read {
        stream_id: sid(s),
        cursor,
        limits: Limits::default(),
        timeout_ms: Some(0),
    })
}

/// Extract the single `Response` from an output vec (asserts exactly one).
fn only_response(out: &[Output]) -> &Response {
    let responses: Vec<&Response> = out
        .iter()
        .filter_map(|o| match o {
            Output::Response(r) => Some(r),
            _ => None,
        })
        .collect();
    assert_eq!(responses.len(), 1, "expected exactly one Response: {out:?}");
    responses[0]
}

fn rec(seq: u64, payload: &[u8]) -> Record {
    Record {
        seq,
        payload: payload.to_vec(),
    }
}

/// The canonical D19 output for a late push: `LogDiagnostic{Warn,
/// PushAfterTerminal}`, wrapped in `Output::Diagnostic`.
fn diag_push_after_terminal() -> Output {
    Output::Diagnostic(LogDiagnostic {
        severity: LogSeverity::Warn,
        code: LogDiagCode::PushAfterTerminal,
    })
}

/// Count the diagnostics in an output vec.
fn count_diagnostics(out: &[Output]) -> usize {
    out.iter()
        .filter(|o| matches!(o, Output::Diagnostic(_)))
        .count()
}

// ── D8: seq origin / START / empty head / absent cursor ─────────────────────

#[test]
fn d8_first_record_seq_is_one() {
    let mut n = node();
    assert_eq!(n.head(), 0, "empty log head must be 0");
    push(&mut n, b"a");
    assert_eq!(n.head(), 1, "first record must be seq=1");
    push(&mut n, b"b");
    assert_eq!(n.head(), 2);
}

#[test]
fn d8_start_cursor_is_seq_zero() {
    assert_eq!(Cursor::START, Cursor { seq: 0 });
}

#[test]
fn d8_absent_cursor_is_start() {
    let mut n = node();
    push(&mut n, b"a");
    push(&mut n, b"b");
    // Read with cursor: None must read from START (seq 0) → both records.
    let out = read_hold(&mut n, "s", None);
    let r = only_response(&out);
    assert_eq!(r.state, State::Data);
    assert_eq!(r.records, vec![rec(1, b"a"), rec(2, b"b")]);
    assert_eq!(r.next_cursor, Cursor::new(2));
}

// ── §3.4 rule 1: data ───────────────────────────────────────────────────────

#[test]
fn rule1_data_next_cursor_is_last_returned() {
    let mut n = node();
    push(&mut n, b"a");
    push(&mut n, b"b");
    push(&mut n, b"c");
    let out = read_hold(&mut n, "s", Some(Cursor::START));
    let r = only_response(&out);
    assert_eq!(r.state, State::Data);
    assert_eq!(r.records.len(), 3);
    assert_eq!(r.next_cursor, Cursor::new(3));
}

#[test]
fn rule1_resume_no_dup_no_skip() {
    let mut n = node();
    for b in [b"a", b"b", b"c", b"d"] {
        push(&mut n, b);
    }
    // First read half.
    let out = n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits {
            max_records: Some(2),
            max_bytes: None,
        },
        timeout_ms: Some(0),
    });
    let r = only_response(&out);
    assert_eq!(r.records, vec![rec(1, b"a"), rec(2, b"b")]);
    let resume = r.next_cursor;
    assert_eq!(resume, Cursor::new(2));
    // Resume from next_cursor → no dup, no skip.
    let out = read_hold(&mut n, "s", Some(resume));
    let r = only_response(&out);
    assert_eq!(r.records, vec![rec(3, b"c"), rec(4, b"d")]);
}

// ── §3.4 rule 2: caught-up terminal branches (D12) ──────────────────────────

#[test]
fn rule2_caught_up_sealed_is_eof() {
    let mut n = node();
    push(&mut n, b"a");
    n.handle(Input::Seal);
    // Read at head (seq 1) → eof.
    let out = read_probe(&mut n, "s", Some(Cursor::new(1)));
    let r = only_response(&out);
    assert_eq!(r.state, State::Eof);
    assert_eq!(r.next_cursor, Cursor::new(1));
    assert!(r.records.is_empty());
}

#[test]
fn rule2_caught_up_closed_is_closed() {
    let mut n = node();
    push(&mut n, b"a");
    n.handle(Input::Close { error: None });
    let out = read_probe(&mut n, "s", Some(Cursor::new(1)));
    let r = only_response(&out);
    assert_eq!(r.state, State::Closed);
    assert!(r.error.is_none());
}

#[test]
fn rule2_caught_up_failed_carries_error() {
    let mut n = node();
    push(&mut n, b"a");
    let err = Error {
        code: ErrorCode::ProducerError,
        message: Some("boom".into()),
    };
    n.handle(Input::Close {
        error: Some(err.clone()),
    });
    let out = read_probe(&mut n, "s", Some(Cursor::new(1)));
    let r = only_response(&out);
    assert_eq!(r.state, State::Failed);
    assert_eq!(r.error, Some(err));
}

// ── §3.4 rule 4: terminal states remain re-readable ─────────────────────────

#[test]
fn rule4_terminal_still_readable_below_head() {
    let mut n = node();
    push(&mut n, b"a");
    push(&mut n, b"b");
    n.handle(Input::Seal);
    // A read from START still returns data even though the log is sealed.
    let out = read_probe(&mut n, "s", Some(Cursor::START));
    let r = only_response(&out);
    assert_eq!(r.state, State::Data);
    assert_eq!(r.records, vec![rec(1, b"a"), rec(2, b"b")]);
    // And a follow-up at head is eof.
    let out = read_probe(&mut n, "s", Some(Cursor::new(2)));
    assert_eq!(only_response(&out).state, State::Eof);
}

// ── §3.4 rule 2 + D14: timeout_ms semantics ─────────────────────────────────

#[test]
fn d14_timeout_absent_holds_until_push() {
    let mut n = node();
    // Read tail with no data, no timeout → held (no output at all).
    let out = read_hold(&mut n, "s", Some(Cursor::START));
    assert!(out.is_empty(), "held read must produce no immediate output");
    assert_eq!(n.stream_count(), 1);
    // A later Push releases it with data.
    let out = push(&mut n, b"a");
    let r = only_response(&out);
    assert_eq!(r.state, State::Data);
    assert_eq!(r.records, vec![rec(1, b"a")]);
    assert_eq!(r.next_cursor, Cursor::new(1));
}

#[test]
fn d14_timeout_zero_is_immediate_would_block() {
    let mut n = node();
    let out = read_probe(&mut n, "s", Some(Cursor::START));
    let r = only_response(&out);
    assert_eq!(r.state, State::WouldBlock);
    assert_eq!(r.next_cursor, Cursor::START);
    // No timer for a probe.
    assert!(!out.iter().any(|o| matches!(o, Output::SetTimer { .. })));
}

#[test]
fn d14_timeout_positive_holds_and_sets_timer() {
    let mut n = node();
    let out = n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(500),
    });
    // A held read with a SetTimer, no Response yet.
    assert_eq!(out.len(), 1);
    match &out[0] {
        Output::SetTimer { token, ms } => {
            assert_eq!(*token, TimerToken(1));
            assert_eq!(*ms, 500);
        }
        other => panic!("expected SetTimer, got {other:?}"),
    }
    // Timer expiry answers would_block.
    let out = n.handle(Input::TimerExpired {
        token: TimerToken(1),
    });
    let r = only_response(&out);
    assert_eq!(r.state, State::WouldBlock);
    assert_eq!(r.next_cursor, Cursor::START);
}

// ── §3.4 rule 3 + D9: invalid cursor → expired ──────────────────────────────

#[test]
fn d9_cursor_beyond_head_is_expired_next_head() {
    let mut n = node();
    push(&mut n, b"a"); // head = 1
    let out = read_probe(&mut n, "s", Some(Cursor::new(5)));
    let r = only_response(&out);
    assert_eq!(r.state, State::Expired);
    assert_eq!(r.next_cursor, Cursor::new(1), "beyond head → next = head");
}

#[test]
fn d9_cursor_below_floor_is_expired_next_floor_minus_one() {
    let mut n = node();
    for b in [b"a", b"b", b"c", b"d"] {
        push(&mut n, b);
    }
    // Evict through seq 2 → floor becomes 3; floor-1 = 2 is earliest resumable.
    n.handle(Input::Evict { up_to_seq: 2 });
    assert_eq!(n.floor(), 3);
    // A cursor at seq 0 (START) has 0 + 1 < 3 → expired, next = floor-1 = 2.
    let out = read_probe(&mut n, "s", Some(Cursor::START));
    let r = only_response(&out);
    assert_eq!(r.state, State::Expired);
    assert_eq!(r.next_cursor, Cursor::new(2));
    // A cursor at exactly floor-1 (seq 2) is VALID: 2 + 1 == floor, reads 3,4.
    let out = read_probe(&mut n, "s", Some(Cursor::new(2)));
    let r = only_response(&out);
    assert_eq!(r.state, State::Data);
    assert_eq!(r.records, vec![rec(3, b"c"), rec(4, b"d")]);
}

// ── D5: supersede ───────────────────────────────────────────────────────────

#[test]
fn d5_new_read_supersedes_held_with_cancel_timer() {
    let mut n = node();
    // First held read with a timer.
    let out = n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    assert!(matches!(out.as_slice(), [Output::SetTimer { token, .. }] if *token == TimerToken(1)));
    // A new Read on the same stream supersedes: old dropped unanswered, its
    // timer canceled. This one probes → immediate would_block.
    let out = read_probe(&mut n, "s", Some(Cursor::START));
    // Must contain a CancelTimer(1) for the superseded read and one Response.
    assert!(
        out.iter()
            .any(|o| matches!(o, Output::CancelTimer { token } if *token == TimerToken(1))),
        "supersede must cancel the old timer: {out:?}"
    );
    let r = only_response(&out);
    assert_eq!(r.state, State::WouldBlock);
    // Only one stream instance exists.
    assert_eq!(n.stream_count(), 1);
}

#[test]
fn d5_supersede_with_new_timer_cancels_old_sets_new() {
    let mut n = node();
    // First held read → timer token 1.
    n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    // Supersede with another held read that also wants a timer → token 2.
    let out = n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(200),
    });
    // Expect exactly: CancelTimer(1) then SetTimer(2), no Response.
    assert_eq!(out.len(), 2, "{out:?}");
    assert!(matches!(out[0], Output::CancelTimer { token } if token == TimerToken(1)));
    assert!(
        matches!(out[1], Output::SetTimer { token, ms } if token == TimerToken(2) && ms == 200)
    );
    assert!(!out.iter().any(|o| matches!(o, Output::Response(_))));
}

#[test]
fn d5_only_one_outstanding_read_answered_on_push() {
    let mut n = node();
    // Two held reads on the SAME stream: the second supersedes the first.
    read_hold(&mut n, "s", Some(Cursor::START));
    read_hold(&mut n, "s", Some(Cursor::START));
    // A push must answer exactly one read (the surviving held one).
    let out = push(&mut n, b"a");
    let responses: Vec<_> = out
        .iter()
        .filter(|o| matches!(o, Output::Response(_)))
        .collect();
    assert_eq!(responses.len(), 1);
}

// ── D4: EndStream ───────────────────────────────────────────────────────────

#[test]
fn d4_end_stream_drops_held_no_response_cancels_timer() {
    let mut n = node();
    let out = n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    assert!(matches!(out.as_slice(), [Output::SetTimer { .. }]));
    // EndStream: no Response, a CancelTimer, reader count → 0.
    let out = n.handle(Input::EndStream {
        stream_id: sid("s"),
    });
    assert!(
        !out.iter().any(|o| matches!(o, Output::Response(_))),
        "EndStream must not answer the held read"
    );
    assert!(out
        .iter()
        .any(|o| matches!(o, Output::CancelTimer { token } if *token == TimerToken(1))));
    assert_eq!(n.stream_count(), 0, "reader count must decrement");
}

#[test]
fn d4_end_stream_removes_watermark() {
    let mut n = node();
    push(&mut n, b"a");
    read_probe(&mut n, "s", Some(Cursor::START)); // watermark → 1
    assert_eq!(n.min_watermark(), Some(1));
    n.handle(Input::EndStream {
        stream_id: sid("s"),
    });
    assert_eq!(n.min_watermark(), None, "watermark removed with the stream");
}

#[test]
fn d4_end_stream_unknown_is_noop() {
    let mut n = node();
    let out = n.handle(Input::EndStream {
        stream_id: sid("ghost"),
    });
    assert!(out.is_empty());
    assert_eq!(n.stream_count(), 0);
}

// ── D6: ProducerStop ────────────────────────────────────────────────────────

#[test]
fn d6_close_transition_emits_producer_stop() {
    // D6 (refined): the FIRST Close (Live→Closed transition) emits ProducerStop.
    let mut n = node(); // ExplicitOnly
    let out = n.handle(Input::Close { error: None });
    assert!(out
        .iter()
        .any(|o| matches!(o, Output::ProducerStop { reason } if *reason == StopReason::Closed)));
}

#[test]
fn d6_close_error_emits_producer_stop_failed() {
    let mut n = node();
    let out = n.handle(Input::Close {
        error: Some(Error {
            code: ErrorCode::Internal,
            message: None,
        }),
    });
    assert!(out
        .iter()
        .any(|o| matches!(o, Output::ProducerStop { reason } if *reason == StopReason::Failed)));
}

#[test]
fn d6_last_reader_transition_emits_producer_stop() {
    let mut n = node_last_reader();
    read_hold(&mut n, "s", Some(Cursor::START)); // reader count 0 → 1
    assert_eq!(n.stream_count(), 1);
    let out = n.handle(Input::EndStream {
        stream_id: sid("s"),
    }); // 1 → 0
    assert!(
        out.iter().any(
            |o| matches!(o, Output::ProducerStop { reason } if *reason == StopReason::LastReaderGone)
        ),
        "last reader gone under LastReader must stop the producer: {out:?}"
    );
}

#[test]
fn d6_explicit_only_no_producer_stop_on_last_reader() {
    let mut n = node(); // ExplicitOnly
    read_hold(&mut n, "s", Some(Cursor::START));
    let out = n.handle(Input::EndStream {
        stream_id: sid("s"),
    });
    assert!(
        !out.iter().any(|o| matches!(o, Output::ProducerStop { .. })),
        "ExplicitOnly must not stop the producer on reader loss"
    );
}

#[test]
fn d6_never_read_log_does_not_stop_producer() {
    let mut n = node_last_reader();
    // No reads ever. EndStream on a ghost stream must not transition 1→0.
    let out = n.handle(Input::EndStream {
        stream_id: sid("ghost"),
    });
    assert!(!out.iter().any(|o| matches!(o, Output::ProducerStop { .. })));
}

// ── D10: limits ─────────────────────────────────────────────────────────────

#[test]
fn d10_max_records_bounds_batch() {
    let mut n = node();
    for b in [b"a", b"b", b"c"] {
        push(&mut n, b);
    }
    let out = n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits {
            max_records: Some(2),
            max_bytes: None,
        },
        timeout_ms: Some(0),
    });
    let r = only_response(&out);
    assert_eq!(r.records.len(), 2);
    assert_eq!(r.next_cursor, Cursor::new(2));
}

#[test]
fn d10_max_bytes_counts_raw_payload_and_stops() {
    let mut n = node();
    push(&mut n, b"aaa"); // 3 bytes
    push(&mut n, b"bbb"); // 3 bytes
    push(&mut n, b"ccc"); // 3 bytes
    let out = n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits {
            max_records: None,
            max_bytes: Some(4), // fits record 1 (3), record 2 would push to 6 > 4
        },
        timeout_ms: Some(0),
    });
    let r = only_response(&out);
    assert_eq!(r.records, vec![rec(1, b"aaa")]);
    assert_eq!(r.next_cursor, Cursor::new(1));
}

#[test]
fn d10_forward_progress_returns_one_even_if_over_max_bytes() {
    let mut n = node();
    push(&mut n, b"huge_payload"); // 12 bytes
    let out = n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits {
            max_records: None,
            max_bytes: Some(1), // smaller than the record
        },
        timeout_ms: Some(0),
    });
    let r = only_response(&out);
    assert_eq!(r.state, State::Data);
    assert_eq!(r.records.len(), 1, "forward progress: ≥1 record");
    assert_eq!(r.records[0].payload, b"huge_payload".to_vec());
}

// ── D12: terminal split + idempotency ───────────────────────────────────────

#[test]
fn d12_seal_is_idempotent() {
    let mut n = node();
    push(&mut n, b"a");
    n.handle(Input::Seal);
    n.handle(Input::Seal); // idempotent, no panic
    let out = read_probe(&mut n, "s", Some(Cursor::new(1)));
    assert_eq!(only_response(&out).state, State::Eof);
}

#[test]
fn d12_close_is_idempotent_first_terminal_wins() {
    let mut n = node();
    push(&mut n, b"a");
    n.handle(Input::Close { error: None }); // closed
    let second = n.handle(Input::Close {
        error: Some(Error {
            code: ErrorCode::Internal,
            message: None,
        }),
    }); // second close is idempotent — stays closed
        // D6 (refined): a Close on an already-terminal log is outputs-idempotent
        // — it emits NOTHING (no second ProducerStop, no re-answered reads).
    assert!(
        second.is_empty(),
        "repeated Close on a terminal log must emit nothing: {second:?}"
    );
    let out = read_probe(&mut n, "s", Some(Cursor::new(1)));
    assert_eq!(only_response(&out).state, State::Closed);
}

#[test]
fn d6_repeated_close_on_terminal_emits_nothing() {
    // The refined D6 rule in isolation: the FIRST Close transitions and emits
    // ProducerStop; every subsequent Close (clean or error) emits nothing.
    let mut n = node();
    let first = n.handle(Input::Close { error: None });
    assert!(
        first
            .iter()
            .any(|o| matches!(o, Output::ProducerStop { reason } if *reason == StopReason::Closed)),
        "first Close must emit ProducerStop: {first:?}"
    );
    let second = n.handle(Input::Close { error: None });
    assert!(second.is_empty(), "second Close emits nothing: {second:?}");
    let third = n.handle(Input::Close {
        error: Some(Error {
            code: ErrorCode::Internal,
            message: None,
        }),
    });
    assert!(
        third.is_empty(),
        "Close-after-terminal (even with error) emits nothing: {third:?}"
    );
}

#[test]
fn d6_close_after_seal_is_a_transition_and_stops_producer() {
    // Seal is terminal for reads (eof) but NOT for the producer: a following
    // Close is a real Sealed→Closed transition, so it fires ProducerStop (the
    // Seal itself never does) and the log now reads `closed` at head.
    let mut n = node();
    push(&mut n, b"a");
    let sealed = n.handle(Input::Seal);
    assert!(
        !sealed
            .iter()
            .any(|o| matches!(o, Output::ProducerStop { .. })),
        "Seal must not stop the producer: {sealed:?}"
    );
    let out = n.handle(Input::Close { error: None });
    // The Sealed→Closed transition fires exactly one ProducerStop{Closed}.
    let stops: Vec<_> = out
        .iter()
        .filter_map(|o| match o {
            Output::ProducerStop { reason } => Some(*reason),
            _ => None,
        })
        .collect();
    assert_eq!(
        stops,
        vec![StopReason::Closed],
        "close-after-seal is a transition and stops the producer once: {out:?}"
    );
    // The log now reads `closed` at head (Close won over Sealed — terminal split).
    let probe = read_probe(&mut n, "s", Some(Cursor::new(1)));
    assert_eq!(only_response(&probe).state, State::Closed);
    // A second Close (now Closed) is the no-op case.
    let again = n.handle(Input::Close { error: None });
    assert!(
        again.is_empty(),
        "close after Closed emits nothing: {again:?}"
    );
}

// ── D16: determinism ────────────────────────────────────────────────────────

#[test]
fn d16_timer_tokens_monotonic_from_one() {
    let mut n = node();
    let mut tokens = Vec::new();
    for s in ["a", "b", "c"] {
        let out = n.handle(Input::Read {
            stream_id: sid(s),
            cursor: Some(Cursor::START),
            limits: Limits::default(),
            timeout_ms: Some(100),
        });
        if let Some(Output::SetTimer { token, .. }) =
            out.iter().find(|o| matches!(o, Output::SetTimer { .. }))
        {
            tokens.push(*token);
        }
    }
    assert_eq!(
        tokens,
        vec![TimerToken(1), TimerToken(2), TimerToken(3)],
        "tokens allocated 1,2,3…"
    );
}

#[test]
fn d16_multi_held_release_in_creation_order() {
    let mut n = node();
    // Two streams created in order s1, then s2, both held on the tail.
    read_hold(&mut n, "s1", Some(Cursor::START));
    read_hold(&mut n, "s2", Some(Cursor::START));
    // One Push wakes both; responses must be s1 then s2 (creation order).
    let out = push(&mut n, b"a");
    let ids: Vec<&StreamId> = out
        .iter()
        .filter_map(|o| match o {
            Output::Response(r) => Some(&r.stream_id),
            _ => None,
        })
        .collect();
    assert_eq!(ids.len(), 2);
    assert_eq!(ids[0], &sid("s1"));
    assert_eq!(ids[1], &sid("s2"));
    // Both got the record.
    for o in &out {
        if let Output::Response(r) = o {
            assert_eq!(r.records, vec![rec(1, b"a")]);
        }
    }
}

#[test]
fn multi_held_release_order_independent_of_wake_id() {
    // Streams created s2 first then s1 (lexically reversed) — creation order,
    // not id order, governs emission (D16).
    let mut n = node();
    read_hold(&mut n, "zzz", Some(Cursor::START)); // created rank 0
    read_hold(&mut n, "aaa", Some(Cursor::START)); // created rank 1
    let out = push(&mut n, b"x");
    let ids: Vec<&StreamId> = out
        .iter()
        .filter_map(|o| match o {
            Output::Response(r) => Some(&r.stream_id),
            _ => None,
        })
        .collect();
    assert_eq!(ids, vec![&sid("zzz"), &sid("aaa")]);
}

// ── D7: eviction / watermarks ───────────────────────────────────────────────

#[test]
fn d7_evict_raises_floor() {
    let mut n = node();
    for b in [b"a", b"b", b"c"] {
        push(&mut n, b);
    }
    assert_eq!(n.floor(), 0, "nothing evicted → floor 0");
    n.handle(Input::Evict { up_to_seq: 1 });
    assert_eq!(n.floor(), 2, "evicted seq 1 → floor = lowest retained = 2");
}

#[test]
fn d7_min_watermark_tracks_slowest_reader() {
    let mut n = node();
    for b in [b"a", b"b", b"c"] {
        push(&mut n, b);
    }
    // s1 reads all (watermark 3); s2 reads one (watermark 1).
    read_probe(&mut n, "s1", Some(Cursor::START)); // watermark 3
    n.handle(Input::Read {
        stream_id: sid("s2"),
        cursor: Some(Cursor::START),
        limits: Limits {
            max_records: Some(1),
            max_bytes: None,
        },
        timeout_ms: Some(0),
    }); // watermark 1
    assert_eq!(n.min_watermark(), Some(1), "min across readers");
}

// ── TimerExpired for unknown/canceled token = no-op ─────────────────────────

#[test]
fn timer_expired_unknown_token_is_noop() {
    let mut n = node();
    // No timers set at all.
    let out = n.handle(Input::TimerExpired {
        token: TimerToken(99),
    });
    assert!(out.is_empty());
}

#[test]
fn timer_expired_after_supersede_is_noop() {
    let mut n = node();
    // Held read with timer token 1.
    n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    // Supersede it (cancels token 1, allocates nothing new for this probe).
    read_probe(&mut n, "s", Some(Cursor::START));
    // The stale token 1 expiry must be a no-op now.
    let out = n.handle(Input::TimerExpired {
        token: TimerToken(1),
    });
    assert!(out.is_empty(), "canceled timer token must be a no-op");
}

// ── EndStream mid-hold with a survivor ──────────────────────────────────────

#[test]
fn end_stream_mid_hold_leaves_other_streams_tailing() {
    let mut n = node();
    read_hold(&mut n, "s1", Some(Cursor::START));
    read_hold(&mut n, "s2", Some(Cursor::START));
    // End s1 mid-hold.
    n.handle(Input::EndStream {
        stream_id: sid("s1"),
    });
    assert_eq!(n.stream_count(), 1);
    // A Push answers only the survivor s2.
    let out = push(&mut n, b"a");
    let r = only_response(&out);
    assert_eq!(r.stream_id, sid("s2"));
    assert_eq!(r.records, vec![rec(1, b"a")]);
}

// ── close releases held reads (lifecycle sweep) ─────────────────────────────

#[test]
fn close_releases_held_reads_with_closed() {
    let mut n = node();
    read_hold(&mut n, "s", Some(Cursor::START)); // held at tail
    let out = n.handle(Input::Close { error: None });
    // The held read is answered `closed`, and ProducerStop fires.
    let r = only_response(&out);
    assert_eq!(r.state, State::Closed);
    assert!(out.iter().any(|o| matches!(o, Output::ProducerStop { .. })));
}

#[test]
fn seal_releases_held_reads_with_eof() {
    let mut n = node();
    read_hold(&mut n, "s", Some(Cursor::START));
    let out = n.handle(Input::Seal);
    let r = only_response(&out);
    assert_eq!(r.state, State::Eof);
}

#[test]
fn close_with_error_releases_held_read_failed_and_cancels_timer() {
    let mut n = node();
    // Held read with a timer.
    n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    let err = Error {
        code: ErrorCode::ProducerError,
        message: Some("gone".into()),
    };
    let out = n.handle(Input::Close {
        error: Some(err.clone()),
    });
    // Held read answered failed (+error), its timer canceled, ProducerStop.
    let r = only_response(&out);
    assert_eq!(r.state, State::Failed);
    assert_eq!(r.error, Some(err));
    assert!(out
        .iter()
        .any(|o| matches!(o, Output::CancelTimer { token } if *token == TimerToken(1))));
    assert!(out
        .iter()
        .any(|o| matches!(o, Output::ProducerStop { reason } if *reason == StopReason::Failed)));
}

// ── timer + push interaction: a timed held read woken by data cancels timer ──

#[test]
fn push_wakes_timed_held_read_and_cancels_timer() {
    let mut n = node();
    // Held read at the tail WITH a timer (timeout_ms > 0).
    let out = n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    assert!(matches!(out.as_slice(), [Output::SetTimer { token, .. }] if *token == TimerToken(1)));
    // A Push both answers the read with data AND cancels the now-moot timer.
    let out = push(&mut n, b"a");
    let r = only_response(&out);
    assert_eq!(r.state, State::Data);
    assert_eq!(r.records, vec![rec(1, b"a")]);
    assert!(
        out.iter()
            .any(|o| matches!(o, Output::CancelTimer { token } if *token == TimerToken(1))),
        "answering a timed held read must cancel its timer: {out:?}"
    );
    // The (now-stale) timer expiring afterward is a no-op.
    let out = n.handle(Input::TimerExpired {
        token: TimerToken(1),
    });
    assert!(out.is_empty());
}

// ── eviction edge: evicting beyond head evicts everything ────────────────────

#[test]
fn evict_beyond_head_clamps_floor_to_head_plus_one() {
    let mut n = node();
    for b in [b"a", b"b", b"c"] {
        push(&mut n, b);
    }
    // Evict far beyond head (head = 3): floor clamps to head + 1 = 4.
    n.handle(Input::Evict { up_to_seq: 100 });
    assert_eq!(n.floor(), 4);
    // Every prior cursor is now expired; next_cursor = floor - 1 = 3 = head.
    let out = read_probe(&mut n, "s", Some(Cursor::new(1)));
    let r = only_response(&out);
    assert_eq!(r.state, State::Expired);
    assert_eq!(r.next_cursor, Cursor::new(3));
    // A read at head is caught-up-live (nothing evicted below it matters).
    let out = read_probe(&mut n, "s2", Some(Cursor::new(3)));
    assert_eq!(only_response(&out).state, State::WouldBlock);
}

// ── ADVERSARIAL PROBES (verifier) ────────────────────────────────────────────

#[test]
fn probe_close_then_read_retained_data_returns_data() {
    let mut n = node();
    push(&mut n, b"a");
    push(&mut n, b"b");
    n.handle(Input::Close { error: None });
    // Below head, retained → must still be data (rule 4), not closed.
    let out = read_probe(&mut n, "s", Some(Cursor::START));
    let r = only_response(&out);
    assert_eq!(r.state, State::Data);
    assert_eq!(r.records, vec![rec(1, b"a"), rec(2, b"b")]);
}

#[test]
fn probe_supersede_with_data_available_cancel_before_response() {
    let mut n = node();
    // Held read with a timer at the tail (empty log).
    n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    push(&mut n, b"a"); // answers + cancels; s now has no held read
                        // Re-establish a held read with a timer, then supersede while data exists.
    n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::new(1)), // at head → held
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    // New read from START (data available) supersedes: CancelTimer then Data.
    let out = read_probe(&mut n, "s", Some(Cursor::START));
    // Order: CancelTimer must precede the Response.
    let cancel_idx = out
        .iter()
        .position(|o| matches!(o, Output::CancelTimer { .. }));
    let resp_idx = out.iter().position(|o| matches!(o, Output::Response(_)));
    assert!(cancel_idx.is_some(), "expected a CancelTimer: {out:?}");
    assert!(cancel_idx < resp_idx, "CancelTimer must precede Response");
    assert_eq!(only_response(&out).state, State::Data);
}

#[test]
fn probe_never_read_close_still_fires_producer_stop_last_reader() {
    // A log constructed with LastReader that is NEVER read: Close must still
    // fire ProducerStop (Close always fires), and must NOT fire LastReaderGone.
    let mut n = node_last_reader();
    let out = n.handle(Input::Close { error: None });
    let stops: Vec<_> = out
        .iter()
        .filter_map(|o| match o {
            Output::ProducerStop { reason } => Some(*reason),
            _ => None,
        })
        .collect();
    assert_eq!(stops, vec![StopReason::Closed], "exactly one Closed stop");
}

#[test]
fn probe_end_stream_nonlast_no_producer_stop() {
    let mut n = node_last_reader();
    read_hold(&mut n, "s1", Some(Cursor::START));
    read_hold(&mut n, "s2", Some(Cursor::START));
    // End s1: count 2 → 1, NOT a ≥1→0 transition.
    let out = n.handle(Input::EndStream {
        stream_id: sid("s1"),
    });
    assert!(
        !out.iter().any(|o| matches!(o, Output::ProducerStop { .. })),
        "2→1 must not fire ProducerStop: {out:?}"
    );
}

#[test]
fn probe_cursor_at_floor_minus_one_boundary_below_is_expired() {
    let mut n = node();
    for b in [b"a", b"b", b"c", b"d"] {
        push(&mut n, b);
    }
    n.handle(Input::Evict { up_to_seq: 2 }); // floor = 3
                                             // seq 1: 1+1=2 < 3 → expired, next = floor-1 = 2
    let out = read_probe(&mut n, "s", Some(Cursor::new(1)));
    let r = only_response(&out);
    assert_eq!(r.state, State::Expired);
    assert_eq!(r.next_cursor, Cursor::new(2));
}

#[test]
fn probe_forward_progress_first_record_over_max_bytes_when_held_released() {
    // A held read (no timeout) with a tiny max_bytes released by a Push whose
    // single record exceeds max_bytes must STILL deliver that record (D10 fwd
    // progress must hold on the release path too, not only the sync path).
    let mut n = node();
    n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::START),
        limits: Limits {
            max_records: None,
            max_bytes: Some(1),
        },
        timeout_ms: None,
    });
    let out = push(&mut n, b"huge_payload"); // 12 bytes > 1
    let r = only_response(&out);
    assert_eq!(r.state, State::Data);
    assert_eq!(r.records.len(), 1, "forward progress on release path");
}

#[test]
fn probe_multi_stream_timer_expired_answers_only_owner() {
    let mut n = node();
    // s1 timer token 1, s2 timer token 2.
    n.handle(Input::Read {
        stream_id: sid("s1"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    n.handle(Input::Read {
        stream_id: sid("s2"),
        cursor: Some(Cursor::START),
        limits: Limits::default(),
        timeout_ms: Some(200),
    });
    // Expire token 2: only s2 answered.
    let out = n.handle(Input::TimerExpired {
        token: TimerToken(2),
    });
    let r = only_response(&out);
    assert_eq!(r.stream_id, sid("s2"));
    assert_eq!(r.state, State::WouldBlock);
    // s1 still held.
    assert_eq!(n.stream_count(), 2);
}

#[test]
fn probe_probe_read_creates_persistent_session_and_reader_count() {
    // A probe read (timeout 0) that returns would_block still creates a
    // session entry (D4 implicit create) — reader count must be 1, and under
    // LastReader a subsequent EndStream must fire the 1→0 stop.
    let mut n = node_last_reader();
    let out = read_probe(&mut n, "s", Some(Cursor::START));
    assert_eq!(only_response(&out).state, State::WouldBlock);
    assert_eq!(n.stream_count(), 1, "probe read creates a session entry");
    let out = n.handle(Input::EndStream {
        stream_id: sid("s"),
    });
    assert!(
        out.iter().any(
            |o| matches!(o, Output::ProducerStop { reason } if *reason == StopReason::LastReaderGone)
        ),
        "EndStream after probe read must be a 1→0 transition: {out:?}"
    );
}

#[test]
fn probe_push_after_seal_behavior() {
    // D19: Push after Seal is DROPPED — nothing appended, head unchanged — and
    // emits exactly one `push_after_terminal` warning (never silent, never
    // fatal). This replaces the earlier as-built behavior where the window
    // accepted the late push.
    let mut n = node();
    push(&mut n, b"a");
    n.handle(Input::Seal);
    let out = push(&mut n, b"b");
    assert_eq!(n.head(), 1, "D19: push after seal must NOT advance head");
    assert_eq!(
        out,
        vec![diag_push_after_terminal()],
        "D19: exactly one push_after_terminal warning, no records/responses",
    );
}

#[test]
fn probe_supersede_data_then_no_persistent_held() {
    // Supersede a held read with a Data read; afterward no held read remains,
    // so a later Push must NOT re-answer that stream.
    let mut n = node();
    push(&mut n, b"a");
    // Held read at head (seq 1) with timer.
    n.handle(Input::Read {
        stream_id: sid("s"),
        cursor: Some(Cursor::new(1)),
        limits: Limits::default(),
        timeout_ms: Some(100),
    });
    // Supersede from START → Data (record 1), sync.
    let out = read_probe(&mut n, "s", Some(Cursor::START));
    assert_eq!(only_response(&out).state, State::Data);
    // A later Push must not produce a Response for s (no held read now).
    let out = push(&mut n, b"b");
    assert!(
        !out.iter().any(|o| matches!(o, Output::Response(_))),
        "no held read should remain after a Data-resolved read: {out:?}"
    );
}

#[test]
fn evict_zero_is_noop() {
    let mut n = node();
    push(&mut n, b"a");
    n.handle(Input::Evict { up_to_seq: 0 });
    assert_eq!(n.floor(), 0, "evict(0) changes nothing");
    let out = read_probe(&mut n, "s", Some(Cursor::START));
    assert_eq!(only_response(&out).records, vec![rec(1, b"a")]);
}

// ── D19: Push after a terminal lifecycle is dropped + warns ─────────────────
//
// Each terminal state (sealed / closed / failed) must, on a late `Push`: leave
// head unchanged, append nothing (no record ever becomes visible to a reader),
// and emit exactly one `LogDiagnostic{Warn, PushAfterTerminal}` per late push
// (two late pushes ⇒ two diagnostics). Never silent, never fatal.

/// Assert a late push was fully dropped: head unchanged, the ONLY output is one
/// `push_after_terminal` diagnostic (no Response, no Record), and a fresh read
/// below head never surfaces the dropped payload.
fn assert_push_dropped_and_warned(n: &mut LogNode, head_before: u64) {
    let out = push(n, b"late");
    assert_eq!(n.head(), head_before, "D19: head must not advance");
    assert_eq!(
        out,
        vec![diag_push_after_terminal()],
        "D19: the only output is one push_after_terminal warning: {out:?}",
    );
    // No dropped record is visible to a reader: read the whole log from START.
    let read = read_probe(n, "reader-check", Some(Cursor::START));
    for o in &read {
        if let Output::Response(r) = o {
            assert!(
                r.records.iter().all(|rec| rec.payload != b"late".to_vec()),
                "D19: dropped payload must never be visible to a reader: {r:?}",
            );
        }
    }
}

#[test]
fn d19_push_after_seal_dropped_head_unchanged_one_diagnostic() {
    let mut n = node();
    push(&mut n, b"a"); // head = 1
    n.handle(Input::Seal);
    assert_push_dropped_and_warned(&mut n, 1);
}

#[test]
fn d19_push_after_close_dropped_head_unchanged_one_diagnostic() {
    let mut n = node();
    push(&mut n, b"a"); // head = 1
    n.handle(Input::Close { error: None });
    assert_push_dropped_and_warned(&mut n, 1);
}

#[test]
fn d19_push_after_failed_dropped_head_unchanged_one_diagnostic() {
    let mut n = node();
    push(&mut n, b"a"); // head = 1
    n.handle(Input::Close {
        error: Some(Error {
            code: ErrorCode::ProducerError,
            message: Some("boom".into()),
        }),
    });
    assert_push_dropped_and_warned(&mut n, 1);
}

#[test]
fn d19_two_late_pushes_emit_two_diagnostics() {
    // One diagnostic PER late push (D19): two late pushes ⇒ two diagnostics.
    for terminal in [
        Input::Seal,
        Input::Close { error: None },
        Input::Close {
            error: Some(Error {
                code: ErrorCode::Internal,
                message: None,
            }),
        },
    ] {
        let mut n = node();
        push(&mut n, b"a"); // head = 1
        n.handle(terminal);
        let out1 = push(&mut n, b"x");
        let out2 = push(&mut n, b"y");
        assert_eq!(
            count_diagnostics(&out1),
            1,
            "first late push ⇒ 1 diagnostic"
        );
        assert_eq!(
            count_diagnostics(&out2),
            1,
            "second late push ⇒ 1 more diagnostic (per-push, not once)",
        );
        assert_eq!(n.head(), 1, "neither late push advanced head");
        assert_eq!(out1, vec![diag_push_after_terminal()]);
        assert_eq!(out2, vec![diag_push_after_terminal()]);
    }
}

#[test]
fn d19_late_push_does_not_release_held_reads() {
    // A held tail read must NOT be released by a dropped push (head did not
    // move). The only output of the late push is the diagnostic; the held read
    // stays parked and is later answered by the terminal-state sweep semantics
    // already exercised elsewhere.
    let mut n = node();
    push(&mut n, b"a"); // head = 1
    n.handle(Input::Seal);
    // A held read at head after seal would already be answered eof by the sweep;
    // instead park a read at head BEFORE the late push via a fresh stream at
    // head. Seal already ran, so a read at head is terminal (eof), not held —
    // so use a probe-free hold on a stream whose cursor is at head: it resolves
    // eof immediately, not held. To genuinely test "no release", assert the
    // late push emits ONLY the diagnostic (no Response) regardless of streams.
    read_probe(&mut n, "s", Some(Cursor::new(1))); // eof, creates session, no held
    let out = push(&mut n, b"late");
    assert_eq!(
        out,
        vec![diag_push_after_terminal()],
        "late push must emit only the diagnostic, never a Response: {out:?}",
    );
}
