use serde::{Deserialize, Serialize};

use super::MatchStatus;

/// Filter configuration matching TypeScript `Filters` interface.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilterConfig {
    /// Minimum timestamp (inclusive)
    pub time_min: Option<f64>,
    /// Maximum timestamp (inclusive)
    pub time_max: Option<f64>,
    /// CAN IDs to include (empty = all)
    #[serde(default)]
    pub can_ids: Vec<u32>,
    /// Message names to filter (substring match, lowercase)
    #[serde(default)]
    pub messages: Vec<String>,
    /// Signal names to filter (substring match, lowercase)
    #[serde(default)]
    pub signals: Vec<String>,
    /// Data pattern filter (e.g., "01 ?? FF")
    pub data_pattern: Option<String>,
    /// Channel to filter (exact match)
    pub channel: Option<String>,
    /// Match status (all, matched, unmatched)
    #[serde(default)]
    pub match_status: MatchStatus,
}
