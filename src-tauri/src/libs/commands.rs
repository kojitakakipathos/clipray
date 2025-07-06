use tauri::{AppHandle, Manager, State, Window, Wry};
use tauri_plugin_clipboard::Clipboard;

use crate::libs::clipboard::copy_to_clipboard_impl;
use crate::libs::config::update_hotkey;
use crate::libs::database::DatabaseManager;
use crate::libs::types::{AppConfig, ClipboardItem};
use crate::libs::window::{hide_window_impl, show_window_impl};

#[tauri::command]
/// Get clipboard history
pub async fn get_clipboard_history(
    db: State<'_, DatabaseManager>,
) -> Result<Vec<ClipboardItem>, String> {
    db.get_clipboard_history().map_err(|e| e.to_string())
}

/// Delete clipboard item
#[tauri::command]
pub async fn delete_clipboard_item(id: i64, db: State<'_, DatabaseManager>) -> Result<(), String> {
    db.delete_clipboard_item(id).map_err(|e| e.to_string())
}

/// Toggle pin
#[tauri::command]
pub async fn toggle_pin(id: i64, db: State<'_, DatabaseManager>) -> Result<(), String> {
    db.toggle_pin(id).map_err(|e| e.to_string())
}

/// Copy to clipboard
#[tauri::command]
pub async fn copy_to_clipboard(
    content: String,
    content_type: String,
    window: Window<Wry>,
) -> Result<(), String> {
    let clipboard = window.state::<Clipboard>();
    copy_to_clipboard_impl(content, content_type, &clipboard)
}

/// Copy to clipboard and hide window
#[tauri::command]
pub async fn copy_and_hide(
    content: String,
    content_type: String,
    window: Window<Wry>,
) -> Result<(), String> {
    // Copy to clipboard
    let clipboard = window.state::<Clipboard>();
    copy_to_clipboard_impl(content, content_type, &clipboard)?;

    // Hide window
    hide_window_impl(&window)?;

    Ok(())
}

// Get configuration
#[tauri::command]
pub async fn get_config(db: State<'_, DatabaseManager>) -> Result<AppConfig, String> {
    db.get_config().map_err(|e| e.to_string())
}

// Update configuration
#[tauri::command]
pub async fn update_config(
    config: AppConfig,
    db: State<'_, DatabaseManager>,
    app_handle: AppHandle<Wry>,
) -> Result<(), String> {
    // Get current configuration
    let current_config = db.get_config().map_err(|e| e.to_string())?;

    // Re-register hotkey if changed
    if current_config.hotkey != config.hotkey {
        update_hotkey(&app_handle, &current_config.hotkey, &config.hotkey)?;
    }

    // Save configuration
    db.update_config(&config).map_err(|e| e.to_string())
}

// Show window
#[tauri::command]
pub async fn show_window(window: Window<Wry>) -> Result<(), String> {
    show_window_impl(&window)
}

// Hide window
#[tauri::command]
pub async fn hide_window(window: Window<Wry>) -> Result<(), String> {
    hide_window_impl(&window)
}

// Exit application
#[tauri::command]
pub async fn exit_app(app_handle: AppHandle<Wry>) -> Result<(), String> {
    app_handle.exit(0);
    Ok(())
}
