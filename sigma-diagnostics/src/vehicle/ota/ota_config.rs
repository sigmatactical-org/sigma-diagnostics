use crate::updates::UpdatesConfig as DbcUpdatesConfig;

/// Shop-side OTA channel configuration (download only; RAUC apply stays
/// on-device).
#[derive(Debug, Clone)]
pub struct OtaConfig {
    pub base_url: String,
    pub channel: String,
    pub current_version: String,
}

impl OtaConfig {
    /// Read the OTA catalog endpoint from env with lab defaults.
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

    /// Endpoint for the channel's latest image release.
    pub fn latest_url(&self) -> String {
        format!("{}/v1/channel/{}/latest", self.base_url, self.channel)
    }

    /// The equivalent DBC-catalog config (same service).
    pub fn as_dbc_updates(&self) -> DbcUpdatesConfig {
        DbcUpdatesConfig {
            base_url: self.base_url.clone(),
        }
    }
}
