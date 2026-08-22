//! CRDT node/client CLI adapters over the shared engine runtime and framing.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufWriter, Read, Write};

use taut_shape::cbor::Cbor;
use taut_shape::crdt::text::project_text;
use taut_shape::generated_crdt::{
    CrdtApply, CrdtDiagnostic, CrdtInstallBootstrap, CrdtMsgType, CrdtReadRequest,
    CrdtReadResponse, CrdtState,
};
use taut_shape::{CrdtInput, CrdtNode, CrdtOutput};

use crate::crdt_json;
use crate::framing::{self, Frame};
use crate::json::{self, Json};
use crate::runtime::{
    AdapterCode, AdapterDiagnostic, EngineAdapter, EngineEffect, EngineEmission, EngineRuntime,
};
use crate::script::{Pending, Script};

pub struct NodeOpts {
    pub max_pending: usize,
    pub script: Option<Script<CrdtInput>>,
}

pub struct ClientOpts {
    pub crdt_id: String,
    pub stream_id: String,
    pub replica_script: String,
    pub text_profile: bool,
}

struct CrdtAdapter {
    node: CrdtNode,
}

impl EngineAdapter for CrdtAdapter {
    type FrameIn = Frame;
    type Input = CrdtInput;
    type Output = CrdtOutput;
    type FrameOut = (u8, Cbor);

    fn shape(&self) -> &'static str {
        "crdt"
    }
    fn decode_input(&mut self, frame: Frame) -> Result<CrdtInput, AdapterDiagnostic> {
        let tag = CrdtMsgType::from_wire(frame.tag as i64).map_err(|_| {
            AdapterDiagnostic::new(
                AdapterCode::UnknownTag,
                "crdt",
                format!("unknown frame tag byte {}", frame.tag),
            )
        })?;
        let malformed = |error: taut_shape::cbor::DecodeError| {
            AdapterDiagnostic::new(
                AdapterCode::MalformedMessage,
                "crdt",
                format!("malformed {tag:?} body: {error}"),
            )
        };
        match tag {
            CrdtMsgType::Apply => CrdtApply::from_cbor(&frame.body)
                .map(CrdtInput::Apply)
                .map_err(malformed),
            CrdtMsgType::InstallBootstrap => CrdtInstallBootstrap::from_cbor(&frame.body)
                .map(CrdtInput::InstallBootstrap)
                .map_err(malformed),
            CrdtMsgType::Seal => taut_shape::generated_crdt::CrdtSeal::from_cbor(&frame.body)
                .map(CrdtInput::Seal)
                .map_err(malformed),
            CrdtMsgType::Close => taut_shape::generated_crdt::CrdtClose::from_cbor(&frame.body)
                .map(CrdtInput::Close)
                .map_err(malformed),
            CrdtMsgType::Read => CrdtReadRequest::from_cbor(&frame.body)
                .map(CrdtInput::Read)
                .map_err(malformed),
            CrdtMsgType::ReadResponse | CrdtMsgType::Diagnostic => Err(AdapterDiagnostic::new(
                AdapterCode::DirectionViolation,
                "crdt",
                format!("output-only tag {} on input channel", tag.wire()),
            )),
        }
    }
    fn dispatch(&mut self, input: CrdtInput) -> Vec<CrdtOutput> {
        self.node.handle(input)
    }
    fn encode_output(&self, output: &CrdtOutput) -> EngineEffect<(u8, Cbor)> {
        let (tag, body) = match output {
            CrdtOutput::ReadResponse(message) => (CrdtMsgType::ReadResponse, message.to_cbor()),
            CrdtOutput::Diagnostic(message) => (CrdtMsgType::Diagnostic, message.to_cbor()),
        };
        EngineEffect {
            frame: (tag.wire() as u8, body),
            timer: None,
            teardown: None,
        }
    }
    fn finish(&mut self) -> Vec<CrdtOutput> {
        Vec::new()
    }
}

