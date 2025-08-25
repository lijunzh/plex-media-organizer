//! Media-specific types and structures

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Basic movie information extracted from files
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MovieInfo {
    pub title: String,
    pub year: Option<u32>,
    pub original_title: Option<String>,
    pub original_language: Option<String>,
    pub part_number: Option<u32>,
    pub is_collection: bool,
    pub collection_name: Option<String>,
    pub is_series: bool,
    pub series_name: Option<String>,
    pub series_number: Option<u32>,
    pub is_anime: bool,
    pub anime_movie_number: Option<u32>,
    pub has_japanese_title: bool,
    pub has_chinese_title: bool,
    pub quality: Option<String>,
    pub source: Option<String>,
    pub language: Option<String>,
}

/// Media file metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaMetadata {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub year: Option<u32>,
    pub language: Vec<String>,
    pub quality: Option<String>,
    pub source: Option<String>,
    pub duration: Option<Duration>,
    pub resolution: Option<Resolution>,
    pub codec: Option<String>,
    pub audio_tracks: Vec<AudioTrack>,
    pub subtitle_tracks: Vec<SubtitleTrack>,
}

/// Video resolution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

/// Audio track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub language: String,
    pub codec: String,
    pub channels: u8,
    pub bitrate: Option<u32>,
}

/// Subtitle track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub language: String,
    pub codec: String,
    pub forced: bool,
    pub sdh: bool, // Subtitles for the deaf and hard of hearing
}