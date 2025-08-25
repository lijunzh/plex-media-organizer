//! Media metadata extraction functionality
//!
//! This module handles the extraction of metadata from media files including
//! video, audio, and subtitle information.

use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use super::types::{MediaAnalysis, MediaMetadata, FormatDetection};

/// Media metadata extractor
#[derive(Debug)]
pub struct MetadataExtractor {
    /// Whether to extract detailed metadata (slower but more comprehensive)
    detailed_extraction: bool,
    /// Maximum file size to analyze (in bytes)
    max_file_size: Option<u64>,
    /// Whether to extract embedded subtitles
    extract_subtitles: bool,
}

impl MetadataExtractor {
    /// Create a new metadata extractor
    pub fn new() -> Self {
        Self {
            detailed_extraction: false,
            max_file_size: None,
            extract_subtitles: true,
        }
    }

    /// Create a new metadata extractor with detailed extraction enabled
    pub fn with_detailed_extraction() -> Self {
        Self {
            detailed_extraction: true,
            max_file_size: None,
            extract_subtitles: true,
        }
    }

    /// Create a new metadata extractor with custom settings
    pub fn with_settings(
        detailed_extraction: bool,
        max_file_size: Option<u64>,
        extract_subtitles: bool,
    ) -> Self {
        Self {
            detailed_extraction,
            max_file_size,
            extract_subtitles,
        }
    }

    /// Extract metadata from a media file
    pub async fn extract_metadata<P: AsRef<Path>>(&self, file_path: P) -> Result<MediaAnalysis> {
        let file_path = file_path.as_ref();
        let start_time = Instant::now();

        // Check file size limit
        if let Some(max_size) = self.max_file_size {
            let metadata = tokio::fs::metadata(file_path).await?;
            if metadata.len() > max_size {
                return Ok(MediaAnalysis {
                    file_path: file_path.to_path_buf(),
                    metadata: MediaMetadata::default(),
                    analysis_duration: start_time.elapsed(),
                    success: false,
                    error_message: Some(format!("File size {} exceeds limit {}", metadata.len(), max_size)),
                });
            }
        }

        // Extract basic metadata
        let metadata = match self.extract_basic_metadata(file_path).await {
            Ok(metadata) => metadata,
            Err(e) => {
                return Ok(MediaAnalysis {
                    file_path: file_path.to_path_buf(),
                    metadata: MediaMetadata::default(),
                    analysis_duration: start_time.elapsed(),
                    success: false,
                    error_message: Some(e.to_string()),
                });
            }
        };

        // Extract detailed metadata if enabled
        let final_metadata = if self.detailed_extraction {
            self.extract_detailed_metadata(file_path, metadata).await?
        } else {
            metadata
        };

        Ok(MediaAnalysis {
            file_path: file_path.to_path_buf(),
            metadata: final_metadata,
            analysis_duration: start_time.elapsed(),
            success: true,
            error_message: None,
        })
    }

    /// Extract basic metadata from a file
    async fn extract_basic_metadata(&self, file_path: &Path) -> Result<MediaMetadata> {
        let metadata = tokio::fs::metadata(file_path).await?;
        
        // Detect format
        let format = self.detect_format(file_path)?;
        
        let mut media_metadata = MediaMetadata::default();
        media_metadata.file_size = Some(metadata.len());
        media_metadata.container = Some(format.container);
        media_metadata.video_codec = format.video_codec;
        media_metadata.audio_codec = format.audio_codec;

        Ok(media_metadata)
    }

    /// Extract detailed metadata from a file
    async fn extract_detailed_metadata(
        &self,
        file_path: &Path,
        mut metadata: MediaMetadata,
    ) -> Result<MediaMetadata> {
        // In a full implementation, this would use libraries like ffprobe
        // to extract detailed video/audio information
        
        // For now, we'll just enhance the basic metadata with filename analysis
        if let Some(filename) = file_path.file_name().and_then(|n| n.to_str()) {
            self.enhance_metadata_from_filename(filename, &mut metadata);
        }

        Ok(metadata)
    }

