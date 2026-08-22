//! Atom jsoncodec-form bridge for scripts, transcripts, and corpus tests.

use std::collections::BTreeMap;

use taut_shape::generated_atom::{
    AtomCancelTimer, AtomClose, AtomDiagCode, AtomDiagnostic, AtomEndStream, AtomError,
    AtomErrorCode, AtomProducerStop, AtomReadRequest, AtomReadResponse, AtomReplace, AtomSeal,
    AtomSetTimer, AtomSeverity, AtomState, AtomStopReason, AtomTimerExpired, AtomValue,
    AtomVersion,
};
use taut_shape::{AtomInput, AtomOutput};

use crate::json::{self, Json};

pub fn input_from_json(value: &Json) -> Result<AtomInput, String> {
    let message_type = value
        .get("type")
        .and_then(Json::as_str)
        .ok_or_else(|| "atom script message missing string `type`".to_string())?;
    match message_type {
        "replace" => Ok(AtomInput::Replace(AtomReplace {
            payload: required_bytes(value, "payload", "replace")?,
        })),
        "seal" => Ok(AtomInput::Seal(AtomSeal {})),
        "close" => Ok(AtomInput::Close(AtomClose {
            error: optional_error(value.get("error"))?,
        })),
        "read" => Ok(AtomInput::Read(AtomReadRequest {
            atom_id: required_string(value, "atom_id", "read")?,
            stream_id: required_string(value, "stream_id", "read")?,
            version: match value.get("version") {
                None | Some(Json::Null) => None,
                Some(version) => Some(AtomVersion {
                    version: required_i64(version, "version", "read.version")?,
                }),
            },
            timeout_ms: optional_i64(value.get("timeout_ms"), "timeout_ms")?,
        })),
        "end_stream" => Ok(AtomInput::EndStream(AtomEndStream {
            atom_id: required_string(value, "atom_id", "end_stream")?,
            stream_id: required_string(value, "stream_id", "end_stream")?,
        })),
        "timer_expired" => Ok(AtomInput::TimerExpired(AtomTimerExpired {
            token: required_i64(value, "token", "timer_expired")?,
        })),
        other => Err(format!("unknown atom input type {other:?}")),
    }
}

pub fn output_to_json(output: &AtomOutput) -> Json {
    match output {
        AtomOutput::ReadResponse(response) => read_response_to_json(response),
        AtomOutput::SetTimer(AtomSetTimer { token, ms }) => obj(vec![
            ("type", json::s("set_timer")),
            ("token", json::i64_str(*token)),
            ("ms", json::i64_str(*ms)),
        ]),
        AtomOutput::CancelTimer(AtomCancelTimer { token }) => obj(vec![
            ("type", json::s("cancel_timer")),
            ("token", json::i64_str(*token)),
        ]),
        AtomOutput::ProducerStop(AtomProducerStop { reason }) => obj(vec![
            ("type", json::s("producer_stop")),
            (
                "reason",
                json::s(match reason {
                    AtomStopReason::LastReaderGone => "last_reader_gone",
                    AtomStopReason::Closed => "closed",
                    AtomStopReason::Failed => "failed",
                }),
            ),
        ]),
        AtomOutput::Diagnostic(AtomDiagnostic { severity, code }) => obj(vec![
            ("type", json::s("diagnostic")),
            (
                "severity",
                json::s(match severity {
                    AtomSeverity::Warn => "warn",
                    AtomSeverity::Error => "error",
                }),
            ),
            (
                "code",
                json::s(match code {
                    AtomDiagCode::ReplaceAfterTerminal => "replace_after_terminal",
                }),
            ),
        ]),
    }
}

