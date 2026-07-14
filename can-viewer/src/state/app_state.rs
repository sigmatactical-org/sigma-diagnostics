use std::ops::Deref;

use parking_lot::Mutex;
use sigma_diagnostics::DiagnosticsState;

use crate::config::SessionConfig;

use super::InitialFiles;

/// Desktop app state: shared diagnostics domain + session persistence.
pub struct AppState {
    /// Reusable diagnostics domain (DBC, capture).
    pub diag: DiagnosticsState,

    /// Initial files from command line.
    pub initial_files: Mutex<InitialFiles>,

    /// Session configuration for persistence.
    pub session: Mutex<SessionConfig>,
}

impl AppState {
    /// State seeded with command-line files to open at startup.
    pub fn with_initial_files(initial_files: InitialFiles) -> Self {
        let session = SessionConfig::load();
        Self {
            diag: DiagnosticsState::new(),
            initial_files: Mutex::new(initial_files),
            session: Mutex::new(session),
        }
    }

    /// Set the DBC database (delegates to [`DiagnosticsState`]).
    pub fn set_dbc(&self, dbc: dbc_rs::Dbc) {
        self.diag.set_dbc(dbc);
    }

    /// Clear the DBC database.
    pub fn clear_dbc(&self) {
        self.diag.clear_dbc();
    }
}

impl Deref for AppState {
    type Target = DiagnosticsState;

    fn deref(&self) -> &Self::Target {
        &self.diag
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::with_initial_files(InitialFiles::default())
    }
}
