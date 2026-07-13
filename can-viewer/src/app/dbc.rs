//! DBC editor tab controller.

use super::helpers::{parse_hex_id, pick_save_file, set_status, vec_model};
use super::mdf4::Mdf4Controller;
use crate::dbc_export::empty_dbc_info;
use crate::dto::{DbcInfo, MessageInfo, NodeInfo, SignalInfo};
use crate::services::{get_dbc_info, get_dbc_path, save_dbc_info};
use crate::state::AppState;
use crate::{
    DbcAttributeRow as UiDbcAttributeRow, DbcBitBand as UiDbcBitBand, DbcBitCell as UiDbcBitCell,
    DbcBitRow as UiDbcBitRow, DbcMessageRow as UiDbcMessageRow, DbcNodeRow as UiDbcNodeRow,
    DbcSignalRow as UiDbcSignalRow, DbcValueDescRow as UiDbcValueDescRow, SigmaDiagnostics,
};
use parking_lot::Mutex;
use slint::Weak;
use std::sync::{Arc, Weak as StdWeak};

pub struct DbcController {
    state: Arc<AppState>,
    ui: Weak<SigmaDiagnostics>,
    /// Header MDF4/DBC buttons own file loading; this tab edits that master DBC.
    file_master: Mutex<StdWeak<Mdf4Controller>>,
    edit_state: Mutex<DbcInfo>,
    path: Mutex<Option<String>>,
    dirty: Mutex<bool>,
    /// Snapshot taken when entering edit / before New — restored by Undo.
    undo_snapshot: Mutex<Option<EditSnapshot>>,
    selected_message: Mutex<i32>,
    selected_signal: Mutex<i32>,
    selected_node: Mutex<i32>,
}

#[derive(Clone)]
struct EditSnapshot {
    info: DbcInfo,
    path: Option<String>,
    dirty: bool,
}

enum SaveTarget {
    ExistingPathOrPrompt,
    PickPath,
}

impl DbcController {
    pub fn new(state: Arc<AppState>, ui: Weak<SigmaDiagnostics>) -> Self {
        Self {
            state,
            ui,
            file_master: Mutex::new(StdWeak::new()),
            edit_state: Mutex::new(empty_dbc_info()),
            path: Mutex::new(None),
            dirty: Mutex::new(false),
            undo_snapshot: Mutex::new(None),
            selected_message: Mutex::new(-1),
            selected_signal: Mutex::new(-1),
            selected_node: Mutex::new(-1),
        }
    }

    pub fn set_file_master(&self, mdf4: StdWeak<Mdf4Controller>) {
        *self.file_master.lock() = mdf4;
    }

    pub fn wire(self: Arc<Self>, ui: &SigmaDiagnostics) {
        if let Ok(Some(info)) = get_dbc_info(&self.state) {
            *self.edit_state.lock() = info;
            *self.path.lock() = get_dbc_path(&self.state);
            self.refresh_ui();
        }

        let this = self.clone();
        ui.on_new_dbc({
            let t = this.clone();
            move || t.new_dbc()
        });
        ui.on_begin_edit({
            let t = this.clone();
            move || t.begin_edit()
        });
        ui.on_undo_dbc({
            let t = this.clone();
            move || t.undo_dbc()
        });
        ui.on_save_dbc({
            let t = this.clone();
            move || t.save_dbc()
        });
        ui.on_save_dbc_as({
            let t = this.clone();
            move || t.save_dbc_as()
        });
        ui.on_field_edited({
            let t = this.clone();
            move || t.field_edited()
        });
        ui.on_add_message({
            let t = this.clone();
            move || t.add_message()
        });
        ui.on_delete_message({
            let t = this.clone();
            move || t.delete_message()
        });
        ui.on_add_signal({
            let t = this.clone();
            move || t.add_signal()
        });
        ui.on_delete_signal({
            let t = this.clone();
            move || t.delete_signal()
        });
        ui.on_add_node({
            let t = this.clone();
            move || t.add_node()
        });
        ui.on_delete_node({
            let t = this.clone();
            move || t.delete_node()
        });
        ui.on_message_selected({
            let t = this.clone();
            move |i| t.select_message(i)
        });
        ui.on_signal_selected({
            let t = this.clone();
            move |i| t.select_signal(i)
        });
        ui.on_node_selected({
            let t = this.clone();
            move |i| t.select_node(i)
        });
        ui.on_refresh_bit_map({
            let t = this.clone();
            move || t.refresh_bit_map()
        });
        ui.on_bit_cell_clicked({
            let t = this.clone();
            move |sig_idx| {
                if sig_idx >= 0 {
                    t.select_signal(sig_idx);
                }
            }
        });
    }

