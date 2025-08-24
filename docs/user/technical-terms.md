# Technical Terms Management

## Overview

Technical terms (like release groups, codecs, quality indicators) are automatically filtered from movie titles during parsing. This ensures clean, Plex-friendly filenames.

## How It Works

### Configuration-Based Approach

Technical terms are managed through the **configuration file** (`config.toml`), not the database. This provides:

- ✅ **Single source of truth**: All terms in one place
- ✅ **Version controlled**: Changes tracked in git
- ✅ **Easy backup/restore**: Just copy the config file
- ✅ **Immediate effect**: Restart app, changes apply
- ✅ **Portable**: Config file moves with the app

### Fallback Protection

Essential terms are hard-coded as a fallback to ensure the parser always works, even without a config file.

## Configuration Structure

```toml
[organization.technical_terms]
# Release group names to filter out
release_groups = [
    "YIFY", "YTS", "RARBG", "3L", "CMCT", "WiKi", "FRDS"
]

# Video/audio codec and quality terms
video_audio_terms = [
    "x264", "x265", "H264", "H265", "DTS", "AC3", "TrueHD", "7.1"
]

# Source/platform names
source_platform_terms = [
    "Netflix", "Amazon", "iTunes", "ATVP"
]

# File format and container terms
file_format_terms = [
    "mkv", "mp4", "avi", "web", "dl", "rip"
]

# Special edition and version terms
special_edition_terms = [
    "Extended", "Director's Cut", "Unrated", "Special Edition"
]

# Additional custom terms
custom_terms = [
    "YourCustomTerm1", "YourCustomTerm2"
]
```

## Managing Technical Terms

### Method 1: Edit Config File Directly

1. Open `config.toml` in your text editor
2. Navigate to `[organization.technical_terms]` section
3. Add/remove terms as needed
4. Save and restart the application

### Method 2: Use CLI Commands (Future)

```bash
# List all technical terms
plex-media-organizer terms --list

# Add a new term
plex-media-organizer terms --add "NewReleaseGroup"

# Remove a term
plex-media-organizer terms --remove "OldTerm"

# Show terms by category
plex-media-organizer terms --categories

# Export terms to file
plex-media-organizer terms --export terms.txt

# Import terms from file
plex-media-organizer terms --import terms.txt
```

## Term Categories

### Release Groups
Names of groups that release media files (e.g., "YIFY", "3L", "CMCT")

### Video/Audio Terms
Codecs, audio formats, and quality indicators:
- **Codecs**: x264, x265, H264, H265, AVC, HEVC
- **Audio**: DTS, AC3, AAC, FLAC, TrueHD, 7.1, 5.1
- **Quality**: 10bit, 8bit, HDR, DoVi

### Source/Platform Terms
Streaming services and platforms:
- Netflix, Amazon, iTunes, ATVP, Hulu, Disney+

### File Format Terms
Container formats and technical indicators:
- mkv, mp4, avi, web, dl, rip, remux

### Special Edition Terms
Version and edition indicators:
- Extended, Director's Cut, Unrated, Special Edition

### Custom Terms
User-defined terms specific to your media collection

## Best Practices

### Adding New Terms

1. **Identify the pattern**: Look at filenames that aren't being cleaned properly
2. **Add to appropriate category**: Choose the most specific category
3. **Test thoroughly**: Use the `test` command to verify filtering works
4. **Use case-insensitive matching**: Terms are matched regardless of case

### Removing Terms

1. **Be cautious**: Only remove terms you're certain should be preserved
2. **Test first**: Use the `test` command to see the impact
3. **Consider context**: Some terms might be legitimate movie titles

### Common Issues

**Problem**: "Movie" is being filtered from legitimate titles
**Solution**: "Movie" is not in the default terms list. If this happens, check for similar terms.

**Problem**: New release group not being filtered
**Solution**: Add the release group name to the `release_groups` list in config.

**Problem**: Audio format appearing in title
**Solution**: Add the audio format to the `video_audio_terms` list.

## Examples

### Before Filtering
```
The.Batman.2022.2160p.Remux.HEVC.DoVi.TrueHD.7.1-3L.mkv
```

### After Filtering
```
The Batman (2022) [2160p] [REMUX].mkv
```

### Configuration Entry
```toml
[organization.technical_terms]
release_groups = [
    "3L",  # This ensures "3L" is filtered
    # ... other release groups
]
video_audio_terms = [
    "TrueHD", "7.1", "DoVi",  # These ensure audio/video terms are filtered
    # ... other terms
]
```

## Troubleshooting

### Terms Not Being Filtered

1. **Check case sensitivity**: Terms are matched case-insensitively
2. **Verify word boundaries**: Terms must be complete words
3. **Check config file**: Ensure terms are in the correct category
4. **Restart application**: Changes require restart to take effect

### Too Many Terms Being Filtered

1. **Review custom terms**: Check if you added terms that are legitimate titles
2. **Use test command**: Test specific filenames to see what's being filtered
3. **Remove problematic terms**: Remove terms that are causing false positives

### Configuration Not Loading

1. **Check file path**: Ensure config.toml is in the correct location
2. **Verify syntax**: Check for TOML syntax errors
3. **Use setup command**: Run `plex-media-organizer setup` to recreate config
