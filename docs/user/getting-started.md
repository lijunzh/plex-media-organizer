# Quick Start Guide

Get up and running with Plex Media Organizer in minutes.

## Prerequisites

- Rust 1.70+ installed
- TMDB API key (optional, free at https://www.themoviedb.org/settings/api)

## Step 1: Installation

### From Source
```bash
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release
```

### Add to PATH (Optional)
```bash
# Add the binary to your PATH
export PATH="$PATH:$(pwd)/target/release"
```

## Step 2: Initial Setup

Run the interactive setup to configure the application:

```bash
plex-media-organizer setup
```

This will guide you through:
- Database configuration
- TMDB API key setup (optional)
- Default output directory
- Confidence thresholds
- Network mode settings

## Step 3: Scan Your Media Directory

Scan a directory to see what media files are found:

```bash
# Scan a directory for media files
plex-media-organizer scan /path/to/movies --verbose

# This will show:
# - File type statistics
# - Potential organization candidates
# - Files that might be skipped
# - Estimated organization time
```

## Step 4: Test Parsing

Test the parsing functionality on your media files:

```bash
# Test parsing without organizing
plex-media-organizer test /path/to/movies --verbose

# Expected output shows:
# - Parsed movie information
# - TMDB match (if found)
# - Confidence score
# - Cache performance (if using --use-cache)
```

## Step 5: Preview Organization

Before organizing your entire collection, preview the changes:

```bash
# Preview organization of a small directory
plex-media-organizer organize /path/to/movies --preview --verbose

# Review the output:
# - Which files will be organized
# - Which files will be skipped
# - The new directory structure
# - Operation details
```

## Step 6: Organize Your Movies

Once you're satisfied with the preview, organize your movies:

```bash
# Organize with default settings (move files)
plex-media-organizer organize /path/to/movies

# Or organize with copy operation (safer for testing)
plex-media-organizer organize /path/to/movies --copy

# Or organize with custom output directory
plex-media-organizer organize /path/to/movies -o /organized/movies
```

## Step 7: Manage Operations

### View Configuration
```bash
# View current configuration
plex-media-organizer config --verbose
```

### Rollback Operations (if needed)
```bash
# List recent operations (operation ID will be shown after organize)
plex-media-organizer rollback <operation-id> --preview

# Execute rollback
plex-media-organizer rollback <operation-id>
```

### Cleanup Old Operations
```bash
# Clean up old operations and optimize database
plex-media-organizer cleanup --keep-count 20
```

## Common First-Time Issues

### Movies Being Skipped
This is **normal and expected**. The organizer uses intelligent parsing:

```bash
# For more permissive matching (requires preview)
plex-media-organizer organize /path/to/movies --min-confidence 0.5 --preview
```

### TMDB API Key Issues
If you get API errors:
1. Verify your API key at https://www.themoviedb.org/settings/api
2. Run `plex-media-organizer setup` again
3. Check your internet connection
4. The application works without TMDB API key for basic functionality

### Permission Issues
If you get permission errors:
```bash
# Make sure the binary is executable
chmod +x target/release/plex-media-organizer

# Check file permissions
ls -la /path/to/movies
```

### Database Issues
If you get database errors:
```bash
# Re-run setup to recreate database
plex-media-organizer setup --force
```

## Next Steps

- Read the [CLI Commands Reference](cli-commands.md) for detailed usage
- Check [Examples](examples/) for real-world scenarios
- Review [Troubleshooting](troubleshooting.md) for common issues

## Safety Tips

1. **Always use `--preview` first**
2. **Start with a small directory**
3. **Review skipped files carefully**
4. **Use `--copy` for testing before moving files**
5. **Keep track of operation IDs for potential rollback**

## Complete Workflow Example

```bash
# 1. Setup
plex-media-organizer setup

# 2. Scan directory
plex-media-organizer scan /path/to/movies

# 3. Test parsing
plex-media-organizer test /path/to/movies --use-cache

# 4. Preview organization
plex-media-organizer organize /path/to/movies --preview

# 5. Organize files
plex-media-organizer organize /path/to/movies

# 6. Cleanup old operations
plex-media-organizer cleanup --keep-count 10
```

Remember: The organizer is designed to be safe and conservative. It's better to skip a file than to organize it incorrectly!
