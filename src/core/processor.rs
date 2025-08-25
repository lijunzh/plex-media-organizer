//! Media processing orchestration

use crate::config::AppConfig;
use crate::core::Scanner;
use crate::parsers::UnifiedMovieParser;
use crate::types::{FailedFile, ParsingResult, ScanResult, ScanStatistics};
use anyhow::Result;
use chrono::Utc;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Instant;

/// Media processing orchestrator
#[derive(Debug)]
pub struct Processor {
    movie_parser: UnifiedMovieParser,
    scanner: Scanner,
    config: AppConfig,
    /// Maximum number of concurrent parsing operations
    concurrency_limit: usize,
}

impl Processor {
    /// Create a new processor with default settings
    pub fn new(movie_parser: UnifiedMovieParser) -> Self {
        let config = AppConfig::load().unwrap_or_default();
        let scanner = Scanner::new();
        
        Self {
            movie_parser,
            scanner,
            config,
            concurrency_limit: 16, // Default to 16 concurrent operations
        }
    }

    /// Create a new processor with custom concurrency limit
    pub fn with_concurrency(movie_parser: UnifiedMovieParser, concurrency_limit: usize) -> Self {
        let config = AppConfig::load().unwrap_or_default();
        let scanner = Scanner::new();
        
        Self {
            movie_parser,
            scanner,
            config,
            concurrency_limit,
        }
    }

    /// Create a processor optimized for network drives
    pub fn for_network_drive(movie_parser: UnifiedMovieParser) -> Self {
        let config = AppConfig::load().unwrap_or_default();
        let scanner = Scanner::for_network_drive();
        
        Self {
            movie_parser,
            scanner,
            config,
            concurrency_limit: 4, // Reduced concurrency for network drives
        }
    }

    /// Create a new processor with custom concurrency limit and config
    pub fn with_concurrency_and_config(
        movie_parser: UnifiedMovieParser,
        concurrency_limit: usize,
        config: &AppConfig,
    ) -> Self {
        let scanner = Scanner::new();
        
        Self {
            movie_parser,
            scanner,
            config: config.clone(),
            concurrency_limit,
        }
    }

