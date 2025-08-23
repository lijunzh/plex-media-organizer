# User Guide

## Overview

The Plex Media Organizer provides a comprehensive CLI workflow for intelligent media file organization. This guide explains how to use the tool effectively and understand its behavior.

## Current Approach & Features

### Intelligent Parsing Strategy
The organizer uses an **intelligent parsing approach** that balances accuracy with coverage:

- **Multi-language Support**: Handles English, Chinese, Japanese, Arabic, Russian, and other languages
- **TMDB Integration**: Enhanced metadata using The Movie Database API
- **Database Backed**: Persistent operation history with rollback capabilities
- **Preview Mode**: Safe testing before making changes
- **Confidence Scoring**: Intelligent confidence calculation for parsing accuracy

### Complete CLI Workflow

The application provides 7 core commands for a complete media organization workflow:

1. **`setup`** - Interactive configuration setup
2. **`config`** - View and modify configuration
3. **`scan`** - Analyze media directories
4. **`test`** - Test parsing functionality
5. **`organize`** - Organize media files
6. **`rollback`** - Revert previous operations
7. **`cleanup`** - Database maintenance

## Configuration

### Confidence Threshold
```bash
# High confidence (default) - high accuracy, lower coverage
--min-confidence 0.7

# Moderate confidence - balanced approach
--min-confidence 0.6

# Lower confidence - higher coverage, review carefully
--min-confidence 0.5
```

### Operation Modes
```bash
# Move files (default) - efficient for large files
plex-media-organizer organize /path/to/movies

# Copy files - safer for testing
plex-media-organizer organize /path/to/movies --copy

# Preview mode - test without making changes
plex-media-organizer organize /path/to/movies --preview
```

### Organization Structure
```bash
# Flat structure (default) - optimal for Plex
plex-media-organizer organize /path/to/movies

# Year-based structure - organize by release year
plex-media-organizer organize /path/to/movies --organize-by-year
```

## Examples

### Chinese Movies
```
Input:  金手指.The.Goldfinger.2023.2160p.WEB-DL.mp4
Output: 金手指 - The Goldfinger (2023) [2160p] [WEB-DL].mp4
```

### Japanese Movies
```
Input:  千と千尋の神隠し.Spirited.Away.2001.WEB-DL.mkv
Output: 千と千尋の神隠し - Spirited Away (2001) [WEB-DL].mkv
```

### English Movies
```
Input:  The.Matrix.1999.1080p.BluRay.mkv
Output: The Matrix (1999) [1080p] [BluRay].mkv
```

### Bilingual Titles
```
Input:  钢铁侠.Iron.Man.2008.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv
Output: 钢铁侠 - Iron Man (2008) [2160p] [BluRay].mkv
```

## Complete Workflow Examples

### Basic Organization
```bash
# 1. Setup (first time only)
plex-media-organizer setup

# 2. Scan directory
plex-media-organizer scan /path/to/movies

# 3. Test parsing
plex-media-organizer test /path/to/movies --use-cache

# 4. Preview organization
plex-media-organizer organize /path/to/movies --preview

# 5. Organize files
plex-media-organizer organize /path/to/movies
```

### Advanced Organization
```bash
# Organize with custom settings
plex-media-organizer organize /path/to/movies \
  --output /organized/movies \
  --copy \
  --min-confidence 0.6 \
  --use-cache \
  --max-parallel 8
```

### Operation Management
```bash
# View configuration
plex-media-organizer config --verbose

# Rollback operation (if needed)
plex-media-organizer rollback <operation-id> --preview
plex-media-organizer rollback <operation-id>

# Cleanup old operations
plex-media-organizer cleanup --keep-count 20
```

## Troubleshooting

### Common Issues

1. **Files being skipped**: This is expected behavior. Check the skipped list and review manually.
2. **Low confidence scores**: Adjust confidence threshold or check filename format.
3. **TMDB API errors**: Verify API key or run without TMDB integration.
4. **Permission errors**: Check file permissions and network drive access.

### Getting Help

1. **Review skipped files**: Always check the list of skipped files
2. **Use preview mode**: Always run with `--preview` first
3. **Check TMDB**: Verify the movie exists in TMDB with the expected title
4. **Use verbose mode**: Add `--verbose` for detailed output

## Safety Features

### Preview Mode
All destructive operations support preview mode:
```bash
# Preview organization
plex-media-organizer organize /path/to/movies --preview

# Preview rollback
plex-media-organizer rollback <operation-id> --preview

# Preview cleanup
plex-media-organizer cleanup --preview
```

### Rollback System
Complete operation history with rollback capability:
```bash
# Rollback any previous operation
plex-media-organizer rollback <operation-id>
```

### Database Backed
- **Persistent History**: All operations stored in SQLite database
- **Operation Tracking**: Detailed metadata for each operation
- **Automatic Cleanup**: Built-in maintenance tools

## Performance Features

### Caching
```bash
# Use database caching for improved performance
plex-media-organizer test /path/to/movies --use-cache
plex-media-organizer organize /path/to/movies --use-cache
```

### Parallel Processing
```bash
# Configure parallel operations for large directories
plex-media-organizer organize /path/to/movies --max-parallel 16
```

### Network Optimization
```bash
# Enable network mode for network drives
plex-media-organizer scan /path/to/movies --network
plex-media-organizer organize /path/to/movies --network
```

## Best Practices

### Before Organizing
1. **Always use preview mode first**: `--preview --verbose`
2. **Review the output carefully**: Check both organized and skipped files
3. **Start with small directories**: Test with a few files first
4. **Use copy mode for testing**: `--copy` before moving files

### When Files Are Skipped
1. **Don't panic**: This is expected behavior
2. **Review the skipped list**: Check why each file was skipped
3. **Manual verification**: Verify file titles in TMDB
4. **Consider lower confidence**: Use `--min-confidence 0.5` for more permissive matching

### Safety Tips
1. **Start small**: Test with a few files first
2. **Use preview mode**: Never skip the preview step
3. **Review changes**: Always review before applying
4. **Keep operation IDs**: Note operation IDs for potential rollback
5. **Regular cleanup**: Run cleanup periodically to maintain database health

## Advanced Usage

### Network Drives
The organizer is optimized for SMB/NFS network drives:
- Efficient file operations
- Proper error handling for network issues
- Progress reporting for large operations

### Batch Processing
For large directories:
1. **Start with preview**: `--preview --verbose`
2. **Review results**: Check organized vs. skipped
3. **Process in batches**: Organize smaller directories first
4. **Monitor progress**: Use `--verbose` for detailed output

### Custom Configuration
See [Configuration Guide](configuration.md) for advanced configuration options.

## Limitations

### Current Limitations
- **Movie Focus**: Currently optimized for movies (TV shows in next iteration)
- **Basic Technical Terms**: Some technical terms may remain in extracted titles
- **Limited Search Strategies**: Primary TMDB integration with filename fallback

### Future Improvements
- **TV Show Support**: Episode detection and season organization
- **Enhanced Technical Terms**: Configurable, extensible pattern database
- **Advanced TMDB Search**: Multiple search strategies and year range matching
- **Web Interface**: Browser-based management interface
- **Scheduled Operations**: Automated organization workflows
