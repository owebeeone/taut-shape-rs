//! The stdin/stdout data channel: length-prefixed, tagged CBOR frames (S4.1).
//!
//! One frame on the wire is exactly three parts, back to back:
//!
//! ```text
//!   ┌────────────────┬──────────┬───────────────────────────┐
//!   │ u32-LE length  │ tag byte │ CBOR body (`length` bytes) │
//!   └────────────────┴──────────┴───────────────────────────┘
//!         4 bytes        1 byte           `length` bytes
//! ```
//!
//! * **length** — a little-endian `u32` byte count of everything that follows
//!   the length prefix itself: the 1 tag byte **plus** the CBOR body. So the
//!   number of body bytes is `length - 1` and the total frame size on the wire
//!   is `4 + length`.
//! * **tag** — the generated [`LogMsgType`] wire value as a single byte (0..=11).
//! * **body** — the message's deterministic-CBOR encoding (the generated
//!   `to_cbor()` fed through [`taut_shape::cbor::encode`]).
//!
//! Truncated tail (a partial frame at EOF) is tolerated by [`read_frame`]: it
//! returns `Ok(None)` on a clean EOF *between* frames and on a short read while
//! reading a frame's length or body, so a producer that dies mid-frame drains
//! cleanly rather than erroring.

use std::io::{self, Read, Write};

use taut_shape::cbor::{self, Cbor};
use taut_shape::generated::LogMsgType;

/// A decoded frame: the message kind (as its wire tag) and the CBOR body.
pub struct Frame {
    pub tag: LogMsgType,
    pub body: Cbor,
}

/// Errors the framing layer can surface. Distinguished from a clean EOF (which
/// is `Ok(None)` out of [`read_frame`]) so the caller can map a *malformed*
/// frame to exit 3 while treating EOF as exit 0.
#[derive(Debug)]
pub enum FrameError {
    /// The 1 tag byte was not a known [`LogMsgType`] wire value.
    UnknownTag(u8),
    /// The CBOR body did not decode, or decoded to the wrong shape for its tag.
    MalformedBody(String),
    /// A `length` prefix that cannot be a valid frame (a zero length leaves no
    /// room for even the mandatory tag byte).
    BadLength(u32),
}

impl core::fmt::Display for FrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FrameError::UnknownTag(t) => write!(f, "unknown frame tag byte {t}"),
            FrameError::MalformedBody(m) => write!(f, "malformed frame body: {m}"),
            FrameError::BadLength(n) => write!(f, "bad frame length {n} (min 1 for the tag byte)"),
        }
    }
}

/// Read one frame from `r`.
///
/// * `Ok(Some(frame))` — a complete, well-formed frame was read.
/// * `Ok(None)` — clean EOF: either between frames, or a truncated tail (a
///   partial length prefix or body at EOF). Both mean "stop, exit 0".
/// * `Err(FrameError)` — a frame was fully present on the wire but malformed
///   (unknown tag, undecodable/ill-shaped body, or an impossible length).
/// * The `io::Error` arm covers real stream faults (not EOF).
pub fn read_frame<R: Read>(r: &mut R) -> io::Result<Result<Option<Frame>, FrameError>> {
    // 1) u32-LE length prefix. A short read here = truncated tail ⇒ clean stop.
    let mut len_buf = [0u8; 4];
    match read_exact_or_eof(r, &mut len_buf)? {
        ReadOutcome::Eof => return Ok(Ok(None)),
        ReadOutcome::Filled => {}
    }
    let len = u32::from_le_bytes(len_buf);
    // The length covers the tag byte + body, so it must be at least 1.
    if len == 0 {
        return Ok(Err(FrameError::BadLength(len)));
    }

    // 2) tag byte + body, exactly `len` bytes. A short read = truncated tail.
    let mut frame_buf = vec![0u8; len as usize];
    match read_exact_or_eof(r, &mut frame_buf)? {
        ReadOutcome::Eof => return Ok(Ok(None)),
        ReadOutcome::Filled => {}
    }
    let tag_byte = frame_buf[0];
    let body_bytes = &frame_buf[1..];

    // 3) tag byte ⇒ LogMsgType. `from_wire` panics on bad values, so range-check
    //    first and turn an out-of-range tag into a typed error (never a panic).
    let tag = match wire_to_tag(tag_byte) {
        Some(t) => t,
        None => return Ok(Err(FrameError::UnknownTag(tag_byte))),
    };

    // 4) Decode the CBOR body. `cbor::decode` panics on malformed input, so
    //    guard it with `catch_unwind` to keep the tool panic-free (exit 3).
    let body = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        cbor::decode(body_bytes)
    })) {
        Ok(c) => c,
        Err(_) => {
            return Ok(Err(FrameError::MalformedBody(
                "CBOR did not decode".to_string(),
            )))
        }
    };

    Ok(Ok(Some(Frame { tag, body })))
}

/// Write one frame to `w`: `u32-LE (1 + body.len())`, the tag byte, then body.
pub fn write_frame<W: Write>(w: &mut W, tag: LogMsgType, body: &Cbor) -> io::Result<()> {
    let encoded = cbor::encode(body);
    let len = (encoded.len() + 1) as u32; // + 1 for the tag byte
    w.write_all(&len.to_le_bytes())?;
    w.write_all(&[tag.wire() as u8])?;
    w.write_all(&encoded)?;
    Ok(())
}

/// Map a raw tag byte to a [`LogMsgType`] without tripping the generated
/// `from_wire` panic on an out-of-range value.
fn wire_to_tag(byte: u8) -> Option<LogMsgType> {
    // 0..=11 are the twelve defined wire values (push..diagnostic). Anything
    // else has no message and is a malformed frame.
    if (0..=11).contains(&byte) {
        Some(LogMsgType::from_wire(byte as i64))
    } else {
        None
    }
}

enum ReadOutcome {
    Filled,
    Eof,
}

/// Like `Read::read_exact`, but a clean EOF (0 bytes, or a short read that ends
/// at EOF) is reported as [`ReadOutcome::Eof`] rather than an error — this is
/// the truncated-tail tolerance the data channel requires.
fn read_exact_or_eof<R: Read>(r: &mut R, buf: &mut [u8]) -> io::Result<ReadOutcome> {
    let mut filled = 0;
    while filled < buf.len() {
        match r.read(&mut buf[filled..]) {
            Ok(0) => {
                // EOF. A partial fill here is a truncated tail — tolerated.
                return Ok(ReadOutcome::Eof);
            }
            Ok(n) => filled += n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(ReadOutcome::Filled)
}
