//! MDF4 inspector tab controller.

use super::helpers::{
    format_can_id, format_data_hex, parse_can_ids, pick_open_file, pick_save_file, vec_model,
};
use crate::config::SessionConfig;
use crate::dto::{CanFrameDto, FrameTableRow};
use crate::services::{
    decode_single_frame, export_logs, fetch_latest_dbc_content, filter_frames, get_dbc_path,
    load_dbc, load_mdf4, save_dbc_content, FilterConfig, MatchStatus, UpdatesConfig,
};
use crate::state::AppState;
use crate::{FrameRow, SigmaDiagnostics, SignalRow};
use parking_lot::Mutex;
use slint::{ModelRc, Weak};
use std::path::Path;
use std::sync::{Arc, Weak as StdWeak};

use super::dbc::DbcController;

pub struct Mdf4Controller {
    state: Arc<AppState>,
    ui: Weak<SigmaDiagnostics>,
    dbc_editor: Mutex<StdWeak<DbcController>>,
    all_frames: Mutex<Vec<CanFrameDto>>,
    filtered_frames: Mutex<Vec<CanFrameDto>>,
}

impl Mdf4Controller {
    pub fn new(state: Arc<AppState>, ui: Weak<SigmaDiagnostics>) -> Self {
        Self {
            state,
            ui,
            dbc_editor: Mutex::new(StdWeak::new()),
            all_frames: Mutex::new(Vec::new()),
            filtered_frames: Mutex::new(Vec::new()),
        }
    }

    /// Wire the DBC editor so master DBC loads refresh that tab.
    pub fn set_dbc_editor(&self, dbc: StdWeak<DbcController>) {
        *self.dbc_editor.lock() = dbc;
    }

    pub fn wire(self: Arc<Self>, ui: &SigmaDiagnostics) {
        ui.on_open_mdf4({
            let this = self.clone();
            move || this.open_mdf4()
        });
        ui.on_mdf4_open_dbc({
            let this = self.clone();
            move || this.open_dbc()
        });
        ui.on_export_mdf4({
            let this = self.clone();
            move || this.export_mdf4()
        });
        ui.on_apply_filters({
            let this = self.clone();
            move || this.apply_filters()
        });
        ui.on_clear_filters({
            let this = self.clone();
            move || this.clear_filters()
        });
        ui.on_frame_selected({
            let this = self.clone();
            move |index| this.select_frame(index)
        });
    }

    /// Refresh after a DBC is loaded from another source (e.g. updates catalog).
    pub fn on_external_dbc_loaded(&self) {
        self.refresh_dbc_status();
        self.apply_filters();
        self.notify_dbc_editor();
    }

    /// Sync header DBC button from the shared app state (after editor save/new).
    pub fn sync_dbc_header_from_state(&self) {
        self.refresh_dbc_status();
        self.apply_filters();
    }

    fn notify_dbc_editor(&self) {
        if let Some(dbc) = self.dbc_editor.lock().upgrade() {
            dbc.on_external_dbc_loaded();
        }
    }

    pub fn load_initial_files(&self) {
        // Prefer the latest Sigma Racer DBC from updates (cached locally).
        if let Err(e) = self.load_latest_dbc_from_updates() {
            log::warn!("Updates DBC fetch skipped: {e}");
            let initial = crate::services::get_initial_files(&self.state);
            if let Some(path) = initial.dbc_path {
                match load_dbc(&path, &self.state) {
                    Ok(_) => {
                        self.refresh_dbc_status();
                        self.notify_dbc_editor();
                    }
                    Err(err) => log::warn!("Startup DBC load failed ({path}): {err}"),
                }
            }
        } else {
            self.notify_dbc_editor();
        }

        let initial = crate::services::get_initial_files(&self.state);
        if let Some(path) = initial.mdf4_path {
            self.load_mdf4_path(&path, true);
        }
    }

    fn with_ui<F: FnOnce(&SigmaDiagnostics)>(&self, f: F) {
        if let Some(ui) = self.ui.upgrade() {
            f(&ui);
        }
    }

