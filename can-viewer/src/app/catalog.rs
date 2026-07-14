//! Sigma-updates DBC catalog picker and download flow.

use super::dbc::DbcController;
use super::helpers::{set_status, vec_model};
use super::mdf4::Mdf4Controller;
use crate::config::SessionConfig;
use crate::services::{
    download_dbc, fetch_dbc_catalog, save_dbc_content, DbcCatalogFile, UpdatesConfig,
};
use crate::state::AppState;
use crate::{DbcCatalogRow, SigmaDiagnostics};
use parking_lot::Mutex;
use slint::Weak;
use std::sync::{Arc, Weak as StdWeak};

/// DBC catalog tab: browse and download dictionaries from sigma-updates.
pub struct CatalogController {
    state: Arc<AppState>,
    ui: Weak<SigmaDiagnostics>,
    mdf4: StdWeak<Mdf4Controller>,
    dbc: StdWeak<DbcController>,
    files: Mutex<Vec<DbcCatalogFile>>,
    updates: UpdatesConfig,
}

impl CatalogController {
    /// Controller bound to the shared state and UI handle.
    pub fn new(
        state: Arc<AppState>,
        ui: Weak<SigmaDiagnostics>,
        mdf4: StdWeak<Mdf4Controller>,
        dbc: StdWeak<DbcController>,
    ) -> Self {
        Self {
            state,
            ui,
            mdf4,
            dbc,
            files: Mutex::new(Vec::new()),
            updates: UpdatesConfig::from_env(),
        }
    }

    /// Hook the catalog tab callbacks.
    pub fn wire(self: Arc<Self>, ui: &SigmaDiagnostics) {
        let this = self.clone();
        ui.on_open_dbc_catalog({
            let t = this.clone();
            move || t.open_catalog()
        });
        ui.on_dbc_catalog_refresh({
            let t = this.clone();
            move || t.refresh_catalog()
        });
        ui.on_dbc_catalog_load({
            let t = this.clone();
            move || t.load_selected()
        });
        ui.on_dbc_catalog_close({
            let t = this.clone();
            move || t.close_catalog()
        });
        ui.on_set_dbc_catalog_selected({
            let t = this.clone();
            move |index| t.select_file(index)
        });
    }

    fn with_ui<F: FnOnce(&SigmaDiagnostics)>(&self, f: F) {
        if let Some(ui) = self.ui.upgrade() {
            f(&ui);
        }
    }

    fn open_catalog(&self) {
        self.with_ui(|ui| {
            ui.set_dbc_catalog_open(true);
            ui.set_dbc_catalog_selected(-1);
        });
        self.refresh_catalog();
    }

    fn close_catalog(&self) {
        self.with_ui(|ui| ui.set_dbc_catalog_open(false));
    }

    fn refresh_catalog(&self) {
        self.with_ui(|ui| {
            ui.set_dbc_catalog_busy(true);
            ui.set_dbc_catalog_status(format!("Fetching from {}…", self.updates.base_url).into());
        });

        match fetch_dbc_catalog(&self.updates) {
            Ok(files) => {
                let rows: Vec<DbcCatalogRow> = files
                    .iter()
                    .map(|f| DbcCatalogRow {
                        filename: f.filename.clone().into(),
                        name: f.name.clone().into(),
                        size_text: format_size(f.size_bytes).into(),
                    })
                    .collect();
                *self.files.lock() = files;
                self.with_ui(|ui| {
                    ui.set_dbc_catalog_files(vec_model(&rows));
                    ui.set_dbc_catalog_status(if rows.is_empty() {
                        "No DBC files in catalog".into()
                    } else {
                        format!("{} schema(s) available", rows.len()).into()
                    });
                    ui.set_dbc_catalog_busy(false);
                });
            }
            Err(err) => self.with_ui(|ui| {
                ui.set_dbc_catalog_status(err.into());
                ui.set_dbc_catalog_busy(false);
            }),
        }
    }

    fn select_file(&self, index: i32) {
        self.with_ui(|ui| ui.set_dbc_catalog_selected(index));
    }

    fn load_selected(&self) {
        let index = self
            .ui
            .upgrade()
            .map(|ui| ui.get_dbc_catalog_selected())
            .unwrap_or(-1);
        if index < 0 {
            self.with_ui(|ui| ui.set_dbc_catalog_status("Select a schema first".into()));
            return;
        }

        let file = {
            let files = self.files.lock();
            files.get(index as usize).cloned()
        };
        let Some(file) = file else {
            self.with_ui(|ui| ui.set_dbc_catalog_status("Invalid selection".into()));
            return;
        };

        self.with_ui(|ui| {
            ui.set_dbc_catalog_busy(true);
            ui.set_dbc_catalog_status(format!("Downloading {}…", file.filename).into());
        });

        let cache_path = match cache_path_for(&file.filename) {
            Ok(path) => path,
            Err(err) => {
                self.with_ui(|ui| {
                    ui.set_dbc_catalog_status(err.into());
                    ui.set_dbc_catalog_busy(false);
                });
                return;
            }
        };

        match download_dbc(&self.updates, &file) {
            Ok(content) => match save_dbc_content(&cache_path, &content, &self.state) {
                Ok(()) => {
                    let msg_count = self
                        .state
                        .dbc
                        .lock()
                        .as_ref()
                        .map(|dbc| dbc.messages().len())
                        .unwrap_or(0);
                    let msg = format!("Loaded {} ({msg_count} messages)", file.name);
                    if let Some(mdf4) = self.mdf4.upgrade() {
                        mdf4.on_external_dbc_loaded();
                    }
                    if let Some(dbc) = self.dbc.upgrade() {
                        dbc.on_external_dbc_loaded();
                    }
                    self.with_ui(|ui| {
                        set_status(ui, &msg);
                        ui.set_dbc_catalog_status(msg.into());
                        ui.set_dbc_catalog_busy(false);
                        ui.set_dbc_catalog_open(false);
                    });
                }
                Err(err) => self.with_ui(|ui| {
                    ui.set_dbc_catalog_status(err.into());
                    ui.set_dbc_catalog_busy(false);
                }),
            },
            Err(err) => self.with_ui(|ui| {
                ui.set_dbc_catalog_status(err.into());
                ui.set_dbc_catalog_busy(false);
            }),
        }
    }
}

fn cache_path_for(filename: &str) -> Result<String, String> {
    let cache_dir = SessionConfig::dbc_cache_dir().ok_or("Could not determine cache directory")?;
    std::fs::create_dir_all(&cache_dir).map_err(|e| format!("Failed to create DBC cache: {e}"))?;
    let path = cache_dir.join(filename);
    Ok(path.to_string_lossy().into_owned())
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}
