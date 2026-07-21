//! Plain-text rendering of analysis reports.
//!
//! Every stated fact carries its evidence (frame counts, distinct payloads,
//! transitions) so the reader can judge the strength of each proof.

use super::correlate::CorrelationReport;
use super::dbc_check::DbcCheckReport;
use super::diff::DiffReport;
use super::survey::{BitState, IdSurvey, SurveyReport};

fn fmt_id(can_id: u32, is_extended: bool) -> String {
    if is_extended {
        format!("{can_id:08X}x")
    } else {
        format!("{can_id:03X}")
    }
}

pub fn render_survey(report: &SurveyReport) -> String {
    let mut out = String::new();
    let duration = report.last_timestamp - report.first_timestamp;
    out.push_str(&format!(
        "Survey: {} frames, {} IDs, {:.1} s\n\n",
        report.total_frames, report.unique_ids, duration
    ));
    for id in &report.ids {
        render_id_survey(&mut out, id);
        out.push('\n');
    }
    out
}

fn render_id_survey(out: &mut String, id: &IdSurvey) {
    out.push_str(&format!(
        "ID {} · {} frames · {} distinct payloads",
        fmt_id(id.can_id, id.is_extended),
        id.frames,
        id.distinct_payloads
    ));
    if let Some(t) = &id.timing {
        out.push_str(&format!(
            " · period median {:.1} ms (min {:.1} / max {:.1} / σ {:.2}, n={})",
            t.median_ms, t.min_ms, t.max_ms, t.stddev_ms, t.samples
        ));
    }
    out.push('\n');
    let dlcs: Vec<String> = id
        .dlc_counts
        .iter()
        .map(|(dlc, n)| format!("{dlc} ({n} frames)"))
        .collect();
    out.push_str(&format!("  DLC: {}\n", dlcs.join(", ")));

    // Bit activity as one row per byte, MSB→LSB: '0' constant-0, '1' constant-1,
    // '~' toggling, '.' unobserved.
    for (byte_index, byte_cells) in id.bit_activity.chunks(8).enumerate() {
        let mut row = String::new();
        for bit_in_byte in (0..byte_cells.len()).rev() {
            row.push(match byte_cells[bit_in_byte].state() {
                BitState::Always0 => '0',
                BitState::Always1 => '1',
                BitState::Toggles => '~',
                BitState::Unobserved => '.',
            });
        }
        out.push_str(&format!("  byte {byte_index}: {row}\n"));
    }

    for c in &id.counters {
        out.push_str(&format!(
            "  COUNTER {} start {} len {} — +1 mod 2^{} held for all {} transitions ({} wraps)\n",
            c.field.endianness.label(),
            c.field.start_bit,
            c.field.length,
            c.field.length,
            c.transitions,
            c.wraps
        ));
    }
    for c in &id.checksums {
        out.push_str(&format!(
            "  CHECKSUM byte {} = {} — matched all {} frames ({} distinct payloads)\n",
            c.byte_index,
            c.algorithm.describe(),
            c.frames,
            c.distinct_payloads
        ));
    }
    if let Some(note) = &id.checksum_note {
        out.push_str(&format!("  checksum sweep {note}\n"));
    }
    for m in &id.mux {
        let variants: Vec<String> = m
            .variants
            .iter()
            .map(|v| format!("0x{:X} ({} frames)", v.selector_value, v.frames))
            .collect();
        out.push_str(&format!(
            "  MUX-CANDIDATE selector start {} len {} — conditional layouts differ across values {}\n",
            m.selector.start_bit,
            m.selector.length,
            variants.join(", ")
        ));
    }
}

pub fn render_diff(report: &DiffReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Differential compare of {} recordings: {}\n\n",
        report.labels.len(),
        report.labels.join(", ")
    ));
    for id in &report.shared {
        let counts: Vec<String> = id
            .frames_per_recording
            .iter()
            .map(|(label, n)| format!("{label}: {n}"))
            .collect();
        out.push_str(&format!(
            "ID {} ({})\n",
            fmt_id(id.can_id, id.is_extended),
            counts.join(", ")
        ));
        for bit in &id.differential_bits {
            let values: Vec<String> = bit
                .values
                .iter()
                .map(|(label, v)| format!("{label}={v}"))
                .collect();
            out.push_str(&format!(
                "  bit {} (byte {} bit {}): {}\n",
                bit.bit,
                bit.bit / 8,
                bit.bit % 8,
                values.join(", ")
            ));
        }
    }
    if report.shared.is_empty() {
        out.push_str("No differential bits found.\n");
    }
    if !report.exclusive.is_empty() {
        out.push_str("\nIDs not present in every recording:\n");
        for ex in &report.exclusive {
            let present: Vec<String> = ex
                .present_in
                .iter()
                .map(|(label, n)| format!("{label} ({n} frames)"))
                .collect();
            out.push_str(&format!(
                "  {} — only in {}\n",
                fmt_id(ex.can_id, ex.is_extended),
                present.join(", ")
            ));
        }
    }
    out
}

