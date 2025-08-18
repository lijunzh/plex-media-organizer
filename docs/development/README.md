# 👨‍💻 Developer Documentation

Documentation for developers who want to understand, modify, or contribute to Plex Media Organizer.

## 🎯 Quick Developer Start

### "I want to understand the code structure quickly"
1. **[Code Architecture](code-architecture.md)** - High-level system overview (5-minute read)
2. **[API Reference](api-reference.md)** - Library API and key functions
3. **[Adding Features](adding-features.md)** - How to extend the system

### "I want to make a specific change"
1. **Find the right component** in [Code Architecture](code-architecture.md)
2. **Check the API** in [API Reference](api-reference.md)
3. **Follow the patterns** in [Adding Features](adding-features.md)
4. **Test your changes** with our test suite

## 🏗️ Architecture Overview

### System Components
```
┌─────────────────────────────────────────┐
│ CLI Interface (src/cli.rs)              │ ← User commands
├─────────────────────────────────────────┤
│ Core Logic                              │
│ ├── Scanner (src/scanner.rs)           │ ← Find media files
│ ├── Parser (src/movie_parser.rs)       │ ← Extract metadata
│ ├── TMDB Client (src/tmdb_client.rs)   │ ← Get movie data
│ └── Organizer (src/organizer.rs)       │ ← Rename files
├─────────────────────────────────────────┤
│ Data Types (src/types.rs)              │ ← Core data structures
├─────────────────────────────────────────┤
│ Configuration (src/config.rs)          │ ← Settings management
└─────────────────────────────────────────┘
```

### Data Flow
```
Directory → Scanner → Parser → TMDB Client → Organizer → Results
    ↓         ↓         ↓          ↓            ↓         ↓
   Files   MediaFile MovieInfo  TmdbMovie  Organized  Report
```

## 📋 Quick Reference

### "I want to add..."

| Feature | Primary File | Supporting Files |
|---------|-------------|------------------|
| New media type | `src/types.rs` | `src/scanner.rs`, `src/cli.rs` |
| Parsing pattern | `src/movie_parser.rs` | Test files |
| Organization rule | `src/organizer.rs` | `src/types.rs` |
| CLI command | `src/cli.rs` | All modules |
| Configuration option | `src/config.rs` | `src/types.rs` |
| API integration | `src/tmdb_client.rs` | `src/types.rs` |

### "I want to understand..."

| Topic | File | Description |
|-------|------|-------------|
| How files are found | `src/scanner.rs` | Directory walking and file filtering |
| How titles are parsed | `src/movie_parser.rs` | Regex patterns and title extraction |
| How TMDB works | `src/tmdb_client.rs` | API integration and fuzzy matching |
| How files are organized | `src/organizer.rs` | Plex naming and file operations |
| What data we store | `src/types.rs` | All data structures |
| How config works | `src/config.rs` | Settings and defaults |

## 🔧 Development Setup

### Quick Development Environment

```bash
# Clone and setup
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer

# Install development tools
rustup component add rustfmt clippy
cargo install cargo-audit cargo-outdated

# Build and test
cargo build
cargo test

# Check code quality
cargo fmt --check
cargo clippy
```

### Development Configuration

```toml
# dev-config.toml
[apis]
tmdb_api_key = "your_dev_key"

[logging]
level = "debug"
log_api_calls = true

[processing]
max_parallel_files = 4  # Easier debugging
enable_cache = false    # Fresh data every time
```

## 🧪 Testing

### Run Tests
```bash
# All tests
cargo test

# Specific module
cargo test movie_parser

# With debug output
cargo test -- --nocapture

# Real-world tests (requires test data)
cargo test test_movie_directory_dynamic
```

### Test Structure
```
tests/
├── dynamic_real_world_test.rs  # Tests with real file data
├── real_world_patterns_test.rs # Pattern validation
└── test_data/                  # Real directory structures
    └── movie_directory.txt     # 417 real movie filenames
```

## 📚 Key Concepts

### Core Data Types

```rust
// Main data structures (src/types.rs)
pub struct MediaFile {
    pub file_path: PathBuf,
    pub file_name: String,
    pub media_type: MediaType,
    // ...
}

pub struct MovieInfo {
    pub title: String,
    pub year: Option<u32>,
    pub quality: Option<String>,
    // ...
}

pub struct ParsingResult {
    pub movie_info: MovieInfo,
    pub confidence: f32,
    pub tmdb_match: Option<TmdbMovie>,
    // ...
}
```

### Processing Pipeline

```rust
// Typical workflow
let files = scanner.scan_directory(path)?;           // Find files
let results = parser.parse_files(files)?;            // Extract metadata
let enriched = tmdb_client.enrich_results(results)?; // Get TMDB data
let organized = organizer.organize_files(enriched)?; // Rename files
```

### Error Handling

