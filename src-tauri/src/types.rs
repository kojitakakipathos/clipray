use serde::{Deserialize, Serialize};

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
