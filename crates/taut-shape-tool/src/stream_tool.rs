//! Stream node/client CLI adapters over the shared framing and engine runtime.

use std::collections::{BTreeSet, HashSet};
use std::io::{self, BufWriter, Read, Write};

use taut_shape::cbor::Cbor;
use taut_shape::generated_stream::{
    StreamCancelTimer, StreamClose, StreamDiagnostic, StreamEndStream, StreamMsgType,
    StreamProducerStop, StreamPush, StreamReadRequest, StreamReadResponse, StreamSeal,
    StreamSetTimer, StreamState, StreamTimerExpired,
};
use taut_shape::{StopWhen, StreamInput, StreamNode, StreamOutput};

use crate::framing::{self, Frame};
use crate::json::{self, Json};
use crate::runtime::{
    AdapterCode, AdapterDiagnostic, EngineAdapter, EngineEffect, EngineEmission, EngineRuntime,
    TeardownAction, TimerAction,
};
use crate::script::{Pending, Script};
use crate::stream_json;

pub struct NodeOpts {
    pub stop_when: StopWhen,
    pub capacity_records: usize,
    pub script: Option<Script<StreamInput>>,
}

pub struct ClientOpts {
    pub stream_ids: Vec<String>,
    pub max_records: Option<i64>,
    pub max_bytes: Option<i64>,
    pub timeout_ms: Option<i64>,
    pub reconnect_dropped: bool,
    pub pause_stream_ids: HashSet<String>,
    pub resume_after_data: Option<usize>,
}

struct StreamAdapter {
    node: StreamNode,
}

impl StreamAdapter {
    fn new(capacity_records: usize, stop_when: StopWhen) -> Self {
        Self {
            node: StreamNode::new(capacity_records, stop_when),
        }
    }
}

impl EngineAdapter for StreamAdapter {
    type FrameIn = Frame;
    type Input = StreamInput;
    type Output = StreamOutput;
    type FrameOut = (u8, Cbor);

    fn shape(&self) -> &'static str {
        "stream"
    }

    fn decode_input(&mut self, frame: Frame) -> Result<StreamInput, AdapterDiagnostic> {
        let tag = StreamMsgType::from_wire(frame.tag as i64).map_err(|_| {
            AdapterDiagnostic::new(
                AdapterCode::UnknownTag,
                "stream",
                format!("unknown frame tag byte {}", frame.tag),
            )
        })?;
        let malformed = |error: taut_shape::cbor::DecodeError| {
            AdapterDiagnostic::new(
                AdapterCode::MalformedMessage,
                "stream",
                format!("malformed {tag:?} body: {error}"),
            )
        };
        match tag {
            StreamMsgType::Push => StreamPush::from_cbor(&frame.body)
                .map(StreamInput::Push)
                .map_err(malformed),
            StreamMsgType::Seal => StreamSeal::from_cbor(&frame.body)
                .map(StreamInput::Seal)
                .map_err(malformed),
            StreamMsgType::Close => StreamClose::from_cbor(&frame.body)
                .map(StreamInput::Close)
                .map_err(malformed),
            StreamMsgType::Read => StreamReadRequest::from_cbor(&frame.body)
                .map(StreamInput::Read)
                .map_err(malformed),
            StreamMsgType::EndStream => StreamEndStream::from_cbor(&frame.body)
                .map(StreamInput::EndStream)
                .map_err(malformed),
            StreamMsgType::TimerExpired => StreamTimerExpired::from_cbor(&frame.body)
                .map(StreamInput::TimerExpired)
                .map_err(malformed),
            StreamMsgType::ReadResponse
            | StreamMsgType::SetTimer
            | StreamMsgType::CancelTimer
            | StreamMsgType::ProducerStop
            | StreamMsgType::Diagnostic => Err(AdapterDiagnostic::new(
                AdapterCode::DirectionViolation,
                "stream",
                format!("output-only tag {} on the input channel", tag.wire()),
            )),
        }
    }

    fn dispatch(&mut self, input: StreamInput) -> Vec<StreamOutput> {
        self.node.handle(input)
    }

    fn encode_output(&self, output: &StreamOutput) -> EngineEffect<(u8, Cbor)> {
        let (tag, body, timer, teardown) = match output {
            StreamOutput::ReadResponse(message) => {
                (StreamMsgType::ReadResponse, message.to_cbor(), None, None)
            }
            StreamOutput::SetTimer(message) => (
                StreamMsgType::SetTimer,
                message.to_cbor(),
                Some(TimerAction::Set {
                    token: message.token,
                    delay_ms: message.ms,
                }),
                None,
            ),
            StreamOutput::CancelTimer(message) => (
                StreamMsgType::CancelTimer,
                message.to_cbor(),
                Some(TimerAction::Cancel {
                    token: message.token,
                }),
                None,
            ),
            StreamOutput::ProducerStop(message) => (
                StreamMsgType::ProducerStop,
                message.to_cbor(),
                None,
                Some(TeardownAction {
                    reason: format!("{:?}", message.reason),
                }),
            ),
            StreamOutput::Diagnostic(message) => {
                (StreamMsgType::Diagnostic, message.to_cbor(), None, None)
            }
        };
        EngineEffect {
            frame: (tag.wire() as u8, body),
            timer,
            teardown,
        }
    }

    fn finish(&mut self) -> Vec<StreamOutput> {
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
            eprintln!("taut-shape-tool stream node: I/O error: {error}");
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
    let mut runtime = EngineRuntime::new(StreamAdapter::new(opts.capacity_records, opts.stop_when));
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
                    eprintln!("taut-shape-tool stream node: {error}");
                    return Ok(3);
                }
            },
            Err(error) => {
                eprintln!("taut-shape-tool stream node: {}: {error}", error.code());
                return Ok(3);
            }
        }
    }
}

