use serde::{Deserialize, Serialize};

/// Match status filter for DBC matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MatchStatus {
    #[default]
    All,
    Matched,
    Unmatched,
}
