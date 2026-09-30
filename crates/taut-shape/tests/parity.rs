//! taut's codec-parity rows, replayed against the runtime this crate ships,
//! `taut_shape::cbor` (TautCheckedDecode.md CD-C3, CD-C4).
//!
//! taut's gate (`tautc parity`) replays taut's own sources, so it cannot see a
//! vendored copy drift. This replays the rows against the copy. They are
//! `src/parity_vectors.rs`, written by `scripts/gen_parity_vectors.py` from the
//! taut tag `cbor.rs` was vendored from. Each is judged as the gate
//! (`taut/src/taut/corpus/parity.py`) judges its `rust` target:
//!
//! * The runtime's depth constants equal the bounds header's (`#constants`).
//! * A `raw_decode` row's bytes expand to its `len`, where it states one. They are
//!   decoded as taut's Rust runner decodes them: `try_decode_with` with the row's
//!   `limits`, else `try_decode`. An accept row must decode and re-encode to its
//!   `reencode`, else to its own bytes. Any other row must be refused with its tag,
//!   and each payload field it names must match as a string, except those the gate
//!   exempts for Rust. A panic is untyped, which fails.
//! * A round-trip int row's values fit the `i64` carrier, and its bytes decode and
//!   re-encode unchanged. An encode-fail int row's value does not fit `i64`, so the
//!   carrier cannot express it ("type-satisfied", as taut's Rust runner reports it).
//!
//! A `from_cbor` or `from_wire` row decodes through a message or enum of taut's
//! parity fixture, whose generated code this crate does not ship. The gate replays
//! those rows against the same runtime source. Here they are counted, not replayed.

#[path = "../src/parity_vectors.rs"]
mod parity_vectors;

use std::fmt::Write as _;
use std::panic;

use parity_vectors::{
    DecodeRow, Expect, Row, CONSTANTS, DECODE_ROWS, ENCODE_FAIL, PAYLOAD_EXEMPT, ROUND_TRIP,
};
use taut_shape::cbor::{self, Cbor, DecodeError};

fn unhex(hex: &str) -> Vec<u8> {
    let pairs = hex.as_bytes().chunks_exact(2);
    assert!(pairs.remainder().is_empty(), "odd-length hex {hex}");
    pairs
        .map(|pair| {
            let digits = std::str::from_utf8(pair).expect("hex is ASCII");
            u8::from_str_radix(digits, 16).unwrap_or_else(|_| panic!("bad hex {hex}"))
        })
        .collect()
}

fn hexof(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut out, byte| {
        let _ = write!(out, "{byte:02x}");
        out
    })
}

/// A row's input: each segment's bytes, `count` times, in order.
fn expand(row: &Row) -> Vec<u8> {
    let mut out = Vec::new();
    for (hex, count) in row.segments {
        let piece = unhex(hex);
        for _ in 0..*count {
            out.extend_from_slice(&piece);
        }
    }
    out
}

/// The raw decode, as taut's Rust runner calls it: with `limits`, `try_decode_with`
/// them, at the default depth where they give none; without, `try_decode`.
fn raw(row: &Row, bytes: &[u8]) -> Result<Cbor, DecodeError> {
    if row.max_depth.is_none() && row.max_encoded_len.is_none() {
        return cbor::try_decode(bytes);
    }
    let depth = row.max_depth.unwrap_or(cbor::DEFAULT_MAX_DEPTH);
    cbor::try_decode_with(bytes, depth, row.max_encoded_len)
}

/// An `err` detail, as taut's Rust runner reports it: the canonical tag, then
/// `;field=value` for each payload field the error carries. Exhaustive on purpose:
/// a re-vendored runtime with a new variant fails the build until it is reported.
fn describe(e: &DecodeError) -> String {
    let payload = match e {
        DecodeError::Truncated
        | DecodeError::TrailingBytes
        | DecodeError::InvalidUtf8
        | DecodeError::NonIntegerMapKey
        | DecodeError::IntOverflow => String::new(),
        DecodeError::UnsupportedInfo(info) => format!(";info={info}"),
        DecodeError::UnsupportedMajor(major) => format!(";major={major}"),
        DecodeError::DuplicateMapKey(key) => format!(";key={key}"),
        DecodeError::NonCanonicalInt(value) => format!(";value={value}"),
        DecodeError::NegativeMapKey(key) => format!(";key={key}"),
        DecodeError::MissingKey(key) => format!(";key={key}"),
        DecodeError::WrongType { expected } => format!(";expected={expected}"),
        DecodeError::UnknownEnum { enum_name, value } => {
            format!(";enum={enum_name};value={value}")
        }
        DecodeError::TooDeep { limit } => format!(";limit={limit}"),
        DecodeError::TooLarge { len, limit } => format!(";len={len};limit={limit}"),
    };
    format!("{}{payload}", e.tag())
}

/// What happened to a row, as a runner reports it.
enum Outcome {
    /// Decoded; the hex of its re-encoding.
    Decoded(String),
    /// Refused; `describe`'s detail.
    Refused(String),
    /// Anything but a `DecodeError` escaped.
    Untyped(String),
}

