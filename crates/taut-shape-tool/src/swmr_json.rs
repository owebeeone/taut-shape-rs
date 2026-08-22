//! SWMR jsoncodec-form bridge for scripts, transcripts, and corpus tests.

use std::collections::BTreeMap;

use taut_shape::generated_swmr::{
    SwmrCancelTimer, SwmrClose, SwmrCursor, SwmrDelta, SwmrDeltaPush, SwmrDiagCode, SwmrDiagnostic,
    SwmrEndStream, SwmrError, SwmrErrorCode, SwmrProducerStop, SwmrReadRequest, SwmrReadResponse,
    SwmrReset, SwmrResetReason, SwmrSeal, SwmrSetTimer, SwmrSeverity, SwmrSnapshot,
    SwmrSnapshotPush, SwmrState, SwmrStopReason, SwmrTimerExpired,
};
use taut_shape::{SwmrInput, SwmrOutput};

use crate::json::{self, Json};

pub fn input_from_json(value: &Json) -> Result<SwmrInput, String> {
    let kind = value
        .get("type")
        .and_then(Json::as_str)
        .ok_or_else(|| "swmr script message missing string `type`".to_string())?;
    match kind {
        "snapshot_push" => Ok(SwmrInput::SnapshotPush(SwmrSnapshotPush {
            writer_id: required_string(value, "writer_id", kind)?,
            payload: required_bytes(value, "payload", kind)?,
        })),
        "delta_push" => Ok(SwmrInput::DeltaPush(SwmrDeltaPush {
            writer_id: required_string(value, "writer_id", kind)?,
            payload: required_bytes(value, "payload", kind)?,
        })),
        "reset" => Ok(SwmrInput::Reset(SwmrReset {
            writer_id: required_string(value, "writer_id", kind)?,
            reason: reset_reason(value.get("reason").and_then(Json::as_str))?,
            detail: optional_bytes(value.get("detail"), "detail")?,
        })),
        "seal" => Ok(SwmrInput::Seal(SwmrSeal {})),
        "close" => Ok(SwmrInput::Close(SwmrClose {
            error: optional_error(value.get("error"))?,
        })),
        "read" => Ok(SwmrInput::Read(SwmrReadRequest {
            swmr_id: required_string(value, "swmr_id", kind)?,
            stream_id: required_string(value, "stream_id", kind)?,
            cursor: optional_cursor(value.get("cursor"))?,
            timeout_ms: optional_i64(value.get("timeout_ms"), "timeout_ms")?,
        })),
        "end_stream" => Ok(SwmrInput::EndStream(SwmrEndStream {
            swmr_id: required_string(value, "swmr_id", kind)?,
            stream_id: required_string(value, "stream_id", kind)?,
        })),
        "timer_expired" => Ok(SwmrInput::TimerExpired(SwmrTimerExpired {
            token: required_i64(value, "token", kind)?,
        })),
        other => Err(format!("unknown swmr input type {other:?}")),
    }
}

pub fn output_to_json(output: &SwmrOutput) -> Json {
    match output {
        SwmrOutput::ReadResponse(response) => read_response_to_json(response),
        SwmrOutput::SetTimer(SwmrSetTimer { token, ms }) => obj(vec![
            ("type", json::s("set_timer")),
            ("token", json::i64_str(*token)),
            ("ms", json::i64_str(*ms)),
        ]),
        SwmrOutput::CancelTimer(SwmrCancelTimer { token }) => obj(vec![
            ("type", json::s("cancel_timer")),
            ("token", json::i64_str(*token)),
        ]),
        SwmrOutput::ProducerStop(SwmrProducerStop { reason }) => obj(vec![
            ("type", json::s("producer_stop")),
            ("reason", json::s(stop_reason(*reason))),
        ]),
        SwmrOutput::Diagnostic(SwmrDiagnostic { severity, code }) => obj(vec![
            ("type", json::s("diagnostic")),
            (
                "severity",
                json::s(match severity {
                    SwmrSeverity::Warn => "warn",
                    SwmrSeverity::Error => "error",
                }),
            ),
            (
                "code",
                json::s(match code {
                    SwmrDiagCode::PushAfterTerminal => "push_after_terminal",
                    SwmrDiagCode::DeltaBeforeSnapshot => "delta_before_snapshot",
                    SwmrDiagCode::WriterConflict => "writer_conflict",
                    SwmrDiagCode::RetentionBoundExceeded => "retention_bound_exceeded",
                }),
            ),
        ]),
    }
}

pub fn read_response_to_json(response: &SwmrReadResponse) -> Json {
    obj(vec![
        ("type", json::s("read_response")),
        ("swmr_id", json::s(response.swmr_id.clone())),
        ("stream_id", json::s(response.stream_id.clone())),
        (
            "snapshot",
            response
                .snapshot
                .as_ref()
                .map(snapshot_to_json)
                .unwrap_or(Json::Null),
        ),
        (
            "deltas",
            Json::Arr(response.deltas.iter().map(delta_to_json).collect()),
        ),
        (
            "next_cursor",
            response
                .next_cursor
                .as_ref()
                .map(cursor_to_json)
                .unwrap_or(Json::Null),
        ),
        (
            "state",
            json::s(match response.state {
                SwmrState::Data => "data",
                SwmrState::WouldBlock => "would_block",
                SwmrState::Eof => "eof",
                SwmrState::Closed => "closed",
                SwmrState::Failed => "failed",
                SwmrState::Reset => "reset",
            }),
        ),
        (
            "reset_reason",
            response
                .reset_reason
                .map(|reason| json::s(reset_reason_name(reason)))
                .unwrap_or(Json::Null),
        ),
        (
            "error",
            response
                .error
                .as_ref()
                .map(error_to_json)
                .unwrap_or(Json::Null),
        ),
        (
            "reset_detail",
            response
                .reset_detail
                .as_ref()
                .map(|detail| json::s(json::base64_encode(detail)))
                .unwrap_or(Json::Null),
        ),
    ])
}

