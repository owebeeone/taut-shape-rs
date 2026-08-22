//! Value jsoncodec-form bridge for scripts, transcripts, and corpus tests.

use std::collections::BTreeMap;

use taut_shape::generated_value::{
    ValueDiagCode, ValueDiagnostic, ValueReadRequest, ValueReadResponse, ValueSet, ValueSeverity,
    ValueState,
};
use taut_shape::{ValueInput, ValueOutput};

use crate::json::{self, Json};

pub fn input_from_json(value: &Json) -> Result<ValueInput, String> {
    let message_type = value
        .get("type")
        .and_then(Json::as_str)
        .ok_or_else(|| "value script message missing string `type`".to_string())?;
    match message_type {
        "set" => Ok(ValueInput::Set(ValueSet {
            origin: required_string(value, "origin", "set")?,
            seq: required_i64(value, "seq", "set")?,
            lamport: required_i64(value, "lamport", "set")?,
            prev: optional_bytes(value, "prev")?,
            payload: required_bytes(value, "payload", "set")?,
        })),
        "read" => Ok(ValueInput::Read(ValueReadRequest {
            value_id: required_string(value, "value_id", "read")?,
            stream_id: required_string(value, "stream_id", "read")?,
        })),
        other => Err(format!("unknown value input type {other:?}")),
    }
}

pub fn output_to_json(output: &ValueOutput) -> Json {
    match output {
        ValueOutput::Diagnostic(ValueDiagnostic { severity, code }) => obj(vec![
            ("type", json::s("diagnostic")),
            (
                "severity",
                json::s(match severity {
                    ValueSeverity::Warn => "warn",
                    ValueSeverity::Error => "error",
                }),
            ),
            (
                "code",
                json::s(match code {
                    ValueDiagCode::Equivocation => "equivocation",
                }),
            ),
        ]),
        ValueOutput::ReadResponse(response) => read_response_to_json(response),
    }
}

pub fn read_response_to_json(response: &ValueReadResponse) -> Json {
    obj(vec![
        ("type", json::s("read_response")),
        ("value_id", json::s(response.value_id.clone())),
        ("stream_id", json::s(response.stream_id.clone())),
        (
            "value",
            response
                .value
                .as_ref()
                .map(|value| json::s(json::base64_encode(value)))
                .unwrap_or(Json::Null),
        ),
        (
            "winner",
            response
                .winner
                .as_ref()
                .map(|winner| {
                    obj(vec![
                        ("origin", json::s(winner.origin.clone())),
                        ("seq", json::i64_str(winner.seq)),
                        ("lamport", json::i64_str(winner.lamport)),
                    ])
                })
                .unwrap_or(Json::Null),
        ),
        (
            "state",
            json::s(match response.state {
                ValueState::Data => "data",
                ValueState::Empty => "empty",
            }),
        ),
    ])
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

fn required_bytes(value: &Json, field: &str, kind: &str) -> Result<Vec<u8>, String> {
    value
        .get(field)
        .and_then(Json::as_str)
        .and_then(json::base64_decode)
        .ok_or_else(|| format!("{kind}: missing base64 `{field}`"))
}

fn optional_bytes(value: &Json, field: &str) -> Result<Option<Vec<u8>>, String> {
    match value.get(field) {
        None | Some(Json::Null) => Ok(None),
        Some(Json::Str(encoded)) => json::base64_decode(encoded)
            .map(Some)
            .ok_or_else(|| format!("invalid base64 `{field}`")),
        Some(_) => Err(format!("`{field}` must be base64 or null")),
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

#[cfg(test)]
mod tests {
    use super::*;
    use taut_shape::ValueNode;

    #[test]
    fn all_value_oracle_vectors_match_whole_transcripts() {
        let document =
            json::parse(include_str!("../../../../taut-shape/corpus/value.v0.json")).unwrap();
        let vectors = document.get("vectors").and_then(Json::as_arr).unwrap();
        assert_eq!(vectors.len(), 13);
        for vector in vectors {
            let mut node = ValueNode::new();
            for step in vector.get("steps").and_then(Json::as_arr).unwrap() {
                let input = input_from_json(step.get("in").unwrap()).unwrap();
                let actual = Json::Arr(node.handle(input).iter().map(output_to_json).collect());
                assert_eq!(actual, step.get("out").unwrap().clone());
            }
        }
    }
}
