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
    pub fn generate_plex_path(
        &self,
        media_file: &MediaFile,
        metadata: &crate::types::MediaMetadata,
    ) -> Result<PathBuf> {
        // Create combined display title: Chinese Title [English Title]
        let display_title = if let (Some(english_title), Some(original_title)) =
            (&metadata.title, &metadata.original_title)
        {
            if english_title != original_title {
                // Combine: Chinese Title [English Title]
                format!("{} [{}]", original_title, english_title)
            } else {
                // Use English title when they're the same
                english_title.clone()
            }
        } else {
            // Fall back to available title
            metadata
                .title
                .as_ref()
                .or_else(|| metadata.original_title.as_ref())
                .ok_or_else(|| anyhow::anyhow!("No title found in metadata"))?
                .clone()
        };

        // Clean title for directory name
        let clean_title = self.clean_title_for_directory(&display_title);

        // Create directory name with or without year
        let dir_name = if let Some(year) = metadata.year {
            format!("{} ({})", clean_title, year)
        } else {
            // Fallback: use current year or create "Unknown Year" directory
            format!("{} (Unknown Year)", clean_title)
        };

        // Get the base directory (the directory being organized)
        // This should be the root of the movie collection, not the release folder
        let base_dir = self.get_base_directory(&media_file.file_path)?;

        // Create new directory path under the base directory
        let new_dir = base_dir.join(dir_name);

        // Generate new filename
        let new_filename = self.generate_plex_filename(media_file, metadata)?;

        // Full new path
        let new_path = new_dir.join(new_filename);

        Ok(new_path)
    }

    /// Get the base directory for organization
    /// This should be the root of the movie collection, not individual release folders
    fn get_base_directory(&self, file_path: &Path) -> Result<PathBuf> {
        // Start from the file's parent directory
        let mut current_dir = file_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("No parent directory found"))?;

        // Walk up the directory tree to find the appropriate base directory
        // We want to organize at the movie collection level, not preserve release folder structure
        while let Some(parent) = current_dir.parent() {
            // Check if this looks like a movie collection directory
            // Common patterns: "Movies", "movie", "Japanese", "English", etc.
            let dir_name = current_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_lowercase();

            // If this looks like a collection directory, use it as the base
            if self.is_collection_directory(&dir_name) {
                return Ok(current_dir.to_path_buf());
            }

            current_dir = parent;
        }

        // If we can't find a collection directory, use the immediate parent
        // This is a fallback for edge cases
        Ok(file_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("No parent directory found"))?
            .to_path_buf())
    }

    /// Check if a directory name looks like a movie collection directory
    fn is_collection_directory(&self, dir_name: &str) -> bool {
        let collection_patterns = [
            "movies",
            "movie",
            "films",
            "film",
            "cinema",
            "japanese",
            "english",
            "chinese",
            "korean",
            "anime",
            "action",
            "comedy",
            "drama",
            "horror",
            "sci-fi",
            "documentary",
            "documentaries",
        ];

        collection_patterns.contains(&dir_name)
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
        // Create combined display title: Chinese Title [English Title]
        let display_title = if let (Some(english_title), Some(original_title)) =
            (&metadata.title, &metadata.original_title)
        {
            if english_title != original_title {
                // Combine: Chinese Title [English Title]
                format!("{} [{}]", original_title, english_title)
            } else {
                // Use English title when they're the same
                english_title.clone()
            }
        } else {
            // Fall back to available title
            metadata
                .title
                .as_ref()
                .or_else(|| metadata.original_title.as_ref())
                .ok_or_else(|| anyhow::anyhow!("No title found in metadata"))?
                .clone()
        };

        // Get file extension
        let extension = media_file
            .file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("mkv");

        // Build filename with or without year
        let mut filename_parts = vec![display_title.to_string()];

        if let Some(year) = metadata.year {
            filename_parts.push(format!("({})", year));
        }

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

        // Move the main media file
        fs::rename(original_path, new_path).context("Failed to move file")?;

        // Handle metadata files (like .nfo files)
        self.move_metadata_files(original_path, new_path)?;

        Ok(())
    }

    /// Move metadata files associated with the media file
    fn move_metadata_files(&self, original_path: &Path, new_path: &Path) -> Result<()> {
        use crate::metadata_extractor::MetadataExtractor;

        // Get metadata files that should be moved
        let metadata_files = MetadataExtractor::get_metadata_files_to_move(original_path)?;

        for metadata_file in metadata_files {
            if metadata_file.exists() {
                // Create new path for metadata file
                let new_metadata_path =
                    new_path.with_extension(metadata_file.extension().unwrap_or_default());

                // Move the metadata file
                if let Err(e) = fs::rename(&metadata_file, &new_metadata_path) {
                    eprintln!(
                        "Warning: Failed to move metadata file {}: {}",
                        metadata_file.display(),
                        e
                    );
                } else {
                    println!(
                        "  📄 Moved metadata file: {} -> {}",
                        metadata_file.display(),
                        new_metadata_path.display()
                    );
                }
            }
        }

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

        // Display trade-off summary
        if result.statistics.failed_files > 0 {
            println!("\n📋 TRADE-OFF SUMMARY:");
            println!(
                "   • {} files organized successfully",
                result.statistics.organized_files
            );
            println!(
                "   • {} files skipped/failed to ensure accuracy",
                result.statistics.failed_files
            );
            println!(
                "   • Success rate: {:.1}%",
                result.statistics.success_rate * 100.0
            );
            println!();
            println!("💡 TIP: Skipped files are often due to:");
            println!("   • Technical terms in filenames (e.g., 'DualAudio', 'iNT', 'TLF')");
            println!("   • Movies not found in TMDB database");
            println!("   • Year/title mismatches between filename and TMDB data");
            println!();
            println!("🔧 To improve coverage:");
            println!("   • Use --min-confidence 0.5 for more permissive matching");
            println!("   • Manually clean filenames before organizing");
            println!("   • Check TMDB for correct movie titles and years");
            println!();
            println!("📖 For more details, see: project/CURRENT_LIMITATIONS.md");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{MediaFile, MediaMetadata};

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
        let media_file = MediaFile {
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

    #[test]
    fn test_is_collection_directory() {
        let organizer = Organizer::new(true, None);

        // Should be collection directories
        assert!(organizer.is_collection_directory("movies"));
        assert!(organizer.is_collection_directory("movie"));
        assert!(organizer.is_collection_directory("japanese"));
        assert!(organizer.is_collection_directory("anime"));
        assert!(organizer.is_collection_directory("action"));

        // Should not be collection directories
        assert!(
            !organizer
                .is_collection_directory("5.Centimeters.Per.Second.2007.BluRay.1080p-ted423@FRDS")
        );
        assert!(!organizer.is_collection_directory("Extras"));
        assert!(!organizer.is_collection_directory("Sample"));
        assert!(!organizer.is_collection_directory("random_folder"));
    }

    #[test]
    fn test_get_base_directory() {
        let organizer = Organizer::new(true, None);

        // Test with a typical movie collection structure
        let file_path = PathBuf::from(
            "/Volumes/media/movie/Japanese/5.Centimeters.Per.Second.2007.BluRay.1080p-ted423@FRDS/movie.mkv",
        );
        let base_dir = organizer.get_base_directory(&file_path).unwrap();
        assert_eq!(base_dir, PathBuf::from("/Volumes/media/movie/Japanese"));

        // Test with a simpler structure
        let file_path = PathBuf::from("/Movies/Action/Iron.Man.2008.BluRay.1080p/movie.mkv");
        let base_dir = organizer.get_base_directory(&file_path).unwrap();
        assert_eq!(base_dir, PathBuf::from("/Movies/Action"));

        // Test with no collection directory found
        let file_path = PathBuf::from("/random/path/to/movie.mkv");
        let base_dir = organizer.get_base_directory(&file_path).unwrap();
        assert_eq!(base_dir, PathBuf::from("/random/path/to"));
    }
}
