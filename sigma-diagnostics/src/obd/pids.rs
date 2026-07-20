//! SAE J1979 Service `$01` standard PIDs.
//!
//! Only PIDs with a fully standardized scaling are listed — those values are
//! authoritative ground truth for correlation. `resolution` is the physical
//! quantization step of the encoding, used as the exactness bound.

/// Decoder metadata for one standard PID.
pub struct PidDef {
    pub pid: u8,
    pub name: &'static str,
    pub unit: &'static str,
    /// Number of data bytes after the PID echo.
    pub data_bytes: usize,
    /// Physical quantization step (value of one raw LSB).
    pub resolution: f64,
    decode: fn(&[u8]) -> f64,
}

impl PidDef {
    pub fn decode(&self, data: &[u8]) -> Option<f64> {
        if data.len() < self.data_bytes {
            return None;
        }
        Some((self.decode)(&data[..self.data_bytes]))
    }
}

fn a(d: &[u8]) -> f64 {
    f64::from(d[0])
}
fn ab(d: &[u8]) -> f64 {
    f64::from(u16::from_be_bytes([d[0], d[1]]))
}
fn percent(d: &[u8]) -> f64 {
    f64::from(d[0]) * 100.0 / 255.0
}
fn temp_minus40(d: &[u8]) -> f64 {
    f64::from(d[0]) - 40.0
}
fn fuel_trim(d: &[u8]) -> f64 {
    f64::from(d[0]) / 1.28 - 100.0
}

/// The supported-PID bitmask PIDs (0x00, 0x20, 0x40, 0x60).
pub const SUPPORTED_BITMASK_PIDS: [u8; 4] = [0x00, 0x20, 0x40, 0x60];

pub const STANDARD_PIDS: &[PidDef] = &[
    PidDef { pid: 0x04, name: "EngineLoad", unit: "%", data_bytes: 1, resolution: 100.0 / 255.0, decode: percent },
    PidDef { pid: 0x05, name: "CoolantTemp", unit: "degC", data_bytes: 1, resolution: 1.0, decode: temp_minus40 },
    PidDef { pid: 0x06, name: "ShortFuelTrim1", unit: "%", data_bytes: 1, resolution: 1.0 / 1.28, decode: fuel_trim },
    PidDef { pid: 0x07, name: "LongFuelTrim1", unit: "%", data_bytes: 1, resolution: 1.0 / 1.28, decode: fuel_trim },
    PidDef { pid: 0x08, name: "ShortFuelTrim2", unit: "%", data_bytes: 1, resolution: 1.0 / 1.28, decode: fuel_trim },
    PidDef { pid: 0x09, name: "LongFuelTrim2", unit: "%", data_bytes: 1, resolution: 1.0 / 1.28, decode: fuel_trim },
    PidDef { pid: 0x0A, name: "FuelPressure", unit: "kPa", data_bytes: 1, resolution: 3.0, decode: |d| 3.0 * f64::from(d[0]) },
    PidDef { pid: 0x0B, name: "IntakeMap", unit: "kPa", data_bytes: 1, resolution: 1.0, decode: a },
    PidDef { pid: 0x0C, name: "EngineRpm", unit: "rpm", data_bytes: 2, resolution: 0.25, decode: |d| ab(d) / 4.0 },
    PidDef { pid: 0x0D, name: "VehicleSpeed", unit: "km/h", data_bytes: 1, resolution: 1.0, decode: a },
    PidDef { pid: 0x0E, name: "TimingAdvance", unit: "deg", data_bytes: 1, resolution: 0.5, decode: |d| f64::from(d[0]) / 2.0 - 64.0 },
    PidDef { pid: 0x0F, name: "IntakeAirTemp", unit: "degC", data_bytes: 1, resolution: 1.0, decode: temp_minus40 },
    PidDef { pid: 0x10, name: "MafRate", unit: "g/s", data_bytes: 2, resolution: 0.01, decode: |d| ab(d) / 100.0 },
    PidDef { pid: 0x11, name: "ThrottlePos", unit: "%", data_bytes: 1, resolution: 100.0 / 255.0, decode: percent },
    PidDef { pid: 0x1F, name: "RunTime", unit: "s", data_bytes: 2, resolution: 1.0, decode: ab },
    PidDef { pid: 0x21, name: "DistanceWithMil", unit: "km", data_bytes: 2, resolution: 1.0, decode: ab },
    PidDef { pid: 0x2F, name: "FuelLevel", unit: "%", data_bytes: 1, resolution: 100.0 / 255.0, decode: percent },
    PidDef { pid: 0x33, name: "BarometricPressure", unit: "kPa", data_bytes: 1, resolution: 1.0, decode: a },
    PidDef { pid: 0x42, name: "ControlModuleVoltage", unit: "V", data_bytes: 2, resolution: 0.001, decode: |d| ab(d) / 1000.0 },
    PidDef { pid: 0x45, name: "RelativeThrottle", unit: "%", data_bytes: 1, resolution: 100.0 / 255.0, decode: percent },
    PidDef { pid: 0x46, name: "AmbientAirTemp", unit: "degC", data_bytes: 1, resolution: 1.0, decode: temp_minus40 },
    PidDef { pid: 0x47, name: "AbsoluteThrottleB", unit: "%", data_bytes: 1, resolution: 100.0 / 255.0, decode: percent },
    PidDef { pid: 0x49, name: "AccelPedalD", unit: "%", data_bytes: 1, resolution: 100.0 / 255.0, decode: percent },
    PidDef { pid: 0x4A, name: "AccelPedalE", unit: "%", data_bytes: 1, resolution: 100.0 / 255.0, decode: percent },
    PidDef { pid: 0x5A, name: "RelativeAccelPedal", unit: "%", data_bytes: 1, resolution: 100.0 / 255.0, decode: percent },
    PidDef { pid: 0x5C, name: "EngineOilTemp", unit: "degC", data_bytes: 1, resolution: 1.0, decode: temp_minus40 },
];

