// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, State, Window, Wry};
use tauri_plugin_clipboard::Clipboard;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClipboardItem {
    pub id: i64,
    pub content: String,
    pub content_type: String, // "text" | "image"
    pub timestamp: String,
    pub pinned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub max_history_count: i32,
    pub hotkey: String,
}

// データベース接続を管理する構造体
pub struct DatabaseManager {
    connection: Mutex<Connection>,
}

impl DatabaseManager {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // テーブル作成
        conn.execute(
            "CREATE TABLE IF NOT EXISTS clipboard_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                content_type TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                pinned BOOLEAN DEFAULT FALSE
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS app_config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // デフォルト設定を挿入
        let _ = conn.execute(
            "INSERT OR IGNORE INTO app_config (key, value) VALUES ('max_history_count', '50')",
            [],
        );
        let _ = conn.execute(
            "INSERT OR IGNORE INTO app_config (key, value) VALUES ('hotkey', 'CommandOrControl+Alt+V')",
            [],
        );

        Ok(DatabaseManager {
            connection: Mutex::new(conn),
        })
    }

    pub fn add_clipboard_item(&self, content: &str, content_type: &str) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        let timestamp = Utc::now().to_rfc3339();

        // 同じ内容が既にある場合は削除（重複を避ける）
        conn.execute(
            "DELETE FROM clipboard_history WHERE content = ?1 AND content_type = ?2",
            [content, content_type],
        )?;

        // 新しいアイテムを追加
        conn.execute(
            "INSERT INTO clipboard_history (content, content_type, timestamp, pinned) VALUES (?1, ?2, ?3, FALSE)",
            [content, content_type, &timestamp],
        )?;

        // 履歴数制限を適用（ピン留めされていないもののみ）
        let max_count: i32 = conn
            .query_row(
                "SELECT value FROM app_config WHERE key = 'max_history_count'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(50);

        conn.execute(
            "DELETE FROM clipboard_history WHERE id IN (
                SELECT id FROM clipboard_history 
                WHERE pinned = FALSE 
                ORDER BY timestamp DESC 
                LIMIT -1 OFFSET ?1
            )",
            [max_count],
        )?;

        Ok(())
    }

    pub fn get_clipboard_history(&self) -> Result<Vec<ClipboardItem>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, content, content_type, timestamp, pinned 
             FROM clipboard_history 
             ORDER BY pinned DESC, timestamp DESC",
        )?;

        let item_iter = stmt.query_map([], |row| {
            Ok(ClipboardItem {
                id: row.get(0)?,
                content: row.get(1)?,
                content_type: row.get(2)?,
                timestamp: row.get(3)?,
                pinned: row.get(4)?,
            })
        })?;

        let mut items = Vec::new();
        for item in item_iter {
            items.push(item?);
        }

        Ok(items)
    }

    pub fn delete_clipboard_item(&self, id: i64) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute("DELETE FROM clipboard_history WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn toggle_pin(&self, id: i64) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "UPDATE clipboard_history SET pinned = NOT pinned WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }

    pub fn get_config(&self) -> Result<AppConfig> {
        let conn = self.connection.lock().unwrap();
        let max_history_count: i32 = conn
            .query_row(
                "SELECT value FROM app_config WHERE key = 'max_history_count'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(50);

        let hotkey: String = conn
            .query_row(
                "SELECT value FROM app_config WHERE key = 'hotkey'",
                [],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "CommandOrControl+Alt+V".to_string());

        Ok(AppConfig {
            max_history_count,
            hotkey,
        })
    }

    pub fn update_config(&self, config: &AppConfig) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "UPDATE app_config SET value = ?1 WHERE key = 'max_history_count'",
            [config.max_history_count.to_string()],
        )?;
        conn.execute(
            "UPDATE app_config SET value = ?1 WHERE key = 'hotkey'",
            [&config.hotkey],
        )?;
        Ok(())
    }
}

