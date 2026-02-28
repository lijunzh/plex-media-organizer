# Contributing to Plex Media Organizer

Thanks for helping improve plex-media-organizer! 🎬

## Reporting Issues

### Files organized incorrectly

1. Go to [Issues → New Issue](https://github.com/lijunzh/plex-media-organizer/issues/new)
2. Include:
   - The source filename
   - What plex-org produced (run `plex-org scan /path/to/file`)
   - What you expected
   - Your OS and plex-org version (`plex-org --version`)

### Parsing issues

If the filename is parsed incorrectly (wrong title, year, season, etc.),
the issue likely belongs in [hunch](https://github.com/lijunzh/hunch/issues)
since that’s where filename parsing lives.

Quick check:

```bash
# See what hunch produces directly
hunch "Your.Filename.Here.mkv"

# See what plex-org does with it
plex-org scan /directory/containing/file
```

If `hunch` gets it wrong → file in [hunch issues](https://github.com/lijunzh/hunch/issues).  
If `hunch` is right but `plex-org` organizes wrong → file here.

## Development

### Setup

```bash
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build
cargo test
```

### Running

```bash
# Run directly
cargo run -- scan /path/to/media
cargo run -- plan /path/to/media -d /tmp/plex_test

# With debug output
cargo run -- -vvv scan /path/to/media
```

### Testing

```bash
cargo test                        # All tests
cargo test scanner                # Single module
cargo test -- --nocapture         # With output
cargo test -- test_movie_path     # Single test
```

### Code Style

- `cargo fmt` before committing
- `cargo clippy` with zero warnings
- Keep files under 600 lines — split into modules if needed
- Follow the existing module structure
- Tests in each module: `#[cfg(test)] mod tests` blocks

### Commit Messages

[Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add TMDb provider for movie enrichment
fix: handle cross-filesystem moves correctly
refactor: split organizer into paths and execute modules
test: add regression tests for anime filenames
docs: update architecture with Phase 2 design
chore: bump hunch to v1.1
```

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full design, decision log,
and module map. Key points:

- **hunch** handles all filename parsing — don't duplicate that logic
- **Core never prints** — return data from library functions; CLI formats output
- **Dry-run by default** — `--execute` for file operations
- **≤600 lines per file** — split by responsibility if growing beyond

## License

By contributing, you agree that your contributions will be licensed under
the [MIT License](LICENSE).
