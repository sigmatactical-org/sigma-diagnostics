//! Active vehicle link for Mechanic (SocketCAN or Wingman WiFi telemetry).

use crate::capture::{self, CaptureSession};
use crate::state::DiagnosticsState;
use crate::vehicle::diagnosis::DiagnosisSnapshot;
use crate::vehicle::m7;
use crate::vehicle::recording::{new_session_path, TelemetryRecorder, TelemetryReplayer};
use crate::vehicle::transport::VehicleTransport;
use parking_lot::Mutex;
use sigma_racer_telemetry::TcpTelemetryClient;
use sigma_racer_telemetry::VehicleState;
use sigma_racer_telemetry::DEFAULT_TCP_PORT;
use std::path::PathBuf;

/// Default Wingman telemetry relay port.
pub const DEFAULT_WIFI_PORT: u16 = DEFAULT_TCP_PORT;

/// Link parameters for SocketCAN or WiFi telemetry.
#[derive(Debug, Clone)]
pub struct VehicleLinkConfig {
    pub transport: VehicleTransport,
    pub interface: String,
    pub bitrate: u32,
    pub wifi_host: String,
    pub wifi_port: u16,
    pub use_m7_draft_dbc: bool,
    pub record_session: bool,
}

impl Default for VehicleLinkConfig {
    fn default() -> Self {
        Self {
            transport: VehicleTransport::SocketCan,
            interface: "can0".into(),
            bitrate: 500_000,
            wifi_host: String::new(),
            wifi_port: DEFAULT_WIFI_PORT,
            use_m7_draft_dbc: true,
            record_session: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleSessionStatus {
    Disconnected,
    Connecting,
    Connected,
    Replaying,
    Error,
}

impl VehicleSessionStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting",
            Self::Connected => "Connected",
            Self::Replaying => "Replaying",
            Self::Error => "Error",
        }
    }
}

fn capture_log_path() -> PathBuf {
    std::env::temp_dir().join("sigma-racer-mechanic-capture.mf4")
}

pub struct VehicleSession {
    config: Mutex<VehicleLinkConfig>,
    status: Mutex<VehicleSessionStatus>,
    last_error: Mutex<Option<String>>,
    capture: Mutex<Option<CaptureSession>>,
    wifi_client: Mutex<Option<TcpTelemetryClient>>,
    vehicle_state: Mutex<VehicleState>,
    telemetry_seq: Mutex<u64>,
    recorder: Mutex<Option<TelemetryRecorder>>,
    replayer: Mutex<Option<TelemetryReplayer>>,
    recording_path: Mutex<Option<PathBuf>>,
}

impl Default for VehicleSession {
    fn default() -> Self {
        Self::new()
    }
}

