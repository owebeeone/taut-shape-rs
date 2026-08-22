//! Atom node/client CLI adapters over the shared framing and engine runtime.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufWriter, Read, Write};

use taut_shape::cbor::Cbor;
use taut_shape::generated_atom::{
    AtomCancelTimer, AtomClose, AtomDiagnostic, AtomEndStream, AtomMsgType, AtomProducerStop,
    AtomReadRequest, AtomReadResponse, AtomReplace, AtomSeal, AtomSetTimer, AtomState,
    AtomTimerExpired, AtomVersion,
};
use taut_shape::{AtomInput, AtomNode, AtomOutput, StopWhen};

use crate::atom_json;
use crate::framing::{self, Frame};
use crate::json::{self, Json};
use crate::runtime::{
    AdapterCode, AdapterDiagnostic, EngineAdapter, EngineEffect, EngineEmission, EngineRuntime,
    TeardownAction, TimerAction,
};
use crate::script::{Pending, Script};

pub struct NodeOpts {
    pub stop_when: StopWhen,
    pub script: Option<Script<AtomInput>>,
}

pub struct ClientOpts {
    pub atom_id: String,
    pub stream_ids: Vec<String>,
    pub from: i64,
    pub timeout_ms: Option<i64>,
}

struct AtomAdapter {
    node: AtomNode,
}

impl AtomAdapter {
    fn new(stop_when: StopWhen) -> Self {
        Self {
            node: AtomNode::new(stop_when),
        }
    }
}

impl EngineAdapter for AtomAdapter {
    type FrameIn = Frame;
    type Input = AtomInput;
    type Output = AtomOutput;
    type FrameOut = (u8, Cbor);

    fn shape(&self) -> &'static str {
        "atom"
    }

    fn decode_input(&mut self, frame: Frame) -> Result<AtomInput, AdapterDiagnostic> {
        let tag = AtomMsgType::from_wire(frame.tag as i64).map_err(|_| {
            AdapterDiagnostic::new(
                AdapterCode::UnknownTag,
                "atom",
                format!("unknown frame tag byte {}", frame.tag),
            )
        })?;
        let malformed = |error: taut_shape::cbor::DecodeError| {
            AdapterDiagnostic::new(
                AdapterCode::MalformedMessage,
                "atom",
                format!("malformed {tag:?} body: {error}"),
            )
        };
        match tag {
            AtomMsgType::Replace => AtomReplace::from_cbor(&frame.body)
                .map(AtomInput::Replace)
                .map_err(malformed),
            AtomMsgType::Seal => AtomSeal::from_cbor(&frame.body)
                .map(AtomInput::Seal)
                .map_err(malformed),
            AtomMsgType::Close => AtomClose::from_cbor(&frame.body)
                .map(AtomInput::Close)
                .map_err(malformed),
            AtomMsgType::Read => AtomReadRequest::from_cbor(&frame.body)
                .map(AtomInput::Read)
                .map_err(malformed),
            AtomMsgType::EndStream => AtomEndStream::from_cbor(&frame.body)
                .map(AtomInput::EndStream)
                .map_err(malformed),
            AtomMsgType::TimerExpired => AtomTimerExpired::from_cbor(&frame.body)
                .map(AtomInput::TimerExpired)
                .map_err(malformed),
            AtomMsgType::ReadResponse
            | AtomMsgType::SetTimer
            | AtomMsgType::CancelTimer
            | AtomMsgType::ProducerStop
            | AtomMsgType::Diagnostic => Err(AdapterDiagnostic::new(
                AdapterCode::DirectionViolation,
                "atom",
                format!("output-only tag {} on the input channel", tag.wire()),
            )),
        }
    }

    fn dispatch(&mut self, input: AtomInput) -> Vec<AtomOutput> {
        self.node.handle(input)
    }

    fn encode_output(&self, output: &AtomOutput) -> EngineEffect<(u8, Cbor)> {
        let (tag, body, timer, teardown) = match output {
            AtomOutput::ReadResponse(message) => {
                (AtomMsgType::ReadResponse, message.to_cbor(), None, None)
            }
            AtomOutput::SetTimer(message) => (
                AtomMsgType::SetTimer,
                message.to_cbor(),
                Some(TimerAction::Set {
                    token: message.token,
                    delay_ms: message.ms,
                }),
                None,
            ),
            AtomOutput::CancelTimer(message) => (
                AtomMsgType::CancelTimer,
                message.to_cbor(),
                Some(TimerAction::Cancel {
                    token: message.token,
                }),
                None,
            ),
            AtomOutput::ProducerStop(message) => (
                AtomMsgType::ProducerStop,
                message.to_cbor(),
                None,
                Some(TeardownAction {
                    reason: format!("{:?}", message.reason),
                }),
            ),
            AtomOutput::Diagnostic(message) => {
                (AtomMsgType::Diagnostic, message.to_cbor(), None, None)
            }
        };
        EngineEffect {
            frame: (tag.wire() as u8, body),
            timer,
            teardown,
        }
    }

    fn finish(&mut self) -> Vec<AtomOutput> {
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
            eprintln!("taut-shape-tool atom node: I/O error: {error}");
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
    let mut runtime = EngineRuntime::new(AtomAdapter::new(opts.stop_when));
    let mut pending = opts.script.map(Pending::new);
    let mut frames = 0;
    if let Some(script) = &mut pending {
        write_injected(&mut runtime, output, transcript, script.take_due(0))?;
        output.flush()?;
    }
    loop {
        match framing::read_frame(input)? {
            Ok(None) => {
                if let Some(script) = &mut pending {
                    write_injected(&mut runtime, output, transcript, script.drain_remaining())?;
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
                        write_injected(&mut runtime, output, transcript, script.take_due(frames))?;
                    }
                    output.flush()?;
                }
                Err(error) => {
                    eprintln!("taut-shape-tool atom node: {error}");
                    return Ok(3);
                }
            },
            Err(error) => {
                eprintln!("taut-shape-tool atom node: {}: {error}", error.code());
                return Ok(3);
            }
        }
    }
}

