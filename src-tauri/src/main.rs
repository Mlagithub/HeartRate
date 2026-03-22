#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use heart_rate::{commands, storage::Database, BleManager};
use tauri::Manager;

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .init();

    log::info!("Starting HeartRate application...");

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

            let db_path = app_data_dir.join("heart_rate.db");
            let db = Database::new(&db_path)
                .expect("Failed to initialize database");

            app_handle.manage(db);

            // Monitor window events for debugging
            let window = app_handle.get_webview_window("main").expect("Failed to get main window");
            window.on_window_event(move |event| {
                match event {
                    tauri::WindowEvent::Focused(focused) => {
                        log::debug!("Window focused: {}", focused);
                    }
                    tauri::WindowEvent::Destroyed => {
                        log::info!("Window destroyed");
                    }
                    _ => {}
                }
            });

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
            commands::get_session_records,
            commands::get_heart_rate_statistics,
            commands::save_heart_rate,
            commands::set_alert_settings,
            commands::get_alert_settings,
            commands::init_database,
            commands::get_hrv_estimate,
            commands::tag_exercise_session,
            commands::get_sessions_list,
            commands::delete_session,
            commands::detect_exercise_session,
            commands::get_resting_baseline,
            commands::detect_exercise_all,
            commands::remove_exercise_tag,
            commands::get_exercise_tags,
            commands::get_exercise_statistics,
            commands::get_exercise_type_statistics,
            commands::get_exercise_types,
            commands::add_exercise_type,
            commands::update_exercise_type,
            commands::delete_exercise_type,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}