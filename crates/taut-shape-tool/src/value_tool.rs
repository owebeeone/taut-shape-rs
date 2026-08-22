//! Value node/client CLI adapters over the shared framing and engine runtime.

use std::io::{self, BufWriter, Read, Write};

use taut_shape::cbor::Cbor;
use taut_shape::generated_value::{
    ValueDiagnostic, ValueMsgType, ValueReadRequest, ValueReadResponse, ValueSet,
};
use taut_shape::{ValueInput, ValueNode, ValueOutput};

use crate::framing::{self, Frame};
use crate::json::{self, Json};
use crate::runtime::{
    AdapterCode, AdapterDiagnostic, EngineAdapter, EngineEffect, EngineEmission, EngineRuntime,
};
use crate::script::{Pending, Script};
use crate::value_json;

pub struct NodeOpts {
    pub script: Option<Script<ValueInput>>,
}

pub struct ClientOpts {
    pub value_id: String,
    pub stream_id: String,
    pub reads: u32,
}

struct ValueAdapter {
    node: ValueNode,
}

impl ValueAdapter {
    fn new() -> Self {
        Self {
            node: ValueNode::new(),
        }
    }
}

impl EngineAdapter for ValueAdapter {
    type FrameIn = Frame;
    type Input = ValueInput;
    type Output = ValueOutput;
    type FrameOut = (u8, Cbor);

    fn shape(&self) -> &'static str {
        "value"
    }

    fn decode_input(&mut self, frame: Frame) -> Result<ValueInput, AdapterDiagnostic> {
        let tag = ValueMsgType::from_wire(frame.tag as i64).map_err(|_| {
            AdapterDiagnostic::new(
                AdapterCode::UnknownTag,
                "value",
                format!("unknown frame tag byte {}", frame.tag),
            )
        })?;
        let malformed = |error: taut_shape::cbor::DecodeError| {
            AdapterDiagnostic::new(
                AdapterCode::MalformedMessage,
                "value",
                format!("malformed {tag:?} body: {error}"),
            )
        };
        match tag {
            ValueMsgType::Set => ValueSet::from_cbor(&frame.body)
                .map(ValueInput::Set)
                .map_err(malformed),
            ValueMsgType::Read => ValueReadRequest::from_cbor(&frame.body)
                .map(ValueInput::Read)
                .map_err(malformed),
            ValueMsgType::ReadResponse | ValueMsgType::Diagnostic => Err(AdapterDiagnostic::new(
                AdapterCode::DirectionViolation,
                "value",
                format!("output-only tag {} on the input channel", tag.wire()),
            )),
        }
    }

    fn dispatch(&mut self, input: ValueInput) -> Vec<ValueOutput> {
        self.node.handle(input)
    }

    fn encode_output(&self, output: &ValueOutput) -> EngineEffect<(u8, Cbor)> {
        let (tag, body) = match output {
            ValueOutput::ReadResponse(message) => (ValueMsgType::ReadResponse, message.to_cbor()),
            ValueOutput::Diagnostic(message) => (ValueMsgType::Diagnostic, message.to_cbor()),
        };
        EngineEffect {
            frame: (tag.wire() as u8, body),
            timer: None,
            teardown: None,
        }
    }

    fn finish(&mut self) -> Vec<ValueOutput> {
        Vec::new()
    }
}

pub fn run_node(opts: NodeOpts) -> u8 {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stderr = io::stderr();
    match drive_node(
        &mut stdin.lock(),
        &mut BufWriter::new(stdout.lock()),
        &mut stderr.lock(),
        opts,
    ) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("taut-shape-tool value node: I/O error: {error}");
            1
        }
    }
}

fn drive_node<R: Read, W: Write, T: Write>(
    input: &mut R,
    output: &mut W,
    transcript: &mut T,
    opts: NodeOpts,
) -> io::Result<u8> {
    let mut runtime = EngineRuntime::new(ValueAdapter::new());
    let mut pending = opts.script.map(Pending::new);
    let mut frames = 0;
    if let Some(script) = &mut pending {
        dispatch_injected(&mut runtime, output, transcript, script.take_due(0))?;
        output.flush()?;
    }
    loop {
        match framing::read_frame(input)? {
            Ok(None) => {
                if let Some(script) = &mut pending {
                    dispatch_injected(&mut runtime, output, transcript, script.drain_remaining())?;
                }
                write_emissions(output, runtime.finish())?;
                output.flush()?;
                return Ok(0);
            }
            Ok(Some(frame)) => match runtime.process(frame) {
                Ok(emissions) => {
                    write_emissions(output, emissions)?;
                    frames += 1;
                    if let Some(script) = &mut pending {
                        dispatch_injected(
                            &mut runtime,
                            output,
                            transcript,
                            script.take_due(frames),
                        )?;
                    }
                    output.flush()?;
                }
                Err(error) => {
                    eprintln!("taut-shape-tool value node: {error}");
                    return Ok(3);
                }
            },
            Err(error) => {
                eprintln!("taut-shape-tool value node: {}: {error}", error.code());
                return Ok(3);
            }
        }
    }
}

