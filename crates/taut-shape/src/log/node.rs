//! The engine: `LogNode` — the pure mailbox endpoint (§A.4, D1).
//!
//! No I/O, no clock, no locks, no callbacks. Held long-polls are ENGINE STATE:
//! a tail `Read` that cannot be answered is parked in the [session
//! table](super::session) and answered when a later input (`Push`/`Seal`/
//! `Close`/`TimerExpired`/`EndStream`) releases it. Outputs are a deterministic
//! function of the input history (D16). Unsynchronized by design — the shell
//! owns serialization (D15).

use alloc::vec;
use alloc::vec::Vec;

use super::msg::{Input, Output, Response};
use super::session::{HeldRead, Table};
use super::types::{Cursor, Error, Limits, State, StopReason, StreamId, TimerToken};
use super::window::{Lifecycle, Window};
use crate::generated::{LogDiagCode, LogDiagnostic, LogSeverity};

/// Construction knob for `ProducerStop` (D6). A log never read must not
/// spuriously stop its producer, so the ≥1→0 reader transition is what fires.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum StopWhen {
    /// Fire `ProducerStop{LastReaderGone}` on the reader-count ≥1→0 transition.
    LastReader,
    /// Never fire on reader loss; only `Close` emits `ProducerStop`.
    #[default]
    ExplicitOnly,
}

/// Engine construction config.
#[derive(Clone, Copy, Debug, Default)]
pub struct Config {
    pub stop_when: StopWhen,
}

/// The pure mailbox endpoint: one per log.
pub struct LogNode {
    window: Window,
    sessions: Table,
    stop_when: StopWhen,
    /// Monotonic timer-token allocator, from 1 (D16).
    next_timer: u64,
}

impl LogNode {
    pub fn new(config: Config) -> Self {
        LogNode {
            window: Window::new(),
            sessions: Table::new(),
            stop_when: config.stop_when,
            next_timer: 0,
        }
    }

    /// The whole API. Total — never panics on protocol-level misuse and never
    /// returns a Rust error: protocol outcomes are messages. Outputs are a
    /// deterministic function of the input history (D16).
    pub fn handle(&mut self, input: Input) -> Vec<Output> {
        match input {
            Input::Push { payload } => self.on_push(payload),
            Input::Seal => self.on_seal(),
            Input::Close { error } => self.on_close(error),
            Input::Read {
                stream_id,
                cursor,
                limits,
                timeout_ms,
            } => self.on_read(stream_id, cursor, limits, timeout_ms),
            Input::EndStream { stream_id } => self.on_end_stream(stream_id),
            Input::TimerExpired { token } => self.on_timer_expired(token),
            Input::Evict { up_to_seq } => self.on_evict(up_to_seq),
        }
    }

    // ── read-only accessors (§A.4) ──────────────────────────────────────────

    /// Highest assigned seq; 0 when empty (D8).
    pub fn head(&self) -> u64 {
        self.window.head()
    }

    /// Lowest retained seq; 0 when nothing evicted.
    pub fn floor(&self) -> u64 {
        self.window.floor()
    }

    /// The minimum per-stream watermark, or `None` when no streams (D7).
    pub fn min_watermark(&self) -> Option<u64> {
        self.sessions.min_watermark()
    }

    /// Number of live stream instances.
    pub fn stream_count(&self) -> usize {
        self.sessions.len()
    }

    // ── internals ───────────────────────────────────────────────────────────

    fn alloc_timer(&mut self) -> TimerToken {
        self.next_timer += 1;
        TimerToken(self.next_timer)
    }

