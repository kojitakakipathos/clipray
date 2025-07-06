#[cfg(test)]
mod tests {
    use crate::libs::types::*;

    #[test]
    fn test_theme_preset_as_str() {
        assert_eq!(ThemePreset::Default.as_str(), "default");
        assert_eq!(ThemePreset::PurpleGradient.as_str(), "purple-gradient");
        assert_eq!(ThemePreset::DeepPurple.as_str(), "deep-purple");
        assert_eq!(ThemePreset::MidnightBlue.as_str(), "midnight-blue");
    }

    #[test]
    fn test_theme_preset_from_str() {
        assert_eq!(ThemePreset::from_str("default"), Some(ThemePreset::Default));
        assert_eq!(
            ThemePreset::from_str("purple-gradient"),
            Some(ThemePreset::PurpleGradient)
        );
        assert_eq!(
            ThemePreset::from_str("deep-purple"),
            Some(ThemePreset::DeepPurple)
        );
        assert_eq!(
            ThemePreset::from_str("midnight-blue"),
            Some(ThemePreset::MidnightBlue)
        );
        assert_eq!(ThemePreset::from_str("invalid"), None);
        assert_eq!(ThemePreset::from_str(""), None);
    }

    #[test]
    fn test_theme_preset_roundtrip() {
        let presets = vec![
            ThemePreset::Default,
            ThemePreset::PurpleGradient,
            ThemePreset::DeepPurple,
            ThemePreset::MidnightBlue,
        ];

        for preset in presets {
            let str_value = preset.as_str();
            let parsed_preset = ThemePreset::from_str(str_value).unwrap();
            assert_eq!(preset, parsed_preset);
        }
    }

    #[test]
    fn test_theme_config_creation() {
        let theme_config = ThemeConfig {
            preset: ThemePreset::DeepPurple,
        };

        assert_eq!(theme_config.preset, ThemePreset::DeepPurple);
    }

    #[test]
    fn test_app_config_creation() {
        let app_config = AppConfig {
            max_history_count: 25,
            hotkey: "Ctrl+Alt+C".to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::MidnightBlue,
            },
        };

        assert_eq!(app_config.max_history_count, 25);
        assert_eq!(app_config.hotkey, "Ctrl+Alt+C");
        assert_eq!(app_config.theme.preset, ThemePreset::MidnightBlue);
    }

    #[test]
    fn test_clipboard_item_creation() {
        let clipboard_item = ClipboardItem {
            id: 1,
            content: "test content".to_string(),
            content_type: "text".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            pinned: false,
        };

        assert_eq!(clipboard_item.id, 1);
        assert_eq!(clipboard_item.content, "test content");
        assert_eq!(clipboard_item.content_type, "text");
        assert_eq!(clipboard_item.timestamp, "2024-01-01T00:00:00Z");
        assert!(!clipboard_item.pinned);
    }

    #[test]
    fn test_clipboard_item_serialization() {
        let clipboard_item = ClipboardItem {
            id: 1,
            content: "test content".to_string(),
            content_type: "text".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            pinned: true,
        };

        // Test JSON serialization
        let json = serde_json::to_string(&clipboard_item).unwrap();
        let deserialized: ClipboardItem = serde_json::from_str(&json).unwrap();

        assert_eq!(clipboard_item.id, deserialized.id);
        assert_eq!(clipboard_item.content, deserialized.content);
        assert_eq!(clipboard_item.content_type, deserialized.content_type);
        assert_eq!(clipboard_item.timestamp, deserialized.timestamp);
        assert_eq!(clipboard_item.pinned, deserialized.pinned);
    }

    #[test]
    fn test_theme_config_serialization() {
        let theme_config = ThemeConfig {
            preset: ThemePreset::PurpleGradient,
        };

        // Test JSON serialization
        let json = serde_json::to_string(&theme_config).unwrap();
        let deserialized: ThemeConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(theme_config.preset, deserialized.preset);
    }

    #[test]
    fn test_app_config_serialization() {
        let app_config = AppConfig {
            max_history_count: 75,
            hotkey: "Ctrl+Shift+X".to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::DeepPurple,
            },
        };

        // Test JSON serialization
        let json = serde_json::to_string(&app_config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(app_config.max_history_count, deserialized.max_history_count);
        assert_eq!(app_config.hotkey, deserialized.hotkey);
        assert_eq!(app_config.theme.preset, deserialized.theme.preset);
    }
}
