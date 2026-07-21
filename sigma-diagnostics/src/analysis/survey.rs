//! Per-ID structural survey: measured timing, DLC census, empirical bit
//! activity, and the proven-facts engines (counters, checksums, mux).
//!
//! Every number in the report is a measurement over the supplied frames.

use serde::{Deserialize, Serialize};

use super::checksum::{find_checksums, ChecksumProof};
use super::counter::{prove_counters, CounterProof};
use super::mux::{mux_facts, MuxFacts};
use crate::dto::CanFrameDto;

/// Tuning knobs for the survey; defaults are conservative evidence gates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyOptions {
    /// Restrict analysis to these CAN IDs (None = all).
    pub id_filter: Option<Vec<u32>>,
    /// Below this many distinct payloads the checksum sweep is skipped.
    pub min_distinct_payloads_for_checksum: usize,
    /// Minimum consecutive-frame transitions for a counter proof.
    pub min_counter_transitions: u64,
    /// Maximum counter width to search, in bits.
    pub max_counter_width: u32,
    /// Mux variants with fewer frames than this are not counted as evidence.
    pub min_frames_per_mux_variant: u64,
    /// Selector candidates with more distinct values than this are skipped.
    pub max_mux_selector_values: usize,
}

impl Default for SurveyOptions {
    fn default() -> Self {
        Self {
            id_filter: None,
            min_distinct_payloads_for_checksum: 8,
            min_counter_transitions: 16,
            max_counter_width: 16,
            min_frames_per_mux_variant: 20,
            max_mux_selector_values: 32,
        }
    }
}

/// Inter-arrival statistics, in milliseconds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingStats {
    pub min_ms: f64,
    pub max_ms: f64,
    pub mean_ms: f64,
    pub median_ms: f64,
    pub stddev_ms: f64,
    /// Number of inter-arrival gaps measured (= frames − 1).
    pub samples: u64,
}

/// Measured activity of one payload bit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitActivityCell {
    pub bit: u32,
    /// Frames (covering this bit) where the bit was 1 / 0.
    pub ones: u64,
    pub zeros: u64,
    /// Value changes between consecutive frames covering the bit.
    pub transitions: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitState {
    Always0,
    Always1,
    Toggles,
    /// No frame covered this bit (payload shorter than DLC area displayed).
    Unobserved,
}

impl BitActivityCell {
    pub fn state(&self) -> BitState {
        match (self.ones, self.zeros) {
            (0, 0) => BitState::Unobserved,
            (0, _) => BitState::Always0,
            (_, 0) => BitState::Always1,
            _ => BitState::Toggles,
        }
    }
}

/// Everything measured and proven about one CAN ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdSurvey {
    pub can_id: u32,
    pub is_extended: bool,
    pub channel: String,
    pub frames: u64,
    pub first_timestamp: f64,
    pub last_timestamp: f64,
    /// Observed DLC values with counts (usually a single entry).
    pub dlc_counts: Vec<(u8, u64)>,
    pub timing: Option<TimingStats>,
    pub distinct_payloads: u64,
    /// One cell per bit up to the largest observed DLC.
    pub bit_activity: Vec<BitActivityCell>,
    pub counters: Vec<CounterProof>,
    pub checksums: Vec<ChecksumProof>,
    /// Why the checksum sweep was skipped, if it was.
    pub checksum_note: Option<String>,
    pub mux: Vec<MuxFacts>,
}

/// Survey of a whole recording.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyReport {
    pub total_frames: u64,
    pub unique_ids: u64,
    pub first_timestamp: f64,
    pub last_timestamp: f64,
    pub ids: Vec<IdSurvey>,
}

