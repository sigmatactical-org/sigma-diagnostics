//! Shared analysis-tab controller bundle (MDF4 / Live / DBC / Catalog / About).

use super::analyze::AnalyzeController;
use super::catalog::CatalogController;
use super::dbc::DbcController;
use super::live::LiveController;
use super::mdf4::Mdf4Controller;
use crate::state::AppState;
use crate::SigmaDiagnostics;
use slint::ComponentHandle;
use std::rc::Rc;
use std::sync::Arc;

/// Controllers for the analysis tabs shared by CAN Viewer and Mechanic.
pub struct AnalysisControllers {
    pub mdf4: Arc<Mdf4Controller>,
    pub live: Rc<LiveController>,
    pub dbc: Arc<DbcController>,
    pub catalog: Arc<CatalogController>,
    pub analyze: Arc<AnalyzeController>,
}

impl AnalysisControllers {
    /// Create controllers and cross-link MDF4 ↔ DBC ↔ Catalog.
    pub fn new(state: Arc<AppState>, ui: &SigmaDiagnostics) -> Self {
        let mdf4 = Arc::new(Mdf4Controller::new(state.clone(), ui.as_weak()));
        let live = Rc::new(LiveController::new(state.clone(), ui.as_weak()));
        let dbc = Arc::new(DbcController::new(state.clone(), ui.as_weak()));
        mdf4.set_dbc_editor(Arc::downgrade(&dbc));
        dbc.set_file_master(Arc::downgrade(&mdf4));
        let catalog = Arc::new(CatalogController::new(
            state.clone(),
            ui.as_weak(),
            Arc::downgrade(&mdf4),
            Arc::downgrade(&dbc),
        ));
        let analyze = Arc::new(AnalyzeController::new(state, ui.as_weak()));
        Self {
            mdf4,
            live,
            dbc,
            catalog,
            analyze,
        }
    }

    /// Wire Slint callbacks for all analysis tabs.
    pub fn wire(&self, ui: &SigmaDiagnostics) {
        Mdf4Controller::wire(self.mdf4.clone(), ui);
        LiveController::wire(self.live.clone(), ui);
        DbcController::wire(self.dbc.clone(), ui);
        CatalogController::wire(self.catalog.clone(), ui);
        AnalyzeController::wire(self.analyze.clone(), ui);
        crate::about::populate(ui);
    }

    /// Load session / catalog DBC and optional MDF4.
    pub fn load_initial_files(&self) {
        self.mdf4.load_initial_files();
    }
}

/// Create, wire, and initialize analysis tabs on a [`SigmaDiagnostics`] root.
pub fn wire_analysis_tabs(state: Arc<AppState>, ui: &SigmaDiagnostics) -> AnalysisControllers {
    let controllers = AnalysisControllers::new(state, ui);
    controllers.wire(ui);
    controllers.load_initial_files();
    controllers
}
