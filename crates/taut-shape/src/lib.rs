//! `taut-shape` — the reference Rust implementation of the Taut `log`
//! delivery-shape mailbox engine.
//!
//! This is Phase 0 scaffolding. The engine (`LogNode`), session/window split,
//! shell, and service layer land in later phases; see
//! [`dev-docs/InitialPlan.md`] and the shared
//! [`taut-shape/dev-docs/TautClientImplPlan.md`] for the contract.
//!
//! The core is `#![no_std]` + `alloc` (D1: the mailbox engine has no clock, no
//! locks, no wakers). The default `std` feature exists for the shell (Phase 3,
//! behind the `async` feature) and for `std`-only ergonomics; the `no_std` gate
//! is `cargo build -p taut-shape --no-default-features`.
//!
//! [`dev-docs/InitialPlan.md`]: https://example.invalid/InitialPlan.md
//! [`taut-shape/dev-docs/TautClientImplPlan.md`]: https://example.invalid/TautClientImplPlan.md
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// Vendored deterministic-CBOR runtime (see the file header for provenance +
/// the recorded plan deviation). Public so the tool crate's framing layer can
/// use the same `Cbor` type the generated codec speaks.
pub mod cbor;

/// tautc-generated `shape_log` message types + CBOR codec (D17). Vendored,
/// header-marked "do not edit", regenerated on schema bump.
///
/// Lints are suppressed here (at the declaration, not in the file body) so the
/// vendored output stays byte-identical to `tautc` while still passing a
/// `clippy -D warnings` CI gate: `unused_variables` for empty generated
/// messages whose `from_cbor(c)` ignores its `Cbor` argument (e.g. `LogSeal`),
/// `clippy::redundant_closure` for the generator's `|x| T::from_cbor(x)`
/// mapping style, and `clippy::needless_question_mark` for the fail-closed
/// codec's `|x| Ok(T::from_cbor(x)?)` list/map element mapping (the generic
/// `Ok(..?)` wrapper is uniform across scalar and message elements).
#[allow(
    unused_variables,
    clippy::redundant_closure,
    clippy::needless_question_mark
)]
pub mod generated;

// The generated `shape_log` message vocabulary (D17) stays namespaced under
// `generated::` (NOT re-exported flat) so the hand-written §A.1 core types in
// [`types`] can own the short names (`Cursor`, `Record`, `State`, …) without
// colliding with the generated `Log*`-prefixed structs. (Intentionally not
// `pub use generated::*`.)

// ── Phase 1: the `log` shape engine (§A.1–A.5) ──────────────────────────────

/// The `log` delivery-shape mailbox engine (D23): core types (§A.1), the
/// `Input`/`Output`/`Response` unions (§A.2–A.3), the session/window split, and
/// the `LogNode` engine. A future second shape becomes a sibling of this module
/// while `cbor` and `generated` stay shared at the crate root.
pub mod log;

// Flat re-exports keep the crate's public API stable across the D23 `log/`
// module boundary: downstream code (including the tool crate) sees these names
// at the crate root exactly as before.
pub use log::{
    Bytes, Config, Cursor, Error, ErrorCode, Input, Limits, LogNode, Output, Record, Response,
    State, StopReason, StopWhen, StreamId, TimerToken,
};
