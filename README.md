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

```bash
# 1. Install Rust and get TMDB API key
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Get free TMDB API key: https://www.themoviedb.org/settings/api

# 2. Clone and build
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release

# 3. Test it works
TMDB_API_KEY=your_key ./target/release/plex-media-organizer test "Movie.Name.2023.1080p.BluRay.mkv"
```

**Result:** `✅ Successfully parsed: Movie Name (2023) 1080p BluRay`

📚 **Need more help?** See the [complete setup guide](docs/quick-start.md) or [installation options](docs/installation.md).

## 📚 Documentation

### **For Users**
- **[🚀 Quick Start Guide](docs/quick-start.md)** - Complete setup and first use
- **[📖 User Guide](docs/user-guide.md)** - Comprehensive usage tutorial
- **[⚙️ Configuration](docs/configuration.md)** - All configuration options
- **[💡 Examples](docs/examples/)** - Real-world usage examples
- **[🔧 Troubleshooting](docs/troubleshooting.md)** - Common issues and solutions

### **For Developers** 
- **[🏗️ Code Architecture](docs/development/code-architecture.md)** - Understanding the codebase
- **[📋 API Reference](docs/development/api-reference.md)** - Library API documentation
- **[🔨 Adding Features](docs/development/adding-features.md)** - How to extend the system

### **Project Management**
- **[📊 Current Status](project/status.md)** - Development progress and metrics
- **[🏛️ Architecture Design](project/architecture.md)** - System design philosophy
- **[🗺️ Roadmap](project/roadmap.md)** - Development timeline and planning

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

💬 **Need help?** Check the [documentation](docs/) or [open an issue](https://github.com/lijunzh/plex-media-organizer/issues).

⭐ **Like this project?** Give it a star on GitHub!