fn write_injected<W: Write, T: Write>(
    runtime: &mut EngineRuntime<AtomAdapter>,
    output: &mut W,
    transcript: &mut T,
    inputs: Vec<AtomInput>,
) -> io::Result<()> {
    for input in inputs {
        for emission in runtime.dispatch(input) {
            writeln!(
                transcript,
                "{}",
                atom_json::output_to_json(&emission.output)
            )?;
            framing::write_frame(output, emission.effect.frame.0, &emission.effect.frame.1)?;
        }
    }
    Ok(())
}

fn write_emissions<W: Write>(
    output: &mut W,
    emissions: Vec<EngineEmission<AtomOutput, (u8, Cbor)>>,
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
            eprintln!("taut-shape-tool atom client: I/O error: {error}");
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
    let mut versions = opts
        .stream_ids
        .iter()
        .map(|stream_id| (stream_id.clone(), opts.from))
        .collect::<BTreeMap<_, _>>();
    let mut active = versions.keys().cloned().collect::<BTreeSet<_>>();
    for (stream_id, version) in &versions {
        write_read(output, &opts.atom_id, stream_id, *version, opts.timeout_ms)?;
    }
    output.flush()?;

    while !active.is_empty() {
        let frame = match framing::read_frame(input)? {
            Ok(Some(frame)) => frame,
            Ok(None) => return final_line(transcript, "channel_eof"),
            Err(error) => {
                eprintln!("taut-shape-tool atom client: {}: {error}", error.code());
                return Ok(3);
            }
        };
        let tag = match AtomMsgType::from_wire(frame.tag as i64) {
            Ok(tag) => tag,
            Err(_) => {
                eprintln!(
                    "taut-shape-tool atom client: TAUT_SHAPE_UNKNOWN_TAG: unknown frame tag byte {}",
                    frame.tag
                );
                return Ok(3);
            }
        };
        match tag {
            AtomMsgType::ReadResponse => {
                let response = match AtomReadResponse::from_cbor(&frame.body) {
                    Ok(response) => response,
                    Err(error) => {
                        eprintln!("taut-shape-tool atom client: malformed read response: {error}");
                        return Ok(3);
                    }
                };
                writeln!(
                    transcript,
                    "{}",
                    atom_json::read_response_to_json(&response)
                )?;
                if !active.contains(&response.stream_id) {
                    eprintln!(
                        "taut-shape-tool atom client: response for inactive stream {:?}",
                        response.stream_id
                    );
                    return Ok(3);
                }
                versions.insert(response.stream_id.clone(), response.next_version.version);
                match response.state {
                    AtomState::Data => {
                        write_read(
                            output,
                            &opts.atom_id,
                            &response.stream_id,
                            response.next_version.version,
                            opts.timeout_ms,
                        )?;
                        output.flush()?;
                    }
                    AtomState::WouldBlock
                    | AtomState::Eof
                    | AtomState::Closed
                    | AtomState::Failed => {
                        active.remove(&response.stream_id);
                    }
                }
            }
            AtomMsgType::SetTimer => {
                if let Err(error) = AtomSetTimer::from_cbor(&frame.body) {
                    eprintln!("taut-shape-tool atom client: malformed set-timer: {error}");
                    return Ok(3);
                }
            }
            AtomMsgType::CancelTimer => {
                if let Err(error) = AtomCancelTimer::from_cbor(&frame.body) {
                    eprintln!("taut-shape-tool atom client: malformed cancel-timer: {error}");
                    return Ok(3);
                }
            }
            AtomMsgType::ProducerStop => {
                if let Err(error) = AtomProducerStop::from_cbor(&frame.body) {
                    eprintln!("taut-shape-tool atom client: malformed producer-stop: {error}");
                    return Ok(3);
                }
                if active.is_empty() {
                    break;
                }
            }
            AtomMsgType::Diagnostic => {
                let diagnostic = match AtomDiagnostic::from_cbor(&frame.body) {
                    Ok(diagnostic) => diagnostic,
                    Err(error) => {
                        eprintln!("taut-shape-tool atom client: malformed diagnostic: {error}");
                        return Ok(3);
                    }
                };
                writeln!(
                    transcript,
                    "{}",
                    atom_json::output_to_json(&AtomOutput::Diagnostic(diagnostic))
                )?;
            }
            AtomMsgType::Replace
            | AtomMsgType::Seal
            | AtomMsgType::Close
            | AtomMsgType::Read
            | AtomMsgType::EndStream
            | AtomMsgType::TimerExpired => {
                eprintln!(
                    "taut-shape-tool atom client: input-only tag {} on response channel",
                    tag.wire()
                );
                return Ok(3);
            }
        }
    }
    let final_state = if versions.is_empty() { "empty" } else { "done" };
    final_line(transcript, final_state)
}

fn write_read<W: Write>(
    output: &mut W,
    atom_id: &str,
    stream_id: &str,
    version: i64,
    timeout_ms: Option<i64>,
) -> io::Result<()> {
    let request = AtomReadRequest {
        atom_id: atom_id.into(),
        stream_id: stream_id.into(),
        version: Some(AtomVersion { version }),
        timeout_ms,
    };
    framing::write_frame(output, AtomMsgType::Read, &request.to_cbor())
}

fn final_line<T: Write>(transcript: &mut T, state: &str) -> io::Result<u8> {
    let mut line = BTreeMap::new();
    line.insert("state".to_string(), json::s(state));
    line.insert("type".to_string(), json::s("client_final"));
    writeln!(transcript, "{}", Json::Obj(line))?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_atom_tag_is_typed_before_dispatch() {
        let error = AtomAdapter::new(StopWhen::LastReader)
            .decode_input(Frame {
                tag: 99,
                body: Cbor::Map(Vec::new()),
            })
            .unwrap_err();
        assert_eq!(error.code, AdapterCode::UnknownTag);
        assert_eq!(error.code.as_str(), "TAUT_SHAPE_UNKNOWN_TAG");
    }
}
