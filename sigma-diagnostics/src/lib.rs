//! Sigma diagnostics domain library.
//!
//! Reusable CAN/DBC/MDF4 services without desktop UI or session persistence.

#![forbid(unsafe_code)]

pub mod analysis;
pub mod capture;
pub mod dbc;
pub mod dbc_export;
pub mod decode;
pub mod dto;
pub mod filter;
pub mod live_capture;
pub mod mdf;
pub mod obd;
pub mod state;
pub mod updates;
pub mod vehicle;
pub mod vss_frame_map;

pub use state::DiagnosticsState;

pub use dto::{CanBpfFilter, CanFrameDto, DecodedSignalDto};

pub use filter::{
    build_message_cache_from_dbc, calculate_frame_stats, detect_dlc, filter_frames,
    filter_frames_with_cache, get_message_counts, match_data_pattern, parse_data_pattern,
    DbcMessageCache, DbcMessageInfo, DlcDetectionResult, FilterConfig, FilterResult, FrameStats,
    MatchStatus, MessageCount,
};

pub use capture::{
    is_capture_running, list_can_interfaces, start_capture, stop_capture, CaptureSession,
};
pub use dbc::{
    clear_dbc, decode_frames, decode_single_frame, get_dbc_info, get_dbc_path,
    get_dbc_specification, load_dbc, save_dbc_content, save_dbc_info, update_dbc_content,
    update_dbc_info,
};
pub use mdf::{export_logs, load_mdf4, parse_can_dataframe};
pub use updates::{
    download_dbc, fetch_dbc_catalog, fetch_latest_dbc, fetch_latest_dbc_content, DbcCatalogFile,
    UpdatesConfig,
};

pub use vehicle::{
    build_diagnosis_advisor, default_sessions_dir, fetch_channel_latest, load_m7_draft_dbc,
    new_session_path, request_log_export, AiConfig, AnomalyRow, ChannelRelease, DiagnosisAdvisor,
    DiagnosisReading, DiagnosisSnapshot, LogExportRequest, MaintenanceAction, MaintenanceService,
    ModelAdvisor, OtaConfig, ReadingSeverity, ReadingSource, RuleBasedAdvisor, SettingsService,
    StubMaintenanceService, StubSettingsService, TelemetryRecorder, TelemetryReplayer,
    VehicleLinkConfig, VehicleSession, VehicleSessionStatus, VehicleSetting, VehicleTransport,
    VitalSignal, DEFAULT_WIFI_PORT, M7_DRAFT_DBC, M7_DRAFT_DBC_NAME,
};
