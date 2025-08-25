//! Media processing pipeline orchestration
//!
//! This module coordinates the entire media processing workflow from scanning
//! through parsing to organization.

use crate::config::AppConfig;
use crate::parsers::MovieParser;
use crate::types::{MediaFile, ParsingResult, ScanResult};
use anyhow::Result;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Instant;

use super::scanner::MediaScanner;

/// Media processing pipeline result
#[derive(Debug)]
pub struct ProcessingResult {
    /// Scan result with discovered files
    pub scan_result: ScanResult,
    /// Parsed media files
    pub parsed_files: Vec<ParsingResult>,
    /// Processing statistics
    pub statistics: ProcessingStatistics,
}

/// Processing statistics
#[derive(Debug)]
pub struct ProcessingStatistics {
    /// Total processing time in seconds
    pub total_duration_seconds: f64,
    /// Files processed per second
    pub files_per_second: f64,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f32,
    /// Number of files successfully parsed
    pub successfully_parsed: u32,
    /// Number of files that failed to parse
    pub failed_to_parse: u32,
}

/// Media processing pipeline orchestrator
#[derive(Debug)]
pub struct MediaProcessor {
    scanner: MediaScanner,
    parser: MovieParser,
    config: AppConfig,
    concurrency_limit: usize,
}

impl MediaProcessor {
    /// Create a new media processor
    pub fn new() -> Result<Self> {
        let config = AppConfig::load()?;
        let scanner = MediaScanner::with_config(config.clone());
        let parser = MovieParser::new();

        Ok(Self {
            scanner,
            parser,
            config,
            concurrency_limit: 16,
        })
    }

    /// Create a new media processor with custom configuration
    pub fn with_config(config: AppConfig) -> Result<Self> {
        let scanner = MediaScanner::with_config(config.clone());
        let parser = MovieParser::with_config(config.clone());

        Ok(Self {
            scanner,
            parser,
            config,
            concurrency_limit: 16,
        })
    }

    /// Create a new media processor with all components
    pub fn with_components(
        scanner: MediaScanner,
        parser: MovieParser,
        config: AppConfig,
    ) -> Self {
        Self {
            scanner,
            parser,
            config,
            concurrency_limit: 16,
        }
    }

    /// Set the concurrency limit for parallel processing
    pub fn set_concurrency_limit(&mut self, limit: usize) {
        self.concurrency_limit = limit;
        self.scanner.set_concurrency_limit(limit);
    }

    /// Process a directory through the complete pipeline
    pub async fn process_directory<P: AsRef<Path>>(&self, directory: P) -> Result<ProcessingResult> {
        let start_time = Instant::now();
        let directory = directory.as_ref();

        println!("🚀 Starting media processing pipeline...");
        println!("📁 Processing directory: {}", directory.display());

        // Step 1: Scan for media files
        println!("\n📋 Step 1: Scanning for media files...");
        let scan_result = self.scanner.scan_directory(directory).await?;
        println!("✅ Found {} media files", scan_result.files.len());

        // Step 2: Parse discovered files
        println!("\n🔍 Step 2: Parsing media files...");
        let parsed_files = self.parse_files(&scan_result.files).await?;
        println!("✅ Successfully parsed {} files", parsed_files.len());

        let duration = start_time.elapsed();
        let success_rate = if scan_result.files.is_empty() {
            0.0
        } else {
            parsed_files.len() as f32 / scan_result.files.len() as f32
        };

        let statistics = ProcessingStatistics {
            total_duration_seconds: duration.as_secs_f64(),
            files_per_second: scan_result.files.len() as f64 / duration.as_secs_f64(),
            success_rate,
            successfully_parsed: parsed_files.len() as u32,
            failed_to_parse: (scan_result.files.len() - parsed_files.len()) as u32,
        };

        println!("\n📊 Processing complete!");
        println!("   Total files: {}", scan_result.files.len());
        println!("   Successfully parsed: {}", statistics.successfully_parsed);
        println!("   Failed to parse: {}", statistics.failed_to_parse);
        println!("   Success rate: {:.1}%", success_rate * 100.0);
        println!("   Processing speed: {:.1} files/second", statistics.files_per_second);

        Ok(ProcessingResult {
            scan_result,
            parsed_files,
            statistics,
        })
    }

    /// Parse a list of media files
    async fn parse_files(&self, files: &[MediaFile]) -> Result<Vec<ParsingResult>> {
        if files.is_empty() {
            return Ok(Vec::new());
        }

        let progress_bar = self.create_progress_bar(files.len());
        let mut parsed_files = Vec::new();

        // Process files in parallel
        let results = stream::iter(files)
            .map(|media_file| async move {
                match self.parser.parse_movie(&media_file.file_path).await {
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
                    Err(e) => {
                        eprintln!(
                            "Failed to parse {}: {}",
                            media_file.file_path.display(),
                            e
                        );
                        Err(e)
                    }
                }
            })
            .buffer_unordered(self.concurrency_limit)
            .collect::<Vec<_>>()
            .await;

        for result in results {
            if let Ok(parsing_result) = result {
                parsed_files.push(parsing_result);
            }
            progress_bar.inc(1);
        }

        progress_bar.finish_with_message("Parsing complete");
        Ok(parsed_files)
    }

    /// Create a progress bar for the processing operation
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

    /// Get the scan result from the last processing operation
    pub fn get_scan_result(&self) -> Option<&ScanResult> {
        // This would need to be implemented with state tracking
        None
    }

    /// Get processing statistics
    pub fn get_statistics(&self) -> Option<&ProcessingStatistics> {
        // This would need to be implemented with state tracking
        None
    }
}

impl Default for MediaProcessor {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback to default configuration if loading fails
            let config = AppConfig::default();
            let scanner = MediaScanner::with_config(config.clone());
            let parser = MovieParser::new();

            Self {
                scanner,
                parser,
                config,
                concurrency_limit: 16,
            }
        })
    }
}