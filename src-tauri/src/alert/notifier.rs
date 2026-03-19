use crate::storage::{AlertSettings, Database};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_notification::NotificationExt;

/// Check if heart rate triggers alert and send notification
pub async fn check_alert(bpm: u16, app_handle: &AppHandle) -> Result<(), String> {
    // Get alert settings from store or use defaults
    let settings: AlertSettings = app_handle
        .try_state::<Database>()
        .map(|db: State<'_, Database>| db.get_alert_settings().unwrap_or_default())
        .unwrap_or_default();

    if !settings.enabled {
        return Ok(());
    }

    // Check high threshold
    if bpm > settings.high_threshold && settings.notify_on_high {
        send_notification(
            app_handle,
            "High Heart Rate Alert",
            &format!("Your heart rate is {} BPM, which is above the threshold of {} BPM", bpm, settings.high_threshold),
        )?;
    }

    // Check low threshold
    if bpm < settings.low_threshold && settings.notify_on_low {
        send_notification(
            app_handle,
            "Low Heart Rate Alert",
            &format!("Your heart rate is {} BPM, which is below the threshold of {} BPM", bpm, settings.low_threshold),
        )?;
    }

    Ok(())
}

/// Send a system notification
fn send_notification(app_handle: &AppHandle, title: &str, body: &str) -> Result<(), String> {
    app_handle
        .notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .map_err(|e| format!("Failed to send notification: {}", e))?;

    Ok(())
}