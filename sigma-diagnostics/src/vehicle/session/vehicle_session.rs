use std::path::PathBuf;

use parking_lot::Mutex;
use sigma_racer_telemetry::anomaly::AnomalyEngine;
use sigma_racer_telemetry::parse_ts_millis;
use sigma_racer_telemetry::TcpTelemetryClient;
use sigma_racer_telemetry::VehicleState;

use crate::capture::{self, CaptureSession};
use crate::state::DiagnosticsState;
use crate::vehicle::diagnosis::{AnomalyRow, DiagnosisSnapshot};
use crate::vehicle::m7;
use crate::vehicle::recording::{new_session_path, TelemetryRecorder, TelemetryReplayer};
use crate::vehicle::transport::VehicleTransport;

use super::{VehicleLinkConfig, VehicleSessionStatus};

fn capture_log_path() -> PathBuf {
    std::env::temp_dir().join("sigma-racer-mechanic-capture.mf4")
}

/// Newest anomaly events kept for the diagnosis panel.
const ANOMALY_LOG_CAP: usize = 200;

/// Active vehicle link for Mechanic (SocketCAN or Wingman WiFi telemetry).
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
    /// Shop-side detector engine (same detectors as the bike daemon).
    anomalies: Mutex<AnomalyEngine>,
    anomaly_log: Mutex<Vec<AnomalyRow>>,
}

impl Default for VehicleSession {
    fn default() -> Self {
        Self::new()
    }
}