/// Group frames by (CAN ID, extended flag), each group sorted by timestamp.
pub(crate) fn group_by_id<'a>(
    frames: &'a [CanFrameDto],
    id_filter: Option<&[u32]>,
) -> std::collections::BTreeMap<(u32, bool), Vec<&'a CanFrameDto>> {
    let mut groups: std::collections::BTreeMap<(u32, bool), Vec<&CanFrameDto>> =
        std::collections::BTreeMap::new();
    for frame in frames {
        if let Some(filter) = id_filter {
            if !filter.contains(&frame.can_id) {
                continue;
            }
        }
        groups
            .entry((frame.can_id, frame.is_extended))
            .or_default()
            .push(frame);
    }
    for group in groups.values_mut() {
        group.sort_by(|a, b| a.timestamp.total_cmp(&b.timestamp));
    }
    groups
}

/// Compute per-bit activity over `bit_span` bits (typically max DLC × 8).
pub(crate) fn bit_activity(group: &[&CanFrameDto], bit_span: u32) -> Vec<BitActivityCell> {
    let mut cells: Vec<BitActivityCell> = (0..bit_span)
        .map(|bit| BitActivityCell {
            bit,
            ones: 0,
            zeros: 0,
            transitions: 0,
        })
        .collect();
    let mut previous: Vec<Option<u8>> = vec![None; bit_span as usize];
    for frame in group {
        for cell in cells.iter_mut() {
            let byte = (cell.bit / 8) as usize;
            if byte >= frame.data.len() {
                continue;
            }
            let value = frame.data[byte] >> (cell.bit % 8) & 1;
            if value == 1 {
                cell.ones += 1;
            } else {
                cell.zeros += 1;
            }
            let slot = &mut previous[cell.bit as usize];
            if let Some(prev) = *slot {
                if prev != value {
                    cell.transitions += 1;
                }
            }
            *slot = Some(value);
        }
    }
    cells
}

