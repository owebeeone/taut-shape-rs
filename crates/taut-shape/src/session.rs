//! The session table: one entry per live stream instance (§A.5, D3/D4).
//!
//! Maps glade `session.rs` / `client_heads`. Each entry is the per-stream
//! "response handler": its ≤1 held read (D5), the timer token that read is
//! waiting on (D14), its delivery watermark (D7), and a creation rank used to
//! order multi-wake emissions (D16). This module knows nothing of bytes or
//! retention — those live in [`crate::window`].

use alloc::collections::BTreeMap;

use crate::types::{Cursor, Limits, StreamId, TimerToken};

/// A parked tail read (shared §3.4 rule 2, held branch). Its timeout has
/// already been resolved into an optional timer token by the node.
#[derive(Clone, Debug)]
pub(crate) struct HeldRead {
    pub(crate) cursor: Cursor,
    pub(crate) limits: Limits,
    /// The timer this held read is waiting on, if any (`timeout_ms` `> 0`).
    /// `None` = hold indefinitely (`timeout_ms` absent).
    pub(crate) timer: Option<TimerToken>,
}

/// One stream instance's state.
#[derive(Clone, Debug)]
pub(crate) struct Entry {
    /// Creation rank — the D16 emission order on a multi-read wake.
    pub(crate) created: u64,
    /// ≤1 outstanding read per stream (D5).
    pub(crate) held: Option<HeldRead>,
    /// Last delivered seq (D7); the eviction-safety watermark.
    pub(crate) watermark: u64,
}

/// The session table. Entries keyed by `stream_id`; `next_rank` is the
/// creation-order counter feeding [`Entry::created`].
pub(crate) struct Table {
    entries: BTreeMap<StreamId, Entry>,
    next_rank: u64,
}

impl Table {
    pub(crate) fn new() -> Self {
        Table {
            entries: BTreeMap::new(),
            next_rank: 0,
        }
    }

    /// Number of live stream instances (D4 reader count).
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get an existing entry, or implicitly create it on first use (D4).
    /// Returns a mutable reference to the entry.
    pub(crate) fn get_or_create(&mut self, id: &StreamId) -> &mut Entry {
        if !self.entries.contains_key(id) {
            let created = self.next_rank;
            self.next_rank += 1;
            self.entries.insert(
                id.clone(),
                Entry {
                    created,
                    held: None,
                    watermark: 0,
                },
            );
        }
        self.entries.get_mut(id).unwrap()
    }

    pub(crate) fn get(&self, id: &StreamId) -> Option<&Entry> {
        self.entries.get(id)
    }

    pub(crate) fn get_mut(&mut self, id: &StreamId) -> Option<&mut Entry> {
        self.entries.get_mut(id)
    }

    /// Remove a stream instance (D4 `EndStream`). Returns the removed entry so
    /// the caller can cancel its timer.
    pub(crate) fn remove(&mut self, id: &StreamId) -> Option<Entry> {
        self.entries.remove(id)
    }

    /// The minimum watermark across all live streams (D7), or `None` when
    /// there are no live streams. The safe eviction floor for the consumer.
    pub(crate) fn min_watermark(&self) -> Option<u64> {
        self.entries.values().map(|e| e.watermark).min()
    }

    /// All stream ids that currently hold a read, in **creation order** (D16).
    /// Used to sweep held reads when an input (`Push`/`Seal`/`Close`) may
    /// release several of them.
    pub(crate) fn held_in_creation_order(&self) -> alloc::vec::Vec<StreamId> {
        let mut held: alloc::vec::Vec<(u64, StreamId)> = self
            .entries
            .iter()
            .filter(|(_, e)| e.held.is_some())
            .map(|(id, e)| (e.created, id.clone()))
            .collect();
        held.sort_by_key(|(rank, _)| *rank);
        held.into_iter().map(|(_, id)| id).collect()
    }
}
