//! Ground-truth affine correlation.
//!
//! Given an authoritative reference time series (e.g. ECU-reported values via
//! OBD), exhaustively search raw field extractions of the broadcast traffic
//! for an affine relation `value = factor · raw + offset` that holds — within
//! the known quantization of both sides — for **every** time-aligned sample.
//! Matches are reported with factor, offset, max residual and evidence counts;
//! nothing below the evidence gates is reported at all.

use serde::{Deserialize, Serialize};

use super::bits::{extract_raw, occupied_bits, occupied_bits_msb_first, Endianness, FieldSpec};
use super::survey::{bit_activity, group_by_id, BitState};
use crate::dto::CanFrameDto;

/// An authoritative reference series (OBD PID, GPS channel, …).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceSeries {
    pub name: String,
    pub unit: String,
    /// Physical quantization step of the reference values.
    pub resolution: f64,
    /// (timestamp, value), on the same timebase as the frames.
    pub samples: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationOptions {
    /// A reference sample aligns with the latest frame at most this old.
    pub window_s: f64,
    /// Minimum aligned samples for any claim.
    pub min_samples: usize,
    /// Minimum distinct reference values (a flat series proves nothing).
    pub min_distinct_values: usize,
    pub min_width: u32,
    pub max_width: u32,
    /// IDs never considered as candidates (default: the OBD diag range —
    /// responses literally contain the reference values).
    pub excluded_ids: Vec<u32>,
}

impl Default for CorrelationOptions {
    fn default() -> Self {
        Self {
            window_s: 0.2,
            min_samples: 100,
            min_distinct_values: 8,
            min_width: 4,
            max_width: 16,
            excluded_ids: (0x7DF..=0x7EF).collect(),
        }
    }
}

