//! The `log` delivery-shape mailbox engine (§A.1–A.5).
//!
//! Everything specific to the `log` shape lives under this module: the core
//! data types (§A.1), the hand-written `Input`/`Output`/`Response` unions
//! (§A.2–A.3), the session/window split, and the `LogNode` engine itself.
//!
//! The shared, cross-shape pieces stay at the crate root: the deterministic
//! CBOR runtime ([`crate::cbor`]) and the tautc-generated `shape_log` message
//! vocabulary ([`crate::generated`]). A future second shape reuses those two
//! and adds its own sibling of this module.
//!
//! The crate's public API re-exports these names flat at the root (see
//! `lib.rs`), so downstream code — including the tool crate — is unaffected by
//! this module boundary.

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
