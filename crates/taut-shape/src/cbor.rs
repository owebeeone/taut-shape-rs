//! Compatibility facade: the actual runtime is maintained once in base taut.
// Source : taut/src/taut/gen/runtime/cbor_fail_closed.rs at taut v0.10.0.
// Regenerate the foundation crate with taut/rust/sync_foundation.py.
// This re-export preserves Cbor, DecodeError and descriptor nominal identity.
pub use taut_foundation::cbor::*;
