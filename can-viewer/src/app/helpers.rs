//! Shared UI helpers.

use crate::config::SessionConfig;
use slint::{ModelRc, VecModel};
use std::path::{Path, PathBuf};

/// Wrap a slice in a Slint `VecModel`.
pub fn vec_model<T: Clone + 'static>(items: &[T]) -> ModelRc<T> {
    let model = VecModel::default();
    for item in items {
        model.push(item.clone());
    }
    ModelRc::new(model)
}

/// CAN id as hex, 3 digits standard / 8 digits extended.
pub fn format_can_id(id: u32, extended: bool) -> String {
    if extended {
        format!("0x{id:08X}")
    } else {
        format!("0x{id:03X}")
    }
}

/// Payload bytes as space-separated uppercase hex.
pub fn format_data_hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Parse a CAN id, accepting `0x` prefixes and bare hex.
pub fn parse_hex_id(s: &str) -> Option<u32> {
    let s = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    u32::from_str_radix(s, 16).ok()
}

/// Parse a comma/space-separated CAN id filter list.
pub fn parse_can_ids(text: &str) -> Vec<u32> {
    text.split([',', ' ', ';'])
        .filter_map(parse_hex_id)
        .collect()
}

/// Show a message in the status bar.
pub fn set_status(ui: &crate::SigmaDiagnostics, message: &str) {
    ui.set_status_text(message.into());
}

/// Directory that is guaranteed to exist for file dialogs (avoids portal errors
/// when the last-used path was under the old `can-viewer` tree).
fn dialog_start_dir() -> PathBuf {
    let session = SessionConfig::load_raw();
    for candidate in [
        session
            .mdf4_path
            .as_ref()
            .and_then(|p| Path::new(p).parent().map(|d| d.to_path_buf())),
        session
            .dbc_path
            .as_ref()
            .and_then(|p| Path::new(p).parent().map(|d| d.to_path_buf())),
        SessionConfig::config_dir(),
        dirs::home_dir(),
    ]
    .into_iter()
    .flatten()
    {
        if candidate.is_dir() {
            return candidate;
        }
    }
    std::env::temp_dir()
}

/// Native open-file dialog; `None` when cancelled.
pub fn pick_open_file(title: &str, extensions: &[(&str, &[&str])]) -> Option<String> {
    let mut dialog = rfd::FileDialog::new()
        .set_title(title)
        .set_directory(dialog_start_dir());
    for (name, exts) in extensions {
        dialog = dialog.add_filter(*name, exts);
    }
    dialog.pick_file().map(|p| p.display().to_string())
}

/// Native save-file dialog; `None` when cancelled.
pub fn pick_save_file(title: &str, extensions: &[(&str, &[&str])]) -> Option<String> {
    let mut dialog = rfd::FileDialog::new()
        .set_title(title)
        .set_directory(dialog_start_dir());
    for (name, exts) in extensions {
        dialog = dialog.add_filter(*name, exts);
    }
    dialog.save_file().map(|p| p.display().to_string())
}

/// Push the four live-capture row models from a `LiveCaptureDisplay` onto a UI.
///
/// Each Slint app compiles its own structurally-identical `Live*Row` types, so
/// the mapping is generated per call site instead of written once against a
/// concrete UI type. Pass the UI handle and the display snapshot.
#[macro_export]
macro_rules! apply_live_rows {
    ($ui:expr, $display:expr) => {{
        let display = $display;
        let messages: Vec<LiveMessageRow> = display
            .messages
            .iter()
            .map(|m| LiveMessageRow {
                can_id: m.can_id.clone().into(),
                message_name: m.message_name.clone().into(),
                data_hex: m.data_hex.clone().into(),
                count: m.count.clone().into(),
                rate: m.rate.clone().into(),
            })
            .collect();
        let signals: Vec<LiveSignalRow> = display
            .signals
            .iter()
            .map(|s| LiveSignalRow {
                message_name: s.message_name.clone().into(),
                signal_name: s.signal_name.clone().into(),
                value: s.value.clone().into(),
                unit: s.unit.clone().into(),
                min_value: s.min_value.clone().into(),
                max_value: s.max_value.clone().into(),
            })
            .collect();
        let frames: Vec<LiveFrameRow> = display
            .frames
            .iter()
            .map(|f| LiveFrameRow {
                timestamp: f.timestamp.clone().into(),
                can_id: f.can_id.clone().into(),
                dlc: f.dlc.clone().into(),
                data_hex: f.data_hex.clone().into(),
                flags: f.flags.clone().into(),
            })
            .collect();
        let errors: Vec<LiveErrorRow> = display
            .errors
            .iter()
            .map(|e| LiveErrorRow {
                timestamp: e.timestamp.clone().into(),
                channel: e.channel.clone().into(),
                error_type: e.error_type.clone().into(),
                details: e.details.clone().into(),
                count: e.count.clone().into(),
            })
            .collect();
        $ui.set_live_messages(slint::ModelRc::new(slint::VecModel::from(messages)));
        $ui.set_live_signals(slint::ModelRc::new(slint::VecModel::from(signals)));
        $ui.set_live_frames(slint::ModelRc::new(slint::VecModel::from(frames)));
        $ui.set_live_errors(slint::ModelRc::new(slint::VecModel::from(errors)));
    }};
}