    fn refresh_dbc_status(&self) {
        self.with_ui(|ui| {
            if let Some(path) = get_dbc_path(&self.state) {
                let name = Path::new(&path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&path);
                ui.set_mdf4_dbc_status(name.to_string().into());
                ui.set_mdf4_dbc_file_name(name.to_string().into());
            } else {
                ui.set_mdf4_dbc_status("No DBC".into());
                ui.set_mdf4_dbc_file_name("".into());
            }
        });
    }

    fn open_mdf4(&self) {
        if let Some(path) = pick_open_file("Open MDF4", &[("MDF4", &["mf4", "mdf"])]) {
            match self.cache_mdf4_file(Path::new(&path)) {
                Ok(cached) => self.load_mdf4_path(&cached, false),
                Err(e) => self.with_ui(|ui| {
                    ui.set_mdf4_frame_count(format!("Cache failed: {e}").into());
                }),
            }
        }
    }

    fn cache_mdf4_file(&self, src: &Path) -> Result<String, String> {
        let cache_dir =
            SessionConfig::mdf4_cache_dir().ok_or("Could not determine MDF4 cache directory")?;
        let dest = SessionConfig::cache_file(src, &cache_dir)?;
        Ok(dest.to_string_lossy().into_owned())
    }

    fn load_mdf4_path(&self, path: &str, startup: bool) {
        if !Path::new(path).is_file() {
            if startup {
                log::warn!("Startup MDF4 path not found: {path}");
            } else {
                self.with_ui(|ui| {
                    ui.set_mdf4_frame_count(format!("File not found: {path}").into());
                });
            }
            return;
        }

        match load_mdf4(path, &self.state) {
            Ok((frames, _decoded)) => {
                *self.all_frames.lock() = frames;
                self.apply_filters();
                self.with_ui(|ui| {
                    ui.set_mdf4_path(path.into());
                    let name = Path::new(path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(path);
                    ui.set_mdf4_file_name(name.to_string().into());
                });
            }
            Err(e) => {
                if startup {
                    log::warn!("Startup MDF4 load failed ({path}): {e}");
                } else {
                    self.with_ui(|ui| {
                        ui.set_mdf4_frame_count(format!("Load failed: {e}").into());
                    });
                }
            }
        }
    }

    /// Header DBC button: fetch latest Sigma Racer DBC from updates and cache it.
    /// Falls back to a local file picker if updates is unreachable.
    fn open_dbc(&self) {
        match self.load_latest_dbc_from_updates() {
            Ok(()) => {}
            Err(updates_err) => {
                log::warn!("Updates DBC unavailable ({updates_err}); falling back to file picker");
                if let Some(path) = pick_open_file("Open DBC", &[("DBC", &["dbc"])]) {
                    match self.cache_and_load_local_dbc(Path::new(&path)) {
                        Ok(()) => {}
                        Err(e) => self.with_ui(|ui| {
                            ui.set_mdf4_frame_count(format!("DBC load failed: {e}").into());
                        }),
                    }
                } else {
                    self.with_ui(|ui| {
                        ui.set_mdf4_frame_count(format!("DBC unavailable: {updates_err}").into());
                    });
                }
            }
        }
    }

    fn load_latest_dbc_from_updates(&self) -> Result<(), String> {
        let cfg = UpdatesConfig::from_env();
        let (filename, content) = fetch_latest_dbc_content(&cfg)?;
        let cache_path = SessionConfig::cache_dbc_bytes(&filename, &content)?;
        let path = cache_path.to_string_lossy().into_owned();
        save_dbc_content(&path, &content, &self.state)?;
        self.refresh_dbc_status();
        self.apply_filters();
        self.notify_dbc_editor();
        Ok(())
    }

    fn cache_and_load_local_dbc(&self, src: &Path) -> Result<(), String> {
        let cache_dir =
            SessionConfig::dbc_cache_dir().ok_or("Could not determine DBC cache directory")?;
        let dest = SessionConfig::cache_file(src, &cache_dir)?;
        let path = dest.to_string_lossy().into_owned();
        load_dbc(&path, &self.state)?;
        self.refresh_dbc_status();
        self.apply_filters();
        self.notify_dbc_editor();
        Ok(())
    }

    fn export_mdf4(&self) {
        let frames = self.filtered_frames.lock().clone();
        if frames.is_empty() {
            self.with_ui(|ui| {
                ui.set_mdf4_frame_count("No frames to export".into());
            });
            return;
        }
        if let Some(path) = pick_save_file("Export MDF4", &[("MDF4", &["mf4"])]) {
            match export_logs(&path, &frames) {
                Ok(count) => self.with_ui(|ui| {
                    ui.set_mdf4_frame_count(format!("Exported {count} frames").into());
                }),
                Err(e) => self.with_ui(|ui| {
                    ui.set_mdf4_frame_count(e.into());
                }),
            }
        }
    }

    fn apply_filters(&self) {
        let all = self.all_frames.lock().clone();
        self.with_ui(|ui| {
            let can_id_text = ui.get_mdf4_filter_can_id().to_string();
            let message_text = ui.get_mdf4_filter_message().to_string();
            let data_text = ui.get_mdf4_filter_data().to_string();

            let mut filters = FilterConfig::default();
            let can_ids = parse_can_ids(&can_id_text);
            if !can_ids.is_empty() {
                filters.can_ids = can_ids;
            }
            if !message_text.is_empty() {
                filters.messages = vec![message_text];
            }
            if !data_text.is_empty() {
                filters.data_pattern = Some(data_text);
            }
            filters.match_status = MatchStatus::All;

            let result = filter_frames(all, filters, &self.state);
            *self.filtered_frames.lock() = result.frames.clone();
            self.update_frame_table(&result.frames);
            ui.set_mdf4_frame_count(
                format!(
                    "Loaded {} / {} frames",
                    result.filtered_count, result.total_count
                )
                .into(),
            );
            ui.set_mdf4_selected_frame(-1);
            ui.set_mdf4_signals(ModelRc::new(slint::VecModel::default()));
        });
    }

    fn clear_filters(&self) {
        self.with_ui(|ui| {
            ui.set_mdf4_filter_can_id("".into());
            ui.set_mdf4_filter_message("".into());
            ui.set_mdf4_filter_data("".into());
        });
        self.apply_filters();
    }

    fn update_frame_table(&self, frames: &[CanFrameDto]) {
        let rows: Vec<FrameTableRow> = frames
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let message_name = self.message_name_for(f);
                FrameTableRow {
                    index: i as i32,
                    timestamp: format!("{:.3}", f.timestamp),
                    can_id: format_can_id(f.can_id, f.is_extended),
                    channel: f.channel.clone(),
                    dlc: f.dlc.to_string(),
                    data_hex: format_data_hex(&f.data),
                    message_name,
                }
            })
            .collect();

