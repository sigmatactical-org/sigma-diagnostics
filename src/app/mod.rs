//! Slint UI wiring and application controller.

mod catalog;
mod dbc;
mod helpers;
mod live;
mod mdf4;

use crate::state::AppState;
use crate::SigmaCanViewer;
use slint::ComponentHandle;
use std::rc::Rc;
use std::sync::Arc;

/// Run the Slint application event loop.
pub fn run(state: Arc<AppState>) -> Result<(), slint::PlatformError> {
    let ui = SigmaCanViewer::new()?;
    ui.set_version_text(env!("CARGO_PKG_VERSION").into());
    ui.set_status_text("Ready".into());
    crate::about::populate(&ui);

    let mdf4 = Arc::new(mdf4::Mdf4Controller::new(state.clone(), ui.as_weak()));
    let live = Rc::new(live::LiveController::new(state.clone(), ui.as_weak()));
    let dbc = Arc::new(dbc::DbcController::new(state.clone(), ui.as_weak()));
    let catalog = Arc::new(catalog::CatalogController::new(
        state.clone(),
        ui.as_weak(),
        Arc::downgrade(&mdf4),
        Arc::downgrade(&dbc),
    ));

    mdf4::Mdf4Controller::wire(mdf4.clone(), &ui);
    live::LiveController::wire(live.clone(), &ui);
    dbc::DbcController::wire(dbc.clone(), &ui);
    catalog::CatalogController::wire(catalog, &ui);

    mdf4.load_initial_files();

    ui.run()
}
