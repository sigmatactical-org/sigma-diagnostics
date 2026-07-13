//! On-bike log export (bulk MDF4 pull deferred; CAN trigger stub).

#[derive(Debug, Clone, Default)]
pub struct LogExportRequest {
    pub note: String,
}

/// Ask the ECU to prepare a log for export. WiFi sessions are recorded as NDJSON from Connect.
pub fn request_log_export(_req: &LogExportRequest) -> Result<String, String> {
    Err(
        "Use Connect → WiFi to record a session, or Logs → Replay session for saved .jsonl files."
            .into(),
    )
}
