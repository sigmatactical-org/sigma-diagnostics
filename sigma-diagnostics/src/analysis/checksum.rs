//! Checksum / CRC verification sweep.
//!
//! Nothing is inferred: every candidate algorithm is verified against every
//! distinct payload of the ID, and **all** algorithms that match are reported
//! (ambiguity stays visible). A constant payload set proves nothing, so the
//! caller gates on a minimum distinct-payload count.

use serde::{Deserialize, Serialize};

/// A checksum algorithm verified against every observed payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChecksumAlgorithm {
    /// `cs = XOR(all other bytes) ^ constant`
    XorConst { constant: u8 },
    /// `cs = (sum of other bytes + constant) mod 256`
    SumConst { constant: u8 },
    /// `cs = (constant - sum of other bytes) mod 256`
    NegSumConst { constant: u8 },
    /// CRC-8 over the other bytes in order. `reflect` = refin+refout.
    Crc8 {
        poly: u8,
        init: u8,
        xorout: u8,
        reflect: bool,
    },
}

impl ChecksumAlgorithm {
    pub fn describe(&self) -> String {
        match self {
            ChecksumAlgorithm::XorConst { constant } => format!("XOR ^ 0x{constant:02X}"),
            ChecksumAlgorithm::SumConst { constant } => format!("SUM + 0x{constant:02X} (mod 256)"),
            ChecksumAlgorithm::NegSumConst { constant } => {
                format!("0x{constant:02X} - SUM (mod 256)")
            }
            ChecksumAlgorithm::Crc8 {
                poly,
                init,
                xorout,
                reflect,
            } => format!(
                "CRC-8 poly=0x{poly:02X} init=0x{init:02X} xorout=0x{xorout:02X}{}",
                if *reflect { " reflected" } else { "" }
            ),
        }
    }
}

/// A checksum algorithm that matched **every** observed payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumProof {
    /// Byte index holding the checksum; data = the remaining bytes in order.
    pub byte_index: u8,
    pub algorithm: ChecksumAlgorithm,
    /// Total frames observed for the ID.
    pub frames: u64,
    /// Distinct payloads among them — the real evidence strength.
    pub distinct_payloads: u64,
}

/// Verify checksum algorithms for one ID's payloads (must all share one DLC).
///
/// Returns all `(byte position, algorithm)` pairs that hold for every distinct
/// payload, or an empty list when payload diversity is below `min_distinct`
/// (a sweep over near-constant data would "prove" almost anything).
pub fn find_checksums(payloads: &[&[u8]], min_distinct: usize) -> Vec<ChecksumProof> {
    let Some(first) = payloads.first() else {
        return Vec::new();
    };
    let dlc = first.len();
    if dlc < 2 || payloads.iter().any(|p| p.len() != dlc) {
        return Vec::new();
    }
    let distinct: Vec<&[u8]> = {
        let mut seen = std::collections::HashSet::new();
        payloads
            .iter()
            .filter(|p| seen.insert(**p))
            .copied()
            .collect()
    };
    if distinct.len() < min_distinct {
        return Vec::new();
    }

    let mut proofs = Vec::new();
    let mut data_buf = vec![0u8; dlc - 1];
    for byte_index in 0..dlc {
        // Simple families: solve the constant from the first payload, verify the rest.
        let cs0 = split_at(distinct[0], byte_index, &mut data_buf);
        let xor0 = data_buf.iter().fold(0u8, |a, &b| a ^ b);
        let sum0 = data_buf.iter().fold(0u8, |a, &b| a.wrapping_add(b));
        let candidates = [
            ChecksumAlgorithm::XorConst {
                constant: cs0 ^ xor0,
            },
            ChecksumAlgorithm::SumConst {
                constant: cs0.wrapping_sub(sum0),
            },
            ChecksumAlgorithm::NegSumConst {
                constant: cs0.wrapping_add(sum0),
            },
        ];
        for algorithm in candidates {
            let ok = distinct.iter().all(|p| {
                let cs = split_at(p, byte_index, &mut data_buf);
                verify_simple(&data_buf, cs, &algorithm)
            });
            if ok {
                proofs.push(ChecksumProof {
                    byte_index: byte_index as u8,
                    algorithm,
                    frames: payloads.len() as u64,
                    distinct_payloads: distinct.len() as u64,
                });
            }
        }

        // CRC-8 sweep: 255 polys × init {00,FF} × xorout {00,FF} × reflect {n,y}.
        // Stage 1 filters against a small payload sample; survivors get full proof.
        let sample_len = distinct.len().min(32);
        for poly in 1..=255u8 {
            for init in [0x00u8, 0xFF] {
                for xorout in [0x00u8, 0xFF] {
                    for reflect in [false, true] {
                        let mut matches = |set: &[&[u8]]| {
                            set.iter().all(|p| {
                                let cs = split_at(p, byte_index, &mut data_buf);
                                crc8(&data_buf, poly, init, xorout, reflect) == cs
                            })
                        };
                        if matches(&distinct[..sample_len]) && matches(&distinct[sample_len..]) {
                            proofs.push(ChecksumProof {
                                byte_index: byte_index as u8,
                                algorithm: ChecksumAlgorithm::Crc8 {
                                    poly,
                                    init,
                                    xorout,
                                    reflect,
                                },
                                frames: payloads.len() as u64,
                                distinct_payloads: distinct.len() as u64,
                            });
                        }
                    }
                }
            }
        }
    }
    proofs
}

