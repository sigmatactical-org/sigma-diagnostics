//! Frame filtering and statistics - all computation in Rust.
//!
//! Replaces TypeScript `getFilteredFrames()` and `calculateFrameStats()`.
//!
//! Configuration and result types each live in their own file; the filtering,
//! statistics, and pattern-matching logic is grouped by concern. Everything is
//! re-exported flat so callers use `filter::Item` regardless of submodule.

mod dbc_message_info;
mod dlc_detection_result;
mod filter_config;
mod filter_result;
mod filtering;
mod frame_stats;
mod match_status;
mod message_count;
mod pattern;
mod statistics;

#[cfg(test)]
mod tests;

pub use dbc_message_info::{DbcMessageCache, DbcMessageInfo, build_message_cache_from_dbc};
pub use dlc_detection_result::DlcDetectionResult;
pub use filter_config::FilterConfig;
pub use filter_result::FilterResult;
pub use filtering::{filter_frames, filter_frames_with_cache};
pub use frame_stats::FrameStats;
pub use match_status::MatchStatus;
pub use message_count::MessageCount;
pub use pattern::{match_data_pattern, parse_data_pattern};
pub use statistics::{calculate_frame_stats, detect_dlc, get_message_counts};