pub fn run_node(opts: NodeOpts) -> u8 {
    match drive_node(
        &mut io::stdin().lock(),
        &mut BufWriter::new(io::stdout().lock()),
        &mut io::stderr().lock(),
        opts,
    ) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("taut-shape-tool CRDT node: I/O error: {error}");
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
    let mut runtime = EngineRuntime::new(CrdtAdapter {
        node: CrdtNode::new(opts.max_pending),
    });
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
                    eprintln!("taut-shape-tool CRDT node: {error}");
                    return Ok(3);
                }
            },
            Err(error) => {
                eprintln!("taut-shape-tool CRDT node: {}: {error}", error.code());
                return Ok(3);
            }
        }
    }
}

fn write_injected<W: Write, T: Write>(
    runtime: &mut EngineRuntime<CrdtAdapter>,
    output: &mut W,
    transcript: &mut T,
    inputs: Vec<CrdtInput>,
) -> io::Result<()> {
    for input in inputs {
        for emission in runtime.dispatch(input) {
            writeln!(
                transcript,
                "{}",
                crdt_json::output_to_json(&emission.output)
            )?;
            framing::write_frame(output, emission.effect.frame.0, &emission.effect.frame.1)?;
        }
    }
    Ok(())
}
fn write_emissions<W: Write>(
    output: &mut W,
    emissions: Vec<EngineEmission<CrdtOutput, (u8, Cbor)>>,
) -> io::Result<()> {
    for emission in emissions {
        framing::write_frame(output, emission.effect.frame.0, &emission.effect.frame.1)?;
    }
    Ok(())
}

pub fn run_client(opts: ClientOpts) -> u8 {
    match drive_client(
        &mut io::stdin().lock(),
        &mut BufWriter::new(io::stdout().lock()),
        &mut io::stderr().lock(),
        opts,
    ) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("taut-shape-tool CRDT client: I/O error: {error}");
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
    let source = match std::fs::read_to_string(&opts.replica_script) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("CRDT replica script: {error}");
            return Ok(2);
        }
    };
    let document = match json::parse(&source) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("CRDT replica script: {error}");
            return Ok(2);
        }
    };
    let Some(inputs) = document.as_arr() else {
        eprintln!("CRDT replica script must be an array");
        return Ok(2);
    };
    let mut node = CrdtNode::default();
    let mut diagnostics: BTreeSet<(String, Option<String>, Option<i64>)> = BTreeSet::new();
    for raw in inputs {
        let decoded = match crdt_json::input_from_json(raw) {
            Ok(value @ (CrdtInput::Apply(_) | CrdtInput::InstallBootstrap(_))) => value,
            Ok(_) => {
                eprintln!("CRDT replica script accepts only apply/bootstrap");
                return Ok(2);
            }
            Err(error) => {
                eprintln!("CRDT replica script: {error}");
                return Ok(2);
            }
        };
        collect(&mut diagnostics, node.handle(decoded.clone()));
        write_input(output, &decoded)?;
    }
    output.flush()?;
    send_read(output, &opts, &node)?;
    loop {
        let frame = match framing::read_frame(input)? {
            Ok(Some(value)) => value,
            Ok(None) => break,
            Err(error) => {
                eprintln!("taut-shape-tool CRDT client: {}: {error}", error.code());
                return Ok(3);
            }
        };
        let tag = match CrdtMsgType::from_wire(frame.tag as i64) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("taut-shape-tool CRDT client: unknown tag {}", frame.tag);
                return Ok(3);
            }
        };
        match tag {
            CrdtMsgType::Diagnostic => match CrdtDiagnostic::from_cbor(&frame.body) {
                Ok(value) => {
                    writeln!(transcript, "{}", crdt_json::diagnostic_to_json(&value))?;
                    collect(&mut diagnostics, vec![CrdtOutput::Diagnostic(value)]);
                }
                Err(error) => {
                    eprintln!("malformed CRDT diagnostic: {error}");
                    return Ok(3);
                }
            },
            CrdtMsgType::ReadResponse => {
                let response = match CrdtReadResponse::from_cbor(&frame.body) {
                    Ok(value) => value,
                    Err(error) => {
                        eprintln!("malformed CRDT response: {error}");
                        return Ok(3);
                    }
                };
                writeln!(
                    transcript,
                    "{}",
                    crdt_json::read_response_to_json(&response)
                )?;
                if let Some(bootstrap) = response.bootstrap.clone() {
                    collect(
                        &mut diagnostics,
                        node.handle(CrdtInput::InstallBootstrap(CrdtInstallBootstrap {
                            bootstrap,
                        })),
                    );
                }
                for op in response.ops {
                    collect(
                        &mut diagnostics,
                        node.handle(CrdtInput::Apply(CrdtApply { op })),
                    );
                }
                if matches!(
                    response.state,
                    CrdtState::Data | CrdtState::BootstrapRequired
                ) {
                    send_read(output, &opts, &node)?;
                    continue;
                }
                break;
            }
            _ => {
                eprintln!(
                    "taut-shape-tool CRDT client: input-only response tag {}",
                    tag.wire()
                );
                return Ok(3);
            }
        }
    }
    let mut pairs = vec![
        ("type", json::s("replica_final")),
        ("clock", crdt_json::clock_to_json(&node.clock())),
        (
            "ops",
            Json::Arr(
                node.operations()
                    .iter()
                    .map(crdt_json::op_to_json)
                    .collect(),
            ),
        ),
        (
            "diagnostics",
            Json::Arr(
                diagnostics
                    .into_iter()
                    .map(|(code, origin, seq)| {
                        obj(vec![
                            ("code", json::s(code)),
                            ("origin", origin.map(json::s).unwrap_or(Json::Null)),
                            ("seq", seq.map(json::i64_str).unwrap_or(Json::Null)),
                        ])
                    })
                    .collect(),
            ),
        ),
    ];
    if opts.text_profile {
        let projection = project_text(&node);
        pairs.push(("text", json::s(projection.text)));
        pairs.push((
            "text_diagnostics",
            Json::Arr(projection.diagnostics.into_iter().map(json::s).collect()),
        ));
    }
    writeln!(transcript, "{}", obj(pairs))?;
    Ok(0)
}

