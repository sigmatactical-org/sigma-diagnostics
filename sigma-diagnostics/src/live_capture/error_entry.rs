/// Error entry for tracking bus errors.
pub(super) struct ErrorEntry {
    pub(super) timestamp: f64,
    pub(super) channel: String,
    pub(super) error_type: String,
    pub(super) details: String,
    pub(super) count: u64,
}
