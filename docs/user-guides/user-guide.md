# Plex Media Organizer - User Guide

## 📖 Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Commands](#basic-commands)
3. [Advanced Usage](#advanced-usage)
4. [Network Drive Optimization](#network-drive-optimization)
5. [Configuration](#configuration)
6. [Troubleshooting](#troubleshooting)
7. [Examples](#examples)

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ installed
- TMDB API key (free at [themoviedb.org](https://www.themoviedb.org/settings/api))
- Media files to organize

### Quick Setup
```bash
# 1. Clone and build
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release

# 2. Configure API keys
./target/release/plex-media-organizer setup

# 3. Test with a sample file
./target/release/plex-media-organizer test "Avengers.Endgame.2019.1080p.BluRay.x264.mkv"
```

## 🎯 Basic Commands

### Scan Directory
```bash
# Basic scan
plex-media-organizer scan /path/to/movies

# With detailed output
plex-media-organizer scan /path/to/movies --verbose

# Network drive optimization
plex-media-organizer scan /path/to/movies --network-mode
```

### Organize Files
```bash
# Preview changes first (recommended)
plex-media-organizer organize /path/to/movies --preview

# Actually organize files
plex-media-organizer organize /path/to/movies

# With backup
plex-media-organizer organize /path/to/movies --backup /path/to/backup
```

### Test Parsing
```bash
# Test single file
plex-media-organizer test "movie.mkv"

# Test directory
plex-media-organizer test /path/to/movies

# Test full workflow
plex-media-organizer test /path/to/movies --organize --preview
```

## Network Drive Optimization

The tool automatically detects network drives and applies optimizations for better performance on SMB/NFS shares.

### Automatic Detection
- **macOS**: Detects paths in `/Volumes/` with spaces (common for network mounts)
- **Linux**: Detects paths in `/mnt/` or `/media/` containing `smb`, `nfs`, or `cifs`
- **Windows**: Detects UNC paths (`\\server\share`)

### Performance Features
- **Reduced Concurrency**: Lower parallel processing to avoid overwhelming network
- **Smaller Batches**: Process files in smaller groups for better responsiveness
- **Sequential Discovery**: Scan directories sequentially instead of parallel
- **Minimal File System Calls**: Optimize metadata extraction to avoid full file reads
- **Enhanced Progress**: Better progress reporting for long-running operations

### Efficient Metadata Extraction
The tool now uses smart metadata extraction that prioritizes:
1. **External metadata files** (`.nfo`, `.txt`, `.info`, `.json`) - Highest priority
2. **Media file headers** - Reads only metadata sections, not entire files
3. **Filename parsing** - Fallback to intelligent filename analysis

This eliminates the need to read entire media files, dramatically improving performance on network drives.

### Performance Comparison
**Before (reading entire files):**
- ❌ **Hanging/Stuck** - Taking over an hour for just 25 files
- ❌ **Massive network I/O** - Reading entire file content (hundreds of GB)
- ❌ **300KB/s write speed bottleneck**

**After (efficient metadata extraction):**
- ✅ **49.22 seconds** for 420 files
- ✅ **8.5 files/second** processing speed
- ✅ **100% success rate**
- ✅ **No full file reading** - only metadata and headers

### Best Practices for Network Drives
1. **Use wired connections** when possible for better stability
2. **Monitor network usage** to avoid overwhelming the connection
3. **Run during off-peak hours** if sharing bandwidth with others
4. **Consider local processing** for very large collections
5. **Use preview mode first** to verify organization plans

### Troubleshooting Network Issues

#### **Very Slow Performance**
```bash
# Reduce concurrency further
plex-media-organizer scan /network/drive \
    --network-mode \
    --max-parallel 1 \
    --batch-size 10
```

#### **Connection Timeouts**
```bash
# Check network connectivity
ping your-network-server
# Test file access
ls /network/drive

# Use smaller batches
plex-media-organizer scan /network/drive \
    --network-mode \
    --batch-size 10
```

#### **Permission Issues**
```bash
# Check file permissions
ls -la /network/drive

# Ensure read/write access
chmod -R 755 /network/drive  # If you own the files
```

## ⚙️ Advanced Usage

### Configuration Management
```bash
# Show current configuration
plex-media-organizer config

# Show config file location
plex-media-organizer config --path

# Reconfigure API keys
plex-media-organizer setup --force
```

### Rollback Operations
```bash
# Preview rollback
plex-media-organizer rollback operation_result_123.json --preview

# Execute rollback
plex-media-organizer rollback operation_result_123.json
```

### Cleanup Old Files
```bash
# Preview cleanup
plex-media-organizer cleanup --preview

# Clean up files older than 30 days
plex-media-organizer cleanup

# Custom retention policy
plex-media-organizer cleanup --keep-days 60 --keep-count 50
```

## 🎛️ Configuration

### Environment Variables
```bash
# API Keys
export TMDB_API_KEY="your_tmdb_api_key"

# Organization settings
export PMO_OUTPUT_DIRECTORY="/path/to/organized"
export PMO_PREFER_ORIGINAL_TITLES="true"
```

### Configuration File
```toml
# ~/.config/plex-media-organizer/config.toml
[apis]
tmdb_api_key = "your_tmdb_api_key"

[organization]
output_directory = "/path/to/organized"
prefer_original_titles = true
include_english_subtitle = false

[processing]
max_parallel_files = 16
batch_size = 100
```

## 🔧 Troubleshooting

### Common Issues

#### **API Key Issues**
```bash
# Check if API key is set
plex-media-organizer config

# Reconfigure if needed
plex-media-organizer setup --force
```

#### **Permission Errors**
```bash
# Check file permissions
ls -la /path/to/movies

# Fix permissions if needed
chmod -R 755 /path/to/movies
```

#### **Slow Performance**
```bash
# For local drives: increase concurrency
plex-media-organizer scan /movies --max-parallel 32

# For network drives: use network mode
plex-media-organizer scan /network/drive --network-mode
```

### Debug Mode
```bash
# Enable verbose output
plex-media-organizer scan /movies --verbose

# Check for specific issues
RUST_LOG=debug plex-media-organizer scan /movies
```

## 📋 Examples

### Basic Workflow
```bash
# 1. Setup
plex-media-organizer setup

# 2. Test with sample file
plex-media-organizer test "The.Matrix.1999.1080p.BluRay.x264.mkv"

# 3. Scan directory
plex-media-organizer scan /home/user/Movies

# 4. Preview organization
plex-media-organizer organize /home/user/Movies --preview

# 5. Organize with backup
plex-media-organizer organize /home/user/Movies --backup /home/user/backup
```

### Network Drive Workflow
```bash
# 1. Scan network drive with optimizations
plex-media-organizer scan /Volumes/My\ Network\ Drive --network-mode

# 2. Preview organization
plex-media-organizer organize /Volumes/My\ Network\ Drive \
    --network-mode \
    --preview \
    --backup /Users/me/backup

# 3. Execute organization
plex-media-organizer organize /Volumes/My\ Network\ Drive \
    --network-mode \
    --backup /Users/me/backup
```

### Large Library Processing
```bash
# For very large libraries (1000+ files)
plex-media-organizer scan /large/movie/library \
    --max-parallel 8 \
    --batch-size 200

# For network drives with large libraries
plex-media-organizer scan /network/large/library \
    --network-mode \
    --max-parallel 2 \
    --batch-size 25
```

### Batch Processing Multiple Directories
```bash
# Process multiple directories
for dir in /movies1 /movies2 /movies3; do
    echo "Processing $dir..."
    plex-media-organizer scan "$dir" --network-mode
    plex-media-organizer organize "$dir" \
        --network-mode \
        --preview \
        --backup /backup/$(basename "$dir")
done
```

---

## 🆘 Getting Help

- **Documentation**: Check the [docs/](../docs/) directory
- **Issues**: Report bugs on [GitHub](https://github.com/lijunzh/plex-media-organizer/issues)
- **Examples**: See [examples/](../docs/examples/) for more use cases
- **Configuration**: See [configuration.md](configuration.md) for detailed settings
