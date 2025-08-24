# Features Guide

This guide provides detailed information about all features of Plex Media Organizer.

## Core Features

### Intelligent Parsing

Plex Media Organizer uses advanced algorithms to parse media filenames and extract meaningful information.

#### Pattern Recognition
- **Movie Detection**: Identifies movies with year, title, and quality information
- **Series Detection**: Recognizes TV show patterns with season/episode numbers
- **Anime Detection**: Specialized parsing for anime titles and episode patterns
- **Documentary Detection**: Handles documentary and educational content

#### Quality Detection
- **Resolution**: Identifies 4K, 1080p, 720p, and other resolutions
- **Source**: Detects Blu-ray, Web-DL, HDRip, and other sources
- **Codec**: Recognizes x264, x265, H.264, H.265, and other codecs
- **Audio**: Identifies DTS, AC3, TrueHD, and other audio formats

#### Confidence Scoring
- **Accuracy Assessment**: Provides confidence levels for parsing accuracy
- **Multiple Factors**: Considers title clarity, year presence, quality indicators
- **Threshold Control**: Configurable confidence threshold for automatic organization
- **Manual Review**: Low-confidence files can be reviewed manually

### Plex Integration

Designed specifically for Plex Media Server compatibility.

#### Naming Conventions
- **Plex Standards**: Follows Plex's recommended naming structure
- **Flat Organization**: Optimized for Plex's flat directory structure
- **Metadata Preservation**: Maintains metadata for Plex indexing
- **Compatibility**: Works seamlessly with Plex's scanning and indexing

#### Directory Structure
```
Movies/
├── Movie Title (Year)/
│   ├── Movie Title (Year).mkv
│   └── extras/
├── Another Movie (Year)/
│   └── Another Movie (Year).mkv
```

### Multi-language Support

Comprehensive support for international content.

#### Supported Languages
- **English**: Primary language with full support
- **Chinese**: Simplified and Traditional Chinese characters
- **Japanese**: Hiragana, Katakana, and Kanji support
- **Arabic**: Arabic script and right-to-left text
- **Russian**: Cyrillic script support
- **Korean**: Hangul script support
- **Other Languages**: Latin-based scripts and Unicode support

#### Language Detection
- **Automatic Detection**: Identifies language based on character sets
- **Mixed Content**: Handles bilingual titles and mixed scripts
- **Title Cleaning**: Removes language-specific artifacts
- **Preservation**: Maintains original language information

### TMDB Integration

Enhanced metadata using The Movie Database API.

#### Enhanced Parsing
- **Title Validation**: Verifies movie titles against TMDB database
- **Year Correction**: Corrects incorrect release years
- **Alternative Titles**: Handles international and alternative titles
- **Confidence Boosting**: Improves parsing confidence with external data

#### Metadata Features
- **Movie Information**: Retrieves official titles, years, and genres
- **Fallback Support**: Works without API key for basic functionality
- **Caching**: Intelligent caching to reduce API calls
- **Rate Limiting**: Respects TMDB API rate limits

### Database Features

Persistent storage for operation history and caching.

#### Operation History
- **Complete Tracking**: Records all organization operations
- **Metadata Storage**: Stores file information and parsing results
- **Timestamp Tracking**: Records when operations were performed
- **User Information**: Tracks who performed operations

#### Rollback Support
- **Full Revert**: Revert any previous operation safely
- **File Restoration**: Restore original file locations and names
- **Metadata Preservation**: Maintain original file metadata
- **Batch Operations**: Support for rolling back multiple files

#### Caching System
- **Parsing Cache**: Caches parsing results for improved performance
- **TMDB Cache**: Caches API responses to reduce external calls
- **File Cache**: Caches file information for faster scanning
- **Intelligent Invalidation**: Automatically invalidates stale cache entries

## Advanced Features

### Technical Terms Management

Configurable filtering of technical terms from movie titles.

#### Term Categories
- **Release Groups**: Filter out release group names (YIFY, YTS, etc.)
- **Codecs**: Remove video/audio codec information (x264, DTS, etc.)
- **Quality Indicators**: Filter quality terms (1080p, BluRay, etc.)
- **Source Information**: Remove source indicators (Web-DL, HDRip, etc.)
- **Custom Terms**: Add your own terms to filter

#### Benefits
- **Cleaner Titles**: Removes technical clutter from movie titles
- **Better Parsing**: Improves parsing accuracy by focusing on content
- **Plex Optimization**: Creates cleaner titles for Plex indexing
- **Customizable**: Fully configurable through settings

### Performance Optimization

Optimized for handling large media libraries.

#### Parallel Processing
- **Configurable Parallelism**: Adjustable number of concurrent operations
- **Batch Processing**: Process files in configurable batches
- **Memory Management**: Efficient memory usage for large directories
- **Progress Tracking**: Real-time progress indicators

