//! A tiny, self-contained JSON value + reader + writer, plus base64.
//!
//! The tool crate stays zero-dependency (matching the engine crate's
//! philosophy — the whole workspace pulls no active external crate). The two
//! JSON touchpoints are both small and structural:
//!
//!   * `--script` files (node + client) — a producer-injection script and the
//!     taut *jsoncodec form* of the injected messages (base64 payloads, i64s as
//!     strings). We only need to *read* these.
//!   * OOB transcripts (client + node) — one jsoncodec object per line on
//!     stderr. We only need to *write* these.
//!
//! So a hand-rolled reader/writer over a small [`Json`] value is enough and
//! keeps the framing byte-contract free of a serde version pin. The reader is
//! strict enough for authored fixtures (it is not a hardened parser for
//! adversarial input — the scripts are trusted test inputs).

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// A JSON value. Object keys are kept in a `BTreeMap` so serialization is
/// deterministic (ascending keys) — the OOB transcript is a golden artifact
/// compared whole, so a stable key order matters.
#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    /// JSON numbers are kept verbatim as their source text so an i64-as-string
    /// vs bare-number distinction round-trips losslessly; callers that want an
    /// integer call [`Json::as_i64`].
    Num(String),
    Str(String),
    Arr(Vec<Json>),
    Obj(BTreeMap<String, Json>),
}

impl Json {
    /// The object field `key`, or `None` (also `None` if `self` is not an obj).
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(m) => m.get(key),
            _ => None,
        }
    }

    /// `self` as a string, or `None`.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    /// `self` as an `i64`. Accepts both a bare JSON number and a numeric string
    /// (the taut jsoncodec renders i64 fields as strings; authored fixtures may
    /// use either), so a `"5"` and a `5` both parse to `5`.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Json::Num(s) | Json::Str(s) => s.trim().parse::<i64>().ok(),
            _ => None,
        }
    }

    /// `self` as an array slice, or `None`.
    pub fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }

    fn write(&self, out: &mut String) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Num(s) => out.push_str(s),
            Json::Str(s) => write_json_string(s, out),
            Json::Arr(a) => {
                out.push('[');
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    v.write(out);
                }
                out.push(']');
            }
            Json::Obj(m) => {
                out.push('{');
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    write_json_string(k, out);
                    out.push(':');
                    v.write(out);
                }
                out.push('}');
            }
        }
    }
}

/// Compact single-line serialization (no whitespace). Object keys emit in
/// ascending order (the `BTreeMap` invariant) so the line is stable — the OOB
/// transcript is a golden artifact compared whole.
impl std::fmt::Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        self.write(&mut out);
        f.write_str(&out)
    }
}

/// Convenience: build a `Json::Str`.
pub fn s(v: impl Into<String>) -> Json {
    Json::Str(v.into())
}

/// Convenience: build a `Json::Num` from an integer (rendered as a numeric
/// *string*, matching taut jsoncodec's i64-as-string convention).
pub fn i64_str(v: i64) -> Json {
    Json::Str(v.to_string())
}