        let slint_rows: Vec<FrameRow> = rows
            .iter()
            .map(|r| FrameRow {
                index: r.index,
                timestamp: r.timestamp.clone().into(),
                can_id: r.can_id.clone().into(),
                channel: r.channel.clone().into(),
                dlc: r.dlc.clone().into(),
                data_hex: r.data_hex.clone().into(),
                message_name: r.message_name.clone().into(),
            })
            .collect();

        self.with_ui(|ui| {
            ui.set_mdf4_frames(vec_model(&slint_rows));
        });
    }

    fn message_name_for(&self, frame: &CanFrameDto) -> String {
        let fast = self.state.fast_dbc.lock();
        if let Some(ref fdbc) = *fast {
            let msg = if frame.is_extended {
                fdbc.get_extended(frame.can_id)
            } else {
                fdbc.get(frame.can_id)
            };
            return msg.map(|m| m.name().to_string()).unwrap_or_default();
        }
        String::new()
    }

    fn select_frame(&self, index: i32) {
        if index < 0 {
            return;
        }
        let frames = self.filtered_frames.lock();
        let Some(frame) = frames.get(index as usize) else {
            return;
        };

        let response = decode_single_frame(frame, &self.state);
        let signal_rows: Vec<SignalRow> = response
            .signals
            .iter()
            .map(|s| SignalRow {
                signal_name: s.signal_name.clone().into(),
                value: format!("{:.4}", s.value).into(),
                unit: s.unit.clone().into(),
                raw_value: s.raw_value.to_string().into(),
                description: s.description.clone().unwrap_or_default().into(),
            })
            .collect();

        self.with_ui(|ui| {
            ui.set_mdf4_selected_frame(index);
            ui.set_mdf4_signals(vec_model(&signal_rows));
        });
    }
}
