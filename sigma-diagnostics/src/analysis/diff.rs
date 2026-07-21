//! Differential compare of labeled recordings (stimulus / response).
//!
//! A bit is *differential* iff it is constant within **every** recording but
//! its constant value differs between at least two recordings. That is the
//! definitive criterion: with enough frames per recording, a differential bit
//! is attributable to whatever distinguishes the recordings.

use serde::{Deserialize, Serialize};

use super::survey::{bit_activity, group_by_id, BitState};
use crate::dto::CanFrameDto;

/// One recording with a human-meaningful label ("brake-off", "brake-on", …).
pub struct LabeledRecording<'a> {
    pub label: String,
    pub frames: &'a [CanFrameDto],
}

/// A bit constant in every recording with differing values across recordings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifferentialBit {
    pub bit: u32,
    /// Constant value of the bit per recording label, in input order.
    pub values: Vec<(String, u8)>,
}

/// Differential facts for one ID present in all recordings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdDiff {
    pub can_id: u32,
    pub is_extended: bool,
    /// Frame count per recording label — the evidence behind every bit fact.
    pub frames_per_recording: Vec<(String, u64)>,
    pub differential_bits: Vec<DifferentialBit>,
}

/// An ID that does not appear in every recording.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExclusiveId {
    pub can_id: u32,
    pub is_extended: bool,
    /// Labels of the recordings the ID appears in, with frame counts.
    pub present_in: Vec<(String, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffReport {
    pub labels: Vec<String>,
    /// IDs present in all recordings, listed only when differential bits exist.
    pub shared: Vec<IdDiff>,
    pub exclusive: Vec<ExclusiveId>,
}

/// Compare two or more labeled recordings.
pub fn diff_recordings(recordings: &[LabeledRecording]) -> DiffReport {
    let labels: Vec<String> = recordings.iter().map(|r| r.label.clone()).collect();
    let grouped: Vec<_> = recordings
        .iter()
        .map(|r| group_by_id(r.frames, None))
        .collect();

    let mut all_ids: std::collections::BTreeSet<(u32, bool)> = std::collections::BTreeSet::new();
    for groups in &grouped {
        all_ids.extend(groups.keys().copied());
    }

    let mut shared = Vec::new();
    let mut exclusive = Vec::new();
    for key in all_ids {
        let per_recording: Vec<Option<&Vec<&CanFrameDto>>> =
            grouped.iter().map(|g| g.get(&key)).collect();
        if per_recording.iter().any(|g| g.is_none()) {
            exclusive.push(ExclusiveId {
                can_id: key.0,
                is_extended: key.1,
                present_in: per_recording
                    .iter()
                    .zip(&labels)
                    .filter_map(|(g, label)| g.map(|frames| (label.clone(), frames.len() as u64)))
                    .collect(),
            });
            continue;
        }
        let groups: Vec<&Vec<&CanFrameDto>> = per_recording.into_iter().flatten().collect();

        // Compare over the bit span every frame of every recording covers.
        let common_bits = groups
            .iter()
            .map(|g| g.iter().map(|f| f.data.len() * 8).min().unwrap_or(0))
            .min()
            .unwrap_or(0) as u32;
        let activities: Vec<_> = groups
            .iter()
            .map(|g| bit_activity(g, common_bits))
            .collect();

        let mut differential_bits = Vec::new();
        for bit in 0..common_bits {
            let states: Vec<BitState> = activities
                .iter()
                .map(|cells| cells[bit as usize].state())
                .collect();
            if !states
                .iter()
                .all(|s| matches!(s, BitState::Always0 | BitState::Always1))
            {
                continue;
            }
            let values: Vec<u8> = states
                .iter()
                .map(|s| u8::from(*s == BitState::Always1))
                .collect();
            if values.windows(2).any(|w| w[0] != w[1]) {
                differential_bits.push(DifferentialBit {
                    bit,
                    values: labels.iter().cloned().zip(values).collect(),
                });
            }
        }
        if !differential_bits.is_empty() {
            shared.push(IdDiff {
                can_id: key.0,
                is_extended: key.1,
                frames_per_recording: labels
                    .iter()
                    .cloned()
                    .zip(groups.iter().map(|g| g.len() as u64))
                    .collect(),
                differential_bits,
            });
        }
    }

    DiffReport {
        labels,
        shared,
        exclusive,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(can_id: u32, timestamp: f64, data: &[u8]) -> CanFrameDto {
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
    fn finds_differential_bit() {
        // Bit 16 (byte 2, bit 0) is 0 in "off" and 1 in "on"; byte 0 counts
        // (not constant → excluded); byte 1 constant and equal (→ excluded).
        let off: Vec<CanFrameDto> = (0..100u32)
            .map(|i| frame(0x321, i as f64 * 0.01, &[(i % 16) as u8, 0x77, 0x00]))
            .collect();
        let on: Vec<CanFrameDto> = (0..100u32)
            .map(|i| frame(0x321, i as f64 * 0.01, &[(i % 16) as u8, 0x77, 0x01]))
            .collect();
        let report = diff_recordings(&[
            LabeledRecording {
                label: "off".into(),
                frames: &off,
            },
            LabeledRecording {
                label: "on".into(),
                frames: &on,
            },
        ]);
        assert_eq!(report.shared.len(), 1);
        let id = &report.shared[0];
        assert_eq!(id.can_id, 0x321);
        assert_eq!(id.differential_bits.len(), 1);
        assert_eq!(id.differential_bits[0].bit, 16);
        assert_eq!(
            id.differential_bits[0].values,
            vec![("off".to_string(), 0), ("on".to_string(), 1)]
        );
        assert!(report.exclusive.is_empty());
    }

    #[test]
    fn exclusive_ids_reported() {
        let a = vec![frame(0x100, 0.0, &[0]), frame(0x200, 0.1, &[0])];
        let b = vec![frame(0x100, 0.0, &[0])];
        let report = diff_recordings(&[
            LabeledRecording {
                label: "a".into(),
                frames: &a,
            },
            LabeledRecording {
                label: "b".into(),
                frames: &b,
            },
        ]);
        assert_eq!(report.exclusive.len(), 1);
        assert_eq!(report.exclusive[0].can_id, 0x200);
        assert_eq!(report.exclusive[0].present_in, vec![("a".to_string(), 1)]);
    }
}
