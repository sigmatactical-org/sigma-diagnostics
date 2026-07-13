//! Vehicle-facing diagnostics for shop tools (Mechanic) over SocketCAN.
//!
//! Protocol PDUs for maintenance/settings writes are defined with Wingman firmware;
//! this module ships stubs and M7 draft decode for live diagnosis.

mod diagnosis;
mod logs;
mod m7;
mod maintenance;
mod ota;
mod session;
mod settings;

pub use diagnosis::{DiagnosisSnapshot, VitalSignal};
pub use logs::{request_log_export, LogExportRequest};
pub use m7::{load_m7_draft_dbc, M7_DRAFT_DBC, M7_DRAFT_DBC_NAME};
pub use maintenance::{MaintenanceAction, MaintenanceService, StubMaintenanceService};
pub use ota::{fetch_channel_latest, ChannelRelease, OtaConfig};
pub use session::{VehicleLinkConfig, VehicleSession, VehicleSessionStatus};
pub use settings::{SettingsService, StubSettingsService, VehicleSetting};
