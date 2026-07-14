use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use sigma_racer_telemetry::protocol::Message;
use sigma_racer_telemetry::VehicleState;

/// Offline replay of a saved NDJSON session.
pub struct TelemetryReplayer {
    lines: Vec<String>,
    index: usize,
    state: VehicleState,
    seq: u64,
    path: PathBuf,
}

impl TelemetryReplayer {
    /// Load a recorded session for replay.
    pub fn open(path: PathBuf) -> Result<Self, String> {
        let file = File::open(&path).map_err(|e| format!("open session: {e}"))?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        for line in reader.lines() {
            let line = line.map_err(|e| format!("read session: {e}"))?;
            if !line.trim().is_empty() {
                lines.push(line);
            }
        }
        if lines.is_empty() {
            return Err("Session file is empty".into());
        }
        Ok(Self {
            lines,
            index: 0,
            state: VehicleState::idle(),
            seq: 0,
            path,
        })
    }

    /// The session file being replayed.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Number of messages in the session.
    pub fn total_lines(&self) -> usize {
        self.lines.len()
    }

    /// Messages replayed so far.
    pub fn position(&self) -> usize {
        self.index
    }

    /// Whether the whole session has been replayed.
    pub fn finished(&self) -> bool {
        self.index >= self.lines.len()
    }

    /// Vehicle state after the messages replayed so far.
    pub fn state(&self) -> &VehicleState {
        &self.state
    }

    /// Sequence number of the last replayed message.
    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// Advance one frame; returns false when the session ends.
    pub fn step(&mut self) -> bool {
        if self.index >= self.lines.len() {
            return false;
        }
        let line = &self.lines[self.index];
        self.index += 1;
        if let Ok(msg) = Message::parse_validated(line) {
            self.seq = msg.seq;
            if let Some(data) = msg.vss_data() {
                self.state.apply_vss_map(data);
            }
        }
        true
    }

    /// Rewind to the start of the session.
    pub fn reset(&mut self) {
        self.index = 0;
        self.state = VehicleState::idle();
        self.seq = 0;
    }
}
