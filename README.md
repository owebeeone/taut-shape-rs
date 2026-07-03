# taut-shape-rs

The **reference** Rust implementation of the Taut `log` delivery-shape — a pure
**mailbox engine** (`LogNode`) plus the conformance/interop tooling that keeps
every `taut-shape-<lang>` honest. `taut-shape-rs` is special: it owns oracle
emission (`gen`), so its hand-written surface is the one the other language
repos mirror.

## Layout

- `crates/taut-shape` — the core library. `#![no_std]` + `alloc`; the engine has
  no clock, no locks, no wakers (D1). Default feature `std`; the shell lands
  behind the `async` feature (implies `std`) in a later phase.
- `crates/taut-shape-tool` — the conformance/interop CLI (`gen`/`check`/`node`/
  `client`). Phase 0 stub today.

The message types in `crates/taut-shape/src/generated.rs` and the CBOR runtime
in `crates/taut-shape/src/cbor.rs` are **vendored/generated** — do not
hand-edit; see each file's header for the source + regen command.

## Plans (the contract — read these first)

- Shared, language-neutral: `../taut-shape/dev-docs/TautClientImplPlan.md`
  (§2 engine model, §3 message vocabulary + §3.4 read resolution, §4 pinned
  decisions D1–D17).
- Rust-specific rendering: [`dev-docs/InitialPlan.md`](dev-docs/InitialPlan.md)
  (§A rendered API, phases, repo structure, and the Deviations note).

## Build / test

```sh
# Core library (default features: std).
cargo build -p taut-shape

# The no_std gate — the core must compile with alloc only.
cargo build -p taut-shape --no-default-features

# The conformance/interop tool (Phase 0 stub: prints usage, exits 2).
cargo build -p taut-shape-tool

# Whole workspace.
cargo build
cargo test
```

## Status

Phase 0 (scaffold & contract intake). The engine, golden conformance, shell,
and CLI modes are not implemented yet — see `dev-docs/InitialPlan.md` phases.
