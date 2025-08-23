# 🎬 Plex Media Organizer v1.0.0

## 🚀 First Stable Release

We're excited to announce the first stable release of Plex Media Organizer! This release delivers a complete, production-ready CLI application for intelligent media file organization.

## ✨ What's New in v1.0.0

### 🎯 Complete CLI Workflow
- **7 Core Commands**: Full workflow from setup to cleanup
- **Database-Backed Operations**: SQLite database with rollback capabilities
- **TMDB Integration**: Enhanced metadata using The Movie Database API
- **Multi-language Support**: English, Chinese, Japanese, Arabic, Russian
- **Safety Features**: Preview mode, rollback, comprehensive error handling
- **Performance Optimization**: Caching, parallel processing, network optimization

### 📋 CLI Commands
1. **`setup`** - Interactive configuration setup
2. **`config`** - View and modify configuration
3. **`scan`** - Analyze media directories
4. **`test`** - Test parsing functionality
5. **`organize`** - Organize media files
6. **`rollback`** - Revert previous operations
7. **`cleanup`** - Database maintenance

### 🎬 Movie Organization Features
- **Intelligent Parsing**: Advanced filename parsing with confidence scoring
- **Plex Compatibility**: Follows Plex naming conventions for optimal media server integration
- **Multi-language Titles**: Handles bilingual titles (e.g., "钢铁侠 - Iron Man")
- **Quality Detection**: Identifies resolution, source, and codec information
- **Series Support**: Detects and handles movie series (Iron Man 1, 2, 3)
- **Anime Support**: Specialized anime pattern detection and organization

### 🔒 Safety & Reliability
- **Preview Mode**: Test operations before making changes
- **Database Rollback**: Complete operation history with rollback capability
- **Error Handling**: Graceful failure recovery and detailed error messages
- **Input Validation**: Robust validation of all inputs and file operations

### 📊 Performance & Scalability
- **Database Caching**: Intelligent caching for improved performance
- **Parallel Processing**: Configurable parallel operations for large directories
- **Network Optimization**: Special handling for network drives
- **Memory Efficiency**: Optimized for large media libraries

## 🛠️ Technical Highlights

### Architecture
- **Modular Design**: Clean separation of concerns with focused modules
- **Database Integration**: SQLite with operation history and caching
- **API Integration**: TMDB API for enhanced movie metadata
- **Comprehensive Testing**: 96+ unit tests with real-world validation

### Supported Platforms
- **macOS**: Native support with optimized performance
- **Linux**: Full compatibility with major distributions
- **Windows**: Cross-platform support (requires Rust toolchain)

### File Formats
- **Movies**: `.mkv`, `.mp4`, `.avi`, `.mov`, `.wmv`
- **Metadata**: XML, NFO, and embedded metadata support
- **Subtitles**: Associated subtitle file handling

## 🚀 Quick Start

### Installation
```bash
# Clone the repository
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer

# Build the project
cargo build --release

# Install (optional)
cargo install --path .
```

### Basic Usage
```bash
# Setup (first time only)
plex-media-organizer setup

# Scan directory
plex-media-organizer scan /path/to/movies

# Test parsing
plex-media-organizer test /path/to/movies --use-cache

# Preview organization
plex-media-organizer organize /path/to/movies --preview

# Organize files
plex-media-organizer organize /path/to/movies

# Cleanup old operations
plex-media-organizer cleanup --keep-count 10
```

## 📚 Documentation

- **[Getting Started](docs/user/getting-started.md)** - Quick start guide
- **[CLI Commands Reference](docs/user-guide/cli-commands.md)** - Complete command documentation
- **[User Guide](docs/user/user-guide.md)** - Comprehensive usage guide
- **[Troubleshooting](docs/user/troubleshooting.md)** - Common issues and solutions

## 🧪 Testing & Quality

- **96+ Unit Tests**: Comprehensive test coverage
- **Integration Tests**: End-to-end workflow testing
- **Real-world Validation**: Tested with 417+ actual movie files
- **Regression Testing**: Ensures no breaking changes
- **Performance Testing**: Optimized for large media libraries

## 🔧 System Requirements

- **Rust**: 1.70+ (for building from source)
- **Memory**: 512MB RAM minimum, 2GB+ recommended for large libraries
- **Storage**: 100MB+ for application and database
- **Network**: Internet connection for TMDB API (optional)

## 🎯 What's Next

### Planned Features
- **TV Show Support**: Episode detection and season organization
- **Web Interface**: Browser-based management interface
- **Scheduled Operations**: Automated organization workflows
- **Cloud Integration**: Google Drive, Dropbox support
- **Music Support**: Music file organization and metadata

### Roadmap
- **v1.1**: TV Show Support
- **v1.2**: Web Interface
- **v1.3**: Advanced Features (scheduling, batch processing)
- **v2.0**: Music Support and Cloud Integration

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
# Fork and clone the repository
git clone https://github.com/your-username/plex-media-organizer.git
cd plex-media-organizer

# Install development dependencies
cargo install cargo-watch  # For development
cargo install cargo-tarpaulin  # For coverage

# Run tests
cargo test

# Run with watch mode
cargo watch -x test
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Plex Media Server**: For naming conventions and inspiration
- **The Movie Database (TMDB)**: For movie metadata API
- **Rust Community**: For excellent tooling and ecosystem
- **Contributors**: Everyone who helped make this release possible

## 🐛 Bug Reports & Support

- **Issues**: [GitHub Issues](https://github.com/lijunzh/plex-media-organizer/issues)
- **Discussions**: [GitHub Discussions](https://github.com/lijunzh/plex-media-organizer/discussions)
- **Documentation**: [User Guide](docs/user-guide/)

---

## 📦 Download

### Pre-built Binaries
- **macOS**: [plex-media-organizer-v1.0.0-x86_64-apple-darwin.tar.gz](https://github.com/lijunzh/plex-media-organizer/releases/download/v1.0.0/plex-media-organizer-v1.0.0-x86_64-apple-darwin.tar.gz)
- **Linux**: [plex-media-organizer-v1.0.0-x86_64-unknown-linux-gnu.tar.gz](https://github.com/lijunzh/plex-media-organizer/releases/download/v1.0.0/plex-media-organizer-v1.0.0-x86_64-unknown-linux-gnu.tar.gz)
- **Windows**: [plex-media-organizer-v1.0.0-x86_64-pc-windows-msvc.zip](https://github.com/lijunzh/plex-media-organizer/releases/download/v1.0.0/plex-media-organizer-v1.0.0-x86_64-pc-windows-msvc.zip)

**Note**: This release includes multi-platform binaries for Linux, macOS, and Windows.

### Build from Source
```bash
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
git checkout v1.0.0
cargo build --release
```

---

**🎉 Thank you for using Plex Media Organizer!**

This release represents months of development and testing. We hope it helps you organize your media library efficiently and safely.

**Made with ❤️ in Rust**
