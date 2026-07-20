//! Desktop service wrappers (session persistence + re-exports).

mod dbc;
mod init;
mod mdf;

pub use dbc::*;
pub use init::*;
pub use mdf::*;

pub use sigma_diagnostics::analysis::{
    survey_frames, BitState, IdSurvey, SurveyOptions, SurveyReport,
};

pub use sigma_diagnostics::{
    clear_dbc as clear_dbc_domain, decode_frames, decode_single_frame, download_dbc, export_logs,
    fetch_dbc_catalog, fetch_latest_dbc, fetch_latest_dbc_content, filter_frames, get_dbc_info,
    get_dbc_path, get_dbc_specification, is_capture_running, list_can_interfaces, start_capture,
    stop_capture, update_dbc_content, update_dbc_info, CaptureSession, DbcCatalogFile,
    FilterConfig, MatchStatus, UpdatesConfig,
};
