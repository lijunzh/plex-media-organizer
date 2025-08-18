//! Directory scanning and file discovery

use crate::movie_parser::MovieParser;
use crate::types::{FailedFile, MediaFile, MediaType, ScanResult, ScanStatistics};
use anyhow::{Context, Result};
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

/// Media file scanner
#[derive(Debug)]
pub struct Scanner {
    movie_parser: MovieParser,
}

impl Scanner {
    /// Create a new scanner
    pub fn new(movie_parser: MovieParser) -> Self {
        Self { movie_parser }
    }

    /// Scan a directory for media files
    pub async fn scan_directory(&self, directory: &Path) -> Result<ScanResult> {
        let start_time = Instant::now();

        println!("Scanning directory: {}", directory.display());

        // Discover all files
        let files = self.discover_files(directory)?;
        println!("Found {} files", files.len());

        // Filter media files
        let media_files = self.filter_media_files(&files)?;
        println!("Found {} media files", media_files.len());

        // Parse media files
        let (parsed_files, failed_files) = self.parse_media_files(&media_files).await?;
        println!("Successfully parsed {} files", parsed_files.len());
        if !failed_files.is_empty() {
            println!("Failed to parse {} files", failed_files.len());
        }

        // Calculate statistics
        let statistics =
            self.calculate_statistics(&media_files, &parsed_files, &failed_files, start_time);

        let result = ScanResult {
            directory: directory.to_path_buf(),
            files: media_files,
            parsed_files,
            failed_files,
            statistics,
            scanned_at: Utc::now(),
        };

        Ok(result)
    }

    /// Discover all files in a directory
    fn discover_files(&self, directory: &Path) -> Result<Vec<PathBuf>> {
        if !directory.exists() {
            anyhow::bail!("Directory does not exist: {}", directory.display());
        }

        if !directory.is_dir() {
            anyhow::bail!("Path is not a directory: {}", directory.display());
        }

        let mut files = Vec::new();

        for entry in WalkDir::new(directory)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }

