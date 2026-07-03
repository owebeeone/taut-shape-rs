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
/// and `clippy::redundant_closure` for the generator's `|x| T::from_cbor(x)`
/// mapping style.
#[allow(unused_variables, clippy::redundant_closure)]
pub mod generated;

// The generated `shape_log` message vocabulary (D17) stays namespaced under
// `generated::` (NOT re-exported flat) so the hand-written §A.1 core types in
// [`types`] can own the short names (`Cursor`, `Record`, `State`, …) without
// colliding with the generated `Log*`-prefixed structs. (Intentionally not
// `pub use generated::*`.)

// ── Phase 1: the engine (§A.1–A.5) ──────────────────────────────────────────

/// Core data types (§A.1): `Cursor`, `Record`, `State`, `Error`, `ErrorCode`,
/// `StreamId`, `TimerToken`, `Limits`, `StopReason`, `Bytes`.
pub mod types;

/// The hand-written `Input`/`Output` unions + `Response` (§A.2–A.3, D17).
pub mod msg;

mod node;
mod session;
mod window;

pub use msg::{Input, Output, Response};
pub use node::{Config, LogNode, StopWhen};
pub use types::{
    Bytes, Cursor, Error, ErrorCode, Limits, Record, State, StopReason, StreamId, TimerToken,
};

#[cfg(test)]
mod tests;
