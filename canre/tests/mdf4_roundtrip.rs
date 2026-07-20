//! End-to-end: synthetic frames with a known counter + CRC-8 are exported to
//! a real MDF4 file, reloaded, and the survey must prove exactly those facts.

use sigma_diagnostics::analysis::{
    correlate, survey_frames, ChecksumAlgorithm, CorrelationOptions, Endianness, ReferenceSeries,
    SurveyOptions,
};
use sigma_diagnostics::obd::{extract_obd_samples, group_series};
use sigma_diagnostics::{export_logs, load_mdf4, CanFrameDto, DiagnosticsState};

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

fn j1850_crc(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xFF;
    for &b in data {
        crc ^= b;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ 0x1D } else { crc << 1 };
        }
    }
    crc ^ 0xFF
}

#[test]
fn survey_proves_counter_and_crc_through_mdf4() {
    let frames: Vec<CanFrameDto> = (0..500u32)
        .map(|i| {
            let mut data = vec![
                (i % 16) as u8,          // 4-bit counter, low nibble byte 0
                (i * 7 % 251) as u8,     // varying data
                (i * 13 % 253) as u8,    // varying data
                0x5A,                    // constant
            ];
            data.push(j1850_crc(&data));
            CanFrameDto {
                timestamp: f64::from(i) * 0.020,
                channel: "can0".to_string(),
                can_id: 0x2A0,
                is_extended: false,
                is_fd: false,
                brs: false,
                esi: false,
                dlc: data.len() as u8,
                data,
            }
        })
        .collect();

    let path = std::env::temp_dir().join("canre_roundtrip_test.mf4");
    let path_str = path.to_str().expect("utf8 temp path");
    export_logs(path_str, &frames).expect("export mdf4");

    let state = DiagnosticsState::new();
    let (reloaded, _) = load_mdf4(path_str, &state).expect("reload mdf4");
    let _ = std::fs::remove_file(&path);
    assert_eq!(reloaded.len(), frames.len());

    let report = survey_frames(&reloaded, &SurveyOptions::default());
    assert_eq!(report.unique_ids, 1);
    let id = &report.ids[0];
    assert_eq!(id.can_id, 0x2A0);
    assert_eq!(id.frames, 500);

    // Exactly one maximal counter: the low nibble of byte 0.
    assert_eq!(id.counters.len(), 1, "counters: {:?}", id.counters);
    let counter = &id.counters[0];
    assert_eq!(counter.field.start_bit, 0);
    assert_eq!(counter.field.length, 4);
    assert_eq!(counter.field.endianness, Endianness::Intel);
    assert_eq!(counter.transitions, 499);

    // The J1850 CRC in the last byte is found, verified over all 500 frames.
    let crc = id
        .checksums
        .iter()
        .find(|c| c.byte_index == 4)
        .expect("CRC proof on byte 4");
    assert_eq!(
        crc.algorithm,
        ChecksumAlgorithm::Crc8 {
            poly: 0x1D,
            init: 0xFF,
            xorout: 0xFF,
            reflect: false,
        }
    );
    assert_eq!(crc.frames, 500);

    // Timing is measured, not invented: 20 ms period.
    let timing = id.timing.as_ref().expect("timing stats");
    assert!((timing.median_ms - 20.0).abs() < 0.5, "median {}", timing.median_ms);
}

#[test]
fn correlate_recovers_rpm_scaling_from_obd_session_through_mdf4() {
    // Broadcast: CAN 0x156 carries raw = RPM×4 in a Motorola 16-bit field
    // at 50 Hz. Diagnostic: 0x7E8 answers PID 0x0C with the same physical RPM
    // at 10 Hz. correlate must recover value = raw/4 without being told.
    let rpm_at = |t: f64| -> f64 { 1500.0 + 4000.0 * (t / 40.0) };
    let mut frames = Vec::new();
    for i in 0..2000u32 {
        let t = f64::from(i) * 0.020;
        let raw = (rpm_at(t) * 4.0) as u16;
        frames.push(frame(
            0x156,
            t,
            &[(raw >> 8) as u8, raw as u8, 0x00, (i % 16) as u8],
        ));
    }
    for i in 0..400u32 {
        let t = f64::from(i) * 0.100 + 0.003;
        let reported = ((rpm_at(t) * 4.0) as u16) as f64 / 4.0; // ECU quantization
        let raw = (reported * 4.0) as u16;
        // Single-frame OBD response: 04 41 0C AB CD ...
        frames.push(frame(
            0x7E8,
            t,
            &[0x04, 0x41, 0x0C, (raw >> 8) as u8, raw as u8, 0, 0, 0],
        ));
    }
    frames.sort_by(|a, b| a.timestamp.total_cmp(&b.timestamp));

    let path = std::env::temp_dir().join("canre_correlate_test.mf4");
    let path_str = path.to_str().unwrap();
    export_logs(path_str, &frames).expect("export");
    let state = DiagnosticsState::new();
    let (reloaded, _) = load_mdf4(path_str, &state).expect("reload");
    let _ = std::fs::remove_file(&path);

    // Extract ground truth from the recorded diag frames, then correlate.
    let samples = extract_obd_samples(&reloaded, None);
    assert!(!samples.is_empty(), "no OBD samples extracted");
    let references: Vec<ReferenceSeries> = group_series(&samples)
        .into_iter()
        .map(|s| ReferenceSeries {
            name: s.name,
            unit: s.unit,
            resolution: s.resolution,
            samples: s.samples,
        })
        .collect();
    let report = correlate(&reloaded, &references, &CorrelationOptions::default());

    let rpm = report
        .series
        .iter()
        .find(|s| s.name == "EngineRpm")
        .expect("EngineRpm series");
    assert!(rpm.note.is_none(), "note: {:?}", rpm.note);
    let best = rpm.matches.first().expect("a match on 0x156");
    assert_eq!(best.can_id, 0x156);
    assert_eq!(best.factor_rational, Some((1, 4)));
    // The 0x7E8 diag ID must never be reported as its own signal source.
    assert!(rpm.matches.iter().all(|m| m.can_id != 0x7E8));
}
