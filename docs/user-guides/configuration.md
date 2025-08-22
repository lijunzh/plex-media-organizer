# ⚙️ Configuration Reference

Complete guide to configuring Plex Media Organizer for your specific needs.

## 📍 Configuration File Locations

The application looks for configuration files in these locations (in order):

### Linux/macOS
```
1. ./config.toml                                    # Current directory
2. ~/.config/plex-media-organizer/config.toml      # User config
3. /etc/plex-media-organizer/config.toml           # System config
```

### Windows
```
1. .\config.toml                                    # Current directory
2. %APPDATA%\plex-media-organizer\config.toml      # User config
3. %PROGRAMDATA%\plex-media-organizer\config.toml  # System config
```

## 🏗️ Configuration File Structure

### Complete Example
```toml
# config.toml - Complete configuration example

[apis]
# TMDB API key (required for movie metadata)
tmdb_api_key = "your_tmdb_api_key_here"

# Future API keys (not used in Iteration 1)
tvdb_api_key = "your_tvdb_key_here"
musicbrainz_user_agent = "PlexMediaOrganizer/1.0 (your_email@example.com)"

[organization]
# Output directory for organized files
output_directory = "/path/to/organized/media"

# Whether to create subdirectories by media type
organize_by_type = true

[organization.quality]
# Preferred quality when multiple versions exist
preferred_quality = "1080p"

# Quality preference order (highest to lowest priority)
quality_order = ["4K", "2160p", "1080p", "720p", "480p"]

[organization.original_titles]
# Strategy for handling non-English titles (CJK, etc.)
prefer_original_titles = false

# Include English title in brackets: "原标题 [English Title] (2023)"
include_english_subtitle = false

# Fall back to English if original title causes filesystem issues
fallback_to_english_on_error = true

# Always preserve original title in metadata
preserve_original_in_metadata = true

[processing]
# Number of files to process in parallel
max_parallel_files = 16

# Enable caching of API responses
enable_cache = true

# Cache TTL in seconds (3600 = 1 hour)
cache_ttl = 3600

# Maximum cache size in MB
max_cache_size = 100

[logging]
# Log level: error, warn, info, debug, trace
level = "info"

# Log to file instead of console
log_to_file = false

# Log file path (if log_to_file = true)
log_file = "plex-organizer.log"
```

## 🔑 API Configuration

### TMDB API Key (Required)
```toml
[apis]
tmdb_api_key = "eyJhbGciOiJIUzI1NiJ9..."
```

