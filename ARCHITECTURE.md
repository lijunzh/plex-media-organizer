# ARCHITECTURE.md — Plex Media Organizer

> **Decision log, architectural rationale, and developer guide.**
> This is the single source of truth for how the project works and why.

---

## Overview

Plex Media Organizer takes a directory of messy torrent downloads and
organizes them into Plex-compatible folder structures.

| Field | Value |
|---|---|
| Crate name | `plex-media-organizer` |
| Binary name | `plex-org` |
| Language | Rust (edition 2021) |
| License | MIT |
| Parser | [hunch](https://crates.io/crates/hunch) v1.0 |
| Ancestor | Python `plex_organizer` (prototype, 560 tests) |

The problem decomposes into four stages:

1. **Scan** — walk directories, filter by extension/size, skip extras
2. **Parse** — extract title, year, season, episode from filenames
3. **Enrich** — validate against TMDb/MusicBrainz databases
4. **Organize** — build Plex paths, move/copy/symlink, undo

---

## Layered Architecture

```
┌──────────────────────────────────────────────────────────────┐
│ Layer 0: Filename Parsing (hunch crate)               │
│   • 49 typed properties from filenames                 │
│   • TOML-driven, regex-only, offline, deterministic     │
│   • microsecond latency, single function call            │
├──────────────────────────────────────────────────────────────┤
│ Layer 1: Pipeline Orchestration (this crate)           │
│   • Scanner: walkdir + extension/size/skip filters      │
│   • Parser: hunch integration + music regex placeholder  │
│   • Enricher: DB lookup + web search fallback            │
│   • Organizer: Plex paths, move/copy/symlink, undo       │
├──────────────────────────────────────────────────────────────┤
│ Layer 2: Database Providers (future — Phase 2–4)       │
│   • TMDb: movie/TV title validation + metadata           │
│   • MusicBrainz: music metadata                         │
│   • Web search: DuckDuckGo → TMDb/MB ID resolution       │
└──────────────────────────────────────────────────────────────┘
```

**Hard boundary**: hunch is a pure, offline parser. Network I/O, database
lookups, and file operations live in this crate, never in hunch.

---

## Pipeline

```
Input: /downloads/ directory
  │
  ├─ 1. Scan (scanner.rs)
  │     ├─ walkdir: recursive directory traversal
  │     ├─ Extension filter: video (.mkv .mp4 ...) + audio (.flac .mp3 ...)
  │     ├─ Size filter: skip video files < 50 MB (menus/promos)
  │     ├─ Skip patterns: sample, trailer, extras, featurettes
  │     ├─ Skip dirs: .hidden, @eaDir, #recycle, lost+found
  │     └─ Output: Vec<MediaFile>
  │
  ├─ 2. Parse (parser.rs)
  │     ├─ Video: hunch::hunch(filename) → 49 typed properties
  │     ├─ Music: regex placeholder (track number, artist/album from dir)
  │     ├─ Confidence scoring: 0–100 based on fields populated
  │     └─ Output: ParsedMedia per file
  │
  ├─ 3. Enrich (enricher.rs)
  │     ├─ Phase 1 (current): pass-through, wraps parsed into enriched models
  │     ├─ Phase 2 (planned): TMDb/MusicBrainz lookup + fuzzy confidence
  │     ├─ Phase 4 (planned): web search fallback for low-confidence
  │     └─ Output: EnrichedMedia per file
  │
  ├─ 4. Organize (organizer.rs)
  │     ├─ Path builder: Plex-compatible destination paths
  │     ├─ Subtitle companion discovery (subtitles.rs)
  │     ├─ Duplicate detection with counter suffix
  │     ├─ Execute: move / copy / symlink
  │     ├─ Undo: JSON manifest + reverse operations
  │     └─ Output: UndoManifest
  │
  └─ 5. CLI (cli.rs)
        ├─ scan: tabular output of discovered files
        ├─ plan: dry-run preview
        ├─ organize: execute with --execute flag
        ├─ undo: reverse last operation
        └─ config: show current settings
```

---

## Module Map

```
src/
├── lib.rs          # Library root, public API
├── main.rs         # CLI entry point (tracing setup)
├── cli.rs          # Command dispatch (clap derive)
├── models.rs       # Data types:
│                     #   MediaFile      — discovered file on disk
│                     #   ParsedMedia    — extracted metadata
│                     #   EnrichedMedia  — validated metadata
│                     #   Movie / TvEpisode / MusicTrack
│                     #   OrganizeAction / UndoManifest
├── config.rs       # TOML config with serde deserialization
├── scanner.rs      # Directory walker + extension/skip/size filters
├── parser.rs       # hunch integration (video) + regex (music)
├── enricher.rs     # DB enrichment orchestrator (pass-through in v0.2)
├── organizer.rs    # Path builder, execute, undo, cleanup
├── subtitles.rs    # Subtitle companion discovery
└── utils.rs        # sanitize_name, format_size, safe_path_join

config/
└── default_config.toml  # Default configuration
```

---

## Design Principles

| Principle | How It's Applied |
|---|---|
| **Dry-run by default** | `--execute` required for file operations |
| **Undo everything** | JSON manifest written per operation |
| **Offline first** | hunch parses filenames with zero network I/O |
| **Core never prints** | Library returns data; CLI formats output |
| **One-way deps** | `lib.rs` never imports from `cli.rs` |
| **Confidence scoring** | ≥90%: auto-organize; <50%: review (future) |
| **Fail gracefully** | TMDb down? Continue with parser confidence |
| **≤600 line files** | Split by responsibility if growing beyond |

---

## The hunch Integration

The core parsing integration is a single function call:

```rust
use hunch::{hunch, HunchResult, MediaType};

let result = hunch("The.Matrix.1999.1080p.BluRay.x264-GROUP.mkv");

// 49 typed property accessors:
assert_eq!(result.title(), Some("The Matrix"));
assert_eq!(result.year(), Some(1999));
assert_eq!(result.source(), Some("Blu-ray"));
assert_eq!(result.video_codec(), Some("H.264"));
assert_eq!(result.screen_size(), Some("1080p"));
assert_eq!(result.release_group(), Some("GROUP"));
assert_eq!(result.container(), Some("mkv"));
assert_eq!(result.media_type(), Some(MediaType::Movie));
```

### What hunch replaces

In the Python prototype, five modules (~900 lines) handled filename
parsing. In Rust, hunch replaces all of them:

| Python Module | Rust Equivalent |
|---|---|
| `core/normalizer.py` (CJK, AKA, channels) | hunch tokenizer + zone map |
| `core/classifier.py` (3-pass type detection) | `hunch::MediaType` |
| `core/parser.py` (guessit wrapper + post-processing) | `hunch::hunch()` |
| `rules/engine.py` (YAML regex rules) | hunch's 20 TOML rule files |
| `core/context.py` (directory parsing) | hunch path-segment tokenizer |

### What hunch does NOT handle

- **Music filenames** — hunch is video-focused. Music uses regex placeholders.
- **Database enrichment** — hunch is offline. TMDb/MB live in this crate.
- **File operations** — hunch is a parser. Move/copy/symlink live here.
- **Confidence tuning** — hunch returns properties; confidence is computed here.

---

## Plex Output Structures

### Movies

```
Movies/Movie Name (Year)/Movie Name (Year).ext
Movies/Movie Name (Year)/Movie Name (Year).en.srt
```

### TV Shows

```
TV Shows/Show Name/Season XX/Show Name - SXXEXX - Episode Title.ext
TV Shows/Show Name/Season XX/Show Name - SXXEXX.en.srt
```

### Music

```
Music/Artist/Album (Year)/01 - Track.ext
```

### Path sanitization

- Characters `< > : " / \ | ? *` and control characters are stripped
- Multiple spaces collapsed to one
- Trailing dots removed
- Max component length: 200 characters
- Empty names fall back to "Unknown"

---

## Confidence Scoring

Parser confidence is a 0–100 heuristic based on how many fields hunch
populated. Capped at 85% — database enrichment is needed for higher.

| Signal | Points |
|---|---|
| Title present | +30 |
| Year present | +20 |
| Media type resolved | +15 |
| Season (TV only) | +15 |
| Episode (TV only) | +15 |
| Year + Movie | +15 |
| **Cap** | **85** |

### Enrichment thresholds (future phases)

```
    0 ─────────── 50 ─────────────── 90 ─── 100
    |  needs       |  web search      |  auto  |
    |  review      |  attempted       |  org   |
```

---

## Subtitle Companion Discovery

For each video file, the organizer searches for subtitles:

1. **Same directory** — files sharing the video's stem
2. **Subdirectories** — `Subs/`, `subs/`, `Subtitles/`, `subtitles/`

Language suffixes are preserved:

```
Movie.2024.srt           → suffix: ""
Movie.2024.en.srt        → suffix: ".en"
Movie.2024.zh.forced.srt → suffix: ".zh.forced"
```

On case-insensitive filesystems (macOS HFS+/APFS), directories are
deduplicated by canonical path to avoid double-counting.

---

## Undo System

Every `organize --execute` writes a JSON undo manifest to
`~/.plex-organizer/undo/`:

```json
{
  "version": 1,
  "created_at": "2026-02-28T10:30:00Z",
  "description": "Organize run at 2026-02-28 10:30:00 UTC",
  "entries": [
    {
      "source": "/downloads/Movie.mkv",
      "destination": "/plex/Movies/Movie (2024)/Movie (2024).mkv",
      "strategy": "move",
      "timestamp": "...",
      "title": "Movie",
      "media_type": "movie"
    }
  ]
}
```

`plex-org undo` reads the most recent manifest and reverses operations:
- **move**: moves files back to original location
- **copy**: deletes the copy
- **symlink**: removes the symlink

Empty parent directories are cleaned up (3 levels deep).

---

## Cross-Filesystem Moves

`fs::rename` fails across filesystem boundaries (common with NAS mounts).
The organizer automatically falls back to copy + delete:

```rust
fs::rename(&source, &dest).or_else(|_| {
    fs::copy(&source, &dest)?;
    fs::remove_file(&source)?;
    Ok(())
})
```

---

## Music Parsing (Placeholder)

hunch is video-focused and doesn't parse music filenames. Until a proper
music parser is built, two simple regex patterns handle common cases:

```
Filename:  "01 - Song Title.flac"    → track_number=1, track_title="Song Title"
Directory: "Artist - Album (2020)"   → artist, album, year
```

Confidence starts at 40% for music (regex-only) and gains +20% if the
parent directory matches the `Artist - Album (Year)` pattern.

---

## Dependencies

| Crate | Purpose | Status |
|---|---|---|
| `hunch` | Filename parsing (49 properties) | Core, permanent |
| `clap` | CLI argument parsing | Permanent |
| `serde` + `serde_json` | Serialization (config, undo manifests) | Permanent |
| `toml` | Config file parsing | Permanent |
| `walkdir` | Recursive directory traversal | Permanent |
| `regex` | Music filename placeholder | Permanent |
| `chrono` | Timestamps for undo manifests | Permanent |
| `tracing` + `tracing-subscriber` | Structured logging | Permanent |
| `anyhow` | Error handling (binary) | Permanent |
| `thiserror` | Error types (library) | Permanent |
| `reqwest` | HTTP client for TMDb/MB | Planned (Phase 2) |
| `quick-xml` | NFO sidecar parsing | Planned (Phase 5) |

---

## Decision Log

### D001: Use hunch as the parsing engine

**Status**: Decided, permanent.

hunch is a Rust rewrite of Python's guessit with 81.7% compatibility,
49 typed properties, and microsecond-level performance. Using it as a
library dependency replaces ~900 lines of Python parsing infrastructure
with a single function call.

**Consequences**:
- No normalizer, classifier, or rule engine needed in this crate
- Music parsing is the only gap (hunch is video-focused)
- Parser accuracy improvements benefit all hunch consumers
- Single static binary with no Python/pip/venv dependencies

### D002: Sync providers (not async)

**Status**: Decided for Phase 2, revisit if batch performance matters.

The Python prototype used synchronous HTTP clients (adequate for CLI).
The Rust version starts the same way with `reqwest::blocking`.

**Tradeoff**: async would enable parallel TMDb calls during batch
enrichment, but adds tokio complexity. Starting sync is simpler and
matches the CLI use case.

### D003: Dry-run by default

**Status**: Decided, permanent.

File operations require `--execute`. This prevents accidental data loss
and makes `plan` the default mental model.

### D004: JSON undo manifests

**Status**: Decided, permanent.

Every organize operation is reversible via a JSON manifest. This is
simpler than a database, human-readable, and git-friendly.

### D005: Confidence cap at 85% for parser-only

**Status**: Decided, permanent.

Parser confidence is capped at 85% because filename parsing alone cannot
distinguish ambiguous cases (e.g., "2001" as title vs year). Database
enrichment is required for higher confidence.

### D006: Python prototype as design reference

**Status**: Historical.

The Python `plex_organizer` (560 tests, 5 phases complete) validated the
pipeline design, enrichment strategy, and edge cases before the Rust
rewrite. Its test suite and CLAUDE.md serve as specifications.

---

## Testing Strategy

### Current (Phase 1)

- **24 tests** across all modules (23 unit + 1 doctest)
- Scanner: filter correctness, skip patterns, edge cases
- Parser: movie/TV/music parsing via hunch, confidence scoring
- Enricher: pass-through, review threshold
- Organizer: path building, execute + undo roundtrip
- Subtitles: companion discovery, case-insensitive FS dedup
- Utils: sanitize, format, safe path operations

### Planned (Phases 2+)

- **Provider tests** with `wiremock` for mocked HTTP
- **E2E tests** with `assert_cmd` + `predicates`
- **Regression suite** ported from Python's 560-test corpus

### Running tests

```bash
cargo test                     # All tests
cargo test scanner             # Single module
cargo test -- --nocapture      # With output
cargo test -- test_movie_path  # Single test
```

---

## File Size Inventory

| File | Lines | Status |
|---|---|---|
| `models.rs` | ~190 | ✅ Well under limit |
| `scanner.rs` | ~290 | ✅ |
| `parser.rs` | ~230 | ✅ |
| `organizer.rs` | ~380 | ✅ |
| `subtitles.rs` | ~155 | ✅ |
| `cli.rs` | ~240 | ✅ |
| `enricher.rs` | ~130 | ✅ |
| `config.rs` | ~80 | ✅ |
| `utils.rs` | ~100 | ✅ |
| `main.rs` | ~25 | ✅ |
| `lib.rs` | ~30 | ✅ |

All files well under the 600-line limit. `organizer.rs` is the largest;
if it grows during Phase 2, split into `organizer/paths.rs` and
`organizer/execute.rs`.

---

## Roadmap

### Phase 1: Foundation ✅

- Scanner with walkdir + filters
- hunch integration for video parsing
- Music regex placeholder
- Enricher (pass-through)
- Organizer (plan, execute, undo)
- Subtitle companion discovery
- CLI (scan, plan, organize, undo, config)
- TOML config
- 24 passing tests

### Phase 2: TMDb Enrichment 📋

- `providers/tmdb.rs` — reqwest client for TMDb API
- Movie search by title + year, fuzzy confidence scoring
- TV show search by title
- Direct ID lookup (for NFO phase)
- Rate limiting

### Phase 3: MusicBrainz + Music Parser 📋

- `providers/musicbrainz.rs` — reqwest client
- Proper music filename parser (replace regex placeholder)
- Artist/album/track extraction from directory structure

### Phase 4: Web Search Fallback 📋

- `providers/websearch.rs` — DuckDuckGo scoped search
- TMDb ID extraction from search result URLs
- MusicBrainz ID extraction
- Fallback for low-confidence cases

### Phase 5: NFO Sidecar Parsing 📋

- `enricher/nfo.rs` — quick-xml for Kodi XML NFO files
- TMDb/IMDB ID extraction from sidecar files
- Direct DB lookup (bypasses search, 95% confidence)

### Phase 6: Polish 📋

- Progress bars (`indicatif`)
- Colored output (`colored`)
- Shell completions (clap)
- `review` command for low-confidence items
- Disc folder handling (VIDEO_TS, BDMV)
