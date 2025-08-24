# User Documentation

Welcome to the Plex Media Organizer user documentation. This guide provides everything you need to know to use the application effectively.

## 🚀 Getting Started

### New Users
Start here if you're new to Plex Media Organizer:

1. **[Getting Started](getting-started.md)** - Quick start guide and basic concepts
2. **[Installation Guide](installation.md)** - Complete installation instructions for all platforms
3. **[CLI Commands](cli-commands.md)** - All available commands and their usage

### Basic Workflow
```bash
# 1. Setup (first time only)
plex-media-organizer setup

# 2. Scan a directory to see what's there
plex-media-organizer scan /path/to/movies

# 3. Test parsing without making changes
plex-media-organizer test /path/to/movies

# 4. Organize files (preview first!)
plex-media-organizer organize /path/to/movies --preview

# 5. Actually organize the files
plex-media-organizer organize /path/to/movies
```

## 📚 Complete Documentation

### Core Guides
- **[Getting Started](getting-started.md)** - Quick start guide for new users
- **[Installation Guide](installation.md)** - Installation for Windows, macOS, and Linux
- **[CLI Commands](cli-commands.md)** - Complete command reference with examples

### Configuration & Customization
- **[Configuration Guide](configuration.md)** - All configuration options and settings
- **[Technical Terms](technical-terms.md)** - Customizing parsing behavior and filtering

### Features & Capabilities
- **[Features Guide](features.md)** - Comprehensive feature documentation
- **[Troubleshooting](troubleshooting.md)** - Common issues and solutions

### Examples & Use Cases
- **[Examples](examples/)** - Real-world usage examples and scenarios

## 🎯 Key Features

### Intelligent Parsing
- **Pattern Recognition**: Detects movies, series, anime, and other media types
- **Quality Detection**: Identifies resolution, source, and codec information
- **Language Support**: Handles multi-language titles and metadata
- **Confidence Scoring**: Provides confidence levels for parsing accuracy

### Plex Integration
- **Naming Conventions**: Follows Plex's recommended naming structure
- **Flat Organization**: Optimized for Plex's flat directory structure
- **Metadata Support**: Extracts and preserves metadata for Plex indexing

### Safety Features
- **Preview Mode**: Test operations before making changes
- **Rollback Support**: Revert any operation with full history
- **Error Handling**: Graceful handling of file system errors
- **Validation**: Comprehensive input validation and error checking

## 📋 Command Reference

### Core Commands
- **`setup`** - Interactive configuration setup
- **`scan`** - Analyze media directories
- **`test`** - Test parsing functionality
- **`organize`** - Organize media files
- **`rollback`** - Revert previous operations
- **`cleanup`** - Database maintenance

### Advanced Commands
- **`config`** - View and modify configuration
- **`migrate`** - Configuration migration utilities

## 🔧 Configuration

### Configuration File Location
- **macOS**: `~/Library/Application Support/plex-media-organizer/config.toml`
- **Linux**: `~/.config/plex-media-organizer/config.toml`
- **Windows**: `%APPDATA%\plex-media-organizer\config.toml`

### Key Settings
- **Database path**: Where operation history is stored
- **TMDB API key**: For enhanced metadata (optional)
- **Output directory**: Where organized files are placed
- **Confidence threshold**: How strict the parser should be
- **Technical terms**: Custom filtering rules

## 🌍 Supported Content

### File Types
- **Movies**: `.mkv`, `.mp4`, `.avi`, `.mov`, `.wmv`
- **TV Shows**: `.mkv`, `.mp4`, `.avi`, `.mov`
- **Documentaries**: `.mkv`, `.mp4`, `.avi`
- **Anime**: `.mkv`, `.mp4`, `.avi`

### Languages
- **English**: Primary language with full support
- **Chinese**: Simplified and Traditional Chinese
- **Japanese**: Hiragana, Katakana, and Kanji
- **Arabic**: Arabic script support
- **Russian**: Cyrillic script support
- **Other Languages**: Latin-based scripts

## 🛠️ Troubleshooting

### Common Issues
- **Installation problems**: Check [Installation Guide](installation.md)
- **Configuration issues**: Review [Configuration Guide](configuration.md)
- **Parsing problems**: See [Technical Terms](technical-terms.md)
- **Performance issues**: Check [Features Guide](features.md)

### Getting Help
- **Documentation**: This user guide
- **Issues**: [GitHub Issues](https://github.com/lijunzh/plex-media-organizer/issues)
- **Discussions**: [GitHub Discussions](https://github.com/lijunzh/plex-media-organizer/discussions)

## 📖 Best Practices

### File Organization
- Always use preview mode before organizing
- Keep backups of important media
- Use consistent naming conventions
- Regular database maintenance

### Performance
- Adjust parallel operations based on your system
- Use appropriate batch sizes for your storage
- Enable network mode for NAS storage
- Regular cache cleanup

### Safety
- Test on a small subset first
- Use rollback features when needed
- Monitor disk space during operations
- Verify file integrity after operations

## 🔄 Migration & Updates

### Upgrading
- Backup your configuration
- Test new versions on sample data
- Review configuration changes
- Update technical terms as needed

### Data Migration
- Export operation history
- Backup database files
- Test migration process
- Verify data integrity

## 📞 Support

### Documentation Issues
- Create an issue for documentation problems
- Include specific page and section references
- Describe what you expected vs. what you found

### Feature Requests
- Use GitHub Discussions for feature requests
- Provide detailed use cases and examples
- Consider existing functionality first

### Bug Reports
- Include error messages and logs
- Describe steps to reproduce
- Mention your system and version

---

**Need more help?** Check the [main documentation index](../README.md) for additional resources.
