use super::*;
use crate::dto::CanFrameDto;

fn make_frame(id: u32, timestamp: f64, data: Vec<u8>) -> CanFrameDto {
    CanFrameDto {
        timestamp,
        channel: "vcan0".to_string(),
        can_id: id,
        is_extended: false,
        is_fd: false,
        brs: false,
        esi: false,
        dlc: data.len() as u8,
        data,
    }
}

#[test]
fn test_parse_data_pattern() {
    let pattern = parse_data_pattern("01 ?? FF");
    assert_eq!(pattern.len(), 3);
    assert_eq!(pattern[0], Some(0x01));
    assert_eq!(pattern[1], None); // Wildcard
    assert_eq!(pattern[2], Some(0xFF));
}

#[test]
fn test_match_data_pattern() {
    let pattern = parse_data_pattern("01 ?? FF");
    assert!(match_data_pattern(&[0x01, 0x00, 0xFF], &pattern));
    assert!(match_data_pattern(&[0x01, 0xAB, 0xFF], &pattern));
    assert!(!match_data_pattern(&[0x01, 0xAB, 0xFE], &pattern));
    assert!(!match_data_pattern(&[0x02, 0xAB, 0xFF], &pattern));
}

#[test]
fn test_calculate_frame_stats_empty() {
    let stats = calculate_frame_stats(vec![]);
    assert_eq!(stats.unique_messages, 0);
    assert_eq!(stats.frame_rate, 0.0);
}

#[test]
fn test_calculate_frame_stats() {
    let frames = vec![
        make_frame(0x100, 0.0, vec![0, 1, 2, 3, 4, 5, 6, 7]),
        make_frame(0x100, 0.001, vec![0, 1, 2, 3, 4, 5, 6, 7]),
        make_frame(0x200, 0.002, vec![0, 1, 2, 3, 4, 5, 6, 7]),
    ];
    let stats = calculate_frame_stats(frames);
    assert_eq!(stats.unique_messages, 2);
    assert!(stats.frame_rate > 0.0);
}

#[test]
fn test_get_message_counts() {
    let frames = vec![
        make_frame(0x100, 0.0, vec![]),
        make_frame(0x100, 0.001, vec![]),
        make_frame(0x200, 0.002, vec![]),
    ];
    let counts = get_message_counts(frames);
    assert_eq!(counts.len(), 2);
    assert_eq!(counts[0].can_id, 0x100); // Higher count first
    assert_eq!(counts[0].count, 2);
    assert_eq!(counts[1].can_id, 0x200);
    assert_eq!(counts[1].count, 1);
}

#[test]
fn test_detect_dlc() {
    let frames = vec![
        make_frame(0x100, 0.0, vec![0; 8]),
        make_frame(0x100, 0.001, vec![0; 8]),
        make_frame(0x100, 0.002, vec![0; 4]),
        make_frame(0x200, 0.003, vec![0; 2]),
    ];
    let result = detect_dlc(frames, 0x100, false);
    assert_eq!(result.detected_dlc, 8);
    assert!(result.confidence > 0.6); // 2/3 = 0.666
    assert_eq!(result.sample_count, 3);
}
