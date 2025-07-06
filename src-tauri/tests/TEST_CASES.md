# Test Cases Documentation

This document provides a comprehensive overview of all test cases in the Clipray backend application.

## Overview

The test suite consists of **35 test cases** organized into three main categories:

- **Database Tests**: 15 tests covering core database operations
- **Types Tests**: 9 tests validating data structures and serialization
- **Complex Tests**: 11 tests covering multi-step workflows and edge cases

## Test Structure

```
src-tauri/src/libs/test/
├── database_tests.rs    # Core database operations (15 tests)
├── types_tests.rs       # Data structure validation (9 tests)
├── complex_tests.rs     # Complex workflows (11 tests)
└── mod.rs              # Test module management
```

## 1. Database Tests (`database_tests.rs`)

### 1.1 Database Initialization Tests

| Test Name                | Purpose                             | Assertions                                                                                     |
| ------------------------ | ----------------------------------- | ---------------------------------------------------------------------------------------------- |
| `test_new_test_database` | Verify test database initialization | - Default config values are set<br>- Tables are created properly<br>- Default theme is applied |

### 1.2 Basic CRUD Operations

| Test Name                      | Purpose                             | Assertions                                                                                                      |
| ------------------------------ | ----------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `test_add_clipboard_item`      | Test adding a single clipboard item | - Item is added successfully<br>- Content and type are stored correctly<br>- Item is not pinned by default      |
| `test_add_duplicate_content`   | Test duplicate content handling     | - Only one instance of duplicate content exists<br>- Latest timestamp is preserved                              |
| `test_get_clipboard_history`   | Test retrieving clipboard history   | - Items are returned in correct order<br>- Pinned items appear first<br>- Non-pinned items ordered by timestamp |
| `test_delete_clipboard_item`   | Test deleting existing items        | - Item is removed from database<br>- History count decreases<br>- Deleted item is not in results                |
| `test_delete_nonexistent_item` | Test deleting non-existent items    | - Operation succeeds gracefully<br>- No error is thrown<br>- Database remains unchanged                         |

### 1.3 Pin/Unpin Operations

| Test Name               | Purpose                     | Assertions                                                            |
| ----------------------- | --------------------------- | --------------------------------------------------------------------- |
| `test_toggle_pin`       | Test pinning an item        | - Item pin status changes<br>- Pinned item appears first in history   |
| `test_toggle_pin_twice` | Test pin toggle reliability | - Double toggle returns to original state<br>- Pin status consistency |

### 1.4 History Management

| Test Name                                | Purpose                       | Assertions                                                                                              |
| ---------------------------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------------- |
| `test_max_history_limit`                 | Test history count limits     | - Only specified number of items kept<br>- Latest items are preserved<br>- Oldest items are removed     |
| `test_pinned_items_not_deleted_by_limit` | Test pinned item preservation | - Pinned items survive history limits<br>- Pinned items appear first<br>- Database integrity maintained |

### 1.5 Configuration Management

| Test Name                    | Purpose                            | Assertions                                                                                 |
| ---------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------ |
| `test_config_management`     | Test configuration CRUD operations | - Default config is correct<br>- Config updates persist<br>- All config fields are updated |
| `test_get_hotkey`            | Test hotkey retrieval and updates  | - Default hotkey is returned<br>- Hotkey updates are persisted                             |
| `test_get_max_history_count` | Test history count configuration   | - Default count is correct<br>- Count updates are applied                                  |
| `test_get_theme`             | Test theme configuration           | - Default theme is applied<br>- Theme updates are persisted                                |

### 1.6 Content Type Handling

| Test Name                      | Purpose                  | Assertions                                                                                                 |
| ------------------------------ | ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| `test_different_content_types` | Test mixed content types | - Text items are handled correctly<br>- Image items are handled correctly<br>- Content types are preserved |

## 2. Types Tests (`types_tests.rs`)

### 2.1 Theme Preset Tests

| Test Name                     | Purpose                             | Assertions                                                                                      |
| ----------------------------- | ----------------------------------- | ----------------------------------------------------------------------------------------------- |
| `test_theme_preset_as_str`    | Test theme preset string conversion | - All presets convert to correct strings<br>- String format is consistent                       |
| `test_theme_preset_from_str`  | Test string to theme preset parsing | - Valid strings parse correctly<br>- Invalid strings return None<br>- Empty strings are handled |
| `test_theme_preset_roundtrip` | Test bidirectional conversion       | - Preset → String → Preset consistency<br>- No data loss in conversion                          |

