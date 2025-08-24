# Configuration Guide

This guide covers all configuration options for Plex Media Organizer.

## Configuration File Location

- **macOS**: `~/Library/Application Support/plex-media-organizer/config.toml`
- **Linux**: `~/.config/plex-media-organizer/config.toml`
- **Windows**: `%APPDATA%\plex-media-organizer\config.toml`

## Configuration Structure

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

## Configuration Sections

### Database Configuration

```toml
[database]
path = "/path/to/database.db"
```

- **`path`**: Path to the SQLite database file
  - Default: Automatically determined based on OS
  - Purpose: Stores operation history, cache, and configuration

### API Configuration

```toml
[apis]
tmdb_api_key = "your-tmdb-api-key"
```

- **`tmdb_api_key`**: The Movie Database API key (optional)
  - Get your key from: https://www.themoviedb.org/settings/api
  - Purpose: Enhances parsing accuracy and provides metadata
  - Note: Application works without this key for basic functionality

### Organization Configuration

```toml
[organization]
default_output_directory = "/path/to/organized/media"
confidence_threshold = 0.7
network_mode = false
max_parallel_operations = 16
batch_size = 100
```

#### Core Settings

- **`default_output_directory`**: Default directory for organized files
  - Default: `~/Movies` (macOS), `~/Videos` (Linux), `%USERPROFILE%\Videos` (Windows)
  - Purpose: Where organized files will be placed

- **`confidence_threshold`**: Minimum confidence level for automatic organization
  - Range: 0.0 to 1.0
  - Default: 0.7
  - Purpose: Controls how strict the parser is about file organization

#### Performance Settings

- **`network_mode`**: Enable network drive optimizations
  - Default: `false`
  - Purpose: Optimizes performance for network-attached storage

- **`max_parallel_operations`**: Maximum concurrent file operations
  - Default: 16
  - Range: 1 to 64
  - Purpose: Controls parallel processing for large directories

- **`batch_size`**: Number of files to process in each batch
  - Default: 100
  - Range: 10 to 1000
  - Purpose: Balances memory usage and performance

### Technical Terms Configuration

Technical terms are automatically filtered from movie titles to improve parsing accuracy. These include release groups, codecs, quality indicators, and other metadata.

```toml
[organization.technical_terms]
release_groups = ["YIFY", "YTS", "RARBG", "3L", "CMCT"]
video_audio_terms = ["x264", "x265", "DTS", "AC3", "TrueHD", "7.1"]
source_platform_terms = ["Netflix", "Amazon", "iTunes"]
file_format_terms = ["mkv", "mp4", "avi", "web", "dl", "rip"]
special_edition_terms = ["Extended", "Director's Cut", "Unrated"]
custom_terms = ["YourCustomTerm1", "YourCustomTerm2"]
```

#### Term Categories

- **`release_groups`**: Names of release groups to filter
  - Examples: "YIFY", "YTS", "RARBG", "3L", "CMCT"
  - Purpose: Remove release group names from titles

- **`video_audio_terms`**: Video and audio codec/quality terms
  - Examples: "x264", "x265", "DTS", "AC3", "TrueHD", "7.1"
  - Purpose: Remove technical specifications from titles

- **`source_platform_terms`**: Streaming platform and source names
  - Examples: "Netflix", "Amazon", "iTunes", "HDRip", "BRRip"
  - Purpose: Remove source information from titles

- **`file_format_terms`**: File format and container terms
  - Examples: "mkv", "mp4", "avi", "web", "dl", "rip"
  - Purpose: Remove format indicators from titles

- **`special_edition_terms`**: Special edition and version terms
  - Examples: "Extended", "Director's Cut", "Unrated", "Theatrical"
  - Purpose: Remove edition information from titles

- **`custom_terms`**: User-defined terms to filter
  - Purpose: Add your own custom terms to filter

## Configuration Management

### Benefits of Configuration File

- ✅ **Single source of truth**: All settings in one file
- ✅ **User-editable**: No need to recompile for changes
- ✅ **Version controlled**: Changes tracked in git
- ✅ **Immediate effect**: Restart app, changes apply
- ✅ **Portable**: Config file moves with the app

### Editing Configuration

1. **Manual editing**: Edit the config file directly
2. **Setup command**: Use `plex-media-organizer setup` for interactive setup
3. **Config command**: Use `plex-media-organizer config` to view current settings

### Configuration Validation

The application validates configuration on startup:
- Checks file paths exist and are writable
- Validates API keys (if provided)
- Ensures numeric values are within valid ranges
- Reports any configuration issues

### Configuration Migration

When upgrading between versions, the application automatically migrates configuration:
- Preserves user settings
- Adds new default values for new options
- Maintains backward compatibility

## Advanced Configuration

### Custom Parsing Rules

You can customize parsing behavior by modifying technical terms:

```toml
[organization.technical_terms]
# Add your own release groups
release_groups = ["YIFY", "YTS", "RARBG", "3L", "CMCT", "YourGroup"]

# Add custom quality terms
video_audio_terms = ["x264", "x265", "DTS", "AC3", "TrueHD", "7.1", "YourTerm"]

# Add platform-specific terms
source_platform_terms = ["Netflix", "Amazon", "iTunes", "YourPlatform"]
```

### Performance Tuning

For large media libraries, adjust performance settings:

```toml
[organization]
# Increase for faster processing (uses more memory)
max_parallel_operations = 32

# Increase for network drives
batch_size = 50

# Enable for NAS/network storage
network_mode = true
```

### Database Configuration

For advanced users, you can specify a custom database location:

```toml
[database]
path = "/custom/path/to/database.db"
```

**Note**: Ensure the directory exists and is writable.

## Troubleshooting

### Common Issues

1. **Configuration not found**: Run `plex-media-organizer setup` to create initial config
2. **Permission errors**: Check file and directory permissions
3. **API key issues**: Verify TMDB API key is valid and has proper permissions
4. **Performance issues**: Adjust `max_parallel_operations` and `batch_size`

### Configuration Reset

To reset to default configuration:

```bash
# Remove config file (will be recreated on next run)
rm ~/.config/plex-media-organizer/config.toml

# Or run setup to recreate
plex-media-organizer setup
```

For more detailed information about technical terms management, see [Technical Terms Guide](technical-terms.md).
