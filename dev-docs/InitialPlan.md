# taut-shape-rs — Rust Implementation Plan

Status: plan (v2 — conforms to the shared **mailbox-engine** model). Scope:
phased plan, **Rust-specific**.
Audience: contributors to `taut-shape-rs`.

This document **specializes** the shared, language-neutral plan
[`../../taut-shape/dev-docs/TautClientImplPlan.md`](../../taut-shape/dev-docs/TautClientImplPlan.md)
(v2) and resolves every `«SPECIALIZE: …»` marker in it for Rust. It also renders
the shared §2–§3 engine API concretely as idiomatic Rust. Read the shared plan
first; this document does not restate *what* must be built, only *how* it is
built in Rust. Pinned decisions are cited by number (D1–D17, shared §4).

Design reference: [`../../taut-shape/dev-docs/TautShapeArchitecture.md`](../../taut-shape/dev-docs/TautShapeArchitecture.md)
(§4 contract summary, §5 backing, §6 mailbox engine + shells, §8 extraction).
Extraction source: `glade/node/src/store.rs`, `glade/node/src/session.rs`,
`glade/wire-rs/src/cbor.rs`.

Guiding constraint (inherited): **getting the API right beats speed.** Phase 1 —
the pure mailbox engine — is the load-bearing decision. `taut-shape-rs` is the
*reference* implementation and the *oracle generator*, so its hand-written
surface (the `Input`/`Output` unions, engine, shell — the message *types*
themselves are generated, D17) is the one the Python and TS repos will mirror.
Treat §A (the rendered API) as the review target.

`taut-shape-rs` is special in one respect the shared plan calls out: it is the
**reference** impl and therefore owns `gen` (oracle emission). The other
language repos only run `check` / `node` / `client`.

v2 note: this revision replaces the v1 draft's "sans-io core + consumer `Store`
trait + `Notify` readiness seam + reader/writer split" with the shared plan's
pure message-queue engine (D1). The `Store` trait, `Notify`/`Notifier`,
`Reader`/`Writer`, and `on_stop` callback are **deleted**; their duties are now
messages or engine-internal state. §A.10 records the disposition of every v1
open question.

---

## 0. Specialization checklist (resolved for Rust)

| Marker (shared §7) | Rust commitment |
|---|---|
| Package manifest | a Cargo **workspace**: `taut-shape` (core, `no_std`+`alloc`), `taut-shape-tool` (CLI bin). Published crate name: `taut-shape`. |
| Build/test runner | `cargo test` (workspace), `cargo build -p taut-shape --no-default-features` for the `no_std` gate. |
| Golden framework | [`insta`](https://insta.rs) snapshots for derived surfaces (CLI transcripts, run reports); the committed oracle JSON is the **primary** golden (serde round-trip + whole-output equality, not insta). |
| Engine rendering | `LogNode` struct; `fn handle(&mut self, input: Input) -> Vec<Output>`; messages as two Rust **enums** (`Input`/`Output`) over plain data structs (the structs tautc-generated, D17). §A.2–A.4. |
| Generated message types | `tautc gen -l rust` from `taut-shape/ir/shape_log.taut.py` (D17), vendored as `crates/taut-shape/src/generated.rs` — the gwz-core `protocol/generated.rs` pattern (header-marked "do not edit", regenerated + re-vendored on schema bump, never hand-maintained). Lives **in the core crate**, so it must stay `no_std`+`alloc`-clean; the generated CBOR codec surface pairs with the vendored `cbor.rs` in the tool's framing layer (the core stays codec-free). Hand-written on top: only the `Input`/`Output` enums, engine, shell. |
| Shell & async idiom | feature `async`: `SharedNode` = `Arc<Mutex<LogNode>>` pump + per-stream `Waker` mailboxes + a `TimerHost` seam; sugar = `LogStream: futures_core::Stream<Item = Response>`; **cancellation = `Drop` → `EndStream`**. §A.6–A.7. |
| Service form | `LogService`: a `HashMap<LogId, LogNode>` router; `handle(Addressed<Input>) -> Vec<Addressed<Output>>`; unknown ids answered with a `failed` response carrying `ErrorCode::UnknownLog`. §A.8. |
| CBOR codec | reuse taut's `cbor.rs` runtime (`Cbor` enum + `encode`/`decode`), vendored to match `glade/wire-rs/src/cbor.rs` conventions. Not a dependency of the **core** crate — it lives in the tool's framing layer. |
| Source to mirror | `glade/node/src/{store,session}.rs` (distill, do not lift-and-shift): `store.rs` → the engine-internal **window**; `session.rs`/`client_heads` → the **session table**. This split is normative (arch §8 note). |
| Distribution | crates.io crate `taut-shape` (core); `taut-shape-tool` bin shipped in-repo, optionally published. |
| docs/ layout | `docs/api/` API specs (one per surface: messages, engine, shell, service, tool protocol); `docs/examples/` worked examples (doctest-checked). |

---

## A. The shared §2–§3 API, rendered in Rust (the part to get right)

This is the normative Rust surface. The engine (§A.1–A.5) is
`#![no_std]`-compatible (`alloc` only) — **no `core::task`, no `Waker`, no
clock, no locks anywhere in it** (D1). The mailbox model makes the `no_std`
story strictly easier than v1: `core::task` appears only in the shell (§A.6),
behind the `async` feature, which implies `std`. Signatures are the review
target — iterate here before Phase 1 calcifies them across three repos.

**Generated, not hand-authored (D17):** the message data types below (the
§A.1–A.3 structs — `Cursor`, `Record`, `Error`, `Response`, …) are declared
once in the payload-agnostic taut schema `taut-shape/ir/shape_log.taut.py`
(messages + codecs, no `service`) and produced by `tautc gen -l rust`, landing
exactly like gwz-core's `protocol/generated.rs` (vendored in S0.3,
header-marked, never hand-edited). The rendering below is therefore the
**expected shape of the generated code**, shown for review — divergence is
fixed in the schema, not by editing the output. Hand-written per D17: only the
`Input`/`Output` enums that wrap the generated messages, the engine, and the
shell.

