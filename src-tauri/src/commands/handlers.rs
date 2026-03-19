use crate::ble::{ConnectionState, DeviceInfo, HeartRateMeasurement, BleManager};
use crate::storage::{AlertSettings, Database, HeartRateRecord};
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

/// Initialize the database
#[tauri::command]
pub async fn init_database(app_handle: AppHandle) -> Result<(), String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    let db_path = app_data_dir.join("health_tracker.db");
    let db = Database::new(&db_path)
        .map_err(|e| format!("Failed to initialize database: {}", e))?;

    // Store the database in app state
    app_handle.manage(db);

    Ok(())
}