# 💡 Examples

Real-world usage examples for Plex Media Organizer.

## 📁 Directory Structure

### **[Basic Examples](basic/)**
Simple, common use cases for getting started.

- **[First Movie Organization](basic/first-movie-organization.md)** - Your first successful movie organization
- **[Testing Files](basic/testing-files.md)** - How to test individual files before organizing
- **[Small Library Setup](basic/small-library-setup.md)** - Organizing a small movie collection

### **[Advanced Examples](advanced/)**
Complex scenarios and power-user workflows.

- **[Large Library Organization](advanced/large-library-organization.md)** - Managing 10,000+ movie collections
- **[Multi-Language Collections](advanced/multi-language-collections.md)** - Handling Chinese, Japanese, and mixed content
- **[Quality-Based Organization](advanced/quality-based-organization.md)** - Organizing by quality tiers
- **[Batch Processing](advanced/batch-processing.md)** - Processing multiple directories efficiently

### **[Integrations](integrations/)**
Integration with media servers and tools.

- **[Plex Server Setup](integrations/plex-server-setup.md)** - Complete Plex integration workflow
- **[Jellyfin Integration](integrations/jellyfin-integration.md)** - Setting up with Jellyfin
- **[Automation Scripts](integrations/automation-scripts.md)** - Automated download → organization workflows

## 🎯 Quick Start Examples

### Test a Single File
```bash
# Quick test to see if your setup works
TMDB_API_KEY=your_key plex-media-organizer test "Avengers.Endgame.2019.1080p.BluRay.mkv"

# Expected result:
# ✅ Movie: Avengers: Endgame (2019)
# 📁 Organized: Avengers Endgame (2019) 1080p BluRay.mkv
```

### Scan a Directory
```bash
# Scan without making changes
plex-media-organizer scan ~/Downloads/movies

# See what would be organized
plex-media-organizer organize ~/Downloads/movies --dry-run
```

### Simple Organization
```bash
# Organize with backup
plex-media-organizer organize ~/Downloads/movies \
    --output ~/Movies/organized \
    --backup ~/backups/organize-backup.json
```

## 📊 Example Results

### Typical Success Rates
- **English movies**: 95-98% automatic identification
- **Popular movies**: 98-99% TMDB matches
- **Foreign films**: 85-95% with proper configuration
- **Complex filenames**: 80-90% with fuzzy matching

### Performance Examples
- **Small collection** (< 100 files): < 30 seconds
- **Medium collection** (1,000 files): 2-5 minutes  
- **Large collection** (10,000 files): 15-30 minutes

### Common File Patterns Successfully Handled
```
✅ Avengers.Endgame.2019.1080p.BluRay.x264.mkv
✅ 复仇者联盟4：终局之战.Avengers.Endgame.2019.2160p.UHD.BluRay.x265.HDR.mkv
✅ [YIFY] The Matrix (1999) [1080p] [BluRay] [5.1] [YTS.MX].mp4
✅ Hero.2002.Criterion.Collection.1080p.BluRay.x264-USURY.mkv
✅ 千与千寻.国日双语.千と千尋の神隠し.Spirited.Away.2001.WEB-DL.2160P.H265.mkv
```

## 🔄 Workflow Examples

### New Movie Collection Workflow
1. **Download** → New movies in `~/Downloads/new-movies/`
2. **Test** → `plex-media-organizer scan ~/Downloads/new-movies`
3. **Review** → Check success rate and failed files
4. **Organize** → `plex-media-organizer organize --dry-run` then real organization
5. **Import** → Move to Plex library and refresh

### Library Cleanup Workflow
1. **Analyze** → Full library scan to understand current state
2. **Plan** → Identify problem areas and organization strategy
3. **Backup** → Create full backup of current state
4. **Execute** → Organize in chunks with frequent backups
5. **Verify** → Check results and fix any issues

### Daily Automation Workflow
1. **Watch** → Monitor download directory for new files
2. **Wait** → Let downloads complete
3. **Process** → Automatically organize and move to media server
4. **Notify** → Send success/failure notifications
5. **Cleanup** → Remove original files and update libraries

## 🛠️ Configuration Examples

### Light User Configuration
```toml
[apis]
tmdb_api_key = "your_key"

[organization.quality]
preferred_quality = "1080p"
```

### Power User Configuration  
```toml
[apis]
tmdb_api_key = "your_key"

[organization]
output_directory = "/media/organized"
organize_by_type = true

[organization.quality]
preferred_quality = "1080p"
quality_order = ["4K", "2160p", "1080p", "720p"]

[organization.original_titles]
prefer_original_titles = false
include_english_subtitle = false
fallback_to_english_on_error = true

[processing]
max_parallel_files = 16
enable_cache = true
cache_ttl = 7200
```

### Developer Configuration
```toml
[apis]
tmdb_api_key = "your_key"

[logging]
level = "debug"
log_api_calls = true

[processing]
max_parallel_files = 4
enable_cache = false  # Always fresh data
```

## 📋 Common Scenarios

### Scenario 1: New Plex User
**Goal**: Get movies working in Plex quickly  
**Example**: [Plex Server Setup](integrations/plex-server-setup.md)

### Scenario 2: Multi-Language Collection
**Goal**: Handle Chinese and English movies properly  
**Example**: [Multi-Language Collections](advanced/multi-language-collections.md)

### Scenario 3: Large Existing Library
**Goal**: Clean up years of accumulated, messy files  
**Example**: [Large Library Organization](advanced/large-library-organization.md)

### Scenario 4: Automation Setup
**Goal**: Automatically organize new downloads  
**Example**: [Automation Scripts](integrations/automation-scripts.md)

### Scenario 5: Quality Management
**Goal**: Organize by quality (4K separate from 1080p)  
**Example**: [Quality-Based Organization](advanced/quality-based-organization.md)

## 🆘 Troubleshooting Examples

### Low Success Rate
```bash
# Debug failed files
plex-media-organizer scan /movies --export-failures failures.txt
cat failures.txt

# Common solutions in troubleshooting.md
```

### Performance Issues
```bash
# Optimize for your system
plex-media-organizer scan /movies --max-parallel 8

# Use progress reporting for large libraries
plex-media-organizer scan /movies --progress
```

### Configuration Problems
```bash
# Validate your configuration
plex-media-organizer config --validate

# Check what settings are active
plex-media-organizer config --sources
```

---

**🎯 Next Steps**: Pick an example that matches your use case and follow the detailed guide!

**💡 Pro Tip**: Start with [Basic Examples](basic/) even if you're a power user - they establish good practices for more complex scenarios.