pub fn pid_def(pid: u8) -> Option<&'static PidDef> {
    STANDARD_PIDS.iter().find(|d| d.pid == pid)
}

/// Human name of a standard PID, if it has defined scaling.
pub fn pid_def_name(pid: u8) -> Option<&'static str> {
    pid_def(pid).map(|d| d.name)
}

/// Build the Service $01 request payload for a PID (pre-ISO-TP).
pub fn service01_request(pid: u8) -> [u8; 2] {
    [0x01, pid]
}

/// Decode a supported-PID bitmask response (PIDs `base+1 ..= base+32`).
pub fn parse_supported_bitmask(base: u8, data: &[u8]) -> Vec<u8> {
    let mut supported = Vec::new();
    for (byte_index, &byte) in data.iter().take(4).enumerate() {
        for bit in 0..8 {
            if byte & (0x80 >> bit) != 0 {
                supported.push(base + (byte_index as u8) * 8 + bit + 1);
            }
        }
    }
    supported
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpm_scaling() {
        // 0x1AF8 / 4 = 1726 rpm
        let def = pid_def(0x0C).unwrap();
        assert_eq!(def.decode(&[0x1A, 0xF8]), Some(1726.0));
        assert_eq!(def.resolution, 0.25);
    }

    #[test]
    fn coolant_temp_scaling() {
        let def = pid_def(0x05).unwrap();
        assert_eq!(def.decode(&[0x7B]), Some(83.0));
    }

    #[test]
    fn throttle_scaling() {
        let def = pid_def(0x11).unwrap();
        assert_eq!(def.decode(&[0xFF]), Some(100.0));
        assert_eq!(def.decode(&[0x00]), Some(0.0));
    }

    #[test]
    fn short_data_is_none() {
        assert_eq!(pid_def(0x0C).unwrap().decode(&[0x1A]), None);
    }

    #[test]
    fn supported_bitmask() {
        // 0xBE1FA813 is the classic example: PIDs 1,2,3,4,6,7,8,12,...
        let pids = parse_supported_bitmask(0x00, &[0xBE, 0x1F, 0xA8, 0x13]);
        assert!(pids.contains(&0x01));
        assert!(!pids.contains(&0x02));
        assert!(pids.contains(&0x0C));
        assert!(pids.contains(&0x0D));
        assert!(pids.contains(&0x20));
        assert_eq!(pids.len(), 0xBEu8.count_ones() as usize
            + 0x1Fu8.count_ones() as usize
            + 0xA8u8.count_ones() as usize
            + 0x13u8.count_ones() as usize);
    }
}
