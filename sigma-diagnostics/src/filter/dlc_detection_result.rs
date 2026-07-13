use serde::{Deserialize, Serialize};

/// DLC detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DlcDetectionResult {
    pub detected_dlc: u8,
    pub confidence: f64,
    pub sample_count: usize,
}
