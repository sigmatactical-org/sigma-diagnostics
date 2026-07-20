//! canre — definitive CAN signal reverse-engineering CLI.
//!
//! Batch analysis over MDF4 recordings: structural surveys, differential
//! compares, and DBC validation. Every reported fact is proven over every
//! observed frame and carries its evidence counts.

use clap::{Parser, Subcommand};
use sigma_diagnostics::analysis::{
    check_dbc, correlate, diff_recordings, render_correlation, render_dbc_check, render_diff,
    render_survey, survey_frames, CorrelationOptions, LabeledRecording, ReferenceSeries,
    SurveyOptions,
};
use sigma_diagnostics::obd::{extract_obd_samples, group_series};
use sigma_diagnostics::{load_mdf4, CanFrameDto, DiagnosticsState};

#[derive(Parser)]
#[command(
    name = "canre",
    version,
    about = "Definitive CAN signal reverse-engineering over MDF4 logs (no guessing: every fact is proven over every observed frame)"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Structural survey: timing, bit activity, proven counters/checksums/mux
    Survey {
        /// MDF4 log file
        log: String,
        /// Restrict to CAN ID(s), hex (e.g. 0x156). Repeatable.
        #[arg(long, value_parser = parse_can_id)]
        id: Vec<u32>,
        /// Emit the full report as JSON
        #[arg(long)]
        json: bool,
    },
    /// Differential compare of two or more recordings (file name = label)
    Diff {
        /// MDF4 log files, at least two
        #[arg(required = true, num_args = 2..)]
        logs: Vec<String>,
        #[arg(long)]
        json: bool,
    },
    /// Validate a DBC against a recording
    CheckDbc {
        /// MDF4 log file
        log: String,
        /// DBC file to validate
        #[arg(long)]
        dbc: String,
        #[arg(long)]
        json: bool,
    },
    /// Correlate OBD ground truth (extracted from the same log) against
    /// broadcast fields, reporting only exact affine matches
    Correlate {
        /// MDF4 log recorded during an OBD poll session
        log: String,
        /// Alignment window in milliseconds
        #[arg(long, default_value_t = 200)]
        window_ms: u64,
        /// OBD response CAN ID (hex), default 0x7E8
        #[arg(long, value_parser = parse_can_id)]
        response_id: Option<u32>,
        #[arg(long)]
        json: bool,
    },
    /// Live OBD interaction over SocketCAN (Linux only)
    #[command(subcommand)]
    Obd(ObdCommand),
}

#[derive(Subcommand)]
enum ObdCommand {
    /// Discover which Service $01 PIDs the ECU supports (definitive: from its
    /// own supported-PID bitmasks)
    Scan {
        /// CAN interface, e.g. can0
        #[arg(long)]
        iface: String,
    },
    /// Record broadcast + diagnostics into one MDF4 while polling the ECU.
    /// Analyze the result later with `canre survey`/`correlate`.
    Record {
        #[arg(long)]
        iface: String,
        /// Output MDF4 path
        #[arg(long)]
        out: String,
        /// PIDs to poll, comma-separated hex (e.g. 0C,0D,05,11).
        /// Omit to poll every supported PID discovered by a scan.
        #[arg(long, value_delimiter = ',', value_parser = parse_pid)]
        pids: Vec<u8>,
        /// Seconds to record
        #[arg(long, default_value_t = 30)]
        seconds: u64,
        /// Milliseconds between successive PID requests
        #[arg(long, default_value_t = 50)]
        period_ms: u64,
    },
}

fn parse_pid(s: &str) -> Result<u8, String> {
    let t = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    u8::from_str_radix(t, 16).map_err(|e| format!("invalid hex PID {s:?}: {e}"))
}

fn parse_can_id(s: &str) -> Result<u32, String> {
    let trimmed = s.trim_start_matches("0x").trim_start_matches("0X");
    u32::from_str_radix(trimmed, 16).map_err(|e| format!("invalid hex CAN ID {s:?}: {e}"))
}

fn load_frames(path: &str) -> Result<Vec<CanFrameDto>, String> {
    let state = DiagnosticsState::new();
    let (frames, _signals) = load_mdf4(path, &state)?;
    if frames.is_empty() {
        return Err(format!("{path}: no CAN frames found"));
    }
    Ok(frames)
}

fn label_for(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
}

