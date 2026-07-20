//! Bit-layout helpers shared by the DBC editor UI and the analysis engines.
//!
//! Bit numbering follows the DBC convention: bit `b` of a payload is
//! `data[b / 8] >> (b % 8) & 1` (bit 0 = least significant bit of byte 0).

use serde::{Deserialize, Serialize};

/// Byte order of a raw field (DBC `@1` Intel / `@0` Motorola).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Endianness {
    /// Little endian (`@1`): start bit is the LSB, ascending bit numbers.
    Intel,
    /// Big endian (`@0`): start bit is the MSB, sawtooth walk toward byte 0's LSB.
    Motorola,
}

impl Endianness {
    pub fn label(&self) -> &'static str {
        match self {
            Endianness::Intel => "Intel",
            Endianness::Motorola => "Motorola",
        }
    }
}

/// A candidate or proven raw field location inside a payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldSpec {
    pub start_bit: u32,
    pub length: u32,
    pub endianness: Endianness,
    pub signed: bool,
}

/// Bits occupied by a signal (DBC Intel `@1` or Motorola `@0` layout).
///
/// Order: Motorola walks MSB→LSB; Intel ascends LSB→MSB. Moved from the
/// can-viewer DBC editor so UI and analysis share one bit walker.
pub fn signal_occupied_bits(start_bit: u32, length: u32, byte_order: &str) -> Vec<u32> {
    let endianness = if byte_order == "big_endian" || byte_order == "motorola" {
        Endianness::Motorola
    } else {
        Endianness::Intel
    };
    occupied_bits(start_bit, length, endianness)
}

/// Bits occupied by a field, in walk order (Motorola MSB→LSB, Intel LSB→MSB).
pub fn occupied_bits(start_bit: u32, length: u32, endianness: Endianness) -> Vec<u32> {
    if length == 0 {
        return Vec::new();
    }
    match endianness {
        Endianness::Motorola => {
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
        }
        Endianness::Intel => (start_bit..start_bit.saturating_add(length)).collect(),
    }
}

/// Occupied bits in MSB-first significance order (for value assembly).
pub fn occupied_bits_msb_first(spec: &FieldSpec) -> Vec<u32> {
    let mut bits = occupied_bits(spec.start_bit, spec.length, spec.endianness);
    if spec.endianness == Endianness::Intel {
        bits.reverse();
    }
    bits
}

/// Extract the raw integer value of a field from a payload.
///
/// Returns `None` if any occupied bit lies outside `data`.
pub fn extract_raw(data: &[u8], spec: &FieldSpec) -> Option<i64> {
    if spec.length == 0 || spec.length > 63 {
        return None;
    }
    let mut value: u64 = 0;
    for bit in occupied_bits_msb_first(spec) {
        let byte = (bit / 8) as usize;
        if byte >= data.len() {
            return None;
        }
        value = (value << 1) | u64::from((data[byte] >> (bit % 8)) & 1);
    }
    if spec.signed && value & (1u64 << (spec.length - 1)) != 0 {
        value |= u64::MAX << spec.length;
    }
    Some(value as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn extract_intel_16bit() {
        // Bytes 0-1 little endian: 0x34, 0x12 -> 0x1234
        let spec = FieldSpec {
            start_bit: 0,
            length: 16,
            endianness: Endianness::Intel,
            signed: false,
        };
        assert_eq!(extract_raw(&[0x34, 0x12], &spec), Some(0x1234));
    }

    #[test]
    fn extract_motorola_16bit() {
        // Motorola start bit 7 len 16: byte0 is high byte
        let spec = FieldSpec {
            start_bit: 7,
            length: 16,
            endianness: Endianness::Motorola,
            signed: false,
        };
        assert_eq!(extract_raw(&[0x12, 0x34], &spec), Some(0x1234));
    }

    #[test]
    fn extract_signed_sign_extends() {
        let spec = FieldSpec {
            start_bit: 0,
            length: 8,
            endianness: Endianness::Intel,
            signed: true,
        };
        assert_eq!(extract_raw(&[0xFF], &spec), Some(-1));
        assert_eq!(extract_raw(&[0x7F], &spec), Some(127));
    }

    #[test]
    fn extract_out_of_range_is_none() {
        let spec = FieldSpec {
            start_bit: 8,
            length: 8,
            endianness: Endianness::Intel,
            signed: false,
        };
        assert_eq!(extract_raw(&[0x00], &spec), None);
    }

    #[test]
    fn extract_intel_partial_nibble() {
        // Bits 4..8 of byte 0: value 0xA from 0xA5
        let spec = FieldSpec {
            start_bit: 4,
            length: 4,
            endianness: Endianness::Intel,
            signed: false,
        };
        assert_eq!(extract_raw(&[0xA5], &spec), Some(0xA));
    }
}
