use crate::ble::{ConnectionState, DeviceInfo, HeartRateMeasurement, BleManager};
use crate::storage::{AlertSettings, Database, HeartRateRecord, PeriodStats};
use tauri::{AppHandle, Manager, State};

/// Start scanning for BLE devices
#[tauri::command]
pub async fn start_scan(
    ble_manager: State<'_, BleManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    ble_manager.start_scan(app_handle).await
}

/// Stop scanning for BLE devices
#[tauri::command]
pub async fn stop_scan(ble_manager: State<'_, BleManager>) -> Result<(), String> {
    ble_manager.stop_scan().await
}

/// Get discovered devices
#[tauri::command]
pub async fn get_devices(ble_manager: State<'_, BleManager>) -> Result<Vec<DeviceInfo>, String> {
    Ok(ble_manager.get_devices().await)
}

/// Connect to a device
#[tauri::command]
pub async fn connect_device(
    ble_manager: State<'_, BleManager>,
    device_id: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    ble_manager.connect_device(device_id, app_handle).await
}

/// Disconnect from current device
#[tauri::command]
pub async fn disconnect_device(
    ble_manager: State<'_, BleManager>,
    app_handle: AppHandle,
) -> Result<(), String> {
    ble_manager.disconnect_device(app_handle).await
}

/// Get current connection state
#[tauri::command]
pub async fn get_connection_state(
    ble_manager: State<'_, BleManager>,
) -> Result<ConnectionState, String> {
    Ok(ble_manager.get_connection_state().await)
}

/// Get connected device info
#[tauri::command]
pub async fn get_connected_device(
    ble_manager: State<'_, BleManager>,
) -> Result<Option<DeviceInfo>, String> {
    Ok(ble_manager.get_connected_device().await)
}

/// Get heart rate history
#[tauri::command]
pub async fn get_heart_rate_history(
    db: State<'_, Database>,
    limit: i64,
    offset: i64,
) -> Result<Vec<HeartRateRecord>, String> {
    db.get_history(limit, offset)
        .map_err(|e| format!("Failed to get history: {}", e))
}

/// Get heart rate history for a time range
#[tauri::command]
pub async fn get_heart_rate_history_range(
    db: State<'_, Database>,
    start_time: i64,
    end_time: i64,
) -> Result<Vec<HeartRateRecord>, String> {
    db.get_history_range(start_time, end_time)
        .map_err(|e| format!("Failed to get history range: {}", e))
}

/// Get aggregated heart rate statistics by time dimension
#[tauri::command]
pub async fn get_heart_rate_statistics(
    db: State<'_, Database>,
    dimension: String,
    start_time: Option<i64>,
    end_time: Option<i64>,
) -> Result<Vec<PeriodStats>, String> {
    db.get_statistics(&dimension, start_time, end_time)
        .map_err(|e| format!("Failed to get statistics: {}", e))
}

/// Save heart rate measurement
#[tauri::command]
pub async fn save_heart_rate(
    db: State<'_, Database>,
    measurement: HeartRateMeasurement,
    session_id: String,
) -> Result<i64, String> {
    let record = HeartRateRecord {
        id: None,
        bpm: measurement.bpm,
        sensor_contact: measurement.sensor_contact,
        timestamp: measurement.timestamp,
        session_id,
    };

    db.save_heart_rate(&record)
        .map_err(|e| format!("Failed to save heart rate: {}", e))
}

/// Set alert settings
#[tauri::command]
pub async fn set_alert_settings(
    db: State<'_, Database>,
    settings: AlertSettings,
) -> Result<(), String> {
    db.set_alert_settings(&settings)
        .map_err(|e| format!("Failed to save alert settings: {}", e))
}

/// Get alert settings
#[tauri::command]
pub async fn get_alert_settings(db: State<'_, Database>) -> Result<AlertSettings, String> {
    db.get_alert_settings()
        .map_err(|e| format!("Failed to get alert settings: {}", e))
}

/// Initialize the database (idempotent - database is initialized at startup)
#[tauri::command]
pub async fn init_database(app_handle: AppHandle) -> Result<(), String> {
    // Database is already initialized in main.rs setup
    // This command exists for frontend compatibility and just verifies the database exists
    if app_handle.try_state::<Database>().is_none() {
        return Err("Database not initialized. Application may have failed to start properly.".to_string());
    }
    Ok(())
}