        Ok(files)
    }

    /// Filter files to only include media files
    fn filter_media_files(&self, files: &[PathBuf]) -> Result<Vec<MediaFile>> {
        let mut media_files = Vec::new();

        for file_path in files {
            if let Some(media_file) = self.create_basic_media_file(file_path)? {
                media_files.push(media_file);
            }
        }

        Ok(media_files)
    }

    /// Create a basic MediaFile from a path
    fn create_basic_media_file(&self, file_path: &Path) -> Result<Option<MediaFile>> {
        // Check if it's a media file by extension
        if let Some(extension) = file_path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();

            if self.is_media_extension(&ext) {
                let metadata =
                    std::fs::metadata(file_path).context("Failed to get file metadata")?;

                let file_name = file_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let media_type = self.detect_media_type(file_path, &ext)?;

                let media_file = MediaFile {
                    id: format!("file_{}", uuid::Uuid::new_v4()),
                    file_path: file_path.to_path_buf(),
                    file_name,
                    file_size: metadata.len(),
                    media_type,
                    content_hash: "".to_string(), // Will be calculated during parsing
                    last_modified: metadata
                        .modified()
                        .map(chrono::DateTime::from)
                        .unwrap_or_else(|_| Utc::now()),
                    metadata: crate::types::MediaMetadata::default(),
                };

                return Ok(Some(media_file));
            }
        }

        Ok(None)
    }

    /// Check if a file extension indicates a media file
    fn is_media_extension(&self, extension: &str) -> bool {
        match extension {
            // Video formats
            "mkv" | "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" => true,
            // Audio formats
            "mp3" | "flac" | "wav" | "m4a" | "aac" | "ogg" | "wma" => true,
            // Subtitle formats
            "srt" | "ass" | "ssa" | "sub" | "vtt" => true,
            _ => false,
        }
    }

    /// Detect media type from file path and extension
    fn detect_media_type(&self, file_path: &Path, extension: &str) -> Result<MediaType> {
        // Check if it's a subtitle file
        if matches!(extension, "srt" | "ass" | "ssa" | "sub" | "vtt") {
            return Ok(MediaType::Subtitle);
        }

        // Check if it's an audio file
        if matches!(
            extension,
            "mp3" | "flac" | "wav" | "m4a" | "aac" | "ogg" | "wma"
        ) {
            return Ok(MediaType::Music);
        }

        // For video files, try to determine if it's a movie or TV show
        // This is a simple heuristic based on directory structure
        if let Some(parent) = file_path.parent() {
            let parent_name = parent
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_lowercase();

            // Check for TV show indicators (more specific to avoid false positives)
            if parent_name.contains("season")
                || parent_name.contains("episode")
                || (parent_name.contains("s")
                    && parent_name.contains("e")
                    && (parent_name.contains("season") || parent_name.contains("episode")))
            {
                return Ok(MediaType::TvShow);
            }

            // Check for movie indicators
            if parent_name.contains("movie")
                || parent_name.contains("movies")
                || parent_name.contains("film")
            {
                return Ok(MediaType::Movie);
            }
        }

        // For now, default to movie for all video files in test directories
        // This will be refined in future iterations
        Ok(MediaType::Movie)
    }

    /// Parse all media files
    async fn parse_media_files(
        &self,
        media_files: &[MediaFile],
    ) -> Result<(Vec<crate::types::ParsingResult>, Vec<FailedFile>)> {
        let mut parsed_files = Vec::new();
        let mut failed_files = Vec::new();

        for media_file in media_files {
            match self.parse_single_file(media_file).await {
                Ok(parsing_result) => {
                    parsed_files.push(parsing_result);
                }
                Err(error) => {
                    let failed_file = FailedFile {
                        media_file: media_file.clone(),
                        error: error.to_string(),
                        failed_at: Utc::now(),
                    };
                    failed_files.push(failed_file);
                }
            }
        }

        Ok((parsed_files, failed_files))
    }

    /// Parse a single media file
    async fn parse_single_file(
        &self,
        media_file: &MediaFile,
    ) -> Result<crate::types::ParsingResult> {
        match media_file.media_type {
            MediaType::Movie => self.movie_parser.parse_movie(&media_file.file_path).await,
            MediaType::TvShow => {
                // TODO: Implement TV show parsing in Iteration 3
                anyhow::bail!("TV show parsing not yet implemented")
            }
            MediaType::Music => {
                // TODO: Implement music parsing in Iteration 4
                anyhow::bail!("Music parsing not yet implemented")
            }
            MediaType::Subtitle => {
                // TODO: Implement subtitle parsing
                anyhow::bail!("Subtitle parsing not yet implemented")
            }
            MediaType::Unknown => {
                anyhow::bail!("Unknown media type")
            }
        }
    }

    /// Calculate scan statistics
    fn calculate_statistics(
        &self,
        media_files: &[MediaFile],
        parsed_files: &[crate::types::ParsingResult],
        failed_files: &[FailedFile],
        start_time: Instant,
    ) -> ScanStatistics {
        let total_files = media_files.len() as u32;
        let parsed_count = parsed_files.len() as u32;
        let failed_count = failed_files.len() as u32;

        let success_rate = if total_files > 0 {
            parsed_count as f32 / total_files as f32
        } else {
            0.0
        };

        let average_confidence = if !parsed_files.is_empty() {
            let total_confidence: f32 = parsed_files
                .iter()
                .map(|result| result.confidence_score)
                .sum();
            total_confidence / parsed_count as f32
        } else {
            0.0
        };

        let duration_seconds = start_time.elapsed().as_secs_f64();

        ScanStatistics {
            total_files,
            parsed_files: parsed_count,
            failed_files: failed_count,
            success_rate,
            average_confidence,
            duration_seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_is_media_extension() {
        let scanner = Scanner::new(MovieParser::new(None));

        assert!(scanner.is_media_extension("mkv"));
        assert!(scanner.is_media_extension("mp4"));
        assert!(scanner.is_media_extension("mp3"));
        assert!(scanner.is_media_extension("flac"));
        assert!(scanner.is_media_extension("srt"));
        assert!(!scanner.is_media_extension("txt"));
        assert!(!scanner.is_media_extension("pdf"));
    }

    #[test]
    fn test_detect_media_type() {
        let scanner = Scanner::new(MovieParser::new(None));

        // Test subtitle detection
        let subtitle_path = PathBuf::from("movie.srt");
        let media_type = scanner.detect_media_type(&subtitle_path, "srt").unwrap();
        assert_eq!(media_type, MediaType::Subtitle);

        // Test audio detection
        let audio_path = PathBuf::from("song.mp3");
        let media_type = scanner.detect_media_type(&audio_path, "mp3").unwrap();
        assert_eq!(media_type, MediaType::Music);

        // Test video detection (defaults to movie)
        let video_path = PathBuf::from("movie.mkv");
        let media_type = scanner.detect_media_type(&video_path, "mkv").unwrap();
        assert_eq!(media_type, MediaType::Movie);
    }

    #[tokio::test]
    async fn test_scan_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let scanner = Scanner::new(MovieParser::new(None));

        let result = scanner.scan_directory(temp_dir.path()).await.unwrap();
        assert_eq!(result.statistics.total_files, 0);
        assert_eq!(result.statistics.parsed_files, 0);
        assert_eq!(result.statistics.failed_files, 0);
    }

    #[test]
    fn test_scanner_debug() {
        let parser = MovieParser::new(None);
        let scanner = Scanner::new(parser);
        let debug_output = format!("{:?}", scanner);

        assert!(debug_output.contains("Scanner"));
        assert!(debug_output.contains("movie_parser"));
    }
}
