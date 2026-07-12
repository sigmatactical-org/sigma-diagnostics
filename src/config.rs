//! Session configuration persistence.
//!
//! Saves and loads user session settings like the last used DBC file.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Embedded sample files for first run
const SAMPLE_MF4: &[u8] = include_bytes!("../examples/sample.mf4");
const SAMPLE_DBC: &[u8] = include_bytes!("../examples/sample.dbc");

/// Session configuration that persists across app restarts.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Path to the last loaded DBC file.
    pub dbc_path: Option<String>,
    /// Path to the last loaded MDF4 file.
    pub mdf4_path: Option<String>,
    /// Whether first-run setup has been completed.
    #[serde(default)]
    pub setup_complete: bool,
}

impl SessionConfig {
    /// Get the config directory for the application.
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("can-viewer"))
    }

    /// Get the config file path for the application.
    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join("session.json"))
    }

    /// Cache directory for DBC files fetched from sigma-updates.
    pub fn dbc_cache_dir() -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join("dbc-cache"))
    }

    /// Cache directory for MDF4 files opened via the header button.
    pub fn mdf4_cache_dir() -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join("mdf4-cache"))
    }

    /// Copy `src` into `cache_dir` under its file name and return the cache path.
    pub fn cache_file(src: &std::path::Path, cache_dir: &std::path::Path) -> Result<PathBuf, String> {
        fs::create_dir_all(cache_dir).map_err(|e| format!("Failed to create cache dir: {e}"))?;
        let name = src
            .file_name()
            .ok_or_else(|| "Invalid source path".to_string())?;
        let dest = cache_dir.join(name);
        fs::copy(src, &dest).map_err(|e| format!("Failed to cache file: {e}"))?;
        Ok(dest)
    }

    /// Write UTF-8 content into the DBC cache and return the path.
    pub fn cache_dbc_bytes(filename: &str, content: &str) -> Result<PathBuf, String> {
        let cache_dir = Self::dbc_cache_dir().ok_or("Could not determine DBC cache directory")?;
        fs::create_dir_all(&cache_dir).map_err(|e| format!("Failed to create DBC cache: {e}"))?;
        let path = cache_dir.join(filename);
        fs::write(&path, content).map_err(|e| format!("Failed to write DBC cache: {e}"))?;
        Ok(path)
    }

    /// Load session config from disk, running first-time setup if needed.
    pub fn load() -> Self {
        // Load existing config or create default
        let mut config: Self = Self::config_path()
            .and_then(|path| fs::read_to_string(&path).ok())
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default();

        // First run: extract samples and set as defaults
        if !config.setup_complete {
            if let Some(config_dir) = Self::config_dir() {
                if let Err(e) = fs::create_dir_all(&config_dir) {
                    log::warn!("Failed to create config directory: {}", e);
                }

                let mf4_path = config_dir.join("sample.mf4");
                let dbc_path = config_dir.join("sample.dbc");

                // Extract sample files
                if let Err(e) = fs::write(&mf4_path, SAMPLE_MF4) {
                    log::warn!("Failed to extract sample MF4: {}", e);
                }
                if let Err(e) = fs::write(&dbc_path, SAMPLE_DBC) {
                    log::warn!("Failed to extract sample DBC: {}", e);
                }

                // Set as defaults
                config.mdf4_path = Some(mf4_path.to_string_lossy().into_owned());
                config.dbc_path = Some(dbc_path.to_string_lossy().into_owned());
                config.setup_complete = true;
                if let Err(e) = config.save() {
                    log::warn!("Failed to save initial config: {}", e);
                }
            }
        }

        config
    }

    /// Save session config to disk.
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("Could not determine config directory")?;

        // Create config directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        fs::write(&path, content).map_err(|e| format!("Failed to write config: {}", e))?;

        Ok(())
    }

    /// Update DBC path and save.
    pub fn set_dbc_path(&mut self, path: Option<String>) -> Result<(), String> {
        self.dbc_path = path;
        self.save()
    }

    /// Update MDF4 path and save.
    pub fn set_mdf4_path(&mut self, path: Option<String>) -> Result<(), String> {
        self.mdf4_path = path;
        self.save()
    }
}
