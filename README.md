# Plex Media Organizer

A powerful, intelligent media file organizer that follows Plex naming conventions. Built in Rust for performance and reliability.

## 🎬 Features

### Core Functionality
- **Intelligent Parsing**: Advanced filename parsing with confidence scoring
- **Plex Compatibility**: Follows Plex naming conventions for optimal media server integration
- **Multi-language Support**: Handles English, Chinese, Japanese, Arabic, and other languages
- **TMDB Integration**: Enhanced metadata using The Movie Database API
- **Database Backed**: Persistent operation history with rollback capabilities

### CLI Commands
- **`setup`** - Interactive configuration setup
- **`config`** - View and modify configuration
- **`scan`** - Analyze media directories
- **`test`** - Test parsing functionality
- **`organize`** - Organize media files
- **`rollback`** - Revert previous operations
- **`cleanup`** - Database maintenance

## 🚀 Quick Start

### Installation

#### Option 1: Download Pre-built Binaries (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/lijunzh/plex-media-organizer/releases):

- **macOS (Intel)**: `plex-media-organizer-v1.0.0-x86_64-apple-darwin.tar.gz`
- **macOS (Apple Silicon)**: `plex-media-organizer-v1.0.0-aarch64-apple-darwin.tar.gz`
- **Linux**: `plex-media-organizer-v1.0.0-x86_64-unknown-linux-gnu.tar.gz`
- **Windows**: `plex-media-organizer-v1.0.0-x86_64-pc-windows-msvc.zip`

Extract and add to your PATH:
```bash
# macOS/Linux
tar -xzf plex-media-organizer-v1.0.0-x86_64-apple-darwin.tar.gz
sudo mv plex-media-organizer /usr/local/bin/

# Windows
# Extract the zip file and add the directory to your PATH
```

#### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer

# Build the project
cargo build --release

# Install (optional)
cargo install --path .
```

### First Time Setup

```bash
# Run the interactive setup
plex-media-organizer setup

# This will guide you through:
# - Database configuration
# - TMDB API key (optional)
# - Default output directory
# - Confidence thresholds
```

### Basic Usage

```bash
# Scan a directory to see what's there
plex-media-organizer scan /path/to/movies

# Test parsing without making changes
plex-media-organizer test /path/to/movies

# Organize files (preview first!)
plex-media-organizer organize /path/to/movies --preview

# Actually organize the files
plex-media-organizer organize /path/to/movies
```

## 📋 Command Reference

### Setup Command
Configure the application for first use.
```bash
plex-media-organizer setup [OPTIONS]
```

### Scan Command
Analyze a directory for media files.
```bash
plex-media-organizer scan <DIRECTORY> [OPTIONS]
```

### Test Command
Test parsing functionality without moving files.
```bash
plex-media-organizer test <DIRECTORY> [OPTIONS]
```

### Organize Command
Organize media files according to Plex conventions.
```bash
plex-media-organizer organize <DIRECTORY> [OPTIONS]
```

### Rollback Command
Revert a previous organization operation.
```bash
plex-media-organizer rollback <OPERATION_ID> [OPTIONS]
```

### Cleanup Command
Maintain database health and clean up old operations.
```bash
plex-media-organizer cleanup [OPTIONS]
```

For detailed command documentation, see [CLI Commands Reference](docs/user-guide/cli-commands.md).

## 🎯 Key Features

### Intelligent Parsing
- **Pattern Recognition**: Detects movies, series, anime, and other media types
- **Quality Detection**: Identifies resolution, source, and codec information
- **Language Support**: Handles multi-language titles and metadata
- **Confidence Scoring**: Provides confidence levels for parsing accuracy
- **Technical Terms Filtering**: User-configurable filtering of release groups, codecs, and quality indicators

### Plex Integration
- **Naming Conventions**: Follows Plex's recommended naming structure
- **Flat Organization**: Optimized for Plex's flat directory structure
- **Metadata Support**: Extracts and preserves metadata for Plex indexing

### Database Features
- **Operation History**: Tracks all organization operations
- **Rollback Support**: Revert any previous operation safely
- **Caching**: Intelligent caching for improved performance
- **Maintenance**: Built-in cleanup and optimization tools

### TMDB Integration
- **Enhanced Metadata**: Uses TMDB API for accurate movie information
- **Confidence Boosting**: Improves parsing confidence with external data
- **Fallback Support**: Works without API key for basic functionality

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

### Configuration File Location
- **macOS**: `~/Library/Application Support/plex-media-organizer/config.toml`
- **Linux**: `~/.config/plex-media-organizer/config.toml`
- **Windows**: `%APPDATA%\plex-media-organizer\config.toml`

### Example Configuration
```toml
[database]
path = "/path/to/database.db"