#[tauri::command]
async fn get_clipboard_history(
    db: State<'_, DatabaseManager>,
) -> Result<Vec<ClipboardItem>, String> {
    db.get_clipboard_history().map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_clipboard_item(id: i64, db: State<'_, DatabaseManager>) -> Result<(), String> {
    db.delete_clipboard_item(id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_pin(id: i64, db: State<'_, DatabaseManager>) -> Result<(), String> {
    db.toggle_pin(id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn copy_to_clipboard(
    content: String,
    content_type: String,
    window: Window<Wry>,
) -> Result<(), String> {
    if content_type == "text" {
        window
            .state::<Clipboard>()
            .write_text(content)
            .map_err(|e| e.to_string())?;
    } else if content_type == "image" {
        // Base64デコード
        let image_data = general_purpose::STANDARD
            .decode(content)
            .map_err(|e| e.to_string())?;
        let image_base64 = general_purpose::STANDARD.encode(&image_data);
        window
            .state::<Clipboard>()
            .write_image_base64(image_base64)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn get_config(db: State<'_, DatabaseManager>) -> Result<AppConfig, String> {
    db.get_config().map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_config(
    config: AppConfig,
    db: State<'_, DatabaseManager>,
    app_handle: AppHandle<Wry>,
) -> Result<(), String> {
    // 現在の設定を取得
    let current_config = db.get_config().map_err(|e| e.to_string())?;

    // ホットキーが変更された場合、再登録
    if current_config.hotkey != config.hotkey {
        let shortcut_manager = app_handle.global_shortcut();

        // 古いホットキーを削除
        if let Ok(old_shortcut) = current_config.hotkey.parse::<Shortcut>() {
            if let Err(e) = shortcut_manager.unregister(old_shortcut) {
                eprintln!("Failed to unregister old hotkey: {}", e);
            }
        }

        // 新しいホットキーを登録
        let window = app_handle.get_webview_window("main").unwrap();
        let hotkey = config
            .hotkey
            .parse::<Shortcut>()
            .map_err(|e| format!("Invalid hotkey format: {}", e))?;

        let window_clone = window.clone();
        shortcut_manager
            .on_shortcut(hotkey, move |_app, _shortcut, _event| {
                let _ = window_clone.emit("show-clipboard", ());
                let _ = window_clone.show();
                let _ = window_clone.set_focus();
            })
            .map_err(|e| format!("Failed to set hotkey handler: {}", e))?;

        shortcut_manager
            .register(hotkey)
            .map_err(|e| format!("Failed to register new hotkey: {}", e))?;
    }

    // 設定を保存
    db.update_config(&config).map_err(|e| e.to_string())
}

#[tauri::command]
async fn show_window(window: Window<Wry>) -> Result<(), String> {
    // ウィンドウを画面中央に配置
    let monitor = window.current_monitor().map_err(|e| e.to_string())?;
    if let Some(monitor) = monitor {
        let screen_size = monitor.size();
        let window_size = PhysicalSize::new(600, 500);
        let x = (screen_size.width - window_size.width) / 2;
        let y = (screen_size.height - window_size.height) / 2;

        let _ = window.set_position(PhysicalPosition::new(x as i32, y as i32));
    }

    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn hide_window(window: Window<Wry>) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}

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
            let shortcut = config.hotkey.parse::<Shortcut>().unwrap();

            app.manage(db);

            let shortcut_manager = app_handle.global_shortcut();
            let window = app_handle.get_webview_window("main").unwrap();
            let window_clone = window.clone();

            // ホットキーのイベントハンドラを設定
            if let Err(e) =
                shortcut_manager.on_shortcut(shortcut, move |_app, _shortcut, _event| {
                    let _ = window_clone.emit("show-clipboard", ());
                    let _ = window_clone.show();
                    let _ = window_clone.set_focus();
                })
            {
                eprintln!("Failed to set hotkey handler: {}", e);
            }

            // ホットキーを登録（失敗してもアプリケーションは継続）
            if let Err(e) = shortcut_manager.register(shortcut) {
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

async fn monitor_clipboard(app_handle: AppHandle<Wry>) {
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