fn write_json_string(v: &str, out: &mut String) {
    out.push('"');
    for c in v.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Parse a JSON document. Returns a human-readable error string on malformed
/// input (the scripts are trusted fixtures, so this favors a clear message over
/// precise positions).
pub fn parse(text: &str) -> Result<Json, String> {
    let mut p = Parser {
        bytes: text.as_bytes(),
        pos: 0,
    };
    p.skip_ws();
    let v = p.value()?;
    p.skip_ws();
    if p.pos != p.bytes.len() {
        return Err(format!("trailing content at byte {}", p.pos));
    }
    Ok(v)
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl Parser<'_> {
    fn skip_ws(&mut self) {
        while let Some(&b) = self.bytes.get(self.pos) {
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn value(&mut self) -> Result<Json, String> {
        self.skip_ws();
        match self.bytes.get(self.pos) {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') => self.literal("true", Json::Bool(true)),
            Some(b'f') => self.literal("false", Json::Bool(false)),
            Some(b'n') => self.literal("null", Json::Null),
            Some(&b) if b == b'-' || b.is_ascii_digit() => self.number(),
            Some(&b) => Err(format!("unexpected byte {:?} at {}", b as char, self.pos)),
            None => Err("unexpected end of input".to_string()),
        }
    }

    fn literal(&mut self, lit: &str, val: Json) -> Result<Json, String> {
        if self.bytes[self.pos..].starts_with(lit.as_bytes()) {
            self.pos += lit.len();
            Ok(val)
        } else {
            Err(format!("invalid literal at {}", self.pos))
        }
    }

    fn number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        while let Some(&b) = self.bytes.get(self.pos) {
            if b.is_ascii_digit() || matches!(b, b'-' | b'+' | b'.' | b'e' | b'E') {
                self.pos += 1;
            } else {
                break;
            }
        }
        let raw = std::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|_| "non-utf8 number".to_string())?;
        Ok(Json::Num(raw.to_string()))
    }

    fn string(&mut self) -> Result<String, String> {
        // opening quote
        self.pos += 1;
        let mut out = String::new();
        loop {
            let b = *self
                .bytes
                .get(self.pos)
                .ok_or_else(|| "unterminated string".to_string())?;
            self.pos += 1;
            match b {
                b'"' => return Ok(out),
                b'\\' => {
                    let e = *self
                        .bytes
                        .get(self.pos)
                        .ok_or_else(|| "bad escape".to_string())?;
                    self.pos += 1;
                    match e {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000c}'),
                        b'u' => {
                            let cp = self.hex4()?;
                            // Surrogate pair handling (enough for fixtures).
                            if (0xD800..=0xDBFF).contains(&cp) {
                                if self.bytes.get(self.pos) == Some(&b'\\')
                                    && self.bytes.get(self.pos + 1) == Some(&b'u')
                                {
                                    self.pos += 2;
                                    let lo = self.hex4()?;
                                    let c = 0x10000
                                        + (((cp - 0xD800) as u32) << 10)
                                        + (lo - 0xDC00) as u32;
                                    out.push(
                                        char::from_u32(c)
                                            .ok_or_else(|| "bad surrogate".to_string())?,
                                    );
                                } else {
                                    return Err("lone high surrogate".to_string());
                                }
                            } else {
                                out.push(
                                    char::from_u32(cp as u32)
                                        .ok_or_else(|| "bad \\u escape".to_string())?,
                                );
                            }
                        }
                        _ => return Err(format!("unknown escape \\{}", e as char)),
                    }
                }
                _ => {
                    // A UTF-8 continuation byte or ASCII: collect raw bytes for
                    // one code point. Simplest correct approach: back up and
                    // decode a full char from the byte stream.
                    let start = self.pos - 1;
                    // advance over any continuation bytes (0b10xxxxxx)
                    while self
                        .bytes
                        .get(self.pos)
                        .is_some_and(|&c| c & 0xC0 == 0x80)
                    {
                        self.pos += 1;
                    }
                    let chunk = &self.bytes[start..self.pos];
                    out.push_str(
                        std::str::from_utf8(chunk).map_err(|_| "invalid utf8 in string")?,
                    );
                }
            }
        }
    }

    fn hex4(&mut self) -> Result<u16, String> {
        let slice = self
            .bytes
            .get(self.pos..self.pos + 4)
            .ok_or_else(|| "short \\u escape".to_string())?;
        let hs = std::str::from_utf8(slice).map_err(|_| "bad \\u hex".to_string())?;
        let v = u16::from_str_radix(hs, 16).map_err(|_| "bad \\u hex".to_string())?;
        self.pos += 4;
        Ok(v)
    }

    fn array(&mut self) -> Result<Json, String> {
        self.pos += 1; // [
        let mut out = Vec::new();
        self.skip_ws();
        if self.bytes.get(self.pos) == Some(&b']') {
            self.pos += 1;
            return Ok(Json::Arr(out));
        }
        loop {
            out.push(self.value()?);
            self.skip_ws();
            match self.bytes.get(self.pos) {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Json::Arr(out));
                }
                _ => return Err(format!("expected , or ] at {}", self.pos)),
            }
        }
    }

    fn object(&mut self) -> Result<Json, String> {
        self.pos += 1; // {
        let mut out = BTreeMap::new();
        self.skip_ws();
        if self.bytes.get(self.pos) == Some(&b'}') {
            self.pos += 1;
            return Ok(Json::Obj(out));
        }
        loop {
            self.skip_ws();
            if self.bytes.get(self.pos) != Some(&b'"') {
                return Err(format!("expected object key string at {}", self.pos));
            }
            let key = self.string()?;
            self.skip_ws();
            if self.bytes.get(self.pos) != Some(&b':') {
                return Err(format!("expected : at {}", self.pos));
            }
            self.pos += 1;
            let val = self.value()?;
            out.insert(key, val);
            self.skip_ws();
            match self.bytes.get(self.pos) {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Json::Obj(out));
                }
                _ => return Err(format!("expected , or }} at {}", self.pos)),
            }
        }
    }
}

