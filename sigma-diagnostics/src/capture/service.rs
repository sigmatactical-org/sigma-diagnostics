//! SocketCAN capture services (Linux only).

use crate::dto::CanBpfFilter;
use crate::state::DiagnosticsState;

use super::CaptureSession;

/// List available SocketCAN interfaces.
#[cfg(target_os = "linux")]
pub fn list_can_interfaces() -> Result<Vec<String>, String> {
    use std::fs;

    let mut interfaces = Vec::new();

    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let type_path = entry.path().join("type");

            if let Ok(type_val) = fs::read_to_string(&type_path) {
                if type_val.trim() == "280" && !interfaces.contains(&name) {
                    interfaces.push(name.clone());
                }
            }

            if (name.starts_with("vcan") || name.starts_with("can")) && !interfaces.contains(&name)
            {
                interfaces.push(name);
            }
        }
    }

    Ok(interfaces)
}

#[cfg(not(target_os = "linux"))]
pub fn list_can_interfaces() -> Result<Vec<String>, String> {
    Err("SocketCAN is only available on Linux".to_string())
}

/// Start capturing CAN frames from an interface.
#[cfg(target_os = "linux")]
pub fn start_capture(
    interface: &str,
    capture_file: &str,
    append: bool,
    filters: Option<Vec<CanBpfFilter>>,
    state: &DiagnosticsState,
) -> Result<CaptureSession, String> {
    use super::capture_message::CaptureMessage;
    use crate::dto::{CanErrorDto, CanFrameDto, LiveCaptureDisplay};
    use crate::live_capture::LiveCaptureState;
    use mdf4_rs::can::RawCanLogger;
    use socketcan::{CanFdSocket, CanFilter, Socket, SocketOptions};
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    if *state.capture_running.lock() {
        return Err("Capture already running".to_string());
    }

    let socket =
        CanFdSocket::open(interface).map_err(|e| format!("Failed to open interface: {e}"))?;

    if let Some(ref filter_list) = filters {
        if !filter_list.is_empty() {
            let can_filters: Vec<CanFilter> = filter_list
                .iter()
                .map(|f| {
                    let can_id = if f.is_extended {
                        f.can_id | 0x8000_0000
                    } else {
                        f.can_id
                    };
                    let mask = if f.is_extended {
                        f.mask | 0x8000_0000
                    } else {
                        f.mask
                    };

                    if f.inverted {
                        CanFilter::new_inverted(can_id, mask)
                    } else {
                        CanFilter::new(can_id, mask)
                    }
                })
                .collect();

            socket
                .set_filters(&can_filters)
                .map_err(|e| format!("Failed to set BPF filters: {e}"))?;

            log::info!(
                "Applied {} kernel-level CAN filter(s) on {}",
                can_filters.len(),
                interface
            );
        }
    }

    socket
        .set_nonblocking(true)
        .map_err(|e| format!("Failed to set non-blocking: {e}"))?;

    let (msg_tx, msg_rx) = mpsc::sync_channel::<CaptureMessage>(1024);
    let (update_tx, update_rx) = mpsc::channel::<LiveCaptureDisplay>();
    let (stop_tx, stop_rx) = mpsc::channel::<mpsc::Sender<Result<String, String>>>();

    *state.capture_stop_tx.lock() = Some(stop_tx);
    *state.capture_running.lock() = true;

    let fast_dbc_clone = state.fast_dbc.lock().clone();
    let interface_name = interface.to_string();
    let capture_file_for_socket = capture_file.to_string();

    let append_mode = append;
    std::thread::spawn(move || {
        use embedded_can::blocking::Can;

        let mut socket = socket;
        let start_time = Instant::now();
        let stop_rx = stop_rx;

        let (mut logger, timestamp_offset_us) =
            if append_mode && std::path::Path::new(&capture_file_for_socket).exists() {
                match RawCanLogger::from_file(&capture_file_for_socket) {
                    Ok(l) => {
                        let offset = l.last_timestamp_us();
                        (Some(l), offset)
                    }
                    Err(_) => (RawCanLogger::new().ok(), 0),
                }
            } else {
                (RawCanLogger::new().ok(), 0)
            };

        loop {
            if let Ok(result_tx) = stop_rx.try_recv() {
                let result = finalize_mdf4(logger, &capture_file_for_socket);
                let _ = result_tx.send(result);
                return;
            }

            match socket.receive() {
                Ok(frame) => {
                    let timestamp = start_time.elapsed().as_secs_f64()
                        + (timestamp_offset_us as f64 / 1_000_000.0);

                    if let socketcan::CanAnyFrame::Error(err_frame) = frame {
                        let error_dto =
                            CanErrorDto::from_error_frame(err_frame, timestamp, &interface_name);
                        let _ = msg_tx.try_send(CaptureMessage::Error(error_dto));
                    } else if let Some(frame_dto) =
                        CanFrameDto::from_any_frame(&frame, timestamp, &interface_name)
                    {
                        log_frame_to_mdf4(&mut logger, &frame_dto);
                        let _ = msg_tx.try_send(CaptureMessage::Frame(frame_dto));
                    }
                }
                Err(socketcan::Error::Io(ref e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(1));
                }
                Err(e) => {
                    log::error!("Capture read error: {e}");
                    let _ = finalize_mdf4(logger, &capture_file_for_socket);
                    return;
                }
            }
        }
    });

    let capture_file_owned = capture_file.to_string();
    std::thread::spawn(move || {
        let mut capture_state = LiveCaptureState::new(capture_file_owned, fast_dbc_clone);
        let mut last_update = Instant::now();
        let update_interval = Duration::from_millis(100);

        loop {
            match msg_rx.recv_timeout(Duration::from_millis(10)) {
                Ok(CaptureMessage::Frame(frame)) => {
                    capture_state.process_frame(frame);
                }
                Ok(CaptureMessage::Error(error)) => {
                    capture_state.process_error(
                        error.timestamp,
                        &error.channel,
                        &error.error_type,
                        &error.details,
                    );
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => return,
            }

            if last_update.elapsed() >= update_interval {
                capture_state.update_rates();
                let update = capture_state.generate_display();
                let _ = update_tx.send(update);
                last_update = Instant::now();
            }
        }
    });

    Ok(CaptureSession { update_rx })
}

#[cfg(target_os = "linux")]
fn log_frame_to_mdf4(
    logger: &mut Option<mdf4_rs::can::RawCanLogger<mdf4_rs::writer::VecWriter>>,
    frame: &crate::dto::CanFrameDto,
) {
    if let Some(logger) = logger {
        crate::mdf::log_frame(logger, frame);
    }
}

#[cfg(target_os = "linux")]
fn finalize_mdf4(
    logger: Option<mdf4_rs::can::RawCanLogger<mdf4_rs::writer::VecWriter>>,
    path: &str,
) -> Result<String, String> {
    if let Some(logger) = logger {
        let bytes = logger
            .finalize()
            .map_err(|e| format!("Failed to finalize MDF4: {e:?}"))?;
        std::fs::write(path, bytes).map_err(|e| format!("Failed to write MDF4: {e}"))?;
    }
    Ok(path.to_string())
}

#[cfg(not(target_os = "linux"))]
pub fn start_capture(
    _interface: &str,
    _capture_file: &str,
    _append: bool,
    _filters: Option<Vec<CanBpfFilter>>,
    _state: &DiagnosticsState,
) -> Result<CaptureSession, String> {
    Err("SocketCAN is only available on Linux".to_string())
}

/// Stop the current capture and wait for MDF4 to be written.
#[cfg(target_os = "linux")]
pub fn stop_capture(state: &DiagnosticsState) -> Result<String, String> {
    use std::sync::mpsc;

    let stop_tx = state.capture_stop_tx.lock().take();

    if let Some(tx) = stop_tx {
        let (result_tx, result_rx) = mpsc::channel();

        if tx.send(result_tx).is_err() {
            return Err("Capture thread already stopped".to_string());
        }

        let result = result_rx
            .recv()
            .map_err(|_| "Capture thread panicked".to_string())?;

        *state.capture_running.lock() = false;
        result
    } else {
        *state.capture_running.lock() = false;
        Err("No capture running".to_string())
    }
}

#[cfg(not(target_os = "linux"))]
pub fn stop_capture(_state: &DiagnosticsState) -> Result<String, String> {
    Err("SocketCAN is only available on Linux".to_string())
}

/// Check if a capture is currently running.
pub fn is_capture_running(state: &DiagnosticsState) -> bool {
    #[cfg(target_os = "linux")]
    {
        *state.capture_running.lock()
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = state;
        false
    }
}
