//! Vehicle link transport (shop PC ↔ bike).

/// How Mechanic reaches the vehicle bus or telemetry stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleTransport {
    /// USB-CAN adapter on a Linux SocketCAN interface.
    SocketCan,
    /// Wingman NDJSON telemetry relay over TCP/WiFi.
    WiFi,
}

impl Default for VehicleTransport {
    fn default() -> Self {
        Self::SocketCan
    }
}

impl VehicleTransport {
    pub fn label(self) -> &'static str {
        match self {
            Self::SocketCan => "SocketCAN",
            Self::WiFi => "WiFi",
        }
    }
}
