//! Shop-side OTA channel catalog (download only; RAUC apply stays on-device).

use serde::Deserialize;
use std::time::Duration;

use crate::updates::UpdatesConfig as DbcUpdatesConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelRelease {
    pub channel: String,
    pub version: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub install: String,
    #[serde(default)]
    pub bundle_url: String,
}

#[derive(Debug, Clone)]
pub struct OtaConfig {
    pub base_url: String,
    pub channel: String,
    pub current_version: String,
}

impl OtaConfig {
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("SIGMA_UPDATES_URL")
                .unwrap_or_else(|_| "http://updates.sigma.localtest.me:30080".into())
                .trim_end_matches('/')
                .to_owned(),
            channel: std::env::var("SIGMA_UPDATES_CHANNEL").unwrap_or_else(|_| "dev".into()),
            current_version: std::env::var("SIGMA_IMAGE_VERSION")
                .unwrap_or_else(|_| "0.0.0".into()),
        }
    }

    pub fn latest_url(&self) -> String {
        format!("{}/v1/channel/{}/latest", self.base_url, self.channel)
    }

    pub fn as_dbc_updates(&self) -> DbcUpdatesConfig {
        DbcUpdatesConfig {
            base_url: self.base_url.clone(),
        }
    }
}

/// Fetch the latest channel release metadata from sigma-updates.
pub fn fetch_channel_latest(cfg: &OtaConfig) -> Result<ChannelRelease, String> {
    let body = ureq::get(&cfg.latest_url())
        .timeout(Duration::from_secs(10))
        .call()
        .map_err(|e| format!("OTA catalog fetch failed: {e}"))?
        .into_string()
        .map_err(|e| format!("OTA catalog response: {e}"))?;
    serde_json::from_str(&body).map_err(|e| format!("OTA catalog JSON: {e}"))
}
