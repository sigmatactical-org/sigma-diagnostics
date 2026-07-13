//! Aggregate statistics computed over captured frames.

use std::collections::{HashMap, HashSet};

use crate::dto::CanFrameDto;

use super::{DlcDetectionResult, FrameStats, MessageCount};

/// Calculate frame statistics.
pub fn calculate_frame_stats(frames: Vec<CanFrameDto>) -> FrameStats {
    if frames.is_empty() {
        return FrameStats {
            unique_messages: 0,
            frame_rate: 0.0,
            avg_delta_ms: 0.0,
            bus_load: 0.0,
        };
    }

    // Count unique message IDs
    let unique_ids: HashSet<u32> = frames.iter().map(|f| f.can_id).collect();
    let unique_messages = unique_ids.len();

    // Calculate frame rate and delta time using recent frames
    let recent_count = frames.len().min(100);
    let recent_frames = &frames[frames.len() - recent_count..];

    let mut frame_rate = 0.0;
    let mut avg_delta_ms = 0.0;

    if recent_frames.len() >= 2 {
        let first_ts = recent_frames[0].timestamp;
        let last_ts = recent_frames[recent_frames.len() - 1].timestamp;
        let duration = last_ts - first_ts;

        if duration > 0.0 {
            frame_rate = (recent_frames.len() - 1) as f64 / duration;
            avg_delta_ms = (duration / (recent_frames.len() - 1) as f64) * 1000.0;
        }
    }

    // Calculate bus load (approximate)
    // Assumes 500kbps CAN, average frame ~100 bits
    // Bus capacity at 500kbps ≈ 5000 frames/sec theoretical max
    let max_frames_per_sec = 5000.0;
    let bus_load = (frame_rate / max_frames_per_sec * 100.0).min(100.0);

    FrameStats {
        unique_messages,
        frame_rate,
        avg_delta_ms,
        bus_load,
    }
}

/// Get message ID counts (sorted by count descending).
pub fn get_message_counts(frames: Vec<CanFrameDto>) -> Vec<MessageCount> {
    let mut counts: HashMap<(u32, bool), u64> = HashMap::new();

    for frame in &frames {
        *counts.entry((frame.can_id, frame.is_extended)).or_default() += 1;
    }

    let mut result: Vec<MessageCount> = counts
        .into_iter()
        .map(|((can_id, is_extended), count)| MessageCount {
            can_id,
            is_extended,
            count,
        })
        .collect();

    // Sort by count descending
    result.sort_by_key(|b| std::cmp::Reverse(b.count));

    result
}

/// Detect DLC from frames for a specific CAN ID.
pub fn detect_dlc(frames: Vec<CanFrameDto>, can_id: u32, is_extended: bool) -> DlcDetectionResult {
    // Filter frames for this CAN ID
    let matching: Vec<&CanFrameDto> = frames
        .iter()
        .filter(|f| f.can_id == can_id && f.is_extended == is_extended)
        .collect();

    if matching.is_empty() {
        return DlcDetectionResult {
            detected_dlc: 8,
            confidence: 0.0,
            sample_count: 0,
        };
    }

    // Build histogram of DLC values
    let mut dlc_counts: HashMap<u8, usize> = HashMap::new();
    for frame in &matching {
        *dlc_counts.entry(frame.dlc).or_default() += 1;
    }

    // Find most common DLC
    let (detected_dlc, max_count) = dlc_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(dlc, count)| (*dlc, *count))
        .unwrap_or((8, 0));

    let confidence = if matching.is_empty() {
        0.0
    } else {
        max_count as f64 / matching.len() as f64
    };

    DlcDetectionResult {
        detected_dlc,
        confidence,
        sample_count: matching.len(),
    }
}