impl VehicleSession {
    /// Disconnected session with config from the environment.
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
            anomalies: Mutex::new(AnomalyEngine::sigma_defaults()),
            anomaly_log: Mutex::new(Vec::new()),
        }
    }

    fn reset_anomalies(&self) {
        *self.anomalies.lock() = AnomalyEngine::sigma_defaults();
        self.anomaly_log.lock().clear();
    }

    /// Feed one telemetry message through the shop-side detector engine:
    /// bike-raised `Event` messages are merged idempotently; state messages
    /// run the local detectors with the message's own timestamp.
    fn observe_message(&self, msg: &sigma_racer_telemetry::Message, state: &VehicleState) {
        let mut engine = self.anomalies.lock();
        let mut log = self.anomaly_log.lock();
        if msg.msg == "Event" {
            if let Some(ev) = engine.ingest_event(msg) {
                push_row(&mut log, AnomalyRow::from_event(&ev, "bike"));
            }
        } else if let Some(ts) = parse_ts_millis(&msg.ts) {
            for ev in engine.observe(ts, state) {
                push_row(&mut log, AnomalyRow::from_event(ev, "shop"));
            }
        }
    }

    /// Copy the anomaly log and worst-active severity into a snapshot,
    /// refreshing each row's `active` flag from the engine.
    fn fill_anomalies(&self, snap: &mut DiagnosisSnapshot) {
        let engine = self.anomalies.lock();
        let active_ids: Vec<&str> = engine.active().map(|slot| slot.id).collect();
        let mut rows = self.anomaly_log.lock().clone();
        for row in &mut rows {
            row.active = active_ids.contains(&row.id.as_str());
        }
        snap.worst_anomaly = engine
            .worst_active()
            .map(|(_, sev)| sev.label().to_string())
            .unwrap_or_default();
        snap.anomalies = rows;
    }

    /// Current link status.
    pub fn status(&self) -> VehicleSessionStatus {
        *self.status.lock()
    }

    /// Most recent connect/stream error, if any.
    pub fn last_error(&self) -> Option<String> {
        self.last_error.lock().clone()
    }

    /// Copy of the active link configuration.
    pub fn config(&self) -> VehicleLinkConfig {
        self.config.lock().clone()
    }

    /// Replace the link configuration (takes effect on next connect).
    pub fn set_config(&self, cfg: VehicleLinkConfig) {
        *self.config.lock() = cfg;
    }

    /// Path of the active session recording, if recording.
    pub fn recording_path(&self) -> Option<PathBuf> {
        self.recording_path.lock().clone()
    }

    /// Connect over the configured transport and start streaming.
    pub fn connect(&self, state: &DiagnosticsState) -> Result<(), String> {
        let cfg = self.config.lock().clone();
        *self.status.lock() = VehicleSessionStatus::Connecting;
        *self.last_error.lock() = None;
        self.stop_replay();
        self.teardown_links(state);
        self.reset_anomalies();

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

    /// Stop streaming and drop the link.
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

    /// Replay a recorded session instead of a live link.
    pub fn start_replay(&self, path: PathBuf) -> Result<(), String> {
        self.teardown_links(&DiagnosticsState::default());
        let replayer = TelemetryReplayer::open(path)?;
        *self.replayer.lock() = Some(replayer);
        *self.status.lock() = VehicleSessionStatus::Replaying;
        *self.last_error.lock() = None;
        self.reset_anomalies();
        Ok(())
    }

    /// Stop an active replay.
    pub fn stop_replay(&self) {
        *self.replayer.lock() = None;
        if matches!(*self.status.lock(), VehicleSessionStatus::Replaying) {
            *self.status.lock() = VehicleSessionStatus::Disconnected;
        }
    }

    /// Whether a live link or replay is currently feeding state.
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

    /// Snapshot vehicle state + link health for the diagnosis view.
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
            // Sigma DBC: the capture rebuilt VehicleState from mapped signals,
            // so the shop-side detectors run here too (parity with WiFi).
            Some(display) if display.vehicle_state.is_some() => {
                let state = display.vehicle_state.as_ref().unwrap();
                self.observe_state(state, now_millis());
                let mut snap = DiagnosisSnapshot::from_vehicle_state(
                    state,
                    true,
                    display.stats.frame_count,
                    "Receiving telemetry (SocketCAN)",
                );
                self.fill_anomalies(&mut snap);
                snap
            }
            // Non-Sigma DBC (no mapped signals): raw frame/signal display only.
            Some(display) => DiagnosisSnapshot::from_live_display(&display, true),
            None => DiagnosisSnapshot {
                connected: true,
                status: "Waiting for frames".into(),
                ..DiagnosisSnapshot::default()
            },
        }
    }

    /// Run the shop-side detectors over a rebuilt state at `ts_ms`.
    fn observe_state(&self, state: &VehicleState, ts_ms: i64) {
        let mut engine = self.anomalies.lock();
        let mut log = self.anomaly_log.lock();
        for ev in engine.observe(ts_ms, state) {
            push_row(&mut log, AnomalyRow::from_event(ev, "shop"));
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
                state.apply_availability(msg.avail_data());
            }
            self.observe_message(&msg, &state);
            if let Some(rec) = recorder.as_mut() {
                let _ = rec.write_message(&msg);
            }
        }

        let mut snap =
            DiagnosisSnapshot::from_vehicle_state(&state, true, *seq, "Receiving telemetry (WiFi)");
        self.fill_anomalies(&mut snap);
        snap
    }

    fn poll_replay(&self) -> DiagnosisSnapshot {
        let mut replay = self.replayer.lock();
        let Some(replayer) = replay.as_mut() else {
            return DiagnosisSnapshot::disconnected("Replay stopped");
        };

        let stepped = replayer.step_message();
        if let Some(msg) = &stepped {
            self.observe_message(msg, replayer.state());
        }

        let mut snap = if stepped.is_none() && replayer.finished() {
            DiagnosisSnapshot::from_vehicle_state(
                replayer.state(),
                false,
                replayer.seq(),
                "Replay finished",
            )
        } else {
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
        };
        self.fill_anomalies(&mut snap);
        snap
    }
}

/// Wall-clock epoch milliseconds for live detector observations.
fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Append with a bounded log: oldest rows fall off first.
fn push_row(log: &mut Vec<AnomalyRow>, row: AnomalyRow) {
    if log.len() >= ANOMALY_LOG_CAP {
        log.remove(0);
    }
    log.push(row);
}
