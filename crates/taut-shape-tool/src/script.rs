//! The `--script` producer-injection format (interop driver, Oracle §7).
//!
//! A script is JSON: an ordered list of injection points
//!
//! ```jsonc
//! [
//!   { "after_frames": 1, "inputs": [ { "type": "push", "payload": "aGVsbG8=" } ] },
//!   { "after_frames": 2, "inputs": [ { "type": "seal" } ] }
//! ]
//! ```
//!
//! `after_frames: k` means "once the k-th client frame has been processed,
//! inject these producer messages, deterministically". A `client frame` is
//! whichever frame the peer client drives: in `node` mode it is an input frame
//! consumed from stdin; the injected [`Input`]s are fed to the local engine and
//! their outputs written out too. This is how an interop scenario drives the
//! producer while the client owns the node's stdin.
//!
//! The top-level may also be an object `{ "steps": [ … ] }` so a script file can
//! carry a comment or metadata alongside the ordered list; only the `steps`
//! array is read.

use crate::json::{self, Json};

/// One injection point: after `after_frames` client frames, feed `inputs`.
pub struct Injection<T> {
    pub after_frames: u64,
    pub inputs: Vec<T>,
}

/// A parsed producer script: injection points sorted by `after_frames` so a
/// single forward cursor over them is enough during the pump.
pub struct Script<T> {
    pub injections: Vec<Injection<T>>,
}

impl Script<Json> {
    /// Parse a script from its JSON text. Returns a human string on any parse or
    /// shape error (mapped by the caller to a usage exit).
    pub fn parse(text: &str) -> Result<Script<Json>, String> {
        let doc = json::parse(text)?;
        let steps: &[Json] = match &doc {
            Json::Arr(a) => a,
            Json::Obj(_) => doc
                .get("steps")
                .and_then(Json::as_arr)
                .ok_or_else(|| "script object must have a `steps` array".to_string())?,
            _ => return Err("script must be an array or {steps: [...]}".to_string()),
        };

        let mut injections = Vec::with_capacity(steps.len());
        for (i, step) in steps.iter().enumerate() {
            let after_frames = step
                .get("after_frames")
                .and_then(Json::as_i64)
                .ok_or_else(|| format!("script step {i}: missing integer `after_frames`"))?;
            if after_frames < 0 {
                return Err(format!("script step {i}: `after_frames` must be >= 0"));
            }
            let raw_inputs = step
                .get("inputs")
                .and_then(Json::as_arr)
                .ok_or_else(|| format!("script step {i}: missing `inputs` array"))?;
            let inputs = raw_inputs.to_vec();
            injections.push(Injection {
                after_frames: after_frames as u64,
                inputs,
            });
        }
        // Stable sort by trigger so the pump advances one cursor forward.
        injections.sort_by_key(|inj| inj.after_frames);
        Ok(Script { injections })
    }

    /// Load and parse a script file.
    pub fn load(path: &str) -> Result<Script<Json>, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read script {path:?}: {e}"))?;
        Script::parse(&text)
    }
}

impl<T> Script<T> {
    /// Decode every raw input through the selected shape's jsoncodec bridge.
    pub fn decode<U>(
        self,
        mut decode: impl FnMut(&T) -> Result<U, String>,
    ) -> Result<Script<U>, String> {
        let mut injections = Vec::with_capacity(self.injections.len());
        for (i, injection) in self.injections.into_iter().enumerate() {
            let mut inputs = Vec::with_capacity(injection.inputs.len());
            for (j, input) in injection.inputs.iter().enumerate() {
                inputs.push(
                    decode(input).map_err(|error| format!("script step {i} input {j}: {error}"))?,
                );
            }
            injections.push(Injection {
                after_frames: injection.after_frames,
                inputs,
            });
        }
        Ok(Script { injections })
    }
}

/// A forward cursor that yields the injections due after a given client-frame
/// count. Because the injections are sorted, [`Pending::take_due`] can be called
/// with a monotonically increasing count and drains each injection exactly once.
pub struct Pending<T> {
    injections: std::vec::IntoIter<Injection<T>>,
    next: Option<Injection<T>>,
}

impl<T> Pending<T> {
    pub fn new(script: Script<T>) -> Self {
        let mut injections = script.injections.into_iter();
        let next = injections.next();
        Pending { injections, next }
    }

    /// Drain every injection whose `after_frames <= processed`, in order, and
    /// return their flattened inputs. Call with a non-decreasing `processed`.
    pub fn take_due(&mut self, processed: u64) -> Vec<T> {
        let mut due = Vec::new();
        while let Some(inj) = &self.next {
            if inj.after_frames <= processed {
                let inj = self.next.take().unwrap();
                due.extend(inj.inputs);
                self.next = self.injections.next();
            } else {
                break;
            }
        }
        due
    }

    /// Any injections still pending (used to flush trailing ones at EOF, whose
    /// trigger count is beyond the frames that actually arrived).
    pub fn drain_remaining(&mut self) -> Vec<T> {
        let mut rest = Vec::new();
        if let Some(inj) = self.next.take() {
            rest.extend(inj.inputs);
        }
        for inj in self.injections.by_ref() {
            rest.extend(inj.inputs);
        }
        rest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jsoncodec;
    use taut_shape::Input;

    #[test]
    fn parses_ordered_injections_and_fires_by_frame_count() {
        let text = r#"[
          { "after_frames": 2, "inputs": [ { "type": "seal" } ] },
          { "after_frames": 1, "inputs": [ { "type": "push", "payload": "aGVsbG8=" } ] }
        ]"#;
        let script = Script::parse(text)
            .unwrap()
            .decode(jsoncodec::input_from_json)
            .unwrap();
        // sorted by after_frames.
        assert_eq!(script.injections[0].after_frames, 1);
        assert_eq!(script.injections[1].after_frames, 2);

        let mut p = Pending::new(script);
        assert!(p.take_due(0).is_empty());
        let due1 = p.take_due(1);
        assert_eq!(
            due1,
            vec![Input::Push {
                payload: b"hello".to_vec()
            }]
        );
        // frame 1 already drained; frame 2 fires the seal.
        let due2 = p.take_due(2);
        assert_eq!(due2, vec![Input::Seal]);
        assert!(p.take_due(3).is_empty());
    }

    #[test]
    fn steps_object_wrapper_and_close_with_error() {
        let text = r#"{ "comment": "x", "steps": [
          { "after_frames": 1, "inputs": [
              { "type": "close", "error": { "code": "producer_error", "message": "boom" } } ] }
        ] }"#;
        let script = Script::parse(text)
            .unwrap()
            .decode(jsoncodec::input_from_json)
            .unwrap();
        let mut p = Pending::new(script);
        match &p.take_due(1)[0] {
            Input::Close { error: Some(e) } => {
                assert_eq!(e.message.as_deref(), Some("boom"));
            }
            other => panic!("expected close{{error}}, got {other:?}"),
        }
    }

    #[test]
    fn drain_remaining_flushes_untriggered_tail() {
        let text = r#"[ { "after_frames": 99, "inputs": [ { "type": "seal" } ] } ]"#;
        let mut p = Pending::new(
            Script::parse(text)
                .unwrap()
                .decode(jsoncodec::input_from_json)
                .unwrap(),
        );
        assert!(p.take_due(3).is_empty());
        assert_eq!(p.drain_remaining(), vec![Input::Seal]);
    }
}
