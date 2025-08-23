# CLI Commands Reference

The Plex Media Organizer provides a comprehensive command-line interface for managing media files. This guide covers all available commands and their usage.

## Quick Start

```bash
# Set up the application for first use
plex-media-organizer setup

# Scan a directory to see what media files are found
plex-media-organizer scan /path/to/media

# Test parsing on a directory
plex-media-organizer test /path/to/media

# Organize media files
plex-media-organizer organize /path/to/media

# Rollback a previous operation
plex-media-organizer rollback <operation-id>

# Clean up old operations
plex-media-organizer cleanup
```

## Command Overview

| Command | Description | Use Case |
|---------|-------------|----------|
| `setup` | Interactive configuration setup | First-time setup |
| `config` | View and modify configuration | Configuration management |
| `scan` | Analyze media directories | Pre-organization analysis |
| `test` | Test parsing functionality | Debugging and validation |
| `organize` | Organize media files | Main organization workflow |
| `rollback` | Revert previous operations | Error recovery |
| `cleanup` | Database maintenance | System maintenance |

---

## Setup Command

Configure the application for first use.

```bash
plex-media-organizer setup [OPTIONS]
```

### Options

- `-f, --force` - Force reconfiguration even if config exists

### Description

The setup command guides you through the initial configuration process:

1. **Database Path** - Where to store the application database
2. **TMDB API Key** - For enhanced movie metadata (optional)
3. **Default Output Directory** - Where organized files should be placed
4. **Confidence Threshold** - Minimum confidence for file organization
5. **Network Mode** - Enable for network drives

### Example

```bash
plex-media-organizer setup
```

---

## Config Command

View and modify application configuration.

```bash
plex-media-organizer config [OPTIONS]
```

### Options

- `-v, --verbose` - Show detailed configuration

### Description

Displays current configuration settings including:
- Database location
- TMDB API integration status
- Default output directory
- Confidence thresholds
- Network mode settings

### Example

```bash
plex-media-organizer config --verbose
```

---

## Scan Command

Analyze a directory for media files and provide statistics.

```bash
plex-media-organizer scan <DIRECTORY> [OPTIONS]
```

### Arguments

- `DIRECTORY` - Directory to scan for media files

### Options

- `-v, --verbose` - Show detailed output
- `-n, --network` - Enable network mode for network drives

### Description

The scan command:
- Recursively searches for media files
- Provides file type statistics
- Shows potential organization candidates
- Identifies files that might be skipped
- Estimates organization time

### Example

```bash
plex-media-organizer scan /Volumes/media/movies --verbose
```

---

## Test Command

Test parsing functionality on media files.

```bash
plex-media-organizer test <DIRECTORY> [OPTIONS]
```

### Arguments

- `DIRECTORY` - Directory containing test files

### Options

- `-p, --patterns <PATTERNS>` - Test specific filename patterns
- `-v, --verbose` - Show detailed output
- `--use-cache` - Use database caching
- `--cache-stats` - Show cache statistics

### Description

The test command:
- Parses media filenames without moving files
- Shows parsing confidence scores
- Displays extracted metadata
- Tests cache performance
- Validates parser accuracy

### Example

```bash
plex-media-organizer test /path/to/test/files --use-cache --cache-stats
```

---

## Organize Command

Organize media files according to Plex naming conventions.

```bash
plex-media-organizer organize <DIRECTORY> [OPTIONS]
```

### Arguments

- `DIRECTORY` - Directory containing media files to organize

### Options

- `-o, --output <OUTPUT>` - Output directory (overrides config)
- `-p, --preview` - Preview changes without making them
- `-v, --verbose` - Show detailed output
- `-c, --copy` - Copy files instead of moving them
- `--min-confidence <CONFIDENCE>` - Minimum confidence threshold
- `--use-cache` - Use database caching
- `--organize-by-year` - Organize files by year (default: flat structure)
- `--max-parallel <MAX>` - Maximum parallel operations
- `--batch-size <SIZE>` - Batch size for processing

### Description

The organize command:
- Parses media filenames using advanced algorithms
- Renames files according to Plex conventions
- Supports both move and copy operations
- Provides preview mode for safe testing
- Integrates with TMDB for enhanced metadata
- Stores operation history for rollback

### Examples

