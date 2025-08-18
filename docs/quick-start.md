# 🚀 Quick Start Guide

Get up and running with Plex Media Organizer in under 5 minutes!

## 📋 Prerequisites (1 minute)

### 1. Install Rust
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
```

### 2. Get TMDB API Key
1. Go to [themoviedb.org](https://www.themoviedb.org/)
2. Create a free account (30 seconds)
3. Go to **Settings** → **API** → **Create** → **Developer**
4. Copy your API key (looks like `abc123def456...`)

## 🔧 Installation (2 minutes)

### Option A: From Source (Recommended)
```bash
# Clone the repository
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer

# Build the project
cargo build --release

# The binary will be at: ./target/release/plex-media-organizer
```

### Option B: Quick Install (Future)
```bash
# Coming soon: Package manager installs
cargo install plex-media-organizer
# OR
brew install plex-media-organizer
```

## 🧪 First Test (1 minute)

Test that everything works with a sample filename:

```bash
# Replace YOUR_API_KEY with your actual TMDB API key
TMDB_API_KEY=YOUR_API_KEY ./target/release/plex-media-organizer test "Avengers.Endgame.2019.1080p.BluRay.x264.mkv"
```

**Expected Output:**
```
🎬 Parsing: Avengers.Endgame.2019.1080p.BluRay.x264.mkv
✅ Movie: Avengers: Endgame (2019)
📊 Confidence: 98.5%
🎯 TMDB Match: Yes (ID: 299534)
⚡ Quality: 1080p
📁 Plex Name: Avengers Endgame (2019) 1080p BluRay.mkv

✅ Success! Your setup is working correctly.
```

## 🎬 Scan Your First Directory (1 minute)

Now let's scan a real directory with your movies:

```bash
# Scan a directory (dry run first - won't change anything)
TMDB_API_KEY=YOUR_API_KEY ./target/release/plex-media-organizer scan /path/to/your/movies --dry-run

# If results look good, scan for real
TMDB_API_KEY=YOUR_API_KEY ./target/release/plex-media-organizer scan /path/to/your/movies
```

**Example Output:**
```
🔍 Scanning: /Users/you/Movies
📁 Found: 25 movie files
⚡ Processing at 180+ files/second...

✅ Successfully parsed: 25/25 files (100%)
🎯 TMDB matches: 24/25 (96%)
📊 Average confidence: 89.2%
⏱️  Total time: 0.8 seconds

📋 Summary:
  • 24 movies identified with TMDB data
  • 1 movie parsed from filename only
  • 0 failures
```

## 🎯 Organize Your Movies (1 minute)

Now let's organize your movies with Plex naming conventions:

```bash
# Preview organization (safe - won't change anything)
TMDB_API_KEY=YOUR_API_KEY ./target/release/plex-media-organizer organize /path/to/your/movies --preview

# If preview looks good, organize for real
TMDB_API_KEY=YOUR_API_KEY ./target/release/plex-media-organizer organize /path/to/your/movies
```

**Example Output:**
```
🎬 Organizing: /Users/you/Movies
📁 Found: 25 movie files
⚡ Processing...

✅ Organized: 25/25 files (100%)
📊 Success rate: 100%
⏱️  Total time: 1.2 seconds

📄 Organization result saved to: organization_result_abc123.json
```

## 🔄 Rollback (If Needed)

If you need to undo an organization:

```bash
# Preview rollback (safe)
./target/release/plex-media-organizer rollback organization_result_abc123.json --preview

# Actually rollback
./target/release/plex-media-organizer rollback organization_result_abc123.json
```

## 🧹 Cleanup (Optional)

Manage old organization files:

```bash
# Preview cleanup (safe)
./target/release/plex-media-organizer cleanup --preview

# Clean up files older than 30 days
./target/release/plex-media-organizer cleanup
```

## ⚙️ Configuration (Optional)

For persistent configuration, create a config file:

```bash
# Create config directory
mkdir -p ~/.config/plex-media-organizer

# Create config file
cat > ~/.config/plex-media-organizer/config.toml << EOF
[apis]
tmdb_api_key = "YOUR_API_KEY"

[organization.quality]
preferred_quality = "1080p"

[organization.original_titles]
prefer_original_titles = false
include_english_subtitle = false
fallback_to_english_on_error = true
preserve_original_in_metadata = true
EOF
```

Now you can run commands without the `TMDB_API_KEY` environment variable:

```bash
./target/release/plex-media-organizer scan /path/to/movies
```

## 🎉 You're Ready!

You now have a working movie organizer! Here's what you can do next:

### **Next Steps:**
- 📖 **[User Guide](user-guide.md)** - Learn all features and advanced usage
- ⚙️ **[Configuration](configuration.md)** - Customize behavior and preferences  
- 💡 **[Examples](examples/)** - See real-world usage examples
- 🔧 **[Troubleshooting](troubleshooting.md)** - Fix common issues

### **Common Commands:**
```bash
# Test a single file
plex-media-organizer test "filename.mkv"

# Scan a directory
plex-media-organizer scan /path/to/movies

# Organize files (rename to Plex conventions)
plex-media-organizer organize /path/to/movies --dry-run

# View current configuration
plex-media-organizer config

# Get help for any command
plex-media-organizer --help
plex-media-organizer scan --help
```

## 🆘 Need Help?

- **Quick Issues**: Check [troubleshooting.md](troubleshooting.md)
- **Questions**: Open an [issue on GitHub](https://github.com/lijunzh/plex-media-organizer/issues)
- **Feature Requests**: See our [roadmap](../project/roadmap.md) or suggest new ideas

---

**🎯 Goal Achieved:** You can now parse and organize your movie collection with confidence!

**⏱️ Total Time:** Under 5 minutes from zero to working movie organizer.
