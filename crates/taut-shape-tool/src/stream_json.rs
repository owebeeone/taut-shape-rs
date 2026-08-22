//! Stream jsoncodec-form bridge for scripts, transcripts, and corpus tests.

use std::collections::BTreeMap;

use taut_shape::generated_stream::{
    StreamCancelTimer, StreamClose, StreamDiagCode, StreamDiagnostic, StreamEndStream, StreamError,
    StreamErrorCode, StreamProducerStop, StreamPush, StreamReadRequest, StreamReadResponse,
    StreamRecord, StreamSeal, StreamSetTimer, StreamSeverity, StreamState, StreamStopReason,
    StreamTimerExpired,
};
use taut_shape::{StreamInput, StreamOutput};

use crate::json::{self, Json};

pub fn input_from_json(value: &Json) -> Result<StreamInput, String> {
    let kind = value
        .get("type")
        .and_then(Json::as_str)
        .ok_or_else(|| "stream script message missing string `type`".to_string())?;
    match kind {
        "push" => Ok(StreamInput::Push(StreamPush {
            payload: required_bytes(value, "payload", "push")?,
        })),
        "seal" => Ok(StreamInput::Seal(StreamSeal {})),
        "close" => Ok(StreamInput::Close(StreamClose {
            error: optional_error(value.get("error"))?,
        })),
        "read" => Ok(StreamInput::Read(StreamReadRequest {
            stream_id: required_string(value, "stream_id", "read")?,
            max_records: optional_i64(value.get("max_records"), "max_records")?,
            max_bytes: optional_i64(value.get("max_bytes"), "max_bytes")?,
            timeout_ms: optional_i64(value.get("timeout_ms"), "timeout_ms")?,
        })),
        "end_stream" => Ok(StreamInput::EndStream(StreamEndStream {
            stream_id: required_string(value, "stream_id", "end_stream")?,
        })),
        "timer_expired" => Ok(StreamInput::TimerExpired(StreamTimerExpired {
            token: required_i64(value, "token", "timer_expired")?,
        })),
        other => Err(format!("unknown stream input type {other:?}")),
    }
}

pub fn output_to_json(output: &StreamOutput) -> Json {
    match output {
        StreamOutput::ReadResponse(response) => read_response_to_json(response),
        StreamOutput::SetTimer(StreamSetTimer { token, ms }) => obj(vec![
            ("type", json::s("set_timer")),
            ("token", json::i64_str(*token)),
            ("ms", json::i64_str(*ms)),
        ]),
        StreamOutput::CancelTimer(StreamCancelTimer { token }) => obj(vec![
            ("type", json::s("cancel_timer")),
            ("token", json::i64_str(*token)),
        ]),
        StreamOutput::ProducerStop(StreamProducerStop { reason }) => obj(vec![
            ("type", json::s("producer_stop")),
            (
                "reason",
                json::s(match reason {
                    StreamStopReason::LastReaderGone => "last_reader_gone",
                    StreamStopReason::Closed => "closed",
                    StreamStopReason::Failed => "failed",
                }),
            ),
        ]),
        StreamOutput::Diagnostic(StreamDiagnostic { severity, code }) => obj(vec![
            ("type", json::s("diagnostic")),
            (
                "severity",
                json::s(match severity {
                    StreamSeverity::Warn => "warn",
                    StreamSeverity::Error => "error",
                }),
            ),
            (
                "code",
                json::s(match code {
                    StreamDiagCode::PushAfterTerminal => "push_after_terminal",
                }),
            ),
        ]),
    }
}

pub fn read_response_to_json(response: &StreamReadResponse) -> Json {
    obj(vec![
        ("type", json::s("read_response")),
        ("stream_id", json::s(response.stream_id.clone())),
        (
            "records",
            Json::Arr(response.records.iter().map(record_to_json).collect()),
        ),
        (
            "next_position",
            obj(vec![("seq", json::i64_str(response.next_position.seq))]),
        ),
        (
            "state",
            json::s(match response.state {
                StreamState::Data => "data",
                StreamState::WouldBlock => "would_block",
                StreamState::Eof => "eof",
                StreamState::Closed => "closed",
                StreamState::Failed => "failed",
                StreamState::Dropped => "dropped",
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

fn record_to_json(record: &StreamRecord) -> Json {
    obj(vec![
        ("seq", json::i64_str(record.seq)),
        ("payload", json::s(json::base64_encode(&record.payload))),
    ])
}

fn error_to_json(error: &StreamError) -> Json {
    obj(vec![
        (
            "code",
            json::s(match error.code {
                StreamErrorCode::UnknownStream => "unknown_stream",
                StreamErrorCode::ProducerError => "producer_error",
                StreamErrorCode::Internal => "internal",
                StreamErrorCode::SlowConsumer => "slow_consumer",
            }),
        ),
        (
            "message",
            error.message.clone().map(json::s).unwrap_or(Json::Null),
        ),
    ])
}

fn optional_error(value: Option<&Json>) -> Result<Option<StreamError>, String> {
    let Some(value) = value else { return Ok(None) };
    if matches!(value, Json::Null) {
        return Ok(None);
    }
    let code = match value.get("code").and_then(Json::as_str) {
        Some("unknown_stream") => StreamErrorCode::UnknownStream,
        Some("producer_error") => StreamErrorCode::ProducerError,
        Some("internal") => StreamErrorCode::Internal,
        Some("slow_consumer") => StreamErrorCode::SlowConsumer,
        other => return Err(format!("close.error: invalid code {other:?}")),
    };
    let message = match value.get("message") {
        None | Some(Json::Null) => None,
        Some(Json::Str(message)) => Some(message.clone()),
        Some(_) => return Err("close.error.message must be string or null".into()),
    };
    Ok(Some(StreamError { code, message }))
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
    use taut_shape::{StopWhen, StreamNode};

    #[test]
    fn all_stream_oracle_vectors_match_whole_transcripts() {
        let document =
            json::parse(include_str!("../../../../taut-shape/corpus/stream.v1.json")).unwrap();
        let vectors = document.get("vectors").and_then(Json::as_arr).unwrap();
        assert_eq!(vectors.len(), 28);
        for vector in vectors {
            let config = vector.get("node").unwrap();
            let stop_when = match config.get("stop_when").and_then(Json::as_str) {
                Some("explicit_only") => StopWhen::ExplicitOnly,
                _ => StopWhen::LastReader,
            };
            let capacity = config
                .get("capacity_records")
                .and_then(Json::as_i64)
                .unwrap() as usize;
            let mut node = StreamNode::new(capacity, stop_when);
            for step in vector.get("steps").and_then(Json::as_arr).unwrap() {
                let input = input_from_json(step.get("in").unwrap()).unwrap();
                let actual = Json::Arr(node.handle(input).iter().map(output_to_json).collect());
                assert_eq!(actual, step.get("out").unwrap().clone());
            }
        }
    }
}
