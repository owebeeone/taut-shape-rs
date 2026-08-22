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
//! * **tag** — the selected shape's message-type wire value as a single byte.
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
use taut_shape::generated_atom::AtomMsgType;
use taut_shape::generated_crdt::CrdtMsgType;
use taut_shape::generated_stream::StreamMsgType;
use taut_shape::generated_swmr::SwmrMsgType;
use taut_shape::generated_value::ValueMsgType;

/// A decoded frame: the message kind (as its wire tag) and the CBOR body.
pub struct Frame {
    pub tag: u8,
    pub body: Cbor,
}

/// Errors the framing layer can surface. Distinguished from a clean EOF (which
/// is `Ok(None)` out of [`read_frame`]) so the caller can map a *malformed*
/// frame to exit 3 while treating EOF as exit 0.
#[derive(Debug)]
pub enum FrameError {
    /// The CBOR body did not decode, or decoded to the wrong shape for its tag.
    MalformedBody(String),
    /// A `length` prefix that cannot be a valid frame (a zero length leaves no
    /// room for even the mandatory tag byte).
    BadLength(u32),
}

impl FrameError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MalformedBody(_) | Self::BadLength(_) => "TAUT_SHAPE_MALFORMED_MESSAGE",
        }
    }
}

impl core::fmt::Display for FrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
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

    // 3) Decode the CBOR body. The selected shape adapter validates the raw tag
    //    before dispatch, since tag registries intentionally overlap by shape.
    //    returns a typed error on any malformed input (no panic), so the old
    //    `catch_unwind` guard is gone — decode is fail-closed at the source.
    let body = match cbor::try_decode(body_bytes) {
        Ok(c) => c,
        Err(e) => return Ok(Err(FrameError::MalformedBody(format!("{e}")))),
    };

    Ok(Ok(Some(Frame {
        tag: tag_byte,
        body,
    })))
}

pub trait FrameTag {
    fn byte(self) -> u8;
}

impl FrameTag for u8 {
    fn byte(self) -> u8 {
        self
    }
}

impl FrameTag for LogMsgType {
    fn byte(self) -> u8 {
        self.wire() as u8
    }
}

impl FrameTag for ValueMsgType {
    fn byte(self) -> u8 {
        self.wire() as u8
    }
}

impl FrameTag for AtomMsgType {
    fn byte(self) -> u8 {
        self.wire() as u8
    }
}

impl FrameTag for CrdtMsgType {
    fn byte(self) -> u8 {
        self.wire() as u8
    }
}

impl FrameTag for StreamMsgType {
    fn byte(self) -> u8 {
        self.wire() as u8
    }
}

impl FrameTag for SwmrMsgType {
    fn byte(self) -> u8 {
        self.wire() as u8
    }
}

/// Write one frame to `w`: `u32-LE (1 + body.len())`, the tag byte, then body.
pub fn write_frame<W: Write, T: FrameTag>(w: &mut W, tag: T, body: &Cbor) -> io::Result<()> {
    let encoded = cbor::encode(body);
    let len = (encoded.len() + 1) as u32; // + 1 for the tag byte
    w.write_all(&len.to_le_bytes())?;
    w.write_all(&[tag.byte()])?;
    w.write_all(&encoded)?;
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_tag_is_deferred_to_the_selected_shape_adapter() {
        // Empty maps are valid CBOR bodies; 99 is intentionally not rejected
        // here because only the selected shape owns the tag registry.
        let mut bytes = &[2, 0, 0, 0, 99, 0xa0][..];
        let frame = read_frame(&mut bytes).unwrap().unwrap().unwrap();
        assert_eq!(frame.tag, 99);
    }
}
