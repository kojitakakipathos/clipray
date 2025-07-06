#[cfg(test)]
mod tests {
    use crate::libs::{database::DatabaseManager, types::*};
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_complete_workflow_add_pin_delete() {
        let db = DatabaseManager::new_test().unwrap();

        // Test adding items
        db.add_clipboard_item("integration test 1", "text").unwrap();
        db.add_clipboard_item("integration test 2", "image")
            .unwrap();

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 2);

        // Test pinning
        let item_id = history[0].id;
        db.toggle_pin(item_id).unwrap();

        let updated_history = db.get_clipboard_history().unwrap();
        let pinned_item = updated_history
            .iter()
            .find(|item| item.id == item_id)
            .unwrap();
        assert!(pinned_item.pinned);

        // Test deletion
        db.delete_clipboard_item(item_id).unwrap();
        let final_history = db.get_clipboard_history().unwrap();
        assert_eq!(final_history.len(), 1);
    }

    #[tokio::test]
    #[serial]
    async fn test_config_update_and_persistence() {
        let db = DatabaseManager::new_test().unwrap();

        // Test default config
        let config = db.get_config().unwrap();
        assert_eq!(config.max_history_count, 50);
        assert_eq!(config.theme.preset, ThemePreset::Default);

        // Test config update
        let new_config = AppConfig {
            max_history_count: 30,
            hotkey: "Ctrl+Alt+V".to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::DeepPurple,
            },
        };

        db.update_config(&new_config).unwrap();

        let updated_config = db.get_config().unwrap();
        assert_eq!(updated_config.max_history_count, 30);
        assert_eq!(updated_config.hotkey, "Ctrl+Alt+V");
        assert_eq!(updated_config.theme.preset, ThemePreset::DeepPurple);
    }

    #[tokio::test]
    #[serial]
    async fn test_history_limit_enforcement() {
        let db = DatabaseManager::new_test().unwrap();

        // Set history limit to 3
        let config = AppConfig {
            max_history_count: 3,
            hotkey: "Ctrl+Shift+V".to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::Default,
            },
        };
        db.update_config(&config).unwrap();

        // Add 5 items
        for i in 1..=5 {
            db.add_clipboard_item(&format!("item {}", i), "text")
                .unwrap();
        }

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 3);

        // Check that the latest items are kept
        assert_eq!(history[0].content, "item 5");
        assert_eq!(history[1].content, "item 4");
        assert_eq!(history[2].content, "item 3");
    }

    #[tokio::test]
    #[serial]
    async fn test_pinned_items_survive_history_limit() {
        let db = DatabaseManager::new_test().unwrap();

        // Set history limit to 1
        let config = AppConfig {
            max_history_count: 1,
            hotkey: "Ctrl+Shift+V".to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::Default,
            },
        };
        db.update_config(&config).unwrap();

        // Add and pin first item
        db.add_clipboard_item("pinned item", "text").unwrap();
        let history = db.get_clipboard_history().unwrap();
        let pinned_id = history[0].id;
        db.toggle_pin(pinned_id).unwrap();

        // Add more items
        db.add_clipboard_item("item 2", "text").unwrap();
        db.add_clipboard_item("item 3", "text").unwrap();

        let final_history = db.get_clipboard_history().unwrap();

        // The get_clipboard_history method applies the limit, so it will only return 1 item
        // But the pinned item should still exist in the database and not be deleted
        assert_eq!(final_history.len(), 1);

        // Pinned item should be first (due to ORDER BY pinned DESC)
        assert!(final_history[0].pinned);
        assert_eq!(final_history[0].content, "pinned item");
    }

    #[tokio::test]
    #[serial]
    async fn test_duplicate_content_replacement() {
        let db = DatabaseManager::new_test().unwrap();

        // Add same content multiple times
        db.add_clipboard_item("duplicate", "text").unwrap();
        db.add_clipboard_item("other content", "text").unwrap();
        db.add_clipboard_item("duplicate", "text").unwrap(); // Should replace the first one

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 2);

        // "duplicate" should be at the top as it was added last
        assert_eq!(history[0].content, "duplicate");
        assert_eq!(history[1].content, "other content");
    }

    #[tokio::test]
    #[serial]
    async fn test_mixed_content_types_handling() {
        let db = DatabaseManager::new_test().unwrap();

        db.add_clipboard_item("text content", "text").unwrap();
        db.add_clipboard_item("image data", "image").unwrap();
        db.add_clipboard_item("another text", "text").unwrap();

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 3);

        // Check content types
        let text_items: Vec<_> = history
            .iter()
            .filter(|item| item.content_type == "text")
            .collect();
        let image_items: Vec<_> = history
            .iter()
            .filter(|item| item.content_type == "image")
            .collect();

        assert_eq!(text_items.len(), 2);
        assert_eq!(image_items.len(), 1);
    }

    #[tokio::test]
    #[serial]
    async fn test_theme_configuration_changes() {
        let db = DatabaseManager::new_test().unwrap();

        // Test all theme presets
        let themes = vec![
            ThemePreset::Default,
            ThemePreset::PurpleGradient,
            ThemePreset::DeepPurple,
            ThemePreset::MidnightBlue,
        ];

        for theme in themes {
            let config = AppConfig {
                max_history_count: 50,
                hotkey: "Ctrl+Shift+V".to_string(),
                theme: ThemeConfig {
                    preset: theme.clone(),
                },
            };

            db.update_config(&config).unwrap();

            let updated_config = db.get_config().unwrap();
            assert_eq!(updated_config.theme.preset, theme);
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_empty_database_operations() {
        let db = DatabaseManager::new_test().unwrap();

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 0);

        // Test operations on empty database
        let result = db.delete_clipboard_item(1);
        assert!(result.is_ok());

        let result = db.toggle_pin(1);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_large_content_storage() {
        let db = DatabaseManager::new_test().unwrap();

        // Test with large content
        let large_content = "a".repeat(10000);
        db.add_clipboard_item(&large_content, "text").unwrap();

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].content.len(), 10000);
        assert_eq!(history[0].content, large_content);
    }

    #[tokio::test]
    #[serial]
    async fn test_special_characters_support() {
        let db = DatabaseManager::new_test().unwrap();

        // Test with special characters
        let special_content = "Hello 世界! 🌍 @#$%^&*()";
        db.add_clipboard_item(special_content, "text").unwrap();

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].content, special_content);
    }

    #[tokio::test]
    #[serial]
    async fn test_multiple_operations_sequence() {
        let db = DatabaseManager::new_test().unwrap();

        // Add some initial data
        db.add_clipboard_item("item 1", "text").unwrap();
        db.add_clipboard_item("item 2", "text").unwrap();

        let history = db.get_clipboard_history().unwrap();
        let item1_id = history[1].id;
        let item2_id = history[0].id;

        // Perform multiple operations
        db.toggle_pin(item1_id).unwrap();
        db.add_clipboard_item("item 3", "text").unwrap();
        db.toggle_pin(item2_id).unwrap();

        let final_history = db.get_clipboard_history().unwrap();
        assert_eq!(final_history.len(), 3);

        // Both pinned items should be at the top
        let pinned_count = final_history.iter().filter(|item| item.pinned).count();
        assert_eq!(pinned_count, 2);
    }
}
