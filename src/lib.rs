//! # Plex Media Organizer
//!
//! Organize torrent-downloaded media into Plex-compatible folder structures.
//!
//! Uses [hunch](https://crates.io/crates/hunch) for offline filename parsing,
//! with optional TMDb/MusicBrainz enrichment (future phases).
//!
//! ## Pipeline
//!
//! ```text
//! Scan → Parse (hunch) → Enrich → Organize
//! ```
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use plex_media_organizer::{scanner, parser};
//! use std::path::Path;
//!
//! let files = scanner::scan_directory(Path::new("/downloads"), &Default::default()).unwrap();
//! for f in &files {
//!     let parsed = parser::parse_media_file(f);
//!     println!("{}: {} ({:?})", f.full_name(), parsed.title, parsed.media_type);
//! }
//! ```

pub mod config;
pub mod enricher;
pub mod models;
pub mod organizer;
pub mod parser;
pub mod scanner;
pub mod subtitles;
pub mod utils;
