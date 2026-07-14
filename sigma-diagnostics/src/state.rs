//! Shared diagnostics runtime state (DBC + capture).

use dbc_rs::{Dbc, FastDbc};
use parking_lot::Mutex;

/// Channel used to request capture stop and receive the finalize result sender.
#[cfg(target_os = "linux")]
pub type CaptureStopTx = std::sync::mpsc::Sender<std::sync::mpsc::Sender<Result<String, String>>>;

/// Domain state shared across diagnostics services (no desktop session config).
pub struct DiagnosticsState {
    /// Loaded DBC database for signal decoding.
    pub dbc: Mutex<Option<Dbc>>,

    /// High-performance DBC wrapper with O(1) message lookup.
    pub fast_dbc: Mutex<Option<FastDbc>>,

    /// Path to the currently loaded DBC file.
    pub dbc_path: Mutex<Option<String>>,

    /// Whether a SocketCAN capture is currently running.
    #[cfg(target_os = "linux")]
    pub capture_running: Mutex<bool>,

    /// Channel to signal capture to stop and receive result.
    #[cfg(target_os = "linux")]
    pub capture_stop_tx: Mutex<Option<CaptureStopTx>>,
}

impl DiagnosticsState {
    /// Empty diagnostics state.
    pub fn new() -> Self {
        Self {
            dbc: Mutex::new(None),
            fast_dbc: Mutex::new(None),
            dbc_path: Mutex::new(None),
            #[cfg(target_os = "linux")]
            capture_running: Mutex::new(false),
            #[cfg(target_os = "linux")]
            capture_stop_tx: Mutex::new(None),
        }
    }

    /// Set the DBC database and create the corresponding FastDbc wrapper.
    pub fn set_dbc(&self, dbc: Dbc) {
        let fast_dbc = FastDbc::new(dbc.clone());
        *self.dbc.lock() = Some(dbc);
        *self.fast_dbc.lock() = Some(fast_dbc);
    }

    /// Clear the DBC database and FastDbc wrapper.
    pub fn clear_dbc(&self) {
        *self.dbc.lock() = None;
        *self.fast_dbc.lock() = None;
    }
}

impl Default for DiagnosticsState {
    fn default() -> Self {
        Self::new()
    }
}