fn cursor_to_json(cursor: &SwmrCursor) -> Json {
    obj(vec![
        ("seq", json::i64_str(cursor.seq)),
        ("epoch", json::i64_str(cursor.epoch)),
    ])
}

fn snapshot_to_json(snapshot: &SwmrSnapshot) -> Json {
    obj(vec![
        ("seq", json::i64_str(snapshot.seq)),
        ("payload", json::s(json::base64_encode(&snapshot.payload))),
    ])
}

fn delta_to_json(delta: &SwmrDelta) -> Json {
    obj(vec![
        ("base_seq", json::i64_str(delta.base_seq)),
        ("seq", json::i64_str(delta.seq)),
        ("payload", json::s(json::base64_encode(&delta.payload))),
    ])
}

fn error_to_json(error: &SwmrError) -> Json {
    obj(vec![
        (
            "code",
            json::s(match error.code {
                SwmrErrorCode::UnknownSwmr => "unknown_swmr",
                SwmrErrorCode::ProducerError => "producer_error",
                SwmrErrorCode::Internal => "internal",
            }),
        ),
        (
            "message",
            error.message.clone().map(json::s).unwrap_or(Json::Null),
        ),
    ])
}

fn optional_error(value: Option<&Json>) -> Result<Option<SwmrError>, String> {
    let Some(value) = value else { return Ok(None) };
    if matches!(value, Json::Null) {
        return Ok(None);
    }
    let code = match value.get("code").and_then(Json::as_str) {
        Some("unknown_swmr") => SwmrErrorCode::UnknownSwmr,
        Some("producer_error") => SwmrErrorCode::ProducerError,
        Some("internal") => SwmrErrorCode::Internal,
        other => return Err(format!("close.error: invalid code {other:?}")),
    };
    let message = match value.get("message") {
        None | Some(Json::Null) => None,
        Some(Json::Str(message)) => Some(message.clone()),
        Some(_) => return Err("close.error.message must be string or null".into()),
    };
    Ok(Some(SwmrError { code, message }))
}

fn optional_cursor(value: Option<&Json>) -> Result<Option<SwmrCursor>, String> {
    let Some(value) = value else { return Ok(None) };
    if matches!(value, Json::Null) {
        return Ok(None);
    }
    Ok(Some(SwmrCursor {
        seq: required_i64(value, "seq", "cursor")?,
        epoch: required_i64(value, "epoch", "cursor")?,
    }))
}

fn reset_reason(value: Option<&str>) -> Result<SwmrResetReason, String> {
    match value {
        Some("producer_requested") => Ok(SwmrResetReason::ProducerRequested),
        Some("retention_exceeded") => Ok(SwmrResetReason::RetentionExceeded),
        Some("invalid_resume_seq") => Ok(SwmrResetReason::InvalidResumeSeq),
        other => Err(format!("reset: invalid reason {other:?}")),
    }
}

fn reset_reason_name(reason: SwmrResetReason) -> &'static str {
    match reason {
        SwmrResetReason::ProducerRequested => "producer_requested",
        SwmrResetReason::RetentionExceeded => "retention_exceeded",
        SwmrResetReason::InvalidResumeSeq => "invalid_resume_seq",
    }
}

fn stop_reason(reason: SwmrStopReason) -> &'static str {
    match reason {
        SwmrStopReason::LastReaderGone => "last_reader_gone",
        SwmrStopReason::Closed => "closed",
        SwmrStopReason::Failed => "failed",
    }
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

fn optional_bytes(value: Option<&Json>, field: &str) -> Result<Option<Vec<u8>>, String> {
    match value {
        None | Some(Json::Null) => Ok(None),
        Some(value) => value
            .as_str()
            .and_then(json::base64_decode)
            .map(Some)
            .ok_or_else(|| format!("`{field}` must be base64 or null")),
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
    use taut_shape::{StopWhen, SwmrNode};

    #[test]
    fn all_swmr_oracle_vectors_match_whole_transcripts() {
        let document =
            json::parse(include_str!("../../../../taut-shape/corpus/swmr.v1.json")).unwrap();
        let vectors = document.get("vectors").and_then(Json::as_arr).unwrap();
        assert_eq!(vectors.len(), 33);
        for vector in vectors {
            let config = vector.get("node").unwrap();
            let stop_when = match config.get("stop_when").and_then(Json::as_str) {
                Some("explicit_only") => StopWhen::ExplicitOnly,
                _ => StopWhen::LastReader,
            };
            let max_deltas = config
                .get("max_deltas")
                .and_then(Json::as_i64)
                .map(|value| value as usize);
            let mut node = SwmrNode::new(stop_when, max_deltas);
            for step in vector.get("steps").and_then(Json::as_arr).unwrap() {
                let input = input_from_json(step.get("in").unwrap()).unwrap();
                let actual = Json::Arr(node.handle(input).iter().map(output_to_json).collect());
                assert_eq!(actual, step.get("out").unwrap().clone());
            }
        }
    }
}