    /// Create a processor optimized for network drives with config
    pub fn for_network_drive_with_config(
        movie_parser: UnifiedMovieParser,
        config: &AppConfig,
    ) -> Self {
        let scanner = Scanner::for_network_drive();
        
        Self {
            movie_parser,
            scanner,
            config: config.clone(),
            concurrency_limit: 4, // Reduced concurrency for network drives
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
        self.scanner.set_network_mode(enabled);
        if enabled {
            // Adjust settings for network drives
            self.concurrency_limit = self.concurrency_limit.min(4);
        }
    }

    /// Process a directory and return scan results
    pub async fn process_directory(&self, directory: &Path) -> Result<ScanResult> {
        let start_time = Instant::now();

        // Auto-detect network drives if not explicitly set
        let is_network = Scanner::detect_network_drive(directory);

        if is_network {
            println!("🌐 Network drive detected - using optimized settings");
            println!(
                "   • Concurrency: {} (reduced for network stability)",
                self.concurrency_limit
            );
        }

        println!("Scanning directory: {}", directory.display());

        // Discover all files with network optimizations
        let files = self.scanner.discover_files(directory)?;
        println!("Found {} files", files.len());

        // Filter media files
        let media_files = self.scanner.filter_media_files(&files)?;
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

    /// Parse media files with parallel processing
    async fn parse_media_files(
        &self,
        media_files: &[crate::types::MediaFile],
    ) -> Result<(Vec<ParsingResult>, Vec<FailedFile>)> {
        let progress_bar = self.create_progress_bar(media_files.len(), "Parsing files");

        let chunks: Vec<_> = media_files
            .chunks(self.concurrency_limit)
            .map(|chunk| chunk.to_vec())
            .collect();

        let mut all_parsed_files = Vec::new();
        let mut all_failed_files = Vec::new();

        for chunk in chunks {
            let chunk_len = chunk.len();
            let futures: Vec<_> = chunk
                .into_iter()
                .map(|media_file| {
                    let parser = &self.movie_parser;
                    async move {
                        match parser.parse_movie(&media_file.file_path).await {
                            Ok(result) => Ok(result),
                            Err(e) => Err(FailedFile {
                                media_file,
                                error: e.to_string(),
                                stage: "parsing".to_string(),
                                failed_at: Utc::now(),
                            }),
                        }
                    }
                })
                .collect();

            let results = stream::iter(futures)
                .buffer_unordered(self.concurrency_limit)
                .collect::<Vec<_>>()
                .await;

            for result in results {
                match result {
                    Ok(parsed_file) => all_parsed_files.push(parsed_file),
                    Err(failed_file) => all_failed_files.push(failed_file),
                }
            }

            progress_bar.inc(chunk_len as u64);
        }

        progress_bar.finish_with_message("Parsing completed");
        Ok((all_parsed_files, all_failed_files))
    }

    /// Parse media files with network-optimized settings
    async fn parse_media_files_network_optimized(
        &self,
        media_files: &[crate::types::MediaFile],
    ) -> Result<(Vec<ParsingResult>, Vec<FailedFile>)> {
        let progress_bar = self.create_progress_bar(media_files.len(), "Parsing files (network)");

        let mut all_parsed_files = Vec::new();
        let mut all_failed_files = Vec::new();

        // Process in smaller batches for network drives
        let batch_size = 10;
        for chunk in media_files.chunks(batch_size) {
            let chunk_len = chunk.len();
            let futures: Vec<_> = chunk
                .iter()
                .map(|media_file| {
                    let parser = &self.movie_parser;
                    let file_path = media_file.file_path.clone();
                    async move {
                        match parser.parse_movie(&file_path).await {
                            Ok(result) => Ok(result),
                                                    Err(e) => Err(FailedFile {
                            media_file: media_file.clone(),
                            error: e.to_string(),
                            stage: "parsing".to_string(),
                            failed_at: Utc::now(),
                        }),
                        }
                    }
                })
                .collect();

            let results = stream::iter(futures)
                .buffer_unordered(2) // Very low concurrency for network drives
                .collect::<Vec<_>>()
                .await;

            for result in results {
                match result {
                    Ok(parsed_file) => all_parsed_files.push(parsed_file),
                    Err(failed_file) => all_failed_files.push(failed_file),
                }
            }

            progress_bar.inc(chunk_len as u64);

            // Small delay between batches for network drives
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        progress_bar.finish_with_message("Parsing completed");
        Ok((all_parsed_files, all_failed_files))
    }

    /// Filter parsed files by confidence threshold
    fn filter_by_confidence(
        &self,
        parsed_files: Vec<ParsingResult>,
    ) -> (Vec<ParsingResult>, Vec<FailedFile>) {
        let min_confidence = self.config.organization.matching.min_confidence_threshold;
        let mut filtered_files = Vec::new();
        let mut failed_files = Vec::new();

        for parsed_file in parsed_files {
            if parsed_file.confidence_score >= min_confidence {
                filtered_files.push(parsed_file);
            } else {
                failed_files.push(FailedFile {
                    media_file: parsed_file.media_file,
                    error: format!(
                        "Low confidence score: {:.2} (threshold: {:.2})",
                        parsed_file.confidence_score, min_confidence
                    ),
                    stage: "confidence_filtering".to_string(),
                    failed_at: Utc::now(),
                });
            }
        }

        (filtered_files, failed_files)
    }

    /// Calculate processing statistics
    fn calculate_statistics(
        &self,
        media_files: &[crate::types::MediaFile],
        parsed_files: &[ParsingResult],
        failed_files: &[FailedFile],
        start_time: Instant,
    ) -> ScanStatistics {
        let total_files = media_files.len();
        let successful_parses = parsed_files.len();
        let failed_parses = failed_files.len();
        let processing_time = start_time.elapsed();

        let success_rate = if total_files > 0 {
            (successful_parses as f32 / total_files as f32) * 100.0
        } else {
            0.0
        };

        let files_per_second = if processing_time.as_secs() > 0 {
            total_files as f32 / processing_time.as_secs() as f32
        } else {
            0.0
        };

        ScanStatistics {
            total_files: total_files as u32,
            parsed_files: successful_parses as u32,
            failed_files: failed_parses as u32,
            success_rate,
            average_confidence: 0.8, // Placeholder
            duration_seconds: processing_time.as_secs_f64(),
            files_per_second: files_per_second as f64,
        }
    }

    /// Create a progress bar for processing feedback
    fn create_progress_bar(&self, total: usize, message: &str) -> ProgressBar {
        let progress_bar = ProgressBar::new(total as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        progress_bar.set_message(message.to_string());
        progress_bar
    }
}