use base64::{engine::general_purpose, Engine};
use tauri::{AppHandle, Emitter, Manager, Wry};
use tauri_plugin_clipboard::Clipboard;

use crate::database::DatabaseManager;

pub async fn monitor_clipboard(app_handle: AppHandle<Wry>) {
    let mut last_content = String::new();
    let mut last_image_content: Option<String> = None;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        if let Some(window) = app_handle.get_webview_window("main") {
            let clipboard = window.state::<Clipboard>();
            // テキスト
            if let Ok(current_content) = clipboard.read_text() {
                if current_content != last_content && !current_content.is_empty() {
                    last_content = current_content.clone();
                    if let Some(db) = app_handle.try_state::<DatabaseManager>() {
                        let _ = db.add_clipboard_item(&current_content, "text");
                        let _ = window.emit("clipboard-updated", ());
                    }
                }
            }
            // 画像
            if let Ok(image_data) = clipboard.read_image_base64() {
                if Some(&image_data) != last_image_content.as_ref() {
                    last_image_content = Some(image_data.clone());
                    let encoded = image_data;
                    if let Some(db) = app_handle.try_state::<DatabaseManager>() {
                        let _ = db.add_clipboard_item(&encoded, "image");
                        let _ = window.emit("clipboard-updated", ());
                    }
                }
            }
        }
    }
}

pub fn copy_to_clipboard_impl(
    content: String,
    content_type: String,
    clipboard: &Clipboard,
) -> Result<(), String> {
    if content_type == "text" {
        clipboard.write_text(content).map_err(|e| e.to_string())?;
    } else if content_type == "image" {
        // Base64デコード
        let image_data = general_purpose::STANDARD
            .decode(content)
            .map_err(|e| e.to_string())?;
        let image_base64 = general_purpose::STANDARD.encode(&image_data);
        clipboard
            .write_image_base64(image_base64)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
