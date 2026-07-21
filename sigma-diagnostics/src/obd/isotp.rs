//! Minimal ISO-TP (ISO 15765-2), 11-bit normal addressing.
//!
//! Covers what OBD-II diagnostics need: single frames, first/consecutive
//! frame reassembly, and flow-control frames for the live side. Offline
//! reassembly from a recorded log needs no flow control at all.

/// OBD-II functional request ID (all ECUs).
pub const OBD_FUNCTIONAL_ID: u32 = 0x7DF;
/// Physical request ID of the primary ECU.
pub const OBD_ECU_REQUEST_ID: u32 = 0x7E0;
/// Response ID of the primary ECU.
pub const OBD_ECU_RESPONSE_ID: u32 = 0x7E8;

/// A fully reassembled ISO-TP message.
#[derive(Debug, Clone, PartialEq)]
pub struct IsotpMessage {
    /// Timestamp of the frame that completed the message.
    pub timestamp: f64,
    pub data: Vec<u8>,
}

/// Streaming reassembler for one ID's frames, fed in capture order.
#[derive(Debug, Default)]
pub struct IsotpReassembler {
    pending: Option<Pending>,
}

#[derive(Debug)]
struct Pending {
    total_len: usize,
    data: Vec<u8>,
    next_sequence: u8,
}

impl IsotpReassembler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed one CAN frame payload; returns a message when one completes.
    ///
    /// Malformed or out-of-sequence frames reset the pending transfer and are
    /// otherwise ignored — a reverse-engineering tool must not invent data.
    pub fn push(&mut self, timestamp: f64, frame_data: &[u8]) -> Option<IsotpMessage> {
        let &pci = frame_data.first()?;
        match pci >> 4 {
            0x0 => {
                // Single frame: low nibble = length 1..=7.
                self.pending = None;
                let len = (pci & 0x0F) as usize;
                if len == 0 || frame_data.len() < len + 1 {
                    return None;
                }
                Some(IsotpMessage {
                    timestamp,
                    data: frame_data[1..1 + len].to_vec(),
                })
            }
            0x1 => {
                // First frame: 12-bit length, first 6 data bytes follow.
                let len = ((pci as usize & 0x0F) << 8) | *frame_data.get(1)? as usize;
                if len <= 7 || frame_data.len() < 8 {
                    self.pending = None;
                    return None;
                }
                self.pending = Some(Pending {
                    total_len: len,
                    data: frame_data[2..8].to_vec(),
                    next_sequence: 1,
                });
                None
            }
            0x2 => {
                // Consecutive frame: low nibble = sequence number mod 16.
                let pending = self.pending.as_mut()?;
                if pci & 0x0F != pending.next_sequence {
                    self.pending = None;
                    return None;
                }
                pending.next_sequence = (pending.next_sequence + 1) % 16;
                let remaining = pending.total_len - pending.data.len();
                let take = remaining.min(frame_data.len() - 1);
                pending.data.extend_from_slice(&frame_data[1..1 + take]);
                if pending.data.len() >= pending.total_len {
                    let done = self.pending.take()?;
                    return Some(IsotpMessage {
                        timestamp,
                        data: done.data,
                    });
                }
                None
            }
            // Flow control (0x3) and anything else: not message data.
            _ => None,
        }
    }
}

/// Encode a payload as a padded single frame (fits 7 bytes or fewer).
pub fn encode_single_frame(payload: &[u8]) -> Option<[u8; 8]> {
    if payload.is_empty() || payload.len() > 7 {
        return None;
    }
    let mut frame = [0x00u8; 8];
    frame[0] = payload.len() as u8;
    frame[1..1 + payload.len()].copy_from_slice(payload);
    Some(frame)
}

/// Flow control "continue to send", block size 0, no separation time.
pub fn flow_control_cts() -> [u8; 8] {
    [0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_frame_roundtrip() {
        let frame = encode_single_frame(&[0x01, 0x0C]).unwrap();
        assert_eq!(frame[0], 0x02);
        let mut r = IsotpReassembler::new();
        let msg = r.push(1.0, &frame).unwrap();
        assert_eq!(msg.data, vec![0x01, 0x0C]);
        assert_eq!(msg.timestamp, 1.0);
    }

    #[test]
    fn multi_frame_reassembly_with_padding() {
        // 12-byte message: FF carries 6, CF carries the rest (+ padding).
        let mut r = IsotpReassembler::new();
        assert!(r
            .push(1.0, &[0x10, 0x0C, 0x41, 0x02, 1, 2, 3, 4])
            .is_none());
        let msg = r
            .push(1.1, &[0x21, 5, 6, 7, 8, 9, 10, 0xAA])
            .expect("complete");
        assert_eq!(msg.data, vec![0x41, 0x02, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(msg.timestamp, 1.1);
    }

    #[test]
    fn out_of_sequence_resets() {
        let mut r = IsotpReassembler::new();
        assert!(r
            .push(1.0, &[0x10, 0x0C, 0x41, 0x02, 1, 2, 3, 4])
            .is_none());
        // Wrong sequence number (0x22 instead of 0x21) drops the transfer.
        assert!(r.push(1.1, &[0x22, 5, 6, 7, 8, 9, 10, 11]).is_none());
        assert!(r.push(1.2, &[0x21, 5, 6, 7, 8, 9, 10, 11]).is_none());
    }

    #[test]
    fn flow_control_is_ignored() {
        let mut r = IsotpReassembler::new();
        assert!(r.push(1.0, &flow_control_cts()).is_none());
    }
}
