use serde::{Deserialize, Serialize};

use crate::dto::CanFrameDto;

/// Result of filtering frames.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterResult {
    /// Filtered frames
    pub frames: Vec<CanFrameDto>,
    /// Total frames before filtering
    pub total_count: usize,
    /// Filtered frame count
    pub filtered_count: usize,
}
