#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use health_tracker::{commands, storage::Database, BleManager};
use tauri::Manager;

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    log::info!("Starting Health Tracker application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(BleManager::default())
        .setup(|app| {
            // Initialize database
            let app_handle = app.handle();
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            let db_path = app_data_dir.join("health_tracker.db");
            let db = Database::new(&db_path)
                .expect("Failed to initialize database");

            app_handle.manage(db);

            log::info!("Application initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_scan,
            commands::stop_scan,
            commands::get_devices,
            commands::connect_device,
            commands::disconnect_device,
            commands::get_connection_state,
            commands::get_connected_device,
            commands::get_heart_rate_history,
            commands::get_heart_rate_history_range,
            commands::get_heart_rate_statistics,
            commands::save_heart_rate,
            commands::set_alert_settings,
            commands::get_alert_settings,
            commands::init_database,
            commands::get_hrv_estimate,
            commands::tag_exercise_session,
            commands::get_sessions_list,
            commands::detect_exercise_session,
            commands::get_resting_baseline,
            commands::detect_exercise_all,
            commands::remove_exercise_tag,
            commands::get_exercise_tags,
            commands::get_exercise_statistics,
            commands::get_exercise_type_statistics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}