fn dispatch_injected<W: Write, T: Write>(
    runtime: &mut EngineRuntime<ValueAdapter>,
    output: &mut W,
    transcript: &mut T,
    inputs: Vec<ValueInput>,
) -> io::Result<()> {
    for input in inputs {
        for emission in runtime.dispatch(input) {
            let _ = writeln!(
                transcript,
                "{}",
                value_json::output_to_json(&emission.output)
            );
            framing::write_frame(output, emission.effect.frame.0, &emission.effect.frame.1)?;
        }
    }
    Ok(())
}

fn write_emissions<W: Write>(
    output: &mut W,
    emissions: Vec<EngineEmission<ValueOutput, (u8, Cbor)>>,
) -> io::Result<()> {
    for emission in emissions {
        framing::write_frame(output, emission.effect.frame.0, &emission.effect.frame.1)?;
    }
    Ok(())
}

pub fn run_client(opts: ClientOpts) -> u8 {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stderr = io::stderr();
    match drive_client(
        &mut stdin.lock(),
        &mut BufWriter::new(stdout.lock()),
        &mut stderr.lock(),
        opts,
    ) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("taut-shape-tool value client: I/O error: {error}");
            1
        }
    }
}

fn drive_client<R: Read, W: Write, T: Write>(
    input: &mut R,
    output: &mut W,
    transcript: &mut T,
    opts: ClientOpts,
) -> io::Result<u8> {
    for _ in 0..opts.reads {
        let request = ValueReadRequest {
            value_id: opts.value_id.clone(),
            stream_id: opts.stream_id.clone(),
        };
        framing::write_frame(output, ValueMsgType::Read, &request.to_cbor())?;
        output.flush()?;
        loop {
            let frame = match framing::read_frame(input)? {
                Ok(Some(frame)) => frame,
                Ok(None) => return Ok(0),
                Err(error) => {
                    eprintln!("taut-shape-tool value client: {}: {error}", error.code());
                    return Ok(3);
                }
            };
            let tag = match ValueMsgType::from_wire(frame.tag as i64) {
                Ok(tag) => tag,
                Err(_) => {
                    eprintln!(
                        "taut-shape-tool value client: TAUT_SHAPE_UNKNOWN_TAG: unknown frame tag byte {}",
                        frame.tag
                    );
                    return Ok(3);
                }
            };
            match tag {
                ValueMsgType::ReadResponse => match ValueReadResponse::from_cbor(&frame.body) {
                    Ok(response) => {
                        writeln!(
                            transcript,
                            "{}",
                            value_json::read_response_to_json(&response)
                        )?;
                        break;
                    }
                    Err(error) => {
                        eprintln!("taut-shape-tool value client: malformed response: {error}");
                        return Ok(3);
                    }
                },
                ValueMsgType::Diagnostic => match ValueDiagnostic::from_cbor(&frame.body) {
                    Ok(diagnostic) => writeln!(
                        transcript,
                        "{}",
                        value_json::output_to_json(&ValueOutput::Diagnostic(diagnostic))
                    )?,
                    Err(error) => {
                        eprintln!("taut-shape-tool value client: malformed diagnostic: {error}");
                        return Ok(3);
                    }
                },
                ValueMsgType::Set | ValueMsgType::Read => {
                    eprintln!(
                        "taut-shape-tool value client: input-only tag {} on response channel",
                        tag.wire()
                    );
                    return Ok(3);
                }
            }
        }
    }
    let mut final_line = std::collections::BTreeMap::new();
    final_line.insert("state".to_string(), json::s("done"));
    final_line.insert("type".to_string(), json::s("client_final"));
    writeln!(transcript, "{}", Json::Obj(final_line))?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_value_tag_is_typed_before_dispatch() {
        let error = ValueAdapter::new()
            .decode_input(Frame {
                tag: 99,
                body: Cbor::Map(Vec::new()),
            })
            .unwrap_err();
        assert_eq!(error.code, AdapterCode::UnknownTag);
        assert_eq!(error.code.as_str(), "TAUT_SHAPE_UNKNOWN_TAG");
    }
}