### 2.2 Data Structure Creation

| Test Name                      | Purpose                      | Assertions                                                          |
| ------------------------------ | ---------------------------- | ------------------------------------------------------------------- |
| `test_theme_config_creation`   | Test ThemeConfig structure   | - Structure is created correctly<br>- Fields are assigned properly  |
| `test_app_config_creation`     | Test AppConfig structure     | - All fields are set correctly<br>- Nested structures work properly |
| `test_clipboard_item_creation` | Test ClipboardItem structure | - All fields are initialized<br>- Data types are correct            |

### 2.3 Serialization Tests

| Test Name                           | Purpose                               | Assertions                                                                              |
| ----------------------------------- | ------------------------------------- | --------------------------------------------------------------------------------------- |
| `test_clipboard_item_serialization` | Test ClipboardItem JSON serialization | - Serialization succeeds<br>- Deserialization is accurate<br>- All fields are preserved |
| `test_theme_config_serialization`   | Test ThemeConfig JSON serialization   | - Theme presets serialize correctly<br>- Roundtrip serialization works                  |
| `test_app_config_serialization`     | Test AppConfig JSON serialization     | - Complex nested structures serialize<br>- All configuration fields preserved           |

## 3. Complex Tests (`complex_tests.rs`)

These tests combine multiple operations to verify complex workflows and integration scenarios.

### 3.1 Operations Matrix

| Test Function                             | Add Item | Get History | Pin Item | Delete Item | Update Config | Check Config | Set Limit | Check Types |
| ----------------------------------------- | -------- | ----------- | -------- | ----------- | ------------- | ------------ | --------- | ----------- |
| `test_complete_workflow_add_pin_delete`   | ○        | ○           | ○        | ○           | -             | -            | -         | -           |
| `test_multiple_operations_sequence`       | ○        | ○           | ○        | -           | -             | -            | -         | -           |
| `test_config_update_and_persistence`      | -        | -           | -        | -           | ○             | ○            | -         | -           |
| `test_theme_configuration_changes`        | -        | -           | -        | -           | ○             | ○            | -         | -           |
| `test_history_limit_enforcement`          | ○        | ○           | -        | -           | ○             | -            | ○         | -           |
| `test_pinned_items_survive_history_limit` | ○        | ○           | ○        | -           | ○             | -            | ○         | -           |
| `test_duplicate_content_replacement`      | ○        | ○           | -        | -           | -             | -            | -         | -           |
| `test_mixed_content_types_handling`       | ○        | ○           | -        | -           | -             | -            | -         | ○           |
| `test_empty_database_operations`          | -        | ○           | ○        | ○           | -             | -            | -         | -           |
| `test_large_content_storage`              | ○        | ○           | -        | -           | -             | -            | -         | -           |
| `test_special_characters_support`         | ○        | ○           | -        | -           | -             | -            | -         | -           |

### 3.2 Test Categories and Details

#### 3.2.1 Workflow Tests (2 tests)

- **`test_complete_workflow_add_pin_delete`**: Tests complete item lifecycle
- **`test_multiple_operations_sequence`**: Tests multiple operations without interference

#### 3.2.2 Configuration Integration Tests (2 tests)

- **`test_config_update_and_persistence`**: Tests configuration updates and persistence
- **`test_theme_configuration_changes`**: Tests theme switching functionality

#### 3.2.3 History Management Tests (2 tests)

- **`test_history_limit_enforcement`**: Tests history count limits
- **`test_pinned_items_survive_history_limit`**: Tests pinned item protection

#### 3.2.4 Content Handling Tests (2 tests)

- **`test_duplicate_content_replacement`**: Tests duplicate content logic
- **`test_mixed_content_types_handling`**: Tests multiple content types

#### 3.2.5 Edge Cases and Robustness Tests (3 tests)

- **`test_empty_database_operations`**: Tests operations on empty database
- **`test_large_content_storage`**: Tests large content handling
- **`test_special_characters_support`**: Tests Unicode and special characters

### 3.3 Operation Coverage Summary