```bash
# Preview organization without making changes
plex-media-organizer organize /path/to/movies --preview

# Organize with custom output directory
plex-media-organizer organize /path/to/movies -o /organized/movies

# Organize with copy operation (safer for testing)
plex-media-organizer organize /path/to/movies --copy

# Organize with year-based structure
plex-media-organizer organize /path/to/movies --organize-by-year
```

---

## Rollback Command

Revert a previous organization operation.

```bash
plex-media-organizer rollback <OPERATION_ID> [OPTIONS]
```

### Arguments

- `OPERATION_ID` - ID of the operation to rollback

### Options

- `-p, --preview` - Preview rollback without making changes
- `-v, --verbose` - Show detailed output

### Description

The rollback command:
- Restores files to their original locations
- Supports move, rename, and copy operations
- Provides preview mode for safe testing
- Shows detailed progress and results
- Handles missing files gracefully

### Examples

```bash
# Preview a rollback
plex-media-organizer rollback abc123-def456 --preview

# Execute rollback
plex-media-organizer rollback abc123-def456 --verbose
```

### Finding Operation IDs

Operation IDs are displayed after each organize command, or you can find them in the database:

```bash
# List recent operations (if you have database access)
sqlite3 ~/.config/plex-media-organizer/movies.db "SELECT operation_id, created_at FROM operations ORDER BY created_at DESC LIMIT 10;"
```

---

## Cleanup Command

Maintain database health and clean up old operations.

```bash
plex-media-organizer cleanup [OPTIONS]
```

### Options

- `--keep-days <DAYS>` - Keep operations for this many days (default: 30)
- `--keep-count <COUNT>` - Keep this many most recent operations (default: 10)
- `-p, --preview` - Preview cleanup without making changes
- `-v, --verbose` - Show detailed output

### Description

The cleanup command:
- Removes old operation history
- Optimizes database performance
- Reclaims disk space
- Updates database statistics
- Rebuilds indexes for better performance

### Examples

```bash
# Preview cleanup
plex-media-organizer cleanup --preview --verbose

# Clean up keeping only 5 most recent operations
plex-media-organizer cleanup --keep-count 5

# Clean up operations older than 7 days
plex-media-organizer cleanup --keep-days 7
```

---

## Configuration

### Default Configuration Location

- **macOS**: `~/Library/Application Support/plex-media-organizer/config.toml`
- **Linux**: `~/.config/plex-media-organizer/config.toml`
- **Windows**: `%APPDATA%\plex-media-organizer\config.toml`

### Configuration File Format

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
```

---

## Best Practices

### 1. Always Use Preview Mode First

```bash
# Test your organization before making changes
plex-media-organizer organize /path/to/media --preview
```

### 2. Use Network Mode for Network Drives

```bash
# Enable network mode for slower drives
plex-media-organizer organize /Volumes/network-drive --network
```

### 3. Regular Database Maintenance

```bash
# Clean up old operations monthly
plex-media-organizer cleanup --keep-count 50
```

### 4. Backup Important Data

```bash
# Backup your configuration
cp ~/.config/plex-media-organizer/config.toml ~/backup/
```

### 5. Monitor Operation History

Keep track of operation IDs for potential rollbacks:

```bash
# After each organize command, note the operation ID
✓ Operation saved to database with ID: abc123-def456-ghi789
```

---

## Troubleshooting

### Common Issues

1. **"No database available"**
   - Run `plex-media-organizer setup` to initialize the database

2. **"Permission denied"**
   - Check file permissions on source and destination directories

3. **"Network timeout"**
   - Use `--network` flag for network drives

4. **"Low confidence"**
   - Adjust confidence threshold with `--min-confidence`
   - Check filename format for parsing issues

### Getting Help

- Use `--help` with any command for detailed options
- Use `--verbose` for detailed output and debugging
- Check logs in the configuration directory

---

## Advanced Usage

### Batch Processing

```bash
# Process multiple directories
for dir in /path/to/movies/*; do
    plex-media-organizer organize "$dir" --preview
done
```

### Integration with Other Tools

```bash
# Use with find to process specific file types
find /path/to/media -name "*.mkv" -exec plex-media-organizer test {} \;
```

### Automation

```bash
# Create a script for regular organization
#!/bin/bash
plex-media-organizer organize /path/to/new/movies
plex-media-organizer cleanup --keep-count 20
```