[apis]
tmdb_api_key = "your-tmdb-api-key"

[organization]
default_output_directory = "/path/to/organized/media"
confidence_threshold = 0.7
network_mode = false
max_parallel_operations = 16
batch_size = 100

# Technical terms filtering configuration
[organization.technical_terms]
# Release group names to filter out
release_groups = ["YIFY", "YTS", "RARBG", "3L", "CMCT"]

# Video/audio codec and quality terms
video_audio_terms = ["x264", "x265", "DTS", "AC3", "TrueHD", "7.1"]

# Source/platform names
source_platform_terms = ["Netflix", "Amazon", "iTunes"]

# File format and container terms
file_format_terms = ["mkv", "mp4", "avi", "web", "dl", "rip"]

# Special edition and version terms
special_edition_terms = ["Extended", "Director's Cut", "Unrated"]

# Additional custom terms
custom_terms = ["YourCustomTerm1", "YourCustomTerm2"]
```

### Technical Terms Management
Technical terms (like release groups, codecs, quality indicators) are automatically filtered from movie titles. These terms are managed through the configuration file and can be customized without recompiling the application.

**Benefits:**
- ✅ **Single source of truth**: All terms in `config.toml`
- ✅ **User-editable**: No need to recompile for changes
- ✅ **Version controlled**: Changes tracked in git
- ✅ **Immediate effect**: Restart app, changes apply
- ✅ **Portable**: Config file moves with the app

See [Technical Terms Management](docs/user-guide/technical-terms.md) for detailed documentation.

## 🛠️ Development

### Prerequisites
- Rust 1.70+ (stable)
- Cargo (comes with Rust)

### Building
```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with specific features
cargo run --features tmdb
```

### Project Structure
```
src/
├── cli/           # Command-line interface
├── config/        # Configuration management
├── database/      # Database operations
├── external/      # External API integrations
├── parsers/       # Media parsing logic
├── scanner/       # File scanning
└── types/         # Common types and structures
```

## 📊 Performance

- **Fast Parsing**: Optimized algorithms for quick filename analysis
- **Parallel Processing**: Configurable parallel operations for large directories
- **Intelligent Caching**: Database-backed caching for repeated operations
- **Memory Efficient**: Minimal memory footprint for large media libraries

## 🔒 Safety Features

- **Preview Mode**: Test operations before making changes
- **Rollback Support**: Revert any operation with full history
- **Error Handling**: Graceful handling of file system errors
- **Validation**: Comprehensive input validation and error checking

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

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Plex Media Server**: For naming conventions and inspiration
- **The Movie Database (TMDB)**: For movie metadata API
- **Rust Community**: For excellent tooling and ecosystem

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/lijunzh/plex-media-organizer/issues)
- **Discussions**: [GitHub Discussions](https://github.com/lijunzh/plex-media-organizer/discussions)
- **Documentation**: [User Guide](docs/user-guide/)

## 🚀 Roadmap

- [ ] TV Show support with episode detection
- [ ] Music file organization
- [ ] Web interface
- [ ] Scheduled organization
- [ ] Cloud storage integration
- [ ] Advanced metadata extraction
- [ ] Plugin system for custom parsers

---

**Made with ❤️ in Rust**
