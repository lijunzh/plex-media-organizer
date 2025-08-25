//! Media file scanner for file discovery and metadata extraction
//!
//! This module handles the discovery of media files in directories and extraction
//! of basic file metadata. It does not handle parsing or organization.

use crate::config::AppConfig;
use crate::types::{MediaFile, MediaType, ScanResult, ScanStatistics};
use anyhow::Result;
use chrono::Utc;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

/// Media file scanner focused on file discovery and metadata extraction
#[derive(Debug)]
pub struct MediaScanner {
    /// Application configuration
    pub config: AppConfig,
    /// Maximum number of concurrent operations
    concurrency_limit: usize,
    /// Whether to use network drive optimizations
    network_mode: bool,
    /// Batch size for network operations
    batch_size: usize,
}

impl MediaScanner {
    /// Create a new media scanner with default settings
    pub fn new() -> Self {
        let config = AppConfig::load().unwrap_or_default();
        Self {
            config,
            concurrency_limit: 16,
            network_mode: false,
            batch_size: 100,
        }
    }

    /// Create a new media scanner with custom concurrency limit
    pub fn with_concurrency(concurrency_limit: usize) -> Self {
        let config = AppConfig::load().unwrap_or_default();
        Self {
            config,
            concurrency_limit,
            network_mode: false,
            batch_size: 100,
        }
    }

    /// Create a scanner optimized for network drives
    pub fn for_network_drive() -> Self {
        let config = AppConfig::load().unwrap_or_default();
        Self {
            config,
            concurrency_limit: 4, // Reduced concurrency for network drives
            network_mode: true,
            batch_size: 50, // Smaller batches for network drives
        }
    }

    /// Create a new media scanner with configuration
    pub fn with_config(config: AppConfig) -> Self {
        Self {
            config,
            concurrency_limit: 16,
            network_mode: false,
            batch_size: 100,
        }
    }

    /// Set the concurrency limit for parallel processing
    pub fn set_concurrency_limit(&mut self, limit: usize) {
        self.concurrency_limit = limit;
    }

    /// Get the current concurrency limit
    pub fn concurrency_limit(&self) -> usize {
        self.concurrency_limit
    }

    /// Scan a directory for media files
    pub async fn scan_directory<P: AsRef<Path>>(&self, directory: P) -> Result<ScanResult> {
        let directory = directory.as_ref();
        let start_time = Instant::now();

        println!("🔍 Scanning directory: {}", directory.display());

        // Discover files
        let files = self.discover_files(directory).await?;
        println!("📁 Found {} files", files.len());

        // Extract metadata
        let media_files = self.extract_metadata(files).await?;
        println!("📊 Extracted metadata for {} files", media_files.len());

        let duration = start_time.elapsed();
        let statistics = ScanStatistics {
            total_files: media_files.len() as u32,
            parsed_files: 0, // Will be filled by processor
            failed_files: 0, // Will be filled by processor
            success_rate: 0.0, // Will be calculated by processor
            average_confidence: 0.0, // Will be calculated by processor
            duration_seconds: duration.as_secs_f64(),
            files_per_second: media_files.len() as f64 / duration.as_secs_f64(),
        };

        Ok(ScanResult {
            directory: directory.to_path_buf(),
            files: media_files,
            parsed_files: Vec::new(), // Will be filled by processor
            failed_files: Vec::new(), // Will be filled by processor
            statistics,
            scanned_at: Utc::now(),
        })
    }

    /// Discover media files in a directory
    async fn discover_files<P: AsRef<Path>>(&self, directory: P) -> Result<Vec<PathBuf>> {
        let directory = directory.as_ref();
        let mut files = Vec::new();

        // Configure walker based on network mode
        let walker = if self.network_mode {
            WalkDir::new(directory)
                .follow_links(false)
                .max_depth(10) // Limit depth for network drives
        } else {
            WalkDir::new(directory).follow_links(false)
        };

        for entry in walker {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && self.is_media_file(path) {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    /// Extract metadata from discovered files
    async fn extract_metadata(&self, files: Vec<PathBuf>) -> Result<Vec<MediaFile>> {
        let progress_bar = self.create_progress_bar(files.len());
        let mut media_files = Vec::new();

        // Process files in batches
        let chunks: Vec<Vec<PathBuf>> = files
            .chunks(self.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        for chunk in chunks {
            let chunk_results = stream::iter(chunk.clone())
                .map(|file_path| async move {
                    match self.extract_single_file_metadata(&file_path).await {
                        Ok(media_file) => Ok(media_file),
                        Err(e) => {
                            eprintln!("Failed to extract metadata for {}: {}", file_path.display(), e);
                            Err(e)
                        }
                    }
                })
                .buffer_unordered(self.concurrency_limit)
                .collect::<Vec<_>>()
                .await;

            for result in chunk_results {
                if let Ok(media_file) = result {
                    media_files.push(media_file);
                }
            }

            progress_bar.inc(chunk.len() as u64);
        }

        progress_bar.finish_with_message("Metadata extraction complete");
        Ok(media_files)
    }

    /// Extract metadata from a single file
    async fn extract_single_file_metadata(&self, file_path: &Path) -> Result<MediaFile> {
        let metadata = tokio::fs::metadata(file_path).await?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let media_type = self.detect_media_type(file_path);
        let content_hash = self.calculate_content_hash(file_path).await?;

        Ok(MediaFile {
            id: uuid::Uuid::new_v4().to_string(),
            file_path: file_path.to_path_buf(),
            file_name,
            file_size: metadata.len(),
            media_type,
            content_hash,
            last_modified: metadata.modified()?.into(),
            metadata: crate::types::MediaMetadata::default(),
        })
    }

    /// Check if a file is a media file
    fn is_media_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let ext_lower = ext_str.to_lowercase();
                return matches!(
                    ext_lower.as_str(),
                    "mkv" | "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "3gp" | "ogv"
                        | "mpg" | "mpeg" | "ts" | "mts" | "m2ts" | "vob" | "iso" | "img"
                        | "flac" | "mp3" | "wav" | "aac" | "ogg" | "wma" | "m4a" | "opus"
                        | "srt" | "ass" | "ssa" | "sub" | "idx" | "vtt"
                );
            }
        }
        false
    }

    /// Detect media type from file path
    fn detect_media_type(&self, path: &Path) -> MediaType {
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let ext_lower = ext_str.to_lowercase();
                return match ext_lower.as_str() {
                    "mkv" | "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "3gp" | "ogv"
                    | "mpg" | "mpeg" | "ts" | "mts" | "m2ts" | "vob" | "iso" | "img" => MediaType::Movie,
                    "flac" | "mp3" | "wav" | "aac" | "ogg" | "wma" | "m4a" | "opus" => MediaType::Music,
                    "srt" | "ass" | "ssa" | "sub" | "idx" | "vtt" => MediaType::Subtitle,
                    _ => MediaType::Unknown,
                };
            }
        }
        MediaType::Unknown
    }

    /// Calculate content hash for a file
    async fn calculate_content_hash(&self, path: &Path) -> Result<String> {
        // For now, use a simple hash based on file size and modification time
        // In a full implementation, this would calculate a proper content hash
        let metadata = tokio::fs::metadata(path).await?;
        let hash_input = format!(
            "{}-{}",
            metadata.len(),
            metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs()
        );
        
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        hash_input.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    /// Create a progress bar for the scanning operation
    fn create_progress_bar(&self, total: usize) -> ProgressBar {
        let progress_bar = ProgressBar::new(total as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        progress_bar
    }
}

impl Default for MediaScanner {
    fn default() -> Self {
        Self::new()
    }
}