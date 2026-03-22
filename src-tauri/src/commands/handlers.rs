use crate::ble::{ConnectionState, DeviceInfo, HeartRateMeasurement, BleManager};
use crate::storage::{
    AlertSettings, BaselineStats, Database, DetectionResult, ExerciseStats, ExerciseTag,
    ExerciseTypeStats, HeartRateRecord, HRVResult, PeriodStats, SessionInfo,
};
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

/// Get heart rate records for a specific session
#[tauri::command]
pub async fn get_session_records(
    db: State<'_, Database>,
    session_id: String,
) -> Result<Vec<HeartRateRecord>, String> {
    db.get_session_records(&session_id)
        .map_err(|e| format!("Failed to get session records: {}", e))
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

/// Get HRV estimation from BPM variance (per D-09, D-10, D-11)
#[tauri::command]
pub async fn get_hrv_estimate(
    db: State<'_, Database>,
    start_time: Option<i64>,
    end_time: Option<i64>,
) -> Result<Option<HRVResult>, String> {
    db.calculate_hrv(start_time, end_time)
        .map_err(|e| format!("Failed to calculate HRV: {}", e))
}

/// Tag a session as exercise (per D-01, D-03, D-04)
#[tauri::command]
pub async fn tag_exercise_session(
    db: State<'_, Database>,
    session_id: String,
    exercise_type: String,
    is_confirmed: bool,
) -> Result<(), String> {
    let tag = ExerciseTag {
        session_id,
        exercise_type,
        is_confirmed,
        confidence: None, // Manual tags have no confidence score
        tagged_at: chrono::Utc::now().timestamp_millis(),
    };

    db.tag_exercise(&tag)
        .map_err(|e| format!("Failed to tag exercise: {}", e))
}

/// Get list of sessions with exercise info
#[tauri::command]
pub async fn get_sessions_list(
    db: State<'_, Database>,
    limit: i64,
    offset: i64,
) -> Result<Vec<SessionInfo>, String> {
    db.get_sessions(limit, offset)
        .map_err(|e| format!("Failed to get sessions: {}", e))
}

/// Delete a session and all its data
#[tauri::command]
pub async fn delete_session(
    db: State<'_, Database>,
    session_id: String,
) -> Result<(), String> {
    db.delete_session(&session_id)
        .map_err(|e| format!("Failed to delete session: {}", e))
}

/// Detect if a session is exercise (per D-05, D-08)
#[tauri::command]
pub async fn detect_exercise_session(
    db: State<'_, Database>,
    session_id: String,
) -> Result<DetectionResult, String> {
    db.detect_exercise(&session_id)
        .map_err(|e| format!("Failed to detect exercise: {}", e))
}

/// Get personal resting baseline (per D-06, D-15)
#[tauri::command]
pub async fn get_resting_baseline(
    db: State<'_, Database>,
) -> Result<Option<BaselineStats>, String> {
    db.calculate_resting_baseline()
        .map_err(|e| format!("Failed to get resting baseline: {}", e))
}

/// Detect exercise for all untagged sessions
#[tauri::command]
pub async fn detect_exercise_all(
    db: State<'_, Database>,
) -> Result<Vec<DetectionResult>, String> {
    db.detect_exercise_for_sessions()
        .map_err(|e| format!("Failed to detect exercise for sessions: {}", e))
}

/// Remove exercise tag from a session
#[tauri::command]
pub async fn remove_exercise_tag(
    db: State<'_, Database>,
    session_id: String,
) -> Result<(), String> {
    db.remove_exercise_tag(&session_id)
        .map_err(|e| format!("Failed to remove exercise tag: {}", e))
}

/// Get all sessions with exercise tags
#[tauri::command]
pub async fn get_exercise_tags(
    db: State<'_, Database>,
    confirmed_only: bool,
) -> Result<Vec<ExerciseTag>, String> {
    db.get_sessions_with_exercise(confirmed_only)
        .map_err(|e| format!("Failed to get exercise tags: {}", e))
}

/// Get exercise vs resting comparison statistics (per D-14, D-15)
#[tauri::command]
pub async fn get_exercise_statistics(db: State<'_, Database>) -> Result<ExerciseStats, String> {
    db.get_exercise_statistics()
        .map_err(|e| format!("Failed to get exercise statistics: {}", e))
}

/// Get statistics grouped by exercise type (per STAT-08)
#[tauri::command]
pub async fn get_exercise_type_statistics(
    db: State<'_, Database>,
) -> Result<Vec<ExerciseTypeStats>, String> {
    db.get_exercise_type_statistics()
        .map_err(|e| format!("Failed to get exercise type statistics: {}", e))
}

// ==================== Exercise Types Management ====================

/// Get all exercise types
#[tauri::command]
pub async fn get_exercise_types(db: State<'_, Database>) -> Result<Vec<String>, String> {
    db.get_exercise_types()
        .map_err(|e| format!("Failed to get exercise types: {}", e))
}

/// Add a new exercise type
#[tauri::command]
pub async fn add_exercise_type(
    db: State<'_, Database>,
    name: String,
) -> Result<(), String> {
    db.add_exercise_type(&name)
        .map_err(|e| format!("Failed to add exercise type: {}", e))
}

/// Update an exercise type
#[tauri::command]
pub async fn update_exercise_type(
    db: State<'_, Database>,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    db.update_exercise_type(&old_name, &new_name)
        .map_err(|e| format!("Failed to update exercise type: {}", e))
}

/// Delete an exercise type
#[tauri::command]
pub async fn delete_exercise_type(
    db: State<'_, Database>,
    name: String,
) -> Result<(), String> {
    db.delete_exercise_type(&name)
        .map_err(|e| format!("Failed to delete exercise type: {}", e))
}