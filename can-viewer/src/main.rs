//! Diagnostics can-viewer — native Slint desktop application.

#![forbid(unsafe_code)]

use can_viewer::app;
use can_viewer::{AppState, InitialFiles};
use clap::Parser;
use std::sync::Arc;

/// MDF4, SocketCAN, and DBC analysis tool.
#[derive(Parser, Debug)]
#[command(author, version, about = "CAN Viewer")]
struct Args {
    /// DBC file to load on startup
    #[arg(short, long)]
    dbc: Option<String>,

    /// MDF4 file to load on startup
    #[arg(short, long)]
    mdf4: Option<String>,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let initial_files = InitialFiles {
        dbc_path: args.dbc,
        mdf4_path: args.mdf4,
    };
    let state = Arc::new(AppState::with_initial_files(initial_files));

    app::run(state).expect("error while running application");
}