/// Run the full structural survey.
pub fn survey_frames(frames: &[CanFrameDto], options: &SurveyOptions) -> SurveyReport {
    let groups = group_by_id(frames, options.id_filter.as_deref());
    let mut ids = Vec::with_capacity(groups.len());

    for ((can_id, is_extended), group) in &groups {
        let n = group.len() as u64;
        let max_dlc_bytes = group.iter().map(|f| f.data.len()).max().unwrap_or(0);
        let min_dlc_bytes = group.iter().map(|f| f.data.len()).min().unwrap_or(0);
        let bit_span = (max_dlc_bytes * 8) as u32;
        let bit_limit = (min_dlc_bytes * 8) as u32;

        // DLC census.
        let mut dlc_counts: std::collections::BTreeMap<u8, u64> = std::collections::BTreeMap::new();
        for frame in group {
            *dlc_counts.entry(frame.dlc).or_default() += 1;
        }

        // Timing.
        let timing = if group.len() >= 2 {
            let mut deltas: Vec<f64> = group
                .windows(2)
                .map(|w| (w[1].timestamp - w[0].timestamp) * 1000.0)
                .collect();
            deltas.sort_by(|a, b| a.total_cmp(b));
            let samples = deltas.len();
            let mean = deltas.iter().sum::<f64>() / samples as f64;
            let variance =
                deltas.iter().map(|d| (d - mean) * (d - mean)).sum::<f64>() / samples as f64;
            let median = if samples % 2 == 1 {
                deltas[samples / 2]
            } else {
                (deltas[samples / 2 - 1] + deltas[samples / 2]) / 2.0
            };
            Some(TimingStats {
                min_ms: deltas[0],
                max_ms: deltas[samples - 1],
                mean_ms: mean,
                median_ms: median,
                stddev_ms: variance.sqrt(),
                samples: samples as u64,
            })
        } else {
            None
        };

        let cells = bit_activity(group, bit_span);
        let toggle_mask: Vec<bool> = cells
            .iter()
            .map(|c| c.state() == BitState::Toggles)
            .collect();

        let payloads: Vec<&[u8]> = group.iter().map(|f| f.data.as_slice()).collect();
        let distinct_payloads = {
            let mut seen = std::collections::HashSet::new();
            payloads.iter().filter(|p| seen.insert(**p)).count() as u64
        };

        let counters = prove_counters(
            &payloads,
            &toggle_mask,
            bit_limit,
            options.max_counter_width,
            options.min_counter_transitions,
        );

        let (checksums, checksum_note) = if dlc_counts.len() > 1 {
            (Vec::new(), Some("skipped: DLC varies".to_string()))
        } else if (distinct_payloads as usize) < options.min_distinct_payloads_for_checksum {
            (
                Vec::new(),
                Some(format!(
                    "skipped: only {distinct_payloads} distinct payloads (need {})",
                    options.min_distinct_payloads_for_checksum
                )),
            )
        } else {
            (
                find_checksums(&payloads, options.min_distinct_payloads_for_checksum),
                None,
            )
        };

        let mux = mux_facts(
            &payloads,
            &toggle_mask,
            bit_limit,
            options.min_frames_per_mux_variant,
            options.max_mux_selector_values,
        );

        ids.push(IdSurvey {
            can_id: *can_id,
            is_extended: *is_extended,
            channel: group[0].channel.clone(),
            frames: n,
            first_timestamp: group[0].timestamp,
            last_timestamp: group[group.len() - 1].timestamp,
            dlc_counts: dlc_counts.into_iter().collect(),
            timing,
            distinct_payloads,
            bit_activity: cells,
            counters,
            checksums,
            checksum_note,
            mux,
        });
    }

    SurveyReport {
        total_frames: frames.len() as u64,
        unique_ids: ids.len() as u64,
        first_timestamp: frames
            .iter()
            .map(|f| f.timestamp)
            .fold(f64::INFINITY, f64::min),
        last_timestamp: frames
            .iter()
            .map(|f| f.timestamp)
            .fold(f64::NEG_INFINITY, f64::max),
        ids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn frame(can_id: u32, timestamp: f64, data: &[u8]) -> CanFrameDto {
        CanFrameDto {
            timestamp,
            channel: "can0".to_string(),
            can_id,
            is_extended: false,
            is_fd: false,
            brs: false,
            esi: false,
            dlc: data.len() as u8,
            data: data.to_vec(),
        }
    }

    #[test]
    fn survey_measures_timing_and_bits() {
        // 10 ms period, byte 0 counts, byte 1 constant 0xAA.
        let frames: Vec<CanFrameDto> = (0..300u32)
            .map(|i| frame(0x123, i as f64 * 0.010, &[(i % 256) as u8, 0xAA]))
            .collect();
        let report = survey_frames(&frames, &SurveyOptions::default());
        assert_eq!(report.total_frames, 300);
        assert_eq!(report.unique_ids, 1);
        let id = &report.ids[0];
        assert_eq!(id.can_id, 0x123);
        assert_eq!(id.frames, 300);
        assert_eq!(id.dlc_counts, vec![(2, 300)]);
        let timing = id.timing.as_ref().unwrap();
        assert!((timing.median_ms - 10.0).abs() < 1e-6);
        assert_eq!(timing.samples, 299);

        // Byte 0 bits toggle; byte 1 = 0xAA pattern is constant.
        assert_eq!(id.bit_activity[0].state(), BitState::Toggles);
        assert_eq!(id.bit_activity[8].state(), BitState::Always0);
        assert_eq!(id.bit_activity[9].state(), BitState::Always1);

        // The counting byte is proven as an 8-bit counter.
        assert_eq!(id.counters.len(), 1);
        assert_eq!(id.counters[0].field.start_bit, 0);
        assert_eq!(id.counters[0].field.length, 8);
    }

    #[test]
    fn checksum_skip_notes_are_reported() {
        let frames: Vec<CanFrameDto> = (0..50u32)
            .map(|i| frame(0x200, i as f64 * 0.1, &[0x01, 0x02]))
            .collect();
        let report = survey_frames(&frames, &SurveyOptions::default());
        let id = &report.ids[0];
        assert!(id.checksums.is_empty());
        assert!(id
            .checksum_note
            .as_ref()
            .unwrap()
            .contains("distinct payloads"));
    }
}
