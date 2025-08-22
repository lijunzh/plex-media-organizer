# Current Limitations and Trade-offs

## Overview

The Plex Media Organizer uses a **conservative approach** to ensure accuracy over completeness. This means some movies may be skipped rather than risk incorrect organization.

## Current Approach

### 1. TMDB-First Strategy
- **Trust TMDB's data completely** - we use TMDB's English title and original title as authoritative
- **High confidence threshold (0.7)** - requires strong evidence before organizing
- **Skip unmatched movies** - rather than use potentially incorrect fallback data

### 2. Title Extraction Limitations
- **Basic technical term filtering** - some technical terms may remain in extracted titles
- **No machine learning** - relies on hard-coded patterns
- **Limited multilingual support** - primarily optimized for Chinese, Japanese, and English

## Why Movies Get Skipped

### 1. "No TMDB match found"
**Cause**: TMDB cannot find a movie with the extracted title and year.

**Common reasons**:
- Technical terms in filename not properly filtered (e.g., "Fight Back to School DualAudio iNT TLF")
- Movie doesn't exist in TMDB database
- Movie has very different title in TMDB vs. filename
- Year mismatch between filename and TMDB data

**Example**:
```
Filename: Fight.Back.to.School.3.1993.BDRip.X264.DualAudio.iNT-TLF.mkv
Extracted title: "Fight Back to School DualAudio iNT TLF"
Issue: Technical terms "DualAudio", "iNT", "TLF" not filtered out
Result: TMDB can't find a movie with this exact title
```

### 2. "Low confidence (X < 0.7)"
**Cause**: TMDB found a match but confidence score is below threshold.

**Common reasons**:
- Partial title match (e.g., "Lost in the Stars" vs "消失的她")
- Year mismatch
- Different title variations

**Example**:
```
Filename: 消失的她.Lost.in.the.Stars.2022.1080p.WEB-DL.mkv
TMDB match: "Lost in the Stars" (2023)
Confidence: 0.69 (below 0.7 threshold)
Issue: Year mismatch (2022 vs 2023) and partial title match
```

## Trade-offs

### Conservative Approach (Current)
**Pros**:
- ✅ **High accuracy** - very low risk of incorrect organization
- ✅ **Clean results** - organized movies are correctly identified
- ✅ **TMDB authoritative** - uses official movie database
- ✅ **Safe for automation** - won't create incorrect directory structures

**Cons**:
- ❌ **Lower coverage** - some movies will be skipped
- ❌ **Manual intervention needed** - skipped movies require manual review
- ❌ **Technical term sensitivity** - filenames with unusual patterns may fail

### Alternative: Aggressive Approach (Not Recommended)
**Pros**:
- ✅ **Higher coverage** - more movies would be organized
- ✅ **Less manual work** - fewer skipped movies

**Cons**:
- ❌ **Risk of errors** - could create incorrect directory structures
- ❌ **Plex indexing issues** - wrong titles could affect search and metadata
- ❌ **Data integrity** - organized files might not match actual movie content

## Recommendations

### For Users

1. **Accept the trade-off**: It's better to skip a movie than organize it incorrectly
2. **Review skipped movies**: Check the skipped list for movies you want to organize manually
3. **Use preview mode**: Always run with `--preview` first to see what will be organized
4. **Safety requirement**: Lower confidence thresholds (below 0.7) **require** `--preview` mode for safety
5. **Adjust confidence threshold**: Use `--min-confidence 0.5` for more permissive matching (but review results carefully)

### For Filename Issues

1. **Clean filenames**: Remove technical terms manually before organizing
2. **Use standard naming**: Follow common movie naming conventions
3. **Check TMDB**: Verify the movie exists in TMDB with the expected title

### For Developers

1. **Iteration 2**: Build a technical terms database for better filtering
2. **Machine learning**: Implement pattern learning for new technical terms
3. **Alternative search strategies**: Try multiple search approaches when TMDB fails
4. **User feedback**: Allow users to correct and improve title extraction

## Configuration Options

### Confidence Threshold
```bash
# Conservative (default) - high accuracy, lower coverage
--min-confidence 0.7

# Moderate - balanced approach (requires --preview)
--min-confidence 0.6 --preview

# Permissive - higher coverage, review carefully (requires --preview)
--min-confidence 0.5 --preview
```

**Safety Note**: Thresholds below 0.7 require `--preview` mode to prevent accidental incorrect organization.

### Skip Unmatched Movies
```bash
# Skip movies with no TMDB match (default)
--skip-unmatched true

# Use fallback data (not recommended)
--skip-unmatched false
```

## Future Improvements (Iteration 2)

1. **Technical Terms Database**
   - JSON/TOML configuration file
   - User-extensible patterns
   - Categorized by type (codec, quality, release group, etc.)

2. **Improved Title Extraction**
   - Machine learning for pattern recognition
   - Fuzzy matching for similar terms
   - Multiple extraction strategies

3. **Enhanced TMDB Integration**
   - Alternative search strategies
   - Year range matching
   - Partial title matching with confidence scoring

4. **User Feedback System**
   - Allow users to correct extraction errors
   - Learn from user corrections
   - Community-driven pattern updates

## Conclusion

The current conservative approach prioritizes **accuracy over completeness**. While some movies will be skipped, this ensures that organized movies are correctly identified and properly structured for Plex.

For users who need higher coverage, consider:
1. Manually cleaning problematic filenames
2. Using a lower confidence threshold (with careful review)
3. Waiting for Iteration 2 improvements

**Remember**: It's always better to skip a movie than to organize it incorrectly!
