//! Live OBD-II poller over SocketCAN (Linux only).
//!
//! The poller's only job is to *stimulate* the ECU: discover which Service
//! `$01` PIDs are supported, then round-robin request the chosen ones. It is
//! designed to run while [`crate::capture::start_capture`] records the same
//! interface, so all responses land in the MDF4 alongside broadcast traffic;
//! ground truth is then extracted **offline** by [`super::extract`]. Keeping
//! extraction offline means the definitive results never depend on live
//! timing or a lossy socket read.

use super::isotp::{encode_single_frame, OBD_ECU_RESPONSE_ID, OBD_FUNCTIONAL_ID};
use super::pids::{parse_supported_bitmask, service01_request, SUPPORTED_BITMASK_PIDS};

/// Build the raw 8-byte CAN payload for a Service `$01` PID request.
pub fn request_payload(pid: u8) -> [u8; 8] {
    // service01_request never exceeds 7 bytes, so this cannot fail.
    encode_single_frame(&service01_request(pid)).expect("2-byte OBD request fits a single frame")
}

/// Result of supported-PID discovery.
#[derive(Debug, Clone, Default)]
pub struct SupportedPids {
    pub pids: Vec<u8>,
}

#[cfg(target_os = "linux")]
mod linux {
    use super::*;
    use socketcan::{CanSocket, EmbeddedFrame, Socket, StandardId};
    use socketcan::embedded_can;
    use std::time::{Duration, Instant};

    fn open(interface: &str) -> Result<CanSocket, String> {
        let socket =
            CanSocket::open(interface).map_err(|e| format!("open {interface}: {e}"))?;
        socket
            .set_read_timeout(Duration::from_millis(200))
            .map_err(|e| format!("set timeout: {e}"))?;
        Ok(socket)
    }

    fn send(socket: &CanSocket, id: u32, payload: &[u8; 8]) -> Result<(), String> {
        let frame = socketcan::CanFrame::new(
            StandardId::new(id as u16).ok_or("bad CAN id")?,
            payload,
        )
        .ok_or("frame build failed")?;
        socket
            .write_frame(&frame)
            .map_err(|e| format!("transmit: {e}"))
    }

    /// Read one Service $01 response for `pid` within `timeout`, returning the
    /// response data bytes (after the PID echo). Uses functional addressing.
    fn request_once(
        socket: &CanSocket,
        pid: u8,
        timeout: Duration,
    ) -> Result<Option<Vec<u8>>, String> {
        send(socket, OBD_FUNCTIONAL_ID, &request_payload(pid))?;
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            match socket.read_frame() {
                Ok(frame) => {
                    let embedded_can::Id::Standard(id) = frame.id() else {
                        continue;
                    };
                    if u32::from(id.as_raw()) != OBD_ECU_RESPONSE_ID {
                        continue;
                    }
                    let data = frame.data();
                    // Single-frame positive response: len, 0x41, pid, data...
                    if data.len() >= 3 && data[1] == 0x41 && data[2] == pid {
                        let n = (data[0] as usize).min(data.len() - 1);
                        return Ok(Some(data[2..1 + n].to_vec()));
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(format!("receive: {e}")),
            }
        }
        Ok(None)
    }

    /// Discover supported Service $01 PIDs via the 0x00/0x20/0x40/0x60 masks.
    pub fn discover_supported_pids(interface: &str) -> Result<SupportedPids, String> {
        let socket = open(interface)?;
        let mut pids = Vec::new();
        for &base in &SUPPORTED_BITMASK_PIDS {
            match request_once(&socket, base, Duration::from_millis(300))? {
                Some(data) if data.len() >= 5 => {
                    // data = [pid_echo, b0, b1, b2, b3]
                    pids.extend(parse_supported_bitmask(base, &data[1..5]));
                }
                _ => break, // no response to this mask → higher ranges unsupported
            }
        }
        pids.sort_unstable();
        pids.dedup();
        Ok(SupportedPids { pids })
    }

    /// Round-robin poll `pids` for `duration`, pausing `period` between
    /// requests. Responses are not returned here — they are captured into the
    /// concurrently-recording MDF4 and extracted offline.
    pub fn poll_pids(
        interface: &str,
        pids: &[u8],
        period: Duration,
        duration: Duration,
    ) -> Result<u64, String> {
        if pids.is_empty() {
            return Err("no PIDs to poll".to_string());
        }
        let socket = open(interface)?;
        let deadline = Instant::now() + duration;
        let mut sent = 0u64;
        let mut cursor = 0usize;
        while Instant::now() < deadline {
            let pid = pids[cursor % pids.len()];
            cursor += 1;
            send(&socket, OBD_FUNCTIONAL_ID, &request_payload(pid))?;
            sent += 1;
            // Drain any responses so the socket buffer does not overflow; the
            // recorder captures them independently.
            let drain_until = Instant::now() + period;
            while Instant::now() < drain_until {
                match socket.read_frame() {
                    Ok(_) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(_) => break,
                }
            }
        }
        Ok(sent)
    }
}

#[cfg(target_os = "linux")]
pub use linux::{discover_supported_pids, poll_pids};

#[cfg(not(target_os = "linux"))]
pub fn discover_supported_pids(_interface: &str) -> Result<SupportedPids, String> {
    Err("SocketCAN OBD polling is only available on Linux".to_string())
}

#[cfg(not(target_os = "linux"))]
pub fn poll_pids(
    _interface: &str,
    _pids: &[u8],
    _period: std::time::Duration,
    _duration: std::time::Duration,
) -> Result<u64, String> {
    Err("SocketCAN OBD polling is only available on Linux".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_payload_is_a_valid_single_frame() {
        let p = request_payload(0x0C);
        assert_eq!(p[0], 0x02); // length
        assert_eq!(p[1], 0x01); // service
        assert_eq!(p[2], 0x0C); // pid
        assert_eq!(&p[3..], &[0, 0, 0, 0, 0]); // padding
    }
}
