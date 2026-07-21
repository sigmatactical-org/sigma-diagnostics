//! Slint UI wiring and application controller.

mod analysis;
mod analyze;
mod catalog;
mod dbc;
mod helpers;
mod live;
mod mdf4;

pub use analysis::{wire_analysis_tabs, AnalysisControllers};
pub use catalog::CatalogController;
pub use dbc::DbcController;
pub use live::LiveController;
pub use mdf4::Mdf4Controller;

use crate::state::AppState;
use crate::SigmaDiagnostics;
use slint::ComponentHandle;
use std::sync::Arc;

/// Run the Slint application event loop.
pub fn run(state: Arc<AppState>) -> Result<(), slint::PlatformError> {
    let ui = SigmaDiagnostics::new()?;
    ui.set_version_text(env!("CARGO_PKG_VERSION").into());
    ui.set_status_text("Ready".into());

    let _analysis = wire_analysis_tabs(state, &ui);

    ui.run()
}