| Operation         | Usage Count | Test Functions                                                                                                                                                                                                                                                                                        |
| ----------------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Add Item**      | 7           | `test_complete_workflow_add_pin_delete`, `test_multiple_operations_sequence`, `test_history_limit_enforcement`, `test_pinned_items_survive_history_limit`, `test_duplicate_content_replacement`, `test_mixed_content_types_handling`, `test_large_content_storage`, `test_special_characters_support` |
| **Get History**   | 8           | All except `test_config_update_and_persistence`, `test_theme_configuration_changes`                                                                                                                                                                                                                   |
| **Pin Item**      | 4           | `test_complete_workflow_add_pin_delete`, `test_multiple_operations_sequence`, `test_pinned_items_survive_history_limit`, `test_empty_database_operations`                                                                                                                                             |
| **Delete Item**   | 2           | `test_complete_workflow_add_pin_delete`, `test_empty_database_operations`                                                                                                                                                                                                                             |
| **Update Config** | 4           | `test_config_update_and_persistence`, `test_theme_configuration_changes`, `test_history_limit_enforcement`, `test_pinned_items_survive_history_limit`                                                                                                                                                 |
| **Check Config**  | 2           | `test_config_update_and_persistence`, `test_theme_configuration_changes`                                                                                                                                                                                                                              |
| **Set Limit**     | 2           | `test_history_limit_enforcement`, `test_pinned_items_survive_history_limit`                                                                                                                                                                                                                           |
| **Check Types**   | 1           | `test_mixed_content_types_handling`                                                                                                                                                                                                                                                                   |

### 3.4 Complex Test Summary

| Test Category                 | Test Count | Primary Focus         | Key Validations                             |
| ----------------------------- | ---------- | --------------------- | ------------------------------------------- |
| **Workflow Tests**            | 2          | Multi-step operations | Operation sequencing, state consistency     |
| **Configuration Integration** | 2          | Config persistence    | Setting updates, theme switching            |
| **History Management**        | 2          | Limit enforcement     | History limits, pinned item protection      |
| **Content Handling**          | 2          | Data integrity        | Duplicate handling, mixed content types     |
| **Edge Cases**                | 3          | Robustness            | Empty state, large data, special characters |
| **Total**                     | **11**     | **Complex Workflows** | **End-to-end functionality**                |

## 4. Test Features

### 4.1 Test Infrastructure

- **In-Memory Database**: Uses SQLite `:memory:` for fast, isolated tests
- **Serial Execution**: Uses `serial_test` crate to prevent race conditions
- **Async Support**: Supports both sync and async test functions
- **Feature Flags**: Tests run with `--features test-utils` flag

### 4.2 Test Data Management

- **Isolation**: Each test uses a fresh database instance
- **Test Data**: `setup_test_data()` provides consistent test fixtures
- **Cleanup**: Automatic cleanup between tests

### 4.3 Assertion Patterns

- **State Verification**: Tests verify database state after operations
- **Error Handling**: Tests ensure graceful error handling
- **Data Integrity**: Tests verify data consistency and integrity
- **Performance**: Tests ensure operations complete in reasonable time

## 5. Running Tests

### 5.1 All Tests

```bash
npm run test
```

### 5.2 Specific Test Modules

```bash
# Database tests only
cargo test --features test-utils libs::test::database_tests

# Types tests only
cargo test --features test-utils libs::test::types_tests

# Complex tests only
cargo test --features test-utils libs::test::complex_tests
```

### 5.3 Specific Test Functions

```bash
# Run a specific test
cargo test --features test-utils test_add_clipboard_item

# Run with verbose output
cargo test --features test-utils -- --nocapture
```

## 6. Test Coverage Summary

| Category       | Test Count | Coverage Area                                 |
| -------------- | ---------- | --------------------------------------------- |
| Database Tests | 15         | Core database operations, CRUD, configuration |
| Types Tests    | 9          | Data structures, serialization, validation    |
| Complex Tests  | 11         | Workflows, integration, edge cases            |
| **Total**      | **35**     | **Complete backend functionality**            |

## 7. Best Practices

### 7.1 Test Design

- Each test focuses on a single concern
- Tests are independent and can run in any order
- Clear test names describe the behavior being tested
- Comprehensive assertions verify all expected outcomes

### 7.2 Maintenance

- Tests are updated when functionality changes
- New features include corresponding tests
- Test documentation is kept current
- Performance implications are considered

### 7.3 Debugging

- Tests provide clear failure messages
- Test data is predictable and debuggable
- Verbose output available for troubleshooting
- Serial execution prevents timing issues

This comprehensive test suite ensures the reliability, correctness, and robustness of the Clipray backend application.