fn send_read<W: Write>(output: &mut W, opts: &ClientOpts, node: &CrdtNode) -> io::Result<()> {
    let request = CrdtReadRequest {
        crdt_id: opts.crdt_id.clone(),
        stream_id: opts.stream_id.clone(),
        cursor: Some(node.clock()),
    };
    framing::write_frame(output, CrdtMsgType::Read, &request.to_cbor())?;
    output.flush()
}
fn write_input<W: Write>(output: &mut W, input: &CrdtInput) -> io::Result<()> {
    match input {
        CrdtInput::Apply(value) => {
            framing::write_frame(output, CrdtMsgType::Apply, &value.to_cbor())
        }
        CrdtInput::InstallBootstrap(value) => {
            framing::write_frame(output, CrdtMsgType::InstallBootstrap, &value.to_cbor())
        }
        CrdtInput::Seal(value) => framing::write_frame(output, CrdtMsgType::Seal, &value.to_cbor()),
        CrdtInput::Close(value) => {
            framing::write_frame(output, CrdtMsgType::Close, &value.to_cbor())
        }
        CrdtInput::Read(value) => framing::write_frame(output, CrdtMsgType::Read, &value.to_cbor()),
    }
}
fn collect(target: &mut BTreeSet<(String, Option<String>, Option<i64>)>, outputs: Vec<CrdtOutput>) {
    for output in outputs {
        if let CrdtOutput::Diagnostic(value) = output {
            target.insert((
                crdt_json::diag_name(value.code).to_string(),
                value.origin,
                value.seq,
            ));
        }
    }
}
fn obj(pairs: Vec<(&str, Json)>) -> Json {
    Json::Obj(
        pairs
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect::<BTreeMap<_, _>>(),
    )
}
