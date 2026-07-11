//! Fetch Sigma Racer DBC schemas from the sigma-updates catalog.

use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct DbcCatalogFile {
    pub filename: String,
    pub name: String,
    pub size_bytes: u64,
    pub download_path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbcCatalogResponse {
    pub files: Vec<DbcCatalogFile>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    #[serde(default)]
    pub query: String,
}

#[derive(Debug, Clone)]
pub struct UpdatesConfig {
    pub base_url: String,
}

impl UpdatesConfig {
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("SIGMA_UPDATES_URL")
                .unwrap_or_else(|_| "http://updates.sigma.localtest.me:30080".into())
                .trim_end_matches('/')
                .to_owned(),
        }
    }

    pub fn list_dbc_url(&self) -> String {
        format!("{}/v1/dbc?page=1&per_page=500", self.base_url)
    }

    pub fn download_url(&self, download_path: &str) -> String {
        if download_path.starts_with("http://") || download_path.starts_with("https://") {
            download_path.to_owned()
        } else {
            format!("{}{}", self.base_url, download_path)
        }
    }
}

pub fn fetch_dbc_catalog(cfg: &UpdatesConfig) -> Result<Vec<DbcCatalogFile>, String> {
    let body = ureq::get(&cfg.list_dbc_url())
        .timeout(Duration::from_secs(10))
        .call()
        .map_err(|e| format!("Catalog fetch failed: {e}"))?
        .into_string()
        .map_err(|e| format!("Catalog response: {e}"))?;
    let response: DbcCatalogResponse =
        serde_json::from_str(&body).map_err(|e| format!("Catalog JSON: {e}"))?;
    Ok(response.files)
}

pub fn download_dbc(cfg: &UpdatesConfig, file: &DbcCatalogFile) -> Result<String, String> {
    let url = cfg.download_url(&file.download_path);
    let body = ureq::get(&url)
        .timeout(Duration::from_secs(30))
        .call()
        .map_err(|e| format!("Download failed: {e}"))?
        .into_string()
        .map_err(|e| format!("Download response: {e}"))?;
    Ok(body)
}