pub fn render_correlation(report: &CorrelationReport) -> String {
    let mut out = String::new();
    for series in &report.series {
        out.push_str(&format!(
            "{} [{}] — {} reference samples, {} distinct values\n",
            series.name, series.unit, series.reference_samples, series.distinct_reference_values
        ));
        if let Some(note) = &series.note {
            out.push_str(&format!("  {note}\n\n"));
            continue;
        }
        if series.matches.is_empty() {
            out.push_str("  no field matched within quantization bounds\n\n");
            continue;
        }
        for m in &series.matches {
            let factor = match m.factor_rational {
                Some((p, q)) => format!("{p}/{q}"),
                None => format!("{:.9}", m.factor),
            };
            out.push_str(&format!(
                "  MATCH ID {} {} start {} len {}{}: value = {} × raw {} {:.4}\n",
                fmt_id(m.can_id, m.is_extended),
                m.field.endianness.label(),
                m.field.start_bit,
                m.field.length,
                if m.field.signed { " signed" } else { "" },
                factor,
                if m.offset >= 0.0 { "+" } else { "−" },
                m.offset.abs()
            ));
            out.push_str(&format!(
                "    held for all {} aligned samples over {:.1} s (max residual {:.4} ≤ bound {:.4}, {} distinct values)\n",
                m.aligned_samples, m.time_span_s, m.max_residual, m.residual_bound,
                m.distinct_reference_values
            ));
        }
        out.push('\n');
    }
    out
}

pub fn render_dbc_check(report: &DbcCheckReport) -> String {
    let mut out = String::new();
    for id in &report.covered {
        out.push_str(&format!(
            "{} {} · {} frames\n",
            fmt_id(id.can_id, id.is_extended),
            id.message_name,
            id.frames
        ));
        if id.dlc_mismatch_frames > 0 {
            out.push_str(&format!(
                "  DLC MISMATCH: {} frames differ from DBC DLC {}\n",
                id.dlc_mismatch_frames, id.expected_dlc
            ));
        }
        if !id.uncovered_toggling_bits.is_empty() {
            let bits: Vec<String> = id
                .uncovered_toggling_bits
                .iter()
                .map(|b| b.to_string())
                .collect();
            out.push_str(&format!(
                "  UNEXPLAINED ACTIVITY: bits {} toggle but no signal covers them\n",
                bits.join(", ")
            ));
        }
        for sig in &id.signals {
            if sig.frames_decoded == 0 {
                out.push_str(&format!(
                    "  {} — never decoded ({} frames too short)\n",
                    sig.signal_name, sig.frames_too_short
                ));
                continue;
            }
            out.push_str(&format!(
                "  {} — raw [{}..{}], phys [{:.3}..{:.3}] over {} frames",
                sig.signal_name,
                sig.raw_min,
                sig.raw_max,
                sig.physical_min,
                sig.physical_max,
                sig.frames_decoded
            ));
            if sig.range_violations > 0 {
                out.push_str(&format!(" · {} RANGE VIOLATIONS", sig.range_violations));
            }
            if sig.never_toggles {
                out.push_str(" · never toggles (unconfirmable)");
            }
            out.push('\n');
        }
        out.push('\n');
    }
    if !report.unknown_ids.is_empty() {
        out.push_str("On bus but not in DBC:\n");
        for (id, ext, frames) in &report.unknown_ids {
            out.push_str(&format!("  {} ({frames} frames)\n", fmt_id(*id, *ext)));
        }
    }
    if !report.unseen_messages.is_empty() {
        out.push_str("In DBC but never seen on bus:\n");
        for (id, name) in &report.unseen_messages {
            out.push_str(&format!("  {} {}\n", fmt_id(*id, false), name));
        }
    }
    out
}
