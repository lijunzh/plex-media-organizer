//! Directory scanning and file discovery

use crate::config::AppConfig;
use crate::parsers::UnifiedMovieParser;
use crate::types::{FailedFile, MediaFile, MediaType, ScanResult, ScanStatistics, ParsingResult};
// use crate::parsers::types::ParserResult; // Unused import
use anyhow::Result;
use chrono::Utc;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

/// Media file scanner with network drive optimizations
#[derive(Debug)]
pub struct Scanner {
    movie_parser: UnifiedMovieParser,
    /// Application configuration
    pub config: AppConfig,
    /// Maximum number of concurrent parsing operations
    concurrency_limit: usize,
    /// Whether to use network drive optimizations
    network_mode: bool,
    /// Batch size for network operations
    batch_size: usize,
}

impl Scanner {
    /// Create a new scanner with default concurrency limit
    pub fn new(movie_parser: UnifiedMovieParser) -> Self {
        let config = AppConfig::load().unwrap_or_default();
        Self {
            movie_parser,
            config,
            concurrency_limit: 16, // Default to 16 concurrent operations
            network_mode: false,
            batch_size: 100,
        }
    }

    /// Create a new scanner with custom concurrency limit
    pub fn with_concurrency(movie_parser: UnifiedMovieParser, concurrency_limit: usize) -> Self {
        let config = AppConfig::load().unwrap_or_default();
        Self {
            movie_parser,
            config,
            concurrency_limit,
            network_mode: false,
            batch_size: 100,
        }
    }

    /// Create a scanner optimized for network drives
    pub fn for_network_drive(movie_parser: UnifiedMovieParser) -> Self {
        let config = AppConfig::load().unwrap_or_default();
        Self {
            movie_parser,
            config,
            concurrency_limit: 4, // Reduced concurrency for network drives
            network_mode: true,
            batch_size: 50, // Smaller batches for network drives
        }
    }

    /// Create a new scanner with custom concurrency limit and config (single load)
    pub fn with_concurrency_and_config(
        movie_parser: UnifiedMovieParser,
        concurrency_limit: usize,
        config: &AppConfig,
    ) -> Self {
        Self {
            movie_parser,
            config: config.clone(),
            concurrency_limit,
            network_mode: false,
            batch_size: 100,
        }
    }