// ── base64 (standard alphabet, with padding) ────────────────────────────────

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encode bytes to standard base64 with `=` padding (the taut jsoncodec form).
pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(B64[((n >> 18) & 0x3f) as usize] as char);
        out.push(B64[((n >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            out.push(B64[((n >> 6) & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(B64[(n & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

/// Decode standard base64 (padding optional). Returns `None` on any invalid
/// character or a malformed length.
pub fn base64_decode(s: &str) -> Option<Vec<u8>> {
    fn val(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some((c - b'A') as u32),
            b'a'..=b'z' => Some((c - b'a' + 26) as u32),
            b'0'..=b'9' => Some((c - b'0' + 52) as u32),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    // Strip padding and whitespace, then fold 6 bits at a time into a running
    // accumulator, emitting a byte whenever 8+ bits are available (MSB-first).
    let mut acc = 0u32;
    let mut nbits = 0u32;
    let mut out = Vec::new();
    for b in s.bytes() {
        if b == b'=' || b.is_ascii_whitespace() {
            continue;
        }
        acc = (acc << 6) | val(b)?;
        nbits += 6;
        if nbits >= 8 {
            nbits -= 8;
            out.push(((acc >> nbits) & 0xff) as u8);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_round_trips_and_matches_known_vectors() {
        // RFC 4648 test vectors.
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
        // corpus payloads decode back.
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
        assert_eq!(base64_decode("cmVjMQ==").unwrap(), b"rec1");
        // NUL-safety / binary round-trip.
        let bin: Vec<u8> = (0u8..=255).collect();
        assert_eq!(base64_decode(&base64_encode(&bin)).unwrap(), bin);
    }

    #[test]
    fn parses_a_corpus_style_read_object() {
        let v = parse(
            r#"{"cursor":{"seq":"0"},"log_id":"log-A","max_records":"10","stream_id":"s1","timeout_ms":"0","type":"read"}"#,
        )
        .unwrap();
        assert_eq!(v.get("type").and_then(Json::as_str), Some("read"));
        assert_eq!(v.get("stream_id").and_then(Json::as_str), Some("s1"));
        assert_eq!(v.get("max_records").and_then(Json::as_i64), Some(10));
        assert_eq!(
            v.get("cursor").and_then(|c| c.get("seq")).and_then(Json::as_i64),
            Some(0)
        );
    }

    #[test]
    fn round_trips_object_serialization_with_stable_keys() {
        let v = parse(r#"{"b":"2","a":"1"}"#).unwrap();
        // BTreeMap => keys ascending regardless of source order.
        assert_eq!(v.to_string(), r#"{"a":"1","b":"2"}"#);
    }
}
