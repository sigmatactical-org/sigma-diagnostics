use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use sigma_racer_telemetry::protocol::Message;

/// Append-only NDJSON session recorder.
pub struct TelemetryRecorder {
    file: File,
    path: PathBuf,
    lines: u64,
}

impl TelemetryRecorder {
    /// Create the session file and start recording.
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

    /// Append one telemetry message as an NDJSON line.
    pub fn write_message(&mut self, msg: &Message) -> Result<(), String> {
        let line = msg.to_line();
        self.file
            .write_all(line.as_bytes())
            .and_then(|_| self.file.write_all(b"\n"))
            .map_err(|e| format!("write session: {e}"))?;
        self.lines += 1;
        Ok(())
    }

    /// The session file being written.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Messages recorded so far.
    pub fn lines_written(&self) -> u64 {
        self.lines
    }
}
