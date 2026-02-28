//! Directory scanner — discovers media files recursively.
//!
//! Walks a directory tree, filters by extension, skips samples/extras/hidden
//! files, and yields `MediaFile` records.

use std::collections::HashSet;
use std::path::Path;

use anyhow::{bail, Result};
use tracing::debug;
use walkdir::WalkDir;

use crate::models::{MediaFile, MediaType};

// ── Extension sets ──────────────────────────────────────────────────────────

pub const VIDEO_EXTENSIONS: &[&str] = &[
    ".mkv", ".mp4", ".avi", ".mov", ".wmv", ".flv", ".webm", ".m4v", ".mpg", ".mpeg", ".ts",
    ".vob", ".divx", ".ogv",
];

pub const AUDIO_EXTENSIONS: &[&str] = &[
    ".flac", ".mp3", ".aac", ".ogg", ".opus", ".wma", ".wav", ".m4a", ".alac", ".ape", ".dsf",
    ".dff", ".wv",
];

pub const SUBTITLE_EXTENSIONS: &[&str] = &[".srt", ".sub", ".idx", ".ssa", ".ass", ".vtt"];

/// Default minimum video file size (50 MB) — filters menus/promos.
pub const DEFAULT_MIN_VIDEO_SIZE: u64 = 50 * 1024 * 1024;

// ── Skip patterns ───────────────────────────────────────────────────────────

const SKIP_PATTERNS: &[&str] = &[
    "sample",
    "trailer",
    "extras",
    "featurette",
    "behind the scenes",
    "deleted scenes",
    "interviews",
    "scenes",
    "shorts",
    "tokuten",
];

const EXTRAS_PREFIXES: &[&str] = &[
    "bdmenu",
    "pv",
    "interview",
    "making.of",
    "making_of",
    "behind.the.scenes",
];

const SKIP_DIRS: &[&str] = &[
    "__macosx",
    ".ds_store",
    "@eadir",
    "#recycle",
    ".recycle",
    "lost+found",
    "$recycle.bin",
];

// ── Scanner options ────────────────────────────────────────────────────────

/// Options controlling the scanner.
#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub include_subtitles: bool,
    /// Minimum file size in bytes for video files. Set to 0 to disable.
    pub min_video_size: u64,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            include_subtitles: false,
            min_video_size: DEFAULT_MIN_VIDEO_SIZE,
        }
    }
}

// ── Filter helpers ──────────────────────────────────────────────────────────

fn is_extras_file(stem_lower: &str) -> bool {
    for prefix in EXTRAS_PREFIXES {
        if stem_lower.starts_with(prefix) {
            if *prefix == "pv" {
                let rest = &stem_lower[prefix.len()..];
                if rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_digit()) {
                    return true;
                }
            } else {
                return true;
            }
        }
    }
    false
}

fn is_sample_file(stem_lower: &str) -> bool {
    let separators = [".", "-", "_", " ", "[", "]"];
    for pattern in SKIP_PATTERNS {
        if !stem_lower.contains(pattern) {
            continue;
        }
        // Skip if it appears as a standalone prefix in short filenames
        if stem_lower.starts_with(pattern) && stem_lower.len() < pattern.len() + 5 {
            return true;
        }
        // Skip if preceded by a separator
        for sep in &separators {
            if stem_lower.contains(&format!("{sep}{pattern}")) {
                return true;
            }
        }
    }
    false
}

fn should_skip_dir(dirname: &str) -> bool {
    dirname.starts_with('.') || SKIP_DIRS.contains(&dirname.to_lowercase().as_str())
}

