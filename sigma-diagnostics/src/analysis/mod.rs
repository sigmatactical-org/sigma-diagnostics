//! Definitive CAN signal reverse-engineering.
//!
//! Every engine in this module reports only facts proven over every observed
//! sample, with the evidence (frame counts, distinct payloads, transitions)
//! attached — never heuristic guesses. See the individual modules:
//!
//! - [`survey`] — per-ID timing, DLC census, empirical bit activity
//! - [`counter`] — rolling counters proven over all consecutive frames
//! - [`checksum`] — checksum/CRC algorithms verified against every payload
//! - [`mux`] — conditional-layout (multiplexor) facts
//! - [`diff`] — differential compare of labeled recordings
//! - [`dbc_check`] — DBC hypothesis validation against a log
//! - [`bits`] — shared DBC-convention bit walker and raw field extraction

pub mod bits;
pub mod checksum;
pub mod correlate;
pub mod counter;
pub mod dbc_check;
pub mod diff;
pub mod mux;
pub mod report;
pub mod survey;

pub use bits::{extract_raw, occupied_bits, signal_occupied_bits, Endianness, FieldSpec};
pub use checksum::{ChecksumAlgorithm, ChecksumProof};
pub use correlate::{
    correlate, AffineMatch, CorrelationOptions, CorrelationReport, ReferenceSeries, SeriesMatches,
};
pub use counter::CounterProof;
pub use dbc_check::{check_dbc, DbcCheckReport, IdCheck, SignalCheck};
pub use diff::{
    diff_recordings, DiffReport, DifferentialBit, ExclusiveId, IdDiff, LabeledRecording,
};
pub use mux::{MuxFacts, MuxVariant};
pub use report::{render_correlation, render_dbc_check, render_diff, render_survey};
pub use survey::{
    survey_frames, BitActivityCell, BitState, IdSurvey, SurveyOptions, SurveyReport, TimingStats,
};
