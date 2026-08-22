//! CRDT jsoncodec-form bridge plus complete mailbox/convergence corpus gates.

use std::collections::BTreeMap;

use taut_shape::generated_crdt::{
    CrdtApply, CrdtBootstrap, CrdtClock, CrdtClockEntry, CrdtClose, CrdtDiagCode, CrdtDiagnostic,
    CrdtError, CrdtErrorCode, CrdtInstallBootstrap, CrdtOp, CrdtReadRequest, CrdtReadResponse,
    CrdtSeal, CrdtSeverity, CrdtState,
};
use taut_shape::{CrdtInput, CrdtOutput};

use crate::json::{self, Json};

pub fn input_from_json(value: &Json) -> Result<CrdtInput, String> {
    match required_string(value, "type")?.as_str() {
        "apply" => Ok(CrdtInput::Apply(CrdtApply {
            op: op_from_json(required(value, "op")?)?,
        })),
        "install_bootstrap" => Ok(CrdtInput::InstallBootstrap(CrdtInstallBootstrap {
            bootstrap: bootstrap_from_json(required(value, "bootstrap")?)?,
        })),
        "seal" => Ok(CrdtInput::Seal(CrdtSeal {})),
        "close" => Ok(CrdtInput::Close(CrdtClose {
            error: error_from_json(value.get("error"))?,
        })),
        "read" => Ok(CrdtInput::Read(CrdtReadRequest {
            crdt_id: required_string(value, "crdt_id")?,
            stream_id: required_string(value, "stream_id")?,
            cursor: optional(value, "cursor").map(clock_from_json).transpose()?,
        })),
        kind => Err(format!("unknown CRDT input {kind:?}")),
    }
}

pub fn op_from_json(value: &Json) -> Result<CrdtOp, String> {
    Ok(CrdtOp {
        origin: required_string(value, "origin")?,
        seq: required_i64(value, "seq")?,
        deps: clock_from_json(required(value, "deps")?)?,
        payload: required(value, "payload")?
            .as_str()
            .and_then(json::base64_decode)
            .ok_or_else(|| "invalid op payload".to_string())?,
    })
}

pub fn bootstrap_from_json(value: &Json) -> Result<CrdtBootstrap, String> {
    Ok(CrdtBootstrap {
        clock: clock_from_json(required(value, "clock")?)?,
        state: required(value, "state")?
            .as_str()
            .and_then(json::base64_decode)
            .ok_or_else(|| "invalid bootstrap state".to_string())?,
    })
}