    /// Create a scanner optimized for network drives with config (single load)
    pub fn for_network_drive_with_config(
        movie_parser: UnifiedMovieParser,
        config: &AppConfig,
    ) -> Self {
        Self {
            movie_parser,
            config: config.clone(),
            concurrency_limit: 4, // Reduced concurrency for network drives
            network_mode: true,
            batch_size: 50, // Smaller batches for network drives
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

    /// Set network mode
    pub fn set_network_mode(&mut self, enabled: bool) {
        self.network_mode = enabled;
        if enabled {
            // Adjust settings for network drives
            self.concurrency_limit = self.concurrency_limit.min(4);
            self.batch_size = self.batch_size.min(50);
        }
    }

    /// Set batch size for operations
    pub fn set_batch_size(&mut self, batch_size: usize) {
        self.batch_size = batch_size;
    }

    /// Detect if a path is likely a network drive
    pub fn detect_network_drive(path: &Path) -> bool {
        // Common network drive patterns
        let path_str = path.to_string_lossy();

        // Windows network paths
        if path_str.starts_with("\\\\") || path_str.starts_with("//") {
            return true;
        }

        // macOS network paths (only if it contains spaces, indicating a mounted network drive)
        if path_str.starts_with("/Volumes/") && path_str.contains(" ") {
            return true;
        }

        // Linux network mounts (only specific network mount points)
        if path_str.starts_with("/mnt/")
            && (path_str.contains("smb") || path_str.contains("nfs") || path_str.contains("cifs"))
        {
            return true;
        }
        if path_str.starts_with("/media/")
            && (path_str.contains("smb") || path_str.contains("nfs") || path_str.contains("cifs"))
        {
            return true;
        }

        // Check for SMB/CIFS in mount info (Linux/macOS) - only for the specific path
        #[cfg(unix)]
        {
            if let Ok(output) = std::process::Command::new("mount").output() {
                let mount_info = String::from_utf8_lossy(&output.stdout);
                for line in mount_info.lines() {
                    if line.contains("smb") || line.contains("cifs") {
                        // Check if this mount point matches our path
                        if let Some(mount_point) = line.split_whitespace().next() {
                            if path_str.starts_with(mount_point) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Scan a directory for media files with network optimizations
    pub async fn scan_directory(&self, directory: &Path) -> Result<ScanResult> {
        let start_time = Instant::now();

        // Auto-detect network drives if not explicitly set
        let is_network = self.network_mode || Self::detect_network_drive(directory);

        if is_network {
            println!("🌐 Network drive detected - using optimized settings");
            println!(
                "   • Concurrency: {} (reduced for network stability)",
                self.concurrency_limit
            );
            println!(
                "   • Batch size: {} (smaller batches for network)",
                self.batch_size
            );
        }

        println!("Scanning directory: {}", directory.display());

        // Discover all files with network optimizations
        let files = self.discover_files(directory)?;
        println!("Found {} files", files.len());

        // Filter media files
        let media_files = self.filter_media_files(&files)?;
        println!("Found {} media files", media_files.len());

        // Parse media files with network-optimized settings
        if media_files.len() > 10 {
            println!(
                "Using {} processing with {} concurrent operations",
                if is_network {
                    "network-optimized"
                } else {
                    "parallel"
                },
                self.concurrency_limit
            );
        }

        let (parsed_files, parse_failed_files) = if is_network {
            self.parse_media_files_network_optimized(&media_files)
                .await?
        } else {
            self.parse_media_files(&media_files).await?
        };

        println!("Successfully parsed {} files", parsed_files.len());
        if !parse_failed_files.is_empty() {
            println!("Failed to parse {} files", parse_failed_files.len());
            // Debug: Show first few parse failures
            for (i, failed) in parse_failed_files.iter().take(3).enumerate() {
                println!(
                    "  Parse failed {}: {} - {}",
                    i + 1,
                    failed.media_file.file_name,
                    failed.error
                );
            }
        }

        // Apply confidence filtering
        let (filtered_files, confidence_failed_files) = self.filter_by_confidence(parsed_files);
        let mut all_failed_files = parse_failed_files;
        all_failed_files.extend(confidence_failed_files.clone());

        println!(
            "After confidence filtering: {} files ready for organization",
            filtered_files.len()
        );
        if !confidence_failed_files.is_empty() {
            println!(
                "Skipped {} files due to low confidence or no TMDB match",
                confidence_failed_files.len()
            );
            // Debug: Show first few failed files
            for (i, failed) in confidence_failed_files.iter().take(3).enumerate() {
                println!(
                    "  Failed {}: {} - {}",
                    i + 1,
                    failed.media_file.file_name,
                    failed.error
                );
            }
        }

        // Calculate statistics
        let statistics =
            self.calculate_statistics(&media_files, &filtered_files, &all_failed_files, start_time);

        let result = ScanResult {
            directory: directory.to_path_buf(),
            files: media_files,
            parsed_files: filtered_files,
            failed_files: all_failed_files,
            statistics,
            scanned_at: Utc::now(),
        };

        Ok(result)
    }

    /// Discover all files in a directory with network optimizations
    fn discover_files(&self, directory: &Path) -> Result<Vec<PathBuf>> {
        if !directory.exists() {
            anyhow::bail!("Directory does not exist: {}", directory.display());
        }

        if !directory.is_dir() {
            anyhow::bail!("Path is not a directory: {}", directory.display());
        }

        // For network drives, use sequential discovery to avoid overwhelming the connection
        if self.network_mode {
            self.discover_files_sequential(directory)
        } else if self.should_use_parallel_discovery(directory) {
            self.discover_files_parallel(directory)
        } else {
            self.discover_files_sequential(directory)
        }
    }

    /// Check if parallel discovery should be used
    fn should_use_parallel_discovery(&self, directory: &Path) -> bool {
        // Don't use parallel discovery for network drives
        if self.network_mode {
            return false;
        }

        // Use parallel discovery for directories with many subdirectories
        // This is a heuristic - in practice, we could count files first
        WalkDir::new(directory)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .take(10) // If we have 10+ subdirectories, use parallel
            .count()
            >= 10
    }

    /// Discover files sequentially (optimized for network drives)
    fn discover_files_sequential(&self, directory: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let mut dir_count = 0;

        for entry in WalkDir::new(directory)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            } else if entry.file_type().is_dir() {
                dir_count += 1;

                // For network drives, show progress during discovery
                if self.network_mode && dir_count % 10 == 0 {
                    println!(
                        "   Discovered {} directories, {} files...",
                        dir_count,
                        files.len()
                    );
                }
            }
        }

        Ok(files)
    }

    /// Discover files using parallel processing (for local drives)
    fn discover_files_parallel(&self, directory: &Path) -> Result<Vec<PathBuf>> {
        use rayon::prelude::*;

        let entries: Vec<_> = WalkDir::new(directory)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect();

        let files: Vec<PathBuf> = entries
            .par_iter()
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.path().to_path_buf())
            .collect();

        Ok(files)
    }

    /// Filter media files with network optimizations
    fn filter_media_files(&self, files: &[PathBuf]) -> Result<Vec<MediaFile>> {
        let mut media_files = Vec::new();
        let mut processed = 0;
        let mut skipped_extras = 0;

        for file_path in files {
            if let Some(extension) = file_path.extension() {
                if self.is_media_extension(extension) {
                // Skip extras content (menus, interviews, trailers, etc.)
                if self.is_extras_content(file_path) {
                    skipped_extras += 1;
                    processed += 1;
                    continue;
                }

                // For network drives, minimize file system calls
                let media_file = if self.network_mode {
                    self.create_media_file_network_optimized(file_path)?
                } else {
                    self.create_media_file(file_path)?
                };
                media_files.push(media_file);
                }
            }

            processed += 1;

            // Show progress for network drives
            if self.network_mode && processed % 100 == 0 {
                println!(
                    "   Filtered {} files, found {} media files, skipped {} extras...",
                    processed,
                    media_files.len(),
                    skipped_extras
                );
            }
        }

        if skipped_extras > 0 {
            println!(
                "   Skipped {} extras files (menus, interviews, trailers, etc.)",
                skipped_extras
            );
        }

        Ok(media_files)
    }

    /// Check if a file is extras content that should be skipped
    fn is_extras_content(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy().to_lowercase();

        // Primary check: Skip everything in extras/bonus directories
        // This is the most reliable method as it follows standard conventions
        let extras_dirs = [
            "extras",
            "bonus",
            "special",
            "behind",
            "making",
            "interviews",
            "trailers",
            "commentaries",
            "deleted.scenes",
            "outtakes",
            "featurettes",
            "promos",
            "samples",
        ];

        for dir in &extras_dirs {
            if path_str.contains(&format!("/{}/", dir))
                || path_str.contains(&format!("\\{}\\", dir))
            {
                return true;
            }
        }

        // Secondary check: Skip specific file extensions that are typically extras
        let extras_extensions = crate::config::AppConfig::load()
            .map(|config| config.get_extras_extensions())
            .unwrap_or_else(|_| vec!["ifo".to_string(), "bup".to_string(), "vob".to_string()]);
        if let Some(ext) = file_path.extension() {
            if let Some(ext_str) = ext.to_str() {
                if extras_extensions.contains(&ext_str.to_lowercase()) {
                    return true;
                }
            }
        }

        // Secondary check: Skip files with obvious extras patterns in filename
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Check for obvious extras patterns in filename
        let obvious_extras_patterns = [
            "bdmenu",
            "menu",
            "bdmv",
            "sample",
            "trailer",
            "pv",
            "interview",
            "commentary",
            "featurette",
            "deleted.scene",
            "bloopers",
            "outtakes",
            "making.of",
            "behind.scenes",
            "promo",
            "teaser",
            "preview",
            // Chinese/Japanese interview patterns
            "访谈",
            "采访",
            "对谈",
            "座谈",
            "访问",
        ];

        for pattern in &obvious_extras_patterns {
            if file_name.contains(pattern) {
                return true;
            }
        }

        // Tertiary check: Skip very small files that are likely not full movies
        // Only apply this check for files that are clearly not movies
        if let Ok(metadata) = std::fs::metadata(file_path) {
            let size_mb = metadata.len() / (1024 * 1024);
            if size_mb < 50 {
                // Only skip if filename clearly indicates it's not a movie
                // This is a minimal list since we primarily rely on directory-based detection
                if file_name.contains("sample")
                    || file_name.contains("trailer")
                    || file_name.contains("pv")
                    || file_name.contains("menu")
                    || file_name.contains("bdmv")
                    || file_name.contains("interview")
                    || file_name.contains("commentary")
                    || file_name.contains("featurette")
                    || file_name.contains("deleted.scene")
                    || file_name.contains("bloopers")
                    || file_name.contains("outtakes")
                {
                    return true;
                }
            }
        }

        false
    }

    /// Create media file with minimal file system calls (network optimized)
    fn create_media_file_network_optimized(&self, file_path: &Path) -> Result<MediaFile> {
        // Minimize file system calls for network drives
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let media_type = self.detect_media_type(file_path);

        // Use a simple hash based on path for network drives to avoid reading file content
        let content_hash = format!("network_{}", file_path.display());

        Ok(MediaFile {
            id: uuid::Uuid::new_v4().to_string(),
            file_path: file_path.to_path_buf(),
            file_name,
            file_size: 0, // Will be filled later if needed
            media_type,
            content_hash,
            last_modified: Utc::now(), // Will be filled later if needed
            metadata: crate::types::MediaMetadata::default(),
        })
    }

    /// Create media file with full metadata (local drives)
    fn create_media_file(&self, file_path: &Path) -> Result<MediaFile> {
        let metadata = std::fs::metadata(file_path)?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let media_type = self.detect_media_type(file_path);

        // Generate content hash for local files
        let content_hash = self.generate_content_hash(file_path)?;

        Ok(MediaFile {
            id: uuid::Uuid::new_v4().to_string(),
            file_path: file_path.to_path_buf(),
            file_name,
            file_size: metadata.len(),
            media_type,
            content_hash,
            last_modified: chrono::DateTime::from(metadata.modified()?),
            metadata: crate::types::MediaMetadata::default(),
        })
    }

    /// Parse media files with network optimizations
    async fn parse_media_files_network_optimized(
        &self,
        media_files: &[MediaFile],
    ) -> Result<(Vec<crate::types::ParsingResult>, Vec<FailedFile>)> {
        let movie_parser = self.movie_parser.clone();
        let total_files = media_files.len();
        let batch_size = self.batch_size;

        // Create progress bar
        let progress_bar = ProgressBar::new(total_files as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) [Network Mode]",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        progress_bar.set_message("Parsing media files (network optimized)...");

        let mut parsed_files = Vec::new();
        let mut failed_files = Vec::new();

        // Process in smaller batches for network drives
        for (batch_idx, chunk) in media_files.chunks(batch_size).enumerate() {
            println!(
                "   Processing batch {}/{} ({} files)...",
                batch_idx + 1,
                total_files.div_ceil(batch_size),
                chunk.len()
            );

            // Process batch with reduced concurrency
            let batch_stream = stream::iter(chunk.iter().cloned())
                .map(|media_file| {
                    let movie_parser = movie_parser.clone();
                    let progress_bar = progress_bar.clone();
                    async move {
                        let result = match movie_parser.parse_movie(&media_file.file_path).await {
                            Ok(parser_result) => {
                                // Convert ParserResult<FilenameComponents> to ParsingResult
                                let parsing_result = ParsingResult {
                                    media_file: media_file.clone(),
                                    parsed_metadata: crate::types::MediaMetadata::default(), // TODO: Convert from FilenameComponents
                                    confidence_score: parser_result.confidence,
                                    parsing_strategy: crate::types::ParsingStrategy::FilenameOnly,
                                    external_sources: Vec::new(),
                                    user_corrections: Vec::new(),
                                    created_at: chrono::Utc::now(),
                                    updated_at: chrono::Utc::now(),
                                };
                                Ok(parsing_result)
                            }
                            Err(error) => {
                                let failed_file = FailedFile {
                                    media_file: media_file.clone(),
                                    error: error.to_string(),
                                    failed_at: Utc::now(),
                                };
                                Err(failed_file)
                            }
                        };

                        progress_bar.inc(1);
                        result
                    }
                })
                .buffer_unordered(self.concurrency_limit);

            let batch_results: Vec<_> = batch_stream.collect().await;

            // Collect results
            for result in batch_results {
                match result {
                    Ok(parsing_result) => parsed_files.push(parsing_result),
                    Err(failed_file) => failed_files.push(failed_file),
                }
            }

            // Small delay between batches for network drives
            if self.network_mode && batch_idx < total_files.div_ceil(batch_size) - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }

        progress_bar.finish_with_message("Network-optimized parsing completed!");

        Ok((parsed_files, failed_files))
    }

    /// Parse all media files using parallel processing (original method)
    async fn parse_media_files(
        &self,
        media_files: &[MediaFile],
    ) -> Result<(Vec<crate::types::ParsingResult>, Vec<FailedFile>)> {
        let movie_parser = self.movie_parser.clone();
        let total_files = media_files.len();

        // Create progress bar
        let progress_bar = ProgressBar::new(total_files as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        progress_bar.set_message("Parsing media files...");

        // Create a stream of parsing operations
        let parsing_stream = stream::iter(media_files.iter().cloned())
            .map(|media_file| {
                let movie_parser = movie_parser.clone();
                let progress_bar = progress_bar.clone();
                async move {
                    let result = match movie_parser.parse_movie(&media_file.file_path).await {
                        Ok(parser_result) => {
                            // Convert ParserResult<FilenameComponents> to ParsingResult
                            let parsing_result = ParsingResult {
                                media_file: media_file.clone(),
                                parsed_metadata: crate::types::MediaMetadata::default(), // TODO: Convert from FilenameComponents
                                confidence_score: parser_result.confidence,
                                parsing_strategy: crate::types::ParsingStrategy::FilenameOnly,
                                external_sources: Vec::new(),
                                user_corrections: Vec::new(),
                                created_at: chrono::Utc::now(),
                                updated_at: chrono::Utc::now(),
                            };
                            Ok(parsing_result)
                        }
                        Err(error) => {
                            let failed_file = FailedFile {
                                media_file: media_file.clone(),
                                error: error.to_string(),
                                failed_at: Utc::now(),
                            };
                            Err(failed_file)
                        }
                    };

                    // Update progress
                    progress_bar.inc(1);
                    result
                }
            })
            .buffer_unordered(self.concurrency_limit);

        // Collect results
        let results: Vec<_> = parsing_stream.collect().await;

        // Finish progress bar
        progress_bar.finish_with_message("Parsing completed!");

        let mut parsed_files = Vec::new();
        let mut failed_files = Vec::new();

        for result in results {
            match result {
                Ok(parsing_result) => parsed_files.push(parsing_result),
                Err(failed_file) => failed_files.push(failed_file),
            }
        }

        Ok((parsed_files, failed_files))
    }

    /// Filter parsing results based on confidence threshold
    fn filter_by_confidence(
        &self,
        parsed_files: Vec<crate::types::ParsingResult>,
    ) -> (Vec<crate::types::ParsingResult>, Vec<FailedFile>) {
        let mut filtered_files = Vec::new();
        let mut skipped_files = Vec::new();
        let threshold = self.config.organization.matching.min_confidence_threshold;
        let skip_unmatched = self.config.organization.matching.skip_unmatched_movies;
        let warn_on_low_confidence = self.config.organization.matching.warn_on_low_confidence;

        for parsing_result in parsed_files {
            let confidence = parsing_result.confidence_score;
            let has_tmdb_match = !parsing_result.external_sources.is_empty();

            // Check if we should skip this file
            let should_skip = if skip_unmatched && !has_tmdb_match {
                // Skip unmatched movies if configured
                true
            } else if confidence < threshold {
                // Skip low confidence matches
                true
            } else {
                false
            };

            if should_skip {
                let failed_file = FailedFile {
                    media_file: parsing_result.media_file.clone(),
                    error: if !has_tmdb_match {
                        format!("No TMDB match found (confidence: {:.2})", confidence)
                    } else {
                        format!(
                            "Low confidence match (confidence: {:.2} < {:.2})",
                            confidence, threshold
                        )
                    },
                    failed_at: Utc::now(),
                };
                skipped_files.push(failed_file);

                // Show warning if configured
                if warn_on_low_confidence {
                    let filename = parsing_result.media_file.file_name.clone();
                    if !has_tmdb_match {
                        println!("⚠️  Skipped: {} - No TMDB match found", filename);
                    } else {
                        println!(
                            "⚠️  Skipped: {} - Low confidence ({:.2} < {:.2})",
                            filename, confidence, threshold
                        );
                    }
                }
            } else {
                // Keep the file but show warning for low confidence
                if warn_on_low_confidence && confidence < 0.7 {
                    let filename = parsing_result.media_file.file_name.clone();
                    println!(
                        "⚠️  Low confidence: {} (confidence: {:.2})",
                        filename, confidence
                    );
                }
                filtered_files.push(parsing_result);
            }
        }

        (filtered_files, skipped_files)
    }

    /// Check if a file extension is a media file
    fn is_media_extension(&self, extension: &std::ffi::OsStr) -> bool {
        let ext_str = extension.to_string_lossy().to_lowercase();
        matches!(
            ext_str.as_str(),
            "mkv" | "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "3gp" | "ogv"
        )
    }

    /// Detect media type based on file extension
    fn detect_media_type(&self, file_path: &Path) -> MediaType {
        if let Some(extension) = file_path.extension() {
            let ext_str = extension.to_string_lossy().to_lowercase();
            match ext_str.as_str() {
                "mkv" | "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "3gp" | "ogv" => {
                    MediaType::Movie
                }
                _ => MediaType::Unknown,
            }
        } else {
            MediaType::Unknown
        }
    }

    /// Generate content hash for a file
    fn generate_content_hash(&self, file_path: &Path) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let metadata = std::fs::metadata(file_path)?;
        let mut hasher = DefaultHasher::new();

        // Hash file size and modification time for efficiency
        metadata.len().hash(&mut hasher);
        metadata.modified()?.hash(&mut hasher);

        Ok(format!("{:x}", hasher.finish()))
    }

    /// Calculate scan statistics
    fn calculate_statistics(
        &self,
        media_files: &[MediaFile],
        parsed_files: &[crate::types::ParsingResult],
        failed_files: &[FailedFile],
        start_time: Instant,
    ) -> ScanStatistics {
        let duration = start_time.elapsed();
        let total_files = media_files.len() as u32;
        let parsed_count = parsed_files.len() as u32;
        let failed_count = failed_files.len() as u32;
        let success_rate = if total_files > 0 {
            parsed_count as f32 / total_files as f32
        } else {
            0.0
        };

        let files_per_second = if duration.as_secs() > 0 {
            total_files as f64 / duration.as_secs_f64()
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

        ScanStatistics {
            total_files,
            parsed_files: parsed_count,
            failed_files: failed_count,
            success_rate,
            average_confidence,
            duration_seconds: duration.as_secs_f64(),
            files_per_second,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::UnifiedMovieParser;

    #[test]
    fn test_network_drive_detection() {
        // Test Windows network paths
        assert!(Scanner::detect_network_drive(Path::new(
            "\\\\server\\share"
        )));
        assert!(Scanner::detect_network_drive(Path::new("//server/share")));

        // Test macOS network paths (with spaces)
        assert!(Scanner::detect_network_drive(Path::new(
            "/Volumes/Network Drive"
        )));
        assert!(!Scanner::detect_network_drive(Path::new(
            "/Volumes/MyDrive"
        )));

        // Test Linux network paths
        assert!(Scanner::detect_network_drive(Path::new("/mnt/smb-share")));
        assert!(Scanner::detect_network_drive(Path::new("/media/nfs-mount")));
        assert!(!Scanner::detect_network_drive(Path::new("/mnt/local")));
        assert!(!Scanner::detect_network_drive(Path::new("/media/usb")));

        // Test local paths
        assert!(!Scanner::detect_network_drive(Path::new("/home/user")));
        assert!(!Scanner::detect_network_drive(Path::new("C:\\Users\\user")));
    }

    #[test]
    fn test_network_mode_settings() {
        let movie_parser = UnifiedMovieParser::new();
        let scanner = Scanner::for_network_drive(movie_parser.clone());

        assert_eq!(scanner.concurrency_limit(), 4);
        assert_eq!(scanner.batch_size, 50);
        assert!(scanner.network_mode);

        // Test setting network mode on existing scanner
        let mut scanner2 = Scanner::new(movie_parser);
        scanner2.set_network_mode(true);

        assert_eq!(scanner2.concurrency_limit(), 4);
        assert_eq!(scanner2.batch_size, 50);
        assert!(scanner2.network_mode);
    }

    #[test]
    fn test_media_extension_detection() {
        let scanner = Scanner::new(UnifiedMovieParser::new());

        assert!(scanner.is_media_extension(std::ffi::OsStr::new("mkv")));
        assert!(scanner.is_media_extension(std::ffi::OsStr::new("MP4")));
        assert!(scanner.is_media_extension(std::ffi::OsStr::new("avi")));
        assert!(!scanner.is_media_extension(std::ffi::OsStr::new("txt")));
        assert!(!scanner.is_media_extension(std::ffi::OsStr::new("pdf")));
    }

    #[test]
    fn test_extras_content_detection() {
        let scanner = Scanner::new(UnifiedMovieParser::new());

        // Directory-based detection (most reliable)
        assert!(scanner.is_extras_content(Path::new("/path/to/extras/BDMenu(JPGLBL).mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/bonus/making.of.batman.mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/interviews/director.interview.mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/trailers/movie.trailer.mkv")));
        assert!(scanner.is_extras_content(Path::new("/movie/extras/DVDSP/樱花抄 动画分镜.mkv")));

        // Obvious filename patterns (fallback for files not in extras directories)
        assert!(scanner.is_extras_content(Path::new("/path/to/BDMenu(JP).mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/水桥研二访谈.mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/movie.trailer.mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/sample.video.mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/director.commentary.mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/behind.scenes.mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/deleted.scene.mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/movie.pv.mkv")));
        assert!(scanner.is_extras_content(Path::new("/path/to/featurette.mkv")));

        // Should NOT be detected as extras (actual movies)
        assert!(!scanner.is_extras_content(Path::new(
            "/path/to/5.Centimeters.Per.Second.2007.BluRay.1080p.x265.10bit.DDP.5.1-ted423@FRDS.mkv"
        )));
        assert!(!scanner.is_extras_content(Path::new("/path/to/Iron.Man.2008.BluRay.1080p.mkv")));
        assert!(!scanner.is_extras_content(Path::new("/path/to/The.Matrix.1999.1080p.mkv")));
        assert!(!scanner.is_extras_content(Path::new("/path/to/Avengers.Endgame.2019.4K.mkv")));
        // Note: We still use some filename-based detection as fallback for obvious extras
        // but this means movies with certain words in titles might be false positives.
        // The primary detection method is directory-based which is much more reliable.
    }
}
