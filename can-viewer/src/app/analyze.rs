//! Analyze tab: definitive CAN reverse-engineering survey over the loaded log.
//!
//! Runs [`survey_frames`] on the MDF4 currently selected in the header and
//! renders the proven facts (bit activity, counters, checksums, mux) — every
//! number shown is measured over all frames of the ID.

use super::helpers::vec_model;
use crate::services::{load_mdf4, survey_frames, BitState, IdSurvey, SurveyOptions, SurveyReport};
use crate::state::AppState;
use crate::{
    AnalyzeBitCell, AnalyzeBitRow, AnalyzeFactRow, AnalyzeIdRow, SigmaDiagnostics,
};
use parking_lot::Mutex;
use slint::Weak;
use std::sync::Arc;

/// Analyze tab controller.
pub struct AnalyzeController {
    state: Arc<AppState>,
    ui: Weak<SigmaDiagnostics>,
    report: Mutex<Option<SurveyReport>>,
}

impl AnalyzeController {
    pub fn new(state: Arc<AppState>, ui: Weak<SigmaDiagnostics>) -> Self {
        Self {
            state,
            ui,
            report: Mutex::new(None),
        }
    }

    pub fn wire(self: Arc<Self>, ui: &SigmaDiagnostics) {
        ui.on_run_analysis({
            let this = self.clone();
            move || this.run_analysis()
        });
        ui.on_analyze_id_selected({
            let this = self.clone();
            move |index| this.select_id(index)
        });
    }

    fn with_ui<F: FnOnce(&SigmaDiagnostics)>(&self, f: F) {
        if let Some(ui) = self.ui.upgrade() {
            f(&ui);
        }
    }

    fn run_analysis(&self) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        let path = ui.get_mdf4_path().to_string();
        if path.is_empty() {
            ui.set_analyze_status("Load an MDF4 log first (header MDF4 button).".into());
            return;
        }
        ui.set_analyze_status("Analyzing…".into());

        let frames = match load_mdf4(&path, &self.state) {
            Ok((frames, _)) => frames,
            Err(e) => {
                ui.set_analyze_status(format!("Load failed: {e}").into());
                return;
            }
        };
        let report = survey_frames(&frames, &SurveyOptions::default());

        let rows: Vec<AnalyzeIdRow> = report.ids.iter().map(id_row).collect();
        ui.set_analyze_ids(vec_model(&rows));
        ui.set_analyze_done(true);
        ui.set_analyze_selected_id(-1);
        ui.set_analyze_bit_rows(vec_model::<AnalyzeBitRow>(&[]));
        ui.set_analyze_facts(vec_model::<AnalyzeFactRow>(&[]));
        ui.set_analyze_status(
            format!(
                "{} frames · {} IDs · {:.1} s",
                report.total_frames,
                report.unique_ids,
                report.last_timestamp - report.first_timestamp
            )
            .into(),
        );
        *self.report.lock() = Some(report);
    }

    fn select_id(&self, index: i32) {
        if index < 0 {
            return;
        }
        let guard = self.report.lock();
        let Some(report) = guard.as_ref() else {
            return;
        };
        let Some(id) = report.ids.get(index as usize) else {
            return;
        };

        let bit_rows = bit_rows(id);
        let facts = fact_rows(id);
        let title = format!(
            "{} · {} frames · {} distinct payloads",
            fmt_id(id.can_id, id.is_extended),
            id.frames,
            id.distinct_payloads
        );

        self.with_ui(|ui| {
            ui.set_analyze_selected_id(index);
            ui.set_analyze_detail_title(title.clone().into());
            ui.set_analyze_bit_rows(vec_model(&bit_rows));
            ui.set_analyze_facts(vec_model(&facts));
        });
    }
}

fn fmt_id(can_id: u32, is_extended: bool) -> String {
    if is_extended {
        format!("{can_id:08X}x")
    } else {
        format!("{can_id:03X}")
    }
}

fn id_row(id: &IdSurvey) -> AnalyzeIdRow {
    let period = id
        .timing
        .as_ref()
        .map(|t| format!("{:.1} ms", t.median_ms))
        .unwrap_or_else(|| "—".to_string());

    let mut facts = Vec::new();
    if !id.counters.is_empty() {
        facts.push(format!("{} counter(s)", id.counters.len()));
    }
    if !id.checksums.is_empty() {
        facts.push(format!("{} checksum(s)", id.checksums.len()));
    }
    if !id.mux.is_empty() {
        facts.push(format!("{} mux candidate(s)", id.mux.len()));
    }
    AnalyzeIdRow {
        can_id: fmt_id(id.can_id, id.is_extended).into(),
        frames: id.frames.to_string().into(),
        period: period.into(),
        distinct: id.distinct_payloads.to_string().into(),
        facts: facts.join(" · ").into(),
    }
}

fn bit_rows(id: &IdSurvey) -> Vec<AnalyzeBitRow> {
    id.bit_activity
        .chunks(8)
        .enumerate()
        .map(|(byte_index, cells)| {
            // cells[0] = bit0 … cells[7] = bit7; display MSB-first (bit7 left).
            let pips: Vec<AnalyzeBitCell> = (0..8)
                .rev()
                .map(|bit_in_byte| {
                    let state = cells
                        .get(bit_in_byte)
                        .map(|c| match c.state() {
                            BitState::Always0 => 0,
                            BitState::Always1 => 1,
                            BitState::Toggles => 2,
                            BitState::Unobserved => 3,
                        })
                        .unwrap_or(3);
                    AnalyzeBitCell { state }
                })
                .collect();
            AnalyzeBitRow {
                byte_label: format!("Byte {byte_index}").into(),
                cells: vec_model(&pips),
            }
        })
        .collect()
}

fn fact_rows(id: &IdSurvey) -> Vec<AnalyzeFactRow> {
    let mut rows = Vec::new();
    for c in &id.counters {
        rows.push(AnalyzeFactRow {
            kind: "COUNTER".into(),
            text: format!(
                "{} start {} len {} — +1 mod 2^{} for all {} transitions ({} wraps)",
                c.field.endianness.label(),
                c.field.start_bit,
                c.field.length,
                c.field.length,
                c.transitions,
                c.wraps
            )
            .into(),
        });
    }
    for c in &id.checksums {
        rows.push(AnalyzeFactRow {
            kind: "CHECKSUM".into(),
            text: format!(
                "byte {} = {} — matched all {} frames ({} distinct)",
                c.byte_index,
                c.algorithm.describe(),
                c.frames,
                c.distinct_payloads
            )
            .into(),
        });
    }
    for m in &id.mux {
        rows.push(AnalyzeFactRow {
            kind: "MUX".into(),
            text: format!(
                "selector start {} len {} — {} values with differing layouts",
                m.selector.start_bit,
                m.selector.length,
                m.distinct_values
            )
            .into(),
        });
    }
    if let Some(note) = &id.checksum_note {
        rows.push(AnalyzeFactRow {
            kind: "note".into(),
            text: format!("checksum sweep {note}").into(),
        });
    }
    if rows.is_empty() {
        rows.push(AnalyzeFactRow {
            kind: "—".into(),
            text: "No structural proofs for this ID (see the bit map for raw activity).".into(),
        });
    }
    rows
}