### A.1 Core types (shared §3.1)

```rust
/// An ordered position in one log. "Records strictly after `seq` are unseen."
/// Named type (not a bare integer) so it can grow (`byte_offset` reserved,
/// shared §3.1) — v0 is seq-only.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub struct Cursor {
    pub seq: u64,
}

impl Cursor {
    /// D8: first record is `seq = 1`; `START = {seq: 0}`; empty log `head = 0`.
    pub const START: Cursor = Cursor { seq: 0 };
}

/// One appended record. Payload is opaque, binary-safe (NUL-safe) bytes —
/// specifically the method's append-type message **already taut-encoded** by
/// the producer (the glade `Op.payload` pattern, D17), never raw app bytes.
/// The engine treats it as opaque; that is how the log vocabulary stays
/// generic without generics. D11: a record carries its own `seq`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Record {
    pub seq: u64,
    pub payload: Bytes, // alias for Vec<u8> in v0 — open question R2
}

/// D13: the canonical state alphabet, one-to-one with the oracle strings.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Data,       // records returned; keep reading
    WouldBlock, // caught up, live: probe / timeout answer (D14)
    Eof,        // sealed and drained (D12)
    Closed,     // Close{} teardown, no error (D12)
    Failed,     // Close{error}; `error` attached to the response (D12)
    Expired,    // invalid cursor — a STATE, never an error (D9);
                // next_cursor = earliest resumable position
}

/// The error carrier attached to `failed` responses (D12) and to the service's
/// unknown-log answer. This is wire vocabulary, not a Rust error type — it is
/// never `?`-propagated (open question R3 on Display/Error impls).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Error {
    pub code: ErrorCode,
    pub message: Option<String>, // alloc::string::String
}

/// `UnknownLog` is **service-level only** (§A.8 / shared §3.5); `LogNode`
/// itself only ever attaches `ProducerError` / `Internal`. There is no
/// `canceled` code: client cancellation is `EndStream`, which has no response.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ErrorCode { UnknownLog, ProducerError, Internal }

/// One stream instance (D3): one logical read loop with its own position.
/// Minted by the *consumer* (the engine never allocates ids); wire form is a
/// string, so a cheaply-clonable shared str. Many per client; disposable —
/// position lives in the client-held cursor.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct StreamId(pub Arc<str>); // alloc::sync::Arc

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

pub type Bytes = alloc::vec::Vec<u8>;
```

### A.2 Input messages (shared §3.2)

```rust
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
```

### A.3 Output messages (shared §3.3)

```rust
/// The addressed read answer. `next_cursor` is ALWAYS present, even when
/// `records` is empty. A named struct (not inlined in the enum) so the shell
/// can route it by value.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Response {
    pub stream_id: StreamId,
    pub records: Vec<Record>,
    pub next_cursor: Cursor,
    pub state: State,
    pub error: Option<Error>, // attached iff state == Failed (D12)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Output {
    Response(Response),
    /// The engine's only "clock": the shell must arrange a
    /// `TimerExpired{token}` after ~`ms` (D14). Tokens monotonic from 1 (D16).
    SetTimer { token: TimerToken, ms: u64 },
    CancelTimer { token: TimerToken },
    /// Emitted on `Close`, and on the reader-count ≥1 → 0 transition when
    /// constructed with `StopWhen::LastReader` (D6). The shell routes this to
    /// the producer; a shell that initiated the close ignores it.
    ProducerStop { reason: StopReason },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopReason { LastReaderGone, Closed, Failed }
```

