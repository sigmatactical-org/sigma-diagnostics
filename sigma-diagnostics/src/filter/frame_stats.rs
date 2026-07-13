use serde::{Deserialize, Serialize};

/// Frame statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameStats {
    /// Number of unique message IDs
    pub unique_messages: usize,
    /// Frames per second
    pub frame_rate: f64,
    /// Average delta time in milliseconds
    pub avg_delta_ms: f64,
    /// Estimated bus load percentage (assumes 500kbps CAN)
    pub bus_load: f64,
}