    /// Reload editor state after a DBC is fetched from sigma-updates.
    pub fn on_external_dbc_loaded(&self) {
        if let Ok(Some(info)) = get_dbc_info(&self.state) {
            *self.edit_state.lock() = info;
            *self.path.lock() = get_dbc_path(&self.state);
            *self.dirty.lock() = false;
            *self.selected_message.lock() = -1;
            *self.selected_signal.lock() = -1;
            *self.selected_node.lock() = -1;
            self.refresh_ui();
        }
    }

    fn with_ui<F: FnOnce(&SigmaDiagnostics)>(&self, f: F) {
        if let Some(ui) = self.ui.upgrade() {
            f(&ui);
        }
    }

    fn mark_dirty(&self) {
        *self.dirty.lock() = true;
        self.with_ui(|ui| ui.set_dbc_dirty(true));
    }

    fn refresh_ui(&self) {
        // Clone everything first — never hold edit_state across Slint property updates
        // (those can re-enter selection callbacks and deadlock on the same mutex).
        let (display_path, dirty, selected_message, selected_signal, selected_node, info) = {
            let info = self.edit_state.lock().clone();
            let path = self.path.lock().clone();
            let dirty = *self.dirty.lock();
            let selected_message = *self.selected_message.lock();
            let selected_signal = *self.selected_signal.lock();
            let selected_node = *self.selected_node.lock();
            let display_path = path
                .as_ref()
                .map(|p| {
                    std::path::Path::new(p)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(p)
                        .to_string()
                })
                .unwrap_or_default();
            (
                display_path,
                dirty,
                selected_message,
                selected_signal,
                selected_node,
                info,
            )
        };

        let messages: Vec<UiDbcMessageRow> = info.messages.iter().map(message_row).collect();
        let signals: Vec<UiDbcSignalRow> = selected_message_signals(&info, selected_message);
        let nodes: Vec<UiDbcNodeRow> = info.nodes.iter().map(node_row).collect();
        let attributes: Vec<UiDbcAttributeRow> = info
            .attribute_definitions
            .iter()
            .map(|a| UiDbcAttributeRow {
                name: a.name.clone().into(),
                object_type: a.object_type.clone().into(),
                value_summary: attribute_value_summary(&a.value_type).into(),
            })
            .collect();
        let value_descriptions: Vec<UiDbcValueDescRow> = info
            .value_descriptions
            .iter()
            .map(|vd| UiDbcValueDescRow {
                message_id: format!("0x{:X}", vd.message_id).into(),
                signal_name: vd.signal_name.clone().into(),
                entries: vd
                    .descriptions
                    .iter()
                    .map(|d| format!("{}=\"{}\"", d.value, d.description))
                    .collect::<Vec<_>>()
                    .join(", ")
                    .into(),
            })
            .collect();

        self.with_ui(|ui| {
            ui.set_dbc_path(display_path.into());
            ui.set_dbc_dirty(dirty);
            ui.set_dbc_version(info.version.clone().unwrap_or_default().into());
            ui.set_dbc_comment(info.comment.clone().unwrap_or_default().into());
            ui.set_dbc_messages(vec_model(&messages));
            ui.set_dbc_signals(vec_model(&signals));
            ui.set_dbc_nodes(vec_model(&nodes));
            ui.set_dbc_attributes(vec_model(&attributes));
            ui.set_dbc_value_descriptions(vec_model(&value_descriptions));
            ui.set_dbc_selected_message(selected_message);
            ui.set_dbc_selected_signal(selected_signal);
            ui.set_dbc_selected_node(selected_node);
        });

        let refresh_bit_map = self
            .ui
            .upgrade()
            .is_some_and(|ui| ui.get_dbc_editor_panel() == 0 && selected_message >= 0);
        if refresh_bit_map {
            self.refresh_bit_map();
        }
    }

    fn apply_edits_from_ui(&self) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        // View mode must never mutate state or mark dirty (e.g. expanding a message).
        if !ui.get_dbc_editing() {
            return;
        }
        let panel = ui.get_dbc_editor_panel();
        let version = ui.get_dbc_version().to_string();
        let comment = ui.get_dbc_comment().to_string();
        let msg_idx = *self.selected_message.lock();
        let sig_idx = *self.selected_signal.lock();
        let node_idx = *self.selected_node.lock();

