//! DBC editor tab controller.

use parking_lot::Mutex;
use slint::Weak;
use std::sync::{Arc, Weak as StdWeak};

use crate::dbc_export::empty_dbc_info;
use crate::dto::{DbcInfo, MessageInfo, NodeInfo, SignalInfo};
use crate::services::{get_dbc_info, get_dbc_path, save_dbc_info};
use crate::state::AppState;
use crate::{
    DbcAttributeRow as UiDbcAttributeRow, DbcMessageRow as UiDbcMessageRow,
    DbcNodeRow as UiDbcNodeRow, DbcSignalRow as UiDbcSignalRow,
    DbcValueDescRow as UiDbcValueDescRow, SigmaDiagnostics,
};

use super::super::helpers::{pick_save_file, set_status, vec_model};
use super::super::mdf4::Mdf4Controller;
use super::bit_map::{bit_bands_model, build_bit_map};
use super::edit_snapshot::EditSnapshot;
use super::edits::{
    apply_message_edits_values, apply_node_edits_values, apply_signal_edits_values,
    attribute_value_summary, next_message_id,
};
use super::endian::{endian_label, endian_storage};
use super::rows::{message_row, node_row, selected_message_signals};
use super::save_target::SaveTarget;

/// DBC tab: load, inspect, edit, and save CAN dictionaries.
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

impl DbcController {
    /// Controller bound to the shared state and UI handle.
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

    /// Let the MDF4 tab re-decode when the active DBC changes.
    pub fn set_file_master(&self, mdf4: StdWeak<Mdf4Controller>) {
        *self.file_master.lock() = mdf4;
    }

    /// Hook the DBC tab callbacks.
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
