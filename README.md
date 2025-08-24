# Plex Media Organizer

[![Crates.io](https://img.shields.io/crates/v/plex-media-organizer)](https://crates.io/crates/plex-media-organizer)
[![Crates.io](https://img.shields.io/crates/d/plex-media-organizer)](https://crates.io/crates/plex-media-organizer)
[![License](https://img.shields.io/github/license/lijunzh/plex-media-organizer)](https://github.com/lijunzh/plex-media-organizer/blob/main/LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Platforms](https://img.shields.io/badge/platforms-linux%20%7C%20macos%20%7C%20windows-lightgrey)](https://github.com/lijunzh/plex-media-organizer/releases)
[![Nightly](https://img.shields.io/github/actions/workflow/status/lijunzh/plex-media-organizer/nightly.yml?label=nightly%20build)](https://github.com/lijunzh/plex-media-organizer/actions/workflows/nightly.yml)
[![Quality](https://img.shields.io/github/actions/workflow/status/lijunzh/plex-media-organizer/quality.yml?label=quality%20checks)](https://github.com/lijunzh/plex-media-organizer/actions/workflows/quality.yml)

A powerful, intelligent media file organizer that follows Plex naming conventions. Built in Rust for performance and reliability.

## 🎬 Features

- **Intelligent Parsing**: Advanced filename parsing with confidence scoring
- **Plex Compatibility**: Follows Plex naming conventions for optimal media server integration
- **Multi-language Support**: Handles English, Chinese, Japanese, Arabic, and other languages
- **TMDB Integration**: Enhanced metadata using The Movie Database API
- **Database Backed**: Persistent operation history with rollback capabilities

## 🚀 Quick Start

### Installation

#### Option 1: Install from Cargo (Recommended)
```bash
# Install the latest version
cargo install plex-media-organizer

# Update to the latest version
cargo install --force plex-media-organizer
```

#### Option 2: Download Pre-built Binaries
Download the latest release for your platform from [GitHub Releases](https://github.com/lijunzh/plex-media-organizer/releases):

- **macOS (Intel/Apple Silicon)**: `plex-media-organizer-v1.0.0-x86_64-apple-darwin.tar.gz` / `plex-media-organizer-v1.0.0-aarch64-apple-darwin.tar.gz`
- **Linux**: `plex-media-organizer-v1.0.0-x86_64-unknown-linux-gnu.tar.gz`
- **Windows**: `plex-media-organizer-v1.0.0-x86_64-pc-windows-msvc.zip`

```bash
# Extract and add to your PATH
tar -xzf plex-media-organizer-v1.0.0-x86_64-apple-darwin.tar.gz
sudo mv plex-media-organizer /usr/local/bin/
```

### First Time Setup

```bash
# Run the interactive setup
plex-media-organizer setup

# Scan a directory to see what's there
plex-media-organizer scan /path/to/movies

# Test parsing without making changes
plex-media-organizer test /path/to/movies

# Organize files (preview first!)
plex-media-organizer organize /path/to/movies --preview
```

## 📋 Commands

- **`setup`** - Interactive configuration setup
- **`scan`** - Analyze media directories
- **`test`** - Test parsing functionality
- **`organize`** - Organize media files
- **`rollback`** - Revert previous operations
- **`cleanup`** - Database maintenance

For detailed command documentation, see [CLI Commands Reference](docs/user-guide/cli-commands.md).

## 📁 Supported File Types

- **Movies**: `.mkv`, `.mp4`, `.avi`, `.mov`, `.wmv`
- **TV Shows**: `.mkv`, `.mp4`, `.avi`, `.mov`
- **Documentaries**: `.mkv`, `.mp4`, `.avi`
- **Anime**: `.mkv`, `.mp4`, `.avi`

## 🌍 Language Support

- **English**: Primary language with full support
- **Chinese**: Simplified and Traditional Chinese
- **Japanese**: Hiragana, Katakana, and Kanji
- **Arabic**: Arabic script support
- **Russian**: Cyrillic script support
- **Other Languages**: Latin-based scripts

## 🔧 Configuration

Configuration is stored in `~/.config/plex-media-organizer/config.toml` (Linux/macOS) or `%APPDATA%\plex-media-organizer\config.toml` (Windows).

For detailed configuration options, see [Configuration Guide](docs/user-guide/configuration.md).

## 🛠️ Development

```bash
# Clone and build
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release

# Run tests
cargo test
```

For development setup and contributing guidelines, see [Contributing Guide](CONTRIBUTING.md).

## 📄 Performance

- **Fast Parsing**: Optimized algorithms for quick filename analysis
- **Parallel Processing**: Configurable parallel operations for large directories
- **Intelligent Caching**: Database-backed caching for repeated operations
- **Memory Efficient**: Minimal memory footprint for large media libraries

## 🔒 Safety Features

- **Preview Mode**: Test operations before making changes
- **Rollback Support**: Revert any operation with full history
- **Error Handling**: Graceful handling of file system errors
- **Validation**: Comprehensive input validation and error checking

## 📚 Documentation

- [User Guide](docs/user-guide/) - Complete user documentation
- [CLI Commands](docs/user-guide/cli-commands.md) - Detailed command reference
- [Configuration](docs/user-guide/configuration.md) - Configuration options
- [Technical Terms](docs/user-guide/technical-terms.md) - Customizing parsing behavior
- [Contributing](CONTRIBUTING.md) - Development and contribution guidelines

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/lijunzh/plex-media-organizer/issues)
- **Discussions**: [GitHub Discussions](https://github.com/lijunzh/plex-media-organizer/discussions)

---

**Made with ❤️ in Rust**
