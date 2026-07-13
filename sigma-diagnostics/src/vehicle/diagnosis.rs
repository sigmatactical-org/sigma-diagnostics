//! Live diagnosis snapshot from decoded vehicle CAN frames or VSS telemetry.

use crate::dto::{LiveCaptureDisplay, LiveSignalRow};
use sigma_racer_telemetry::VehicleState;

/// One vital for the Mechanic diagnosis panel (VSS-aligned labels).
#[derive(Debug, Clone, Default)]
pub struct VitalSignal {
    pub name: String,
    pub value: String,
    pub unit: String,
}

/// Aggregated diagnosis view for the shop UI.
#[derive(Debug, Clone, Default)]
pub struct DiagnosisSnapshot {
    pub connected: bool,
    pub frame_count: u64,
    pub rpm: String,
    pub coolant_c: String,
    pub oil_c: String,
    pub dtc_count: String,
    pub gear: String,
    pub side_stand: String,
    pub performance_mode: String,
    pub vitals: Vec<VitalSignal>,
    pub status: String,
}

impl DiagnosisSnapshot {
    /// Build a snapshot from the latest live capture display (DBC-decoded).
    pub fn from_live_display(display: &LiveCaptureDisplay, connected: bool) -> Self {
        let mut snap = Self {
            connected,
            frame_count: display.stats.frame_count,
            status: if connected {
                "Receiving".into()
            } else {
                "Disconnected".into()
            },
            ..Self::default()
        };

        for sig in &display.signals {
            apply_signal(&mut snap, sig);
        }

        snap.vitals = display
            .signals
            .iter()
            .take(24)
            .map(|s| VitalSignal {
                name: format!("{}.{}", s.message_name, s.signal_name),
                value: s.value.clone(),
                unit: s.unit.clone(),
            })
            .collect();

        snap
    }

    /// Build a snapshot from Wingman VSS telemetry (WiFi / replay).
    pub fn from_vehicle_state(
        state: &VehicleState,
        connected: bool,
        seq: u64,
        status: &str,
    ) -> Self {
        Self {
            connected,
            frame_count: seq,
            rpm: format!("{:.0} rpm", state.rpm),
            coolant_c: format!("{} °C", state.coolant_c),
            oil_c: format!("{} °C", state.oil_c),
            dtc_count: state.dtc.to_string(),
            gear: state.gear.to_string(),
            side_stand: if state.side_stand {
                "Down".into()
            } else {
                "Up".into()
            },
            performance_mode: state.riding_mode.clone(),
            vitals: vec![
                VitalSignal {
                    name: "Vehicle.Speed".into(),
                    value: format!("{:.0}", state.speed),
                    unit: "km/h".into(),
                },
                VitalSignal {
                    name: "Vehicle.Powertrain.CombustionEngine.ThrottlePosition".into(),
                    value: format!("{:.0}", state.throttle_pct),
                    unit: "%".into(),
                },
                VitalSignal {
                    name: "Vehicle.FuelSystem.Level".into(),
                    value: format!("{:.0}", state.fuel_pct),
                    unit: "%".into(),
                },
                VitalSignal {
                    name: "Vehicle.ElectricalSystem.Battery.Voltage".into(),
                    value: format!("{:.1}", state.battery_v),
                    unit: "V".into(),
                },
                VitalSignal {
                    name: "Vehicle.Acceleration.Lateral".into(),
                    value: format!("{:.1}", state.lean_angle),
                    unit: "°".into(),
                },
                VitalSignal {
                    name: "Vehicle.TraveledDistance".into(),
                    value: format!("{:.1}", state.odometer),
                    unit: "km".into(),
                },
            ],
            status: status.to_string(),
        }
    }

    /// Empty disconnected snapshot with a status message.
    pub fn disconnected(reason: &str) -> Self {
        Self {
            connected: false,
            status: reason.to_string(),
            ..Self::default()
        }
    }
}

fn apply_signal(snap: &mut DiagnosisSnapshot, sig: &LiveSignalRow) {
    let key = sig.signal_name.to_ascii_lowercase();
    let msg = sig.message_name.to_ascii_lowercase();
    let val = if sig.unit.is_empty() {
        sig.value.clone()
    } else {
        format!("{} {}", sig.value, sig.unit)
    };

    if key.contains("rpm")
        || key.contains("enginespeed")
        || (msg.contains("engine") && key.contains("speed"))
    {
        snap.rpm = val;
    } else if key.contains("coolant") {
        snap.coolant_c = val;
    } else if key.contains("oil") && (key.contains("temp") || msg.contains("engine")) {
        snap.oil_c = val;
    } else if key.contains("dtc") {
        snap.dtc_count = val;
    } else if key.contains("gear") {
        snap.gear = val;
    } else if key.contains("sidestand") || key.contains("side_stand") || key.contains("sidestand") {
        snap.side_stand = val;
    } else if key.contains("performance") || (key.contains("mode") && msg.contains("throttle")) {
        snap.performance_mode = val;
    }
}
