use crate::ble::HeartRateMeasurement;
use std::time::{SystemTime, UNIX_EPOCH};

/// Parse heart rate measurement from BLE notification data
///
/// Format according to Bluetooth Heart Rate Profile:
/// - Byte 0: Flags
///   - Bit 0: Heart Rate Value Format (0 = UINT8, 1 = UINT16)
///   - Bit 1-2: Sensor Contact Status
///   - Bit 3: Energy Expended Present
///   - Bit 4: RR-Interval Present
/// - Byte 1-2: Heart Rate Measurement Value
/// - Optional: Energy Expended (2 bytes)
/// - Optional: RR-Intervals (2 bytes each)
pub fn parse_heart_rate_measurement(data: &[u8]) -> Option<HeartRateMeasurement> {
    if data.is_empty() {
        return None;
    }

    let flags = data[0];
    let is_16_bit = (flags & 0x01) != 0;
    let sensor_contact_supported = (flags & 0x02) != 0;
    let sensor_contact_detected = (flags & 0x04) != 0;
    let energy_expended_present = (flags & 0x08) != 0;
    let rr_interval_present = (flags & 0x10) != 0;

    // Parse heart rate value
    let (bpm, mut offset) = if is_16_bit {
        if data.len() < 3 {
            return None;
        }
        let value = u16::from_le_bytes([data[1], data[2]]);
        (value, 3)
    } else {
        if data.len() < 2 {
            return None;
        }
        (data[1] as u16, 2)
    };

    // Parse sensor contact
    let sensor_contact = if sensor_contact_supported {
        Some(sensor_contact_detected)
    } else {
        None
    };

    // Parse energy expended
    let energy_expended = if energy_expended_present {
        if data.len() >= offset + 2 {
            let value = u16::from_le_bytes([data[offset], data[offset + 1]]);
            offset += 2;
            Some(value)
        } else {
            None
        }
    } else {
        None
    };

    // Parse RR intervals (in 1/1024 seconds, convert to seconds)
    let mut rr_intervals = Vec::new();
    if rr_interval_present {
        while offset + 1 < data.len() {
            let rr_raw = u16::from_le_bytes([data[offset], data[offset + 1]]);
            // Convert from 1/1024 seconds to seconds
            rr_intervals.push(rr_raw as f64 / 1024.0);
            offset += 2;
        }
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);

    Some(HeartRateMeasurement {
        bpm,
        sensor_contact,
        energy_expended,
        rr_intervals,
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_8_bit_heart_rate() {
        // Flags: 0x00 (8-bit HR, no sensor contact, no energy, no RR)
        // HR: 72 BPM
        let data = [0x00, 72];
        let result = parse_heart_rate_measurement(&data).unwrap();
        assert_eq!(result.bpm, 72);
        assert!(result.sensor_contact.is_none());
        assert!(result.energy_expended.is_none());
        assert!(result.rr_intervals.is_empty());
    }

    #[test]
    fn test_parse_16_bit_heart_rate() {
        // Flags: 0x01 (16-bit HR)
        // HR: 150 BPM (0x0096 LE)
        let data = [0x01, 0x96, 0x00];
        let result = parse_heart_rate_measurement(&data).unwrap();
        assert_eq!(result.bpm, 150);
    }

    #[test]
    fn test_parse_with_rr_intervals() {
        // Flags: 0x10 (RR interval present)
        // HR: 80
        // RR: 1024 (1 second in 1/1024 units = 0x0400 LE)
        let data = [0x10, 80, 0x00, 0x04];
        let result = parse_heart_rate_measurement(&data).unwrap();
        assert_eq!(result.bpm, 80);
        assert_eq!(result.rr_intervals.len(), 1);
        assert!((result.rr_intervals[0] - 1.0).abs() < 0.01);
    }
}