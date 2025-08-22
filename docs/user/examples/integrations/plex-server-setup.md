# 🎬 Complete Plex Server Setup

End-to-end guide for organizing movies and setting up Plex Media Server.

## 🎯 Goal

Transform your movie collection into a perfectly organized Plex library with rich metadata, thumbnails, and a beautiful interface.

## 📋 Prerequisites

- ✅ Plex Media Organizer installed
- ✅ Plex Media Server installed ([download here](https://www.plex.tv/media-server-downloads/))
- ✅ TMDB API key
- ✅ Movie collection to organize

## 🚀 Complete Workflow

### Step 1: Plan Your Library Structure

Decide on your final organization structure:

```
/media/
├── Movies/
│   ├── Avengers Endgame (2019)/
│   │   └── Avengers Endgame (2019) 1080p BluRay.mkv
│   ├── The Matrix (1999)/
│   │   └── The Matrix (1999) 1080p BluRay.mkv
│   └── ...
└── TV Shows/ (for future)
```

### Step 2: Organize Your Movies

```bash
# Create organized library
mkdir -p /media/Movies

# Organize with Plex-optimized settings
plex-media-organizer organize ~/Downloads/movies/ \
    --output /media/Movies \
    --template "{title} ({year})" \
    --create-directories \
    --backup ~/backups/plex-organization.json \
    --dry-run

# If preview looks good, execute:
plex-media-organizer organize ~/Downloads/movies/ \
    --output /media/Movies \
    --template "{title} ({year})" \
    --create-directories \
    --backup ~/backups/plex-organization.json
```

### Step 3: Set Proper Permissions

```bash
# Make files accessible to Plex
sudo chown -R plex:plex /media/Movies
sudo chmod -R 755 /media/Movies

# Or if running Plex as your user:
chown -R $USER:$USER /media/Movies
chmod -R 755 /media/Movies
```

### Step 4: Add Library to Plex

1. **Open Plex Web Interface**: http://localhost:32400/web
2. **Go to Settings** → **Libraries**
3. **Add Library** → **Movies**
4. **Add Folder**: `/media/Movies`
5. **Advanced Settings**:
   - Scanner: **Plex Movie Scanner**
   - Agent: **Plex Movie** (new agent) or **The Movie Database**
   - Enable: **Use local assets**, **Find trailers and extras automatically**

### Step 5: Configure Movie Agent

**For optimal metadata:**

1. **Go to Settings** → **Agents** → **Movies** → **Plex Movie**
2. **Enable and order**:
   - ✅ The Movie Database (primary)
   - ✅ Local Media Assets (secondary)
   - ✅ Embedded metadata
3. **Set language**: English (or your preference)
4. **Enable**: Use plot summaries from, Certification country

### Step 6: Trigger Library Scan

```bash
# Force immediate scan
curl -X POST "http://localhost:32400/library/sections/1/refresh?X-Plex-Token=YOUR_TOKEN"

# Or in Plex Web UI: Libraries → ... → Scan Library Files
```

## 🎯 Optimization Tips

### For Best Plex Experience

```bash
# Organize with year in directory name (Plex preferred)
plex-media-organizer organize ~/movies \
    --output /media/Movies \
    --directory-template "{title} ({year})" \
    --filename-template "{title} ({year}) {quality} {source}.{ext}"
```

### Handle Multiple Versions

```bash
# If you have multiple versions of same movie
# Plex will group them automatically if in same directory:

/media/Movies/The Matrix (1999)/
├── The Matrix (1999) 1080p BluRay.mkv
├── The Matrix (1999) 4K UHD HDR.mkv
└── The Matrix (1999) Directors Commentary.mkv
```

### Optimize for Performance

```bash
# For large libraries, use parallel processing
plex-media-organizer organize ~/movies \
    --output /media/Movies \
    --max-parallel 8 \
    --progress
```

## 📊 Expected Results

### Organization Results
```
✅ Successfully organized: 847/864 files (98.0%)
🎯 TMDB matches: 832/847 (98.2%)
📁 Created directories: 847
⏱️  Total time: 12.3 minutes
💾 Backup: ~/backups/plex-organization.json
```

### Plex Library After Scan
- **Rich metadata**: Plot summaries, cast, ratings, genres
- **High-quality posters**: Automatically downloaded
- **Trailers**: Available for most movies
- **Collections**: Automatically grouped (Marvel, DC, etc.)
- **Similar movies**: Recommendations based on viewing

## 🔧 Advanced Configuration

### Custom Naming for Plex

Create a Plex-optimized configuration:

```toml
# ~/.config/plex-media-organizer/plex-config.toml
[organization]
output_directory = "/media/Movies"
organize_by_type = true

[organization.templates]
movie_directory = "{title} ({year})"
movie_filename = "{title} ({year}) {quality} {source}.{ext}"

[organization.quality]
preferred_quality = "1080p"
include_quality_in_filename = true
include_source_in_filename = true

[apis]
tmdb_api_key = "your_key_here"
```

Use with:
```bash
plex-media-organizer --config plex-config.toml organize ~/movies
```

### Multiple Libraries

For different movie types:

```bash
# Main movies
plex-media-organizer organize ~/movies/english \
    --output /media/Movies/English

# Foreign films
plex-media-organizer organize ~/movies/foreign \
    --output /media/Movies/Foreign \
    --prefer-original-titles

# 4K collection
plex-media-organizer organize ~/movies/4k \
    --output /media/Movies/4K \
    --quality-filter "4K,2160p,UHD"
```

## 🎨 Plex Optimization

### Library Settings

**Recommended Plex settings for organized library:**

1. **Scanner Settings**:
   - Scanner: **Plex Movie Scanner**
   - Enable: **Scan my library automatically**
   - Enable: **Run a partial scan when changes are detected**

2. **Agent Settings**:
   - Primary: **The Movie Database**
   - Enable: **Prefer local metadata**
   - Language: **English** (or preferred)

3. **Advanced Settings**:
   - Enable: **Use local assets**
   - Enable: **Find trailers and extras automatically**
   - Enable: **Find behind the scenes and interviews automatically**

### Custom Poster and Metadata

Add custom assets:

```
The Matrix (1999)/
├── The Matrix (1999) 1080p BluRay.mkv
├── poster.jpg                     # Custom poster
├── background.jpg                 # Custom background
└── The Matrix (1999)-trailer.mkv  # Local trailer
```

## 🚀 Automation

### Automatic Organization

Create a script for continuous organization:

```bash
#!/bin/bash
# organize-for-plex.sh

DOWNLOAD_DIR="$HOME/Downloads/movies"
PLEX_DIR="/media/Movies"
BACKUP_DIR="$HOME/backups"

# Only run if new files exist
if [ "$(ls -A $DOWNLOAD_DIR)" ]; then
    echo "New movies found, organizing..."
    
    # Organize new downloads
    plex-media-organizer organize "$DOWNLOAD_DIR" \
        --output "$PLEX_DIR" \
        --backup "$BACKUP_DIR/auto-$(date +%Y%m%d-%H%M%S).json"
    
    # Update Plex library
    curl -X POST "http://localhost:32400/library/sections/1/refresh?X-Plex-Token=$PLEX_TOKEN"
    
    echo "Organization complete, Plex scanning..."
else
    echo "No new movies to organize"
fi
```

Run with cron:
```bash
# Add to crontab (run every hour)
0 * * * * /home/user/scripts/organize-for-plex.sh
```

## 📱 Mobile and Remote Access

### Enable Remote Access

1. **Plex Settings** → **Remote Access**
2. **Enable**: Manually specify public port
3. **Configure router**: Port forward 32400
4. **Test**: Verify external access works

### Optimize for Mobile

```bash
# Organize with mobile-friendly names (no special characters)
plex-media-organizer organize ~/movies \
    --output /media/Movies \
    --clean-filenames \
    --ascii-only
```

## 🔍 Troubleshooting

### Movies Not Appearing in Plex

```bash
# Check permissions
ls -la /media/Movies/
# Should show: drwxr-xr-x plex plex

# Force library refresh
curl -X POST "http://localhost:32400/library/sections/1/refresh?force=1&X-Plex-Token=$PLEX_TOKEN"

# Check Plex logs
tail -f "/var/lib/plexmediaserver/Library/Application Support/Plex Media Server/Logs/Plex Media Server.log"
```

### Metadata Issues

```bash
# Re-scan with fresh metadata
curl -X POST "http://localhost:32400/library/sections/1/refresh?force=1&X-Plex-Token=$PLEX_TOKEN"

# Fix unmatched movies manually in Plex:
# Library → Unmatched → Fix Match
```

### Performance Issues

```bash
# For very large libraries
plex-media-organizer organize ~/movies \
    --output /media/Movies \
    --batch-size 500 \
    --max-parallel 4
```

## 🎉 Final Result

After following this guide, you'll have:

- ✅ **Perfectly organized movie library** with Plex naming conventions
- ✅ **Rich metadata** for all movies (plots, posters, cast)
- ✅ **Beautiful Plex interface** with trailers and extras
- ✅ **Mobile access** to your library anywhere
- ✅ **Automated workflow** for new downloads
- ✅ **Backup system** for safe organization

### Example Final Library

Your Plex library will show:
- **Movie posters** in grid view
- **Plot summaries** and cast information
- **Related movies** and recommendations
- **Collections** (Marvel Movies, Star Wars, etc.)
- **Trailers** and behind-the-scenes content
- **Multiple versions** grouped together

---

**⏱️ Total setup time**: 1-2 hours for complete setup  
**🎯 Result**: Professional media server with perfectly organized content

**💡 Pro Tip**: Start with a small collection to test the workflow, then scale up to your full library!