impl VehicleSession {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(VehicleLinkConfig::default()),
            status: Mutex::new(VehicleSessionStatus::Disconnected),
            last_error: Mutex::new(None),
            capture: Mutex::new(None),
            wifi_client: Mutex::new(None),
            vehicle_state: Mutex::new(VehicleState::idle()),
            telemetry_seq: Mutex::new(0),
            recorder: Mutex::new(None),
            replayer: Mutex::new(None),
            recording_path: Mutex::new(None),
        }
    }

    pub fn status(&self) -> VehicleSessionStatus {
        *self.status.lock()
    }

    pub fn last_error(&self) -> Option<String> {
        self.last_error.lock().clone()
    }

    pub fn config(&self) -> VehicleLinkConfig {
        self.config.lock().clone()
    }

    pub fn set_config(&self, cfg: VehicleLinkConfig) {
        *self.config.lock() = cfg;
    }

    pub fn recording_path(&self) -> Option<PathBuf> {
        self.recording_path.lock().clone()
    }

    pub fn connect(&self, state: &DiagnosticsState) -> Result<(), String> {
        let cfg = self.config.lock().clone();
        *self.status.lock() = VehicleSessionStatus::Connecting;
        *self.last_error.lock() = None;
        self.stop_replay();
        self.teardown_links(state);

        if cfg.use_m7_draft_dbc {
            m7::load_m7_draft_dbc(state)?;
        }

        match cfg.transport {
            VehicleTransport::SocketCan => self.connect_socketcan(state, &cfg),
            VehicleTransport::WiFi => self.connect_wifi(&cfg),
        }
    }

    fn connect_socketcan(
        &self,
        state: &DiagnosticsState,
        cfg: &VehicleLinkConfig,
    ) -> Result<(), String> {
        let log_path = capture_log_path();
        let log_str = log_path.to_string_lossy().to_string();
        match capture::start_capture(&cfg.interface, &log_str, false, None, state) {
            Ok(session) => {
                *self.capture.lock() = Some(session);
                *self.status.lock() = VehicleSessionStatus::Connected;
                Ok(())
            }
            Err(e) => {
                *self.status.lock() = VehicleSessionStatus::Error;
                *self.last_error.lock() = Some(e.clone());
                Err(e)
            }
        }
    }

    fn connect_wifi(&self, cfg: &VehicleLinkConfig) -> Result<(), String> {
        let host = cfg.wifi_host.trim();
        if host.is_empty() {
            let err: String = "WiFi host is required (Wingman IP on the shop network)".into();
            *self.status.lock() = VehicleSessionStatus::Error;
            *self.last_error.lock() = Some(err.clone());
            return Err(err);
        }

        let client = TcpTelemetryClient::connect(host, cfg.wifi_port)?;
        *self.wifi_client.lock() = Some(client);
        *self.vehicle_state.lock() = VehicleState::idle();
        *self.telemetry_seq.lock() = 0;

        if cfg.record_session {
            let path = new_session_path();
            let recorder = TelemetryRecorder::start(path.clone())?;
            *self.recording_path.lock() = Some(path);
            *self.recorder.lock() = Some(recorder);
        } else {
            *self.recording_path.lock() = None;
            *self.recorder.lock() = None;
        }

        *self.status.lock() = VehicleSessionStatus::Connected;
        Ok(())
    }

    pub fn disconnect(&self, state: &DiagnosticsState) {
        self.teardown_links(state);
        *self.status.lock() = VehicleSessionStatus::Disconnected;
    }

    fn teardown_links(&self, state: &DiagnosticsState) {
        let _ = capture::stop_capture(state);
        *self.capture.lock() = None;
        *self.wifi_client.lock() = None;
        *self.recorder.lock() = None;
    }

    pub fn start_replay(&self, path: PathBuf) -> Result<(), String> {
        self.teardown_links(&DiagnosticsState::default());
        let replayer = TelemetryReplayer::open(path)?;
        *self.replayer.lock() = Some(replayer);
        *self.status.lock() = VehicleSessionStatus::Replaying;
        *self.last_error.lock() = None;
        Ok(())
    }

    pub fn stop_replay(&self) {
        *self.replayer.lock() = None;
        if matches!(*self.status.lock(), VehicleSessionStatus::Replaying) {
            *self.status.lock() = VehicleSessionStatus::Disconnected;
        }
    }

    pub fn is_connected(&self, state: &DiagnosticsState) -> bool {
        match *self.status.lock() {
            VehicleSessionStatus::Connected => match self.config.lock().transport {
                VehicleTransport::SocketCan => capture::is_capture_running(state),
                VehicleTransport::WiFi => self.wifi_client.lock().is_some(),
            },
            VehicleSessionStatus::Replaying => self.replayer.lock().is_some(),
            _ => false,
        }
    }

    pub fn poll_diagnosis(&self, state: &DiagnosticsState) -> DiagnosisSnapshot {
        if matches!(*self.status.lock(), VehicleSessionStatus::Replaying) {
            return self.poll_replay();
        }

        let cfg = self.config.lock().clone();
        if !self.is_connected(state) {
            let reason = self
                .last_error
                .lock()
                .clone()
                .unwrap_or_else(|| "Not connected".into());
            return DiagnosisSnapshot::disconnected(&reason);
        }

        match cfg.transport {
            VehicleTransport::SocketCan => self.poll_socketcan(),
            VehicleTransport::WiFi => self.poll_wifi(),
        }
    }

    fn poll_socketcan(&self) -> DiagnosisSnapshot {
        let capture = self.capture.lock();
        let Some(session) = capture.as_ref() else {
            return DiagnosisSnapshot::disconnected("No capture session");
        };

        match session.poll_update() {
            Some(display) => DiagnosisSnapshot::from_live_display(&display, true),
            None => DiagnosisSnapshot {
                connected: true,
                status: "Waiting for frames".into(),
                ..DiagnosisSnapshot::default()
            },
        }
    }

    fn poll_wifi(&self) -> DiagnosisSnapshot {
        let mut client_guard = self.wifi_client.lock();
        let Some(client) = client_guard.as_mut() else {
            return DiagnosisSnapshot::disconnected("WiFi telemetry disconnected");
        };

        let mut state = self.vehicle_state.lock();
        let mut seq = self.telemetry_seq.lock();
        let mut recorder = self.recorder.lock();

        for msg in client.drain() {
            *seq = msg.seq;
            if let Some(data) = msg.vss_data() {
                state.apply_vss_map(data);
            }
            if let Some(rec) = recorder.as_mut() {
                let _ = rec.write_message(&msg);
            }
        }

        DiagnosisSnapshot::from_vehicle_state(&state, true, *seq, "Receiving telemetry (WiFi)")
    }

    fn poll_replay(&self) -> DiagnosisSnapshot {
        let mut replay = self.replayer.lock();
        let Some(replayer) = replay.as_mut() else {
            return DiagnosisSnapshot::disconnected("Replay stopped");
        };

        if !replayer.step() {
            return DiagnosisSnapshot::from_vehicle_state(
                replayer.state(),
                false,
                replayer.seq(),
                "Replay finished",
            );
        }

        DiagnosisSnapshot::from_vehicle_state(
            replayer.state(),
            true,
            replayer.seq(),
            &format!(
                "Replaying {}/{}",
                replayer.position(),
                replayer.total_lines()
            ),
        )
    }
}