fn write_injected<W: Write, T: Write>(
    runtime: &mut EngineRuntime<StreamAdapter>,
    output: &mut W,
    transcript: &mut T,
    inputs: Vec<StreamInput>,
) -> io::Result<()> {
    for input in inputs {
        for emission in runtime.dispatch(input) {
            writeln!(
                transcript,
                "{}",
                stream_json::output_to_json(&emission.output)
            )?;
            framing::write_frame(output, emission.effect.frame.0, &emission.effect.frame.1)?;
        }
    }
    Ok(())
}

fn write_emissions<W: Write>(
    output: &mut W,
    emissions: Vec<EngineEmission<StreamOutput, (u8, Cbor)>>,
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
            eprintln!("taut-shape-tool stream client: I/O error: {error}");
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
    let mut paused = BTreeSet::new();
    let mut reconnected = BTreeSet::new();
    let mut data_responses = 0_usize;
    for stream_id in &opts.stream_ids {
        write_read(output, stream_id, &opts)?;
    }
    output.flush()?;

    while !active.is_empty() {
        let frame = match framing::read_frame(input)? {
            Ok(Some(frame)) => frame,
            Ok(None) => return final_line(transcript, "channel_eof"),
            Err(error) => {
                eprintln!("taut-shape-tool stream client: {}: {error}", error.code());
                return Ok(3);
            }
        };
        let tag = match StreamMsgType::from_wire(frame.tag as i64) {
            Ok(tag) => tag,
            Err(_) => {
                eprintln!(
                    "taut-shape-tool stream client: unknown frame tag byte {}",
                    frame.tag
                );
                return Ok(3);
            }
        };
        match tag {
            StreamMsgType::ReadResponse => {
                let response = match StreamReadResponse::from_cbor(&frame.body) {
                    Ok(response) => response,
                    Err(error) => {
                        eprintln!(
                            "taut-shape-tool stream client: malformed read response: {error}"
                        );
                        return Ok(3);
                    }
                };
                writeln!(
                    transcript,
                    "{}",
                    stream_json::read_response_to_json(&response)
                )?;
                if !active.contains(&response.stream_id) {
                    eprintln!(
                        "taut-shape-tool stream client: response for inactive stream {:?}",
                        response.stream_id
                    );
                    return Ok(3);
                }
                match response.state {
                    StreamState::Data => {
                        data_responses += 1;
                        if opts.pause_stream_ids.contains(&response.stream_id) {
                            paused.insert(response.stream_id.clone());
                        } else {
                            write_read(output, &response.stream_id, &opts)?;
                        }
                        if opts
                            .resume_after_data
                            .is_some_and(|limit| data_responses >= limit)
                        {
                            for stream_id in core::mem::take(&mut paused) {
                                write_read(output, &stream_id, &opts)?;
                            }
                        }
                        output.flush()?;
                    }
                    StreamState::Dropped
                        if opts.reconnect_dropped && !reconnected.contains(&response.stream_id) =>
                    {
                        reconnected.insert(response.stream_id.clone());
                        write_read(output, &response.stream_id, &opts)?;
                        output.flush()?;
                    }
                    StreamState::WouldBlock
                    | StreamState::Eof
                    | StreamState::Closed
                    | StreamState::Failed
                    | StreamState::Dropped => {
                        active.remove(&response.stream_id);
                    }
                }
            }
            StreamMsgType::SetTimer => {
                if let Err(error) = StreamSetTimer::from_cbor(&frame.body) {
                    eprintln!("taut-shape-tool stream client: malformed set-timer: {error}");
                    return Ok(3);
                }
            }
            StreamMsgType::CancelTimer => {
                if let Err(error) = StreamCancelTimer::from_cbor(&frame.body) {
                    eprintln!("taut-shape-tool stream client: malformed cancel-timer: {error}");
                    return Ok(3);
                }
            }
            StreamMsgType::ProducerStop => {
                if let Err(error) = StreamProducerStop::from_cbor(&frame.body) {
                    eprintln!("taut-shape-tool stream client: malformed producer-stop: {error}");
                    return Ok(3);
                }
            }
            StreamMsgType::Diagnostic => {
                let diagnostic = match StreamDiagnostic::from_cbor(&frame.body) {
                    Ok(diagnostic) => diagnostic,
                    Err(error) => {
                        eprintln!("taut-shape-tool stream client: malformed diagnostic: {error}");
                        return Ok(3);
                    }
                };
                writeln!(
                    transcript,
                    "{}",
                    stream_json::output_to_json(&StreamOutput::Diagnostic(diagnostic))
                )?;
            }
            StreamMsgType::Push
            | StreamMsgType::Seal
            | StreamMsgType::Close
            | StreamMsgType::Read
            | StreamMsgType::EndStream
            | StreamMsgType::TimerExpired => {
                eprintln!(
                    "taut-shape-tool stream client: input-only tag {} on response channel",
                    tag.wire()
                );
                return Ok(3);
            }
        }
    }
    final_line(transcript, "done")
}

fn write_read<W: Write>(output: &mut W, stream_id: &str, opts: &ClientOpts) -> io::Result<()> {
    let request = StreamReadRequest {
        stream_id: stream_id.into(),
        max_records: opts.max_records,
        max_bytes: opts.max_bytes,
        timeout_ms: opts.timeout_ms,
    };
    framing::write_frame(output, StreamMsgType::Read, &request.to_cbor())
}

fn final_line<T: Write>(transcript: &mut T, state: &str) -> io::Result<u8> {
    let mut line = std::collections::BTreeMap::new();
    line.insert("state".to_string(), json::s(state));
    line.insert("type".to_string(), json::s("client_final"));
    writeln!(transcript, "{}", Json::Obj(line))?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_stream_tag_is_typed_before_dispatch() {
        let error = StreamAdapter::new(64, StopWhen::LastReader)
            .decode_input(Frame {
                tag: 99,
                body: Cbor::Map(Vec::new()),
            })
            .unwrap_err();
        assert_eq!(error.code, AdapterCode::UnknownTag);
    }
}
