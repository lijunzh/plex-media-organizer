# Quick Start Guide

Get up and running with Plex Media Organizer in minutes.

## Prerequisites

- Rust 1.70+ installed
- TMDB API key (free at https://www.themoviedb.org/settings/api)

## Step 1: Installation

### From Source
```bash
git clone https://github.com/your-repo/plex-media-organizer.git
cd plex-media-organizer
cargo build --release
```

### Add to PATH (Optional)
```bash
# Add the binary to your PATH
export PATH="$PATH:$(pwd)/target/release"
```

## Step 2: Initial Setup

Run the interactive setup to configure your TMDB API key:

```bash
plex-media-organizer setup
```

This will:
- Prompt for your TMDB API key
- Create a configuration file
- Test the connection to TMDB

## Step 3: Test with a Single File

Start with a single movie file to verify everything works:

```bash
# Test parsing without organizing
plex-media-organizer test /path/to/movie.mkv --verbose

# Expected output shows:
# - Parsed movie information
# - TMDB match (if found)
# - Confidence score
```

## Step 4: Preview Organization

Before organizing your entire collection, preview the changes:

```bash
# Preview organization of a small directory
plex-media-organizer organize /path/to/movies --preview --verbose

# Review the output:
# - Which files will be organized
# - Which files will be skipped
# - The new directory structure
```

## Step 5: Organize Your Movies

Once you're satisfied with the preview, organize your movies:

```bash
# Organize with backup (recommended)
plex-media-organizer organize /path/to/movies --backup /path/to/backup

# Or organize without backup (use with caution)
plex-media-organizer organize /path/to/movies
```

## Common First-Time Issues

### Movies Being Skipped
This is **normal and expected**. The organizer uses a conservative approach:

```bash
# For more permissive matching (requires preview)
plex-media-organizer organize /path/to/movies --min-confidence 0.5 --preview
```

### TMDB API Key Issues
If you get API errors:
1. Verify your API key at https://www.themoviedb.org/settings/api
2. Run `plex-media-organizer setup` again
3. Check your internet connection

### Permission Issues
If you get permission errors:
```bash
# Make sure the binary is executable
chmod +x target/release/plex-media-organizer

# Check file permissions
ls -la /path/to/movies
```

## Next Steps

- Read the [User Guide](user-guide.md) for detailed usage
- Check [Examples](examples/) for real-world scenarios
- Review [Troubleshooting](troubleshooting.md) for common issues

## Safety Tips

1. **Always use `--preview` first**
2. **Use `--backup` for important data**
3. **Start with a small directory**
4. **Review skipped movies carefully**

Remember: It's better to skip a movie than to organize it incorrectly!
