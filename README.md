# Plex Media Organizer

[![Crates.io Stable](https://img.shields.io/badge/crates.io-0.1.1-orange)](https://crates.io/crates/plex-media-organizer)
[![License](https://img.shields.io/github/license/lijunzh/plex-media-organizer)](https://github.com/lijunzh/plex-media-organizer/blob/main/LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-linux%20%7C%20macos%20%7C%20windows-lightgrey)](https://github.com/lijunzh/plex-media-organizer/releases)
[![Nightly](https://img.shields.io/github/actions/workflow/status/lijunzh/plex-media-organizer/nightly.yml?label=nightly%20build)](https://github.com/lijunzh/plex-media-organizer/actions/workflows/nightly.yml)
[![Crates.io Nightly](https://img.shields.io/badge/crates.io-nightly-blue)](https://crates.io/crates/plex-media-organizer)

A powerful, intelligent media file organizer that follows Plex naming conventions. Built in Rust for performance and reliability.

## 🚀 Quick Start

### Installation

**Cargo (Recommended):**
```bash
# Install stable version
cargo install plex-media-organizer

# Install specific stable version
cargo install plex-media-organizer --version "0.1.1"

# Install latest nightly (preview of next version)
cargo install plex-media-organizer --version "0.1.2-nightly.20241201.a1b2c3d"
```

**Homebrew (macOS/Linux):**
```bash
brew install lijunzh/plex-media-organizer/plex-media-organizer
```

**Pre-built Binaries:**
Download from [GitHub Releases](https://github.com/lijunzh/plex-media-organizer/releases)

### First Steps

```bash
# Setup configuration
plex-media-organizer setup

# Scan a directory
plex-media-organizer scan /path/to/movies

# Organize files (preview first!)
plex-media-organizer organize /path/to/movies --preview
```

## ✨ Features

- **Intelligent Parsing**: Advanced filename parsing with confidence scoring
- **Plex Compatibility**: Follows Plex naming conventions for optimal media server integration
- **Multi-language Support**: Handles English, Chinese, Japanese, Arabic, and other languages
- **TMDB Integration**: Enhanced metadata using The Movie Database API
- **Database Backed**: Persistent operation history with rollback capabilities
- **Safety First**: Preview mode and rollback support for all operations

## 📋 Commands

- **`setup`** - Interactive configuration setup
- **`scan`** - Analyze media directories
- **`test`** - Test parsing functionality
- **`organize`** - Organize media files
- **`rollback`** - Revert previous operations
- **`cleanup`** - Database maintenance

## 📁 Supported Formats

- **Video**: `.mkv`, `.mp4`, `.avi`, `.mov`, `.wmv`
- **Languages**: English, Chinese, Japanese, Arabic, Russian, and more
- **Platforms**: Linux, macOS, Windows

## 📚 Documentation

- [User Guide](docs/user-guide/) - Complete documentation
- [CLI Commands](docs/user-guide/cli-commands.md) - Command reference
- [Configuration](docs/user-guide/configuration.md) - Setup options

## 🤝 Contributing

We welcome contributions! See [Contributing Guide](CONTRIBUTING.md) for details.

## 📝 License

MIT License - see [LICENSE](LICENSE) for details.

---

**Made with ❤️ in Rust**
