use tauri::{AppHandle, Manager, State, Window, Wry};
use tauri_plugin_clipboard::Clipboard;

use crate::clipboard::copy_to_clipboard_impl;
use crate::config::update_hotkey;
use crate::database::DatabaseManager;
use crate::types::{AppConfig, ClipboardItem};
use crate::window::{hide_window_impl, show_window_impl};

#[tauri::command]
pub async fn get_clipboard_history(
    db: State<'_, DatabaseManager>,
) -> Result<Vec<ClipboardItem>, String> {
    db.get_clipboard_history().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_clipboard_item(id: i64, db: State<'_, DatabaseManager>) -> Result<(), String> {
    db.delete_clipboard_item(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_pin(id: i64, db: State<'_, DatabaseManager>) -> Result<(), String> {
    db.toggle_pin(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn copy_to_clipboard(
    content: String,
    content_type: String,
    window: Window<Wry>,
) -> Result<(), String> {
    let clipboard = window.state::<Clipboard>();
    copy_to_clipboard_impl(content, content_type, &clipboard)
}

#[tauri::command]
pub async fn get_config(db: State<'_, DatabaseManager>) -> Result<AppConfig, String> {
    db.get_config().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_config(
    config: AppConfig,
    db: State<'_, DatabaseManager>,
    app_handle: AppHandle<Wry>,
) -> Result<(), String> {
    // 現在の設定を取得
    let current_config = db.get_config().map_err(|e| e.to_string())?;

    // ホットキーが変更された場合、再登録
    if current_config.hotkey != config.hotkey {
        update_hotkey(&app_handle, &current_config.hotkey, &config.hotkey)?;
    }

    // 設定を保存
    db.update_config(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn show_window(window: Window<Wry>) -> Result<(), String> {
    show_window_impl(&window)
}

#[tauri::command]
pub async fn hide_window(window: Window<Wry>) -> Result<(), String> {
    hide_window_impl(&window)
}
