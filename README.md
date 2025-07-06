# Clipray 📋

[![Test](https://github.com/yourusername/clipray/workflows/Test/badge.svg)](https://github.com/yourusername/clipray/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/yourusername/clipray/branch/main/graph/badge.svg)](https://codecov.io/gh/yourusername/clipray)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight, cross-platform clipboard manager built with Tauri and React. Clipray provides a seamless clipboard history experience similar to Windows' "Windows + V" functionality, but available on Windows, macOS, and Linux.

## ✨ Features

- **📋 Clipboard History**: Automatically saves text and image clipboard content
- **⚡ Quick Access**: Invoke with customizable global hotkeys (default: `Ctrl+Shift+V`)
- **🔍 Smart Search**: Filter clipboard history with real-time search
- **📌 Pin Items**: Mark important items as favorites to prevent auto-deletion
- **🖼️ Image Support**: Store and preview clipboard images
- **🎨 Theme Support**: Multiple built-in themes for personalization
- **⚙️ Configurable**: Customize history limit, hotkeys, and appearance
- **🔒 Privacy-First**: All data stored locally, no external data transmission
- **🌐 Cross-Platform**: Native support for Windows, macOS, and Linux

## 🚀 Installation

### Download Pre-built Binaries

Visit the [Releases](https://github.com/yourusername/clipray/releases) page to download the latest version for your platform:

- **Windows**: `clipray-setup.exe`
- **macOS**: `clipray.dmg`
- **Linux**: `clipray.AppImage` or `clipray.deb`

### Build from Source

#### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

#### Steps

1. Clone the repository:

```bash
git clone https://github.com/yourusername/clipray.git
cd clipray
```

2. Install dependencies:

```bash
npm install
```

3. Build the application:

```bash
npm run tauri --release
```

The built application will be available in `src-tauri/target/release/bundle/`.

## 📖 Usage

### Basic Usage

1. **Start Clipray**: Launch the application (it will run in the background)
2. **Copy Content**: Copy text or images as usual (`Ctrl+C` / `Cmd+C`)
3. **Access History**: Press `Ctrl+Shift+V` (or your configured hotkey)
4. **Select Item**: Use arrow keys or click to select from history
5. **Paste**: Press `Enter` or click to paste the selected item

### Keyboard Shortcuts

| Shortcut       | Action                  |
| -------------- | ----------------------- |
| `Ctrl+Shift+V` | Open clipboard history  |
| `↑/↓`          | Navigate through items  |
| `Enter`        | Paste selected item     |
| `Delete`       | Delete selected item    |
| `Ctrl+P`       | Pin/unpin selected item |
| `Escape`       | Close window            |
| `Ctrl+I`       | Open settings           |

### Settings

Access settings by pressing `Ctrl+S` or clicking the gear icon:

- **History Limit**: Set maximum number of items to store (default: 50)
- **Global Hotkey**: Customize the activation shortcut
- **Theme**: Choose from available themes
- **Auto-start**: Enable/disable startup with system

## 🛠️ Development

### Development Setup

1. Clone and install dependencies:

```bash
git clone https://github.com/yourusername/clipray.git
cd clipray
npm install
```

2. Start development server:

```bash
npm run tauri dev
```

### Project Structure

```
clipray/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/             # Custom React hooks
│   ├── types/             # TypeScript type definitions
│   └── utils/             # Utility functions
├── src-tauri/             # Rust backend
│   ├── src/libs/          # Core functionality
│   │   ├── clipboard.rs   # Clipboard monitoring
│   │   ├── database.rs    # SQLite database
│   │   └── commands.rs    # Tauri commands
│   └── tauri.conf.json    # Tauri configuration
└── package.json           # Node.js dependencies
```

### Available Scripts

#### Development

- `npm run dev` - Start Vite development server
- `npm run build` - Build frontend for production
- `npm run tauri dev` - Start Tauri development mode
- `npm run build:release` - Build release version

#### Testing

- `npm run test` - Run all tests

## 🤝 Contributing

We welcome contributions! Please follow these guidelines:

### Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Test thoroughly
5. Commit with clear messages: `git commit -m 'Add amazing feature'`
6. Push to your fork: `git push origin feature/amazing-feature`
7. Open a Pull Request

### Development Guidelines

- Follow the existing code style
- Write clear commit messages
- Add tests for new features
- Update documentation as needed
- Ensure cross-platform compatibility

### Bug Reports

When reporting bugs, please include:

- Operating system and version
- Clipray version
- Steps to reproduce
- Expected vs actual behavior
- Screenshots (if applicable)

### Feature Requests

We love new ideas! Please:

- Check existing issues first
- Describe the feature clearly
- Explain the use case
- Consider implementation complexity

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/) for cross-platform desktop apps
- UI powered by [React](https://reactjs.org/) and [TypeScript](https://www.typescriptlang.org/)
- Icons from [Lucide](https://lucide.dev/)
- Database powered by [SQLite](https://sqlite.org/)

## 🔗 Links

- [Documentation](https://github.com/yourusername/clipray/wiki)
- [Issue Tracker](https://github.com/yourusername/clipray/issues)
- [Discussions](https://github.com/yourusername/clipray/discussions)
- [Changelog](CHANGELOG.md)

---

<p align="center">
  Made with ❤️ by the Clipray team
</p>
