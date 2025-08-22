# Plex Media Organizer

An intelligent media file organizer that follows Plex naming conventions and uses TMDB for accurate movie metadata.

## 🚀 Quick Start

### 1. Setup
```bash
# Run interactive setup
plex-media-organizer setup

# This will prompt for your TMDB API key
# Get one at: https://www.themoviedb.org/settings/api
```

### 2. Organize Movies
```bash
# Preview organization (recommended first)
plex-media-organizer organize /path/to/movies --preview --verbose

# Apply organization
plex-media-organizer organize /path/to/movies --backup /path/to/backup
```

### 3. Test Parsing
```bash
# Test a single file
plex-media-organizer test /path/to/movie.mkv --verbose

# Test entire directory
plex-media-organizer test /path/to/movies --organize --preview
```

## ✨ Key Features

- **TMDB Integration**: Uses The Movie Database for accurate movie information
- **Plex-Compatible Naming**: Follows Plex naming conventions for optimal indexing
- **Multilingual Support**: Handles Chinese, Japanese, and English titles
- **Conservative Approach**: Prioritizes accuracy over completeness
- **Preview Mode**: Dry-run mode to preview changes before applying
- **Rollback Support**: Easy rollback of organization operations
- **Network Drive Optimization**: Optimized for SMB/NFS network drives

## 📚 Documentation

### For Users
- [User Guide](docs/user-guide.md) - Comprehensive usage tutorial
- [Quick Start](docs/quick-start.md) - Get started quickly
- [Configuration](docs/configuration.md) - All configuration options
- [Troubleshooting](docs/troubleshooting.md) - Common issues and solutions
- [Examples](docs/examples/) - Real-world usage examples

### For Developers
- [Development Guide](docs/development/README.md) - Complete developer documentation
- [Architecture](docs/architecture/) - System design and architecture
- [Roadmap](docs/architecture/roadmap.md) - Development timeline and planning
- [Git Commit Practices](docs/development/git-commit-practices.md) - Best practices for contributors

## 🛠️ Installation

### From Source
```bash
git clone https://github.com/your-repo/plex-media-organizer.git
cd plex-media-organizer
cargo build --release
```

### Requirements
- Rust 1.70+
- TMDB API key (free at https://www.themoviedb.org/settings/api)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

See [Development Guide](docs/development/README.md) for detailed contribution guidelines.

## 📄 License

MIT License - see LICENSE file for details.

## 🆘 Support

- **Issues**: Report bugs and feature requests on GitHub
- **Documentation**: Check the docs/ directory for detailed information
- **Trade-offs**: Understand the conservative approach in [User Guide](docs/user-guide.md)

---

**Remember**: It's always better to skip a movie than to organize it incorrectly!