**How to get:**
1. Go to [themoviedb.org](https://www.themoviedb.org/)
2. Create account → Settings → API → Create → Developer
3. Copy the API key

**Environment Variable Alternative:**
```bash
export TMDB_API_KEY="your_key_here"
plex-media-organizer scan /movies
```

### Future API Keys
```toml
[apis]
# TV shows (Iteration 2+)
tvdb_api_key = "your_tvdb_key"

# Music (Iteration 3+)
musicbrainz_user_agent = "YourApp/1.0 (email@example.com)"

# Anime (Iteration 4+)
anidb_username = "your_username"
anidb_password = "your_password"
```

## 📁 Organization Settings

### Basic Organization
```toml
[organization]
# Where to place organized files
output_directory = "/path/to/organized/media"

# Create subdirectories: Movies/, TV Shows/, Music/
organize_by_type = true

# Naming template for movies
movie_template = "{title} ({year}) {quality}"

# Directory template for movies
movie_directory_template = "{title} ({year})"
```

### Quality Preferences
```toml
[organization.quality]
# Preferred quality when multiple versions exist
preferred_quality = "1080p"

# Quality detection and preference order
quality_order = [
    "4K", "2160p", "UHD",      # 4K variants
    "1080p", "FHD",            # 1080p variants
    "720p", "HD",              # 720p variants
    "480p", "SD"               # Standard definition
]

# Include quality in filename
include_quality_in_filename = true

# Include source in filename (BluRay, WEB-DL, etc.)
include_source_in_filename = true
```

### Original Language Titles
```toml
[organization.original_titles]
# Use original titles for organization (vs English titles)
prefer_original_titles = false

# Examples of each setting:
# prefer_original_titles = false:
#   "Hero (2002)" (uses English title)
# prefer_original_titles = true:
#   "英雄 (2002)" (uses original Chinese title)

# Include English subtitle: "英雄 [Hero] (2002)"
include_english_subtitle = false

# Fall back to English if original causes filesystem issues
fallback_to_english_on_error = true

# Always keep original title in metadata regardless of filename
preserve_original_in_metadata = true

# Character sets to treat as "original" (auto-detected)
original_title_languages = ["zh", "ja", "ko", "ar", "th", "hi"]
```

## ⚡ Performance Settings

### Parallel Processing
```toml
[processing]
# Number of files to process simultaneously
max_parallel_files = 16

# Use all CPU cores: 0 = auto-detect
max_parallel_files = 0

# Conservative setting for slower systems
max_parallel_files = 4
```

### Caching
```toml
[processing]
# Enable API response caching
enable_cache = true

# Cache duration in seconds
cache_ttl = 3600  # 1 hour

# Maximum cache size in MB
max_cache_size = 100

# Cache location (auto-detected if not specified)
cache_directory = "~/.cache/plex-media-organizer"
```

### Memory Usage
```toml
[processing]
# Batch size for large directory processing
batch_size = 1000

# Maximum memory usage in MB (0 = unlimited)
max_memory_usage = 512

# Enable memory-efficient mode for large libraries
low_memory_mode = false
```

## 📝 Logging Configuration

### Basic Logging
```toml
[logging]
# Log levels: error, warn, info, debug, trace
level = "info"

# Enable colored output (auto-detected for terminals)
colored_output = true

# Show timestamps in logs
show_timestamps = true
```

### File Logging
```toml
[logging]
# Write logs to file instead of console
log_to_file = true

# Log file path
log_file = "/var/log/plex-organizer.log"

# Rotate logs when they exceed this size (MB)
max_log_size = 10

# Keep this many old log files
max_log_files = 5
```

### Advanced Logging
```toml
[logging]
# Enable debug logging for specific modules
debug_modules = ["tmdb_client", "movie_parser"]

# Log API requests and responses (debug level)
log_api_calls = false

# Log file operations (useful for troubleshooting)
log_file_operations = true
```

## 🎛️ Advanced Configuration

### Custom Naming Templates
```toml
[organization.templates]
# Movie naming template
movie_filename = "{title} ({year}) {quality} {source}.{ext}"

# Movie directory template
movie_directory = "{title} ({year})"

# Available variables:
# {title} - Movie title
# {original_title} - Original language title
# {year} - Release year
# {quality} - Video quality (1080p, 720p, etc.)
# {source} - Source (BluRay, WEB-DL, etc.)
# {codec} - Video codec (x264, x265, etc.)
# {ext} - File extension
```

### File Filtering
```toml
[processing.filters]
# Minimum file size in MB (skip very small files)
min_file_size = 100

# Maximum file size in GB (skip very large files)
max_file_size = 50

# Skip files matching these patterns
ignore_patterns = [
    "*sample*",
    "*trailer*",
    "*.nfo",
    "*.txt"
]

# Only process files with these extensions
allowed_extensions = [
    "mkv", "mp4", "avi", "mov", "wmv", "flv", "webm"
]
```

### Network Settings
```toml
[network]
# API request timeout in seconds
timeout = 30

# Maximum number of API retries
max_retries = 3

# Delay between retries in milliseconds
retry_delay = 1000

# User agent for API requests
user_agent = "PlexMediaOrganizer/1.0"

# Enable request rate limiting
enable_rate_limiting = true

# Maximum requests per second
max_requests_per_second = 10
```

## 🌍 Environment Variables

You can override any configuration value using environment variables:

### API Keys
```bash
export TMDB_API_KEY="your_key"
export TVDB_API_KEY="your_key"
export MUSICBRAINZ_USER_AGENT="YourApp/1.0"
```

### Organization Settings
```bash
export PMO_OUTPUT_DIRECTORY="/path/to/organized"
export PMO_PREFERRED_QUALITY="1080p"
export PMO_PREFER_ORIGINAL_TITLES="true"
```

### Performance Settings
```bash
export PMO_MAX_PARALLEL_FILES="8"
export PMO_ENABLE_CACHE="true"
export PMO_CACHE_TTL="7200"
```

### Logging Settings
```bash
export PMO_LOG_LEVEL="debug"
export PMO_LOG_TO_FILE="true"
export PMO_LOG_FILE="/tmp/plex-organizer.log"
```

## 🔍 Configuration Validation

### Check Current Configuration
```bash
# View current configuration
plex-media-organizer config

# Validate configuration file
plex-media-organizer config --validate

# Show configuration sources
plex-media-organizer config --sources
```

### Example Output
```
Configuration loaded from:
✅ Environment variables: TMDB_API_KEY
✅ ~/.config/plex-media-organizer/config.toml
❌ ./config.toml (not found)

Current settings:
[apis]
tmdb_api_key = "***hidden***"

[organization.quality]
preferred_quality = "1080p"

[organization.original_titles]
prefer_original_titles = false
fallback_to_english_on_error = true

[processing]
max_parallel_files = 16
enable_cache = true
```

## 📋 Configuration Profiles

### Development Profile
```toml
# config-dev.toml
[logging]
level = "debug"
log_api_calls = true

[processing]
max_parallel_files = 4
enable_cache = false
```

### Production Profile
```toml
# config-prod.toml
[logging]
level = "warn"
log_to_file = true

[processing]
max_parallel_files = 16
enable_cache = true
max_cache_size = 500
```

### Performance Profile
```toml
# config-performance.toml
[processing]
max_parallel_files = 0  # Use all cores
batch_size = 2000
enable_cache = true
max_cache_size = 1000
low_memory_mode = false
```

Use profiles with:
```bash
plex-media-organizer --config config-dev.toml scan /movies
```

## 🛠️ Troubleshooting Configuration

### Common Issues

#### Configuration Not Found
```bash
# Check where config is being loaded from
plex-media-organizer config --sources

# Create default config
plex-media-organizer setup --config-only
```

#### Invalid Configuration
```bash
# Validate configuration syntax
plex-media-organizer config --validate

# Check specific setting
plex-media-organizer config --get apis.tmdb_api_key
```

#### Permission Issues
```bash
# Fix config directory permissions
chmod 755 ~/.config/plex-media-organizer
chmod 644 ~/.config/plex-media-organizer/config.toml
```

### Configuration Templates

Generate configuration templates:
```bash
# Generate minimal config
plex-media-organizer setup --minimal

# Generate full config with all options
plex-media-organizer setup --full

# Generate config for specific use case
plex-media-organizer setup --template movies-only
```

---

**Next Steps**: Once configured, see the [User Guide](user-guide.md) to learn about all available features and commands.