    /// Classify + resolve a `Read`, returning the immediate outputs. If the
    /// read must be held, the entry is parked and (for `timeout_ms > 0`) a
    /// `SetTimer` output is produced.
    fn on_read(
        &mut self,
        stream_id: StreamId,
        cursor: Option<Cursor>,
        limits: Limits,
        timeout_ms: Option<u64>,
    ) -> Vec<Output> {
        let cursor = cursor.unwrap_or(Cursor::START);
        let mut out = Vec::new();

        // D5 supersede: a new Read on a stream with a held read drops the old
        // one unanswered and cancels its timer.
        self.sessions.get_or_create(&stream_id);
        if let Some(prev) = self.sessions.clear_held(&stream_id) {
            if let Some(tok) = prev.timer {
                out.push(Output::CancelTimer { token: tok });
            }
        }

        // Resolve.
        match self.classify(cursor) {
            Resolution::Data => {
                let (records, last) = self.window.scan(cursor.seq, limits);
                // scan is only reached when data exists, so records is non-empty.
                let entry = self.sessions.get_mut(&stream_id).unwrap();
                entry.watermark = last;
                out.push(Output::Response(Response {
                    stream_id,
                    records,
                    next_cursor: Cursor::new(last),
                    state: State::Data,
                    error: None,
                }));
            }
            Resolution::Terminal(state, error) => {
                out.push(Output::Response(Response {
                    stream_id,
                    records: Vec::new(),
                    next_cursor: cursor,
                    state,
                    error,
                }));
            }
            Resolution::Expired(next_cursor) => {
                out.push(Output::Response(Response {
                    stream_id,
                    records: Vec::new(),
                    next_cursor,
                    state: State::Expired,
                    error: None,
                }));
            }
            Resolution::CaughtUpLive => {
                // shared §3.4 rule 2, live branch — depends on timeout_ms (D14).
                match timeout_ms {
                    Some(0) => {
                        // Immediate would_block probe.
                        out.push(Output::Response(Response {
                            stream_id,
                            records: Vec::new(),
                            next_cursor: cursor,
                            state: State::WouldBlock,
                            error: None,
                        }));
                    }
                    None => {
                        // Hold indefinitely.
                        self.sessions.set_held(
                            &stream_id,
                            HeldRead {
                                cursor,
                                limits,
                                timer: None,
                            },
                        );
                    }
                    Some(ms) => {
                        // Hold + SetTimer.
                        let token = self.alloc_timer();
                        self.sessions.set_held(
                            &stream_id,
                            HeldRead {
                                cursor,
                                limits,
                                timer: Some(token),
                            },
                        );
                        out.push(Output::SetTimer { token, ms });
                    }
                }
            }
        }
        out
    }

    /// Classify a cursor against the current window (shared §3.4 rules 1–4).
    fn classify(&self, cursor: Cursor) -> Resolution {
        let head = self.window.head();
        let floor = self.window.floor();
        let c = cursor.seq;

        // Rule 3a: beyond head — position never existed → expired, next = head.
        if c > head {
            return Resolution::Expired(Cursor::new(head));
        }
        // Rule 3b: below floor — records were evicted → expired, next = floor-1.
        // `c + 1 < floor` (D9). floor==0 means nothing evicted, never trips.
        if floor > 0 && c + 1 < floor {
            return Resolution::Expired(Cursor::new(floor - 1));
        }
        // Rule 1: data available (c < head, records retained after c).
        if c < head {
            return Resolution::Data;
        }
        // Rule 2: caught up (c == head). Depends on lifecycle.
        match self.window.lifecycle() {
            Lifecycle::Sealed => Resolution::Terminal(State::Eof, None),
            Lifecycle::Closed => Resolution::Terminal(State::Closed, None),
            Lifecycle::Failed(e) => Resolution::Terminal(State::Failed, Some(e.clone())),
            Lifecycle::Live => Resolution::CaughtUpLive,
        }
    }

    fn on_push(&mut self, payload: super::types::Bytes) -> Vec<Output> {
        // D19: a Push after any terminal lifecycle (Sealed/Closed/Failed) is
        // dropped — nothing appended, head unchanged — and emits exactly one
        // `push_after_terminal` warning per late push. A late in-flight push is
        // an expected race with `ProducerStop`, made visible, never silent or
        // fatal. Held reads are untouched: head did not move, so nothing to
        // release.
        if !matches!(self.window.lifecycle(), Lifecycle::Live) {
            return vec![Output::Diagnostic(LogDiagnostic {
                severity: LogSeverity::Warn,
                code: LogDiagCode::PushAfterTerminal,
            })];
        }
        self.window.push(payload);
        // Data now available for held reads: release them in creation order.
        self.release_held()
    }

    fn on_seal(&mut self) -> Vec<Output> {
        self.window.seal();
        self.release_held()
    }

    fn on_close(&mut self, error: Option<Error>) -> Vec<Output> {
        // D6 (refined): `ProducerStop` fires on the TRANSITION into a terminal
        // state via `Close`. A `Close` on an already-terminal (Closed/Failed)
        // log is a no-op that emits NOTHING — outputs-idempotent, symmetric with
        // `Seal`: no held reads to answer (they were drained on the first
        // terminal), no timers to cancel, no re-`ProducerStop`. A `Close` on a
        // `Live` OR `Sealed` log is a real transition, so it runs the full path.
        if matches!(
            self.window.lifecycle(),
            Lifecycle::Closed | Lifecycle::Failed(_)
        ) {
            return Vec::new();
        }
        let failed = error.is_some();
        self.window.close(error);
        let mut out = self.release_held();
        // Real transition into a terminal state via Close ⇒ ProducerStop (D6).
        let reason = if failed {
            StopReason::Failed
        } else {
            StopReason::Closed
        };
        out.push(Output::ProducerStop { reason });
        out
    }

