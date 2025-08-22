# 🎬 Your First Movie Organization

Step-by-step guide to successfully organizing your first batch of movies.

## 🎯 Goal

Transform a messy collection of movie files into a perfectly organized, Plex-compatible library.

**Before:**
```
~/Downloads/movies/
├── Avengers.Endgame.2019.1080p.BluRay.x264.mkv
├── matrix_1999_720p.mp4
├── Hero.2002.DVDRip.XviD.avi
└── some.random.movie.file.mkv
```

**After:**
```
~/Movies/organized/
├── Avengers Endgame (2019)/
│   └── Avengers Endgame (2019) 1080p BluRay.mkv
├── The Matrix (1999)/
│   └── The Matrix (1999) 720p.mp4
├── Hero (2002)/
│   └── Hero (2002) DVDRip.avi
└── Unknown Movie (Unknown Year)/
    └── some.random.movie.file.mkv
```

## 📋 Prerequisites

- ✅ Plex Media Organizer installed ([installation guide](../installation.md))
- ✅ TMDB API key ([get free key](https://www.themoviedb.org/settings/api))
- ✅ 4-20 movie files to practice with

## 🚀 Step-by-Step Process

### Step 1: Prepare Test Data (2 minutes)

Create a small test collection to practice with:

```bash
# Create test directory
mkdir ~/movie-test
cd ~/movie-test

# Copy a few movie files for testing
cp ~/Downloads/movies/*.mkv ~/movie-test/
cp ~/Downloads/movies/*.mp4 ~/movie-test/

# Check what we have
ls -la ~/movie-test/
```

**🎯 Start Small**: Use 4-20 files for your first attempt. You can always run it on your full library later.

### Step 2: Test Individual Files (3 minutes)

Before organizing everything, test a few files to make sure your setup works:

```bash
# Test with your TMDB API key
TMDB_API_KEY=your_actual_key_here plex-media-organizer test "Avengers.Endgame.2019.1080p.BluRay.x264.mkv"
```

**Expected Output:**
```
🎬 Testing: Avengers.Endgame.2019.1080p.BluRay.x264.mkv

✅ Parsing Results:
  • Title: Avengers: Endgame
  • Year: 2019
  • Quality: 1080p
  • Source: BluRay
  • Codec: x264
  • Confidence: 98.5%

🎯 TMDB Match:
  • Title: Avengers: Endgame
  • Release Date: 2019-04-26
  • TMDB ID: 299534
  • Rating: 8.4/10

📁 Organized Name:
  • Filename: Avengers Endgame (2019) 1080p BluRay.mkv
  • Directory: Avengers Endgame (2019)/
```

**🔍 What to Look For:**
- ✅ **High confidence** (>90% is excellent, >70% is good)
- ✅ **TMDB match found** (means metadata is available)
- ✅ **Correct title and year** detected
- ✅ **Sensible organized name** generated

### Step 3: Scan Your Test Directory (2 minutes)

Now scan all files in your test directory:

```bash
# Scan all files to see what will happen
TMDB_API_KEY=your_key plex-media-organizer scan ~/movie-test/
```

**Expected Output:**
```
🔍 Scanning: /Users/you/movie-test
📁 Found: 4 movie files
⚡ Processing...

✅ Results:
  • Successfully parsed: 3/4 files (75%)
  • TMDB matches: 3/3 (100%)
  • Failed to parse: 1 file
  • Processing time: 1.2 seconds (3.3 files/sec)

📊 Parse Results:
  ✅ Avengers.Endgame.2019.1080p.BluRay.x264.mkv → Avengers: Endgame (2019)
  ✅ matrix_1999_720p.mp4 → The Matrix (1999)
  ✅ Hero.2002.DVDRip.XviD.avi → Hero (2002)
  ❌ some.random.movie.file.mkv → Failed to parse

🎯 TMDB Matches: 3/3 (100%)
⚡ Average confidence: 92.3%
```

**📊 Success Rate Expectations:**
- **75-90%** is normal for mixed collections
- **90-95%** is excellent
- **95%+** means you have well-named files

### Step 4: Preview Organization (1 minute)

See exactly what changes will be made without actually doing them:

```bash
# Dry run - shows what will happen but doesn't change anything
TMDB_API_KEY=your_key plex-media-organizer organize ~/movie-test/ \
    --output ~/Movies/organized \
    --dry-run
```

**Expected Output:**
```
🎬 Organizing: /Users/you/movie-test → /Users/you/Movies/organized
📁 Found: 4 movie files

🔍 Preview Mode (--dry-run) - No files will be changed

📋 Planned Changes:
  📁 CREATE: /Users/you/Movies/organized/Avengers Endgame (2019)/
  ➡️  MOVE: Avengers.Endgame.2019.1080p.BluRay.x264.mkv
      → Avengers Endgame (2019)/Avengers Endgame (2019) 1080p BluRay.mkv

  📁 CREATE: /Users/you/Movies/organized/The Matrix (1999)/
  ➡️  MOVE: matrix_1999_720p.mp4
      → The Matrix (1999)/The Matrix (1999) 720p.mp4

  📁 CREATE: /Users/you/Movies/organized/Hero (2002)/
  ➡️  MOVE: Hero.2002.DVDRip.XviD.avi
      → Hero (2002)/Hero (2002) DVDRip.avi

  ⚠️  SKIP: some.random.movie.file.mkv (could not parse)

📊 Summary:
  • Will organize: 3/4 files (75%)
  • Will create: 3 directories
  • Will skip: 1 file
  • Estimated time: < 1 second
```

**🎯 Review the Preview:**
- ✅ **Directory names** look correct
- ✅ **File names** follow Plex conventions
- ✅ **No conflicts** with existing files
- ✅ **Skipped files** are acceptable

### Step 5: Execute Organization (1 minute)

If the preview looks good, do the actual organization:

```bash
# Actually organize the files (with backup)
TMDB_API_KEY=your_key plex-media-organizer organize ~/movie-test/ \
    --output ~/Movies/organized \
    --backup ~/backups/first-organization.json
```

**Expected Output:**
```
🎬 Organizing: /Users/you/movie-test → /Users/you/Movies/organized
📁 Found: 4 movie files

⚡ Processing files...
  ✅ Avengers.Endgame.2019.1080p.BluRay.x264.mkv → Avengers Endgame (2019) 1080p BluRay.mkv
  ✅ matrix_1999_720p.mp4 → The Matrix (1999) 720p.mp4
  ✅ Hero.2002.DVDRip.XviD.avi → Hero (2002) DVDRip.avi
  ⚠️  some.random.movie.file.mkv → Skipped (could not parse)

✅ Organization Complete:
  • Successfully organized: 3/4 files (75%)
  • Created directories: 3
  • Skipped files: 1
  • Total size: 8.2 GB
  • Time: 0.8 seconds

💾 Backup created: ~/backups/first-organization.json

🔄 Rollback available:
  plex-media-organizer rollback ~/backups/first-organization.json
```

### Step 6: Verify Results (1 minute)

Check that everything worked correctly:

```bash
# Look at the organized structure
tree ~/Movies/organized/

# Or use ls if tree isn't available
ls -la ~/Movies/organized/*/
```

**Expected Structure:**
```
~/Movies/organized/
├── Avengers Endgame (2019)/
│   └── Avengers Endgame (2019) 1080p BluRay.mkv
├── Hero (2002)/
│   └── Hero (2002) DVDRip.avi
└── The Matrix (1999)/
    └── The Matrix (1999) 720p.mp4
```

**✅ Success Indicators:**
- Each movie has its own directory
- Directory names include year: `Movie Name (Year)`
- File names are clean and Plex-compatible
- Quality and source information preserved

## 🎉 Congratulations!

You've successfully organized your first batch of movies! 

### What You've Accomplished:
- ✅ **Learned the workflow**: Test → Scan → Preview → Execute
- ✅ **Created Plex-compatible structure**: Each movie in its own directory
- ✅ **Generated clean filenames**: Easy to read and media server friendly
- ✅ **Created rollback capability**: Can undo changes if needed

## 🚀 Next Steps

### Ready for More?

1. **Handle Failed Files**: See [testing-files.md](testing-files.md) for dealing with files that couldn't be parsed
2. **Organize Full Library**: Apply this to your complete movie collection
3. **Set Up Plex**: Import your organized library into Plex Media Server
4. **Automate**: Set up automatic organization for new downloads

### Scale Up to Full Library

```bash
# When you're confident, organize your full collection
TMDB_API_KEY=your_key plex-media-organizer organize ~/Downloads/movies/ \
    --output ~/Movies/organized \
    --backup ~/backups/full-library-$(date +%Y%m%d).json
```

## 🛠️ Troubleshooting

### Common Issues

#### "API key not found"
```bash
# Make sure your API key is set correctly
echo $TMDB_API_KEY
# Should show your key, not empty

# Or set it in config file instead
plex-media-organizer config --set apis.tmdb_api_key your_key_here
```

#### "No files found"
```bash
# Check for supported file extensions
ls ~/movie-test/*.{mkv,mp4,avi,mov,wmv}

# Supported extensions: mkv, mp4, avi, mov, wmv, flv, webm
```

#### Low success rate
```bash
# Check what failed
plex-media-organizer scan ~/movie-test/ --export-failures failures.txt
cat failures.txt

# Common causes:
# - Very messy filenames
# - Non-English titles
# - Very old or obscure movies
```

### Getting Help

- **Detailed troubleshooting**: [troubleshooting.md](../../troubleshooting.md)
- **Configuration options**: [configuration.md](../../configuration.md)
- **Advanced workflows**: [../advanced/](../advanced/)

---

**🎯 Total Time**: ~10 minutes from start to organized movies

**💡 Pro Tip**: Always use `--dry-run` first and create backups for important collections!
