# 🎬 Plex Media Organizer

A powerful, intelligent media file organizer built in Rust that automatically parses and organizes your media collection according to Plex naming conventions.

## ✨ Features

- **🎬 Movie Parsing**: Intelligent parsing of complex movie filenames with 100% success rate
- **🔍 TMDB Integration**: Enhanced matching with fuzzy search and multiple strategies
- **🌏 Multi-language Support**: Chinese, English, and mixed content
- **⚡ High Performance**: 180+ files/second with parallel processing and caching
- **🎯 Plex Compliance**: Strict adherence to Plex naming conventions
- **🔧 CLI Interface**: Easy-to-use command-line tool

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ ([install via rustup](https://rustup.rs/))
- TMDB API key ([get free key](https://www.themoviedb.org/settings/api))

### Installation & Setup
```bash
# Clone and build
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release

# Setup configuration
./target/release/plex-media-organizer setup

# Test with a sample file
./target/release/plex-media-organizer test "Movie.Name.2023.1080p.BluRay.mkv"

# Scan a directory
./target/release/plex-media-organizer scan /path/to/your/movies
```

## 📖 Usage

### Basic Commands
```bash
# Scan and parse movies in a directory
plex-media-organizer scan /path/to/movies

# Test individual file parsing
plex-media-organizer test "filename.mkv"

# View configuration
plex-media-organizer config

# Get help
plex-media-organizer --help
```

### Example Output
```
✅ Successfully parsed: 417 files
📊 Performance: 181 files/second
🎯 TMDB matches: 95% accuracy
```

## 📚 Documentation

For detailed information about the project:

- **[📋 Current Status](docs/CURRENT_STATUS.md)** - Project progress and recent updates
- **[🏗️ Architecture](docs/ARCHITECTURE.md)** - System design and technical details
- **[🗺️ Roadmap](docs/IMPLEMENTATION_ROADMAP.md)** - Development phases and timeline
- **[📖 Documentation Overview](docs/README.md)** - Complete documentation guide

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture
```

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code standards
- Testing requirements
- Pull request process

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🎯 Project Status

**✅ Iteration 1 Complete**: Movie MVP with enhanced TMDB matching, parallel processing, and comprehensive testing.

**🚀 Ready for Iteration 2**: Database integration and advanced features.

---

**Need help?** Check the [documentation](docs/) or [open an issue](https://github.com/lijunzh/plex-media-organizer/issues).