        let msg_id = ui.get_dbc_edit_message_id().to_string();
        let msg_name = ui.get_dbc_edit_message_name().to_string();
        let msg_dlc = ui.get_dbc_edit_message_dlc().to_string();
        let msg_sender = ui.get_dbc_edit_message_sender().to_string();

        let sig_name = ui.get_dbc_edit_signal_name().to_string();
        let sig_start = ui.get_dbc_edit_signal_start().to_string();
        let sig_length = ui.get_dbc_edit_signal_length().to_string();
        let sig_factor = ui.get_dbc_edit_signal_factor().to_string();
        let sig_offset = ui.get_dbc_edit_signal_offset().to_string();
        let sig_unit = ui.get_dbc_edit_signal_unit().to_string();
        let sig_byte_order = ui.get_dbc_edit_signal_byte_order().to_string();

        let node_name = ui.get_dbc_edit_node_name().to_string();
        let node_comment = ui.get_dbc_edit_node_comment().to_string();

        let changed = {
            let mut info = self.edit_state.lock();
            let before = info.clone();

            // Keep Option semantics: empty UI text == None (avoids false dirty).
            info.version = if version.is_empty() {
                None
            } else {
                Some(version)
            };
            info.comment = if comment.is_empty() {
                None
            } else {
                Some(comment)
            };

            // 0 Messages (+ nested signals), 1 Nodes, 2 Header, 3 Attributes, 4 Val Desc
            if panel == 0 {
                apply_message_edits_values(
                    &mut info,
                    msg_idx,
                    &msg_id,
                    &msg_name,
                    &msg_dlc,
                    &msg_sender,
                );
                apply_signal_edits_values(
                    &mut info,
                    msg_idx,
                    sig_idx,
                    &sig_name,
                    &sig_start,
                    &sig_length,
                    &sig_factor,
                    &sig_offset,
                    &sig_unit,
                    &sig_byte_order,
                );
            } else if panel == 1 {
                apply_node_edits_values(&mut info, node_idx, &node_name, &node_comment);
            }
            *info != before
        };
        if changed {
            self.mark_dirty();
        }
    }

    fn capture_undo_snapshot(&self) {
        *self.undo_snapshot.lock() = Some(EditSnapshot {
            info: self.edit_state.lock().clone(),
            path: self.path.lock().clone(),
            dirty: *self.dirty.lock(),
        });
    }

    fn begin_edit(&self) {
        self.capture_undo_snapshot();
    }

    /// Apply pending field edits; enable Save when data actually changed.
    fn field_edited(&self) {
        self.apply_edits_from_ui();
        self.refresh_bit_map();
    }

    fn undo_dbc(&self) {
        let Some(snapshot) = self.undo_snapshot.lock().clone() else {
            self.with_ui(|ui| set_status(ui, "Nothing to undo"));
            return;
        };
        *self.edit_state.lock() = snapshot.info;
        *self.path.lock() = snapshot.path;
        *self.dirty.lock() = snapshot.dirty;
        *self.selected_message.lock() = -1;
        *self.selected_signal.lock() = -1;
        *self.selected_node.lock() = -1;
        self.refresh_ui();
        self.sync_master_header();
        self.with_ui(|ui| set_status(ui, "Reverted changes"));
    }

    fn new_dbc(&self) {
        self.capture_undo_snapshot();
        *self.edit_state.lock() = empty_dbc_info();
        *self.path.lock() = None;
        *self.dirty.lock() = false;
        *self.selected_message.lock() = -1;
        *self.selected_signal.lock() = -1;
        *self.selected_node.lock() = -1;
        let _ = crate::services::clear_dbc(&self.state);
        self.refresh_ui();
        self.sync_master_header();
        self.with_ui(|ui| set_status(ui, "New DBC created"));
    }

    /// Load/reload is owned by the header DBC master button (`mdf4-open-dbc`).
    fn sync_master_header(&self) {
        if let Some(master) = self.file_master.lock().upgrade() {
            master.sync_dbc_header_from_state();
        }
    }

    fn save_dbc(self: &Arc<Self>) {
        let this = Arc::clone(self);
        slint::Timer::single_shot(std::time::Duration::from_millis(1), move || {
            this.run_save(SaveTarget::ExistingPathOrPrompt);
        });
    }

    fn save_dbc_as(self: &Arc<Self>) {
        let this = Arc::clone(self);
        slint::Timer::single_shot(std::time::Duration::from_millis(1), move || {
            this.run_save(SaveTarget::PickPath);
        });
    }

    fn run_save(self: &Arc<Self>, target: SaveTarget) {
        self.apply_edits_from_ui();
        let path = match target {
            SaveTarget::ExistingPathOrPrompt => self
                .path
                .lock()
                .clone()
                .or_else(|| pick_save_file("Save DBC", &[("DBC", &["dbc"])])),
            SaveTarget::PickPath => pick_save_file("Save DBC", &[("DBC", &["dbc"])]),
        };
        let Some(path) = path else {
            return;
        };

        let info = self.edit_state.lock().clone();
        match save_dbc_info(&path, &info, &self.state) {
            Ok(()) => {
                *self.path.lock() = Some(path.clone());
                *self.dirty.lock() = false;
                self.capture_undo_snapshot();

                let this = Arc::clone(self);
                // Refresh UI on the next tick — avoid re-entering Slint from save work.
                slint::Timer::single_shot(std::time::Duration::from_millis(1), move || {
                    this.refresh_ui();
                    this.sync_master_header();
                    this.with_ui(|ui| {
                        ui.set_dbc_dirty(false);
                        ui.set_dbc_editing(false);
                        set_status(ui, &format!("Saved {path}"));
                    });
                });
            }
            Err(e) => self.with_ui(|ui| set_status(ui, &e)),
        }
    }

    fn add_message(&self) {
        self.apply_edits_from_ui();
        let new_index = {
            let mut info = self.edit_state.lock();
            let id = next_message_id(&info);
            info.messages.push(MessageInfo {
                id,
                is_extended: false,
                name: format!("Message_{id}"),
                dlc: 8,
                sender: "Vector__XXX".to_string(),
                signals: Vec::new(),
                comment: None,
            });
            (info.messages.len() as i32) - 1
        };
        *self.selected_message.lock() = new_index;
        *self.selected_signal.lock() = -1;
        self.mark_dirty();
        self.refresh_ui();
        self.show_message(new_index);
    }

    fn delete_message(&self) {
        let idx = *self.selected_message.lock();
        if idx < 0 {
            return;
        }
        self.apply_edits_from_ui();
        {
            let mut info = self.edit_state.lock();
            if let Ok(i) = usize::try_from(idx) {
                if i < info.messages.len() {
                    info.messages.remove(i);
                }
            }
        }
        *self.selected_message.lock() = -1;
        *self.selected_signal.lock() = -1;
        self.mark_dirty();
        self.refresh_ui();
    }

    fn add_signal(&self) {
        let msg_idx = *self.selected_message.lock();
        if msg_idx < 0 {
            self.with_ui(|ui| set_status(ui, "Select a message first"));
            return;
        }
        self.apply_edits_from_ui();
        let new_sig = {
            let mut info = self.edit_state.lock();
            let Some(msg) = info.messages.get_mut(msg_idx as usize) else {
                return;
            };
            msg.signals.push(SignalInfo {
                name: format!("Signal_{}", msg.signals.len()),
                start_bit: 0,
                length: 8,
                byte_order: "little_endian".to_string(),
                is_signed: false,
                factor: 1.0,
                offset: 0.0,
                min: 0.0,
                max: 0.0,
                unit: String::new(),
                receivers: vec!["Vector__XXX".to_string()],
                is_multiplexer: false,
                multiplexer_value: None,
                comment: None,
            });
            (msg.signals.len() as i32) - 1
        };
        *self.selected_signal.lock() = new_sig;
        self.mark_dirty();
        self.refresh_ui();
        self.show_signal(new_sig);
    }

    fn delete_signal(&self) {
        let msg_idx = *self.selected_message.lock();
        let sig_idx = *self.selected_signal.lock();
        if msg_idx < 0 || sig_idx < 0 {
            return;
        }
        self.apply_edits_from_ui();
        {
            let mut info = self.edit_state.lock();
            if let Some(msg) = info.messages.get_mut(msg_idx as usize) {
                if let Ok(i) = usize::try_from(sig_idx) {
                    if i < msg.signals.len() {
                        msg.signals.remove(i);
                    }
                }
            }
        }
        *self.selected_signal.lock() = -1;
        self.mark_dirty();
        self.refresh_ui();
    }

    fn add_node(&self) {
        self.apply_edits_from_ui();
        let new_index = {
            let mut info = self.edit_state.lock();
            let name = format!("Node_{}", info.nodes.len());
            info.nodes.push(NodeInfo {
                name,
                comment: None,
            });
            (info.nodes.len() as i32) - 1
        };
        *self.selected_node.lock() = new_index;
        self.mark_dirty();
        self.refresh_ui();
        self.show_node(new_index);
    }

    fn delete_node(&self) {
        let idx = *self.selected_node.lock();
        if idx < 0 {
            return;
        }
        self.apply_edits_from_ui();
        {
            let mut info = self.edit_state.lock();
            if let Ok(i) = usize::try_from(idx) {
                if i < info.nodes.len() {
                    info.nodes.remove(i);
                }
            }
        }
        *self.selected_node.lock() = -1;
        self.mark_dirty();
        self.refresh_ui();
    }

    fn select_message(&self, index: i32) {
        self.apply_edits_from_ui();
        let current = *self.selected_message.lock();
        if current == index {
            *self.selected_message.lock() = -1;
            *self.selected_signal.lock() = -1;
            self.with_ui(|ui| {
                ui.set_dbc_selected_message(-1);
                ui.set_dbc_selected_signal(-1);
                ui.set_dbc_signals(vec_model(&[]));
                ui.set_dbc_bit_bands(slint::ModelRc::new(slint::VecModel::default()));
                ui.set_dbc_bit_dlc(0);
                ui.set_dbc_bit_summary("".into());
            });
            return;
        }
        *self.selected_message.lock() = index;
        *self.selected_signal.lock() = -1;
        self.show_message(index);
    }

    fn select_signal(&self, index: i32) {
        self.apply_edits_from_ui();
        *self.selected_signal.lock() = index;
        self.show_signal(index);
    }

    fn select_node(&self, index: i32) {
        self.apply_edits_from_ui();
        *self.selected_node.lock() = index;
        self.show_node(index);
    }

    fn show_message(&self, index: i32) {
        let info = self.edit_state.lock().clone();
        let signals = selected_message_signals(&info, index);
        self.with_ui(|ui| {
            ui.set_dbc_selected_message(index);
            ui.set_dbc_selected_signal(-1);
            if let Some(msg) = info.messages.get(index as usize) {
                ui.set_dbc_edit_message_id(format!("0x{:X}", msg.id).into());
                ui.set_dbc_edit_message_name(msg.name.clone().into());
                ui.set_dbc_edit_message_dlc(msg.dlc.to_string().into());
                ui.set_dbc_edit_message_sender(msg.sender.clone().into());
            }
            ui.set_dbc_signals(vec_model(&signals));
        });
        self.refresh_bit_map();
    }

    fn show_signal(&self, index: i32) {
        let msg_idx = *self.selected_message.lock();
        let info = self.edit_state.lock().clone();
        self.with_ui(|ui| {
            ui.set_dbc_selected_signal(index);
            if let Some(msg) = info.messages.get(msg_idx as usize) {
                if let Some(sig) = msg.signals.get(index as usize) {
                    ui.set_dbc_edit_signal_name(sig.name.clone().into());
                    ui.set_dbc_edit_signal_start(sig.start_bit.to_string().into());
                    ui.set_dbc_edit_signal_length(sig.length.to_string().into());
                    ui.set_dbc_edit_signal_factor(sig.factor.to_string().into());
                    ui.set_dbc_edit_signal_offset(sig.offset.to_string().into());
                    ui.set_dbc_edit_signal_unit(sig.unit.clone().into());
                    ui.set_dbc_edit_signal_byte_order(endian_label(&sig.byte_order).into());
                }
            }
        });
        self.refresh_bit_map();
    }

    /// Rebuild Intel bit map for the expanded message (live edit fields when editing).
    fn refresh_bit_map(&self) {
        let msg_idx = *self.selected_message.lock();
        let selected_signal = *self.selected_signal.lock();
        if msg_idx < 0 {
            self.with_ui(|ui| {
                ui.set_dbc_bit_bands(slint::ModelRc::new(slint::VecModel::default()));
                ui.set_dbc_bit_dlc(0);
                ui.set_dbc_bit_summary("".into());
            });
            return;
        }

        let info = self.edit_state.lock().clone();
        let Some(msg) = info.messages.get(msg_idx as usize).cloned() else {
            return;
        };

        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        let editing = ui.get_dbc_editing();
        let dlc = if editing {
            ui.get_dbc_edit_message_dlc()
                .parse::<u8>()
                .unwrap_or(msg.dlc)
                .clamp(0, 64)
        } else {
            msg.dlc
        };

        let mut msg = msg;
        let live_selected = if editing && selected_signal >= 0 {
            if let Some(sig) = msg.signals.get_mut(selected_signal as usize) {
                sig.byte_order = endian_storage(&ui.get_dbc_edit_signal_byte_order()).to_string();
            }
            Some((
                ui.get_dbc_edit_signal_start().parse::<u32>().unwrap_or(0),
                ui.get_dbc_edit_signal_length().parse::<u32>().unwrap_or(0),
                ui.get_dbc_edit_signal_name().to_string(),
            ))
        } else {
            None
        };

        let (rows, summary) = build_bit_map(&msg, dlc, selected_signal, live_selected.as_ref());
        ui.set_dbc_bit_dlc(i32::from(dlc));
        ui.set_dbc_bit_summary(summary.into());
        ui.set_dbc_bit_bands(bit_bands_model(&rows));
    }

    fn show_node(&self, index: i32) {
        let info = self.edit_state.lock().clone();
        self.with_ui(|ui| {
            ui.set_dbc_selected_node(index);
            if let Some(node) = info.nodes.get(index as usize) {
                ui.set_dbc_edit_node_name(node.name.clone().into());
                ui.set_dbc_edit_node_comment(node.comment.clone().unwrap_or_default().into());
            }
        });
    }
}