### A.4 The engine: `LogNode` (D1)

```rust
/// The pure mailbox endpoint: one per log. No I/O, no clock, no locks, no
/// callbacks. Held long-polls are ENGINE STATE — a tail `Read` that cannot be
/// answered is parked in the session table and answered when a later input
/// (`Push`/`Seal`/`Close`/`TimerExpired`/`EndStream`) releases it. There is no
/// readiness/notify seam (D1): readiness dissolved into message ordering.
/// Unsynchronized by design — the shell owns serialization (D15).
pub struct LogNode {
    window: window::Window,   // store core (crate-private, §A.5)
    sessions: session::Table, // session table (crate-private, §A.5)
    stop_when: StopWhen,      // D6
    next_timer: u64,          // monotonic token allocator, from 1 (D16)
}

/// Construction knob for `ProducerStop` (D6). A log never read must not
/// spuriously stop its producer, so the ≥1→0 transition is what fires.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopWhen { LastReader, ExplicitOnly }

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub stop_when: StopWhen, // Default: ExplicitOnly
}

impl LogNode {
    pub fn new(config: Config) -> Self;

    /// The whole API. Total — never panics on protocol-level misuse and never
    /// returns a Rust error: protocol outcomes are messages (`failed`
    /// responses, `expired` states — D9/D12), and invalid inputs (unknown
    /// timer token, unknown `EndStream` id) are defined as no-ops (shared
    /// §3.2). Outputs are a deterministic function of the input history
    /// (D16): timer tokens monotonic from 1; when one input releases several
    /// held reads, responses are emitted in stream-creation order.
    pub fn handle(&mut self, input: Input) -> Vec<Output>;

    // Read-only accessors (shared §2.1): in-process conveniences for shells
    // and producers. Never mutating, never part of the wire contract.
    pub fn head(&self) -> u64;                    // 0 when empty (D8)
    pub fn floor(&self) -> u64;                   // lowest retained seq; 0 = nothing evicted
    pub fn min_watermark(&self) -> Option<u64>;   // safe eviction floor (D7)
    pub fn stream_count(&self) -> usize;
}
```

**`Vec<Output>` over `impl Iterator<Item = Output> + '_`** (the rendered
choice; kept open as R1): the outputs must be materialized anyway — the golden
harness compares whole output sequences (shared §5), and the pump dispatches
all outputs under the lock before the next input. A borrowing iterator would
pin `&mut LogNode` until drained, which forbids storing the engine and its
pending outputs side by side in the pump and invites half-drained-state bugs.
Allocation cost is an explicit non-goal (shared §10), and `Vec` keeps `handle`
trivially FFI-able and object-safe.

**No `Result`, no `LogError`** (rationale): v1's `LogError` enum existed
because reads were function calls. In v2 every failure the protocol knows
about is *in-band* — `expired` is a state with a resumable cursor (D9),
producer failure is `failed` + `Error` (D12), unknown log is a service-level
response (§A.8). A Rust-level `Result` would create a second, out-of-band
error channel the oracle cannot see. The `Error` struct is wire data.

### A.5 Internal split: window + session table (D2, D3 — internal modules, not public seams)

The v1 `Store` **trait is gone** (D2): v0 bytes live in an engine-internal
bounded window, retention driven by the `Evict` input. The external-store
extension (`ScanRequest`/`ScanResult`, shared §9) is additive and not v0.
The split below is crate-private structure — normative architecture (arch §8
note: glade's `store.rs` vs `session.rs`/`client_heads` is exactly this
separation), but **not** public API.

```rust
// window.rs (pub(crate)) — the store core. Distilled from glade store.rs:
// the `scan(from)` resume discipline (no dup / no skip) and `heads` collapsed
// to a single origin. Glade's u32-len+CBOR record framing and truncated-tail
// handling are file-format concerns: they resurface in the TOOL's data-channel
// framing (S4.1), not in this in-memory window.
pub(crate) struct Window {
    records: VecDeque<Record>, // the bounded window (D2)
    head: u64,                 // highest assigned seq; 0 when empty (D8)
    floor: u64,                // lowest retained seq; 0 when nothing evicted
    lifecycle: Lifecycle,      // Live | Sealed | Closed | Failed(Error) (D12)
}

impl Window {
    pub(crate) fn push(&mut self, payload: Bytes) -> u64;      // assigns head+1
    /// Records with `seq > from`, in order, bounded by `limits` with the
    /// D10 forward-progress guarantee. Returns records + last returned seq.
    pub(crate) fn scan(&self, from: u64, limits: Limits) -> (Vec<Record>, u64);
    pub(crate) fn evict(&mut self, up_to_seq: u64);
    // head() / floor() / lifecycle() accessors …
}

// session.rs (pub(crate)) — the session table: one entry per live stream
// instance, the per-stream "response handler". Maps glade session.rs /
// client_heads. No byte/retention concepts here; no stream concepts in Window.
pub(crate) struct Table { /* entries + creation-order ranks */ }

pub(crate) struct Entry {
    created: u64,           // creation rank — D16 emission order on multi-wake
    held: Option<HeldRead>, // ≤1 outstanding per stream (D5)
    timer: Option<TimerToken>,
    watermark: u64,         // last delivered seq (D7)
}

pub(crate) struct HeldRead {
    cursor: Cursor,
    limits: Limits,
    // timeout already converted to a timer token (or none = hold forever)
}
```

