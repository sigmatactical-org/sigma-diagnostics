//! Record and replay Wingman NDJSON telemetry sessions.

use sigma_racer_telemetry::protocol::Message;
use sigma_racer_telemetry::VehicleState;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Append-only NDJSON session recorder.
pub struct TelemetryRecorder {
    file: File,
    path: PathBuf,
    lines: u64,
}

impl TelemetryRecorder {
    pub fn start(path: PathBuf) -> Result<Self, String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("create session dir: {e}"))?;
        }
        let file = File::create(&path).map_err(|e| format!("create session file: {e}"))?;
        Ok(Self {
            file,
            path,
            lines: 0,
        })
    }

    pub fn write_message(&mut self, msg: &Message) -> Result<(), String> {
        let line = msg.to_line();
        self.file
            .write_all(line.as_bytes())
            .and_then(|_| self.file.write_all(b"\n"))
            .map_err(|e| format!("write session: {e}"))?;
        self.lines += 1;
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn lines_written(&self) -> u64 {
        self.lines
    }
}

/// Offline replay of a saved NDJSON session.
pub struct TelemetryReplayer {
    lines: Vec<String>,
    index: usize,
    state: VehicleState,
    seq: u64,
    path: PathBuf,
}

impl TelemetryReplayer {
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

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn total_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn position(&self) -> usize {
        self.index
    }

    pub fn finished(&self) -> bool {
        self.index >= self.lines.len()
    }

    pub fn state(&self) -> &VehicleState {
        &self.state
    }

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

    pub fn reset(&mut self) {
        self.index = 0;
        self.state = VehicleState::idle();
        self.seq = 0;
    }
}

pub fn default_sessions_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".config")
        .join("sigma-racer-mechanic")
        .join("sessions")
}

pub fn new_session_path() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    default_sessions_dir().join(format!("wingman-{stamp}.jsonl"))
}
