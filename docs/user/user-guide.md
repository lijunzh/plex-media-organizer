# User Guide

## Overview

The Plex Media Organizer uses a **conservative approach** that prioritizes **accuracy over completeness**. This guide explains how to use the tool effectively and understand its behavior.

## Current Approach & Trade-offs

### Conservative Strategy
The organizer uses a **conservative approach** that prioritizes **accuracy over completeness**:

- **High confidence threshold (0.7)**: Requires strong evidence before organizing
- **TMDB-first**: Trusts TMDB's data completely for movie information
- **Skip unmatched**: Skips movies rather than risk incorrect organization

### Why Some Movies Are Skipped

Movies may be skipped for these reasons:

1. **"No TMDB match found"**: 
   - Technical terms in filename not filtered (e.g., "DualAudio", "iNT", "TLF")
   - Movie doesn't exist in TMDB database
   - Year/title mismatch between filename and TMDB

2. **"Low confidence"**: 
   - TMDB found a match but confidence is below threshold
   - Partial title matches or year mismatches

### Trade-offs

**Conservative Approach (Default)**
- ✅ **High accuracy** - very low risk of incorrect organization
- ✅ **Clean results** - organized movies are correctly identified
- ❌ **Lower coverage** - some movies will be skipped

**Permissive Approach** (use `--min-confidence 0.5 --preview`)
- ✅ **Higher coverage** - more movies organized
- ❌ **Risk of errors** - may create incorrect directory structures
- ⚠️ **Safety requirement** - preview mode required for lower thresholds

## Configuration

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

## Examples

### Chinese Movies
```
Input:  金手指.The.Goldfinger.2023.2160p.WEB-DL.mp4
Output: 金手指 [The Goldfinger] (2023)/金手指 [The Goldfinger] (2023) 2160p WEB-DL.mp4
```

### Japanese Movies
```
Input:  千と千尋の神隠し.Spirited.Away.2001.WEB-DL.mkv
Output: 千と千尋の神隠し [Spirited Away] (2001)/千と千尋の神隠し [Spirited Away] (2001) WEB-DL.mkv
```

### English Movies
```
Input:  The.Matrix.1999.1080p.BluRay.mkv
Output: The Matrix (1999)/The Matrix (1999) 1080p BluRay.mkv
```

## Troubleshooting

### Common Issues

1. **Movies being skipped**: This is expected behavior. Check the skipped list and review manually.
2. **Technical terms in titles**: Some technical terms may not be filtered. This will be improved in Iteration 2.
3. **Year mismatches**: TMDB may have different year than filename. Use `--min-confidence 0.5` for more permissive matching.

### Getting Help

1. **Review skipped movies**: Always check the list of skipped movies
2. **Use preview mode**: Always run with `--preview` first
3. **Check TMDB**: Verify the movie exists in TMDB with the expected title
4. **Clean filenames**: Remove technical terms manually if needed

## Limitations

### Current Limitations
- **Basic technical term filtering**: Some technical terms may remain in extracted titles
- **Limited search strategies**: Only exact title + year matching with TMDB
- **No learning system**: Doesn't improve based on user feedback

### Future Improvements (Iteration 2)
- **Technical terms database**: Configurable, extensible pattern database
- **Enhanced TMDB search**: Multiple search strategies and year range matching
- **User feedback system**: Learn from user corrections
- **Machine learning**: Pattern recognition for better title extraction

## Best Practices

### Before Organizing
1. **Always use preview mode first**: `--preview --verbose`
2. **Review the output carefully**: Check both organized and skipped movies
3. **Backup important data**: Use `--backup` option for safety

### When Movies Are Skipped
1. **Don't panic**: This is expected behavior
2. **Review the skipped list**: Check why each movie was skipped
3. **Manual verification**: Verify movie titles in TMDB
4. **Consider lower confidence**: Use `--min-confidence 0.5 --preview` for more permissive matching

### Safety Tips
1. **Start small**: Test with a few movies first
2. **Use backups**: Always have a backup before organizing
3. **Review changes**: Never skip the preview step
4. **Understand trade-offs**: Accuracy vs. coverage

## Advanced Usage

### Network Drives
The organizer is optimized for SMB/NFS network drives:
- Efficient file operations
- Proper error handling for network issues
- Progress reporting for large operations

### Batch Processing
For large directories:
1. **Start with preview**: `--preview --verbose`
2. **Review results**: Check organized vs. skipped
3. **Process in batches**: Organize smaller directories first
4. **Monitor progress**: Use `--verbose` for detailed output

### Custom Configuration
See [Configuration Guide](configuration.md) for advanced configuration options.
