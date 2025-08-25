# Filename Parsing Strategy Guide

## What's New

The plex-media-organizer now uses a **TMDB-first approach** for parsing movie filenames. This ensures more accurate title extraction and better language-aware formatting.

## How It Works

### Step 1: Clean Title Extraction
The parser first removes all technical terms from the filename to get a clean title:

**Before**: `The.Batman.2022.2160p.Remux.HEVC.DoVi.TrueHD.7.1-3L.mkv`
**After**: `The Batman`

### Step 2: TMDB Lookup
The clean title + year is used to search TMDB for the official movie record.

### Step 3: Language-Aware Formatting
Based on TMDB's original language data, the title is formatted appropriately:

#### English Movies
```
Input:  The.Batman.2022.2160p.Remux.HEVC.DoVi.TrueHD.7.1-3L.mkv
Output: The Batman (2022) [2160p]
```

#### Non-English Movies
```
Input:  武状元苏乞儿.King.of.Beggars.1992.2160p.WEB-DL.H264.AAC.2Audio-OurTV.mp4
Output: 武状元苏乞儿 (1992) [King of Beggars] [2160p]
```

## Benefits

✅ **Accurate Language Detection**: Uses TMDB's authoritative data  
✅ **Better Organization**: Consistent formatting for Plex  
✅ **Clean Titles**: Removes all technical clutter  
✅ **Flexible Output**: Configurable quality display  

## Configuration

### Include Quality in Title
```toml
[organization]
# Set to true to include quality in the final title
include_quality_in_title = false
```

### TMDB Integration
```toml
[apis]
# Your TMDB API key (optional but recommended)
tmdb_api_key = "your_api_key_here"

[organization.matching]
# Minimum confidence for TMDB enhancement
min_tmdb_confidence = 0.3
```

## Examples

| Input Filename | Output Title | Language |
|---|---|---|
| `Iron.Man.2008.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv` | `Iron Man (2008) [2160p]` | English |
| `钢铁侠.Iron.Man.2008.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv` | `Iron Man (2008) [2160p]` | English (TMDB confirmed) |
| `武状元苏乞儿.King.of.Beggars.1992.2160p.WEB-DL.H264.AAC.2Audio-OurTV.mp4` | `武状元苏乞儿 (1992) [King of Beggars] [2160p]` | Chinese (TMDB confirmed) |
| `アニメ.Movie.2023.1080p.BluRay.x264.mkv` | `アニメ Movie (2023) [1080p]` | Japanese (no English found) |

## Technical Terms Filtered

The parser automatically removes these technical terms:

- **Years**: 2023, 1999, etc.
- **Quality**: 1080p, 2160p, 4K, UHD, etc.
- **Release Groups**: YTS, FRDS, 3L, etc.
- **Codecs**: x264, x265, H264, H265, etc.
- **Audio**: DTS, AC3, AAC, TrueHD, etc.
- **Sources**: BluRay, WEB-DL, HDTV, etc.

## Fallback Behavior

If TMDB lookup fails or confidence is low:
- Uses the clean title as-is
- Maintains the year and quality information
- Preserves the original language detection logic

## Migration from Old System

The old system created bilingual formats like `"钢铁侠 - Iron Man"`. The new system:
- Uses TMDB to determine the correct language
- Formats English movies as English titles only
- Formats non-English movies with original title + English in brackets

This provides more accurate and consistent results for your media library organization.
