#[cfg(test)]
mod tests {
    use crate::libs::{constants::DEFAULT_HOTKEY, database::DatabaseManager, types::*};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_new_test_database() {
        let db = DatabaseManager::new_test().unwrap();

        // Check if tables are created
        let config = db.get_config().unwrap();
        assert_eq!(config.max_history_count, 50);
        assert_eq!(config.hotkey, DEFAULT_HOTKEY);
        assert_eq!(config.theme.preset, ThemePreset::Default);
    }

    #[test]
    #[serial]
    fn test_add_clipboard_item() {
        let db = DatabaseManager::new_test().unwrap();

        let result = db.add_clipboard_item("test content", "text");
        assert!(result.is_ok());

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].content, "test content");
        assert_eq!(history[0].content_type, "text");
        assert!(!history[0].pinned);
    }

    #[test]
    #[serial]
    fn test_add_duplicate_content() {
        let db = DatabaseManager::new_test().unwrap();

        // Add same content twice
        db.add_clipboard_item("duplicate content", "text").unwrap();
        db.add_clipboard_item("duplicate content", "text").unwrap();

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 1); // Should only have one item
        assert_eq!(history[0].content, "duplicate content");
    }

    #[test]
    #[serial]
    fn test_get_clipboard_history() {
        let db = DatabaseManager::new_test().unwrap();
        db.setup_test_data().unwrap();

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 3);

        // Check if pinned items come first
        assert!(history[0].pinned);
        assert_eq!(history[0].content, "test content 2");

        // Check ordering by timestamp for non-pinned items
        assert!(!history[1].pinned);
        assert_eq!(history[1].content, "test image data"); // Latest non-pinned
        assert!(!history[2].pinned);
        assert_eq!(history[2].content, "test content 1"); // Oldest non-pinned
    }

    #[test]
    #[serial]
    fn test_delete_clipboard_item() {
        let db = DatabaseManager::new_test().unwrap();
        db.setup_test_data().unwrap();

        let history_before = db.get_clipboard_history().unwrap();
        let item_id = history_before[0].id;

        let result = db.delete_clipboard_item(item_id);
        assert!(result.is_ok());

        let history_after = db.get_clipboard_history().unwrap();
        assert_eq!(history_after.len(), history_before.len() - 1);
        assert!(!history_after.iter().any(|item| item.id == item_id));
    }

    #[test]
    #[serial]
    fn test_delete_nonexistent_item() {
        let db = DatabaseManager::new_test().unwrap();

        let result = db.delete_clipboard_item(999); // Non-existent ID
        assert!(result.is_ok()); // Should not fail

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 0);
    }

    #[test]
    #[serial]
    fn test_toggle_pin() {
        let db = DatabaseManager::new_test().unwrap();
        db.setup_test_data().unwrap();

        let history_before = db.get_clipboard_history().unwrap();
        let unpinned_item = history_before.iter().find(|item| !item.pinned).unwrap();
        let item_id = unpinned_item.id;

        let result = db.toggle_pin(item_id);
        assert!(result.is_ok());

        let history_after = db.get_clipboard_history().unwrap();
        let updated_item = history_after
            .iter()
            .find(|item| item.id == item_id)
            .unwrap();
        assert!(updated_item.pinned);
    }

    #[test]
    #[serial]
    fn test_toggle_pin_twice() {
        let db = DatabaseManager::new_test().unwrap();
        db.setup_test_data().unwrap();

        let history_before = db.get_clipboard_history().unwrap();
        let item = &history_before[0];
        let original_pinned = item.pinned;

        // Toggle twice
        db.toggle_pin(item.id).unwrap();
        db.toggle_pin(item.id).unwrap();

        let history_after = db.get_clipboard_history().unwrap();
        let updated_item = history_after.iter().find(|i| i.id == item.id).unwrap();
        assert_eq!(updated_item.pinned, original_pinned);
    }

    #[test]
    #[serial]
    fn test_max_history_limit() {
        let db = DatabaseManager::new_test().unwrap();

        // Set limit to 2
        let config = AppConfig {
            max_history_count: 2,
            hotkey: "Ctrl+Shift+V".to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::Default,
            },
        };
        db.update_config(&config).unwrap();

        // Add 3 items
        db.add_clipboard_item("item1", "text").unwrap();
        db.add_clipboard_item("item2", "text").unwrap();
        db.add_clipboard_item("item3", "text").unwrap();

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].content, "item3"); // Latest first
        assert_eq!(history[1].content, "item2");
    }

    #[test]
    #[serial]
    fn test_pinned_items_not_deleted_by_limit() {
        let db = DatabaseManager::new_test().unwrap();

        // Set limit to 1
        let config = AppConfig {
            max_history_count: 1,
            hotkey: "Ctrl+Shift+V".to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::Default,
            },
        };
        db.update_config(&config).unwrap();

        // Add item and pin it
        db.add_clipboard_item("pinned item", "text").unwrap();
        let history = db.get_clipboard_history().unwrap();
        let pinned_id = history[0].id;
        db.toggle_pin(pinned_id).unwrap();

        // Add new item
        db.add_clipboard_item("new item", "text").unwrap();

        let final_history = db.get_clipboard_history().unwrap();
        // The get_clipboard_history method applies the limit, so it will only return 1 item
        // But the pinned item should still exist in the database and not be deleted
        assert_eq!(final_history.len(), 1);

        // Check that the pinned item is preserved (it should be returned first due to ORDER BY pinned DESC)
        assert!(final_history[0].pinned);
        assert_eq!(final_history[0].content, "pinned item");

        // Verify that both items exist in the database by checking without limit
        let conn = db.get_connection().lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM clipboard_history")
            .unwrap();
        let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(count, 2); // Both items should exist in the database
    }

    #[test]
    #[serial]
    fn test_config_management() {
        let db = DatabaseManager::new_test().unwrap();

        // Check default config
        let config = db.get_config().unwrap();
        assert_eq!(config.max_history_count, 50);
        assert_eq!(config.hotkey, DEFAULT_HOTKEY);
        assert_eq!(config.theme.preset, ThemePreset::Default);

        // Update config
        let new_config = AppConfig {
            max_history_count: 100,
            hotkey: "Ctrl+Alt+V".to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::DeepPurple,
            },
        };

        let result = db.update_config(&new_config);
        assert!(result.is_ok());

        // Verify updated config
        let updated_config = db.get_config().unwrap();
        assert_eq!(updated_config.max_history_count, 100);
        assert_eq!(updated_config.hotkey, "Ctrl+Alt+V");
        assert_eq!(updated_config.theme.preset, ThemePreset::DeepPurple);
    }

    #[test]
    #[serial]
    fn test_get_hotkey() {
        let db = DatabaseManager::new_test().unwrap();

        let hotkey = db.get_hotkey().unwrap();
        assert_eq!(hotkey, DEFAULT_HOTKEY);

        // Update hotkey
        let config = AppConfig {
            max_history_count: 50,
            hotkey: "Ctrl+Shift+C".to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::Default,
            },
        };
        db.update_config(&config).unwrap();

        let updated_hotkey = db.get_hotkey().unwrap();
        assert_eq!(updated_hotkey, "Ctrl+Shift+C");
    }

    #[test]
    #[serial]
    fn test_get_max_history_count() {
        let db = DatabaseManager::new_test().unwrap();

        let count = db.get_max_history_count().unwrap();
        assert_eq!(count, 50);

        // Update count
        let config = AppConfig {
            max_history_count: 25,
            hotkey: DEFAULT_HOTKEY.to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::Default,
            },
        };
        db.update_config(&config).unwrap();

        let updated_count = db.get_max_history_count().unwrap();
        assert_eq!(updated_count, 25);
    }

    #[test]
    #[serial]
    fn test_get_theme() {
        let db = DatabaseManager::new_test().unwrap();

        let theme = db.get_theme().unwrap();
        assert_eq!(theme.preset, ThemePreset::Default);

        // Update theme
        let config = AppConfig {
            max_history_count: 50,
            hotkey: DEFAULT_HOTKEY.to_string(),
            theme: ThemeConfig {
                preset: ThemePreset::MidnightBlue,
            },
        };
        db.update_config(&config).unwrap();

        let updated_theme = db.get_theme().unwrap();
        assert_eq!(updated_theme.preset, ThemePreset::MidnightBlue);
    }

    #[test]
    #[serial]
    fn test_different_content_types() {
        let db = DatabaseManager::new_test().unwrap();

        db.add_clipboard_item("text content", "text").unwrap();
        db.add_clipboard_item("image data", "image").unwrap();

        let history = db.get_clipboard_history().unwrap();
        assert_eq!(history.len(), 2);

        let text_item = history
            .iter()
            .find(|item| item.content_type == "text")
            .unwrap();
        let image_item = history
            .iter()
            .find(|item| item.content_type == "image")
            .unwrap();

        assert_eq!(text_item.content, "text content");
        assert_eq!(image_item.content, "image data");
    }
}