fn observe(row: &Row, bytes: &[u8]) -> Outcome {
    match panic::catch_unwind(|| raw(row, bytes)) {
        Ok(Ok(tree)) => Outcome::Decoded(hexof(&cbor::encode(&tree))),
        Ok(Err(e)) => Outcome::Refused(describe(&e)),
        Err(payload) => {
            let text = payload
                .downcast_ref::<&str>()
                .map(|text| text.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "non-string panic payload".to_string());
            Outcome::Untyped(format!("panic: {text}"))
        }
    }
}

/// `parity.judge` for a row the runtime decoded: `None` when it passes, else why
/// it fails. `bytes` is the row's expanded input.
fn judge(case: &DecodeRow, bytes: &[u8], outcome: &Outcome) -> Option<String> {
    let want = match case.expect {
        Expect::Accept { .. } => "accept".to_string(),
        Expect::Refuse { tag, payload } => {
            payload.iter().fold(tag.to_string(), |want, (name, value)| {
                format!("{want};{name}={value}")
            })
        }
    };
    match (outcome, &case.expect) {
        (Outcome::Decoded(again), Expect::Accept { reencode }) => {
            // D2's law: decode ok => encode(decode(bytes)) == bytes, unless the row
            // declares its re-encoding.
            let expected = reencode.map_or_else(|| hexof(bytes), str::to_string);
            if *again == expected {
                None
            } else {
                Some(format!("re-encoded {again}, expected {expected}"))
            }
        }
        (Outcome::Decoded(_), Expect::Refuse { .. }) => {
            Some(format!("decoded ok, expected {want}"))
        }
        (Outcome::Untyped(why), _) => Some(format!("untyped {why}, expected {want}")),
        (Outcome::Refused(detail), Expect::Accept { .. }) => {
            Some(format!("got {detail}, expected accept"))
        }
        (Outcome::Refused(detail), Expect::Refuse { tag, payload }) => {
            let mut fields = detail.split(';');
            let got = fields.next().unwrap_or_default();
            let reported: Vec<(&str, &str)> = fields
                .map(|field| field.split_once('=').unwrap_or((field, "")))
                .collect();
            // As the gate parses a detail into a dict: a repeated field's last value.
            let drift = payload.iter().any(|(name, value)| {
                let exempt = PAYLOAD_EXEMPT.contains(&(got, *name));
                let seen = reported.iter().rev().find(|(field, _)| field == name);
                !exempt && seen.map(|(_, seen)| seen) != Some(value)
            });
            if got != *tag || drift {
                Some(format!("got {detail}, expected {want}"))
            } else {
                None
            }
        }
    }
}

#[test]
fn runtime_constants_equal_the_bounds_header() {
    let reported = format!(
        "default_max_depth={};max_depth_ceiling={}",
        cbor::DEFAULT_MAX_DEPTH,
        cbor::MAX_DEPTH_CEILING
    );
    assert_eq!(reported, CONSTANTS);
}

#[test]
fn raw_decode_rows_are_judged_as_taut_judges_rust() {
    let mut failures = Vec::new();
    let (mut replayed, mut typed) = (0, 0);
    for case in DECODE_ROWS {
        let row = &case.row;
        match row.stage {
            "raw_decode" => {
                replayed += 1;
                let bytes = expand(row);
                let why = match row.len {
                    Some(len) if bytes.len() != len => Some(format!(
                        "untyped: bytes expand to {} bytes, len is {len}",
                        bytes.len()
                    )),
                    _ => judge(case, &bytes, &observe(row, &bytes)),
                };
                if let Some(why) = why {
                    failures.push(format!("{}: {why}", row.name));
                }
            }
            "from_cbor" | "from_wire" => {
                typed += 1;
                if row.schema.is_empty() {
                    failures.push(format!("{}: a {} row names no schema", row.name, row.stage));
                }
            }
            other => failures.push(format!("{}: unknown stage {other}", row.name)),
        }
    }
    assert!(
        replayed > 0 && typed > 0,
        "{replayed} raw and {typed} typed rows"
    );
    assert!(
        failures.is_empty(),
        "{} of {replayed} raw_decode rows fail:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn int_rows_fit_the_i64_carrier_and_round_trip_through_the_runtime() {
    assert!(!ROUND_TRIP.is_empty() && !ENCODE_FAIL.is_empty());
    let mut failures = Vec::new();
    for row in ROUND_TRIP {
        let values = row.by_id.iter().flat_map(|(key, value)| [*key, *value]);
        for value in std::iter::once(row.n).chain(values) {
            if value.parse::<i64>().is_err() {
                failures.push(format!("{}: {value} does not fit i64", row.name));
            }
        }
        match cbor::try_decode(&unhex(row.cbor)) {
            Ok(tree) => {
                let again = hexof(&cbor::encode(&tree));
                if again != row.cbor {
                    failures.push(format!(
                        "{}: re-encoded {again}, expected {}",
                        row.name, row.cbor
                    ));
                }
            }
            Err(e) => failures.push(format!("{}: decode {}", row.name, describe(&e))),
        }
    }
    for row in ENCODE_FAIL {
        // `i64` is the encode side's subset guard: a value outside it cannot be built.
        if row.value.parse::<i64>().is_ok() {
            failures.push(format!(
                "{}: {} fits i64, expected out-of-subset",
                row.name, row.value
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
