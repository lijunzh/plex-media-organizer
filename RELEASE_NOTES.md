# Plex Media Organizer v0.2.0 — The hunch Rewrite

Complete rewrite powered by [hunch](https://crates.io/crates/hunch) v1.0
for filename parsing. Replaces the abandoned v0.1.x approach.

## What's New

- **hunch integration** — 49 typed properties from filenames via a single
  function call. Replaces ~900 lines of Python parsing infrastructure.
- **Full pipeline** — scan → parse → enrich → organize with dry-run,
  undo, and subtitle co-location.
- **Single static binary** — no Python, pip, or venv required.

## Pipeline

```text
Scan → Parse (hunch) → Enrich → Organize
```

- **Scan**: walkdir + extension/size/skip filters
- **Parse**: hunch for video, regex placeholder for music
- **Enrich**: pass-through stub (TMDb/MB planned for Phase 2+)
- **Organize**: Plex paths, move/copy/symlink, undo manifests

## Commands

```bash
plex-org scan /downloads                          # Discover files
plex-org plan /downloads -d /plex                  # Preview plan
plex-org organize /downloads -d /plex --execute    # Execute
plex-org undo                                      # Reverse
plex-org config                                    # Show config
```

## Output Structures

```
Movies/Movie Name (Year)/Movie Name (Year).ext
TV Shows/Show Name/Season XX/Show Name - SXXEXX.ext
Music/Artist/Album (Year)/01 - Track.ext
```

## Stats

- **24 tests** passing (23 unit + 1 doctest)
- **~1,600 lines** of Rust (55% fewer than the Python prototype)
- **11 source files**, all under 400 lines
- **Zero warnings** from cargo build + clippy

## Install

```bash
cargo install plex-media-organizer
```

## What's Next

| Phase | Description |
|---|---|
| 2 | TMDb provider + movie/TV enrichment |
| 3 | MusicBrainz + proper music parser |
| 4 | Web search fallback |
| 5 | NFO sidecar parsing |
| 6 | Progress bars, colors, shell completions |

## Full Changelog

See [CHANGELOG.md](CHANGELOG.md) for the complete history.
