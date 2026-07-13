//! Live capture display state.
//!
//! Handles signal decoding and statistics for live display.
//! MDF4 logging happens in the socket thread for lossless capture.
//!
//! [`LiveCaptureState`] is the public entry point; the per-message,
//! per-signal, and per-error bookkeeping structs it owns are private
//! implementation details in their own files.

mod error_entry;
mod message_entry;
mod signal_entry;
mod state;

pub use state::LiveCaptureState;
