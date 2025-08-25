//! Media file organizer for Plex naming conventions
//!
//! This module handles the organization and renaming of media files according to
//! Plex naming standards. It does not handle scanning or parsing.

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

/// Media file organizer for Plex naming conventions
#[derive(Debug)]
pub struct MediaOrganizer {
    /// Whether to perform dry-run (no actual changes)
    dry_run: bool,
    /// Backup directory for rollback files
    backup_dir: Option<PathBuf>,
    /// Target directory for organized files
    target_dir: Option<PathBuf>,
}

impl MediaOrganizer {
    /// Create a new media organizer
    pub fn new(dry_run: bool, backup_dir: Option<PathBuf>) -> Self {
        Self {
            dry_run,
            backup_dir,
            target_dir: None,
        }
    }

    /// Create a new media organizer with target directory
    pub fn with_target_dir(
        dry_run: bool,
        backup_dir: Option<PathBuf>,
        target_dir: PathBuf,
    ) -> Self {
        Self {
            dry_run,
            backup_dir,
            target_dir: Some(target_dir),
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

        for media_file in &scan_result.files {
            match self.organize_single_file(media_file).await {
                Ok(organized_file) => {
                    organized_files.push(organized_file);
                }
                Err(e) => {
                    failed_files.push(FailedOrganization {
                        media_file: media_file.clone(),
                        error: e.to_string(),
                        failed_at: Utc::now(),
                    });
                }
            }
        }

        let duration = start_time.elapsed();
        let total_files = scan_result.files.len() as u32;
        let organized_count = organized_files.len() as u32;
        let failed_count = failed_files.len() as u32;
        let success_rate = if total_files > 0 {
            organized_count as f32 / total_files as f32
        } else {
            0.0
        };

        let statistics = OrganizationStatistics {
            total_files,
            organized_files: organized_count,
            failed_files: failed_count,
            success_rate,
            duration_seconds: duration.as_secs_f64(),
        };

        Ok(OrganizationResult {
            operation_id,
            timestamp: Utc::now(),
            organized_files,
            failed_files,
            statistics,
        })
    }

    /// Organize files from parsing results
    pub async fn organize_parsing_results(
        &self,
        parsing_results: &[ParsingResult],
    ) -> Result<OrganizationResult> {
        let start_time = std::time::Instant::now();
        let operation_id = Uuid::new_v4().to_string();

        println!("🎬 Starting file organization from parsing results...");
        if self.dry_run {
            println!("🔍 DRY-RUN MODE: No actual changes will be made");
        }

        let mut organized_files = Vec::new();
        let mut failed_files = Vec::new();

        for parsing_result in parsing_results {
            match self.organize_parsed_file(parsing_result).await {
                Ok(organized_file) => {
                    organized_files.push(organized_file);
                }
                Err(e) => {
                    failed_files.push(FailedOrganization {
                        media_file: parsing_result.media_file.clone(),
                        error: e.to_string(),
                        failed_at: Utc::now(),
                    });
                }
            }
        }

        let duration = start_time.elapsed();
        let total_files = parsing_results.len() as u32;
        let organized_count = organized_files.len() as u32;
        let failed_count = failed_files.len() as u32;
        let success_rate = if total_files > 0 {
            organized_count as f32 / total_files as f32
        } else {
            0.0
        };

        let statistics = OrganizationStatistics {
            total_files,
            organized_files: organized_count,
            failed_files: failed_count,
            success_rate,
            duration_seconds: duration.as_secs_f64(),
        };

        Ok(OrganizationResult {
            operation_id,
            timestamp: Utc::now(),
            organized_files,
            failed_files,
            statistics,
        })
    }

    /// Organize a single media file
    async fn organize_single_file(&self, media_file: &MediaFile) -> Result<OrganizedFile> {
        let new_path = self.generate_plex_path(media_file)?;
        
        if !self.dry_run {
            self.perform_file_operation(&media_file.file_path, &new_path).await?;
        }

        Ok(OrganizedFile {
            original_path: media_file.file_path.clone(),
            new_path,
            media_file: media_file.clone(),
            parsed_metadata: media_file.metadata.clone(),
            dry_run: self.dry_run,
        })
    }

    /// Organize a parsed file
    async fn organize_parsed_file(&self, parsing_result: &ParsingResult) -> Result<OrganizedFile> {
        let new_path = self.generate_plex_path_from_parsed(parsing_result)?;
        
        if !self.dry_run {
            self.perform_file_operation(&parsing_result.media_file.file_path, &new_path).await?;
        }

        Ok(OrganizedFile {
            original_path: parsing_result.media_file.file_path.clone(),
            new_path,
            media_file: parsing_result.media_file.clone(),
            parsed_metadata: parsing_result.parsed_metadata.clone(),
            dry_run: self.dry_run,
        })
    }

    /// Generate Plex-compliant path for a media file
    fn generate_plex_path(&self, media_file: &MediaFile) -> Result<PathBuf> {
        // Basic implementation - in a full implementation, this would use
        // the parsed metadata to generate proper Plex naming
        let filename = media_file.file_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

        let new_filename = self.clean_filename(filename);
        
        if let Some(target_dir) = &self.target_dir {
            Ok(target_dir.join(new_filename))
        } else {
            Ok(media_file.file_path.parent()
                .unwrap_or_else(|| Path::new("."))
                .join(new_filename))
        }
    }

    /// Generate Plex-compliant path from parsed result
    fn generate_plex_path_from_parsed(&self, parsing_result: &ParsingResult) -> Result<PathBuf> {
        // This would use the parsed metadata to generate proper Plex naming
        // For now, use a basic implementation
        self.generate_plex_path(&parsing_result.media_file)
    }

    /// Clean filename for Plex compatibility
    fn clean_filename(&self, filename: &str) -> String {
        // Basic filename cleaning - remove invalid characters
        filename
            .chars()
            .map(|c| if c.is_alphanumeric() || c.is_whitespace() || ".-_()[]".contains(c) { c } else { '_' })
            .collect()
    }

    /// Perform the actual file operation (move/rename)
    async fn perform_file_operation(&self, source: &Path, destination: &Path) -> Result<()> {
        // Create destination directory if it doesn't exist
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Backup file if backup directory is specified
        if let Some(backup_dir) = &self.backup_dir {
            let backup_path = backup_dir.join(source.file_name().unwrap());
            fs::copy(source, &backup_path)
                .with_context(|| format!("Failed to backup file: {}", source.display()))?;
        }

        // Move/rename the file
        fs::rename(source, destination)
            .with_context(|| format!("Failed to move file from {} to {}", source.display(), destination.display()))?;

        Ok(())
    }

    /// Set the target directory for organized files
    pub fn set_target_dir(&mut self, target_dir: PathBuf) {
        self.target_dir = Some(target_dir);
    }

    /// Get the current target directory
    pub fn target_dir(&self) -> Option<&PathBuf> {
        self.target_dir.as_ref()
    }

    /// Check if dry-run mode is enabled
    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }
}