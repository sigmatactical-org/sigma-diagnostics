//! Filesystem locations for recorded telemetry sessions.

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Where recorded telemetry sessions are stored.
pub fn default_sessions_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".config")
        .join("sigma-racer-mechanic")
        .join("sessions")
}

/// Timestamped path for a new session recording.
pub fn new_session_path() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    default_sessions_dir().join(format!("wingman-{stamp}.jsonl"))
}