#### Caching Strategy
- **Multi-level Caching**: Database, memory, and file system caching
- **Intelligent Invalidation**: Automatic cache management
- **Performance Monitoring**: Track and optimize performance
- **Resource Management**: Efficient use of system resources

### Safety Features

Comprehensive safety measures to protect your media files.

#### Preview Mode
- **Dry Run**: Test operations without making changes
- **Detailed Output**: Show exactly what changes would be made
- **Confidence Display**: Show parsing confidence for each file
- **Validation**: Verify operations before execution

#### Error Handling
- **Graceful Failures**: Handle errors without data loss
- **Detailed Logging**: Comprehensive error reporting
- **Recovery Options**: Multiple recovery strategies
- **Validation**: Comprehensive input validation

#### File Protection
- **Backup Creation**: Automatic backup of original files
- **Permission Preservation**: Maintain original file permissions
- **Metadata Preservation**: Preserve all file metadata
- **Integrity Checking**: Verify file integrity after operations

## CLI Features

### Command Line Interface

Powerful command-line interface for automation and scripting.

#### Command Structure
- **Consistent Syntax**: Uniform command structure across all operations
- **Help System**: Comprehensive help and documentation
- **Tab Completion**: Shell completion for commands and options
- **Verbose Output**: Detailed output for debugging and monitoring

#### Automation Support
- **Scripting**: Full support for shell scripting and automation
- **Batch Processing**: Process multiple directories in sequence
- **Scheduled Operations**: Support for cron jobs and scheduled tasks
- **Integration**: Easy integration with other tools and workflows

### Configuration Management

Flexible configuration system for customization.

#### Configuration Options
- **File-based Config**: TOML-based configuration file
- **Environment Variables**: Support for environment-based configuration
- **Command-line Overrides**: Override settings via command line
- **Validation**: Automatic configuration validation

#### Migration Support
- **Version Migration**: Automatic migration between versions
- **Backward Compatibility**: Maintain compatibility with older configs
- **Import/Export**: Support for configuration backup and restore
- **Templates**: Pre-configured templates for common use cases

## Integration Features

### External Tool Integration

Seamless integration with other media management tools.

#### Plex Integration
- **Direct Integration**: Works directly with Plex Media Server
- **Metadata Sync**: Synchronize metadata with Plex
- **Scan Triggering**: Trigger Plex library scans after organization
- **Compatibility**: Full compatibility with Plex naming conventions

#### File System Integration
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Network Support**: Support for network-attached storage
- **Permission Handling**: Proper handling of file permissions
- **Symbolic Links**: Support for symbolic links and junctions

### API Integration

Programmatic access to functionality.

#### REST API (Planned)
- **HTTP Interface**: RESTful API for web integration
- **JSON Responses**: Standard JSON response format
- **Authentication**: Secure authentication and authorization
- **Rate Limiting**: Built-in rate limiting and throttling

#### Plugin System (Planned)
- **Extensible Architecture**: Plugin system for custom functionality
- **Custom Parsers**: Support for custom parsing rules
- **Integration Hooks**: Hooks for external system integration
- **Community Extensions**: Community-contributed plugins

## Performance Features

### Scalability

Designed to handle media libraries of any size.

#### Large Library Support
- **Millions of Files**: Support for libraries with millions of files
- **Distributed Processing**: Support for distributed processing
- **Incremental Updates**: Efficient incremental processing
- **Resource Optimization**: Optimized resource usage

#### Monitoring and Metrics
- **Performance Tracking**: Track processing performance
- **Resource Monitoring**: Monitor system resource usage
- **Progress Reporting**: Real-time progress reporting
- **Analytics**: Performance analytics and optimization suggestions

### Optimization

Continuous optimization for better performance.

#### Algorithm Optimization
- **Efficient Parsing**: Optimized parsing algorithms
- **Memory Management**: Efficient memory usage patterns
- **I/O Optimization**: Optimized file system operations
- **Caching Strategy**: Intelligent caching for performance

#### Hardware Optimization
- **Multi-core Support**: Full utilization of multi-core processors
- **SSD Optimization**: Optimized for solid-state drives
- **Network Optimization**: Optimized for network storage
- **GPU Acceleration**: Support for GPU acceleration (planned)

## Future Features

### Planned Enhancements

- **TV Show Support**: Full TV show organization with episode detection
- **Music Organization**: Music file organization and metadata
- **Web Interface**: Web-based user interface
- **Scheduled Organization**: Automated scheduled organization
- **Cloud Storage**: Integration with cloud storage providers
- **Advanced Metadata**: Enhanced metadata extraction and management
- **Plugin System**: Extensible plugin architecture
- **Mobile App**: Mobile application for remote management
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
