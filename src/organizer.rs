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

        for parsing_result in &scan_result.parsed_files {
            match self.organize_single_file(parsing_result).await {
                Ok(organized_file) => {
                    organized_files.push(organized_file);
                }
                Err(error) => {
                    let failed_org = FailedOrganization {
                        media_file: parsing_result.media_file.clone(),
                        error: error.to_string(),
                        failed_at: Utc::now(),
                    };
                    failed_files.push(failed_org);
                }
            }
        }

        let duration = start_time.elapsed();
        let total_files = scan_result.parsed_files.len() as u32;
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

        let result = OrganizationResult {
            operation_id,
            timestamp: Utc::now(),
            organized_files,
            failed_files,
            statistics,
        };

        self.display_organization_results(&result);

        Ok(result)
    }

    /// Organize a single file
    async fn organize_single_file(&self, parsing_result: &ParsingResult) -> Result<OrganizedFile> {
        let media_file = &parsing_result.media_file;
        let metadata = &parsing_result.parsed_metadata;

        // Generate new file path according to Plex conventions
        let new_path = self.generate_plex_path(media_file, metadata)?;

        // Create backup if needed
        if let Some(backup_dir) = &self.backup_dir {
            self.create_backup(&media_file.file_path, backup_dir)?;
        }

        // Perform the actual organization
        if !self.dry_run {
            self.perform_file_organization(&media_file.file_path, &new_path)?;
        }

        let organized_file = OrganizedFile {
            original_path: media_file.file_path.clone(),
            new_path: new_path.clone(),
            media_file: media_file.clone(),
            parsed_metadata: metadata.clone(),
            dry_run: self.dry_run,
        };

        Ok(organized_file)
    }

    /// Generate Plex-compliant file path
    fn generate_plex_path(
        &self,
        media_file: &MediaFile,
        metadata: &crate::types::MediaMetadata,
    ) -> Result<PathBuf> {
        let title = metadata
            .title
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No title found in metadata"))?;

        let year = metadata
            .year
            .ok_or_else(|| anyhow::anyhow!("No year found in metadata"))?;

        // Clean title for directory name
        let clean_title = self.clean_title_for_directory(title);

        // Create directory name: "Movie Name (Year)"
        let dir_name = format!("{} ({})", clean_title, year);

        // Get the parent directory of the original file
        let parent_dir = media_file
            .file_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("No parent directory found"))?;

        // Create new directory path
        let new_dir = parent_dir.join(dir_name);

        // Generate new filename
        let new_filename = self.generate_plex_filename(media_file, metadata)?;

        // Full new path
        let new_path = new_dir.join(new_filename);

        Ok(new_path)
    }

    /// Clean title for use in directory name
    fn clean_title_for_directory(&self, title: &str) -> String {
        // Remove invalid characters for directory names
        let mut cleaned = title.to_string();

        // Replace invalid characters
        cleaned = cleaned.replace(['<', '>', ':', '"', '|', '?', '*'], "");

        // Replace backslashes and forward slashes
        cleaned = cleaned.replace(['\\', '/'], " ");

        // Trim whitespace
        cleaned.trim().to_string()
    }

    /// Generate Plex-compliant filename
    fn generate_plex_filename(
        &self,
        media_file: &MediaFile,
        metadata: &crate::types::MediaMetadata,
    ) -> Result<String> {
        let title = metadata
            .title
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No title found in metadata"))?;

        let year = metadata
            .year
            .ok_or_else(|| anyhow::anyhow!("No year found in metadata"))?;

        // Get file extension
        let extension = media_file
            .file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("mkv");

        // Build filename: "Movie Name (Year) Quality Source.ext"
        let mut filename_parts = vec![title.to_string(), format!("({})", year)];

        // Add quality if available
        if let Some(quality) = &metadata.quality {
            filename_parts.push(quality.to_string());
        }

        // Add source if available
        if let Some(source) = &metadata.source {
            filename_parts.push(source.to_string());
        }

        let filename = filename_parts.join(" ");
        let full_filename = format!("{}.{}", filename, extension);

        Ok(full_filename)
    }

    /// Create backup of original file
    fn create_backup(&self, original_path: &Path, backup_dir: &Path) -> Result<()> {
        if !backup_dir.exists() {
            fs::create_dir_all(backup_dir).context("Failed to create backup directory")?;
        }

        let filename = original_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("No filename found"))?;

        let backup_path = backup_dir.join(filename);

        fs::copy(original_path, &backup_path).context("Failed to create backup file")?;

        Ok(())
    }

    /// Perform actual file organization
    fn perform_file_organization(&self, original_path: &Path, new_path: &Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = new_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent).context("Failed to create directory")?;
        }

        // Move the file
        fs::rename(original_path, new_path).context("Failed to move file")?;

        Ok(())
    }

    /// Display organization results
    fn display_organization_results(&self, result: &OrganizationResult) {
        println!("\n📊 Organization Results:");
        println!("Operation ID: {}", result.operation_id);
        println!("Timestamp: {}", result.timestamp);
        println!("Total files: {}", result.statistics.total_files);
        println!("Organized: {}", result.statistics.organized_files);
        println!("Failed: {}", result.statistics.failed_files);
        println!(
            "Success rate: {:.1}%",
            result.statistics.success_rate * 100.0
        );
        println!("Duration: {:.2}s", result.statistics.duration_seconds);

        if !result.organized_files.is_empty() {
            println!("\n✅ Organized Files:");
            for (i, file) in result.organized_files.iter().enumerate() {
                println!(
                    "{}. {} -> {}",
                    i + 1,
                    file.original_path.display(),
                    file.new_path.display()
                );
            }
        }

        if !result.failed_files.is_empty() {
            println!("\n❌ Failed Files:");
            for failed in &result.failed_files {
                println!("- {}: {}", failed.media_file.file_name, failed.error);
            }
        }

        if self.dry_run {
            println!("\n🔍 DRY-RUN COMPLETE: No actual changes were made");
        } else {
            println!("\n✅ Organization complete!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{MediaFile, MediaMetadata, ParsingResult, ParsingStrategy};
    use tempfile::tempdir;

    #[test]
    fn test_clean_title_for_directory() {
        let organizer = Organizer::new(true, None);

        assert_eq!(organizer.clean_title_for_directory("Iron Man"), "Iron Man");
        assert_eq!(
            organizer.clean_title_for_directory("Iron Man: The Movie"),
            "Iron Man The Movie"
        );
        assert_eq!(
            organizer.clean_title_for_directory("Movie/With/Slashes"),
            "Movie With Slashes"
        );
        assert_eq!(
            organizer.clean_title_for_directory("Movie<With>Invalid:Chars"),
            "MovieWithInvalidChars"
        );
    }

    #[test]
    fn test_generate_plex_filename() {
        let organizer = Organizer::new(true, None);
        let mut media_file = MediaFile {
            id: "test".to_string(),
            file_path: PathBuf::from("movie.mkv"),
            file_name: "movie.mkv".to_string(),
            file_size: 1000,
            media_type: crate::types::MediaType::Movie,
            content_hash: "hash".to_string(),
            last_modified: Utc::now(),
            metadata: MediaMetadata::default(),
        };

        let mut metadata = MediaMetadata::default();
        metadata.title = Some("Iron Man".to_string());
        metadata.year = Some(2008);
        metadata.quality = Some("1080p".to_string());
        metadata.source = Some("BluRay".to_string());

        let filename = organizer
            .generate_plex_filename(&media_file, &metadata)
            .unwrap();
        assert_eq!(filename, "Iron Man (2008) 1080p BluRay.mkv");
    }

    #[tokio::test]
    async fn test_organizer_creation() {
        let organizer = Organizer::new(true, None);
        assert!(organizer.dry_run);
        assert!(organizer.backup_dir.is_none());
    }
}
