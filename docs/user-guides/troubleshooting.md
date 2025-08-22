# 🔧 Troubleshooting Guide

Solutions to common issues and problems when using Plex Media Organizer.

## 🚨 Quick Fixes

### Most Common Issues

| Problem | Quick Solution |
|---------|---------------|
| "TMDB API key not found" | Set `TMDB_API_KEY=your_key` or add to config file |
| "No files found" | Check file extensions (mkv, mp4, avi supported) |
| Low parsing success rate | Enable debug mode: `--log-level debug` |
| Permission denied | Fix file permissions: `chmod 755 /path/to/files` |
| Very slow processing | Reduce parallel processing: `--max-parallel 4` |

## 🔍 Diagnostic Commands

### Check Your Setup

```bash
# Verify installation
plex-media-organizer --version

# Check configuration
plex-media-organizer config --validate

# Test API connection
TMDB_API_KEY=your_key plex-media-organizer test "The.Matrix.1999.1080p.mkv"
```

### Get Detailed Information

```bash
# Debug a specific file
plex-media-organizer test "problematic-file.mkv" --log-level debug

# Analyze failed files
plex-media-organizer scan /movies --export-failures failures.txt
cat failures.txt

# Check system resources
plex-media-organizer scan /movies --show-stats
```

## 📋 Installation Issues

### Rust Not Found

**Problem**: `cargo: command not found` or `rustc: command not found`

**Solution**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Build Failures

**Problem**: Compilation errors during `cargo build`

**Solutions**:
```bash
# Update Rust to latest
rustup update

# Clean and rebuild
cargo clean
cargo build --release

# Check system dependencies (Linux)
sudo apt install build-essential pkg-config libssl-dev

# Check system dependencies (macOS)
xcode-select --install
```

### Permission Errors

**Problem**: Permission denied during installation or execution

**Solutions**:
```bash
# Don't use sudo with cargo
# If ownership is wrong:
sudo chown -R $USER:$USER ~/.cargo

# For binary installation location
sudo cp target/release/plex-media-organizer /usr/local/bin/
sudo chmod +x /usr/local/bin/plex-media-organizer
```

## 🔑 API and Configuration Issues

### TMDB API Key Problems

**Problem**: "Invalid API key" or "API key not found"

**Solutions**:
```bash
# Check if key is set
echo $TMDB_API_KEY

# Set in environment
export TMDB_API_KEY="your_actual_key_here"

# Or set in config file
plex-media-organizer config --set apis.tmdb_api_key "your_key"

# Verify API key works
curl "https://api.themoviedb.org/3/movie/550?api_key=your_key"
```