`node.rs` orchestrates: read resolution (shared §3.4, rules 1–4) classifies a
`Read` against the window (`data` / `eof` / `closed` / `failed` / `expired`
with earliest-resumable `next_cursor` — D9) or parks it in the table; `Push`/
`Seal`/`Close`/`TimerExpired` sweep the table in creation order (D16) and emit
the released responses; `EndStream` drops the entry and, on the ≥1→0
transition under `StopWhen::LastReader`, emits `ProducerStop` (D6). Terminal
states remain re-readable (a later `Read` below head still returns `data`
until evicted) — terminal describes the log, not the stream.

### A.6 The shell: pump + timers (feature `async`, implies `std`)

The engine has no clock and no wakers, so the shell is where `std::sync` and
`core::task` first appear. It stays **executor-agnostic**: `std::sync::Mutex`
+ `core::task::Waker` + `futures-core` (for the `Stream` trait) — no `tokio`
in this crate, ever. The cross-runtime/PyO3 wakeup is explicitly *not* here
(arch §6: it is gwz's bridge).

```rust
/// The pump: ONE lock around the engine — the lock IS the serialization the
/// engine requires (D15). `send` feeds one input, then dispatches every
/// output before releasing: `Response` → the addressed stream's mailbox
/// (waking its parked Waker), `SetTimer`/`CancelTimer` → the TimerHost,
/// `ProducerStop` → the stop signal. Cheap to clone (Arc).
pub struct SharedNode { inner: Arc<Mutex<Inner>> }

struct Inner {
    node: LogNode,
    mailboxes: HashMap<StreamId, Mailbox>, // parked Waker + delivered Response
    timers: Box<dyn TimerHost>,
    stop: StopSlot,                        // latched StopReason + Wakers
}

impl SharedNode {
    pub fn new(config: Config, timers: impl TimerHost) -> Self;

    /// The raw message door — everything below is sugar over it.
    pub fn send(&self, input: Input);

    // Producer-side sugar (node-local inputs):
    pub fn push(&self, payload: Bytes);
    pub fn seal(&self);
    pub fn close(&self, error: Option<Error>);
    pub fn evict(&self, up_to_seq: u64);

    /// One-shot probe: a `Read` with `timeout_ms = Some(0)` is answered inside
    /// the same `handle` call (D14), so this is synchronous — the pump plucks
    /// the addressed `Response` from the returned outputs.
    pub fn read_now(&self, cursor: Cursor, limits: Limits) -> Response;

    /// Idiomatic streaming sugar (shared S3.2) — §A.7.
    pub fn stream(&self, from: Cursor, limits: Limits) -> LogStream;

    /// Resolves when the engine emits `ProducerStop` (D6). The producer task
    /// selects on this; a shell that initiated the close just drops it.
    pub fn stopped(&self) -> StopSignal; // impl Future<Output = StopReason>
}

/// The only clock in the system, supplied by the embedder. `set` must arrange
/// `shared.send(Input::TimerExpired { token })` after ~`ms`; `cancel` is
/// best-effort — late expiries are engine no-ops (shared §3.2), so a sloppy
/// host is safe by construction. A no-op host is valid when every read uses
/// `timeout_ms` `None`/`0` (the common tail + probe paths need no timer).
pub trait TimerHost: Send + 'static {
    fn set(&self, token: TimerToken, ms: u64);
    fn cancel(&self, token: TimerToken);
}
```

### A.7 Streaming sugar: `LogStream` (cancellation → `EndStream`)

```rust
/// `futures_core::Stream<Item = Response>` over one stream instance.
/// Each cycle: issue `Read { stream_id, cursor, timeout_ms: None }` (hold
/// indefinitely — D14), park the task's Waker in the mailbox, yield the
/// addressed `Response` when the pump delivers it, advance
/// `cursor = next_cursor`. Terminal states (`Eof`/`Closed`/`Failed`) are
/// yielded once, then the stream ends; `Expired` is yielded as a normal item
/// (it is a state with a resumable cursor — D9) and the caller decides
/// whether to continue lossy from `next_cursor`.
pub struct LogStream {
    shared: SharedNode,
    stream_id: StreamId, // minted by the shell (uuid-ish); engine never mints ids
    cursor: Cursor,
    limits: Limits,
    done: bool,
}

impl futures_core::Stream for LogStream { type Item = Response; /* … */ }

/// Idiomatic Rust cancellation is dropping the future/stream. Drop sends
/// `EndStream { stream_id }` (D4): held read dropped unanswered, timer
/// canceled, watermark removed — and, under `StopWhen::LastReader`, the last
/// drop triggers `ProducerStop` (D6). No bespoke cancel token.
impl Drop for LogStream { /* self.shared.send(Input::EndStream { .. }) */ }
```

A lost/leaked stream costs only its held poll: position lives in the
client-held cursor (D3), so recovery is a fresh `stream_id` + the old cursor.

### A.8 The service layer: `LogService` (shared §3.5)

Thin, and engine-flavored itself (pure, sync, message-in/messages-out) so the
same pump pattern wraps it for the tool's `node` mode.

```rust
/// Opaque log handle minted by the producing call. Cheap to clone.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct LogId(pub Arc<str>);

/// Service-level vocabulary = node message + log_id (the taut companion
/// messages are this form on the wire).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Addressed<T> { pub log_id: LogId, pub msg: T }

pub struct LogService { nodes: HashMap<LogId, LogNode>, /* id mint */ }

impl LogService {
    pub fn create(&mut self, config: Config) -> LogId;
    pub fn remove(&mut self, log_id: &LogId) -> bool;

    /// Route by `log_id`. Unknown id + a `Read` ⇒ a `failed` `Response` with
    /// `ErrorCode::UnknownLog` (the only place that code is minted); unknown
    /// id + anything else ⇒ no-op. Outputs carry the `log_id` back, so the
    /// shell's timer key is `(log_id, token)` — tokens are per-node (D16).
    pub fn handle(&mut self, input: Addressed<Input>) -> Vec<Addressed<Output>>;

    /// Producer-local access (v0: the producer lives with the node).
    pub fn node(&mut self, log_id: &LogId) -> Option<&mut LogNode>;
}
```

### A.9 Idiom rationale (summary)

- **Two enums over trait objects / many structs**: `Input`/`Output` make
  `handle` a total `match`, exhaustiveness-checked by the compiler each time
  the vocabulary grows (e.g. the §9 `ScanRequest`/`ScanResult` extension), and
  serde/CBOR-mappable one-to-one with the oracle vectors.
- **`no_std` + `alloc` core**: gwz-core can embed the engine; the mailbox
  model needs no `Waker`, clock, or `std::sync` in the core, so the `no_std`
  gate now costs nothing and enforces D1 mechanically.
- **States in `Response`, never `Result`**: matches the oracle exactly; a
  consumer loop is `match response.state`, no `?`-on-`would_block`
  anti-pattern; wire errors stay in-band (D9, D12).
- **`Mutex` pump, executor-agnostic shell**: the engine demands external
  serialization (D15); one `std::sync::Mutex` is the honest rendering. The
  shell depends only on `core::task` + `futures-core`, so it runs under tokio,
  smol, async-std, or a hand-rolled poll loop.
- **`Drop` ⇒ `EndStream`**: idiomatic Rust cancellation, and exactly the
  message an adapter injects on transport death (D4) — one cleanup path.
- **`TimerHost` as the shell's only side-effect seam**: timers are the one
  thing a pump cannot do with a lock alone; late/canceled expiries being
  engine no-ops makes the trait contract forgiving.

### A.10 v1 open questions — disposition under v2

| v1 | Disposition |
|---|---|
| Q1 — `Cursor` width & shape | **Resolved by shared §3.1**: named type, `seq`-only in v0, `byte_offset` reserved. Rust rendering: plain `Copy` struct (no `#[non_exhaustive]` awkwardness); pre-1.0 semver covers later growth. |
| Q2 — `seq` start 0 or 1 | **Resolved by D8**: first record `seq = 1`; `START = {seq: 0}`; empty log `head = 0`. |
| Q3 — `Bytes` owned vs `Cow` | **Still open** — carried forward as R2. |
| Q4 — where `sealed`/`closed` live | **Resolved by D1/D2**: lifecycle is engine-internal state (`Window.lifecycle`). There is no reader/writer split left to keep coherent — the `Lifecycle` sharing question dissolved with it. |
| Q5 — `dyn Store` escape hatch | **Moot**: the `Store` trait is deleted (D2). The external store is the additive message pair of shared §9, not a trait. |
| Q6 — `Notify: Send` vs `Send + Sync` | **Moot**: `Notify`/`Notifier` are deleted (D1). Wakers exist only in the shell's mailboxes, which the pump's `Mutex` already governs. |
| Q7 — `evict` policy ownership | **Resolved by D7**: retention is consumer-driven (`Evict` input); the engine only tracks per-stream watermarks and exposes `min_watermark()`. |

### A.11 Open Rust-local questions (v2)

- **R1 — `handle` return: `Vec<Output>` vs `impl Iterator`.** Rendered as
  `Vec` (§A.4 rationale); confirm at Phase 1 review. A `SmallVec` is a later
  non-breaking swap if profiling ever cares (non-goal).
- **R2 — payload `Bytes`: owned `Vec<u8>` vs `Cow`/`bytes::Bytes`.** Owned is
  simplest and `Send`; a zero-copy consumer (gwz cache replay) may want
  `bytes::Bytes`. Records now round-trip engine-internally (window → response
  clone), which strengthens the case for a cheap-clone `Bytes` — but that adds
  a dependency to the core. Decide at Phase 1 review.
- **R3 — `Error` trait impls & `thiserror` gating.** v2's `Error` is wire
  data, not a `?`-propagated Rust error, so `thiserror` may be unnecessary
  altogether; if we do impl `Display`/`core::error::Error` for ergonomics,
  gate `thiserror` behind `std` and hand-write the `no_std` impls.
- **R4 — `TimerHost`: `Box<dyn TimerHost>` field vs generic parameter on
  `SharedNode`.** `dyn` keeps `SharedNode` un-generic (nicer for consumers
  holding many); a generic is zero-cost. Leaning `dyn` — the host is called
  at most once per held read.
- **R5 — id representations.** `StreamId`/`LogId` as `Arc<str>` (rendered) vs
  `String` vs interned `u64` with a wire-side mapping. `Arc<str>` matches the
  wire STR form with cheap clones; confirm against the CBOR codec's string
  handling.

---

## Phases

Same phase names as the shared plan (§6). Steps are Rust-specific,
foundational-first, parallel-friendly, each an aspirational **< 500 LOC** goal.

### Phase 0 — Scaffold & contract intake

- **S0.1 — Workspace skeleton.** Root `Cargo.toml` workspace with members
  `crates/taut-shape` (lib, `no_std`+`alloc`, `name = "taut-shape"`) and
  `crates/taut-shape-tool` (bin). `README.md`, `LICENSE`, `docs/api/`,
  `docs/examples/`, `tests/`. Core crate features declared up front:
  `default = ["std"]`, `async` (implies `std`, adds `futures-core`), and a
  bare `no_std` config. *(manifest, repo structure)*
- **S0.2 — Test + golden harness.** Wire `cargo test`; add `insta` as a
  dev-dependency for derived-surface snapshots; add the `no_std` build gate
  (`cargo build -p taut-shape --no-default-features`) to CI. GitHub Actions:
  `fmt`, `clippy -D warnings`, `test`, the `no_std` gate. *(runner, golden)*
- **S0.3 — Contract intake.** Generate the shape_log message types (D17): run
  `tautc gen -l rust` on `taut-shape/ir/shape_log.taut.py` and vendor the
  output as `crates/taut-shape/src/generated.rs` — the gwz-core
  `protocol/generated.rs` pattern (header-marked, never hand-edited; record
  the schema path + commit tracked). It must pass the core crate's `no_std`
  gate; its generated codec surface is vendored into the tool crate beside
  `cbor.rs`, keeping the core codec-free. Vendor taut's `cbor.rs` runtime into
  `crates/taut-shape-tool/src/cbor.rs` (framing/tool layer, not the core),
  recording the exact taut path + commit it tracks. Pin the oracle corpus
  location and schema version from `taut-shape`. Pin the tool wire-protocol
  (length-prefixed CBOR data + OOB JSONL control) in
  `docs/api/tool-protocol.md`. *(cbor source, generated types)*

### Phase 1 — The engine (foundational; the API to get right)

Distill from `glade/node/src/store.rs` + `session.rs`. **Reuse:** the
`scan(from)` resume discipline (no dup / no skip), `head`/`heads` collapsed to
a single origin, and the store-vs-session structural split (normative — arch
§8 note). Glade's `u32-len + CBOR` framing and truncated-tail handling move to
the tool's data channel (S4.1). **Drop:** `origin`/`seq`-vector/`lamport`/
`prev`-hash/`refs`/fold — the resume *vector* collapses to a scalar `Cursor`;
drop equivocation/chain-break (single origin, single writer). **Add:** the
finite-log lifecycle glade lacks — seal/`eof`, `close`/`failed`, held reads +
timers, bounded window + `Evict`/floor/watermarks, `ProducerStop`.

- **S1.1 — Messages + core types.** The data structs (`Cursor`, `Record`,
  `State`, `Error`, `ErrorCode`, `StreamId`, `TimerToken`, `Limits`,
  `Response`, `StopReason`) come from the S0.3-vendored `generated.rs` (D17) —
  verify they match the expected §A.1–A.3 shape; divergence is fixed in the
  schema, not the output. Hand-write only the `Input`/`Output` enums wrapping
  them, with D-numbers committed in doc comments (D8, D11, D13 in particular).
  No logic; the API surface.
- **S1.2 — Store core (window).** `window.rs`: `push` append (head+1), `scan`
  honoring `Limits` with the D10 forward-progress rule, `evict`/`floor`,
  `head`, the `Lifecycle` enum. Crate-private.
- **S1.3 — Session table + read resolution.** `session.rs` + the `node.rs`
  resolution of shared §3.4: implicit create (D4), held reads, supersede (D5),
  `EndStream`, `expired` + earliest-resumable `next_cursor` (D9), watermarks
  (D7), reader-count and `ProducerStop`/`StopWhen` (D6). The heart;
  golden-tested in Phase 2.
- **S1.4 — Lifecycle + timers.** `Seal`/`Close{error?}` and the
  `closed`/`failed` split (D12); `SetTimer`/`CancelTimer`/`TimerExpired` and
  `timeout_ms` semantics (D14); monotonic tokens + creation-order emission
  (D16). Plus the read-only accessors (§A.4).

### Phase 2 — Golden conformance

- **S2.1 — Oracle-vector loader.** A serde model for the oracle JSON — each
  vector a pure `(input message sequence) → (expected output message
  sequence)` pair (shared §5) — and a loader reading the pinned corpus from
  `taut-shape`. Lives in the tool crate's `oracle.rs`, reused by the tests.
- **S2.2 — Replay through the engine.** A runner that feeds each vector's
  inputs to a fresh `LogNode` (or `LogService` for multi-log vectors) and
  compares the **whole observed output sequence** structurally against the
  expected — never per-field assertions. The oracle JSON is the primary golden
  (direct equality); `insta` snapshots the human-readable run report for
  review. *(framework: oracle JSON + insta)*
- **S2.3 — Lockstep gate.** `gen` (Phase 4) writes the corpus; a CI test
  asserts the committed corpus in `taut-shape` reproduces byte-stable through
  this engine, so a corpus bump is caught. (`taut-shape-rs` is the generator,
  so this gate also guards every other language repo.)

### Phase 3 — Shell & idiomatic stream surface

- **S3.1 — The pump.** Behind the `async` feature: `SharedNode` =
  `Arc<Mutex<Inner>>` (the lock is the D15 serialization); output dispatch
  (mailbox routing + Waker wake, `TimerHost` calls, `StopSignal` latch);
  `send` + producer sugar + `read_now`. Resolves R4. *(loop/lock form)*
- **S3.2 — Streaming sugar + cancellation.** `LogStream:
  futures_core::Stream<Item = Response>`: issue `Read` (no timeout) → park →
  yield the addressed `Response` → advance cursor → repeat; terminal states
  end the stream; `Drop` sends `EndStream` (D4) → last-reader `ProducerStop`
  (D6). Plus `stopped()` for the producer side. *(async idiom)*

### Phase 4 — Conformance / interop CLI tool

`crates/taut-shape-tool`, depending on the core (`async`) and the vendored
`cbor.rs`. Follows taut's `test_kotlin.py` convention: a Python driver spawns
this bin and asserts on structured output. The tool is little more than the
engine + framing — by design.

- **S4.1 — Framing.** **Data channel** = length-prefixed CBOR taut companion
  messages (`u32-LE len + CBOR body` — the framing pattern distilled from
  glade `store.rs`, including truncated-tail tolerance) over stdin/stdout.
  **Control/result channel** = OOB JSONL (scenario in, observed transcript
  out) on a separate descriptor (stderr or fd 3).
- **S4.2 — Modes.** `gen` (reference-only: emit oracle vectors from scripted
  scenarios), `check` (replay the committed oracle through the engine, report
  pass/fail), `node` (run `LogService` + pump behind the framing), `client`
  (run the reading side — the cursor loop — against a node). A thin hand-rolled
  arg layer in the bin crate only; the core stays CLI-free.

### Phase 5 — Interop matrix participation

- **S5.1 — Matrix pairings.** Make `taut-shape-tool node` / `client` drivable
  by `taut-shape`'s Python interop matrix; pass every `node(rs) ⊗ client(Y)`
  and `node(X) ⊗ client(rs)` pairing. No new Rust surface — wiring + docs + a
  CI job that invokes the matrix.

### Phase 6 — `stream` shape (deferred)

After `log` lands across all languages. Not planned here.

---

## Repository structure (nominated)

```
taut-shape-rs/
├─ Cargo.toml                      # [workspace]; members below
├─ README.md                       # what/why, quickstart, oracle/conformance pointer
├─ LICENSE
├─ crates/
│  ├─ taut-shape/                  # core: no_std + alloc engine; std shell behind `async`
│  │  ├─ Cargo.toml                # name = "taut-shape"; features: std, async
│  │  └─ src/
│  │     ├─ lib.rs                 # #![no_std]; pub re-exports
│  │     ├─ generated.rs           # tautc-generated shape_log message types (D17) — vendored, do NOT hand-maintain
│  │     ├─ types.rs               # thin re-exports over generated.rs: Cursor, Record, State… (A.1)
│  │     ├─ msg.rs                 # hand-written Input/Output unions over generated messages (A.2–A.3)
│  │     ├─ node.rs                # LogNode: handle + read resolution + accessors (A.4)
│  │     ├─ window.rs              # pub(crate) store core: window/head/floor/lifecycle (A.5)
│  │     ├─ session.rs             # pub(crate) session table: held reads/timers/watermarks (A.5)
│  │     ├─ shell.rs               # SharedNode, TimerHost, LogStream, StopSignal (feature async)
│  │     └─ service.rs             # LogService, LogId, Addressed<T> (A.8)
│  └─ taut-shape-tool/             # the conformance/interop CLI bin
│     ├─ Cargo.toml
│     └─ src/
│        ├─ main.rs                # gen/check/node/client dispatch
│        ├─ cbor.rs                # vendored from taut (records the source path)
│        ├─ framing.rs             # u32-len+CBOR data; OOB JSONL control
│        ├─ oracle.rs              # serde vector model + loader (shared w/ tests)
│        ├─ gen.rs                 # reference-only oracle emission
│        ├─ check.rs               # replay committed oracle, pass/fail
│        ├─ node.rs                # LogService + pump behind framing
│        └─ client.rs              # the cursor loop (reading side)
├─ docs/
│  ├─ api/                         # see "docs/ deliverables"
│  └─ examples/                    # worked examples (doctest-checked)
├─ tests/
│  ├─ oracle.rs                    # S2.2 whole-output replay vs committed corpus (primary golden)
│  ├─ snapshots/                   # insta .snap files (human-readable run reports)
│  └─ tool_cli.rs                  # spawns taut-shape-tool, asserts framing/modes
└─ dev-docs/
   └─ InitialPlan.md               # this file
```

## docs/ deliverables

**`docs/api/`** (one spec per surface; the rendered §A is the source):
- `messages.md` — the full `Input`/`Output` vocabulary (§A.1–A.3) with the
  D-number for every pinned behavior (D8 seq origin, D9 expired, D10 limits,
  D11 Record.seq, D12 closed/failed, D13 states, D14 timeout, D17 types
  generated from `shape_log.taut.py` + payload = taut-encoded append message).
- `engine.md` — `LogNode`, the `handle` contract (pure, total, deterministic —
  D1/D16), the read-resolution table (shared §3.4), held-read/supersede/
  `EndStream` semantics (D4/D5), `StopWhen`/`ProducerStop` (D6), watermarks +
  `Evict` (D7), and the internal window/session split (informative).
- `shell.md` — `SharedNode` pump (the D15 lock), `TimerHost`, `LogStream`,
  cancellation-via-`Drop` → `EndStream`, `StopSignal`, and the explicit
  non-goals (no `tokio`, no PyO3, no cross-runtime wakeup — arch §6).
- `service.md` — `LogService` routing, `LogId` minting, `unknown_log`, the
  `(log_id, token)` timer key, and how service messages map to the taut
  companion wire schema.
- `tool-protocol.md` — the CLI wire contract (length-prefixed CBOR data + OOB
  JSONL control), the four modes (`gen`/`check`/`node`/`client`), and the
  `test_kotlin.py`-style driver convention.

**`docs/examples/`** (each a small, doctest-checked program):
- `01_push_and_read.rs` — raw engine: `Push` ×3, `Read` from `START` → `data`;
  `Seal`; `Read` at head → `eof`.
- `02_resume_from_cursor.rs` — read half, resume from `next_cursor`, show
  no dup / no skip.
- `03_hold_and_release.rs` — probe (`timeout_ms = 0`) → `would_block`; then a
  held `Read` (no timeout) answered by the outputs of a later `Push` — the
  mailbox model in one screen.
- `04_cancel_stream.rs` — shell: `stop_when = LastReader`; drop a `LogStream`
  mid-tail → `EndStream` → `ProducerStop(LastReaderGone)` observed via
  `stopped()`.
- `05_evict_expired.rs` — `Evict`, read below the floor → `expired` state with
  the earliest-resumable `next_cursor` (D9), continue lossy from there.
- `06_two_streams.rs` — two held streams woken by one `Push`; responses in
  stream-creation order (D16); `EndStream` one, the other keeps tailing.

## Non-goals (inherited)

Production transport/framing beyond the conformance tool; the external-store
extension (shared §9) until a real consumer needs it; `swmr`/`snapshot_delta`/
`crdt` shapes; any cross-runtime/PyO3 bridging (that is gwz's bridge);
performance work before there is profiling evidence.
