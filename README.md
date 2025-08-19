# Plex Media Organizer

An intelligent media file organizer that follows Plex naming conventions and uses TMDB for accurate movie metadata.

## Features

- **TMDB Integration**: Uses The Movie Database for accurate movie information
- **Plex-Compatible Naming**: Follows Plex naming conventions for optimal indexing
- **Multilingual Support**: Handles Chinese, Japanese, and English titles
- **Conservative Approach**: Prioritizes accuracy over completeness
- **Preview Mode**: Dry-run mode to preview changes before applying
- **Rollback Support**: Easy rollback of organization operations
- **Network Drive Optimization**: Optimized for SMB/NFS network drives

## Quick Start

### 1. Setup
```bash
# Run interactive setup
plex-media-organizer setup

# This will prompt for your TMDB API key
# Get one at: https://www.themoviedb.org/settings/api
```

### 2. Organize Movies
```bash
# Preview organization (recommended first)
plex-media-organizer organize /path/to/movies --preview --verbose

# Apply organization
plex-media-organizer organize /path/to/movies --backup /path/to/backup
```

### 3. Test Parsing
```bash
# Test a single file
plex-media-organizer test /path/to/movie.mkv --verbose

# Test entire directory
plex-media-organizer test /path/to/movies --organize --preview
```

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

## Documentation

- [User Guide](docs/user-guide.md) - Comprehensive usage tutorial
- [Quick Start](docs/quick-start.md) - Get started quickly
- [Configuration](docs/configuration.md) - All configuration options
- [Troubleshooting](docs/troubleshooting.md) - Common issues and solutions
- [Examples](docs/examples/) - Real-world usage examples

### For Developers

- [Development Guide](project/development/README.md) - Complete developer documentation
- [Current Limitations](project/CURRENT_LIMITATIONS.md) - Detailed explanation of trade-offs
- [Iteration 2 Plan](project/ITERATION_2_PLAN.md) - Future improvements roadmap
- [Architecture](project/architecture.md) - System design and architecture
- [Roadmap](project/roadmap.md) - Development timeline and planning

## Installation

### From Source
```bash
git clone https://github.com/your-repo/plex-media-organizer.git
cd plex-media-organizer
cargo build --release
```

### Requirements
- Rust 1.70+
- TMDB API key (free at https://www.themoviedb.org/settings/api)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

- **Issues**: Report bugs and feature requests on GitHub
- **Documentation**: Check the docs/ directory for detailed information
- **Trade-offs**: Understand the conservative approach in [Current Limitations](project/CURRENT_LIMITATIONS.md)

---

**Remember**: It's always better to skip a movie than to organize it incorrectly!
