use serde::{Deserialize, Serialize};

/// Message ID counts for aggregation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageCount {
    pub can_id: u32,
    pub is_extended: bool,
    pub count: u64,
}
