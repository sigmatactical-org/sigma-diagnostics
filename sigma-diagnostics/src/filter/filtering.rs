//! Core frame-filtering logic (public for pro crate reuse).

use std::collections::{HashMap, HashSet};

use crate::dto::CanFrameDto;
use crate::state::DiagnosticsState;

use super::pattern::{match_data_pattern, parse_data_pattern};
use super::{DbcMessageCache, FilterConfig, FilterResult, MatchStatus, build_message_cache_from_dbc};

/// Filter frames using a pre-built message cache.
/// This is the core filtering function that both base and pro versions use.
pub fn filter_frames_with_cache(
    frames: Vec<CanFrameDto>,
    filters: &FilterConfig,
    msg_cache: &DbcMessageCache,
) -> FilterResult {
    let total_count = frames.len();

    // Build CAN ID set for O(1) lookup
    let can_id_set: HashSet<u32> = filters.can_ids.iter().copied().collect();
    let has_can_id_filter = !can_id_set.is_empty();

    // Parse data pattern once
    let data_pattern = filters.data_pattern.as_ref().map(|p| parse_data_pattern(p));
    let has_data_pattern = data_pattern.as_ref().is_some_and(|p| !p.is_empty());

    // Lowercase message/signal filters for case-insensitive matching
    let message_filters: Vec<String> = filters.messages.iter().map(|s| s.to_lowercase()).collect();
    let has_message_filter = !message_filters.is_empty();

    let signal_filters: Vec<String> = filters.signals.iter().map(|s| s.to_lowercase()).collect();
    let has_signal_filter = !signal_filters.is_empty();

    // Check if we need DBC-related filters
    let needs_dbc =
        filters.match_status != MatchStatus::All || has_message_filter || has_signal_filter;

    // Filter frames
    let filtered: Vec<CanFrameDto> = frames
        .into_iter()
        .filter(|frame| {
            // Time range filter
            if let Some(min) = filters.time_min {
                if frame.timestamp < min {
                    return false;
                }
            }
            if let Some(max) = filters.time_max {
                if frame.timestamp > max {
                    return false;
                }
            }

            // CAN ID filter
            if has_can_id_filter && !can_id_set.contains(&frame.can_id) {
                return false;
            }

            // Channel filter
            if let Some(ref ch) = filters.channel {
                if frame.channel != *ch {
                    return false;
                }
            }

            // Data pattern filter
            if has_data_pattern {
                if let Some(ref pattern) = data_pattern {
                    if !match_data_pattern(&frame.data, pattern) {
                        return false;
                    }
                }
            }

            // DBC-related filters
            if needs_dbc {
                let key = (frame.can_id, frame.is_extended);
                let msg_info = msg_cache.get(&key);
                let has_match = msg_info.is_some();

                // Match status filter
                match filters.match_status {
                    MatchStatus::All => {}
                    MatchStatus::Matched => {
                        if !has_match {
                            return false;
                        }
                    }
                    MatchStatus::Unmatched => {
                        if has_match {
                            return false;
                        }
                    }
                }

                // Message name filter
                if has_message_filter {
                    let Some(info) = msg_info else {
                        return false;
                    };
                    if !message_filters.iter().any(|m| info.name.contains(m)) {
                        return false;
                    }
                }

                // Signal name filter
                if has_signal_filter {
                    let Some(info) = msg_info else {
                        return false;
                    };
                    if !signal_filters
                        .iter()
                        .any(|s| info.signal_names.iter().any(|sn| sn.contains(s)))
                    {
                        return false;
                    }
                }
            }

            true
        })
        .collect();

    let filtered_count = filtered.len();

    FilterResult {
        frames: filtered,
        total_count,
        filtered_count,
    }
}

/// Filter frames based on filter configuration.
pub fn filter_frames(
    frames: Vec<CanFrameDto>,
    filters: FilterConfig,
    state: &DiagnosticsState,
) -> FilterResult {
    // Build DBC message cache if needed
    let needs_dbc = filters.match_status != MatchStatus::All
        || !filters.messages.is_empty()
        || !filters.signals.is_empty();

    let msg_cache = if needs_dbc {
        build_message_cache(state)
    } else {
        HashMap::new()
    };

    filter_frames_with_cache(frames, &filters, &msg_cache)
}

/// Build message info cache from DiagnosticsState's DBC.
fn build_message_cache(state: &DiagnosticsState) -> DbcMessageCache {
    let dbc_guard = state.dbc.lock();
    match *dbc_guard {
        Some(ref dbc) => build_message_cache_from_dbc(dbc),
        None => HashMap::new(),
    }
}
