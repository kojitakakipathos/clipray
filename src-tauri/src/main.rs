// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use tauri::Manager;

pub mod libs;

use libs::clipboard::monitor_clipboard;
use libs::commands::*;
use libs::config::register_hotkey;
use libs::database::DatabaseManager;

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    run();
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_global_shortcut::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let app_handle = app.handle();
            let db_path = app_handle.path().app_data_dir().unwrap().join("clipray.db");

            // Create database directory if it doesn't exist
            if let Some(db_dir) = db_path.parent() {
                std::fs::create_dir_all(db_dir).unwrap();
            }

            let db = DatabaseManager::new(db_path).unwrap();

            // Load configuration and setup hotkey
            let config = db.get_config().unwrap();

            app.manage(db);

            // Register hotkey (application continues even if this fails)
            if let Err(e) = register_hotkey(&app_handle, &config.hotkey) {
                eprintln!("Failed to register hotkey '{}': {}", config.hotkey, e);
                eprintln!("The hotkey might already be in use by another application.");
                eprintln!("You can change the hotkey in the settings.");
            } else {
                println!("Hotkey '{}' registered successfully!", config.hotkey);
            }

            // Start clipboard monitoring
            let handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                monitor_clipboard(handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_clipboard_history,
            delete_clipboard_item,
            toggle_pin,
            copy_to_clipboard,
            copy_and_hide,
            get_config,
            update_config,
            show_window,
            hide_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
