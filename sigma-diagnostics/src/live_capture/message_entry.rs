/// Internal message monitor entry.
pub(super) struct MessageEntry {
    pub(super) can_id: u32,
    pub(super) message_name: String,
    pub(super) data: Vec<u8>,
    pub(super) dlc: u8,
    pub(super) count: u64,
    pub(super) last_update: f64,
    // For rate calculation
    pub(super) last_count: u64,
    pub(super) last_rate_time: f64,
    pub(super) rate: f64,
}
