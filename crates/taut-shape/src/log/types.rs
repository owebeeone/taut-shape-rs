//! Core types (shared §3.1, rendered §A.1).
//!
//! These are the hand-written, idiomatic Rust renderings of the log
//! vocabulary's data types. The wire-facing structs live in [`crate::generated`]
//! (D17); these are the ergonomic in-engine forms (`u64` seq/tokens, `Arc<str>`
//! ids, an `Option`-per-axis `Limits`). Conversion to/from the generated types
//! is the tool crate's framing concern, not the engine's.

use alloc::string::String;
use alloc::sync::Arc;

/// An ordered position in one log. "Records strictly after `seq` are unseen."
///
/// Named type (not a bare integer) so it can grow (`byte_offset` reserved,
/// shared §3.1) — v0 is seq-only.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub struct Cursor {
    pub seq: u64,
}

impl Cursor {
    /// D8: first record is `seq = 1`; `START = {seq: 0}`; empty log `head = 0`.
    pub const START: Cursor = Cursor { seq: 0 };

    /// Convenience constructor.
    pub const fn new(seq: u64) -> Self {
        Cursor { seq }
    }
}

/// One appended record. Payload is opaque, binary-safe (NUL-safe) bytes —
/// specifically the method's append-type message **already taut-encoded** by
/// the producer (the glade `Op.payload` pattern, D17), never raw app bytes.
/// D11: a record carries its own `seq`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Record {
    pub seq: u64,
    pub payload: Bytes,
}

/// D13: the canonical state alphabet, one-to-one with the oracle strings.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    /// Records returned; keep reading.
    Data,
    /// Caught up, live: probe / timeout answer (D14).
    WouldBlock,
    /// Sealed and drained (D12).
    Eof,
    /// `Close{}` teardown, no error (D12).
    Closed,
    /// `Close{error}`; `error` attached to the response (D12).
    Failed,
    /// Invalid cursor — a STATE, never an error (D9);
    /// `next_cursor` = earliest resumable position.
    Expired,
}

/// The error carrier attached to `failed` responses (D12) and to the service's
/// unknown-log answer. This is wire vocabulary, not a Rust error type — it is
/// never `?`-propagated.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Error {
    pub code: ErrorCode,
    pub message: Option<String>,
}

/// `UnknownLog` is **service-level only** (§A.8 / shared §3.5); `LogNode`
/// itself only ever attaches `ProducerError` / `Internal`. There is no
/// `canceled` code: client cancellation is `EndStream`, which has no response.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ErrorCode {
    UnknownLog,
    ProducerError,
    Internal,
}

/// One stream instance (D3): one logical read loop with its own position.
/// Minted by the *consumer* (the engine never allocates ids); wire form is a
/// string, so a cheaply-clonable shared str. Many per client; disposable —
/// position lives in the client-held cursor.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StreamId(pub Arc<str>);

impl StreamId {
    /// Build a `StreamId` from anything string-like.
    pub fn new(id: impl AsRef<str>) -> Self {
        StreamId(Arc::from(id.as_ref()))
    }
}

impl From<&str> for StreamId {
    fn from(s: &str) -> Self {
        StreamId(Arc::from(s))
    }
}

/// Timer correlation token. D16: allocated by the engine, monotonic from 1.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TimerToken(pub u64);

/// Batch bounds for a Read. `None` on an axis = unbounded on that axis.
/// D10: `max_bytes` counts **raw payload bytes only**, with the
/// forward-progress guarantee (≥1 record whenever any is available).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Limits {
    pub max_records: Option<u32>,
    pub max_bytes: Option<u64>,
}

/// Why the producer was told to stop (D6).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopReason {
    LastReaderGone,
    Closed,
    Failed,
}

/// The opaque payload type. Owned `Vec<u8>` in v0 (R2 open question).
pub type Bytes = alloc::vec::Vec<u8>;
