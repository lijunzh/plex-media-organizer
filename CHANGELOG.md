# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.1] - 2025-08-23

### Added
- **Apple Silicon Support**: Added native ARM64 builds for Apple M1/M2/M3 processors
- **Dual macOS Architecture**: Now supports both Intel (x86_64) and Apple Silicon (aarch64) Macs
- **Matrix Build Strategy**: macOS builds now use matrix strategy for multiple architectures

## [1.0.0] - 2025-08-23

### Added
- **Multi-platform Build Support**: GitHub Actions now builds for Linux, macOS (Intel & Apple Silicon), and Windows
- **Native Platform Builds**: Separate build jobs for each platform for better reliability
- **Platform-specific Archives**: .tar.gz for Linux/macOS, .zip for Windows
- **Updated Documentation**: Installation instructions for all platforms
- **Complete CLI Workflow**: Setup, scan, test, organize, rollback, and cleanup commands
- **TMDB Integration**: Enhanced movie metadata from The Movie Database
- **Database-backed Operations**: Persistent storage for parsing results and operation history
- **Intelligent Caching**: Performance optimization with database caching
- **Safety Features**: Preview mode, rollback functionality, and operation history
- **Non-English Title Support**: Comprehensive handling of Chinese, Japanese, Korean, Arabic, and Cyrillic scripts
- **Plex-optimized Organization**: Flat directory structure for optimal Plex indexing
- **Bilingual Title Format**: "Original Title - English Title" format for better Plex recognition

### Added
- Complete CLI workflow implementation with 7 core commands
- Database-backed operation history with rollback capabilities
- TMDB API integration for enhanced movie metadata
- Multi-language support (English, Chinese, Japanese, Arabic, Russian)
- Intelligent filename parsing with confidence scoring
- Plex-compatible naming conventions
- Preview mode for safe testing
- Database maintenance and optimization tools

### Changed
- Refactored parser architecture for better modularity
- Improved error handling and user feedback
- Enhanced configuration management
- Optimized database operations and caching

### Fixed
- Various clippy warnings and code formatting issues
- Timestamp parsing issues in database operations
- File path handling for cross-platform compatibility

## [0.1.0] - 2024-01-XX

### Added
- **Setup Command**: Interactive configuration setup
  - Database path configuration
  - TMDB API key setup
  - Default output directory
  - Confidence thresholds
  - Network mode settings

- **Config Command**: Configuration management
  - View current configuration
  - Verbose configuration display
  - Configuration validation

- **Scan Command**: Directory analysis
  - Recursive media file discovery
  - File type statistics
  - Organization candidate identification
  - Network drive support

- **Test Command**: Parsing functionality testing
  - Filename parsing without file operations
  - Confidence score display
  - Cache performance testing
  - Pattern-specific testing

- **Organize Command**: Media file organization
  - Advanced filename parsing
  - Plex naming convention compliance
  - Move/copy operation support
  - TMDB metadata integration
  - Operation history tracking
  - Preview mode for safety
  - Parallel processing support

- **Rollback Command**: Operation recovery
  - Database-backed rollback system
  - Preview mode for safe rollback
  - Detailed progress reporting
  - Graceful error handling
  - Support for move/rename/copy operations

- **Cleanup Command**: Database maintenance
  - Old operation cleanup
  - Database optimization (VACUUM, ANALYZE, REINDEX)
  - Space reclamation
  - Statistics tracking
  - Preview mode for maintenance

### Technical Features
- **Database Integration**: SQLite-based operation history
  - Persistent operation tracking
  - Foreign key constraints
  - Indexed queries for performance
  - Automatic cleanup capabilities

- **TMDB Integration**: Enhanced metadata support
  - Automatic TMDB client creation
  - Confidence score boosting
  - Fallback to basic parsing
  - Rate limiting and retry logic

- **Parser Architecture**: Modular parsing system
  - Unified movie parser
  - Pattern detection modules
  - Language detection
  - Quality and source detection
  - Technical terms filtering

- **Safety Features**:
  - Preview mode for all operations
  - Comprehensive error handling
  - Input validation
  - Graceful failure recovery
  - Operation rollback support

### Performance Improvements
- Parallel processing for large directories
- Intelligent caching system
- Database query optimization
- Memory-efficient operations
- Network drive optimization

### Documentation
- Comprehensive CLI commands reference
- User guide with examples
- Best practices documentation
- Troubleshooting guide
- Development documentation

### Testing
- 96 unit tests covering all functionality
- Integration tests for CLI commands
- Regression tests for parsing accuracy
- Real-world pattern testing
- Error condition testing

## [Pre-0.1.0] - Development Phase

### Initial Development
- Basic filename parsing functionality
- Plex naming convention support
- Multi-language title handling
- Configuration system
- Database schema design
- CLI framework setup

---

## Migration Guide

### From Previous Versions
This is the first major release with a complete CLI workflow. Users should:

1. **Run Setup**: Execute `plex-media-organizer setup` to configure the application
2. **Test First**: Use `plex-media-organizer test` to validate parsing
3. **Preview Operations**: Always use `--preview` flag before organizing files
4. **Backup Data**: Ensure important media files are backed up before organization

### Breaking Changes
- Configuration file format has changed
- Database schema is new and incompatible with previous versions
- CLI command structure is completely new

### New Features
- All CLI commands are new and provide comprehensive functionality
- Database-backed operations with rollback support
- TMDB integration for enhanced metadata
- Multi-language support for international media

---

## Contributing

For information on contributing to this project, see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
