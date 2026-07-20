//! Offline OBD ground-truth extraction from a recorded log.
//!
//! The poll session records both broadcast traffic and the diagnostic
//! request/response exchange into one MDF4; this module reassembles the
//! `0x7E8` ISO-TP responses and decodes Service `$01` values, yielding a
//! ground-truth time series on the same timebase as the broadcast frames.

use serde::{Deserialize, Serialize};

use super::isotp::{IsotpReassembler, OBD_ECU_RESPONSE_ID};
use super::pids::pid_def;
use crate::dto::CanFrameDto;

/// One authoritative ECU-reported value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundTruthSample {
    /// Timestamp of the frame completing the response.
    pub timestamp: f64,
    pub pid: u8,
    pub name: String,
    pub unit: String,
    /// Physical quantization step of the PID encoding.
    pub resolution: f64,
    pub value: f64,
}

/// A per-PID series of ground-truth samples, time-ordered.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundTruthSeries {
    pub pid: u8,
    pub name: String,
    pub unit: String,
    pub resolution: f64,
    /// (timestamp, value) pairs.
    pub samples: Vec<(f64, f64)>,
}

/// Extract all Service `$01` responses from `response_id` (default `0x7E8`).
pub fn extract_obd_samples(
    frames: &[CanFrameDto],
    response_id: Option<u32>,
) -> Vec<GroundTruthSample> {
    let response_id = response_id.unwrap_or(OBD_ECU_RESPONSE_ID);
    let mut responses: Vec<&CanFrameDto> = frames
        .iter()
        .filter(|f| f.can_id == response_id && !f.is_extended)
        .collect();
    responses.sort_by(|x, y| x.timestamp.total_cmp(&y.timestamp));

    let mut reassembler = IsotpReassembler::new();
    let mut samples = Vec::new();
    for frame in responses {
        let Some(message) = reassembler.push(frame.timestamp, &frame.data) else {
            continue;
        };
        // Positive Service $01 response: 0x41, PID echo, data bytes.
        let [0x41, pid, data @ ..] = message.data.as_slice() else {
            continue;
        };
        let Some(def) = pid_def(*pid) else {
            continue;
        };
        let Some(value) = def.decode(data) else {
            continue;
        };
        samples.push(GroundTruthSample {
            timestamp: message.timestamp,
            pid: *pid,
            name: def.name.to_string(),
            unit: def.unit.to_string(),
            resolution: def.resolution,
            value,
        });
    }
    samples
}

/// Group samples into per-PID series.
pub fn group_series(samples: &[GroundTruthSample]) -> Vec<GroundTruthSeries> {
    let mut by_pid: std::collections::BTreeMap<u8, GroundTruthSeries> =
        std::collections::BTreeMap::new();
    for sample in samples {
        by_pid
            .entry(sample.pid)
            .or_insert_with(|| GroundTruthSeries {
                pid: sample.pid,
                name: sample.name.clone(),
                unit: sample.unit.clone(),
                resolution: sample.resolution,
                samples: Vec::new(),
            })
            .samples
            .push((sample.timestamp, sample.value));
    }
    by_pid.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response_frame(timestamp: f64, data: &[u8]) -> CanFrameDto {
        CanFrameDto {
            timestamp,
            channel: "can0".to_string(),
            can_id: OBD_ECU_RESPONSE_ID,
            is_extended: false,
            is_fd: false,
            brs: false,
            esi: false,
            dlc: 8,
            data: data.to_vec(),
        }
    }

    #[test]
    fn extracts_rpm_and_speed_series() {
        let frames = vec![
            // RPM = 0x1AF8 / 4 = 1726
            response_frame(1.0, &[0x04, 0x41, 0x0C, 0x1A, 0xF8, 0x00, 0x00, 0x00]),
            // Speed = 60 km/h
            response_frame(1.1, &[0x03, 0x41, 0x0D, 0x3C, 0x00, 0x00, 0x00, 0x00]),
            // Negative response (0x7F) is skipped
            response_frame(1.2, &[0x03, 0x7F, 0x01, 0x12, 0x00, 0x00, 0x00, 0x00]),
        ];
        let samples = extract_obd_samples(&frames, None);
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].pid, 0x0C);
        assert_eq!(samples[0].value, 1726.0);
        assert_eq!(samples[1].pid, 0x0D);
        assert_eq!(samples[1].value, 60.0);

        let series = group_series(&samples);
        assert_eq!(series.len(), 2);
        assert_eq!(series[0].name, "EngineRpm");
        assert_eq!(series[0].samples, vec![(1.0, 1726.0)]);
    }
}
