//! Conversions between DBC byte-order storage strings and UI labels.

/// UI label for a stored byte-order value.
pub(super) fn endian_label(byte_order: &str) -> &'static str {
    if byte_order == "big_endian" || byte_order == "motorola" || byte_order == "Motorola" {
        "Motorola"
    } else {
        "Intel"
    }
}

/// Stored byte-order value for a UI label.
pub(super) fn endian_storage(label: &str) -> &'static str {
    if label == "Motorola" || label == "big_endian" || label == "motorola" {
        "big_endian"
    } else {
        "little_endian"
    }
}
