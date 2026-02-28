//! Core data models for plex-media-organizer.

use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// ── Media Type ─────────────────────────────────────────────────────────────

/// Detected media type for a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Tv,
    Music,
    #[default]
    Unknown,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Movie => write!(f, "movie"),
            Self::Tv => write!(f, "tv"),
            Self::Music => write!(f, "music"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

// ── Discovered File ────────────────────────────────────────────────────────

/// A media file discovered on disk by the scanner.
#[derive(Debug, Clone)]
pub struct MediaFile {
    pub source_path: PathBuf,
    /// Filename stem (no extension).
    pub filename: String,
    /// Extension including dot, lowercase (e.g., ".mkv").
    pub extension: String,
    /// Initial type guess from extension.
    pub detected_type: MediaType,
    pub size_bytes: u64,
    /// Immediate parent directory name.
    pub parent_dir: String,
}

impl MediaFile {
    /// Full filename with extension.
    pub fn full_name(&self) -> String {
        format!("{}{}", self.filename, self.extension)
    }
}

// ── Parsed Metadata ────────────────────────────────────────────────────────

/// Metadata extracted from a filename via hunch or music regex.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ParsedMedia {
    pub title: String,
    pub year: Option<i32>,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub episode_end: Option<i32>,
    pub episode_title: Option<String>,
    pub media_type: MediaType,
    pub release_group: Option<String>,
    pub quality: String,
    pub source_tag: Option<String>,
    pub language: Option<String>,
    pub confidence: f64,
    pub raw_filename: String,
    // Music-specific (placeholder regex)
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<u32>,
    pub track_title: Option<String>,
}

// ── Enriched Metadata ──────────────────────────────────────────────────────

/// Enriched movie metadata (from DB lookup — future phase).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: Option<i32>,
    pub tmdb_id: Option<u64>,
    pub original_title: Option<String>,
    pub confidence: f64,
}

/// Enriched TV episode metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TvEpisode {
    pub show_title: String,
    pub season: i32,
    pub episode: i32,
    pub episode_end: Option<i32>,
    pub episode_title: Option<String>,
    pub year: Option<i32>,
    pub tmdb_id: Option<u64>,
    pub confidence: f64,
}

/// Enriched music track metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicTrack {
    pub artist: String,
    pub album: Option<String>,
    pub track_title: Option<String>,
    pub track_number: Option<u32>,
    pub year: Option<i32>,
    pub confidence: f64,
}

/// Result of the enrichment pipeline.
#[derive(Debug, Clone)]
pub struct EnrichedMedia {
    pub parsed: ParsedMedia,
    pub movie: Option<Movie>,
    pub tv_episode: Option<TvEpisode>,
    pub music_track: Option<MusicTrack>,
    pub media_type: MediaType,
    pub confidence: f64,
    pub needs_review: bool,
    pub enrichment_source: Option<String>,
}

impl EnrichedMedia {
    /// Build from parsed metadata with no enrichment (pass-through).
    pub fn from_parsed(parsed: ParsedMedia) -> Self {
        let media_type = parsed.media_type;
        let confidence = parsed.confidence;
        Self {
            parsed,
            movie: None,
            tv_episode: None,
            music_track: None,
            media_type,
            confidence,
            needs_review: false,
            enrichment_source: None,
        }
    }

    /// Best available title for display.
    pub fn best_title(&self) -> &str {
        if let Some(m) = &self.movie {
            return &m.title;
        }
        if let Some(tv) = &self.tv_episode {
            return &tv.show_title;
        }
        if let Some(music) = &self.music_track {
            return &music.artist;
        }
        &self.parsed.title
    }
}

// ── Organize Action ────────────────────────────────────────────────────────

/// A planned file operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizeAction {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub strategy: String,
    pub media_type: MediaType,
    pub title: String,
    pub confidence: f64,
}

// ── Undo ───────────────────────────────────────────────────────────────────

/// A single reversible file operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoEntry {
    pub source: String,
    pub destination: String,
    pub strategy: String,
    pub timestamp: String,
    pub title: String,
    pub media_type: String,
}

/// Collection of undo entries for a single organize run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoManifest {
    pub version: u32,
    pub entries: Vec<UndoEntry>,
    pub created_at: String,
    pub description: String,
}

impl Default for UndoManifest {
    fn default() -> Self {
        Self {
            version: 1,
            entries: Vec::new(),
            created_at: String::new(),
            description: String::new(),
        }
    }
}
