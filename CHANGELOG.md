# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Configuration-based technical terms management**: Migrated from hard-coded technical terms to user-editable configuration
  - New `TechnicalTermsConfig` structure in `config.toml` for managing filtering terms
  - Comprehensive term categories: release groups, video/audio terms, source/platform terms, file formats, special editions, custom terms
  - Essential fallback terms for critical filtering (3L, YIFY, YTS, etc.)
  - Improved case-insensitive matching and title cleaning
  - Added missing technical terms (3L, TrueHD, 7.1, 5.1, 2.0, DoVi)
  - Comprehensive documentation in `docs/user-guide/technical-terms.md`
  - CLI `terms` command structure (placeholder for future implementation)

### Changed
- **Technical terms filtering**: Now uses configuration file instead of hard-coded lists
  - Single source of truth: `config.toml` file
  - User-editable without recompiling
  - Version controlled and portable
  - Immediate effect on restart
- **Title cleaning**: Improved separator handling for better filename formatting

### Fixed
- **3L release group filtering**: Fixed issue where "3L" was appearing in organized movie titles
- **Technical terms case sensitivity**: Improved case-insensitive matching for better filtering accuracy

## [1.0.1] - 2025-08-23

### Added
- **Multi-platform support**: Native builds for Linux (x86_64), macOS (x86_64 and aarch64), and Windows (x86_64)
- **Apple Silicon support**: Native ARM64 builds for Apple M1/M2/M3 processors
- **Enhanced GitHub Actions**: Automated multi-platform release workflow with native runners
- **Improved release artifacts**: Platform-specific archives with proper naming conventions

### Changed
- **GitHub Actions workflow**: Refactored to use matrix strategy for multi-platform builds
- **Build process**: Updated to use native runners for each platform (ubuntu-latest, macos-latest, windows-latest)
- **Artifact management**: Updated to actions/upload-artifact@v4 and actions/download-artifact@v4

### Fixed
- **Windows build compatibility**: Fixed PowerShell command usage in GitHub Actions
- **macOS matrix builds**: Corrected Rust toolchain target configuration for Apple Silicon

## [1.0.0] - 2025-08-23

### Added
- **Complete CLI workflow**: Full set of commands for media organization
  - `setup`: Interactive configuration wizard
  - `config`: View and edit configuration settings
  - `scan`: Scan directories for media files
  - `test`: Test filename parsing and organization
  - `organize`: Organize media files with preview and safety features
  - `rollback`: Database-backed operation history and rollback
  - `cleanup`: Clean up old operations and optimize database
- **TMDB API integration**: Enhanced movie metadata with The Movie Database
- **Database caching**: Persistent storage for parsing results and operation history
- **Non-English title support**: Comprehensive handling of Chinese, Japanese, Korean, Arabic, and other non-Latin scripts
- **Bilingual title formatting**: "Original Title - English Title" format for better Plex indexing
- **Safety features**: Preview mode, dry-run operations, comprehensive error handling
- **Performance optimizations**: Intelligent caching, database optimization, efficient parsing
- **Comprehensive documentation**: User guides, developer docs, architecture documentation

### Changed
- **Architecture**: Complete refactoring with modular parser components
- **Configuration**: TOML-based configuration with platform-specific defaults
- **Database**: SQLite with connection pooling and optimization
- **Error handling**: Comprehensive error handling with user-friendly messages
- **File organization**: Flat structure optimized for Plex with year-based option

### Fixed
- **Filename parsing**: Improved accuracy for complex patterns and edge cases
- **Technical terms filtering**: Enhanced filtering of release groups, codecs, and quality indicators
- **Title extraction**: Better handling of special characters and formatting
- **Database operations**: Robust transaction handling and error recovery

## [0.1.0] - 2025-08-23

### Added
- Initial release with basic movie organization functionality
- Filename parsing and metadata extraction
- Basic CLI interface
- Configuration management
- Database storage for parsing results
