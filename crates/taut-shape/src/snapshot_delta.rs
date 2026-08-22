//! Fixed expiry profile over the shared SWMR store and resolver.

use alloc::{string::String, vec::Vec};

use crate::generated_swmr::{SwmrReadResponse, SwmrResetReason, SwmrState};
use crate::{StopWhen, SwmrInput, SwmrNode, SwmrOutput, SwmrRecoveryPolicy};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SnapshotDeltaRefreshReason {
    RetentionExpired,
    InvalidCursor,
    SourceChanged,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotDeltaRefreshRequired {
    pub swmr_id: String,
    pub stream_id: String,
    pub reason: SnapshotDeltaRefreshReason,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SnapshotDeltaOutput {
    Core(SwmrOutput),
    RefreshRequired(SnapshotDeltaRefreshRequired),
}

pub fn project_snapshot_delta_output(output: SwmrOutput) -> SnapshotDeltaOutput {
    match output {
        SwmrOutput::ReadResponse(SwmrReadResponse {
            swmr_id,
            stream_id,
            state: SwmrState::Reset,
            reset_reason,
            ..
        }) => SnapshotDeltaOutput::RefreshRequired(SnapshotDeltaRefreshRequired {
            swmr_id,
            stream_id,
            reason: match reset_reason {
                Some(SwmrResetReason::RetentionExceeded) => {
                    SnapshotDeltaRefreshReason::RetentionExpired
                }
                Some(SwmrResetReason::InvalidResumeSeq) => {
                    SnapshotDeltaRefreshReason::InvalidCursor
                }
                Some(SwmrResetReason::ProducerRequested) | None => {
                    SnapshotDeltaRefreshReason::SourceChanged
                }
            },
        }),
        output => SnapshotDeltaOutput::Core(output),
    }
}

pub struct SnapshotDeltaNode {
    core: SwmrNode,
}

impl SnapshotDeltaNode {
    pub fn new(stop_when: StopWhen, max_deltas: usize) -> Self {
        assert!(max_deltas > 0, "snapshot_delta max_deltas must be positive");
        Self {
            core: SwmrNode::with_recovery(stop_when, Some(max_deltas), SwmrRecoveryPolicy::Expire),
        }
    }

    pub const fn recovery_policy(&self) -> SwmrRecoveryPolicy {
        self.core.recovery_policy()
    }

    pub fn handle(&mut self, input: SwmrInput) -> Vec<SnapshotDeltaOutput> {
        self.core
            .handle(input)
            .into_iter()
            .map(project_snapshot_delta_output)
            .collect()
    }
}

impl Default for SnapshotDeltaNode {
    fn default() -> Self {
        Self::new(StopWhen::LastReader, 64)
    }
}
