use crate::storage::{AlertSettings, Database};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_notification::NotificationExt;

/// Minimum time between alerts of the same type (in seconds)
const ALERT_COOLDOWN_SECS: u64 = 60;

/// Last alert timestamps (stored as Unix timestamps in milliseconds)
static LAST_HIGH_ALERT: AtomicU64 = AtomicU64::new(0);
static LAST_LOW_ALERT: AtomicU64 = AtomicU64::new(0);

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

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    // Check high threshold with cooldown (atomic compare_exchange to prevent race)
    if bpm > settings.high_threshold && settings.notify_on_high {
        loop {
            let last = LAST_HIGH_ALERT.load(Ordering::Relaxed);
            if now - last > ALERT_COOLDOWN_SECS * 1000 {
                // Try to atomically update - if another thread updated first, retry
                match LAST_HIGH_ALERT.compare_exchange(
                    last,
                    now,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        send_notification(
                            app_handle,
                            "High Heart Rate Alert",
                            &format!("Your heart rate is {} BPM, which is above the threshold of {} BPM", bpm, settings.high_threshold),
                        )?;
                        break;
                    }
                    Err(_) => continue, // Another thread updated, retry check
                }
            } else {
                break;
            }
        }
    }

    // Check low threshold with cooldown (atomic compare_exchange to prevent race)
    if bpm < settings.low_threshold && settings.notify_on_low {
        loop {
            let last = LAST_LOW_ALERT.load(Ordering::Relaxed);
            if now - last > ALERT_COOLDOWN_SECS * 1000 {
                // Try to atomically update - if another thread updated first, retry
                match LAST_LOW_ALERT.compare_exchange(
                    last,
                    now,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        send_notification(
                            app_handle,
                            "Low Heart Rate Alert",
                            &format!("Your heart rate is {} BPM, which is below the threshold of {} BPM", bpm, settings.low_threshold),
                        )?;
                        break;
                    }
                    Err(_) => continue, // Another thread updated, retry check
                }
            } else {
                break;
            }
        }
    }

    Ok(())
}

/// Reset alert cooldown timers (call when settings change or on disconnect)
pub fn reset_alert_cooldowns() {
    LAST_HIGH_ALERT.store(0, Ordering::Relaxed);
    LAST_LOW_ALERT.store(0, Ordering::Relaxed);
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