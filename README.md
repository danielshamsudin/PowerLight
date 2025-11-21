# ⚡ Powerlight

A blazingly fast, lightweight application launcher for Windows built with Rust and Tauri.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-blue)

## ✨ Features

- **🚀 Lightning Fast** - Built with Rust for instant search results
- **🔍 Fuzzy Search** - Find apps even with typos or partial names
- **⌨️ Global Hotkey** - Press `Ctrl+Alt+Space` to launch from anywhere
- **🎨 Modern UI** - Clean, minimal dark interface with transparency
- **💾 Lightweight** - Small memory footprint, runs in system tray
- **📦 Auto-Indexing** - Automatically indexes Start Menu applications
- **🎯 Keyboard Navigation** - Arrow keys to navigate, Enter to launch, Esc to close

## 📥 Installation

### Download Prebuilt Binary

Download the latest release from the [Releases page](https://github.com/danielshamsudin/powerlight/releases):

- **NSIS Installer** (`.exe`) - Recommended for most users
- **MSI Installer** (`.msi`) - For enterprise deployment

### Build from Source

**Prerequisites:**
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)

**Steps:**
```bash
# Clone the repository
git clone https://github.com/danielshamsudin/powerlight.git
cd powerlight

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

The built executables will be in `src-tauri/target/release/bundle/`.

## 🎯 Usage

1. **Launch** - Press `Ctrl+Alt+Space` (or click the system tray icon)
2. **Search** - Start typing the name of an application
3. **Navigate** - Use ↑/↓ arrow keys to select
4. **Open** - Press Enter or click to launch
5. **Close** - Press Esc or click outside

### System Tray

Powerlight runs in the system tray when not in use. Right-click the icon for:
- **Show Powerlight** - Open the search window
- **Quit** - Exit the application

## ⚙️ Configuration

### Changing the Hotkey

Currently, the hotkey is hardcoded. To change it:

1. Edit `src-tauri/src/lib.rs`
2. Find the `shortcuts_to_try` array (around line 63)
3. Modify the key combinations
4. Rebuild the app

Future releases will include a settings UI for this.

## 🏗️ Project Structure

```
powerlight/
├── src/                  # Frontend (TypeScript/HTML/CSS)
│   ├── main.ts          # Main application logic
│   ├── styles.css       # UI styling
│   └── index.html       # HTML template
├── src-tauri/           # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs       # Main Tauri app setup
│   │   └── search.rs    # Search & indexing logic
│   ├── icons/           # App icons
│   └── Cargo.toml       # Rust dependencies
└── .github/workflows/   # CI/CD pipelines
```

## 🛠️ Technologies

- **[Tauri](https://tauri.app/)** - Desktop app framework
- **[Rust](https://www.rust-lang.org/)** - Backend language
- **[TypeScript](https://www.typescriptlang.org/)** - Frontend language
- **[Vite](https://vitejs.dev/)** - Build tool
- **[fuzzy-matcher](https://crates.io/crates/fuzzy-matcher)** - Fuzzy search algorithm

## 🗺️ Roadmap

- [ ] File search (documents, images, etc.)
- [ ] Calculator functionality
- [ ] Web search integration
- [ ] Settings UI (customize hotkey, theme)
- [ ] Auto-updater
- [ ] Search history
- [ ] Usage analytics (rank by frequency)
- [ ] Plugin system

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [PowerToys Run](https://github.com/microsoft/PowerToys) and macOS Spotlight
- Built with the amazing [Tauri](https://tauri.app/) framework

## 📧 Support

If you encounter any issues or have questions:
- Open an [Issue](https://github.com/danielshamsudin/powerlight/issues)
- Check existing issues for solutions

---

Made with ⚡ by [Daniel Shamsudin]
