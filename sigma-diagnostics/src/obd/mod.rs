//! OBD-II over CAN (ISO 15765-4): minimal ISO-TP, Service `$01` PID table,
//! and offline ground-truth extraction from recorded logs.

pub mod extract;
pub mod isotp;
pub mod pids;
pub mod poll;

pub use extract::{extract_obd_samples, group_series, GroundTruthSample, GroundTruthSeries};
pub use isotp::{
    encode_single_frame, flow_control_cts, IsotpMessage, IsotpReassembler, OBD_ECU_REQUEST_ID,
    OBD_ECU_RESPONSE_ID, OBD_FUNCTIONAL_ID,
};
pub use pids::{
    parse_supported_bitmask, pid_def, pid_def_name, service01_request, PidDef, STANDARD_PIDS,
    SUPPORTED_BITMASK_PIDS,
};
pub use poll::{discover_supported_pids, poll_pids, request_payload, SupportedPids};