pub fn read_response_to_json(response: &AtomReadResponse) -> Json {
    obj(vec![
        ("type", json::s("read_response")),
        ("atom_id", json::s(response.atom_id.clone())),
        ("stream_id", json::s(response.stream_id.clone())),
        (
            "value",
            response
                .value
                .as_ref()
                .map(value_to_json)
                .unwrap_or(Json::Null),
        ),
        (
            "next_version",
            obj(vec![(
                "version",
                json::i64_str(response.next_version.version),
            )]),
        ),
        (
            "state",
            json::s(match response.state {
                AtomState::Data => "data",
                AtomState::WouldBlock => "would_block",
                AtomState::Eof => "eof",
                AtomState::Closed => "closed",
                AtomState::Failed => "failed",
            }),
        ),
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

fn value_to_json(value: &AtomValue) -> Json {
    obj(vec![
        ("version", json::i64_str(value.version)),
        ("payload", json::s(json::base64_encode(&value.payload))),
    ])
}

fn error_to_json(error: &AtomError) -> Json {
    obj(vec![
        (
            "code",
            json::s(match error.code {
                AtomErrorCode::UnknownAtom => "unknown_atom",
                AtomErrorCode::ProducerError => "producer_error",
                AtomErrorCode::Internal => "internal",
            }),
        ),
        (
            "message",
            error.message.clone().map(json::s).unwrap_or(Json::Null),
        ),
    ])
}

fn optional_error(value: Option<&Json>) -> Result<Option<AtomError>, String> {
    let Some(value) = value else { return Ok(None) };
    if matches!(value, Json::Null) {
        return Ok(None);
    }
    let code = match value.get("code").and_then(Json::as_str) {
        Some("unknown_atom") => AtomErrorCode::UnknownAtom,
        Some("producer_error") => AtomErrorCode::ProducerError,
        Some("internal") => AtomErrorCode::Internal,
        other => return Err(format!("close.error: invalid code {other:?}")),
    };
    let message = match value.get("message") {
        None | Some(Json::Null) => None,
        Some(Json::Str(message)) => Some(message.clone()),
        Some(_) => return Err("close.error.message must be string or null".into()),
    };
    Ok(Some(AtomError { code, message }))
}

fn required_string(value: &Json, field: &str, kind: &str) -> Result<String, String> {
    value
        .get(field)
        .and_then(Json::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("{kind}: missing string `{field}`"))
}

fn required_i64(value: &Json, field: &str, kind: &str) -> Result<i64, String> {
    value
        .get(field)
        .and_then(Json::as_i64)
        .ok_or_else(|| format!("{kind}: missing i64 `{field}`"))
}

fn optional_i64(value: Option<&Json>, field: &str) -> Result<Option<i64>, String> {
    match value {
        None | Some(Json::Null) => Ok(None),
        Some(value) => value
            .as_i64()
            .map(Some)
            .ok_or_else(|| format!("`{field}` must be an i64 or null")),
    }
}

fn required_bytes(value: &Json, field: &str, kind: &str) -> Result<Vec<u8>, String> {
    value
        .get(field)
        .and_then(Json::as_str)
        .and_then(json::base64_decode)
        .ok_or_else(|| format!("{kind}: missing base64 `{field}`"))
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
    use taut_shape::{AtomNode, StopWhen};

    #[test]
    fn all_atom_oracle_vectors_match_whole_transcripts() {
        let document =
            json::parse(include_str!("../../../../taut-shape/corpus/atom.v1.json")).unwrap();
        let vectors = document.get("vectors").and_then(Json::as_arr).unwrap();
        assert_eq!(vectors.len(), 28);
        for vector in vectors {
            let stop_when = match vector
                .get("node")
                .and_then(|node| node.get("stop_when"))
                .and_then(Json::as_str)
            {
                Some("explicit_only") => StopWhen::ExplicitOnly,
                _ => StopWhen::LastReader,
            };
            let mut node = AtomNode::new(stop_when);
            for step in vector.get("steps").and_then(Json::as_arr).unwrap() {
                let input = input_from_json(step.get("in").unwrap()).unwrap();
                let actual = Json::Arr(node.handle(input).iter().map(output_to_json).collect());
                assert_eq!(actual, step.get("out").unwrap().clone());
            }
        }
    }
}
