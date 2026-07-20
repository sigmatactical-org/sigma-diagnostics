//! Rolling-counter proofs.
//!
//! A field is reported as a counter only when `raw[n+1] == (raw[n] + 1) mod 2^k`
//! holds for **every** consecutive frame pair of the ID and at least one wrap
//! was observed (so the modulus is actually exercised). A single violation —
//! including one caused by capture drops — kills the proof; that is deliberate.

use serde::{Deserialize, Serialize};

use super::bits::{extract_raw, occupied_bits, occupied_bits_msb_first, Endianness, FieldSpec};

/// A proven rolling counter. All numbers are evidence, not extrapolation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterProof {
    pub field: FieldSpec,
    /// Consecutive frame pairs checked (= frames - 1). Every one satisfied +1 mod 2^k.
    pub transitions: u64,
    /// Observed wrap-arounds (2^k - 1 → 0).
    pub wraps: u64,
}

/// Enumerate candidate fields and keep those proven over all payloads.
///
/// `payloads` must be in capture order for one CAN ID. `toggle_mask[b]` is true
/// iff bit `b` changed at least once in the log; candidates are restricted to
/// spans of toggling bits (constant bits cannot belong to a minimal counter —
/// a widened span over constant top bits fails at the first wrap anyway).
/// `bit_limit` bounds the payload area every frame is known to cover
/// (min DLC × 8). Only maximal proven spans are returned.
pub fn prove_counters(
    payloads: &[&[u8]],
    toggle_mask: &[bool],
    bit_limit: u32,
    max_width: u32,
    min_transitions: u64,
) -> Vec<CounterProof> {
    if payloads.len() < 2 {
        return Vec::new();
    }
    let mut proofs: Vec<CounterProof> = Vec::new();
    // Distinct (endianness, start, length) triples can denote the same field
    // (any field within one byte, e.g. Intel 8|4 ≡ Motorola 11|4) — dedupe by
    // the MSB-first bit sequence, which fully determines the extracted value.
    let mut seen: std::collections::HashSet<Vec<u32>> = std::collections::HashSet::new();
    for endianness in [Endianness::Intel, Endianness::Motorola] {
        for start_bit in 0..bit_limit {
            for length in 2..=max_width {
                let bits = occupied_bits(start_bit, length, endianness);
                if bits.iter().any(|&b| {
                    b >= bit_limit || !toggle_mask.get(b as usize).copied().unwrap_or(false)
                }) {
                    continue;
                }
                let spec = FieldSpec {
                    start_bit,
                    length,
                    endianness,
                    signed: false,
                };
                if !seen.insert(occupied_bits_msb_first(&spec)) {
                    continue;
                }
                if let Some(proof) = verify_counter(payloads, &spec, min_transitions) {
                    proofs.push(proof);
                }
            }
        }
    }
    retain_maximal(proofs)
}

fn verify_counter(
    payloads: &[&[u8]],
    spec: &FieldSpec,
    min_transitions: u64,
) -> Option<CounterProof> {
    let modulus = 1i64 << spec.length;
    let mut wraps = 0u64;
    let mut transitions = 0u64;
    let mut prev = extract_raw(payloads[0], spec)?;
    for payload in &payloads[1..] {
        let value = extract_raw(payload, spec)?;
        if value != (prev + 1) % modulus {
            return None;
        }
        if value < prev {
            wraps += 1;
        }
        transitions += 1;
        prev = value;
    }
    if wraps == 0 || transitions < min_transitions {
        // Without a wrap the modulus is never exercised and every wider span
        // over constant top bits would "prove" vacuously.
        return None;
    }
    Some(CounterProof {
        field: *spec,
        transitions,
        wraps,
    })
}

/// Drop proofs whose occupied bits are a subset of another proof's bits
/// (a proven k-bit counter implies its low sub-fields).
fn retain_maximal(proofs: Vec<CounterProof>) -> Vec<CounterProof> {
    let bit_sets: Vec<std::collections::HashSet<u32>> = proofs
        .iter()
        .map(|p| {
            occupied_bits(p.field.start_bit, p.field.length, p.field.endianness)
                .into_iter()
                .collect()
        })
        .collect();
    let mut keep = Vec::new();
    'outer: for (i, proof) in proofs.iter().enumerate() {
        for (j, other) in bit_sets.iter().enumerate() {
            if i != j && bit_sets[i].len() < other.len() && bit_sets[i].is_subset(other) {
                continue 'outer;
            }
        }
        keep.push(proof.clone());
    }
    keep
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counter_payloads(count: usize) -> Vec<Vec<u8>> {
        // 4-bit counter in the low nibble of byte 1, wrapping mod 16.
        (0..count)
            .map(|i| vec![0x55, (i % 16) as u8, 0xAA])
            .collect()
    }

    fn as_refs(payloads: &[Vec<u8>]) -> Vec<&[u8]> {
        payloads.iter().map(|p| p.as_slice()).collect()
    }

    fn toggle_mask_for(payloads: &[&[u8]]) -> Vec<bool> {
        let bits = payloads[0].len() * 8;
        (0..bits)
            .map(|b| {
                let first = payloads[0][b / 8] >> (b % 8) & 1;
                payloads.iter().any(|p| p[b / 8] >> (b % 8) & 1 != first)
            })
            .collect()
    }

    #[test]
    fn proves_4bit_counter() {
        let payloads = counter_payloads(100);
        let refs = as_refs(&payloads);
        let mask = toggle_mask_for(&refs);
        let proofs = prove_counters(&refs, &mask, 24, 16, 16);
        assert_eq!(
            proofs.len(),
            1,
            "expected exactly one maximal proof: {proofs:?}"
        );
        let p = &proofs[0];
        assert_eq!(p.field.start_bit, 8);
        assert_eq!(p.field.length, 4);
        assert_eq!(p.field.endianness, Endianness::Intel);
        assert_eq!(p.transitions, 99);
        assert_eq!(p.wraps, 99 / 16);
    }

    #[test]
    fn single_corrupt_frame_kills_proof() {
        let mut payloads = counter_payloads(100);
        payloads[50][1] = payloads[50][1].wrapping_add(2); // skip one count
        let refs = as_refs(&payloads);
        let mask = toggle_mask_for(&refs);
        assert!(prove_counters(&refs, &mask, 24, 16, 16).is_empty());
    }

    #[test]
    fn no_wrap_means_no_proof() {
        // 5,6,7: every candidate field increments by 1 but never wraps, so
        // the modulus is never exercised and nothing may be claimed.
        let payloads: Vec<Vec<u8>> = (5..8u8).map(|i| vec![i]).collect();
        let refs = as_refs(&payloads);
        let mask = toggle_mask_for(&refs);
        assert!(prove_counters(&refs, &mask, 8, 16, 2).is_empty());
    }

    #[test]
    fn full_byte_counter_is_maximal() {
        let payloads: Vec<Vec<u8>> = (0..600usize).map(|i| vec![(i % 256) as u8]).collect();
        let refs = as_refs(&payloads);
        let mask = toggle_mask_for(&refs);
        let proofs = prove_counters(&refs, &mask, 8, 16, 16);
        assert_eq!(proofs.len(), 1);
        assert_eq!(proofs[0].field.length, 8);
        assert_eq!(proofs[0].wraps, 2);
    }
}
