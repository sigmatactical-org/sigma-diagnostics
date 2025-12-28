//! CAN Viewer - A Tauri application for viewing CAN bus data.
//!
//! Supports:
//! - Live capture from SocketCAN interfaces (Linux)
//! - Loading MDF4 measurement files
//! - DBC-based signal decoding

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use can_viewer::{AppState, InitialFiles, base_commands};
use clap::Parser;
use std::sync::Arc;
use tauri::{Manager, Runtime};

/// CAN Data Viewer with MDF4 and SocketCAN support.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// DBC file to load on startup
    #[arg(short, long)]
    dbc: Option<String>,

    /// MDF4 file to load on startup
    #[arg(short, long)]
    mdf4: Option<String>,
}

fn main() {
    let app_state = create_app_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(base_commands!())
        // Pro: add .setup(pro::setup) here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Create the application state from CLI args.
fn create_app_state() -> Arc<AppState> {
    let args = Args::parse();
    let initial_files = InitialFiles {
        dbc_path: args.dbc,
        mdf4_path: args.mdf4,
    };
    Arc::new(AppState::with_initial_files(initial_files))
}

/// Setup hook for app initialization.
/// Pro versions can add their own setup logic.
#[allow(dead_code)]
fn setup<R: Runtime>(app: &tauri::App<R>) -> Result<(), Box<dyn std::error::Error>> {
    let _window = app.get_webview_window("main").unwrap();
    // Pro: Initialize pro features here
    Ok(())
}
