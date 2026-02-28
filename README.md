# 🎬 Plex Media Organizer

**Organize torrent-downloaded movies, TV shows, and music into
[Plex-compatible folder structures](https://support.plex.tv/articles/naming-and-organizing-your-movie-media-files/) —
powered by [hunch](https://crates.io/crates/hunch) for fast, offline filename parsing.**

Plex Media Organizer parses messy media filenames, classifies them by type,
and moves/copies/symlinks them into clean folder hierarchies that Plex
recognizes automatically. Dry-run by default. Undo everything.

## Install

### Cargo (from source)

```bash
cargo install plex-media-organizer
```

### As a library

```bash
cargo add plex-media-organizer
```

## Quick Start

```bash
plex-org scan /downloads                          # See what's detected
plex-org plan /downloads -d /plex                  # Preview the plan
plex-org organize /downloads -d /plex --execute    # Do it
plex-org undo                                      # Oops? Reverse it
```

## Usage

### scan

Discover media files and show parsed metadata.

```bash
$ plex-org scan /downloads

Filename                                           Type     Title                                    Year   S/E      Size
--------------------------------------------------------------------------------------------------------------------------
Breaking.Bad.S01E01.720p.HDTV.x264-LOL.mkv         tv       Breaking Bad                                    S01E01   1.4 GB
Inception.2010.2160p.UHD.BluRay.mkv                movie    Inception                                2010            8.2 GB
The.Matrix.1999.1080p.BluRay.x264-GROUP.mkv        movie    The Matrix                               1999            2.1 GB

3 media files found.
```

### plan

Preview the organization plan without touching any files.

```bash
$ plex-org plan /downloads -d /media/plex

📋 Plan (3 actions):

  /downloads/Breaking.Bad.S01E01.mkv → /media/plex/TV Shows/Breaking Bad/Season 01/Breaking Bad - S01E01.mkv
  /downloads/Inception.2010.mkv → /media/plex/Movies/Inception (2010)/Inception (2010).mkv
  /downloads/The.Matrix.1999.mkv → /media/plex/Movies/The Matrix (1999)/The Matrix (1999).mkv

Dry-run complete. Use `organize --execute` to apply.
```

### organize

Execute the plan. **Dry-run by default** — requires `--execute`.

```bash
plex-org organize /downloads -d /media/plex                            # Preview (dry-run)
plex-org organize /downloads -d /media/plex --execute                  # Move files
plex-org organize /downloads -d /media/plex --execute -s copy          # Keep originals
plex-org organize /downloads -d /media/plex --execute -s symlink       # Zero-copy
```

### undo

Reverse the last organize operation.

```bash
plex-org undo
```

### config

View the current configuration.

```bash
plex-org config
```

### Global Options

```
-v, --verbose       Increase verbosity (-v, -vv, -vvv)
-c, --config <FILE> Use a custom config file
-h, --help          Print help
-V, --version       Print version
```

## Output Structures

```
Movies/Movie Name (Year)/Movie Name (Year).ext
Movies/Movie Name (Year)/Movie Name (Year).en.srt    ← subtitles co-located
TV Shows/Show Name/Season XX/Show Name - SXXEXX - Episode Title.ext
TV Shows/Show Name/Season XX/Show Name - SXXEXX.en.srt
Music/Artist/Album (Year)/01 - Track.ext
```

Subtitle files (`.srt`, `.ass`, `.sub`, `.vtt`, `.ssa`, `.idx`) are automatically
discovered next to video files (including `Subs/` subdirectories) and moved
alongside them with matching names. Language suffixes like `.en`, `.zh.forced`
are preserved.

## Configuration

Plex Media Organizer reads TOML configuration files. Override with `--config`:

```bash
plex-org plan /downloads --config ~/my_config.toml
```

### Full Config Reference

```toml
source_dirs = []
destination = ""
auto_organize_threshold = 90.0    # Above this: auto-organize
review_threshold = 50.0           # Below this: flag for manual review

[organize]
strategy = "move"        # move | copy | symlink
movies_dir = "Movies"
tv_dir = "TV Shows"
music_dir = "Music"
```

## Strategies: Move / Copy / Symlink

| Strategy | Behavior | Use Case |
|----------|----------|----------|
| `move` | Moves files to destination | Default; clean source dir |
| `copy` | Copies files (originals stay) | Keep seeding torrents |
| `symlink` | Creates symlinks | Fastest, zero disk overhead |

---

## The Journey: From Python to Rust

Plex Media Organizer started as a **Python prototype** (`plex_organizer`)
to validate the idea and nail down the pipeline design. That prototype
reached 560 tests, full TMDb/MusicBrainz enrichment, and a standalone
Nuitka binary — but it depended on Python's `guessit` library for
filename parsing.

Meanwhile, **[hunch](https://github.com/lijunzh/hunch)** was built as a
Rust rewrite of guessit — a fast, offline, deterministic media filename
parser with 81.7% guessit compatibility and microsecond-level performance.

With hunch at v1.0, it was time to bring it all together.

### What hunch replaced

The Python version had **~900 lines** of parsing infrastructure:

| Python Module | Lines | Replaced By |
|---|---|---|
| `core/normalizer.py` | 150 | hunch tokenizer + zone map |
| `core/classifier.py` | 130 | `hunch::MediaType` |
| `core/parser.py` | 200 | `hunch::hunch()` — one function call |
| `rules/engine.py` | 70 | hunch TOML rule engine |
| `config/rules.yaml` | 60 | hunch's 20 embedded TOML files |
| `core/context.py` | 150 | hunch path-segment tokenizer |

All of this collapsed into a single function call:

```rust
let result = hunch::hunch("The.Matrix.1999.1080p.BluRay.x264-GROUP.mkv");
// → title: "The Matrix", year: 1999, source: "Blu-ray", ...
```

### What carried over

The Python prototype's **pipeline design** and **data models** translated
cleanly to Rust:

| Concept | Python | Rust |
|---|---|---|
| Pipeline | `Scan → Classify → Parse → Enrich → Organize` | `Scan → Parse (hunch) → Enrich → Organize` |
| Data models | Pydantic | `serde` structs |
| CLI | Typer + Rich | clap |
| Config | TOML + deep merge | TOML + serde |
| Undo | JSON manifests | JSON manifests |
| Strategies | move / copy / symlink | move / copy / symlink |
| Subtitle co-location | companion discovery | companion discovery |

### Architecture comparison

```
Python plex_organizer (v0.1, 560 tests, ~3,600 lines core):
  cli/ → core/ → models/ → providers/ → rules/ → utils/
     7 core modules + 5 providers + rule engine + normalizer

Rust plex-media-organizer (v0.2, 24 tests, ~1,600 lines):
  main.rs → lib.rs → 9 modules
     No normalizer, no classifier, no rule engine — hunch handles it
```

The Rust version is **~55% fewer lines** for the same Phase 1 functionality,
and produces a single static binary with no runtime dependencies.

### Previous attempts (v0.1.x)

v0.1.0 and v0.1.1 attempted a Rust media organizer without hunch, using
custom regex patterns for filename parsing. These had limited accuracy and
couldn't handle the long tail of real-world filenames (anime, foreign
languages, multi-episode ranges, release group disambiguation, etc.).

hunch's 49-property parser with 81.7% guessit compatibility solved this
entirely — the organizer can now focus purely on enrichment and file
operations.

---

## Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| 1 | ✅ | Scanner + hunch integration + organize/undo + CLI |
| 2 | 📋 | TMDb provider + movie/TV enrichment (`reqwest`) |
| 3 | 📋 | MusicBrainz provider + proper music parser |
| 4 | 📋 | Web search fallback (DuckDuckGo → TMDb/MB ID resolution) |
| 5 | 📋 | NFO sidecar parsing (`quick-xml`) |
| 6 | 📋 | Polish: progress bars, colored output, shell completions |

## Development

```bash
cargo test       # Run all tests (24 tests)
cargo build      # Build debug binary
cargo clippy     # Lint
cargo fmt        # Format
cargo run -- scan /path/to/media  # Run directly
```

See **[ARCHITECTURE.md](ARCHITECTURE.md)** for the full design,
decision log, and developer guide.

## License

[MIT](LICENSE)
