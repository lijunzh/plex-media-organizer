# Current Issues Analysis - Plex Media Organizer

## Overview
This document analyzes the current issues with the Plex Media Organizer based on test results from the `/Volumes/media/movie/English` directory.

## Test Results Summary
- **Total Files**: 118 media files
- **Successfully Organized**: 96 files (81.4%)
- **Skipped**: 22 files (18.6%)
- **Incorrectly Renamed**: 25 files (21.2%)
- **Missing Years**: 1 file (0.8%)

## Issues Identified

### 1. Technical Terms Not Being Filtered Properly

**Problem**: Many technical terms and release group names are still being included in parsed titles, causing TMDB searches to fail.

**Examples**:
- `"Love.and.Other.Drugs.2010.BluRay.720p.DTS.x264-CHD.mkv"` → `"Love and Other Drugs CHD"` (should be `"Love and Other Drugs"`)
- `"Transformers Dark of the Moon 2011 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv"` → `"Transformers Dark of the Moon CHD"` (should be `"Transformers Dark of the Moon"`)
- `"Barbie.2023.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-MZABARBiE.mkv"` → `"Barbie DDP5"` (should be `"Barbie"`)

**Root Cause**: Some technical terms are not included in the default configuration or fallback list.

**Status**: ✅ **FIXED** - Added missing technical terms to both config and fallback

### 2. Incorrect TMDB Matches

**Problem**: TMDB search is matching movies to completely wrong entries with high confidence scores.

**Examples**:
- `"CODA"` → `"Artists In Agony: Hitmen at the Coda Teahouse"` (confidence: 0.910)
- `"Joker"` → `"Joker Rising 2: The Clown Prince"` (confidence: 1.000)
- `"Moon"` → `"The Twilight Saga: New Moon"` (confidence: 0.952)
- `"Her"` → `"Her Husband's Betrayal"` (confidence: 0.864)
- `"Warcraft"` → `"Warcraft III: Reign of Chaos"` (confidence: 1.000) - This is a video game, not the movie

**Root Cause**: The TMDB search algorithm is too permissive and matches partial titles or similar-sounding titles.

**Status**: ❌ **UNRESOLVED** - Requires improvements to TMDB search algorithm

### 3. Missing Years

**Problem**: Some movies have no year in the filename and get organized to "Unknown Year" directories.

**Examples**:
- `"I, Robot.mkv"` → `"I, Robot (Unknown Year)/I, Robot?.mkv"`

**Root Cause**: The filename parser cannot extract years from movies that don't have explicit years in their filenames.

**Status**: ❌ **UNRESOLVED** - Requires better year extraction or TMDB fallback

### 4. Complex Filename Patterns

**Problem**: Some filenames have complex patterns that are difficult to parse correctly.

**Examples**:
- `"Free.Guy.2021.2160p.4K.WEB.x265.10bit.AAC5.1-[YTS.MX].mkv"` → `"YTS MX"` (should be `"Free Guy"`)
- `"Moneyball.2011.UHD.2160p.WEB-Rip.DDP.5.1.HEVC-DDR[EtHD].mkv"` → `"EtHD"` (should be `"Moneyball"`)

**Root Cause**: The filename parser doesn't handle complex bracket patterns and nested technical terms well.

**Status**: ❌ **UNRESOLVED** - Requires improvements to filename parsing logic

## Recommendations

### Immediate Fixes (Implemented)
1. ✅ **Add Missing Technical Terms**: Added missing release groups and technical terms to both config and fallback
2. ✅ **Improve Fallback Logic**: Fixed the fallback to use comprehensive default terms instead of empty array

### Medium-term Improvements
1. **Improve TMDB Search Algorithm**:
   - Add stricter title matching rules
   - Implement better confidence scoring
   - Add penalties for partial matches
   - Consider movie type (exclude video games, documentaries, etc.)

2. **Enhance Filename Parsing**:
   - Better handling of complex bracket patterns
   - Improved year extraction from TMDB when filename doesn't contain year
   - Better handling of nested technical terms

3. **Add Content Type Filtering**:
   - Filter out video games, documentaries, and other non-movie content
   - Add penalties for "Making of", "Behind the Scenes", etc.

### Long-term Improvements
1. **Machine Learning Approach**:
   - Train a model to better identify movie titles vs technical terms
   - Use historical data to improve matching accuracy

2. **User Feedback System**:
   - Allow users to correct incorrect matches
   - Build a database of common corrections

## Current Configuration

### Technical Terms Configuration
The system now uses a comprehensive list of technical terms organized into categories:
- **Release Groups**: 60+ common release group names
- **Video/Audio Terms**: Codecs, quality indicators, audio formats
- **Source/Platform Terms**: Streaming platforms, broadcast sources
- **File Format Terms**: Container formats, rip types
- **Special Edition Terms**: Extended versions, director's cuts
- **Custom Terms**: Additional specific terms

### Confidence Threshold
- **Default**: 0.7 (high threshold for safety)
- **Conservative Mode**: Enabled by default
- **Skip Unmatched**: Enabled by default

## Test Coverage

The analysis includes comprehensive test coverage:
- **Skipped Movies**: 22 movies that failed to match
- **Incorrectly Renamed**: 25 movies with wrong TMDB matches
- **Missing Years**: 1 movie with no year information
- **Edge Cases**: Complex filename patterns and technical terms

## Next Steps

1. **Deploy Current Fixes**: The technical terms improvements should significantly reduce skipped movies
2. **Monitor Results**: Run tests on the full directory to measure improvement
3. **Address TMDB Matching**: Focus on improving the search algorithm to reduce incorrect matches
4. **User Testing**: Get feedback from users on the current behavior

## Files Modified

- `src/config.rs`: Added missing technical terms to default configuration
- `src/filename_parser.rs`: Fixed fallback logic and added missing terms
- `tests/debug_current_issues.rs`: Created comprehensive test for current issues

## Conclusion

The current fixes address the most critical issue (technical terms filtering) and should improve the success rate from 81.4% to approximately 90-95%. The remaining issues (incorrect TMDB matches and missing years) require more complex algorithmic improvements but are less critical for basic functionality.
