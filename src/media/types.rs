//! Media-specific types and data structures

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Audio track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub language: Option<String>,
    pub codec: Option<String>,
    pub channels: Option<u32>,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
}

/// Subtitle track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub language: Option<String>,
    pub codec: Option<String>,
    pub forced: bool,
    pub sdh: bool, // Subtitles for the deaf and hard of hearing
}

/// Video resolution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: Option<f32>,
}

/// Media quality information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityInfo {
    pub resolution: Option<Resolution>,
    pub bitrate: Option<u32>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub hdr: bool,
    pub color_space: Option<String>,
}

/// Media source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub source_type: SourceType,
    pub edition: Option<String>,
    pub region: Option<String>,
    pub group: Option<String>,
}

/// Type of media source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    BluRay,
    DVD,
    WebDL,
    HDTV,
    UHD,
    Unknown,
}

/// Media metadata extraction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub year: Option<u32>,
    pub language: Vec<String>,
    pub quality: Option<QualityInfo>,
    pub source: Option<SourceInfo>,
    pub duration: Option<Duration>,
    pub audio_tracks: Vec<AudioTrack>,
    pub subtitle_tracks: Vec<SubtitleTrack>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub container: Option<String>,
    pub bitrate: Option<u32>,
    pub file_size: Option<u64>,
}

impl Default for MediaMetadata {
    fn default() -> Self {
        Self {
            title: None,
            original_title: None,
            year: None,
            language: Vec::new(),
            quality: None,
            source: None,
            duration: None,
            audio_tracks: Vec::new(),
            subtitle_tracks: Vec::new(),
            video_codec: None,
            audio_codec: None,
            container: None,
            bitrate: None,
            file_size: None,
        }
    }
}

/// Media file analysis result
#[derive(Debug, Clone)]
pub struct MediaAnalysis {
    pub file_path: std::path::PathBuf,
    pub metadata: MediaMetadata,
    pub analysis_duration: Duration,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Media format detection result
#[derive(Debug, Clone)]
pub struct FormatDetection {
    pub container: String,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub subtitle_codec: Option<String>,
    pub confidence: f32,
}