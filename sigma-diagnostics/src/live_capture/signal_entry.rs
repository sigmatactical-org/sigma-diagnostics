use std::collections::VecDeque;

/// Signal key: (can_id, signal_index) - avoids string allocation in hot path.
pub(super) type SignalKey = (u32, usize);

/// Internal signal monitor entry with history for sparklines.
pub(super) struct SignalEntry {
    pub(super) signal_name: String,
    pub(super) message_name: String,
    pub(super) value: f64,
    pub(super) unit: String,
    pub(super) last_update: f64,
    // History for sparkline
    pub(super) history: VecDeque<f64>,
    pub(super) min_value: f64,
    pub(super) max_value: f64,
}
