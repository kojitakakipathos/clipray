# Clipray 📋

[![Publish](https://github.com/kojitakakipathos/clipray/actions/workflows/release.yml/badge.svg)](https://github.com/kojitakakipathos/clipray/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A lightweight, cross-platform clipboard manager built with **Tauri v2** and **React**. Clipray keeps clipboard history locally (SQLite) and opens a floating window via a global shortcut (default: `Ctrl+Shift+V` on Windows/Linux).

## ✨ Features

- **📋 Clipboard history**: Saves text and image clipboard content locally
- **⚡ Quick access**: Global shortcut to show/hide the window (default: `Ctrl+Shift+V`)
- **🔍 Search**: Filter history in real time
- **📌 Pin items**: Pin important entries so they are not trimmed by the history limit
- **🖼️ Images**: Store and preview clipboard images (as local data)
- **🎨 Themes**: Built-in theme presets
- **⚙️ Configurable**: History limit, hotkey, theme, and start-on-boot (where supported)
- **🔒 Local-first**: Clipboard data stays on your machine; see [Privacy](#privacy) below
- **🌐 Cross-platform**: Windows, macOS, and Linux (build targets depend on your release pipeline)

## 🚀 Installation

### Pre-built binaries

Download builds from [Releases](https://github.com/kojitakakipathos/clipray/releases) when available. Asset names may vary by CI configuration and Tauri bundle format (e.g. `.msi`, `.dmg`, `.AppImage`, `.deb`).

### Build from source

#### Prerequisites

- [Node.js](https://nodejs.org/) **18+** recommended (this repo uses Vite 6)
- [Rust](https://rustup.rs/) (latest stable)
- [Tauri v2 system prerequisites](https://tauri.app/start/prerequisites/) (especially on Linux: WebKitGTK and related packages)

You do **not** need a global Tauri CLI install for normal development: the CLI is pulled in via npm (`@tauri-apps/cli`) when you run the scripts below.

#### Steps

```bash
git clone https://github.com/kojitakakipathos/clipray.git
cd clipray
npm install
npm run build:release
```

Release bundles are produced under `src-tauri/target/release/bundle/` (paths depend on OS).

## 📖 Usage

### Basic usage

1. **Start Clipray** and keep it running (tray/background behavior depends on platform)
2. **Copy** text or images as usual (`Ctrl+C` / `Cmd+C`)
3. **Open history** with your configured shortcut (default `Ctrl+Shift+V`)
4. **Navigate** with arrow keys or the mouse
5. **Paste** the selected item with `Enter` (or use the UI actions shown in the app)

### Keyboard shortcuts (in-app)

These shortcuts are handled in the desktop window (see `src/hooks/useKeyboardNavigation.ts`). On macOS, **`Control`** is used for `Ctrl+*` bindings below (not `Command`).

| Shortcut        | Action                         |
| --------------- | -------------------------------- |
| _Configured_    | Open/toggle clipboard window (global; default `Ctrl+Shift+V`) |
| `↑` / `↓`       | Move selection                   |
| `Enter`         | Copy selected item and hide window |
| `Delete`        | Delete selected item             |
| `Ctrl+P`        | Pin / unpin selected item        |
| `Ctrl+Tab`      | Switch between History / Pinned tabs |
| `Ctrl+I`        | Open / close settings            |
| `Escape`        | Close settings or hide window    |

### Settings

Open settings with **`Ctrl+I`** or the gear icon in the header.

- **History limit**: Maximum items to keep
- **Global hotkey**: Shortcut registered with the OS (examples: `CommandOrControl+Shift+V`, `Alt+V`, `Ctrl+Space`)
- **Theme**: UI preset
- **Start on boot**: Launch with the system (platform-dependent)

## 🛠️ Development

### Development setup

1. Install prerequisites above (Node **18+**, Rust stable, and Tauri system dependencies).
2. Install JavaScript dependencies:

```bash
git clone https://github.com/kojitakakipathos/clipray.git
cd clipray
npm install
```

3. Run the full desktop app in dev mode (frontend + Rust backend, hot reload):

```bash
npm run tauri dev
```

4. (Optional) Frontend-only Vite server:

```bash
npm run dev
```

This serves the React UI only; Tauri IPC and clipboard features need `npm run tauri dev`.

### Project structure

```
clipray/
├── src/                 # React (Vite) frontend
│   ├── components/
│   ├── hooks/
│   ├── types/
│   └── utils/
├── src-tauri/           # Rust + Tauri backend
│   ├── src/libs/        # Clipboard, SQLite, commands, etc.
│   └── tauri.conf.json
├── package.json
└── package-lock.json
```

### Scripts

| Script | Description |
| ------ | ----------- |
| `npm run dev` | Vite dev server (UI only) |
| `npm run build` | Typecheck + production frontend build |
| `npm run tauri dev` | Tauri dev mode (recommended for feature work) |
| `npm run build:release` | Production Tauri build for the current platform |
| `npm run test` | Rust tests (`src-tauri`, `test-utils` feature) |

Run a single Rust test:

```bash
cd src-tauri && cargo test --features test-utils -- <test_name>
```

High-level architecture notes for contributors: [CLAUDE.md](CLAUDE.md).

### Privacy

Clipray stores clipboard history in a **local SQLite database** on your device. This repository’s application code is intended to operate **without sending clipboard contents to remote servers**. If you find behavior that contradicts this, please open an issue.

## 🤝 Contributing

Contributions are welcome.

1. Fork the repository
2. Create a branch: `git checkout -b feature/your-change`
3. Make changes and run `npm run test` (and manual testing with `npm run tauri dev`)
4. Open a Pull Request

For bugs, include OS version, app version, and steps to reproduce.

## 📝 License

This project is licensed under the MIT License — see [LICENSE](LICENSE).

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/)
- [React](https://react.dev/) and [TypeScript](https://www.typescriptlang.org/)
- [Lucide](https://lucide.dev/)
- [SQLite](https://sqlite.org/)

## 🔗 Links

- [Issues](https://github.com/kojitakakipathos/clipray/issues)
- [Changelog](CHANGELOG.md)
