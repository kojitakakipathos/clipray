# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Clipray is a cross-platform desktop clipboard manager built with Tauri v2 (Rust backend) + React 18 (TypeScript frontend). It monitors the system clipboard in real-time, stores history in local SQLite, and provides a global hotkey (default: Ctrl+Shift+V) to toggle the UI.

## Commands

```bash
# Development
npm run tauri dev        # Run with hot reload (frontend + backend)
npm run dev              # Frontend-only Vite dev server

# Build
npm run build            # TypeScript compile + Vite build (frontend only)
npm run build:release    # Full Tauri release build for current platform

# Tests (Rust backend only)
npm run test             # cargo test --features test-utils in src-tauri/
cd src-tauri && cargo test --features test-utils -- <test_name>  # Run single test
```

No frontend tests exist. All tests are Rust unit tests in `src-tauri/src/libs/test/`.

## Architecture

### Frontend → Backend Communication

The frontend calls Tauri IPC commands via `invoke()` from `@tauri-apps/api/core`. All commands are defined in `src-tauri/src/libs/commands.rs` and registered in `main.rs`.

Key IPC commands: `get_clipboard_history`, `delete_clipboard_item`, `toggle_pin`, `copy_to_clipboard`, `copy_and_hide`, `get_config`, `update_config`, `show_window`, `hide_window`, `exit_app`.

Backend emits events to frontend: `clipboard-updated` (new item added) and `show-clipboard` (hotkey triggered).

### Data Flow

1. Rust async task in `clipboard.rs` polls the OS clipboard on an interval
2. New items are saved to SQLite via `database.rs`
3. `clipboard-updated` event is emitted to the frontend
4. Frontend (`useClipboard.ts` hook) receives the event and re-fetches history

### Frontend State

State management lives in `src/hooks/useClipboard.ts`. The main `App.tsx` holds minimal UI state (tab selection, search query). Keyboard navigation is handled by `useKeyboardNavigation.ts`, and theming by `useTheme.ts`.

### Database

SQLite stored locally via `rusqlite`. Two tables:
- `clipboard_history`: id, content, content_type (`"text"` or `"image"`), timestamp, pinned
- `app_config`: key-value store for settings (max_history_count, hotkey, theme)

Images are stored as Base64-encoded strings.

### Window Behavior

The window is frameless, always-on-top, 400×500px, hidden by default, and skips the taskbar. It appears/disappears via the global hotkey without appearing in the taskbar or alt-tab list.

## Code Conventions

- All code comments must be written in English (enforced by `.cursor/rules/comment.mdc`)
- Rust modules live under `src-tauri/src/libs/`
- TypeScript strict mode is enabled; unused variables are errors
- Theme constants/enums are defined in `src/types/theme.ts`
