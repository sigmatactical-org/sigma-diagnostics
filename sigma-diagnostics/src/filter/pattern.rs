//! Data-pattern parsing and matching (e.g. `"01 ?? FF"` with wildcards).

/// Parse data pattern into bytes and wildcards.
/// Pattern format: "01 ?? FF" where ?? is wildcard.
pub fn parse_data_pattern(pattern: &str) -> Vec<Option<u8>> {
    pattern
        .split_whitespace()
        .map(|s| {
            let s = s.to_uppercase();
            if s == "??" || s == "XX" {
                None // Wildcard
            } else {
                u8::from_str_radix(&s, 16).ok()
            }
        })
        .collect()
}

/// Check if frame data matches pattern.
pub fn match_data_pattern(data: &[u8], pattern: &[Option<u8>]) -> bool {
    if pattern.len() > data.len() {
        return false;
    }

    for (i, expected) in pattern.iter().enumerate() {
        if let Some(expected_byte) = expected {
            if data[i] != *expected_byte {
                return false;
            }
        }
        // None = wildcard, always matches
    }
    true
}