/// An affine relation that held for every aligned sample.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffineMatch {
    pub can_id: u32,
    pub is_extended: bool,
    pub field: FieldSpec,
    pub factor: f64,
    pub offset: f64,
    /// `factor` as the simplest fraction consistent with the residual bound.
    pub factor_rational: Option<(i64, i64)>,
    /// Largest |predicted − reference| over all aligned samples.
    pub max_residual: f64,
    /// The acceptance bound: reference_resolution/2 + |factor|/2 (+ float eps).
    pub residual_bound: f64,
    pub aligned_samples: u64,
    pub distinct_reference_values: u64,
    pub time_span_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesMatches {
    pub name: String,
    pub unit: String,
    pub reference_samples: u64,
    pub distinct_reference_values: u64,
    pub matches: Vec<AffineMatch>,
    /// Why nothing was searched, when applicable.
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationReport {
    pub series: Vec<SeriesMatches>,
}

/// Search all broadcast IDs for affine matches against each reference series.
pub fn correlate(
    frames: &[CanFrameDto],
    references: &[ReferenceSeries],
    options: &CorrelationOptions,
) -> CorrelationReport {
    let groups = group_by_id(frames, None);
    let mut series_out = Vec::new();

    for reference in references {
        let distinct_values = {
            let mut seen = std::collections::HashSet::new();
            reference
                .samples
                .iter()
                .filter(|(_, v)| seen.insert(v.to_bits()))
                .count()
        };
        let mut result = SeriesMatches {
            name: reference.name.clone(),
            unit: reference.unit.clone(),
            reference_samples: reference.samples.len() as u64,
            distinct_reference_values: distinct_values as u64,
            matches: Vec::new(),
            note: None,
        };
        if reference.samples.len() < options.min_samples {
            result.note = Some(format!(
                "skipped: {} reference samples (need {})",
                reference.samples.len(),
                options.min_samples
            ));
            series_out.push(result);
            continue;
        }
        if distinct_values < options.min_distinct_values {
            result.note = Some(format!(
                "skipped: only {distinct_values} distinct reference values (need {})",
                options.min_distinct_values
            ));
            series_out.push(result);
            continue;
        }

        for ((can_id, is_extended), group) in &groups {
            if options.excluded_ids.contains(can_id) {
                continue;
            }
            result.matches.extend(match_id(
                *can_id,
                *is_extended,
                group,
                reference,
                options,
            ));
        }
        // Widest (most informative) first, then tightest fit.
        result.matches.sort_by(|a, b| {
            b.field
                .length
                .cmp(&a.field.length)
                .then(a.max_residual.total_cmp(&b.max_residual))
        });
        series_out.push(result);
    }

    CorrelationReport { series: series_out }
}

fn match_id(
    can_id: u32,
    is_extended: bool,
    group: &[&CanFrameDto],
    reference: &ReferenceSeries,
    options: &CorrelationOptions,
) -> Vec<AffineMatch> {
    // Align: latest frame with timestamp in [t − window, t] per reference sample.
    let times: Vec<f64> = group.iter().map(|f| f.timestamp).collect();
    let mut aligned: Vec<(&CanFrameDto, f64)> = Vec::new();
    for &(t, value) in &reference.samples {
        let idx = times.partition_point(|&ft| ft <= t);
        if idx == 0 {
            continue;
        }
        let frame = group[idx - 1];
        if t - frame.timestamp <= options.window_s {
            aligned.push((frame, value));
        }
    }
    if aligned.len() < options.min_samples {
        return Vec::new();
    }
    let distinct_reference_values = {
        let mut seen = std::collections::HashSet::new();
        aligned.iter().filter(|(_, v)| seen.insert(v.to_bits())).count()
    };
    if distinct_reference_values < options.min_distinct_values {
        return Vec::new();
    }
    let time_span_s = aligned[aligned.len() - 1].0.timestamp - aligned[0].0.timestamp;

    // Candidates only over bits that toggle within the aligned frames.
    let aligned_frames: Vec<&CanFrameDto> = aligned.iter().map(|(f, _)| *f).collect();
    let bit_limit = aligned_frames
        .iter()
        .map(|f| f.data.len() * 8)
        .min()
        .unwrap_or(0) as u32;
    let activity = bit_activity(&aligned_frames, bit_limit);
    let toggle_mask: Vec<bool> = activity
        .iter()
        .map(|c| c.state() == BitState::Toggles)
        .collect();

    let mut matches = Vec::new();
    let mut seen: std::collections::HashSet<(Vec<u32>, bool)> = std::collections::HashSet::new();
    for endianness in [Endianness::Intel, Endianness::Motorola] {
        for start_bit in 0..bit_limit {
            for length in options.min_width..=options.max_width {
                let bits = occupied_bits(start_bit, length, endianness);
                if bits.iter().any(|&b| {
                    b >= bit_limit || !toggle_mask.get(b as usize).copied().unwrap_or(false)
                }) {
                    continue;
                }
                for signed in [false, true] {
                    let spec = FieldSpec {
                        start_bit,
                        length,
                        endianness,
                        signed,
                    };
                    if !seen.insert((occupied_bits_msb_first(&spec), signed)) {
                        continue;
                    }
                    if let Some(m) =
                        try_fit(can_id, is_extended, &spec, &aligned, reference.resolution)
                    {
                        matches.push(AffineMatch {
                            aligned_samples: aligned.len() as u64,
                            distinct_reference_values: distinct_reference_values as u64,
                            time_span_s,
                            ..m
                        });
                    }
                }
            }
        }
    }
    matches
}

fn try_fit(
    can_id: u32,
    is_extended: bool,
    spec: &FieldSpec,
    aligned: &[(&CanFrameDto, f64)],
    resolution: f64,
) -> Option<AffineMatch> {
    let mut raws = Vec::with_capacity(aligned.len());
    for (frame, _) in aligned {
        raws.push(extract_raw(&frame.data, spec)? as f64);
    }
    let n = raws.len() as f64;
    let mean_raw = raws.iter().sum::<f64>() / n;
    let mean_val = aligned.iter().map(|(_, v)| v).sum::<f64>() / n;
    let var_raw = raws.iter().map(|r| (r - mean_raw).powi(2)).sum::<f64>();
    if var_raw == 0.0 {
        return None;
    }
    let cov = raws
        .iter()
        .zip(aligned)
        .map(|(r, (_, v))| (r - mean_raw) * (v - mean_val))
        .sum::<f64>();
    let factor = cov / var_raw;
    if factor == 0.0 || !factor.is_finite() {
        return None;
    }
    let offset = mean_val - factor * mean_raw;

    let scale = aligned
        .iter()
        .map(|(_, v)| v.abs())
        .fold(1.0f64, f64::max);
    let bound = resolution / 2.0 + factor.abs() / 2.0 + 1e-9 * scale;
    let max_residual = residual(&raws, aligned, factor, offset);
    if max_residual > bound {
        return None;
    }

    // Snap the factor to the simplest fraction that still passes the proof.
    let max_abs_raw = raws.iter().fold(0.0f64, |a, r| a.max(r.abs())).max(1.0);
    let mut best = AffineMatch {
        can_id,
        is_extended,
        field: *spec,
        factor,
        offset,
        factor_rational: None,
        max_residual,
        residual_bound: bound,
        aligned_samples: 0,
        distinct_reference_values: 0,
        time_span_s: 0.0,
    };
    if let Some((p, q)) = simplest_rational(factor, bound / (2.0 * max_abs_raw)) {
        let snapped = p as f64 / q as f64;
        let snapped_offset = aligned
            .iter()
            .zip(&raws)
            .map(|((_, v), r)| v - snapped * r)
            .sum::<f64>()
            / n;
        let snapped_residual = residual(&raws, aligned, snapped, snapped_offset);
        if snapped_residual <= bound {
            best.factor = snapped;
            best.offset = snapped_offset;
            best.factor_rational = Some((p, q));
            best.max_residual = snapped_residual;
        }
    }
    Some(best)
}

fn residual(raws: &[f64], aligned: &[(&CanFrameDto, f64)], factor: f64, offset: f64) -> f64 {
    raws.iter()
        .zip(aligned)
        .map(|(r, (_, v))| (factor * r + offset - v).abs())
        .fold(0.0f64, f64::max)
}

/// Simplest fraction p/q with |p/q − x| ≤ tolerance (continued fractions).
fn simplest_rational(x: f64, tolerance: f64) -> Option<(i64, i64)> {
    if !x.is_finite() || tolerance <= 0.0 {
        return None;
    }
    let (mut h0, mut h1) = (0i64, 1i64);
    let (mut k0, mut k1) = (1i64, 0i64);
    let mut value = x;
    for _ in 0..64 {
        let a = value.floor();
        if a.abs() > 1e15 {
            return None;
        }
        let a_int = a as i64;
        let h2 = a_int.checked_mul(h1)?.checked_add(h0)?;
        let k2 = a_int.checked_mul(k1)?.checked_add(k0)?;
        if k2 != 0 && (h2 as f64 / k2 as f64 - x).abs() <= tolerance {
            return Some((h2, k2));
        }
        h0 = h1;
        h1 = h2;
        k0 = k1;
        k1 = k2;
        let frac = value - a;
        if frac.abs() < 1e-15 {
            break;
        }
        value = 1.0 / frac;
    }
    None
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

    /// Broadcast RPM×4 in a Motorola 16-bit field at 50 Hz; reference decoded
    /// from the exact same raw at 5 Hz with PID 0x0C quantization (0.25 rpm).
    fn rpm_setup() -> (Vec<CanFrameDto>, ReferenceSeries) {
        let raw_at = |t: f64| -> u16 { (3200.0 + 18000.0 * (t / 30.0)) as u16 };
        let frames: Vec<CanFrameDto> = (0..1500u32)
            .map(|i| {
                let t = f64::from(i) * 0.020;
                let raw = raw_at(t);
                frame(0x156, t, &[(raw >> 8) as u8, raw as u8, 0x00, (i % 4) as u8])
            })
            .collect();
        let samples: Vec<(f64, f64)> = (1..150u32)
            .map(|i| {
                let t = f64::from(i) * 0.200 + 0.005;
                // Reference is what the ECU reports: raw/4 of the latest frame.
                let latest = frames
                    .iter()
                    .rev()
                    .find(|f| f.timestamp <= t)
                    .unwrap();
                let raw = (u16::from(latest.data[0]) << 8) | u16::from(latest.data[1]);
                (t, f64::from(raw) / 4.0)
            })
            .collect();
        let reference = ReferenceSeries {
            name: "EngineRpm".into(),
            unit: "rpm".into(),
            resolution: 0.25,
            samples,
        };
        (frames, reference)
    }

    #[test]
    fn recovers_exact_affine_relation() {
        let (frames, reference) = rpm_setup();
        let report = correlate(&frames, &[reference], &CorrelationOptions::default());
        assert_eq!(report.series.len(), 1);
        let series = &report.series[0];
        assert!(series.note.is_none(), "note: {:?}", series.note);
        assert!(!series.matches.is_empty(), "no matches found");
        let best = &series.matches[0];
        assert_eq!(best.can_id, 0x156);
        // Max RPM raw (0x52D0) never sets bit 15, so the widest *toggling*
        // field is 15 bits from bit 6 — identical values, honestly reported.
        assert_eq!(best.field.length, 15);
        assert_eq!(best.field.endianness, Endianness::Motorola);
        assert_eq!(best.field.start_bit, 6);
        assert_eq!(best.factor_rational, Some((1, 4)));
        assert!(best.offset.abs() < 0.25, "offset {}", best.offset);
        assert!(best.max_residual <= best.residual_bound);
    }

    #[test]
    fn noisy_reference_is_rejected() {
        let (frames, mut reference) = rpm_setup();
        // Perturb beyond the quantization bound: no exact relation exists.
        for (i, (_, v)) in reference.samples.iter_mut().enumerate() {
            if i % 7 == 0 {
                *v += 40.0;
            }
        }
        let report = correlate(&frames, &[reference], &CorrelationOptions::default());
        let series = &report.series[0];
        assert!(
            series.matches.iter().all(|m| m.can_id != 0x156 || m.field.length != 16),
            "perturbed reference must not yield the 16-bit match"
        );
    }

    #[test]
    fn flat_reference_is_skipped() {
        let (frames, mut reference) = rpm_setup();
        for (_, v) in reference.samples.iter_mut() {
            *v = 1000.0;
        }
        let report = correlate(&frames, &[reference], &CorrelationOptions::default());
        assert!(report.series[0].note.as_ref().unwrap().contains("distinct"));
    }

    #[test]
    fn simplest_rational_finds_quarters() {
        assert_eq!(simplest_rational(0.25, 1e-9), Some((1, 4)));
        assert_eq!(simplest_rational(0.1, 1e-9), Some((1, 10)));
        assert_eq!(simplest_rational(3.0, 1e-9), Some((3, 1)));
    }
}
