//! CAN Viewer library.
//!
//! Native Slint desktop application for MDF4 inspection, SocketCAN capture,
//! and DBC editing. Domain logic lives in [`sigma_diagnostics`].

slint::include_modules!();

pub mod about;
pub mod app;
pub mod config;
pub mod services;
pub mod state;

pub use app::{wire_analysis_tabs, AnalysisControllers};
pub use state::{AppState, InitialFiles};

pub use sigma_diagnostics::dbc_export;
pub use sigma_diagnostics::decode;
pub use sigma_diagnostics::dto;
pub use sigma_diagnostics::live_capture;

pub use sigma_diagnostics::{CanBpfFilter, CanFrameDto, DecodedSignalDto};

pub use sigma_diagnostics::{
    build_message_cache_from_dbc, calculate_frame_stats, detect_dlc, filter_frames,
    filter_frames_with_cache, get_message_counts, match_data_pattern, parse_data_pattern,
    DbcMessageCache, DbcMessageInfo, DlcDetectionResult, FilterConfig, FilterResult, FrameStats,
    MatchStatus, MessageCount,
};

pub use config::SessionConfig;
pub use sigma_diagnostics::parse_can_dataframe;
