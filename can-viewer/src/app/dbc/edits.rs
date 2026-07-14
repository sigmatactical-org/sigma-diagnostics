//! Apply edited UI field values back onto the in-memory [`DbcInfo`].

use crate::dto::{AttributeValueType, DbcInfo};

use super::super::helpers::parse_hex_id;
use super::endian::endian_storage;

/// First unused CAN id for a newly created message.
pub(super) fn next_message_id(info: &DbcInfo) -> u32 {
    info.messages.iter().map(|m| m.id).max().unwrap_or(255) + 1
}

/// Apply edited message fields back into the DBC model.
pub(super) fn apply_message_edits_values(
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
pub(super) fn apply_signal_edits_values(
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

/// Short human-readable form of an attribute's type/range.
pub(super) fn attribute_value_summary(value_type: &AttributeValueType) -> String {
    match value_type {
        AttributeValueType::Int { min, max } => format!("INT {min}..{max}"),
        AttributeValueType::Hex { min, max } => format!("HEX {min}..{max}"),
        AttributeValueType::Float { min, max } => format!("FLOAT {min}..{max}"),
        AttributeValueType::String => "STRING".to_string(),
        AttributeValueType::Enum { values } => format!("ENUM ({})", values.len()),
    }
}

/// Apply edited node fields back into the DBC model.
pub(super) fn apply_node_edits_values(info: &mut DbcInfo, idx: i32, name: &str, comment: &str) {
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
