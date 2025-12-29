//! DBC file loading and management commands.

use crate::decode::{DecodeResult, decode_frame};
use crate::dto::{CanFrameDto, DecodeResponse};
use crate::state::AppState;
use dbc_rs::Dbc;
use serde::Serialize;
use std::sync::Arc;
use tauri::State;

/// Signal definition from DBC.
#[derive(Debug, Clone, Serialize)]
pub struct SignalInfo {
    pub name: String,
    pub start_bit: u32,
    pub length: u32,
    pub byte_order: String,
    pub is_signed: bool,
    pub factor: f64,
    pub offset: f64,
    pub min: f64,
    pub max: f64,
    pub unit: String,
    /// Comment from CM_ SG_ entry
    pub comment: Option<String>,
}

/// Message definition from DBC.
#[derive(Debug, Clone, Serialize)]
pub struct MessageInfo {
    pub id: u32,
    pub name: String,
    pub dlc: u8,
    pub sender: String,
    pub signals: Vec<SignalInfo>,
    /// Comment from CM_ BO_ entry
    pub comment: Option<String>,
}

/// Full DBC structure for display.
#[derive(Debug, Clone, Serialize)]
pub struct DbcInfo {
    pub messages: Vec<MessageInfo>,
}

/// Load and parse a DBC file.
/// Saves the path to session config for persistence.
#[tauri::command]
pub async fn load_dbc(path: String, state: State<'_, Arc<AppState>>) -> Result<String, String> {
    let content =
        std::fs::read_to_string(&path).map_err(|e| format!("Failed to read DBC: {}", e))?;

    let dbc = Dbc::parse(&content).map_err(|e| format!("Failed to parse DBC: {:?}", e))?;
    let msg_count = dbc.messages().len();

    state.set_dbc(dbc);
    *state.dbc_path.lock() = Some(path.clone());

    // Save to session config for persistence
    if let Err(e) = state.session.lock().set_dbc_path(Some(path.clone())) {
        log::warn!("Failed to save session: {}", e);
    }

    Ok(format!("Loaded {} messages", msg_count))
}

/// Clear the loaded DBC data.
/// Removes from session config.
#[tauri::command]
pub async fn clear_dbc(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.clear_dbc();
    *state.dbc_path.lock() = None;

    // Clear from session config
    if let Err(e) = state.session.lock().set_dbc_path(None) {
        log::warn!("Failed to save session: {}", e);
    }

    Ok(())
}

/// Get the path to the currently loaded DBC file.
#[tauri::command]
pub async fn get_dbc_path(state: State<'_, Arc<AppState>>) -> Result<Option<String>, String> {
    Ok(state.dbc_path.lock().clone())
}

/// Decode a single CAN frame using the loaded DBC.
#[tauri::command]
pub async fn decode_single_frame(
    frame: CanFrameDto,
    state: State<'_, Arc<AppState>>,
) -> Result<DecodeResponse, String> {
    let dbc_guard = state.dbc.lock();
    let Some(ref dbc) = *dbc_guard else {
        return Ok(DecodeResponse {
            signals: Vec::new(),
            errors: Vec::new(),
        });
    };

    match decode_frame(&frame, dbc) {
        DecodeResult::Signals(signals) => Ok(DecodeResponse {
            signals,
            errors: Vec::new(),
        }),
        DecodeResult::Error(err) => Ok(DecodeResponse {
            signals: Vec::new(),
            errors: vec![err],
        }),
    }
}

/// Decode multiple CAN frames using the loaded DBC.
#[tauri::command]
pub async fn decode_frames(
    frames: Vec<CanFrameDto>,
    state: State<'_, Arc<AppState>>,
) -> Result<DecodeResponse, String> {
    let dbc_guard = state.dbc.lock();
    let Some(ref dbc) = *dbc_guard else {
        return Ok(DecodeResponse {
            signals: Vec::new(),
            errors: Vec::new(),
        });
    };

    let mut signals = Vec::new();
    let mut errors = Vec::new();

    for frame in &frames {
        match decode_frame(frame, dbc) {
            DecodeResult::Signals(sigs) => signals.extend(sigs),
            DecodeResult::Error(err) => errors.push(err),
        }
    }

    Ok(DecodeResponse { signals, errors })
}

/// Save DBC content to a file.
/// Validates the content by parsing it before writing.
#[tauri::command]
pub async fn save_dbc_content(
    path: String,
    content: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    // Validate by parsing the content BEFORE writing to file
    let dbc = Dbc::parse(&content).map_err(|e| format!("Invalid DBC content: {:?}", e))?;

    // Content is valid, now write to file
    std::fs::write(&path, &content).map_err(|e| format!("Failed to write DBC: {}", e))?;

    // Update state with the parsed DBC
    state.set_dbc(dbc);
    *state.dbc_path.lock() = Some(path.clone());

    // Save to session config
    if let Err(e) = state.session.lock().set_dbc_path(Some(path.clone())) {
        log::warn!("Failed to save session: {}", e);
    }

    Ok(())
}

/// Update the in-memory DBC from content string (for live editing).
/// Does NOT save to file or update the file path.
#[tauri::command]
pub async fn update_dbc_content(
    content: String,
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let dbc = Dbc::parse(&content).map_err(|e| format!("Failed to parse DBC: {:?}", e))?;
    let msg_count = dbc.messages().len();

    state.set_dbc(dbc);
    Ok(format!("Updated DBC with {} messages", msg_count))
}

/// Get information about the loaded DBC.
#[tauri::command]
pub async fn get_dbc_info(state: State<'_, Arc<AppState>>) -> Result<Option<DbcInfo>, String> {
    let dbc_guard = state.dbc.lock();
    let Some(ref dbc) = *dbc_guard else {
        return Ok(None);
    };

    let messages: Vec<MessageInfo> = dbc
        .messages()
        .iter()
        .map(|msg| {
            let signals: Vec<SignalInfo> = msg
                .signals()
                .iter()
                .map(|sig| {
                    let byte_order = match sig.byte_order() {
                        dbc_rs::ByteOrder::BigEndian => "big_endian",
                        dbc_rs::ByteOrder::LittleEndian => "little_endian",
                    };
                    SignalInfo {
                        name: sig.name().to_string(),
                        start_bit: sig.start_bit() as u32,
                        length: sig.length() as u32,
                        byte_order: byte_order.to_string(),
                        is_signed: !sig.is_unsigned(),
                        factor: sig.factor(),
                        offset: sig.offset(),
                        min: sig.min(),
                        max: sig.max(),
                        unit: sig.unit().unwrap_or("").to_string(),
                        comment: sig.comment().map(|s| s.to_string()),
                    }
                })
                .collect();

            MessageInfo {
                id: msg.id(),
                name: msg.name().to_string(),
                dlc: msg.dlc(),
                sender: msg.sender().to_string(),
                signals,
                comment: msg.comment().map(|s| s.to_string()),
            }
        })
        .collect();

    Ok(Some(DbcInfo { messages }))
}