pub fn clock_from_json(value: &Json) -> Result<CrdtClock, String> {
    Ok(CrdtClock {
        entries: required(value, "entries")?
            .as_arr()
            .ok_or_else(|| "clock entries must be an array".to_string())?
            .iter()
            .map(|entry| {
                Ok(CrdtClockEntry {
                    origin: required_string(entry, "origin")?,
                    seq: required_i64(entry, "seq")?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
    })
}

fn error_from_json(value: Option<&Json>) -> Result<Option<CrdtError>, String> {
    let Some(value) = value.filter(|value| !matches!(value, Json::Null)) else {
        return Ok(None);
    };
    let code = match required_string(value, "code")?.as_str() {
        "unknown_crdt" => CrdtErrorCode::UnknownCrdt,
        "producer_error" => CrdtErrorCode::ProducerError,
        "internal" => CrdtErrorCode::Internal,
        code => return Err(format!("unknown CRDT error code {code:?}")),
    };
    Ok(Some(CrdtError {
        code,
        message: optional(value, "message")
            .and_then(Json::as_str)
            .map(str::to_string),
    }))
}

pub fn output_to_json(output: &CrdtOutput) -> Json {
    match output {
        CrdtOutput::ReadResponse(response) => read_response_to_json(response),
        CrdtOutput::Diagnostic(message) => diagnostic_to_json(message),
    }
}

pub fn read_response_to_json(response: &CrdtReadResponse) -> Json {
    obj(vec![
        ("type", json::s("read_response")),
        ("crdt_id", json::s(response.crdt_id.clone())),
        ("stream_id", json::s(response.stream_id.clone())),
        (
            "bootstrap",
            response
                .bootstrap
                .as_ref()
                .map(bootstrap_to_json)
                .unwrap_or(Json::Null),
        ),
        (
            "ops",
            Json::Arr(response.ops.iter().map(op_to_json).collect()),
        ),
        ("next_cursor", clock_to_json(&response.next_cursor)),
        ("state", json::s(state_name(response.state))),
        (
            "error",
            response
                .error
                .as_ref()
                .map(error_to_json)
                .unwrap_or(Json::Null),
        ),
    ])
}

pub fn diagnostic_to_json(message: &CrdtDiagnostic) -> Json {
    obj(vec![
        ("type", json::s("diagnostic")),
        (
            "severity",
            json::s(if message.severity == CrdtSeverity::Warn {
                "warn"
            } else {
                "error"
            }),
        ),
        ("code", json::s(diag_name(message.code))),
        (
            "origin",
            message.origin.clone().map(json::s).unwrap_or(Json::Null),
        ),
        ("seq", message.seq.map(json::i64_str).unwrap_or(Json::Null)),
    ])
}

pub fn op_to_json(op: &CrdtOp) -> Json {
    obj(vec![
        ("origin", json::s(op.origin.clone())),
        ("seq", json::i64_str(op.seq)),
        ("deps", clock_to_json(&op.deps)),
        ("payload", json::s(json::base64_encode(&op.payload))),
    ])
}

pub fn bootstrap_to_json(value: &CrdtBootstrap) -> Json {
    obj(vec![
        ("clock", clock_to_json(&value.clock)),
        ("state", json::s(json::base64_encode(&value.state))),
    ])
}

pub fn clock_to_json(clock: &CrdtClock) -> Json {
    obj(vec![(
        "entries",
        Json::Arr(
            clock
                .entries
                .iter()
                .map(|entry| {
                    obj(vec![
                        ("origin", json::s(entry.origin.clone())),
                        ("seq", json::i64_str(entry.seq)),
                    ])
                })
                .collect(),
        ),
    )])
}

fn error_to_json(error: &CrdtError) -> Json {
    obj(vec![
        (
            "code",
            json::s(match error.code {
                CrdtErrorCode::UnknownCrdt => "unknown_crdt",
                CrdtErrorCode::ProducerError => "producer_error",
                CrdtErrorCode::Internal => "internal",
            }),
        ),
        (
            "message",
            error.message.clone().map(json::s).unwrap_or(Json::Null),
        ),
    ])
}

pub fn state_name(state: CrdtState) -> &'static str {
    match state {
        CrdtState::Data => "data",
        CrdtState::Empty => "empty",
        CrdtState::Eof => "eof",
        CrdtState::Closed => "closed",
        CrdtState::Failed => "failed",
        CrdtState::BootstrapRequired => "bootstrap_required",
        CrdtState::InvalidCursor => "invalid_cursor",
    }
}

pub fn diag_name(code: CrdtDiagCode) -> &'static str {
    match code {
        CrdtDiagCode::ApplyAfterTerminal => "apply_after_terminal",
        CrdtDiagCode::InvalidOperation => "invalid_operation",
        CrdtDiagCode::Equivocation => "equivocation",
        CrdtDiagCode::BootstrapConflict => "bootstrap_conflict",
        CrdtDiagCode::PendingBoundExceeded => "pending_bound_exceeded",
    }
}

fn required<'a>(value: &'a Json, field: &str) -> Result<&'a Json, String> {
    value.get(field).ok_or_else(|| format!("missing `{field}`"))
}
fn required_string(value: &Json, field: &str) -> Result<String, String> {
    required(value, field)?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("`{field}` must be a string"))
}
fn required_i64(value: &Json, field: &str) -> Result<i64, String> {
    required(value, field)?
        .as_i64()
        .ok_or_else(|| format!("`{field}` must be an i64"))
}
fn optional<'a>(value: &'a Json, field: &str) -> Option<&'a Json> {
    value.get(field).filter(|item| !matches!(item, Json::Null))
}
fn obj(pairs: Vec<(&str, Json)>) -> Json {
    Json::Obj(
        pairs
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect::<BTreeMap<_, _>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use taut_shape::crdt::text::project_text;
    use taut_shape::CrdtNode;

    #[test]
    fn complete_crdt_mailbox_corpus_matches() {
        let document =
            json::parse(include_str!("../../../../taut-shape/corpus/crdt.v1.json")).unwrap();
        let vectors = document.get("vectors").and_then(Json::as_arr).unwrap();
        assert_eq!(vectors.len(), 15);
        for vector in vectors {
            let max_pending = vector
                .get("node")
                .and_then(|node| node.get("max_pending"))
                .and_then(Json::as_i64)
                .unwrap_or(1024) as usize;
            let mut node = CrdtNode::new(max_pending);
            for step in vector.get("steps").and_then(Json::as_arr).unwrap() {
                let actual = Json::Arr(
                    node.handle(input_from_json(step.get("in").unwrap()).unwrap())
                        .iter()
                        .map(output_to_json)
                        .collect(),
                );
                assert_eq!(actual, step.get("out").unwrap().clone());
            }
        }
    }

    #[test]
    fn all_generic_and_text_replica_orders_converge() {
        check_convergence(
            include_str!("../../../../taut-shape/corpus/crdt.convergence.v1.json"),
            false,
        );
        check_convergence(
            include_str!("../../../../taut-shape/corpus/text_crdt.profile.v1.json"),
            true,
        );
    }

    fn check_convergence(source: &str, include_text: bool) {
        let document = json::parse(source).unwrap();
        for scenario in document.get("scenarios").and_then(Json::as_arr).unwrap() {
            let ops: Vec<CrdtOp> = scenario
                .get("ops")
                .and_then(Json::as_arr)
                .unwrap()
                .iter()
                .map(|value| op_from_json(value).unwrap())
                .collect();
            for replica in scenario.get("replicas").and_then(Json::as_arr).unwrap() {
                let mut node = CrdtNode::new(
                    scenario.get("max_pending").and_then(Json::as_i64).unwrap() as usize,
                );
                let mut diagnostics = BTreeSet::new();
                if let Some(value) = optional(scenario, "bootstrap") {
                    collect(
                        &mut diagnostics,
                        node.handle(CrdtInput::InstallBootstrap(CrdtInstallBootstrap {
                            bootstrap: bootstrap_from_json(value).unwrap(),
                        })),
                    );
                }
                for index in replica.get("order").and_then(Json::as_arr).unwrap() {
                    collect(
                        &mut diagnostics,
                        node.handle(CrdtInput::Apply(CrdtApply {
                            op: ops[index.as_i64().unwrap() as usize].clone(),
                        })),
                    );
                }
                let mut pairs = vec![
                    ("clock", clock_to_json(&node.clock())),
                    (
                        "ops",
                        Json::Arr(node.operations().iter().map(op_to_json).collect()),
                    ),
                    (
                        "diagnostics",
                        Json::Arr(
                            diagnostics
                                .into_iter()
                                .map(|(code, origin, seq)| {
                                    Json::Arr(vec![
                                        json::s(code),
                                        origin.map(json::s).unwrap_or(Json::Null),
                                        seq.map(json::i64_str).unwrap_or(Json::Null),
                                    ])
                                })
                                .collect(),
                        ),
                    ),
                ];
                if include_text {
                    let projection = project_text(&node);
                    pairs.push((
                        "projection",
                        obj(vec![
                            ("text", json::s(projection.text)),
                            (
                                "diagnostics",
                                Json::Arr(
                                    projection.diagnostics.into_iter().map(json::s).collect(),
                                ),
                            ),
                        ]),
                    ));
                }
                assert_eq!(obj(pairs), scenario.get("expect").unwrap().clone());
            }
        }
    }

    fn collect(
        target: &mut BTreeSet<(String, Option<String>, Option<i64>)>,
        outputs: Vec<CrdtOutput>,
    ) {
        for output in outputs {
            if let CrdtOutput::Diagnostic(value) = output {
                target.insert((diag_name(value.code).to_string(), value.origin, value.seq));
            }
        }
    }
}
