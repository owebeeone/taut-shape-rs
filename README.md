# taut-shape-rs

The Rust implementation of the Taut `atom`, `log`, `stream`, and `value` delivery shapes — pure
mailbox engines (`AtomNode`, `LogNode`, `StreamNode`, `ValueNode`) plus conformance/interop tooling that keeps
every `taut-shape-<lang>` honest. Its hand-written engine surface is the one
the other language repos mirror; oracle emission itself is owned by the
canonical `taut-shape` repo's Python generator (`corpus/gen.py`), not by this
repo's CLI.

## Layout

- `crates/taut-shape` — the core library. `#![no_std]` + `alloc`; the engine has
  no clock, no locks, no wakers (D1). Default feature `std`; the shell lands
  behind the `async` feature (implies `std`) in a later phase.
- `crates/taut-shape-tool` — the conformance/interop CLI (`node`/`client`/
  `gen`/`check`). The `node`/`client` modes drive the selected engine over the shared
  length-prefixed, tagged-CBOR stdin/stdout framing (`src/framing.rs`, the
  cross-language reference) and are what the interop matrix runs. `gen`/`check`
  are **unimplemented stubs** (each prints what it would do and exits 2, see
  `crates/taut-shape-tool/src/main.rs`) — oracle emission/verification is done
  by the canonical repo's generator, not by this CLI.

Both `node` and `client` accept `--shape <NAME>` (default: `log`). The exact
implemented engine registry contains `atom`, `log`, `stream`, and `value`; another name exits
2 with `TAUT_SHAPE_UNSUPPORTED_SHAPE` before the process reads or writes a data
frame.

The message types in `crates/taut-shape/src/generated.rs` and
`generated_atom.rs`, `generated_stream.rs`, `generated_value.rs`, and the CBOR runtime
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

# The conformance/interop tool (gen/check/node/client modes).
cargo build -p taut-shape-tool

# Whole workspace.
cargo build
cargo test
```

## Status

The engines (`LogNode`, unit-tested against the D1–D19 rules, the attributed
LWW `ValueNode`, the latest-state `AtomNode`, and the bounded live `StreamNode`) and the `node`/
`client` interop CLI modes are implemented and green (workspace `cargo test`,
`clippy`, the `--no-default-features` no_std gate, plus the cross-language
interop matrix in `../taut-shape/matrix/`). The CLI's `gen`/`check` modes are
still exit-2 stubs — not implemented. The async shell (`async` feature)
remains a later phase — see `dev-docs/InitialPlan.md`.
