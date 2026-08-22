//! The session table: one entry per live stream instance (§A.5, D3/D4).
//!
//! Maps glade `session.rs` / `client_heads`. Each entry is the per-stream
//! "response handler": its ≤1 held read (D5), the timer token that read is
//! waiting on (D14), its delivery watermark (D7), and a creation rank used to
//! order multi-wake emissions (D16). This module knows nothing of bytes or
//! retention — those live in [`super::window`].

use alloc::collections::BTreeMap;

use super::types::{Cursor, Limits, StreamId, TimerToken};

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
///
/// 56-F6: `held_order` and `timer_index` are auxiliary indices kept in
/// lockstep with `entries[*].held` by [`Table::set_held`]/[`Table::clear_held`]
/// (the only two ways `held` may change). They hold *only* the currently held
/// stream ids, so a wake ([`Table::held_in_creation_order`]) traverses `H`
/// (held reads) instead of `S` (all live streams), and a timer expiry
/// ([`Table::find_by_timer`]) is an `O(log H)` map lookup instead of an
/// `O(S)`/`O(S + H log H)` scan.
pub(crate) struct Table {
    entries: BTreeMap<StreamId, Entry>,
    next_rank: u64,
    /// Held stream ids, keyed by creation rank (D16) so iteration order is
    /// creation order for free — no per-wake sort.
    held_order: BTreeMap<u64, StreamId>,
    /// `timer_token -> stream_id`, updated on park/supersede/end/expiry (D14).
    timer_index: BTreeMap<TimerToken, StreamId>,
}

impl Table {
    pub(crate) fn new() -> Self {
        Table {
            entries: BTreeMap::new(),
            next_rank: 0,
            held_order: BTreeMap::new(),
            timer_index: BTreeMap::new(),
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
    /// the caller can cancel its timer. Clears any auxiliary held/timer index
    /// entries for it (56-F6).
    pub(crate) fn remove(&mut self, id: &StreamId) -> Option<Entry> {
        let entry = self.entries.remove(id)?;
        if let Some(held) = &entry.held {
            self.held_order.remove(&entry.created);
            if let Some(tok) = held.timer {
                self.timer_index.remove(&tok);
            }
        }
        Some(entry)
    }

    /// The minimum watermark across all live streams (D7), or `None` when
    /// there are no live streams. The safe eviction floor for the consumer.
    pub(crate) fn min_watermark(&self) -> Option<u64> {
        self.entries.values().map(|e| e.watermark).min()
    }

    /// Park (or replace) `id`'s held read, updating the creation-order and
    /// timer indices in lockstep (D16/D14). The caller must have already
    /// released any prior held read via [`Self::clear_held`].
    pub(crate) fn set_held(&mut self, id: &StreamId, held: HeldRead) {
        let created = self.get(id).expect("entry must exist").created;
        if let Some(tok) = held.timer {
            self.timer_index.insert(tok, id.clone());
        }
        self.held_order.insert(created, id.clone());
        self.entries.get_mut(id).unwrap().held = Some(held);
    }

    /// Release `id`'s held read, if any, clearing both auxiliary indices.
    /// Returns the removed `HeldRead`, or `None` if it was not held.
    pub(crate) fn clear_held(&mut self, id: &StreamId) -> Option<HeldRead> {
        let entry = self.entries.get_mut(id)?;
        let created = entry.created;
        let held = entry.held.take()?;
        self.held_order.remove(&created);
        if let Some(tok) = held.timer {
            self.timer_index.remove(&tok);
        }
        Some(held)
    }

    /// All stream ids that currently hold a read, in **creation order** (D16).
    /// Used to sweep held reads when an input (`Push`/`Seal`/`Close`) may
    /// release several of them. `O(H)`: `held_order` contains only held ids.
    pub(crate) fn held_in_creation_order(&self) -> alloc::vec::Vec<StreamId> {
        self.held_order.values().cloned().collect()
    }

    /// The stream id whose held read is waiting on `token`, if any (D14).
    /// `O(log H)` via the timer index, instead of scanning held reads.
    pub(crate) fn find_by_timer(&self, token: TimerToken) -> Option<StreamId> {
        self.timer_index.get(&token).cloned()
    }
}