fn message_row(m: &MessageInfo) -> UiDbcMessageRow {
    UiDbcMessageRow {
        id: format!("0x{:X}", m.id).into(),
        name: m.name.clone().into(),
        dlc: m.dlc.to_string().into(),
        sender: m.sender.clone().into(),
        signal_count: m.signals.len().to_string().into(),
    }
}

fn node_row(n: &NodeInfo) -> UiDbcNodeRow {
    UiDbcNodeRow {
        name: n.name.clone().into(),
        comment: n.comment.clone().unwrap_or_default().into(),
    }
}

fn selected_message_signals(info: &DbcInfo, msg_idx: i32) -> Vec<UiDbcSignalRow> {
    info.messages
        .get(msg_idx as usize)
        .map(|m| {
            m.signals
                .iter()
                .map(|s| UiDbcSignalRow {
                    name: s.name.clone().into(),
                    start_bit: s.start_bit.to_string().into(),
                    length: s.length.to_string().into(),
                    byte_order: endian_label(&s.byte_order).into(),
                    factor: s.factor.to_string().into(),
                    unit: s.unit.clone().into(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn endian_label(byte_order: &str) -> &'static str {
    if byte_order == "big_endian" || byte_order == "motorola" || byte_order == "Motorola" {
        "Motorola"
    } else {
        "Intel"
    }
}

fn endian_storage(label: &str) -> &'static str {
    if label == "Motorola" || label == "big_endian" || label == "motorola" {
        "big_endian"
    } else {
        "little_endian"
    }
}

/// Bits occupied by a signal (DBC Intel `@1` or Motorola `@0` layout).
fn signal_occupied_bits(start_bit: u32, length: u32, byte_order: &str) -> Vec<u32> {
    if length == 0 {
        return Vec::new();
    }
    let big_endian = byte_order == "big_endian" || byte_order == "motorola";
    if big_endian {
        // Motorola: start_bit is MSB; walk toward LSB, wrapping to next byte's bit7.
        let mut bits = Vec::with_capacity(length as usize);
        let mut bit = start_bit;
        for _ in 0..length {
            bits.push(bit);
            if bit.is_multiple_of(8) {
                bit = bit.saturating_add(15);
            } else {
                bit = bit.saturating_sub(1);
            }
        }
        bits
    } else {
        // Intel: start_bit is LSB; consecutive ascending bit numbers.
        (start_bit..start_bit.saturating_add(length)).collect()
    }
}

/// Occupancy grid using each signal's byte order (Intel vs Motorola).
fn build_bit_map(
    msg: &MessageInfo,
    dlc: u8,
    selected_signal: i32,
    live_selected: Option<&(u32, u32, String)>,
) -> (Vec<(String, Vec<UiDbcBitCell>)>, String) {
    let capacity = (dlc as usize).saturating_mul(8);

    let signal_start_len = |sig_i: usize, sig: &SignalInfo| -> (u32, u32) {
        if selected_signal == sig_i as i32 {
            if let Some((s, l, _)) = live_selected {
                return (*s, *l);
            }
        }
        (sig.start_bit, sig.length)
    };

    let signal_bits = |sig_i: usize, sig: &SignalInfo| -> Vec<u32> {
        let (start, length) = signal_start_len(sig_i, sig);
        signal_occupied_bits(start, length, &sig.byte_order)
    };

    let mut max_bit = capacity;
    for (sig_i, sig) in msg.signals.iter().enumerate() {
        for bit in signal_bits(sig_i, sig) {
            max_bit = max_bit.max(bit as usize + 1);
        }
    }
    let display_bytes = max_bit.div_ceil(8).max(dlc as usize);
    let bit_count = display_bytes * 8;

    let mut occupancy: Vec<u32> = vec![0; bit_count];
    let mut owner: Vec<i32> = vec![-1; bit_count];
    let mut selected_bits: std::collections::HashSet<u32> = std::collections::HashSet::new();

    for (sig_i, sig) in msg.signals.iter().enumerate() {
        let bits = signal_bits(sig_i, sig);
        if sig_i as i32 == selected_signal {
            selected_bits.extend(bits.iter().copied());
        }
        for bit in bits {
            let idx = bit as usize;
            if idx >= bit_count {
                continue;
            }
            occupancy[idx] = occupancy[idx].saturating_add(1);
            if owner[idx] < 0 || sig_i as i32 == selected_signal {
                owner[idx] = sig_i as i32;
            }
        }
    }

    // state: 0 free, 1 used, 2 selected, 3 conflict (overlap or past DLC)
    let mut bits: Vec<(i32, i32)> = vec![(0, -1); bit_count];
    for bit in 0..bit_count {
        let past_dlc = bit >= capacity;
        let overlap = occupancy[bit] >= 2;
        let is_selected = selected_bits.contains(&(bit as u32));

        let state = if (past_dlc && occupancy[bit] > 0) || overlap {
            3
        } else if past_dlc {
            0
        } else if is_selected {
            2
        } else if occupancy[bit] > 0 {
            1
        } else {
            0
        };

        let sig_idx = if is_selected {
            selected_signal
        } else {
            owner[bit]
        };
        bits[bit] = (state, sig_idx);
    }

    let used = (0..capacity).filter(|&b| occupancy[b] > 0).count();
    let free = capacity.saturating_sub(used);
    let overlap_bits = (0..capacity).filter(|&b| occupancy[b] >= 2).count();
    let past_dlc_bits = (capacity..bit_count).filter(|&b| occupancy[b] > 0).count();

    let (sel_start, sel_len, sel_name, sel_endian) = if selected_signal >= 0 {
        if let Some(sig) = msg.signals.get(selected_signal as usize) {
            let (start, length) = signal_start_len(selected_signal as usize, sig);
            let name = if let Some((_, _, n)) = live_selected {
                n.clone()
            } else {
                sig.name.clone()
            };
            let endian = if sig.byte_order == "big_endian" || sig.byte_order == "motorola" {
                "Motorola"
            } else {
                "Intel"
            };
            (start, length, name, endian)
        } else {
            (0, 0, String::new(), "")
        }
    } else {
        (0, 0, String::new(), "")
    };

    let mut summary = format!("DLC {dlc} · {capacity} bits · used {used} · free {free}");
    if overlap_bits > 0 {
        summary.push_str(&format!(" · overlap {overlap_bits}"));
    }
    if past_dlc_bits > 0 {
        summary.push_str(&format!(" · past DLC {past_dlc_bits}"));
    }
    if selected_signal >= 0 && !sel_name.is_empty() {
        let range = if sel_endian == "Motorola" {
            format!("start {sel_start} len {sel_len} {sel_endian}")
        } else {
            let end = sel_start.saturating_add(sel_len.saturating_sub(1));
            format!("{sel_start}–{end} {sel_endian}")
        };
        summary.push_str(&format!(" · {sel_name} [{range}]"));
    }

    let mut rows = Vec::with_capacity(display_bytes);
    for byte in 0..display_bytes {
        let mut cells = Vec::with_capacity(8);
        for display_col in 0..8 {
            let linear = byte * 8 + (7 - display_col);
            let (state, sig_idx) = bits.get(linear).copied().unwrap_or((0, -1));
            cells.push(UiDbcBitCell {
                state,
                signal_index: sig_idx,
            });
        }
        let label = if byte < dlc as usize {
            format!("Byte {byte}")
        } else {
            format!("+{byte}")
        };
        rows.push((label, cells));
    }

    (rows, summary)
}

/// Chunk byte strips into bands of up to 4 (four columns, wrap to next row).
fn bit_bands_model(rows: &[(String, Vec<UiDbcBitCell>)]) -> slint::ModelRc<UiDbcBitBand> {
    const COLS: usize = 4;
    let bands_model = slint::VecModel::default();
    for chunk in rows.chunks(COLS) {
        let bytes = slint::VecModel::default();
        for (label, cells) in chunk {
            let cell_model = slint::VecModel::default();
            for c in cells {
                cell_model.push(c.clone());
            }
            bytes.push(UiDbcBitRow {
                byte_label: label.clone().into(),
                cells: slint::ModelRc::new(cell_model),
            });
        }
        bands_model.push(UiDbcBitBand {
            bytes: slint::ModelRc::new(bytes),
        });
    }
    slint::ModelRc::new(bands_model)
}

fn next_message_id(info: &DbcInfo) -> u32 {
    info.messages.iter().map(|m| m.id).max().unwrap_or(255) + 1
}

fn apply_message_edits_values(
    info: &mut DbcInfo,
    idx: i32,
    id_text: &str,
    name: &str,
    dlc: &str,
    sender: &str,
) {
    if idx < 0 {
        return;
    }
    if let Some(msg) = info.messages.get_mut(idx as usize) {
        if let Some(id) = parse_hex_id(id_text) {
            msg.id = id;
            msg.is_extended = id > 0x7FF;
        }
        msg.name = name.to_string();
        msg.dlc = dlc.parse().unwrap_or(8);
        msg.sender = sender.to_string();
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_signal_edits_values(
    info: &mut DbcInfo,
    msg_idx: i32,
    sig_idx: i32,
    name: &str,
    start: &str,
    length: &str,
    factor: &str,
    offset: &str,
    unit: &str,
    byte_order_label: &str,
) {
    if msg_idx < 0 || sig_idx < 0 {
        return;
    }
    if let Some(msg) = info.messages.get_mut(msg_idx as usize) {
        if let Some(sig) = msg.signals.get_mut(sig_idx as usize) {
            sig.name = name.to_string();
            sig.start_bit = start.parse().unwrap_or(0);
            sig.length = length.parse().unwrap_or(8);
            sig.byte_order = endian_storage(byte_order_label).to_string();
            sig.factor = factor.parse().unwrap_or(1.0);
            sig.offset = offset.parse().unwrap_or(0.0);
            sig.unit = unit.to_string();
        }
    }
}

fn attribute_value_summary(value_type: &crate::dto::AttributeValueType) -> String {
    match value_type {
        crate::dto::AttributeValueType::Int { min, max } => format!("INT {min}..{max}"),
        crate::dto::AttributeValueType::Hex { min, max } => format!("HEX {min}..{max}"),
        crate::dto::AttributeValueType::Float { min, max } => format!("FLOAT {min}..{max}"),
        crate::dto::AttributeValueType::String => "STRING".to_string(),
        crate::dto::AttributeValueType::Enum { values } => format!("ENUM ({})", values.len()),
    }
}

fn apply_node_edits_values(info: &mut DbcInfo, idx: i32, name: &str, comment: &str) {
    if idx < 0 {
        return;
    }
    if let Some(node) = info.nodes.get_mut(idx as usize) {
        node.name = name.to_string();
        node.comment = if comment.is_empty() {
            None
        } else {
            Some(comment.to_string())
        };
    }
}

#[cfg(test)]
mod bit_map_tests {
    use super::signal_occupied_bits;

    #[test]
    fn motorola_rpm_style_16_from_bit_7() {
        let bits = signal_occupied_bits(7, 16, "big_endian");
        assert_eq!(
            bits,
            vec![7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]
        );
    }

    #[test]
    fn intel_oil_pressure_32_len_16() {
        let bits = signal_occupied_bits(32, 16, "little_endian");
        assert_eq!(bits, (32..48).collect::<Vec<_>>());
    }

    #[test]
    fn engine_data_signals_do_not_overlap() {
        // Matches sample.dbc EngineData packing
        let rpm = signal_occupied_bits(7, 16, "big_endian");
        let temp = signal_occupied_bits(23, 8, "big_endian");
        let throttle = signal_occupied_bits(31, 8, "big_endian");
        let oil = signal_occupied_bits(32, 16, "little_endian");
        let mut all = std::collections::HashSet::new();
        for set in [&rpm, &temp, &throttle, &oil] {
            for b in set {
                assert!(all.insert(*b), "overlap on bit {b}");
            }
        }
        assert_eq!(all.len(), 48);
    }
}