```rust
// We use anyhow for error handling
use anyhow::{Result, Context};

fn parse_file(path: &Path) -> Result<MovieInfo> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    
    // ... parsing logic
    
    Ok(movie_info)
}
```

## 🎯 Common Development Tasks

### Adding a New Parsing Pattern

1. **Add test case** in `src/movie_parser.rs`:
   ```rust
   #[test]
   fn test_my_new_pattern() {
       let result = parse_filename("My.New.Pattern.2023.1080p.mkv");
       assert_eq!(result.title, "My New Pattern");
       assert_eq!(result.year, Some(2023));
   }
   ```

2. **Add regex pattern** to `parse_filename()`:
   ```rust
   // Add to existing patterns
   if let Some(caps) = PATTERN_REGEX.captures(&clean_filename) {
       // Extract title, year, etc.
   }
   ```

3. **Test and validate**:
   ```bash
   cargo test test_my_new_pattern
   ```

### Adding a Configuration Option

1. **Add to config struct** in `src/config.rs`:
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct Config {
       // ... existing fields
       pub my_new_option: bool,
   }
   ```

2. **Add default value**:
   ```rust
   impl Default for Config {
       fn default() -> Self {
           Self {
               // ... existing defaults
               my_new_option: false,
           }
       }
   }
   ```

3. **Use in code**:
   ```rust
   if config.my_new_option {
       // New behavior
   }
   ```

### Adding a CLI Command

1. **Add to CLI enum** in `src/cli.rs`:
   ```rust
   #[derive(Subcommand)]
   pub enum Commands {
       // ... existing commands
       MyNewCommand {
           #[arg(short, long)]
           my_option: bool,
       },
   }
   ```

2. **Handle in main**:
   ```rust
   match cli.command {
       // ... existing matches
       Commands::MyNewCommand { my_option } => {
           handle_my_command(my_option).await?;
       }
   }
   ```

## 🔍 Debugging Guide

### Debug Specific Files
```bash
# Test single file with debug output
RUST_LOG=debug cargo run -- test "problematic-file.mkv"

# Debug TMDB API calls
RUST_LOG=plex_media_organizer::tmdb_client=debug cargo run -- test "file.mkv"
```

### Debug Parsing Issues
```rust
// Add debug prints in movie_parser.rs
eprintln!("Parsing: {}", filename);
eprintln!("Regex captures: {:?}", captures);
eprintln!("Extracted title: {}", title);
```

### Debug Configuration
```bash
# Check current configuration
cargo run -- config

# Validate configuration
cargo run -- config --validate
```

## 📖 Code Style and Standards

### Rust Conventions
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Follow standard Rust naming conventions
- Add documentation for public APIs

### Error Handling
```rust
// Good: Use anyhow for application errors
use anyhow::{Result, Context};

fn my_function() -> Result<String> {
    let data = read_file()
        .with_context(|| "Failed to read input file")?;
    Ok(data)
}

// Good: Use specific error types for library APIs
#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    #[error("Invalid filename format")]
    InvalidFormat,
    #[error("Missing required field: {field}")]
    MissingField { field: String },
}
```

### Documentation
```rust
/// Parse a movie filename to extract metadata.
/// 
/// # Arguments
/// 
/// * `filename` - The filename to parse
/// 
/// # Returns
/// 
/// Returns a `ParsingResult` with extracted metadata and confidence score.
/// 
/// # Examples
/// 
/// ```rust
/// let result = parse_filename("The Matrix (1999) 1080p BluRay.mkv");
/// assert_eq!(result.title, "The Matrix");
/// assert_eq!(result.year, Some(1999));
/// ```
pub fn parse_filename(filename: &str) -> Result<ParsingResult> {
    // Implementation
}
```

## 🤝 Contributing

### Development Workflow
1. **Fork and clone** the repository
2. **Create feature branch**: `git checkout -b feature/my-feature`
3. **Make changes** and add tests
4. **Run full test suite**: `cargo test`
5. **Check code quality**: `cargo fmt && cargo clippy`
6. **Submit pull request** with clear description

### Pull Request Guidelines
- **Clear title** describing the change
- **Description** explaining why and what changed
- **Tests** for new functionality
- **Documentation** updates if needed
- **No breaking changes** without discussion

### Getting Help
- **Code questions**: [GitHub Discussions](https://github.com/lijunzh/plex-media-organizer/discussions)
- **Bug reports**: [GitHub Issues](https://github.com/lijunzh/plex-media-organizer/issues)
- **Architecture questions**: See [project architecture](../../project/architecture.md)

---

## 📚 Next Steps

- **[Code Architecture](code-architecture.md)** - Detailed system design
- **[API Reference](api-reference.md)** - Complete API documentation  
- **[Adding Features](adding-features.md)** - Step-by-step feature development

**💡 Pro Tip**: Start by reading real code in `src/` and the existing tests. The code is well-documented and follows clear patterns!
