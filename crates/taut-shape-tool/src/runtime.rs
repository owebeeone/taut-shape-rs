//! Shape-neutral engine dispatch shell for the conformance CLI.

use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterCode {
    UnsupportedShape,
    UnknownTag,
    MalformedMessage,
    DirectionViolation,
    #[allow(dead_code)] // Reserved for host/adapter invariant failures.
    AdapterFailure,
}

impl AdapterCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedShape => "TAUT_SHAPE_UNSUPPORTED_SHAPE",
            Self::UnknownTag => "TAUT_SHAPE_UNKNOWN_TAG",
            Self::MalformedMessage => "TAUT_SHAPE_MALFORMED_MESSAGE",
            Self::DirectionViolation => "TAUT_SHAPE_DIRECTION_VIOLATION",
            Self::AdapterFailure => "TAUT_SHAPE_ADAPTER_FAILURE",
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AdapterDiagnostic {
    pub code: AdapterCode,
    pub shape: String,
    pub message: String,
}

impl AdapterDiagnostic {
    pub fn new(code: AdapterCode, shape: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            shape: shape.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for AdapterDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

impl std::error::Error for AdapterDiagnostic {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimerAction {
    Set { token: i64, delay_ms: i64 },
    Cancel { token: i64 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeardownAction {
    pub reason: String,
}

#[derive(Debug)]
pub struct EngineEffect<FrameOut> {
    pub frame: FrameOut,
    pub timer: Option<TimerAction>,
    pub teardown: Option<TeardownAction>,
}

#[derive(Debug)]
pub struct EngineEmission<Output, FrameOut> {
    pub output: Output,
    pub effect: EngineEffect<FrameOut>,
}

type AdapterEmissions<A> =
    Vec<EngineEmission<<A as EngineAdapter>::Output, <A as EngineAdapter>::FrameOut>>;

/// Every associated type belongs to the shape. The shell knows only call order.
pub trait EngineAdapter {
    type FrameIn;
    type Input;
    type Output;
    type FrameOut;

    fn shape(&self) -> &'static str;
    fn decode_input(&mut self, frame: Self::FrameIn) -> Result<Self::Input, AdapterDiagnostic>;
    fn dispatch(&mut self, input: Self::Input) -> Vec<Self::Output>;
    fn encode_output(&self, output: &Self::Output) -> EngineEffect<Self::FrameOut>;
    fn finish(&mut self) -> Vec<Self::Output>;
}

pub struct EngineRuntime<A> {
    adapter: A,
}

impl<A: EngineAdapter> EngineRuntime<A> {
    pub fn new(adapter: A) -> Self {
        debug_assert!(!adapter.shape().is_empty());
        Self { adapter }
    }

    pub fn process(&mut self, frame: A::FrameIn) -> Result<AdapterEmissions<A>, AdapterDiagnostic> {
        let input = self.adapter.decode_input(frame)?;
        Ok(self.dispatch(input))
    }

    pub fn dispatch(&mut self, input: A::Input) -> Vec<EngineEmission<A::Output, A::FrameOut>> {
        self.adapter
            .dispatch(input)
            .into_iter()
            .map(|output| {
                let effect = self.adapter.encode_output(&output);
                EngineEmission { output, effect }
            })
            .collect()
    }

    pub fn finish(&mut self) -> Vec<EngineEmission<A::Output, A::FrameOut>> {
        let outputs = self.adapter.finish();
        outputs
            .into_iter()
            .map(|output| {
                let effect = self.adapter.encode_output(&output);
                EngineEmission { output, effect }
            })
            .collect()
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
    }
}

pub const SUPPORTED_ENGINE_SHAPES: &[&str] = &[
    "atom",
    "crdt",
    "log",
    "snapshot_delta",
    "stream",
    "swmr",
    "text_crdt",
    "value",
];

pub fn require_engine_shape(shape: &str) -> Result<&'static str, AdapterDiagnostic> {
    if let Some(supported) = SUPPORTED_ENGINE_SHAPES
        .iter()
        .copied()
        .find(|supported| *supported == shape)
    {
        return Ok(supported);
    }
    Err(AdapterDiagnostic::new(
        AdapterCode::UnsupportedShape,
        shape,
        format!(
            "unsupported engine shape {shape:?}; supported: {}",
            SUPPORTED_ENGINE_SHAPES.join(", ")
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TextAdapter;

    impl EngineAdapter for TextAdapter {
        type FrameIn = String;
        type Input = usize;
        type Output = bool;
        type FrameOut = &'static str;

        fn shape(&self) -> &'static str {
            "text-test"
        }

        fn decode_input(&mut self, frame: String) -> Result<usize, AdapterDiagnostic> {
            Ok(frame.len())
        }

        fn dispatch(&mut self, input: usize) -> Vec<bool> {
            vec![input % 2 == 0]
        }

        fn encode_output(&self, output: &bool) -> EngineEffect<&'static str> {
            EngineEffect {
                frame: if *output { "even" } else { "odd" },
                timer: None,
                teardown: None,
            }
        }

        fn finish(&mut self) -> Vec<bool> {
            Vec::new()
        }
    }

    #[test]
    fn runtime_contract_is_shape_neutral() {
        let mut runtime = EngineRuntime::new(TextAdapter);
        let emissions = runtime.process("abcd".to_owned()).unwrap();
        assert_eq!(emissions.len(), 1);
        assert!(emissions[0].output);
        assert_eq!(emissions[0].effect.frame, "even");
    }

    #[test]
    fn unknown_shape_is_a_typed_diagnostic() {
        let error = require_engine_shape("window").unwrap_err();
        assert_eq!(error.code, AdapterCode::UnsupportedShape);
        assert_eq!(error.code.as_str(), "TAUT_SHAPE_UNSUPPORTED_SHAPE");
        assert_eq!(error.shape, "window");
    }

    #[test]
    fn registry_names_exactly_the_implemented_engines() {
        assert_eq!(
            SUPPORTED_ENGINE_SHAPES,
            &[
                "atom",
                "crdt",
                "log",
                "snapshot_delta",
                "stream",
                "swmr",
                "text_crdt",
                "value"
            ]
        );
    }
}
