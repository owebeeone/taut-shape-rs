use std::collections::BTreeMap;

use taut_shape::{
    SnapshotDeltaNode, SnapshotDeltaOutput, SnapshotDeltaRefreshReason, StopWhen,
    SwmrRecoveryPolicy,
};

use crate::json::{self, Json};
use crate::{swmr_json, swmr_json::input_from_json};

fn output_to_json(output: &SnapshotDeltaOutput) -> Json {
    match output {
        SnapshotDeltaOutput::Core(output) => swmr_json::output_to_json(output),
        SnapshotDeltaOutput::RefreshRequired(refresh) => {
            let mut fields = BTreeMap::new();
            fields.insert("type".into(), json::s("refresh_required"));
            fields.insert("swmr_id".into(), json::s(refresh.swmr_id.clone()));
            fields.insert("stream_id".into(), json::s(refresh.stream_id.clone()));
            fields.insert(
                "reason".into(),
                json::s(match refresh.reason {
                    SnapshotDeltaRefreshReason::RetentionExpired => "retention_expired",
                    SnapshotDeltaRefreshReason::InvalidCursor => "invalid_cursor",
                    SnapshotDeltaRefreshReason::SourceChanged => "source_changed",
                }),
            );
            Json::Obj(fields)
        }
    }
}

#[test]
fn all_snapshot_delta_profile_vectors_share_the_swmr_core() {
    let document = json::parse(include_str!(
        "../../../../taut-shape/corpus/snapshot_delta.profile.v1.json"
    ))
    .unwrap();
    let vectors = document.get("vectors").and_then(Json::as_arr).unwrap();
    assert_eq!(vectors.len(), 4);
    for vector in vectors {
        let config = vector.get("node").unwrap();
        let stop_when = match config.get("stop_when").and_then(Json::as_str) {
            Some("explicit_only") => StopWhen::ExplicitOnly,
            _ => StopWhen::LastReader,
        };
        let max_deltas = config.get("max_deltas").and_then(Json::as_i64).unwrap() as usize;
        let mut node = SnapshotDeltaNode::new(stop_when, max_deltas);
        assert_eq!(node.recovery_policy(), SwmrRecoveryPolicy::Expire);
        for step in vector.get("steps").and_then(Json::as_arr).unwrap() {
            let input = input_from_json(step.get("in").unwrap()).unwrap();
            let actual = Json::Arr(node.handle(input).iter().map(output_to_json).collect());
            assert_eq!(actual, step.get("out").unwrap().clone());
        }
    }
}
