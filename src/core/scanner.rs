//! File discovery and media file filtering

use crate::types::{MediaFile, MediaType};
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Media file scanner focused on file discovery and filtering
#[derive(Debug)]
pub struct Scanner {
    /// Whether to use network drive optimizations
    network_mode: bool,
    /// Batch size for network operations
    batch_size: usize,
}

impl Scanner {
    /// Create a new scanner with default settings
    pub fn new() -> Self {
        Self {
            network_mode: false,
            batch_size: 100,
        }
    }

    /// Create a scanner optimized for network drives
    pub fn for_network_drive() -> Self {
        Self {
            network_mode: true,
            batch_size: 50, // Smaller batches for network drives
        }
    }

    /// Set network mode
    pub fn set_network_mode(&mut self, enabled: bool) {
        self.network_mode = enabled;
        if enabled {
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

    /// Discover all files in a directory with network optimizations
    pub fn discover_files(&self, directory: &Path) -> Result<Vec<PathBuf>> {
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
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .count() > 10
    }

    /// Discover files sequentially (safer for network drives)
    fn discover_files_sequential(&self, directory: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let walker = WalkDir::new(directory)
            .follow_links(false)
            .same_file_system(true);

        for entry in walker {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        files.push(entry.path().to_path_buf());
                    }
                }
                Err(e) => {
                    // Log but continue for network drives
                    if self.network_mode {
                        eprintln!("Warning: Failed to access file: {}", e);
                    } else {
                        return Err(e.into());
                    }
                }
            }
        }

        Ok(files)
    }

    /// Discover files in parallel (faster for local drives)
    fn discover_files_parallel(&self, directory: &Path) -> Result<Vec<PathBuf>> {
        use rayon::prelude::*;

        let entries: Vec<_> = WalkDir::new(directory)
            .follow_links(false)
            .same_file_system(true)
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

    /// Filter files to only include media files
    pub fn filter_media_files(&self, files: &[PathBuf]) -> Result<Vec<MediaFile>> {
        let mut media_files = Vec::new();

        for file_path in files {
            if let Some(media_type) = self.detect_media_type(file_path) {
                let media_file = self.create_media_file(file_path, media_type)?;
                media_files.push(media_file);
            }
        }

        Ok(media_files)
    }

    /// Detect if a file is a media file and return its type
    fn detect_media_type(&self, file_path: &Path) -> Option<MediaType> {
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            // Video files
            "mkv" | "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "3gp" | "ogv" => {
                Some(MediaType::Video)
            }
            // Audio files
            "mp3" | "flac" | "wav" | "aac" | "ogg" | "wma" | "m4a" | "opus" => Some(MediaType::Audio),
            // Subtitle files
            "srt" | "ass" | "ssa" | "sub" | "idx" | "vtt" => Some(MediaType::Subtitle),
            // Image files (for potential future use)
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" => Some(MediaType::Image),
            _ => None,
        }
    }

    /// Create a MediaFile from a path and media type
    fn create_media_file(&self, file_path: &Path, media_type: MediaType) -> Result<MediaFile> {
        let metadata = std::fs::metadata(file_path)?;
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let content_hash = self.calculate_content_hash(file_path)?;

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

    /// Calculate a simple content hash for change detection
    fn calculate_content_hash(&self, file_path: &Path) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let metadata = std::fs::metadata(file_path)?;
        let mut hasher = DefaultHasher::new();
        
        // Hash file size and modification time for quick change detection
        metadata.len().hash(&mut hasher);
        metadata.modified()?.hash(&mut hasher);
        
        Ok(format!("{:x}", hasher.finish()))
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}