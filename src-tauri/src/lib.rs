// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

// モジュール宣言
pub mod clipboard;
pub mod commands;
pub mod config;
pub mod database;
pub mod types;
pub mod window;

use clipboard::monitor_clipboard;
use commands::*;
use config::register_hotkey;
use database::DatabaseManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_global_shortcut::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let app_handle = app.handle();
            let db_path = app_handle.path().app_data_dir().unwrap().join("clipray.db");

            // データベースディレクトリが存在しない場合は作成
            if let Some(db_dir) = db_path.parent() {
                std::fs::create_dir_all(db_dir).unwrap();
            }

            let db = DatabaseManager::new(db_path).unwrap();

            // 設定の読み込みとショートカットの設定
            let config = db.get_config().unwrap();

            app.manage(db);

            // ホットキーを登録（失敗してもアプリケーションは継続）
            if let Err(e) = register_hotkey(&app_handle, &config.hotkey) {
                eprintln!("Failed to register hotkey '{}': {}", config.hotkey, e);
                eprintln!("The hotkey might already be in use by another application.");
                eprintln!("You can change the hotkey in the settings.");
            } else {
                println!("Hotkey '{}' registered successfully!", config.hotkey);
            }

            // クリップボード監視を開始
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
            get_config,
            update_config,
            show_window,
            hide_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