fn detect_type_by_extension(ext: &str) -> MediaType {
    if AUDIO_EXTENSIONS.contains(&ext) {
        return MediaType::Music;
    }
    // Video could be movie or TV — hunch will disambiguate
    MediaType::Unknown
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Scan a directory tree and discover media files.
pub fn scan_directory(path: &Path, options: &ScanOptions) -> Result<Vec<MediaFile>> {
    if !path.exists() {
        bail!("Path does not exist: {}", path.display());
    }
    if !path.is_dir() {
        bail!("Path is not a directory: {}", path.display());
    }

    let mut allowed: HashSet<&str> = HashSet::new();
    for ext in VIDEO_EXTENSIONS {
        allowed.insert(ext);
    }
    for ext in AUDIO_EXTENSIONS {
        allowed.insert(ext);
    }
    if options.include_subtitles {
        for ext in SUBTITLE_EXTENSIONS {
            allowed.insert(ext);
        }
    }

    let video_set: HashSet<&str> = VIDEO_EXTENSIONS.iter().copied().collect();
    let mut results = Vec::new();

    let walker = WalkDir::new(path).follow_links(false).into_iter();

    for entry in walker.filter_entry(|e| {
        // Skip hidden / blacklisted directories (but never the root)
        if e.file_type().is_dir() && e.depth() > 0 {
            return !should_skip_dir(e.file_name().to_str().unwrap_or(""));
        }
        true
    }) {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                debug!("walkdir error: {}", err);
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let file_name = entry.file_name().to_str().unwrap_or("");

        // Skip hidden files
        if file_name.starts_with('.') {
            continue;
        }

        // Extract extension
        let dot_idx = match file_name.rfind('.') {
            Some(i) if i > 0 => i,
            _ => continue,
        };
        let ext = file_name[dot_idx..].to_lowercase();
        if !allowed.contains(ext.as_str()) {
            continue;
        }

        let stem = &file_name[..dot_idx];
        let stem_lower = stem.to_lowercase();

        if is_sample_file(&stem_lower) || is_extras_file(&stem_lower) {
            continue;
        }

        // Size check for video files
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let size = metadata.len();

        if options.min_video_size > 0 && video_set.contains(ext.as_str()) && size < options.min_video_size {
            continue;
        }

        let parent_dir = entry
            .path()
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        results.push(MediaFile {
            source_path: entry.path().to_path_buf(),
            filename: stem.to_string(),
            detected_type: detect_type_by_extension(&ext),
            extension: ext,
            size_bytes: size,
            parent_dir,
        });
    }

    results.sort_by(|a, b| a.source_path.cmp(&b.source_path));
    debug!("scanned {} → {} media files found", path.display(), results.len());
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_dir(tmp: &Path) {
        fs::create_dir_all(tmp.join("movies")).unwrap();
        // Create a "large enough" video file
        let video = tmp.join("movies/Movie.2024.1080p.mkv");
        fs::write(&video, vec![0u8; 100]).unwrap();
        // Create audio file (no size threshold)
        let audio = tmp.join("music.flac");
        fs::write(&audio, b"audio").unwrap();
        // Create sample file (should be skipped)
        let sample = tmp.join("movies/movie-sample.mkv");
        fs::write(&sample, vec![0u8; 100]).unwrap();
        // Hidden file (should be skipped)
        fs::write(tmp.join(".hidden.mkv"), b"x").unwrap();
    }

    #[test]
    fn test_scan_finds_media_files() {
        let tmp = tempfile::tempdir().unwrap();
        create_test_dir(tmp.path());

        let opts = ScanOptions {
            min_video_size: 0, // Disable size check for tests
            ..Default::default()
        };
        let files = scan_directory(tmp.path(), &opts).unwrap();

        let names: Vec<String> = files.iter().map(|f| f.full_name()).collect();
        assert!(names.contains(&"Movie.2024.1080p.mkv".to_string()));
        assert!(names.contains(&"music.flac".to_string()));
        assert!(!names.iter().any(|n| n.contains("sample")));
        assert!(!names.iter().any(|n| n.starts_with('.')));
    }

    #[test]
    fn test_scan_music_detected_as_music() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("song.flac"), b"audio").unwrap();

        let files = scan_directory(tmp.path(), &Default::default()).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].detected_type, MediaType::Music);
    }

    #[test]
    fn test_scan_nonexistent_dir_errors() {
        let result = scan_directory(Path::new("/nonexistent_xyz"), &Default::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_extras_prefix_filtering() {
        assert!(is_extras_file("bdmenu"));
        assert!(is_extras_file("pv1"));
        assert!(is_extras_file("pv"));
        assert!(!is_extras_file("pvris")); // band name, not extras
        assert!(is_extras_file("interview_cast"));
    }
}
