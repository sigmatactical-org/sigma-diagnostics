//! Validate a DBC hypothesis against a recorded log.
//!
//! Facts reported per message/signal: observed raw and physical ranges,
//! range-violation counts, signals whose bits never change (unconfirmable
//! from this log), toggling bits no signal covers, DLC mismatches, and the
//! ID coverage in both directions.
//!
//! Raw values are extracted with the same audited bit walker the analysis
//! engines use (`analysis::bits`), not the DBC decode fast path — for a
//! validation tool the extraction must be independently auditable, and
//! multiplexed signals need explicit selector handling.

use serde::{Deserialize, Serialize};

use super::bits::{extract_raw, occupied_bits, Endianness, FieldSpec};
use super::survey::{bit_activity, group_by_id, BitState};
use crate::dto::CanFrameDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCheck {
    pub signal_name: String,
    /// Frames in which the signal was present (mux selector matched) and
    /// extractable.
    pub frames_decoded: u64,
    /// Frames where the payload was too short to extract the signal.
    pub frames_too_short: u64,
    pub raw_min: i64,
    pub raw_max: i64,
    pub physical_min: f64,
    pub physical_max: f64,
    /// Frames whose physical value fell outside the DBC [min|max] range
    /// (only counted when the DBC defines a range, i.e. min < max).
    pub range_violations: u64,
    /// True when none of the signal's bits ever changed — the log cannot
    /// confirm this signal at all.
    pub never_toggles: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCheck {
    pub can_id: u32,
    pub is_extended: bool,
    pub message_name: String,
    pub frames: u64,
    pub expected_dlc: u8,
    /// Frames whose DLC differed from the DBC's.
    pub dlc_mismatch_frames: u64,
    /// Bits that toggle in the log but are covered by no signal — undecoded
    /// activity the DBC does not explain.
    pub uncovered_toggling_bits: Vec<u32>,
    pub signals: Vec<SignalCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbcCheckReport {
    /// Messages present in both DBC and log, with per-signal facts.
    pub covered: Vec<IdCheck>,
    /// IDs on the bus the DBC does not know: (id, extended, frames).
    pub unknown_ids: Vec<(u32, bool, u64)>,
    /// Messages in the DBC never seen on the bus: (id, name).
    pub unseen_messages: Vec<(u32, String)>,
}

/// Check every DBC message against the log.
pub fn check_dbc(frames: &[CanFrameDto], dbc: &dbc_rs::Dbc) -> DbcCheckReport {
    let groups = group_by_id(frames, None);

    let mut covered = Vec::new();
    let mut seen_message_ids = std::collections::BTreeSet::new();
    for msg in dbc.messages().iter() {
        let key = (msg.id(), msg.is_extended());
        let Some(group) = groups.get(&key) else {
            continue;
        };
        seen_message_ids.insert(key);

        let max_bits = group.iter().map(|f| f.data.len() * 8).max().unwrap_or(0) as u32;
        let activity = bit_activity(group, max_bits);
        let toggling: std::collections::BTreeSet<u32> = activity
            .iter()
            .filter(|c| c.state() == BitState::Toggles)
            .map(|c| c.bit)
            .collect();

        // Mux selector spec, if the message has one.
        let mux_switch: Option<FieldSpec> = msg
            .signals()
            .iter()
            .find(|s| s.is_multiplexer_switch())
            .map(field_spec);

        let mut all_signal_bits: std::collections::BTreeSet<u32> =
            std::collections::BTreeSet::new();
        let mut signal_checks = Vec::new();
        for sig in msg.signals().iter() {
            let spec = field_spec(sig);
            let sig_bits = occupied_bits(spec.start_bit, spec.length, spec.endianness);
            all_signal_bits.extend(sig_bits.iter().copied());

            let range_defined = sig.min() < sig.max();
            let mut check = SignalCheck {
                signal_name: sig.name().to_string(),
                frames_decoded: 0,
                frames_too_short: 0,
                raw_min: i64::MAX,
                raw_max: i64::MIN,
                physical_min: f64::INFINITY,
                physical_max: f64::NEG_INFINITY,
                range_violations: 0,
                never_toggles: sig_bits.iter().all(|b| !toggling.contains(b)),
            };
            for frame in group {
                // Multiplexed signal: only check frames whose selector matches.
                if let (Some(mux_value), Some(switch)) =
                    (sig.multiplexer_switch_value(), mux_switch.as_ref())
                {
                    match extract_raw(&frame.data, switch) {
                        Some(selector) if selector as u64 == mux_value => {}
                        _ => continue,
                    }
                }
                let Some(raw) = extract_raw(&frame.data, &spec) else {
                    check.frames_too_short += 1;
                    continue;
                };
                let physical = sig.factor() * raw as f64 + sig.offset();
                check.frames_decoded += 1;
                check.raw_min = check.raw_min.min(raw);
                check.raw_max = check.raw_max.max(raw);
                check.physical_min = check.physical_min.min(physical);
                check.physical_max = check.physical_max.max(physical);
                if range_defined {
                    let tolerance = 1e-9 * sig.max().abs().max(sig.min().abs()).max(1.0);
                    if physical < sig.min() - tolerance || physical > sig.max() + tolerance {
                        check.range_violations += 1;
                    }
                }
            }
            signal_checks.push(check);
        }

        covered.push(IdCheck {
            can_id: msg.id(),
            is_extended: msg.is_extended(),
            message_name: msg.name().to_string(),
            frames: group.len() as u64,
            expected_dlc: msg.dlc(),
            dlc_mismatch_frames: group
                .iter()
                .filter(|f| u64::from(f.dlc) != u64::from(msg.dlc()))
                .count() as u64,
            uncovered_toggling_bits: toggling
                .iter()
                .filter(|b| !all_signal_bits.contains(b))
                .copied()
                .collect(),
            signals: signal_checks,
        });
    }

    let unknown_ids = groups
        .iter()
        .filter(|(key, _)| !seen_message_ids.contains(key) && !in_dbc(dbc, **key))
        .map(|((id, ext), group)| (*id, *ext, group.len() as u64))
        .collect();

    let unseen_messages = dbc
        .messages()
        .iter()
        .filter(|m| !groups.contains_key(&(m.id(), m.is_extended())))
        .map(|m| (m.id(), m.name().to_string()))
        .collect();

    DbcCheckReport {
        covered,
        unknown_ids,
        unseen_messages,
    }
}

fn in_dbc(dbc: &dbc_rs::Dbc, key: (u32, bool)) -> bool {
    dbc.messages()
        .iter()
        .any(|m| m.id() == key.0 && m.is_extended() == key.1)
}

fn field_spec(sig: &dbc_rs::Signal) -> FieldSpec {
    FieldSpec {
        start_bit: sig.start_bit() as u32,
        length: sig.length() as u32,
        endianness: match sig.byte_order() {
            dbc_rs::ByteOrder::BigEndian => Endianness::Motorola,
            dbc_rs::ByteOrder::LittleEndian => Endianness::Intel,
        },
        signed: !sig.is_unsigned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DBC: &str = r#"VERSION "1"

BU_: ECU

BO_ 256 ENGINE : 3 ECU
 SG_ Rpm : 0|16@1+ (1,0) [0|8000] "rpm" ECU
 SG_ Flag : 16|1@1+ (1,0) [0|1] "" ECU
"#;

    fn frame(can_id: u32, timestamp: f64, data: &[u8]) -> CanFrameDto {
        CanFrameDto {
            timestamp,
            channel: "can0".to_string(),
            can_id,
            is_extended: false,
            is_fd: false,
            brs: false,
            esi: false,
            dlc: data.len() as u8,
            data: data.to_vec(),
        }
    }

    #[test]
    fn reports_range_violations_and_coverage() {
        let dbc = dbc_rs::Dbc::parse(TEST_DBC).expect("test dbc");
        let mut frames: Vec<CanFrameDto> = (0..100u32)
            .map(|i| {
                let rpm = (i * 100) as u16; // up to 9900 -> violates [0|8000]
                let mut data = rpm.to_le_bytes().to_vec();
                // Bit 17 toggles but no signal covers it.
                data.push(if i % 2 == 0 { 0b10 } else { 0b00 });
                frame(0x100, i as f64 * 0.01, &data)
            })
            .collect();
        frames.push(frame(0x400, 1.5, &[0xFF])); // unknown ID

        let report = check_dbc(&frames, &dbc);
        assert_eq!(report.covered.len(), 1);
        let engine = &report.covered[0];
        assert_eq!(engine.message_name, "ENGINE");
        assert_eq!(engine.frames, 100);
        assert_eq!(engine.uncovered_toggling_bits, vec![17]);

        let rpm = &engine.signals[0];
        assert_eq!(rpm.signal_name, "Rpm");
        assert_eq!(rpm.frames_decoded, 100);
        assert_eq!(rpm.raw_max, 9900);
        // 8100..=9900 step 100 -> 19 violating frames
        assert_eq!(rpm.range_violations, 19);

        let flag = &engine.signals[1];
        assert!(flag.never_toggles);

        assert_eq!(report.unknown_ids, vec![(0x400, false, 1)]);
        assert!(report.unseen_messages.is_empty());
    }
}
