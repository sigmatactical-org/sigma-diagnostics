//! HTTP fetch/download helpers for the sigma-updates DBC catalog.

use std::time::Duration;

use super::{DbcCatalogFile, DbcCatalogResponse, UpdatesConfig};

/// List the DBC files published on the updates service.
pub fn fetch_dbc_catalog(cfg: &UpdatesConfig) -> Result<Vec<DbcCatalogFile>, String> {
    let body = ureq::get(&cfg.list_dbc_url())
        .config()
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .call()
        .map_err(|e| format!("Catalog fetch failed: {e}"))?
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("Catalog response: {e}"))?;
    let response: DbcCatalogResponse =
        serde_json::from_str(&body).map_err(|e| format!("Catalog JSON: {e}"))?;
    Ok(response.files)
}

/// Fetch metadata for the latest Sigma Racer DBC from sigma-updates.
pub fn fetch_latest_dbc(cfg: &UpdatesConfig) -> Result<DbcCatalogFile, String> {
    let body = ureq::get(&cfg.latest_dbc_url())
        .config()
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .call()
        .map_err(|e| format!("Latest DBC fetch failed: {e}"))?
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("Latest DBC response: {e}"))?;
    serde_json::from_str(&body).map_err(|e| format!("Latest DBC JSON: {e}"))
}

/// Download one catalog DBC and return its local path.
pub fn download_dbc(cfg: &UpdatesConfig, file: &DbcCatalogFile) -> Result<String, String> {
    let url = cfg.download_url(&file.download_path);
    let body = ureq::get(&url)
        .config()
        .timeout_global(Some(Duration::from_secs(30)))
        .build()
        .call()
        .map_err(|e| format!("Download failed: {e}"))?
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("Download response: {e}"))?;
    Ok(body)
}

/// Fetch the latest Sigma Racer DBC body from updates and return (filename, content).
pub fn fetch_latest_dbc_content(cfg: &UpdatesConfig) -> Result<(String, String), String> {
    let meta = fetch_latest_dbc(cfg)?;
    let content = download_dbc(cfg, &meta)?;
    Ok((meta.filename, content))
}