/// Copy `payload` minus the byte at `byte_index` into `buf`; return that byte.
fn split_at(payload: &[u8], byte_index: usize, buf: &mut [u8]) -> u8 {
    let mut k = 0;
    for (i, &b) in payload.iter().enumerate() {
        if i != byte_index {
            buf[k] = b;
            k += 1;
        }
    }
    payload[byte_index]
}

fn verify_simple(data: &[u8], cs: u8, algorithm: &ChecksumAlgorithm) -> bool {
    match algorithm {
        ChecksumAlgorithm::XorConst { constant } => {
            data.iter().fold(*constant, |a, &b| a ^ b) == cs
        }
        ChecksumAlgorithm::SumConst { constant } => {
            data.iter().fold(*constant, |a: u8, &b| a.wrapping_add(b)) == cs
        }
        ChecksumAlgorithm::NegSumConst { constant } => {
            constant.wrapping_sub(data.iter().fold(0u8, |a, &b| a.wrapping_add(b))) == cs
        }
        ChecksumAlgorithm::Crc8 { .. } => unreachable!("CRC handled in sweep"),
    }
}

/// Bitwise CRC-8. `reflect` reflects input bytes and the final CRC (refin =
/// refout), which is the standard equivalent of the reflected algorithm form;
/// the swept init values 0x00/0xFF are palindromic so no separate reflected
/// init is needed.
fn crc8(data: &[u8], poly: u8, init: u8, xorout: u8, reflect: bool) -> u8 {
    let mut crc = init;
    for &b in data {
        crc ^= if reflect { b.reverse_bits() } else { b };
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 {
                (crc << 1) ^ poly
            } else {
                crc << 1
            };
        }
    }
    if reflect {
        crc = crc.reverse_bits();
    }
    crc ^ xorout
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SAE J1850 CRC-8: poly 0x1D, init 0xFF, xorout 0xFF.
    fn j1850(data: &[u8]) -> u8 {
        crc8(data, 0x1D, 0xFF, 0xFF, false)
    }

    fn payloads_with(cs: impl Fn(&[u8]) -> u8, last_is_cs: bool) -> Vec<Vec<u8>> {
        (0u32..200)
            .map(|i| {
                let data = [
                    (i % 251) as u8,
                    (i * 7 % 253) as u8,
                    (i * 13 % 256) as u8,
                    (i / 3) as u8,
                ];
                let c = cs(&data);
                let mut p = data.to_vec();
                if last_is_cs {
                    p.push(c);
                } else {
                    p.insert(0, c);
                }
                p
            })
            .collect()
    }

    fn as_refs(p: &[Vec<u8>]) -> Vec<&[u8]> {
        p.iter().map(|v| v.as_slice()).collect()
    }

    #[test]
    fn crc8_known_vector() {
        // CRC-8/SAE-J1850 check value: "123456789" -> 0x4B
        assert_eq!(j1850(b"123456789"), 0x4B);
        // CRC-8/MAXIM (reflected, poly 0x31, init 0x00): "123456789" -> 0xA1
        assert_eq!(crc8(b"123456789", 0x31, 0x00, 0x00, true), 0xA1);
    }

    #[test]
    fn finds_j1850_crc_in_last_byte() {
        let payloads = payloads_with(j1850, true);
        let refs = as_refs(&payloads);
        let proofs = find_checksums(&refs, 8);
        let hit = proofs.iter().find(|p| {
            p.byte_index == 4
                && p.algorithm
                    == ChecksumAlgorithm::Crc8 {
                        poly: 0x1D,
                        init: 0xFF,
                        xorout: 0xFF,
                        reflect: false,
                    }
        });
        assert!(hit.is_some(), "proofs: {proofs:?}");
        assert_eq!(hit.unwrap().frames, 200);
    }

    #[test]
    fn finds_xor_checksum_in_first_byte() {
        let payloads = payloads_with(|d| d.iter().fold(0u8, |a, &b| a ^ b), false);
        let refs = as_refs(&payloads);
        let proofs = find_checksums(&refs, 8);
        assert!(proofs.iter().any(
            |p| p.byte_index == 0 && p.algorithm == ChecksumAlgorithm::XorConst { constant: 0 }
        ));
    }

    #[test]
    fn corrupt_frame_kills_all_proofs_at_that_position() {
        let mut payloads = payloads_with(j1850, true);
        payloads[100][4] ^= 0x01;
        let refs = as_refs(&payloads);
        let proofs = find_checksums(&refs, 8);
        assert!(
            proofs.iter().all(|p| p.byte_index != 4),
            "proofs: {proofs:?}"
        );
    }

    #[test]
    fn low_diversity_reports_nothing() {
        let payloads = vec![vec![0x01, 0x02, 0x03]; 50];
        let refs = as_refs(&payloads);
        assert!(find_checksums(&refs, 8).is_empty());
    }
}
