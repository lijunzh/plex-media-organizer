# 📖 Complete User Guide

Comprehensive guide to using Plex Media Organizer for all your media organization needs.

## 📚 Table of Contents

- [Getting Started](#getting-started)
- [Core Commands](#core-commands)
- [Advanced Usage](#advanced-usage)
- [Real-World Workflows](#real-world-workflows)
- [Performance Tuning](#performance-tuning)
- [Best Practices](#best-practices)

---

## 🚀 Getting Started

### Your First Organization Project

After [installation](installation.md) and [quick start](quick-start.md), here's how to organize your first library:

#### 1. Plan Your Organization
```bash
# Understand your current library
find /path/to/movies -name "*.mkv" -o -name "*.mp4" | wc -l
# Shows: 1,247 movie files found

# Check free space
df -h /path/to/organized/
# Make sure you have enough space
```

#### 2. Test with Small Sample
```bash
# Create test directory with 10-20 files
mkdir ~/movie-test
cp /path/to/movies/Movie.*.mkv ~/movie-test/

# Test organization
plex-media-organizer scan ~/movie-test --dry-run
```

#### 3. Full Organization
```bash
# Once satisfied, organize the full library
plex-media-organizer organize /path/to/movies --output /path/to/organized --dry-run

# If results look good:
plex-media-organizer organize /path/to/movies --output /path/to/organized
```

---

## 🔧 Core Commands

### scan - Analyze Your Media

The `scan` command analyzes media files without changing anything.

```bash
# Basic scan
plex-media-organizer scan /path/to/movies

# Scan with detailed output
plex-media-organizer scan /path/to/movies --verbose

# Scan recursively (default behavior)
plex-media-organizer scan /path/to/movies --recursive

# Scan single directory only
plex-media-organizer scan /path/to/movies --no-recursive

# Export scan results
plex-media-organizer scan /path/to/movies --output scan-results.json
```

**Example Output:**
```
🔍 Scanning: /Users/you/Movies
📁 Found: 1,247 media files
⚡ Processing...

✅ Results:
  • Successfully parsed: 1,195/1,247 (95.8%)
  • TMDB matches: 1,180/1,195 (98.7%)
  • Failed to parse: 52 files
  • Processing time: 6.8 seconds (183 files/sec)

📊 Quality Distribution:
  • 4K/2160p: 156 files (12.5%)
  • 1080p: 789 files (63.3%)
  • 720p: 250 files (20.1%)
  • Other: 52 files (4.2%)

🌍 Language Distribution:
  • English: 980 files (78.6%)
  • Chinese: 167 files (13.4%)
  • Japanese: 98 files (7.9%)
```

### test - Quick File Testing

Test individual files or patterns quickly.

```bash
# Test single file
plex-media-organizer test "Avengers.Endgame.2019.1080p.BluRay.x264.mkv"

# Test multiple files
plex-media-organizer test "Movie1.mkv" "Movie2.mp4" "Movie3.avi"

# Test with pattern
plex-media-organizer test "/movies/*.mkv"

# Test with different configurations
plex-media-organizer test "电影.2023.1080p.mkv" --prefer-original-titles
```

**Example Output:**
```
🎬 Testing: Avengers.Endgame.2019.1080p.BluRay.x264.mkv

✅ Parsing Results:
  • Title: Avengers: Endgame
  • Year: 2019
  • Quality: 1080p
  • Source: BluRay
  • Codec: x264
  • Confidence: 98.5%

🎯 TMDB Match:
  • Title: Avengers: Endgame
  • Release Date: 2019-04-26
  • TMDB ID: 299534
  • Rating: 8.4/10

📁 Organized Name:
  • Filename: Avengers Endgame (2019) 1080p BluRay.mkv
  • Directory: Avengers Endgame (2019)/
  • Full Path: /organized/Movies/Avengers Endgame (2019)/Avengers Endgame (2019) 1080p BluRay.mkv
```

### organize - Rename and Move Files

The `organize` command actually renames and moves your files.

```bash
# Basic organization with dry-run (safe)
plex-media-organizer organize /path/to/movies --dry-run

# Organize to specific output directory
plex-media-organizer organize /path/to/movies --output /path/to/organized

# Organize in-place (renames files in same directory)
plex-media-organizer organize /path/to/movies --in-place

# Create backup before organizing
plex-media-organizer organize /path/to/movies --backup

# Handle duplicates
plex-media-organizer organize /path/to/movies --duplicate-action skip|rename|overwrite
```

**Example Output:**
```
🎬 Organizing: /Users/you/Movies → /Users/you/Organized
📁 Found: 25 movie files

⚡ Processing files...
  [████████████████████] 25/25 files processed

✅ Organization Complete:
  • Successfully organized: 24/25 files (96%)
  • Skipped: 1 file (already organized)
  • Errors: 0 files
  • Time: 2.3 seconds

📊 Summary:
  • Created directories: 24
  • Renamed files: 24
  • Total size: 89.2 GB
  • Backup created: ~/backups/organize-2024-08-17-143022.json

🔄 Rollback available:
  plex-media-organizer rollback ~/backups/organize-2024-08-17-143022.json
```

### config - Manage Configuration

View and manage your configuration settings.

```bash
# View current configuration
plex-media-organizer config

# View specific setting
plex-media-organizer config --get apis.tmdb_api_key

# Set configuration value
plex-media-organizer config --set organization.preferred_quality 1080p

# Validate configuration
plex-media-organizer config --validate

# Show configuration sources
plex-media-organizer config --sources

# Reset to defaults
plex-media-organizer config --reset
```

---

## 🎯 Advanced Usage

### Working with Large Libraries

For libraries with 10,000+ files:

```bash
# Use batch processing
plex-media-organizer scan /huge/library --batch-size 1000

# Limit parallel processing for stability
plex-media-organizer scan /huge/library --max-parallel 8

# Enable progress reporting
plex-media-organizer scan /huge/library --progress

# Process in chunks
for dir in /huge/library/*/; do
    plex-media-organizer organize "$dir" --output /organized/
    sleep 10  # Brief pause between chunks
done
```

### Multi-Language Media Libraries

Handling international content:

```bash
# Enable original title preference
plex-media-organizer organize /movies \
    --prefer-original-titles \
    --include-english-subtitle \
    --output /organized

# Example results:
# "Hero.2002.BluRay.mkv" → "英雄 [Hero] (2002)/英雄 [Hero] (2002) BluRay.mkv"
```

### Quality-Based Organization

Organize by quality tiers:

```bash
# Organize high-quality movies separately
plex-media-organizer organize /movies \
    --quality-filter "4K,2160p,1080p" \
    --output /organized/high-quality

# Organize lower quality separately
plex-media-organizer organize /movies \
    --quality-filter "720p,480p" \
    --output /organized/standard-quality
```

### Custom Naming Templates

Create custom naming schemes:

```bash
# Custom movie template
plex-media-organizer organize /movies \
    --template "{title} ({year}) [{quality}] {source}.{ext}" \
    --output /organized

# Results in: "Avengers Endgame (2019) [1080p] BluRay.mkv"
```

---

## 🔄 Real-World Workflows

### Workflow 1: New Movie Collection

You just downloaded a bunch of new movies:

```bash
# 1. Quick test on a few files
plex-media-organizer test ~/Downloads/new-movies/*.mkv | head -20

# 2. Scan the full directory
plex-media-organizer scan ~/Downloads/new-movies --output new-movies-scan.json

# 3. Review any failures
jq '.failed_files' new-movies-scan.json

# 4. Organize with dry-run
plex-media-organizer organize ~/Downloads/new-movies \
    --output ~/Movies/organized --dry-run

# 5. Execute if satisfied
plex-media-organizer organize ~/Downloads/new-movies \
    --output ~/Movies/organized --backup
```

### Workflow 2: Cleaning Up Existing Library

You have an existing, messy library:

```bash
# 1. Full library scan
plex-media-organizer scan ~/Movies --recursive --verbose > library-analysis.txt

# 2. Identify problem files
grep "Failed to parse" library-analysis.txt

# 3. Organize in chunks by subdirectory
for subdir in ~/Movies/*/; do
    echo "Processing: $subdir"
    plex-media-organizer organize "$subdir" \
        --output ~/Movies/cleaned \
        --duplicate-action rename \
        --backup
done
```

### Workflow 3: Plex Server Integration

Organizing for Plex Media Server:

```bash
# 1. Organize to Plex structure
plex-media-organizer organize ~/Downloads/movies \
    --output "/plex/Movies" \
    --template "{title} ({year})" \
    --create-directories

# 2. Set proper permissions
chmod -R 755 /plex/Movies
chown -R plex:plex /plex/Movies

# 3. Trigger Plex scan
curl -X POST "http://localhost:32400/library/sections/1/refresh?X-Plex-Token=YOUR_TOKEN"
```

### Workflow 4: Backup and Rollback

Safely organize with rollback capability:

```bash
# 1. Create organization with backup
plex-media-organizer organize ~/Movies \
    --output ~/Organized \
    --backup ~/backups/organize-$(date +%Y%m%d-%H%M%S).json

# 2. If something goes wrong, rollback
plex-media-organizer rollback ~/backups/organize-20240817-143022.json

# 3. Verify rollback
plex-media-organizer scan ~/Movies --compare-with ~/backups/organize-20240817-143022.json
```

---

## ⚡ Performance Tuning

### Optimize for Your System

```bash
# Check your system capabilities
nproc  # Number of CPU cores
free -h  # Available memory

# Optimize parallel processing
# For 8-core system with 16GB RAM:
plex-media-organizer scan /movies --max-parallel 16

# For 4-core system with 8GB RAM:
plex-media-organizer scan /movies --max-parallel 8

# For slower systems:
plex-media-organizer scan /movies --max-parallel 4 --batch-size 500
```

### Cache Management

```bash
# Check cache status
plex-media-organizer cache --status

# Clear cache if issues
plex-media-organizer cache --clear

# Preload cache for large libraries
plex-media-organizer cache --preload /movies
```

### Network Optimization

```bash
# For slow internet connections
plex-media-organizer scan /movies \
    --timeout 60 \
    --retries 5 \
    --rate-limit 5

# For fast connections
plex-media-organizer scan /movies \
    --timeout 10 \
    --rate-limit 20
```

---

## 🎯 Best Practices

### File Organization

1. **Always test first**
   ```bash
   # Never organize without testing
   plex-media-organizer organize /movies --dry-run
   ```

2. **Create backups**
   ```bash
   # Always use backup for large operations
   plex-media-organizer organize /movies --backup
   ```

3. **Start small**
   ```bash
   # Test with subset first
   mkdir test-movies
   cp /movies/sample*.mkv test-movies/
   plex-media-organizer organize test-movies/
   ```

### Library Management

1. **Consistent naming**
   ```bash
   # Use consistent quality preferences
   plex-media-organizer config --set organization.preferred_quality 1080p
   ```

2. **Regular scans**
   ```bash
   # Weekly library health check
   plex-media-organizer scan /movies --health-check
   ```

3. **Monitor failures**
   ```bash
   # Keep track of problematic files
   plex-media-organizer scan /movies --export-failures failures.txt
   ```

### Maintenance

1. **Update cache regularly**
   ```bash
   # Monthly cache refresh
   plex-media-organizer cache --refresh
   ```

2. **Backup configurations**
   ```bash
   # Backup your configuration
   cp ~/.config/plex-media-organizer/config.toml ~/backups/
   ```

3. **Monitor disk space**
   ```bash
   # Check space before large operations
   df -h /organized/
   ```

---

## 🔍 Monitoring and Reporting

### Generate Reports

```bash
# Library statistics
plex-media-organizer report --type statistics /movies

# Quality distribution
plex-media-organizer report --type quality /movies

# Language analysis
plex-media-organizer report --type languages /movies

# Duplicates detection
plex-media-organizer report --type duplicates /movies
```

### Health Checks

```bash
# Verify organized library
plex-media-organizer verify /organized/movies

# Check for missing metadata
plex-media-organizer scan /movies --missing-metadata

# Find naming inconsistencies
plex-media-organizer scan /movies --naming-check
```

---

## 🆘 Common Issues and Solutions

### Files Not Being Parsed

```bash
# Debug specific file
plex-media-organizer test "problematic-file.mkv" --debug

# Common solutions:
# 1. File might be too complex - try manual override
plex-media-organizer test "file.mkv" --title "Actual Title" --year 2023

# 2. File might be missing extension
mv "movie-file" "movie-file.mkv"
```

### API Rate Limiting

```bash
# Reduce request rate
plex-media-organizer scan /movies --rate-limit 5

# Use cache more aggressively
plex-media-organizer config --set processing.cache_ttl 7200  # 2 hours
```

### Permission Issues

```bash
# Fix ownership
sudo chown -R $USER:$USER /movies

# Fix permissions
chmod -R 755 /movies
```

---

## 📚 Next Steps

- **[Configuration](configuration.md)** - Fine-tune settings for your needs
- **[Examples](examples/)** - See real-world usage scenarios
- **[Troubleshooting](troubleshooting.md)** - Fix common issues
- **[Development](development/)** - Extend functionality

---

**💡 Pro Tip**: Start with small directories, use dry-run mode extensively, and always create backups before major reorganization operations!
