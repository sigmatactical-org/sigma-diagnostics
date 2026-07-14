//! Sigma Racer Mechanic — shop tool over SocketCAN.

#![forbid(unsafe_code)]

use can_viewer::InitialFiles;
use clap::Parser;
use sigma_racer_mechanic::app;
use sigma_racer_mechanic::AppState;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about = "Sigma Racer Mechanic")]
struct Args {
    #[arg(short, long)]
    dbc: Option<String>,

    #[arg(short, long)]
    mdf4: Option<String>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let state = Arc::new(AppState::new(InitialFiles {
        dbc_path: args.dbc,
        mdf4_path: args.mdf4,
    }));
    app::run(state).expect("error while running Sigma Racer Mechanic");
}
