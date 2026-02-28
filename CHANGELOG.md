# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2026-02-28

### Added

- **Complete rewrite** using [hunch](https://crates.io/crates/hunch) v1.0
  as the filename parsing engine, replacing the failed v0.1.x approach of
  custom regex patterns.
- **Scanner** (`scanner.rs`) — recursive directory walker using `walkdir`,
  with extension filtering (14 video + 13 audio + 6 subtitle extensions),
  50 MB minimum video size filter, sample/extras/hidden file skip patterns.
- **Parser** (`parser.rs`) — hunch integration for video files (49 typed
  properties in a single function call), plus simple regex placeholders
  for music files (`NN - Track Title` + `Artist - Album (Year)` directory).
- **Enricher** (`enricher.rs`) — pass-through stub that wraps parsed data
  into enriched models (Movie, TvEpisode, MusicTrack). Database providers
  planned for Phase 2+.
- **Organizer** (`organizer.rs`) — Plex-compatible path builder for movies,
  TV shows, and music. Supports move/copy/symlink strategies. Duplicate
  detection with counter suffix. Cross-filesystem move fallback (copy+delete).
- **Undo system** — JSON manifests written per organize run, reversible via
  `plex-org undo`. Empty parent cleanup (3 levels deep).
- **Subtitle companion discovery** (`subtitles.rs`) — finds `.srt`, `.ass`,
  `.sub`, `.vtt` files adjacent to video files and in `Subs/` subdirectories.
  Preserves language/forced suffixes (`.en`, `.zh.forced`). Case-insensitive
  filesystem dedup on macOS.
- **CLI** (`cli.rs`) — `scan`, `plan`, `organize`, `undo`, `config` commands
  via clap derive macros. Dry-run by default; `--execute` required.
- **Configuration** (`config.rs`) — TOML config with serde deserialization.
  Supports auto-organize and review confidence thresholds.
- **Utilities** (`utils.rs`) — `sanitize_name` (unsafe char removal, space
  collapse, length cap), `format_size`, `safe_path_join` (traversal protection).
- **24 passing tests** across all modules (23 unit + 1 doctest).
- Full documentation: README.md, ARCHITECTURE.md, CONTRIBUTING.md.

### Changed

- **Version bumped to 0.2.0** — fresh start with hunch, replacing the
  v0.1.x approach that used custom regex patterns with limited accuracy.

### Architecture

- Pipeline: `Scan → Parse (hunch) → Enrich → Organize`
- ~1,600 lines of Rust (55% fewer than the Python prototype's 3,600)
- hunch replaces ~900 lines of Python (normalizer, classifier, parser,
  rule engine, context parser)
- Single static binary with no runtime dependencies

## [0.1.1] - 2026-01 (abandoned)

### Summary

Second attempt at a Rust media organizer using custom regex patterns
for filename parsing. Limited accuracy on real-world filenames — could
not handle anime, foreign languages, multi-episode ranges, or release
group disambiguation.

## [0.1.0] - 2025-12 (abandoned)

### Summary

First attempt at a Rust media organizer. Basic scanning and regex-based
parsing. Abandoned due to the complexity of building a complete filename
parser from scratch.

[0.2.0]: https://github.com/lijunzh/plex-media-organizer/releases/tag/v0.2.0
[0.1.1]: https://github.com/lijunzh/plex-media-organizer/releases/tag/v0.1.1
[0.1.0]: https://github.com/lijunzh/plex-media-organizer/releases/tag/v0.1.0
