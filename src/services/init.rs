//! Initialization helpers.

use crate::config::SessionConfig;
use crate::state::AppState;
use serde::Serialize;
use std::path::Path;

/// Initial files response.
#[derive(Serialize)]
pub struct InitialFilesResponse {
    pub dbc_path: Option<String>,
    pub mdf4_path: Option<String>,
}

fn existing_file(path: Option<String>) -> Option<String> {
    path.filter(|p| Path::new(p).is_file())
}

fn config_sample(name: &str) -> Option<String> {
    SessionConfig::config_dir()
        .map(|d| d.join(name))
        .filter(|p| p.is_file())
        .map(|p| p.to_string_lossy().into_owned())
}

/// Get initial files from CLI args or saved session.
///
/// Paths that no longer exist on disk are skipped. Falls back to the sample
/// files in `~/.config/diagnostics` when session paths are stale.
pub fn get_initial_files(state: &AppState) -> InitialFilesResponse {
    let files = state.initial_files.lock();
    let mut session = state.session.lock();

    let dbc_path = existing_file(files.dbc_path.clone())
        .or_else(|| existing_file(session.dbc_path.clone()))
        .or_else(|| config_sample("sample.dbc"));

    let mdf4_path = existing_file(files.mdf4_path.clone())
        .or_else(|| existing_file(session.mdf4_path.clone()))
        .or_else(|| config_sample("sample.mf4"));

    // Prune stale session paths so the next run does not retry missing files.
    let mut session_changed = false;
    if session
        .dbc_path
        .as_ref()
        .is_some_and(|p| !Path::new(p).is_file())
    {
        session.dbc_path = dbc_path.clone();
        session_changed = true;
    }
    if session
        .mdf4_path
        .as_ref()
        .is_some_and(|p| !Path::new(p).is_file())
    {
        session.mdf4_path = mdf4_path.clone();
        session_changed = true;
    }
    if session_changed {
        if let Err(e) = session.save() {
            log::warn!("Failed to update session after pruning stale paths: {e}");
        }
    }

    InitialFilesResponse {
        dbc_path,
        mdf4_path,
    }
}
