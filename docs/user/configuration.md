# Configuration Guide

## Overview

The Plex Media Organizer uses a configuration file to store settings. This guide covers all configuration options and their effects.

## Configuration File Location

The configuration file is stored at:
- **macOS**: `~/Library/Application Support/plex-media-organizer/config.toml`
- **Linux**: `~/.local/share/plex-media-organizer/config.toml`
- **Windows**: `%APPDATA%\plex-media-organizer\config.toml`

## Configuration Options

### Database Settings

```toml
[database]
# Database file path (auto-configured based on platform)
path = "~/Library/Application Support/plex-media-organizer/movies.db"

# Connection pool settings
max_connections = 10
connection_timeout = 30

# Cache settings
cache_ttl_hours = 24
```

### API Settings

```toml
[apis]
# TMDB API key (required)
tmdb_api_key = "your-api-key-here"

# API request settings
request_timeout = 30
max_retries = 3
```

### Organization Settings

```toml
[organization]
# File organization preferences
create_backup = true
backup_directory = "~/backups/plex-organizer"

# Directory structure
use_collection_directories = true
create_extras_directory = true

# File naming
preserve_original_extension = true
use_plex_naming = true
```

### Matching Settings

```toml
[organization.matching]
# Confidence threshold (0.0 to 1.0)
min_confidence_threshold = 0.7

# Skip unmatched movies
skip_unmatched_movies = true

# TMDB search settings
search_strategies = ["exact", "fuzzy", "year_range"]
max_search_results = 5
```

### Parsing Settings

```toml
[parsing]
# Technical terms to filter from titles
technical_terms = [
    "DualAudio",
    "iNT",
    "TLF",
    "WEB-DL",
    "BluRay",
    "HDRip"
]

# Common words to preserve
common_words = [
    "Lord",
    "Pirates",
    "Iron",
    "Man"
]

# Quality patterns
quality_patterns = [
    "1080p",
    "2160p",
    "720p",
    "4K",
    "UHD"
]
```

## Environment Variables

You can override configuration using environment variables:

```bash
# Database path
export PLEX_MEDIA_ORGANIZER_DATABASE_PATH="/custom/path/movies.db"

# TMDB API key
export PLEX_MEDIA_ORGANIZER_TMDB_API_KEY="your-api-key"

# Confidence threshold
export PLEX_MEDIA_ORGANIZER_MIN_CONFIDENCE="0.5"
```

## CLI Overrides

Command-line arguments take precedence over configuration:

```bash
# Override confidence threshold
plex-media-organizer organize /path --min-confidence 0.5

# Override database path
plex-media-organizer organize /path --database-path /custom/path

# Skip unmatched movies
plex-media-organizer organize /path --skip-unmatched false
```

## Configuration Priority

Settings are applied in this order (highest to lowest priority):

1. **Command-line arguments**
2. **Environment variables**
3. **Configuration file**
4. **Default values**

## Advanced Configuration

### Custom Technical Terms

Add your own technical terms to filter:

```toml
[parsing]
technical_terms = [
    "DualAudio",
    "iNT",
    "TLF",
    "WEB-DL",
    "BluRay",
    "HDRip",
    "YourCustomTerm"  # Add your own terms here
]
```

### Custom Quality Patterns

Define custom quality detection patterns:

```toml
[parsing]
quality_patterns = [
    "1080p",
    "2160p",
    "720p",
    "4K",
    "UHD",
    "YourCustomQuality"  # Add your own patterns
]
```

### Database Optimization

For large collections, optimize database settings:

```toml
[database]
# Increase connection pool for better performance
max_connections = 20

# Longer cache TTL for better performance
cache_ttl_hours = 48

# Enable WAL mode for better concurrency
wal_mode = true
```

## Configuration Validation

The organizer validates configuration on startup:

```bash
# Check configuration
plex-media-organizer config --validate

# Show current configuration
plex-media-organizer config --show
```

## Troubleshooting Configuration

### Common Issues

1. **Invalid API key**: Check your TMDB API key
2. **Permission errors**: Check file permissions for database path
3. **Invalid paths**: Ensure paths exist and are writable

### Configuration Reset

To reset to defaults:

```bash
# Remove configuration file
rm ~/.local/share/plex-media-organizer/config.toml

# Run setup again
plex-media-organizer setup
```

## Best Practices

### Security
- **Never commit API keys** to version control
- **Use environment variables** for sensitive data
- **Restrict file permissions** on configuration files

### Performance
- **Use appropriate cache TTL** for your usage patterns
- **Optimize connection pool** for large collections
- **Enable WAL mode** for better concurrency

### Maintenance
- **Backup configuration** before major changes
- **Test changes** with preview mode
- **Document custom settings** for team members
