use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Heart Rate Service UUID (standard Bluetooth SIG)
pub const HEART_RATE_SERVICE_UUID: Uuid = uuid::uuid!("0000180d-0000-1000-8000-00805f9b34fb");

/// Heart Rate Measurement Characteristic UUID (standard Bluetooth SIG)
pub const HEART_RATE_MEASUREMENT_UUID: Uuid = uuid::uuid!("00002a37-0000-1000-8000-00805f9b34fb");

/// Device information for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Unique device identifier
    pub id: String,
    /// Device name (may be "Unknown" if not advertised)
    pub name: String,
    /// Signal strength in dBm
    pub rssi: i16,
    /// Whether device supports Heart Rate Service
    pub supports_heart_rate: bool,
}

/// Connection state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error(String),
}

/// Heart rate measurement data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRateMeasurement {
    /// Heart rate in BPM
    pub bpm: u16,
    /// Sensor contact status (if available)
    #[serde(default)]
    pub sensor_contact: Option<bool>,
    /// Energy expended in kJ (if available)
    #[serde(default)]
    pub energy_expended: Option<u16>,
    /// RR intervals in seconds (if available)
    #[serde(default)]
    pub rr_intervals: Vec<f64>,
    /// Timestamp of measurement
    pub timestamp: i64,
}