//! The store core: a bounded in-engine record window (§A.5, D2).
//!
//! Distilled from glade `store.rs`: the `scan(from)` resume discipline
//! (no dup / no skip) with `heads` collapsed to a single origin. This module
//! knows nothing of streams, held reads, timers, or watermarks — those live in
//! [`super::session`].

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::types::{Bytes, Error, Limits, Record};

/// The lifecycle of the backing log (D12). `Live` is the only non-terminal
/// state; the three terminals describe the *log*, not any stream, and still
/// permit re-reads of retained data (shared §3.4 rule 4).
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Lifecycle {
    Live,
    /// Finite log complete: drained readers see `eof`.
    Sealed,
    /// `Close{}`: drained readers see `closed`.
    Closed,
    /// `Close{error}`: drained readers see `failed`, with the error attached.
    Failed(Error),
}

/// The store core. Records live in a `VecDeque` ordered by ascending `seq`;
/// `head` is the highest assigned seq (0 when empty, D8); `floor` is the lowest
/// retained seq (0 when nothing has been evicted).
pub(crate) struct Window {
    records: VecDeque<Record>,
    head: u64,
    floor: u64,
    lifecycle: Lifecycle,
}

impl Window {
    pub(crate) fn new() -> Self {
        Window {
            records: VecDeque::new(),
            head: 0,
            floor: 0,
            lifecycle: Lifecycle::Live,
        }
    }

    /// Highest assigned seq; 0 when empty (D8).
    pub(crate) fn head(&self) -> u64 {
        self.head
    }

    /// Lowest retained seq; 0 when nothing has been evicted.
    pub(crate) fn floor(&self) -> u64 {
        self.floor
    }

    pub(crate) fn lifecycle(&self) -> &Lifecycle {
        &self.lifecycle
    }

    /// Append one record: assigns `seq := head + 1` (first record `seq = 1`,
    /// D8) and returns the assigned seq. A no-op-free append: even after a
    /// terminal lifecycle the caller (node) is responsible for gating pushes;
    /// the window itself always accepts (kept simple and total).
    pub(crate) fn push(&mut self, payload: Bytes) -> u64 {
        self.head += 1;
        let seq = self.head;
        self.records.push_back(Record { seq, payload });
        seq
    }

    /// Mark the log sealed. Idempotent; a terminal `Close`/`Failed` is not
    /// overwritten by a later `Seal` (Close wins — terminal is terminal).
    pub(crate) fn seal(&mut self) {
        if matches!(self.lifecycle, Lifecycle::Live) {
            self.lifecycle = Lifecycle::Sealed;
        }
    }

    /// Mark the log closed (`error: None` → `Closed`; `Some` → `Failed`).
    /// Idempotent: the first terminal transition wins (a later `Close` does not
    /// mutate an already-terminal state).
    pub(crate) fn close(&mut self, error: Option<Error>) {
        if matches!(self.lifecycle, Lifecycle::Live | Lifecycle::Sealed) {
            self.lifecycle = match error {
                None => Lifecycle::Closed,
                Some(e) => Lifecycle::Failed(e),
            };
        }
    }

    /// Records with `seq > from`, in ascending order, bounded by `limits` with
    /// the D10 forward-progress guarantee (≥1 record whenever any is
    /// available, even if it alone exceeds `max_bytes`). `max_bytes` counts
    /// **raw payload bytes only**. Returns the records plus the seq of the last
    /// one returned (the caller's `next_cursor` on `data`).
    ///
    /// Assumes the caller has already classified `from` as valid-for-data
    /// (`from < head`, not below floor). Returns an empty vec + `from` if there
    /// is nothing strictly after `from`.
    ///
    /// 56-F5: `records` is dense (`records[i].seq == front.seq + i`, since
    /// `push` only appends the next seq and `evict` only drops a front prefix),
    /// so the first candidate index is computed directly from the front
    /// record's seq instead of scanning from the front and skipping already-
    /// consumed records. A read is `O(K)` for `K` returned records rather than
    /// `O(N)` in the retained window size.
    pub(crate) fn scan(&self, from: u64, limits: Limits) -> (Vec<Record>, u64) {
        let mut out = Vec::new();
        let mut last = from;
        let mut bytes: u64 = 0;
        let n = self.records.len();
        let start = match self.records.front() {
            Some(front) if from >= front.seq => (from - front.seq + 1) as usize,
            Some(_) => 0,
            None => return (out, last),
        };
        for i in start..n {
            let rec = &self.records[i];
            debug_assert!(rec.seq > from, "dense-sequence invariant violated");
            // max_records bound.
            if let Some(maxr) = limits.max_records {
                if out.len() as u64 >= maxr as u64 {
                    break;
                }
            }
            // max_bytes bound, with forward progress: always take at least one.
            if let Some(maxb) = limits.max_bytes {
                let next_bytes = bytes.saturating_add(rec.payload.len() as u64);
                if !out.is_empty() && next_bytes > maxb {
                    break;
                }
                bytes = next_bytes;
            }
            last = rec.seq;
            out.push(rec.clone());
        }
        (out, last)
    }

    /// Drop records with `seq <= up_to_seq`, raising the floor (D7). The floor
    /// becomes the lowest seq still retained (so `floor - 1` is the last
    /// evicted seq — the D9 earliest-resumable position). Clamped so eviction
    /// never claims to have dropped beyond `head`.
    pub(crate) fn evict(&mut self, up_to_seq: u64) {
        if up_to_seq == 0 {
            return;
        }
        while let Some(front) = self.records.front() {
            if front.seq <= up_to_seq {
                self.records.pop_front();
            } else {
                break;
            }
        }
        // The new floor is the lowest retained seq. If everything ≤ up_to_seq
        // is gone, that is up_to_seq + 1 (clamped to head + 1 when the window
        // is now empty but records had existed). Floor only ever rises.
        let evicted_through = up_to_seq.min(self.head);
        let new_floor = evicted_through + 1;
        if new_floor > self.floor {
            self.floor = new_floor;
        }
    }
}