    /// Detect media format from file
    fn detect_format(&self, file_path: &Path) -> Result<FormatDetection> {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let ext_lower = ext_str.to_lowercase();
                
                return match ext_lower.as_str() {
                    "mkv" => Ok(FormatDetection {
                        container: "Matroska".to_string(),
                        video_codec: None, // Would be detected from file content
                        audio_codec: None,  // Would be detected from file content
                        subtitle_codec: None,
                        confidence: 1.0,
                    }),
                    "mp4" => Ok(FormatDetection {
                        container: "MP4".to_string(),
                        video_codec: None,
                        audio_codec: None,
                        subtitle_codec: None,
                        confidence: 1.0,
                    }),
                    "avi" => Ok(FormatDetection {
                        container: "AVI".to_string(),
                        video_codec: None,
                        audio_codec: None,
                        subtitle_codec: None,
                        confidence: 1.0,
                    }),
                    "mov" => Ok(FormatDetection {
                        container: "QuickTime".to_string(),
                        video_codec: None,
                        audio_codec: None,
                        subtitle_codec: None,
                        confidence: 1.0,
                    }),
                    "flac" => Ok(FormatDetection {
                        container: "FLAC".to_string(),
                        video_codec: None,
                        audio_codec: Some("FLAC".to_string()),
                        subtitle_codec: None,
                        confidence: 1.0,
                    }),
                    "mp3" => Ok(FormatDetection {
                        container: "MP3".to_string(),
                        video_codec: None,
                        audio_codec: Some("MP3".to_string()),
                        subtitle_codec: None,
                        confidence: 1.0,
                    }),
                    _ => Ok(FormatDetection {
                        container: ext_str.to_uppercase(),
                        video_codec: None,
                        audio_codec: None,
                        subtitle_codec: None,
                        confidence: 0.5,
                    }),
                };
            }
        }

        Ok(FormatDetection {
            container: "Unknown".to_string(),
            video_codec: None,
            audio_codec: None,
            subtitle_codec: None,
            confidence: 0.0,
        })
    }

    /// Enhance metadata by analyzing filename patterns
    fn enhance_metadata_from_filename(&self, filename: &str, metadata: &mut MediaMetadata) {
        let filename_lower = filename.to_lowercase();
        
        // Detect quality indicators
        if filename_lower.contains("1080p") || filename_lower.contains("1920x1080") {
            metadata.quality = Some(super::types::QualityInfo {
                resolution: Some(super::types::Resolution {
                    width: 1920,
                    height: 1080,
                    aspect_ratio: Some(16.0 / 9.0),
                }),
                bitrate: None,
                codec: None,
                container: None,
                hdr: false,
                color_space: None,
            });
        } else if filename_lower.contains("720p") || filename_lower.contains("1280x720") {
            metadata.quality = Some(super::types::QualityInfo {
                resolution: Some(super::types::Resolution {
                    width: 1280,
                    height: 720,
                    aspect_ratio: Some(16.0 / 9.0),
                }),
                bitrate: None,
                codec: None,
                container: None,
                hdr: false,
                color_space: None,
            });
        } else if filename_lower.contains("4k") || filename_lower.contains("2160p") {
            metadata.quality = Some(super::types::QualityInfo {
                resolution: Some(super::types::Resolution {
                    width: 3840,
                    height: 2160,
                    aspect_ratio: Some(16.0 / 9.0),
                }),
                bitrate: None,
                codec: None,
                container: None,
                hdr: filename_lower.contains("hdr"),
                color_space: None,
            });
        }

        // Detect source type
        if filename_lower.contains("bluray") || filename_lower.contains("blu-ray") {
            metadata.source = Some(super::types::SourceInfo {
                source_type: super::types::SourceType::BluRay,
                edition: None,
                region: None,
                group: None,
            });
        } else if filename_lower.contains("webdl") || filename_lower.contains("web-dl") {
            metadata.source = Some(super::types::SourceInfo {
                source_type: super::types::SourceType::WebDL,
                edition: None,
                region: None,
                group: None,
            });
        } else if filename_lower.contains("hdtv") {
            metadata.source = Some(super::types::SourceInfo {
                source_type: super::types::SourceType::HDTV,
                edition: None,
                region: None,
                group: None,
            });
        }

        // Detect year
        if let Some(year) = self.extract_year_from_filename(filename) {
            metadata.year = Some(year);
        }
    }

    /// Extract year from filename
    fn extract_year_from_filename(&self, filename: &str) -> Option<u32> {
        // Simple regex-like pattern matching for years
        let words: Vec<&str> = filename.split(|c: char| !c.is_alphanumeric()).collect();
        
        for word in words {
            if word.len() == 4 {
                if let Ok(year) = word.parse::<u32>() {
                    if year >= 1900 && year <= 2030 {
                        return Some(year);
                    }
                }
            }
        }
        
        None
    }

    /// Set detailed extraction mode
    pub fn set_detailed_extraction(&mut self, enabled: bool) {
        self.detailed_extraction = enabled;
    }

    /// Set maximum file size for analysis
    pub fn set_max_file_size(&mut self, max_size: Option<u64>) {
        self.max_file_size = max_size;
    }

    /// Set subtitle extraction mode
    pub fn set_extract_subtitles(&mut self, enabled: bool) {
        self.extract_subtitles = enabled;
    }
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        Self::new()
    }
}