**Get a new API key**:
1. Go to [themoviedb.org](https://www.themoviedb.org/)
2. Create account → Settings → API → Create → Developer
3. Copy the v3 API key (not v4)

### Configuration File Issues

**Problem**: Configuration not loading or invalid

**Solutions**:
```bash
# Check where config is loaded from
plex-media-organizer config --sources

# Validate syntax
plex-media-organizer config --validate

# Create default config
plex-media-organizer setup --config-only

# Reset to defaults
plex-media-organizer config --reset
```

### Network Issues

**Problem**: API timeouts or connection failures

**Solutions**:
```bash
# Test network connectivity
curl -I https://api.themoviedb.org/

# Increase timeout
plex-media-organizer scan /movies --timeout 60

# Reduce request rate
plex-media-organizer scan /movies --rate-limit 5

# Use proxy if needed
export https_proxy=http://proxy.example.com:8080
```

## 📁 File Processing Issues

### Files Not Being Found

**Problem**: "No media files found" in directories with movies

**Solutions**:
```bash
# Check supported extensions
ls /movies/*.{mkv,mp4,avi,mov,wmv,flv,webm}

# Check file permissions
ls -la /movies/

# Enable recursive scanning
plex-media-organizer scan /movies --recursive

# Check for hidden files
ls -la /movies/
```

### Low Parsing Success Rate

**Problem**: Many files failing to parse correctly

**Debug Steps**:
```bash
# Get detailed analysis
plex-media-organizer scan /movies --log-level debug --export-failures failures.txt

# Check common failure reasons
cat failures.txt | jq '.failures[] | .reason' | sort | uniq -c

# Test specific problematic files
plex-media-organizer test "$(head -1 failures.txt)" --log-level debug
```

**Common Causes & Solutions**:

#### Very Messy Filenames
```bash
# Example problem: "mov1e.f1le.w1th.numb3rs.mkv"
# Solution: Manual override
plex-media-organizer test "mov1e.f1le.w1th.numb3rs.mkv" \
    --title "Movie Title" --year 2023
```

#### Non-English Titles
```bash
# Enable original title preference
plex-media-organizer scan /movies --prefer-original-titles

# Or configure permanently
plex-media-organizer config --set organization.original_titles.prefer_original_titles true
```

#### Very Old or Obscure Movies
```bash
# Lower confidence threshold
plex-media-organizer scan /movies --confidence-threshold 0.5

# Enable fuzzy matching
plex-media-organizer scan /movies --fuzzy-matching
```

### Parsing Errors

**Problem**: Specific files causing crashes or errors

**Solutions**:
```bash
# Skip problematic files
plex-media-organizer scan /movies --skip-on-error

# Process with single thread for debugging
plex-media-organizer scan /movies --max-parallel 1

# Check file integrity
ffprobe "problematic-file.mkv"
```

## 🚀 Performance Issues

### Very Slow Processing

**Problem**: Processing much slower than expected

**Solutions**:
```bash
# Check system resources
top
htop  # if available

# Reduce parallel processing
plex-media-organizer scan /movies --max-parallel 4

# Process in smaller batches
plex-media-organizer scan /movies --batch-size 100

# Disable cache temporarily
plex-media-organizer scan /movies --no-cache
```

### High Memory Usage

**Problem**: Process consuming too much RAM

**Solutions**:
```bash
# Enable low-memory mode
plex-media-organizer scan /movies --low-memory

# Reduce batch size
plex-media-organizer scan /movies --batch-size 50

# Process directories separately
for dir in /movies/*/; do
    plex-media-organizer scan "$dir"
    sleep 5
done
```

### API Rate Limiting

**Problem**: "Rate limit exceeded" errors

**Solutions**:
```bash
# Reduce request rate
plex-media-organizer scan /movies --rate-limit 3

# Increase delays between requests
plex-media-organizer scan /movies --request-delay 2000

# Check your TMDB account limits
# Free accounts: 1000 requests/day
```

## 📂 File Organization Issues

### Permission Denied

**Problem**: Cannot move or rename files

**Solutions**:
```bash
# Check file permissions
ls -la /movies/

# Fix ownership
sudo chown -R $USER:$USER /movies/

# Fix permissions
chmod -R 755 /movies/

# Use dry-run to test
plex-media-organizer organize /movies --dry-run
```

### Naming Conflicts

**Problem**: "File already exists" errors

**Solutions**:
```bash
# Handle duplicates automatically
plex-media-organizer organize /movies --duplicate-action rename

# Or skip duplicates
plex-media-organizer organize /movies --duplicate-action skip

# Or overwrite (careful!)
plex-media-organizer organize /movies --duplicate-action overwrite
```

### Rollback Issues

**Problem**: Cannot rollback organization

**Solutions**:
```bash
# Check backup file exists
ls -la ~/backups/organize-*.json

# Verify backup format
head ~/backups/organize-backup.json

# Force rollback
plex-media-organizer rollback ~/backups/organize-backup.json --force

# Manual rollback if automated fails
jq -r '.moves[] | "mv \"\(.to)\" \"\(.from)\""' backup.json | bash
```

## 🔧 System-Specific Issues

### macOS Issues

**Problem**: "Developer cannot be verified" error

**Solutions**:
```bash
# Allow app in System Preferences
# Security & Privacy → Allow apps downloaded from → App Store and identified developers

# Or remove quarantine
xattr -d com.apple.quarantine plex-media-organizer

# Build from source instead
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release
```

### Linux Issues

**Problem**: Missing dependencies

**Solutions**:
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# RHEL/CentOS/Fedora
sudo dnf install gcc make openssl-devel pkg-config

# Check library linking
ldd target/release/plex-media-organizer
```

### Windows Issues

**Problem**: Build failures or missing tools

**Solutions**:
```powershell
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

# Or use chocolatey
choco install visualstudio2022buildtools

# Check PATH includes Rust
$env:PATH -split ';' | Select-String cargo
```

## 🧪 Advanced Troubleshooting

### Enable Debug Logging

```bash
# Maximum debugging
RUST_LOG=debug plex-media-organizer scan /movies --log-level debug

# Log to file
plex-media-organizer scan /movies --log-level debug --log-file debug.log

# Focus on specific modules
RUST_LOG=plex_media_organizer::tmdb_client=debug plex-media-organizer scan /movies
```

### Performance Profiling

```bash
# Install profiling tools
cargo install cargo-profiler

# Profile performance
cargo profiler callgrind --bin plex-media-organizer -- scan /movies

# Memory profiling
valgrind --tool=memcheck target/release/plex-media-organizer scan /movies
```

### Database Issues

```bash
# Check cache database
sqlite3 ~/.cache/plex-media-organizer/cache.db ".tables"

# Clear corrupted cache
rm ~/.cache/plex-media-organizer/cache.db

# Reset all cache
plex-media-organizer cache --clear-all
```

## 📞 Getting Help

### Before Asking for Help

1. **Check this troubleshooting guide**
2. **Search existing issues**: [GitHub Issues](https://github.com/lijunzh/plex-media-organizer/issues)
3. **Gather diagnostic information**:
   ```bash
   # System info
   uname -a
   plex-media-organizer --version
   
   # Error details
   plex-media-organizer scan /movies --log-level debug 2>&1 | tee debug.log
   ```

### Create a Bug Report

Include this information:

- **Operating System**: `uname -a`
- **Version**: `plex-media-organizer --version`
- **Command used**: Exact command that failed
- **Error message**: Complete error output
- **Sample files**: Names of files that fail (no need to share actual files)
- **Configuration**: Your config file (remove API keys)

### Community Support

- **GitHub Discussions**: [Project Discussions](https://github.com/lijunzh/plex-media-organizer/discussions)
- **Issues**: [Report Bugs](https://github.com/lijunzh/plex-media-organizer/issues/new)
- **Documentation**: [User Guide](user-guide.md) | [Configuration](configuration.md)

---

## 🎯 Prevention Tips

### Best Practices

1. **Always test first**: Use `--dry-run` before actual organization
2. **Create backups**: Use `--backup` for important operations
3. **Start small**: Test with 10-20 files before processing thousands
4. **Monitor resources**: Check disk space and memory before large operations
5. **Keep logs**: Save debug logs for troubleshooting

### Regular Maintenance

```bash
# Weekly cache cleanup
plex-media-organizer cache --cleanup

# Monthly configuration validation
plex-media-organizer config --validate

# Backup important organization results
cp ~/backups/organize-*.json ~/important-backups/
```

---

**💡 Remember**: Most issues are configuration or permission related. Start with the quick fixes and work your way down to more complex solutions!
