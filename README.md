# 🎥 Plex Media Organizer

A CLI tool that organizes torrent-downloaded movies, TV shows, and music into
[Plex-compatible folder structures](https://support.plex.tv/articles/naming-and-organizing-your-movie-media-files/).
Powered by [hunch](https://crates.io/crates/hunch) for fast, offline filename parsing.

## Pipeline

```text
Scan → Parse (hunch) → Enrich → Organize
```

- **Scan** — walk directories, filter by extension, skip samples/extras
- **Parse** — extract title, year, season, episode, codec via hunch (video) or regex (music)
- **Enrich** — validate against TMDb/MusicBrainz (future phase, currently pass-through)
- **Organize** — move/copy/symlink into Plex folder structures with undo support

## Install

```bash
cargo install plex-media-organizer
```

## Quick Start

```bash
plex-org scan /downloads                          # See what's detected
plex-org plan /downloads -d /plex                  # Preview the plan
plex-org organize /downloads -d /plex --execute    # Do it
plex-org undo                                      # Oops? Reverse it
```

## Output Structures

```
Movies/Movie Name (Year)/Movie Name (Year).ext
TV Shows/Show Name/Season XX/Show Name - SXXEXX - Episode Title.ext
TV Shows/Show Name/Season XX/Show Name - SXXEXX.en.srt
Music/Artist/Album (Year)/01 - Track.ext
```

Subtitle files (`.srt`, `.ass`, `.sub`, `.vtt`) are automatically discovered
next to video files (including `Subs/` subdirectories) and co-located alongside
them with matching names. Language suffixes like `.en`, `.zh.forced` are preserved.

## Commands

| Command | Description |
|---------|-------------|
| `scan <path>` | Discover media files and show parsed metadata |
| `plan <path> -d <dest>` | Preview organization plan (dry-run) |
| `organize <path> -d <dest> --execute` | Execute file organization |
| `undo` | Reverse the last organize operation |
| `config` | Show current configuration |

### Options

```
-v, --verbose    Increase verbosity (-v, -vv, -vvv)
-c, --config     Config file path
-s, --strategy   move (default), copy, or symlink
```

## Configuration

```toml
source_dirs = []
destination = ""
auto_organize_threshold = 90.0
review_threshold = 50.0

[organize]
strategy = "move"
movies_dir = "Movies"
tv_dir = "TV Shows"
music_dir = "Music"
```

## Architecture

This project uses [hunch](https://crates.io/crates/hunch) as its parsing engine
— a Rust rewrite of Python's guessit that extracts 49 typed properties from
media filenames in microseconds.

```
src/
├── lib.rs          # Library root
├── main.rs         # CLI entry point
├── cli.rs          # Command dispatch (clap)
├── models.rs       # Data types: MediaFile, ParsedMedia, EnrichedMedia, etc.
├── config.rs       # TOML config with serde
├── scanner.rs      # Directory walker + extension/skip filters
├── parser.rs       # hunch integration (video) + regex (music placeholder)
├── enricher.rs     # DB enrichment (pass-through in v0.2, TMDb/MB planned)
├── organizer.rs    # Path builder, execute, undo
├── subtitles.rs    # Subtitle companion discovery
└── utils.rs        # Filesystem helpers, sanitize_name
```

## Development

```bash
cargo test       # Run all tests (23 tests)
cargo build      # Build debug binary
cargo run -- scan /path/to/media  # Run directly
```

## Roadmap

- [x] Phase 1: Scanner + hunch integration + organize/undo
- [ ] Phase 2: TMDb provider + movie/TV enrichment
- [ ] Phase 3: MusicBrainz + proper music parser
- [ ] Phase 4: Web search fallback (DuckDuckGo → TMDb/MB ID resolution)
- [ ] Phase 5: NFO sidecar parsing
- [ ] Phase 6: Progress bars, Rich-style output, shell completions

## License

MIT
