use chrono::Utc;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::libs::{
    constants::{DEFAULT_HOTKEY, DEFAULT_MAX_HISTORY_COUNT},
    types::{AppConfig, ClipboardItem, ThemeConfig, ThemePreset},
};

// Structure to manage database connections
pub struct DatabaseManager {
    connection: Mutex<Connection>,
}

impl DatabaseManager {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Create tables
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

        // Insert default settings
        let _ = conn.execute(
            "INSERT OR IGNORE INTO app_config (key, value) VALUES ('max_history_count', '50')",
            [],
        );
        let _ = conn.execute(
            "INSERT OR IGNORE INTO app_config (key, value) VALUES ('hotkey', ?1)",
            [DEFAULT_HOTKEY],
        );
        let _ = conn.execute(
            "INSERT OR IGNORE INTO app_config (key, value) VALUES ('theme_preset', 'default')",
            [],
        );

        // Migrate 'dark' theme to 'deep-purple'
        let _ = conn.execute(
            "UPDATE app_config SET value = 'deep-purple' WHERE key = 'theme_preset' AND value = 'dark'",
            [],
        );

        // Migrate 'light' theme to 'default' and 'default' to 'purple-gradient'
        let _ = conn.execute(
            "UPDATE app_config SET value = 'purple-gradient' WHERE key = 'theme_preset' AND value = 'default'",
            [],
        );
        let _ = conn.execute(
            "UPDATE app_config SET value = 'default' WHERE key = 'theme_preset' AND value = 'light'",
            [],
        );

        Ok(DatabaseManager {
            connection: Mutex::new(conn),
        })
    }

    pub fn add_clipboard_item(&self, content: &str, content_type: &str) -> Result<()> {
        // Apply history count limit (only for non-pinned items)
        let max_count: u32 = self.get_max_history_count()?;

        let conn = self.connection.lock().unwrap();
        let timestamp = Utc::now().to_rfc3339();

        // Delete if the same content already exists (to avoid duplicates)
        conn.execute(
            "DELETE FROM clipboard_history WHERE content = ?1 AND content_type = ?2",
            [content, content_type],
        )?;

        // Add new item
        conn.execute(
            "INSERT INTO clipboard_history (content, content_type, timestamp, pinned) VALUES (?1, ?2, ?3, FALSE)",
            [content, content_type, &timestamp],
        )?;

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
        let max_count: u32 = self.get_max_history_count()?;
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, content, content_type, timestamp, pinned 
             FROM clipboard_history 
             ORDER BY pinned DESC, timestamp DESC
             LIMIT ?1",
        )?;

        let item_iter = stmt.query_map([max_count], |row| {
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
        let max_history_count: u32 = self.get_max_history_count()?;
        let hotkey: String = self.get_hotkey()?;
        let theme: ThemeConfig = self.get_theme()?;
        Ok(AppConfig {
            max_history_count,
            hotkey,
            theme,
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
        conn.execute(
            "UPDATE app_config SET value = ?1 WHERE key = 'theme_preset'",
            [config.theme.preset.as_str()],
        )?;
        Ok(())
    }

    pub fn get_hotkey(&self) -> Result<String> {
        let conn = self.connection.lock().unwrap();
        let hotkey: String = conn
            .query_row(
                "SELECT value FROM app_config WHERE key = 'hotkey'",
                [],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| DEFAULT_HOTKEY.to_string());
        Ok(hotkey)
    }

    /// get max history count from app_config
    pub fn get_max_history_count(&self) -> Result<u32> {
        let conn = self.connection.lock().unwrap();
        let value_str: String = conn
            .query_row(
                "SELECT value FROM app_config WHERE key = 'max_history_count'",
                [],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| DEFAULT_MAX_HISTORY_COUNT.to_string());

        let max_history_count = value_str.parse::<u32>().unwrap_or_else(|e| {
            eprintln!("Failed to parse max_history_count: {}", e);
            DEFAULT_MAX_HISTORY_COUNT
        });

        Ok(max_history_count)
    }

    /// get theme config from app_config
    pub fn get_theme(&self) -> Result<ThemeConfig> {
        let conn = self.connection.lock().unwrap();
        let preset_str: String = conn
            .query_row(
                "SELECT value FROM app_config WHERE key = 'theme_preset'",
                [],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "default".to_string());

        let preset = ThemePreset::from_str(&preset_str).unwrap_or(ThemePreset::Default);

        Ok(ThemeConfig { preset })
    }
}
