//! SWMR node/client CLI adapters over the shared framing and engine runtime.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufWriter, Read, Write};

use taut_shape::cbor::Cbor;
use taut_shape::generated_swmr::{
    SwmrCancelTimer, SwmrClose, SwmrCursor, SwmrDeltaPush, SwmrDiagnostic, SwmrEndStream,
    SwmrMsgType, SwmrProducerStop, SwmrReadRequest, SwmrReadResponse, SwmrReset, SwmrSeal,
    SwmrSetTimer, SwmrSnapshotPush, SwmrState, SwmrTimerExpired,
};
use taut_shape::{StopWhen, SwmrInput, SwmrNode, SwmrOutput, SwmrRecoveryPolicy};

use crate::framing::{self, Frame};
use crate::json::{self, Json};
use crate::runtime::{
    AdapterCode, AdapterDiagnostic, EngineAdapter, EngineEffect, EngineEmission, EngineRuntime,
    TeardownAction, TimerAction,
};
use crate::script::{Pending, Script};
use crate::swmr_json;

pub struct NodeOpts {
    pub stop_when: StopWhen,
    pub max_deltas: Option<usize>,
    pub script: Option<Script<SwmrInput>>,
    pub expire_profile: bool,
}

pub struct ClientOpts {
    pub swmr_id: String,
    pub stream_ids: Vec<String>,
    pub initial_cursor: Option<SwmrCursor>,
    pub timeout_ms: Option<i64>,
    pub expire_profile: bool,
}

struct SwmrAdapter {
    node: SwmrNode,
}

impl SwmrAdapter {
    fn new(stop_when: StopWhen, max_deltas: Option<usize>, expire_profile: bool) -> Self {
        Self {
            node: SwmrNode::with_recovery(
                stop_when,
                max_deltas,
                if expire_profile {
                    SwmrRecoveryPolicy::Expire
                } else {
                    SwmrRecoveryPolicy::Repair
                },
            ),
        }
    }
}

impl EngineAdapter for SwmrAdapter {
    type FrameIn = Frame;
    type Input = SwmrInput;
    type Output = SwmrOutput;
    type FrameOut = (u8, Cbor);

    fn shape(&self) -> &'static str {
        "swmr"
    }

    fn decode_input(&mut self, frame: Frame) -> Result<SwmrInput, AdapterDiagnostic> {
        let tag = SwmrMsgType::from_wire(frame.tag as i64).map_err(|_| {
            AdapterDiagnostic::new(
                AdapterCode::UnknownTag,
                "swmr",
                format!("unknown frame tag byte {}", frame.tag),
            )
        })?;
        let malformed = |error: taut_shape::cbor::DecodeError| {
            AdapterDiagnostic::new(
                AdapterCode::MalformedMessage,
                "swmr",
                format!("malformed {tag:?} body: {error}"),
            )
        };
        match tag {
            SwmrMsgType::SnapshotPush => SwmrSnapshotPush::from_cbor(&frame.body)
                .map(SwmrInput::SnapshotPush)
                .map_err(malformed),
            SwmrMsgType::DeltaPush => SwmrDeltaPush::from_cbor(&frame.body)
                .map(SwmrInput::DeltaPush)
                .map_err(malformed),
            SwmrMsgType::Reset => SwmrReset::from_cbor(&frame.body)
                .map(SwmrInput::Reset)
                .map_err(malformed),
            SwmrMsgType::Seal => SwmrSeal::from_cbor(&frame.body)
                .map(SwmrInput::Seal)
                .map_err(malformed),
            SwmrMsgType::Close => SwmrClose::from_cbor(&frame.body)
                .map(SwmrInput::Close)
                .map_err(malformed),
            SwmrMsgType::Read => SwmrReadRequest::from_cbor(&frame.body)
                .map(SwmrInput::Read)
                .map_err(malformed),
            SwmrMsgType::EndStream => SwmrEndStream::from_cbor(&frame.body)
                .map(SwmrInput::EndStream)
                .map_err(malformed),
            SwmrMsgType::TimerExpired => SwmrTimerExpired::from_cbor(&frame.body)
                .map(SwmrInput::TimerExpired)
                .map_err(malformed),
            SwmrMsgType::ReadResponse
            | SwmrMsgType::SetTimer
            | SwmrMsgType::CancelTimer
            | SwmrMsgType::ProducerStop
            | SwmrMsgType::Diagnostic => Err(AdapterDiagnostic::new(
                AdapterCode::DirectionViolation,
                "swmr",
                format!("output-only tag {} on the input channel", tag.wire()),
            )),
        }
    }

    fn dispatch(&mut self, input: SwmrInput) -> Vec<SwmrOutput> {
        self.node.handle(input)
    }

    fn encode_output(&self, output: &SwmrOutput) -> EngineEffect<(u8, Cbor)> {
        let (tag, body, timer, teardown) = match output {
            SwmrOutput::ReadResponse(message) => {
                (SwmrMsgType::ReadResponse, message.to_cbor(), None, None)
            }
            SwmrOutput::SetTimer(message) => (
                SwmrMsgType::SetTimer,
                message.to_cbor(),
                Some(TimerAction::Set {
                    token: message.token,
                    delay_ms: message.ms,
                }),
                None,
            ),
            SwmrOutput::CancelTimer(message) => (
                SwmrMsgType::CancelTimer,
                message.to_cbor(),
                Some(TimerAction::Cancel {
                    token: message.token,
                }),
                None,
            ),
            SwmrOutput::ProducerStop(message) => (
                SwmrMsgType::ProducerStop,
                message.to_cbor(),
                None,
                Some(TeardownAction {
                    reason: format!("{:?}", message.reason),
                }),
            ),
            SwmrOutput::Diagnostic(message) => {
                (SwmrMsgType::Diagnostic, message.to_cbor(), None, None)
            }
        };
        EngineEffect {
            frame: (tag.wire() as u8, body),
            timer,
            teardown,
        }
    }

    fn finish(&mut self) -> Vec<SwmrOutput> {
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
            eprintln!("taut-shape-tool swmr node: I/O error: {error}");
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
    let mut runtime = EngineRuntime::new(SwmrAdapter::new(
        opts.stop_when,
        opts.max_deltas,
        opts.expire_profile,
    ));
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
                    eprintln!("taut-shape-tool swmr node: {error}");
                    return Ok(3);
                }
            },
            Err(error) => {
                eprintln!("taut-shape-tool swmr node: {}: {error}", error.code());
                return Ok(3);
            }
        }
    }
}