fn run() -> Result<(), String> {
    match Cli::parse().command {
        Command::Survey { log, id, json } => {
            let frames = load_frames(&log)?;
            let options = SurveyOptions {
                id_filter: if id.is_empty() { None } else { Some(id) },
                ..SurveyOptions::default()
            };
            let report = survey_frames(&frames, &options);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
                );
            } else {
                print!("{}", render_survey(&report));
            }
        }
        Command::Diff { logs, json } => {
            let loaded: Vec<(String, Vec<CanFrameDto>)> = logs
                .iter()
                .map(|path| Ok((label_for(path), load_frames(path)?)))
                .collect::<Result<_, String>>()?;
            let recordings: Vec<LabeledRecording> = loaded
                .iter()
                .map(|(label, frames)| LabeledRecording {
                    label: label.clone(),
                    frames,
                })
                .collect();
            let report = diff_recordings(&recordings);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
                );
            } else {
                print!("{}", render_diff(&report));
            }
        }
        Command::CheckDbc { log, dbc, json } => {
            let frames = load_frames(&log)?;
            let content = std::fs::read_to_string(&dbc).map_err(|e| format!("{dbc}: {e}"))?;
            let parsed = dbc_rs_parse(&content)?;
            let report = check_dbc(&frames, &parsed);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
                );
            } else {
                print!("{}", render_dbc_check(&report));
            }
        }
        Command::Correlate {
            log,
            window_ms,
            response_id,
            json,
        } => {
            let frames = load_frames(&log)?;
            let samples = extract_obd_samples(&frames, response_id);
            if samples.is_empty() {
                return Err(
                    "no OBD Service $01 responses found in log — was it recorded during a poll session?"
                        .to_string(),
                );
            }
            let references: Vec<ReferenceSeries> = group_series(&samples)
                .into_iter()
                .map(|s| ReferenceSeries {
                    name: s.name,
                    unit: s.unit,
                    resolution: s.resolution,
                    samples: s.samples,
                })
                .collect();
            let options = CorrelationOptions {
                window_s: window_ms as f64 / 1000.0,
                ..CorrelationOptions::default()
            };
            let report = correlate(&frames, &references, &options);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
                );
            } else {
                print!("{}", render_correlation(&report));
            }
        }
        Command::Obd(obd) => run_obd(obd)?,
    }
    Ok(())
}

fn run_obd(command: ObdCommand) -> Result<(), String> {
    use sigma_diagnostics::obd::{discover_supported_pids, pid_def_name, poll_pids};
    use sigma_diagnostics::{start_capture, stop_capture};
    use std::time::Duration;

    match command {
        ObdCommand::Scan { iface } => {
            let supported = discover_supported_pids(&iface)?;
            if supported.pids.is_empty() {
                println!("No supported PIDs reported (no ECU response on {iface}?).");
                return Ok(());
            }
            println!("Supported Service $01 PIDs on {iface}:");
            for pid in &supported.pids {
                match pid_def_name(*pid) {
                    Some(name) => println!("  0x{pid:02X}  {name}"),
                    None => println!("  0x{pid:02X}  (no standard scaling — not usable as ground truth)"),
                }
            }
        }
        ObdCommand::Record {
            iface,
            out,
            pids,
            seconds,
            period_ms,
        } => {
            let pids = if pids.is_empty() {
                let supported = discover_supported_pids(&iface)?;
                let usable: Vec<u8> = supported
                    .pids
                    .into_iter()
                    .filter(|p| pid_def_name(*p).is_some())
                    .collect();
                if usable.is_empty() {
                    return Err("no usable PIDs discovered to poll".to_string());
                }
                println!("Polling {} discovered PIDs", usable.len());
                usable
            } else {
                pids
            };

            let state = DiagnosticsState::new();
            start_capture(&iface, &out, false, None, &state)?;
            println!("Recording {iface} → {out} for {seconds}s while polling…");
            let sent = poll_pids(
                &iface,
                &pids,
                Duration::from_millis(period_ms),
                Duration::from_secs(seconds),
            );
            let summary = stop_capture(&state)?;
            let sent = sent?;
            println!("Sent {sent} requests. {summary}");
            println!("Analyze with:\n  canre survey {out}\n  canre correlate {out}");
        }
    }
    Ok(())
}

fn dbc_rs_parse(content: &str) -> Result<dbc_rs::Dbc, String> {
    dbc_rs::Dbc::parse(content).map_err(|e| format!("DBC parse error: {e}"))
}

fn main() {
    if let Err(message) = run() {
        eprintln!("error: {message}");
        std::process::exit(1);
    }
}
