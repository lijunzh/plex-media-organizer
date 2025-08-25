//! File organization module for Plex naming conventions
//!
//! This module handles renaming and organizing media files according to Plex standards
//! with safety features including dry-run mode and rollback capability.

use crate::types::{MediaFile, ParsingResult, ScanResult};
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Organization operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationResult {
    /// Unique operation ID
    pub operation_id: String,
    /// When the operation was performed
    pub timestamp: chrono::DateTime<Utc>,
    /// Files that were organized
    pub organized_files: Vec<OrganizedFile>,
    /// Files that failed to organize
    pub failed_files: Vec<FailedOrganization>,
    /// Statistics about the operation
    pub statistics: OrganizationStatistics,
}

/// Information about an organized file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizedFile {
    /// Original file path
    pub original_path: PathBuf,
    /// New file path after organization
    pub new_path: PathBuf,
    /// Original file information
    pub media_file: MediaFile,
    /// Parsed metadata
    pub parsed_metadata: crate::types::MediaMetadata,
    /// Whether this was a dry-run (no actual changes)
    pub dry_run: bool,
}

/// Information about a failed organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedOrganization {
    /// The media file that failed
    pub media_file: MediaFile,
    /// Error that occurred
    pub error: String,
    /// When the failure occurred
    pub failed_at: chrono::DateTime<Utc>,
}

/// Statistics about an organization operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStatistics {
    /// Total files processed
    pub total_files: u32,
    /// Files successfully organized
    pub organized_files: u32,
    /// Files that failed to organize
    pub failed_files: u32,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f32,
    /// Operation duration in seconds
    pub duration_seconds: f64,
}

/// File organizer for Plex naming conventions
#[derive(Debug)]
pub struct Organizer {
    /// Whether to perform dry-run (no actual changes)
    dry_run: bool,
    /// Backup directory for rollback files
    backup_dir: Option<PathBuf>,
}

impl Organizer {
    /// Create a new organizer
    pub fn new(dry_run: bool, backup_dir: Option<PathBuf>) -> Self {
        Self {
            dry_run,
            backup_dir,
        }
    }

    /// Organize files from a scan result
    pub async fn organize_scan_result(
        &self,
        scan_result: &ScanResult,
    ) -> Result<OrganizationResult> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4().to_string();

        println!("🎬 Starting file organization...");
        if self.dry_run {
            println!("🔍 DRY-RUN MODE: No actual changes will be made");
        }

        let mut organized_files = Vec::new();
        let mut failed_files = Vec::new();

        for parsed_file in &scan_result.parsed_files {
            match self.organize_single_file(parsed_file).await {
                Ok(organized_file) => {
                    organized_files.push(organized_file);
                }
                Err(e) => {
                    failed_files.push(FailedOrganization {
                        media_file: parsed_file.media_file.clone(),
                        error: e.to_string(),
                        failed_at: Utc::now(),
                    });
                }
            }
        }

        let duration = start_time.elapsed();
        let statistics = OrganizationStatistics {
            total_files: scan_result.parsed_files.len() as u32,
            organized_files: organized_files.len() as u32,
            failed_files: failed_files.len() as u32,
            success_rate: if scan_result.parsed_files.is_empty() {
                0.0
            } else {
                organized_files.len() as f32 / scan_result.parsed_files.len() as f32
            },
            duration_seconds: duration.as_secs_f64(),
        };

        let result = OrganizationResult {
            operation_id,
            timestamp: Utc::now(),
            organized_files,
            failed_files,
            statistics: statistics.clone(),
        };

        // Print summary
        println!("✅ Organization completed!");
        println!("   • Total files: {}", statistics.total_files);
        println!("   • Organized: {}", statistics.organized_files);
        println!("   • Failed: {}", statistics.failed_files);
        println!("   • Success rate: {:.1}%", statistics.success_rate * 100.0);
        println!("   • Duration: {:.2}s", statistics.duration_seconds);

        Ok(result)
    }

    /// Organize a single file according to Plex naming conventions
    async fn organize_single_file(&self, parsed_file: &ParsingResult) -> Result<OrganizedFile> {
        let original_path = &parsed_file.media_file.file_path;
        let new_path = self.generate_plex_path(parsed_file)?;

        // Create backup if needed
        if !self.dry_run && self.backup_dir.is_some() {
            self.create_backup(original_path).await?;
        }

        // Perform the actual organization
        if !self.dry_run {
            self.perform_file_operation(original_path, &new_path).await?;
        }

        Ok(OrganizedFile {
            original_path: original_path.clone(),
            new_path,
            media_file: parsed_file.media_file.clone(),
            parsed_metadata: parsed_file.parsed_metadata.clone(),
            dry_run: self.dry_run,
        })
    }

    /// Generate a Plex-compliant file path
    fn generate_plex_path(&self, parsed_file: &ParsingResult) -> Result<PathBuf> {
        let metadata = &parsed_file.parsed_metadata;
        let title = metadata.title.as_ref().ok_or_else(|| {
            anyhow::anyhow!("No title found for file: {}", parsed_file.media_file.file_name)
        })?;

        let year = metadata.year.ok_or_else(|| {
            anyhow::anyhow!("No year found for file: {}", parsed_file.media_file.file_name)
        })?;

        // Get the original file extension
        let extension = parsed_file
            .media_file
            .file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("mkv");

        // Build the new filename
        let mut filename_parts = vec![title.clone()];

        // Add year
        filename_parts.push(format!("({})", year));

        // Add quality if available
        if let Some(quality) = &metadata.quality {
            filename_parts.push(quality.clone());
        }

        // Add source if available
        if let Some(source) = &metadata.source {
            filename_parts.push(source.clone());
        }

        // Join parts and add extension
        let filename = format!("{}.{}", filename_parts.join(" "), extension);

        // Create the directory structure
        let mut new_path = parsed_file.media_file.file_path.parent().unwrap().to_path_buf();
        new_path.push(filename);

        Ok(new_path)
    }

    /// Create a backup of the original file
    async fn create_backup(&self, file_path: &Path) -> Result<()> {
        if let Some(backup_dir) = &self.backup_dir {
            let backup_path = backup_dir.join(file_path.file_name().unwrap());
            
            // Ensure backup directory exists
            if let Some(parent) = backup_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Copy the file to backup location
            fs::copy(file_path, &backup_path)
                .with_context(|| format!("Failed to create backup: {}", backup_path.display()))?;
        }
        Ok(())
    }

    /// Perform the actual file operation (move/rename)
    async fn perform_file_operation(&self, source: &Path, destination: &Path) -> Result<()> {
        // Ensure destination directory exists
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Move the file
        fs::rename(source, destination)
            .with_context(|| {
                format!(
                    "Failed to move file from {} to {}",
                    source.display(),
                    destination.display()
                )
            })?;

        Ok(())
    }
}