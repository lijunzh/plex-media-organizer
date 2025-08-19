# TMDB Matching Challenges and Solutions

## Overview

The TMDB (The Movie Database) API integration faces several challenges when matching movie titles from filenames to actual movies in the database. This document outlines the challenges encountered and the general solutions implemented.

## Key Challenges

### 1. Extras Content Misidentification

**Problem**: TMDB contains many "extras" content entries that include the original movie title, leading to incorrect matches.

**Examples**:
- "Only Yesterday" → "The Making of Only Yesterday"
- "The Cat Returns" → "The Cat Returns - Making of"
- "Ghost in the Shell" → "Ghost in the Shell: Production Report"
- "Princess Mononoke" → "The Birth of Princess Mononoke Part 3"

**Root Cause**: Extras content often contains the original movie title in its name, making fuzzy matching algorithms score them highly.

### 2. Generic Title Matching

**Problem**: Short or generic titles can match multiple unrelated movies.

**Examples**:
- "The " (from Slam Dunk files) → "The Fantastic 4: First Steps"
- "Suzume" → "Suzume Highlight Film Concert"

**Root Cause**: TMDB's fuzzy matching algorithm can find partial matches that are semantically unrelated.

### 3. Multilingual Title Complexity

**Problem**: Japanese/Chinese movies often have complex filenames with multiple language titles and technical metadata.

**Examples**:
- `[晨曦&老M制作][剧场版][灌篮高手][BDrip][HEVC Main10P FLAC MKV].mkv`
- `千与千寻.国日双语.千と千尋の神隠し.Spirited.Away.2001.WEB-DL.2160P.H265.10bits.HDR.2xDDP 5.1-PTHweb.mkv`

**Root Cause**: Filename parsing needs to distinguish between actual movie titles and technical metadata.

## General Solutions Implemented

### 1. Content Type Penalties

**Approach**: Apply heavy penalties to content types that are typically extras rather than main movies.

**Implementation**:
```rust
let problematic_patterns = [
    "production report",
    "making of",
    "behind the scenes",
    "highlight film concert",
    "documentary",
    "special feature",
    "bonus content",
    "extras",
    "commentary",
    "interview",
];

for pattern in &problematic_patterns {
    if title_lower.contains(pattern) {
        score -= 1000.0; // Very heavy penalty
        break;
    }
}
```

**Benefits**:
- Reduces false positives for extras content
- Applies broadly to any similar content
- Doesn't require maintaining specific movie title lists

### 2. Enhanced Confidence Scoring

**Approach**: Use TMDB's sophisticated scoring system directly instead of simple thresholds.

**Implementation**:
- Convert TMDB's raw scores (50-400 range) to confidence scores (0.3-1.0 range)
- Normalize scores to provide meaningful confidence levels
- Use confidence scores in filtering decisions

**Benefits**:
- More nuanced matching quality assessment
- Leverages TMDB's existing sophisticated algorithms
- Provides better filtering decisions

### 3. Improved Title Extraction

**Approach**: Better distinguish between actual movie titles and technical metadata in filenames.

**Implementation**:
- Enhanced tokenization for bracketed content
- Improved detection of Chinese/Japanese/English titles
- Special handling for known movie titles that might be filtered out
- Metadata token filtering to exclude technical terms

**Benefits**:
- More accurate title extraction from complex filenames
- Better handling of multilingual content
- Reduces false positives from technical metadata

### 4. Configurable Confidence Thresholds

**Approach**: Allow users to control matching strictness through configuration.

**Implementation**:
- `min_confidence_threshold`: Minimum confidence required (default: 0.7)
- `skip_unmatched_movies`: Skip files without TMDB matches (default: true)
- `warn_on_low_confidence`: Show warnings for low-confidence matches

**Benefits**:
- Users can adjust strictness based on their needs
- Prevents incorrect organization of low-confidence matches
- Provides transparency about matching quality

## Lessons Learned

### 1. Avoid Specific Title Penalties

**Problem**: Initially implemented specific penalties for individual movie titles like "The Fantastic 4", "The Pickup", etc.

**Issue**: This approach is brittle and doesn't scale. New problematic matches would require code changes.

**Solution**: Focus on general content type penalties that apply broadly.

### 2. Confidence-Based Filtering is Essential

**Problem**: Simple yes/no matching decisions led to incorrect organization of low-quality matches.

**Solution**: Use confidence scores to make informed decisions about whether to organize or skip files.

### 3. Title Extraction is Critical

**Problem**: Poor title extraction led to generic titles that matched wrong movies.

**Solution**: Invest in robust title extraction that properly handles multilingual content and technical metadata.

## Future Improvements

### 1. Machine Learning Approach

Consider training a model to better distinguish between main movies and extras content based on TMDB metadata patterns.

### 2. User Feedback Integration

Allow users to provide feedback on incorrect matches to improve the system over time.

### 3. Alternative Data Sources

Consider integrating additional movie databases (IMDB, OMDb) for better coverage and validation.

### 4. Semantic Analysis

Implement more sophisticated semantic analysis to understand movie title similarity beyond simple string matching.

## Configuration Recommendations

### For Conservative Users (High Quality)
```toml
min_confidence_threshold = 0.8
skip_unmatched_movies = true
warn_on_low_confidence = true
```

### For Aggressive Users (High Coverage)
```toml
min_confidence_threshold = 0.5
skip_unmatched_movies = false
warn_on_low_confidence = true
```

## Testing Strategy

### 1. Real-World Test Cases
- Test with actual user media collections
- Focus on edge cases and problematic filenames
- Validate that correct movies are matched and incorrect ones are skipped

### 2. Confidence Score Validation
- Verify that confidence scores correlate with match quality
- Test threshold settings with various media collections
- Ensure skipped files are actually problematic

### 3. Performance Testing
- Monitor API call frequency and response times
- Test with large media collections
- Ensure caching is working effectively