fn write_injected<W: Write, T: Write>(
    runtime: &mut EngineRuntime<SwmrAdapter>,
    output: &mut W,
    transcript: &mut T,
    inputs: Vec<SwmrInput>,
) -> io::Result<()> {
    for input in inputs {
        for emission in runtime.dispatch(input) {
            writeln!(
                transcript,
                "{}",
                swmr_json::output_to_json(&emission.output)
            )?;
            framing::write_frame(output, emission.effect.frame.0, &emission.effect.frame.1)?;
        }
    }
    Ok(())
}

fn write_emissions<W: Write>(
    output: &mut W,
    emissions: Vec<EngineEmission<SwmrOutput, (u8, Cbor)>>,
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
            eprintln!("taut-shape-tool swmr client: I/O error: {error}");
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
    let mut active = opts.stream_ids.iter().cloned().collect::<BTreeSet<_>>();
    let mut cursors = opts
        .stream_ids
        .iter()
        .cloned()
        .map(|stream_id| (stream_id, opts.initial_cursor.clone()))
        .collect::<BTreeMap<_, _>>();
    for stream_id in &opts.stream_ids {
        write_read(output, stream_id, cursors[stream_id].as_ref(), &opts)?;
    }
    output.flush()?;

    while !active.is_empty() {
        let frame = match framing::read_frame(input)? {
            Ok(Some(frame)) => frame,
            Ok(None) => return final_line(transcript, "channel_eof"),
            Err(error) => {
                eprintln!("taut-shape-tool swmr client: {}: {error}", error.code());
                return Ok(3);
            }
        };
        let tag = match SwmrMsgType::from_wire(frame.tag as i64) {
            Ok(tag) => tag,
            Err(_) => {
                eprintln!(
                    "taut-shape-tool swmr client: unknown frame tag byte {}",
                    frame.tag
                );
                return Ok(3);
            }
        };
        match tag {
            SwmrMsgType::ReadResponse => {
                let response = match SwmrReadResponse::from_cbor(&frame.body) {
                    Ok(response) => response,
                    Err(error) => {
                        eprintln!("taut-shape-tool swmr client: malformed read response: {error}");
                        return Ok(3);
                    }
                };
                if !active.contains(&response.stream_id) {
                    eprintln!(
                        "taut-shape-tool swmr client: response for inactive stream {:?}",
                        response.stream_id
                    );
                    return Ok(3);
                }
                if opts.expire_profile && response.state == SwmrState::Reset {
                    let mut refresh = BTreeMap::new();
                    refresh.insert("type".to_string(), json::s("refresh_required"));
                    refresh.insert("swmr_id".to_string(), json::s(response.swmr_id));
                    refresh.insert("stream_id".to_string(), json::s(response.stream_id.clone()));
                    refresh.insert(
                        "reason".to_string(),
                        json::s(match response.reset_reason {
                            Some(
                                taut_shape::generated_swmr::SwmrResetReason::RetentionExceeded,
                            ) => "retention_expired",
                            Some(taut_shape::generated_swmr::SwmrResetReason::InvalidResumeSeq) => {
                                "invalid_cursor"
                            }
                            Some(
                                taut_shape::generated_swmr::SwmrResetReason::ProducerRequested,
                            )
                            | None => "source_changed",
                        }),
                    );
                    writeln!(transcript, "{}", Json::Obj(refresh))?;
                    active.remove(&response.stream_id);
                    continue;
                }
                writeln!(
                    transcript,
                    "{}",
                    swmr_json::read_response_to_json(&response)
                )?;
                match response.state {
                    SwmrState::Data | SwmrState::Reset => {
                        cursors.insert(response.stream_id.clone(), response.next_cursor.clone());
                        write_read(
                            output,
                            &response.stream_id,
                            cursors[&response.stream_id].as_ref(),
                            &opts,
                        )?;
                        output.flush()?;
                    }
                    SwmrState::WouldBlock
                    | SwmrState::Eof
                    | SwmrState::Closed
                    | SwmrState::Failed => {
                        active.remove(&response.stream_id);
                    }
                }
            }
            SwmrMsgType::SetTimer => {
                if let Err(error) = SwmrSetTimer::from_cbor(&frame.body) {
                    eprintln!("taut-shape-tool swmr client: malformed set-timer: {error}");
                    return Ok(3);
                }
            }
            SwmrMsgType::CancelTimer => {
                if let Err(error) = SwmrCancelTimer::from_cbor(&frame.body) {
                    eprintln!("taut-shape-tool swmr client: malformed cancel-timer: {error}");
                    return Ok(3);
                }
            }
            SwmrMsgType::ProducerStop => {
                if let Err(error) = SwmrProducerStop::from_cbor(&frame.body) {
                    eprintln!("taut-shape-tool swmr client: malformed producer-stop: {error}");
                    return Ok(3);
                }
            }
            SwmrMsgType::Diagnostic => {
                let diagnostic = match SwmrDiagnostic::from_cbor(&frame.body) {
                    Ok(diagnostic) => diagnostic,
                    Err(error) => {
                        eprintln!("taut-shape-tool swmr client: malformed diagnostic: {error}");
                        return Ok(3);
                    }
                };
                writeln!(
                    transcript,
                    "{}",
                    swmr_json::output_to_json(&SwmrOutput::Diagnostic(diagnostic))
                )?;
            }
            SwmrMsgType::SnapshotPush
            | SwmrMsgType::DeltaPush
            | SwmrMsgType::Reset
            | SwmrMsgType::Seal
            | SwmrMsgType::Close
            | SwmrMsgType::Read
            | SwmrMsgType::EndStream
            | SwmrMsgType::TimerExpired => {
                eprintln!(
                    "taut-shape-tool swmr client: input-only tag {} on response channel",
                    tag.wire()
                );
                return Ok(3);
            }
        }
    }
    final_line(transcript, "done")
}

fn write_read<W: Write>(
    output: &mut W,
    stream_id: &str,
    cursor: Option<&SwmrCursor>,
    opts: &ClientOpts,
) -> io::Result<()> {
    let request = SwmrReadRequest {
        swmr_id: opts.swmr_id.clone(),
        stream_id: stream_id.into(),
        cursor: cursor.cloned(),
        timeout_ms: opts.timeout_ms,
    };
    framing::write_frame(output, SwmrMsgType::Read, &request.to_cbor())
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
    fn unknown_swmr_tag_is_typed_before_dispatch() {
        let error = SwmrAdapter::new(StopWhen::LastReader, None, false)
            .decode_input(Frame {
                tag: 99,
                body: Cbor::Map(Vec::new()),
            })
            .unwrap_err();
        assert_eq!(error.code, AdapterCode::UnknownTag);
    }
}
