/// Connection state of a [`super::VehicleSession`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleSessionStatus {
    Disconnected,
    Connecting,
    Connected,
    Replaying,
    Error,
}

impl VehicleSessionStatus {
    /// Human-readable status name.
    pub fn label(self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting",
            Self::Connected => "Connected",
            Self::Replaying => "Replaying",
            Self::Error => "Error",
        }
    }
}
