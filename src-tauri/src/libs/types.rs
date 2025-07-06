use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClipboardItem {
    pub id: i64,
    pub content: String,
    pub content_type: String, // "text" | "image"
    pub timestamp: String,
    pub pinned: bool,
}

// テーマプリセットのENUM
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ThemePreset {
    Default,
    PurpleGradient,
    DeepPurple,
    MidnightBlue,
}

impl ThemePreset {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemePreset::Default => "default",
            ThemePreset::PurpleGradient => "purple-gradient",
            ThemePreset::DeepPurple => "deep-purple",
            ThemePreset::MidnightBlue => "midnight-blue",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "default" => Some(ThemePreset::Default),
            "purple-gradient" => Some(ThemePreset::PurpleGradient),
            "deep-purple" => Some(ThemePreset::DeepPurple),
            "midnight-blue" => Some(ThemePreset::MidnightBlue),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    pub preset: ThemePreset,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub max_history_count: u32,
    pub hotkey: String,
    pub theme: ThemeConfig,
}
