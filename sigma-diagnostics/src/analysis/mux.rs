//! Multiplexor facts.
//!
//! Reports the factual observation only: "conditioned on this byte/nibble,
//! the sets of toggling bits differ between partitions". Whether that makes
//! the field a mux selector semantically is left to the reader — but a
//! candidate is only flagged when every counted partition has enough frames
//! and at least two partitions disagree on layout.

use serde::{Deserialize, Serialize};

use super::bits::{extract_raw, Endianness, FieldSpec};

/// Conditional bit activity for one selector value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuxVariant {
    pub selector_value: u64,
    pub frames: u64,
    /// Bits that toggle *within* frames carrying this selector value.
    pub toggling_bits: Vec<u32>,
}

/// Partition analysis for one candidate selector field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuxFacts {
    pub selector: FieldSpec,
    pub distinct_values: u64,
    /// Only variants with at least `min_frames_per_variant` frames are listed.
    pub variants: Vec<MuxVariant>,
    /// True iff at least two listed variants have different toggling-bit sets.
    pub variant_layouts_differ: bool,
}

/// Analyze byte- and nibble-aligned selector candidates for one ID.
///
/// Only candidates where `variant_layouts_differ` is true are returned —
/// partitioning on an ordinary changing field yields identical conditional
/// layouts and proves nothing.
pub fn mux_facts(
    payloads: &[&[u8]],
    toggle_mask: &[bool],
    bit_limit: u32,
    min_frames_per_variant: u64,
    max_selector_values: usize,
) -> Vec<MuxFacts> {
    let mut out = Vec::new();
    let mut candidates = Vec::new();
    for start in (0..bit_limit).step_by(8) {
        candidates.push(FieldSpec {
            start_bit: start,
            length: 8,
            endianness: Endianness::Intel,
            signed: false,
        });
    }
    for start in (0..bit_limit).step_by(4) {
        candidates.push(FieldSpec {
            start_bit: start,
            length: 4,
            endianness: Endianness::Intel,
            signed: false,
        });
    }

    for selector in candidates {
        let sel_bits: Vec<u32> =
            (selector.start_bit..selector.start_bit + selector.length).collect();
        if sel_bits.iter().any(|&b| b >= bit_limit) {
            continue;
        }
        // Selector must actually change, else there is nothing to partition.
        if !sel_bits
            .iter()
            .any(|&b| toggle_mask.get(b as usize).copied().unwrap_or(false))
        {
            continue;
        }

        // Partition frames by selector value.
        let mut partitions: std::collections::BTreeMap<u64, Vec<&[u8]>> =
            std::collections::BTreeMap::new();
        let mut overflow = false;
        for payload in payloads {
            let Some(value) = extract_raw(payload, &selector) else {
                continue;
            };
            let entry = partitions.entry(value as u64).or_default();
            entry.push(payload);
            if partitions.len() > max_selector_values {
                overflow = true;
                break;
            }
        }
        if overflow || partitions.len() < 2 {
            continue;
        }

        let variants: Vec<MuxVariant> = partitions
            .iter()
            .filter(|(_, frames)| frames.len() as u64 >= min_frames_per_variant)
            .map(|(&selector_value, frames)| MuxVariant {
                selector_value,
                frames: frames.len() as u64,
                toggling_bits: toggling_bits_excluding(frames, bit_limit, &sel_bits),
            })
            .collect();
        if variants.len() < 2 {
            continue;
        }
        let variant_layouts_differ = variants
            .windows(2)
            .any(|w| w[0].toggling_bits != w[1].toggling_bits);
        if variant_layouts_differ {
            out.push(MuxFacts {
                selector,
                distinct_values: partitions.len() as u64,
                variants,
                variant_layouts_differ,
            });
        }
    }
    out
}

fn toggling_bits_excluding(frames: &[&[u8]], bit_limit: u32, exclude: &[u32]) -> Vec<u32> {
    let first = frames[0];
    (0..bit_limit)
        .filter(|&b| {
            if exclude.contains(&b) {
                return false;
            }
            let byte = (b / 8) as usize;
            if byte >= first.len() {
                return false;
            }
            let reference = first[byte] >> (b % 8) & 1;
            frames
                .iter()
                .any(|p| byte < p.len() && p[byte] >> (b % 8) & 1 != reference)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toggle_mask_for(payloads: &[&[u8]], bit_limit: u32) -> Vec<bool> {
        (0..bit_limit as usize)
            .map(|b| {
                let first = payloads[0][b / 8] >> (b % 8) & 1;
                payloads.iter().any(|p| p[b / 8] >> (b % 8) & 1 != first)
            })
            .collect()
    }

    #[test]
    fn detects_byte0_mux() {
        // Selector byte 0 ∈ {1, 2}: page 1 toggles byte 1, page 2 toggles byte 2.
        let mut payloads = Vec::new();
        for i in 0u8..100 {
            payloads.push(vec![1, i, 0x00, 0xFF]);
            payloads.push(vec![2, 0x00, i, 0xFF]);
        }
        let refs: Vec<&[u8]> = payloads.iter().map(|p| p.as_slice()).collect();
        let mask = toggle_mask_for(&refs, 32);
        let facts = mux_facts(&refs, &mask, 32, 20, 32);
        let byte0 = facts
            .iter()
            .find(|f| f.selector.start_bit == 0 && f.selector.length == 8)
            .expect("byte 0 selector facts");
        assert!(byte0.variant_layouts_differ);
        assert_eq!(byte0.variants.len(), 2);
        // i only reaches 99, so the MSB of the counting byte never sets.
        assert_eq!(byte0.variants[0].toggling_bits, (8..15).collect::<Vec<_>>());
        assert_eq!(
            byte0.variants[1].toggling_bits,
            (16..23).collect::<Vec<_>>()
        );
    }

    #[test]
    fn plain_counter_byte_is_not_a_mux() {
        // Byte 0 counts 0..4; the rest of the payload toggles identically
        // regardless of the "selector" value.
        let mut payloads = Vec::new();
        for i in 0u32..400 {
            payloads.push(vec![(i % 4) as u8, (i % 2) as u8, 0x55]);
        }
        let refs: Vec<&[u8]> = payloads.iter().map(|p| p.as_slice()).collect();
        let mask = toggle_mask_for(&refs, 24);
        let facts = mux_facts(&refs, &mask, 24, 20, 32);
        assert!(
            facts
                .iter()
                .all(|f| f.selector.start_bit != 0 || f.selector.length != 8),
            "facts: {facts:?}"
        );
    }
}
