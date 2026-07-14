//! Session configuration for Sigma Racer Mechanic.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionConfig {
    pub dbc_path: Option<String>,
    pub mdf4_path: Option<String>,
    pub can_interface: Option<String>,
    pub can_bitrate: Option<u32>,
    #[serde(default)]
    pub setup_complete: bool,
}

impl SessionConfig {
    /// Mechanic's per-user config directory.
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("sigma-racer-mechanic"))
    }

    /// Path of the persisted config file.
    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join("session.json"))
    }

    /// Share DBC cache with diagnostics desktop tools.
    pub fn dbc_cache_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("sigma-diagnostics").join("dbc-cache"))
    }

    /// Load the persisted config, falling back to defaults.
    pub fn load() -> Self {
        let mut cfg: SessionConfig = Self::config_path()
            .and_then(|path| fs::read_to_string(&path).ok())
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default();
        if let Some(ref p) = cfg.dbc_path {
            if !PathBuf::from(p).is_file() {
                cfg.dbc_path = None;
            }
        }
        if let Some(ref p) = cfg.mdf4_path {
            if !PathBuf::from(p).is_file() {
                cfg.mdf4_path = None;
            }
        }
        cfg
    }

    /// Persist the config to disk.
    pub fn save(&self) -> Result<(), String> {
        let dir = Self::config_dir().ok_or("No config dir")?;
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let path = Self::config_path().ok_or("No config path")?;
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())
    }

    /// Set and persist the preferred CAN interface.
    pub fn set_can_interface(&mut self, iface: Option<String>) -> Result<(), String> {
        self.can_interface = iface;
        self.save()
    }
}
