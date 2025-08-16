//! Core data types for the Plex Media Organizer

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Represents a media file in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    /// Unique identifier for the file
    pub id: String,
    /// Full path to the file
    pub file_path: PathBuf,
    /// Original filename
    pub file_name: String,
    /// File size in bytes
    pub file_size: u64,
    /// Type of media
    pub media_type: MediaType,
    /// Content hash for change detection
    pub content_hash: String,
    /// Last modification time
    pub last_modified: DateTime<Utc>,
    /// Extracted metadata
    pub metadata: MediaMetadata,
}

/// Type of media content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Movie,
    TvShow,
    Music,
    Subtitle,
    Unknown,
}

/// Metadata extracted from a media file
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaMetadata {
    /// Title of the media
    pub title: Option<String>,
    /// Original title (for foreign language content)
    pub original_title: Option<String>,
    /// Release year
    pub year: Option<u32>,
    /// Languages available
    pub language: Vec<String>,
    /// Video quality (720p, 1080p, 4K, etc.)
    pub quality: Option<String>,
    /// Source (BluRay, WEB-DL, HDTV, etc.)
    pub source: Option<String>,
    /// Duration in seconds
    pub duration: Option<u64>,
    /// Resolution (width x height)
    pub resolution: Option<Resolution>,
    /// Video codec
    pub codec: Option<String>,
    /// Audio tracks
    pub audio_tracks: Vec<AudioTrack>,
    /// Subtitle tracks
    pub subtitle_tracks: Vec<SubtitleTrack>,
}

/// Video resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

/// Audio track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub language: String,
    pub codec: Option<String>,
    pub channels: Option<u8>,
    pub bitrate: Option<u32>,
}

/// Subtitle track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub language: String,
    pub codec: Option<String>,
    pub forced: bool,
    pub sdh: bool, // Subtitles for the deaf and hard of hearing
}

/// Result of parsing a media file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingResult {
    /// The media file that was parsed
    pub media_file: MediaFile,
    /// Parsed metadata
    pub parsed_metadata: MediaMetadata,
    /// Confidence score (0.0 to 1.0)
    pub confidence_score: f32,
    /// Strategy used for parsing
    pub parsing_strategy: ParsingStrategy,
    /// External sources used
    pub external_sources: Vec<ExternalSource>,
    /// User corrections made
    pub user_corrections: Vec<UserCorrection>,
    /// When the result was created
    pub created_at: DateTime<Utc>,
    /// When the result was last updated
    pub updated_at: DateTime<Utc>,
}

/// Strategy used for parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParsingStrategy {
    /// Filename parsing only
    FilenameOnly,
    /// External API lookup (TMDB, etc.)
    ExternalApi,
    /// Pattern matching from learned examples
    PatternMatching,
    /// User manual correction
    UserCorrection,
    /// Combined multiple strategies
    Combined,
}

/// External source used for metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSource {
    /// Name of the source (TMDB, TVDB, etc.)
    pub name: String,
    /// ID in the external system
    pub external_id: String,
    /// URL to the source
    pub url: Option<String>,
    /// When the data was fetched
    pub fetched_at: DateTime<Utc>,
}

/// User correction to parsing results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCorrection {
    /// What was corrected
    pub field: String,
    /// Original value
    pub original_value: String,
    /// Corrected value
    pub corrected_value: String,
    /// When the correction was made
    pub corrected_at: DateTime<Utc>,
}

/// Movie-specific information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieInfo {
    /// Movie title
    pub title: String,
    /// Original title (for foreign language movies)
    pub original_title: Option<String>,
    /// Release year
    pub year: Option<u32>,
    /// Part number for multi-part movies
    pub part_number: Option<u32>,
    /// Whether this is part of a collection
    pub is_collection: bool,
    /// Collection name if applicable
    pub collection_name: Option<String>,
    /// Video quality
    pub quality: Option<String>,
    /// Source
    pub source: Option<String>,
    /// Languages available
    pub language: Option<String>,
}

/// TMDB movie data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbMovie {
    pub id: u32,
    pub title: String,
    pub original_title: Option<String>,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<u32>,
    pub popularity: Option<f32>,
}

/// Scan result for a directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Directory that was scanned
    pub directory: PathBuf,
    /// Files found
    pub files: Vec<MediaFile>,
    /// Files that were successfully parsed
    pub parsed_files: Vec<ParsingResult>,
    /// Files that failed to parse
    pub failed_files: Vec<FailedFile>,
    /// Scan statistics
    pub statistics: ScanStatistics,
    /// When the scan was performed
    pub scanned_at: DateTime<Utc>,
}

/// File that failed to parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedFile {
    /// The media file
    pub media_file: MediaFile,
    /// Error that occurred
    pub error: String,
    /// When the failure occurred
    pub failed_at: DateTime<Utc>,
}

/// Statistics from a scan operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatistics {
    /// Total files found
    pub total_files: u32,
    /// Files successfully parsed
    pub parsed_files: u32,
    /// Files that failed to parse
    pub failed_files: u32,
    /// Parsing success rate
    pub success_rate: f32,
    /// Average confidence score
    pub average_confidence: f32,
    /// Scan duration in seconds
    pub duration_seconds: f64,
}
