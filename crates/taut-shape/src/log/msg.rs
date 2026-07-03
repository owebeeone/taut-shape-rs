//! The hand-written `Input`/`Output` unions over the core types (§A.2–A.3).
//!
//! Per D17 only these two enums, the engine, and the shell are hand-written;
//! the message *data* types are generated ([`crate::generated`]). These enums
//! make [`crate::LogNode::handle`] a total, exhaustiveness-checked `match`.

use alloc::vec::Vec;

use crate::generated::LogDiagnostic;
use super::types::{Bytes, Cursor, Error, Limits, Record, State, StopReason, StreamId, TimerToken};

/// Everything that can happen to a log, as one enum. Producer-side inputs are
/// node-local and unaddressed (the producer lives with the node in v0);
/// stream-side inputs are addressed by `stream_id`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Input {
    // ── producer-side (node-local) ──────────────────────────────────────
    /// Append: assigns `seq := head + 1` (first record `seq = 1`, D8),
    /// answers any held reads. `payload` is the method's append-type message
    /// already taut-encoded (glade `Op.payload` pattern, D17), opaque here.
    Push { payload: Bytes },
    /// Finite log complete; held reads answered `eof`. Idempotent.
    Seal,
    /// Teardown. `error: None` → held reads answered `closed`;
    /// `Some(e)` → `failed` with `e` attached (D12). Timers canceled;
    /// `ProducerStop` emitted. Idempotent.
    Close { error: Option<Error> },

    // ── stream-side (addressed) ─────────────────────────────────────────
    /// `cursor: None` ⇒ `Cursor::START` (D8). First use of a `stream_id`
    /// implicitly creates its session entry (D4). A `Read` on a stream with a
    /// held read **supersedes** it: the old read is dropped without a
    /// response and its timer canceled (D5).
    ///
    /// `timeout_ms` (D14): `None` = hold indefinitely; `Some(0)` = probe
    /// (immediate `would_block`); `Some(n>0)` = hold + `SetTimer`.
    Read {
        stream_id: StreamId,
        cursor: Option<Cursor>,
        limits: Limits,
        timeout_ms: Option<u64>,
    },
    /// Drop the held read (no response), cancel its timer, remove the
    /// watermark, decrement the reader count (D4). Unknown `stream_id` =
    /// no-op. Adapters inject this on transport death — it IS the
    /// disconnect cleanup.
    EndStream { stream_id: StreamId },

    // ── environment ─────────────────────────────────────────────────────
    /// If `token` maps to a held read, answer it `would_block`; otherwise
    /// ignore (late/canceled timers are no-ops).
    TimerExpired { token: TimerToken },
    /// Drop records with `seq <= up_to_seq`, raising the floor. Retention is
    /// consumer-driven in v0 (D2, D7).
    Evict { up_to_seq: u64 },
}

/// The addressed read answer. `next_cursor` is ALWAYS present, even when
/// `records` is empty. A named struct (not inlined in the enum) so the shell
/// can route it by value.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Response {
    pub stream_id: StreamId,
    pub records: Vec<Record>,
    pub next_cursor: Cursor,
    pub state: State,
    /// Attached iff `state == Failed` (D12).
    pub error: Option<Error>,
}

// NOTE: `Output` is `PartialEq` but NOT `Eq`: the `Diagnostic` variant wraps the
// generated [`LogDiagnostic`], whose tautc output derives only `PartialEq` (kept
// byte-identical to the generator, so we cannot add `Eq` there). `Response` and
// the core types keep `Eq`; nothing keys a set/map on `Output`.
#[derive(Clone, PartialEq, Debug)]
pub enum Output {
    Response(Response),
    /// The engine's only "clock": the shell must arrange a
    /// `TimerExpired{token}` after ~`ms` (D14). Tokens monotonic from 1 (D16).
    SetTimer {
        token: TimerToken,
        ms: u64,
    },
    CancelTimer {
        token: TimerToken,
    },
    /// Emitted on `Close`, and on the reader-count ≥1 → 0 transition when
    /// constructed with `StopWhen::LastReader` (D6). The shell routes this to
    /// the producer; a shell that initiated the close ignores it.
    ProducerStop {
        reason: StopReason,
    },
    /// An engine warning delegated to the caller (D18): a sans-io engine cannot
    /// log, so the shell routes this to the host's logging facility. Wraps the
    /// generated [`LogDiagnostic`] (code-only, no free text — prose would freeze
    /// byte-identical strings into the cross-language oracle). First and only v0
    /// use: `Push` after a terminal lifecycle (D19) → one
    /// `LogDiagnostic{Warn, PushAfterTerminal}` per late push.
    Diagnostic(LogDiagnostic),
}
