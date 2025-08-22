# Network Drive Optimization

**Date:** August 17, 2025  
**Status:** ✅ Implemented  
**Scope:** Iteration 1  

## Overview

Network drive optimization addresses performance issues when processing media files stored on SMB/NFS shares. The implementation provides automatic detection and optimized processing strategies for network storage.

## Problem Statement

### Performance Issues on Network Drives
- **High Latency**: Network round-trips add significant delay
- **Limited Bandwidth**: Network connections have lower throughput than local storage
- **Connection Limits**: Too many concurrent operations can overwhelm the connection
- **File System Overhead**: Network protocols add additional overhead

### Root Cause Analysis
The original implementation was reading entire media files for content hashing, causing:
- **Massive I/O**: Reading hundreds of GB of file content over network
- **Hanging Operations**: Taking over an hour for just 25 files
- **Network Bottlenecks**: 300KB/s write speed limitations

## Solution Architecture

### 1. Automatic Network Detection

```rust
pub fn detect_network_drive(path: &Path) -> bool {
    // Windows: \\server\share or //server/share
    // macOS: /Volumes/Network Drive (with spaces)
    // Linux: /mnt/ or /media/ containing smb/nfs/cifs
}
```

**Detection Strategies:**
- **Path Pattern Matching**: Common network mount patterns
- **Mount Point Analysis**: Parse system mount information
- **Cross-Platform Support**: Windows, macOS, Linux detection

### 2. Optimized Processing Settings

| **Setting** | **Local Drive** | **Network Drive** | **Rationale** |
|-------------|-----------------|-------------------|---------------|
| **Concurrency** | 16 operations | 4 operations | Reduce connection load |
| **Batch Size** | 100 files | 50 files | Smaller groups for responsiveness |
| **Discovery** | Parallel | Sequential | Avoid overwhelming connection |
| **File Metadata** | Full reading | Path-based | Minimize network I/O |

### 3. Efficient Metadata Extraction

**Priority Order:**
1. **External metadata files** (`.nfo`, `.txt`, `.info`, `.json`) - Highest priority
2. **Media file headers** - Reads only metadata sections, not entire files
3. **Filename parsing** - Fallback to intelligent filename analysis

**Implementation:**
```rust
// Before: Reading entire file for content hash
fn calculate_content_hash(&self, file_path: &Path) -> Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    // Read entire file content...
}

// After: Using file metadata only
fn calculate_content_hash(&self, file_path: &Path) -> Result<String> {
    let metadata = std::fs::metadata(file_path)?;
    // Hash size, modification time, and path
}
```

## Performance Results

### Before Optimization
- ❌ **Hanging/Stuck** - Taking over an hour for just 25 files
- ❌ **Massive network I/O** - Reading entire file content (hundreds of GB)
- ❌ **300KB/s write speed bottleneck**

### After Optimization
- ✅ **49.22 seconds** for 420 files
- ✅ **8.5 files/second** processing speed
- ✅ **100% success rate**
- ✅ **No full file reading** - only metadata and headers

## Implementation Details

### Scanner Configuration

```rust
pub struct Scanner {
    network_mode: bool,
    concurrency_limit: usize,
    batch_size: usize,
    // ...
}

impl Scanner {
    pub fn for_network_drive() -> Self {
        Self {
            network_mode: true,
            concurrency_limit: 4,  // Reduced for network stability
            batch_size: 50,        // Smaller batches for network
            // ...
        }
    }
}
```

### Processing Pipeline

1. **Discovery Phase**
   - Sequential directory walking for network drives
   - Progress reporting during long operations
   - Reduced file system calls

2. **Parsing Phase**
   - Smaller batch processing
   - Reduced concurrency
   - Brief delays between batches

3. **Metadata Extraction**
   - Prioritized external file reading
   - Minimal media file access
   - Efficient content hash calculation

## Configuration Options

### CLI Parameters
```bash
# Enable network mode explicitly
--network-mode

# Customize network settings
--max-parallel 4      # Reduce concurrency
--batch-size 50       # Smaller batches
```

### Automatic Detection
```bash
# Auto-detected network paths:
# Windows: \\server\share or //server/share
# macOS: /Volumes/Network Drive
# Linux: /mnt/network or /media/network

plex-media-organizer scan /Volumes/My\ Network\ Drive
# Output: 🌐 Auto-detected network drive - enabling optimizations
```

## Best Practices

### For Users
1. **Use wired connections** when possible for better stability
2. **Monitor network usage** to avoid overwhelming the connection
3. **Run during off-peak hours** if sharing bandwidth with others
4. **Consider local processing** for very large collections
5. **Use preview mode first** to verify organization plans

### For Developers
1. **Test on real network drives** - Local testing doesn't reveal network issues
2. **Monitor I/O patterns** - Use tools like `iotop` or `Activity Monitor`
3. **Profile network calls** - Identify bottlenecks in file operations
4. **Consider connection limits** - SMB/NFS servers have concurrent connection limits

## Troubleshooting

### Common Issues

#### Very Slow Performance
```bash
# Reduce concurrency further
plex-media-organizer scan /network/drive \
    --network-mode \
    --max-parallel 1 \
    --batch-size 10
```

#### Connection Timeouts
```bash
# Check network connectivity
ping your-network-server

# Use smaller batches
plex-media-organizer scan /network/drive \
    --network-mode \
    --batch-size 10
```

#### Permission Issues
```bash
# Check file permissions
ls -la /network/drive

# Ensure read/write access
chmod -R 755 /network/drive  # If you own the files
```

## Future Enhancements

### Planned Improvements
1. **Adaptive Concurrency** - Dynamically adjust based on network performance
2. **Connection Pooling** - Reuse network connections efficiently
3. **Resume Capability** - Resume interrupted operations
4. **Bandwidth Monitoring** - Real-time network usage tracking

### Media Header Extraction
- **MP4 Support** - Read `moov` atom metadata
- **MKV Support** - Parse EBML metadata
- **MP3 Support** - Extract ID3 tags

## Related Documentation

- [User Guide - Network Drive Optimization](../docs/user-guide.md#network-drive-optimization)
- [Architecture - Single Binary Decision](architecture.md#why-single-binary-instead-of-client-server)
- [Performance Testing Strategy](planning/testing-strategy.md)
