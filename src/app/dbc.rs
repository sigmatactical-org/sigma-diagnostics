//! DBC editor tab controller.

use super::helpers::{parse_hex_id, pick_save_file, set_status, vec_model};
use super::mdf4::Mdf4Controller;
use crate::dbc_export::empty_dbc_info;
use crate::dto::{DbcInfo, MessageInfo, NodeInfo, SignalInfo};
use crate::services::{get_dbc_info, get_dbc_path, save_dbc_info};
use crate::state::AppState;
use crate::{
    DbcAttributeRow as UiDbcAttributeRow, DbcMessageRow as UiDbcMessageRow,
    DbcNodeRow as UiDbcNodeRow, DbcSignalRow as UiDbcSignalRow,
    DbcValueDescRow as UiDbcValueDescRow, SigmaDiagnostics,
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
    selected_message: Mutex<i32>,
    selected_signal: Mutex<i32>,
    selected_node: Mutex<i32>,
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
        ui.on_save_dbc({
            let t = this.clone();
            move || t.save_dbc()
        });
        ui.on_save_dbc_as({
            let t = this.clone();
            move || t.save_dbc_as()
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
    }

    fn apply_edits_from_ui(&self) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
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

        let node_name = ui.get_dbc_edit_node_name().to_string();
        let node_comment = ui.get_dbc_edit_node_comment().to_string();

        {
            let mut info = self.edit_state.lock();
            info.version = Some(version);
            info.comment = Some(comment);

            if panel == 0 {
                apply_message_edits_values(&mut info, msg_idx, &msg_id, &msg_name, &msg_dlc, &msg_sender);
            } else if panel == 1 {
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
                );
            } else if panel == 2 {
                apply_node_edits_values(&mut info, node_idx, &node_name, &node_comment);
            }
        }
        self.mark_dirty();
    }

    fn new_dbc(&self) {
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

    fn save_dbc(&self) {
        if let Some(path) = self.path.lock().clone() {
            self.save_to_path(&path);
        } else {
            self.save_dbc_as();
        }
    }

    fn save_dbc_as(&self) {
        if let Some(path) = pick_save_file("Save DBC", &[("DBC", &["dbc"])]) {
            self.save_to_path(&path);
        }
    }

    fn save_to_path(&self, path: &str) {
        self.apply_edits_from_ui();
        let info = self.edit_state.lock().clone();
        match save_dbc_info(path, &info, &self.state) {
            Ok(()) => {
                *self.path.lock() = Some(path.to_string());
                *self.dirty.lock() = false;
                self.refresh_ui();
                self.sync_master_header();
                self.with_ui(|ui| set_status(ui, &format!("Saved {path}")));
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
                }
            }
        });
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
                    factor: s.factor.to_string().into(),
                    unit: s.unit.clone().into(),
                })
                .collect()
        })
        .unwrap_or_default()
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
) {
    if msg_idx < 0 || sig_idx < 0 {
        return;
    }
    if let Some(msg) = info.messages.get_mut(msg_idx as usize) {
        if let Some(sig) = msg.signals.get_mut(sig_idx as usize) {
            sig.name = name.to_string();
            sig.start_bit = start.parse().unwrap_or(0);
            sig.length = length.parse().unwrap_or(8);
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