    fn on_evict(&mut self, up_to_seq: u64) -> Vec<Output> {
        self.window.evict(up_to_seq);
        // Eviction does not release held reads (they are caught-up at head,
        // above any evicted region).
        Vec::new()
    }

    fn on_end_stream(&mut self, stream_id: StreamId) -> Vec<Output> {
        let mut out = Vec::new();
        let before = self.sessions.len();
        if let Some(entry) = self.sessions.remove(&stream_id) {
            // Drop held read (no response), cancel its timer.
            if let Some(held) = entry.held {
                if let Some(tok) = held.timer {
                    out.push(Output::CancelTimer { token: tok });
                }
            }
            // D6: reader-count ≥1→0 transition under StopWhen::LastReader.
            let after = self.sessions.len();
            if self.stop_when == StopWhen::LastReader && before >= 1 && after == 0 {
                out.push(Output::ProducerStop {
                    reason: StopReason::LastReaderGone,
                });
            }
        }
        // Unknown stream_id = no-op (nothing removed, no outputs).
        out
    }

    fn on_timer_expired(&mut self, token: TimerToken) -> Vec<Output> {
        // Find the held read waiting on this token; answer it would_block.
        // Unknown/canceled token = no-op. 56-F6: O(log H) index lookup
        // instead of scanning held reads for a matching token.
        let Some(id) = self.sessions.find_by_timer(token) else {
            return Vec::new();
        };
        let held = self.sessions.clear_held(&id).unwrap();
        vec![Output::Response(Response {
            stream_id: id,
            records: Vec::new(),
            next_cursor: held.cursor,
            state: State::WouldBlock,
            error: None,
        })]
    }

    /// Sweep held reads in creation order (D16), releasing each the current
    /// window state can now answer. Every held read is re-resolved from its
    /// parked cursor: a `Push` may now yield `data`; a `Seal`/`Close` yields
    /// the terminal state (or `data`, if pushes were buffered before it —
    /// re-resolution covers both). A read that still classifies as caught-up +
    /// live is re-parked, untouched.
    fn release_held(&mut self) -> Vec<Output> {
        let mut out = Vec::new();
        // 56-F6: held_in_creation_order() is O(H), not O(S) / O(S + H log H).
        for id in self.sessions.held_in_creation_order() {
            // Take the held read; re-resolve from its parked cursor.
            let held = self.sessions.clear_held(&id).unwrap();
            let resp = self.resolve_held(&id, &held, &mut out);
            match resp {
                Some(response) => out.push(Output::Response(response)),
                None => {
                    // Still cannot answer (caught up + live): re-park it.
                    self.sessions.set_held(&id, held);
                }
            }
        }
        out
    }

    /// Re-resolve a released held read. Returns the `Response` to emit, or
    /// `None` if it should stay parked (still caught-up + live). Cancels the
    /// held read's timer as a side effect when the read is now answerable.
    fn resolve_held(
        &mut self,
        id: &StreamId,
        held: &HeldRead,
        out: &mut Vec<Output>,
    ) -> Option<Response> {
        let cursor = held.cursor;
        let limits: Limits = held.limits;
        let response = match self.classify(cursor) {
            Resolution::Data => {
                let (records, last) = self.window.scan(cursor.seq, limits);
                let entry = self.sessions.get_mut(id).unwrap();
                entry.watermark = last;
                Response {
                    stream_id: id.clone(),
                    records,
                    next_cursor: Cursor::new(last),
                    state: State::Data,
                    error: None,
                }
            }
            Resolution::Terminal(state, error) => Response {
                stream_id: id.clone(),
                records: Vec::new(),
                next_cursor: cursor,
                state,
                error,
            },
            Resolution::Expired(next_cursor) => Response {
                stream_id: id.clone(),
                records: Vec::new(),
                next_cursor,
                state: State::Expired,
                error: None,
            },
            Resolution::CaughtUpLive => return None,
        };
        // Answerable: cancel its timer if it had one.
        if let Some(tok) = held.timer {
            out.push(Output::CancelTimer { token: tok });
        }
        Some(response)
    }
}

/// The outcome of classifying a cursor (shared §3.4).
enum Resolution {
    /// Rule 1: records available after the cursor.
    Data,
    /// Rule 2 terminal branch: sealed/closed/failed at head.
    Terminal(State, Option<Error>),
    /// Rule 3: invalid cursor; carries the earliest-resumable `next_cursor`.
    Expired(Cursor),
    /// Rule 2 live branch: caught up, log still live — hold or probe.
    CaughtUpLive,
}
