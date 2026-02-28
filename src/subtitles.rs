//! Subtitle companion discovery.
//!
//! Finds subtitle files (.srt, .ass, .sub, .vtt, etc.) adjacent to video
//! files and extracts language/forced suffixes for Plex compatibility.

use std::ffi::OsStr;
use std::path::Path;

use tracing::debug;

use crate::scanner::SUBTITLE_EXTENSIONS;

/// A subtitle companion found next to a video file.
#[derive(Debug, Clone)]
pub struct SubtitleCompanion {
    pub path: std::path::PathBuf,
    /// Language/forced suffix, e.g., ".en", ".zh.forced", or "" for default.
    pub suffix: String,
}

/// Find subtitle files adjacent to a video file.
///
/// Matches subtitles sharing the video's stem, extracting language/forced
/// suffixes. Also checks common subdirectories (Subs/, Subtitles/).
///
/// # Examples
///
/// ```text
/// video: Movie.Name.2020.mkv
/// finds: Movie.Name.2020.srt       → suffix=""
///        Movie.Name.2020.en.srt    → suffix=".en"
///        Movie.Name.2020.zh.forced.srt → suffix=".zh.forced"
/// ```
pub fn find_companions(video_path: &Path) -> Vec<SubtitleCompanion> {
    let video_stem = match video_path.file_stem().and_then(OsStr::to_str) {
        Some(s) => s,
        None => return Vec::new(),
    };
    let parent = match video_path.parent() {
        Some(p) => p,
        None => return Vec::new(),
    };

    let sub_ext_set: std::collections::HashSet<&str> =
        SUBTITLE_EXTENSIONS.iter().copied().collect();

    // Search in same directory + common subtitle subdirectories.
    // Deduplicate by canonical path to avoid double-counting on
    // case-insensitive filesystems (macOS HFS+/APFS).
    let mut search_dirs = vec![parent.to_path_buf()];
    let mut seen_canonical: std::collections::HashSet<std::path::PathBuf> = std::collections::HashSet::new();
    if let Ok(canon) = std::fs::canonicalize(parent) {
        seen_canonical.insert(canon);
    }
    for name in ["Subs", "subs", "Subtitles", "subtitles"] {
        let sub_dir = parent.join(name);
        if sub_dir.is_dir() {
            let dominated = std::fs::canonicalize(&sub_dir)
                .map(|c| !seen_canonical.insert(c))
                .unwrap_or(false);
            if !dominated {
                search_dirs.push(sub_dir);
            }
        }
    }

    let mut companions = Vec::new();

    for dir in &search_dirs {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let name = match path.file_name().and_then(OsStr::to_str) {
                Some(n) => n.to_string(),
                None => continue,
            };

            // Check extension
            let ext = match path.extension().and_then(OsStr::to_str) {
                Some(e) => format!(".{}", e.to_lowercase()),
                None => continue,
            };
            if !sub_ext_set.contains(ext.as_str()) {
                continue;
            }

            // Must start with video stem
            if !name.starts_with(video_stem) {
                continue;
            }

            // Extract suffix between stem and extension
            let remainder = &name[video_stem.len()..name.len() - ext.len()];
            let suffix = remainder.to_string();

            debug!("subtitle companion: {} → suffix={:?}", name, suffix);

            companions.push(SubtitleCompanion {
                path,
                suffix,
            });
        }
    }

    companions.sort_by(|a, b| a.path.cmp(&b.path));
    companions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_subtitle_companions() {
        let tmp = tempfile::tempdir().unwrap();
        let video = tmp.path().join("Movie.2024.mkv");
        fs::write(&video, b"video").unwrap();

        // Create subtitles
        fs::write(tmp.path().join("Movie.2024.srt"), b"sub").unwrap();
        fs::write(tmp.path().join("Movie.2024.en.srt"), b"sub").unwrap();
        fs::write(tmp.path().join("Movie.2024.zh.forced.srt"), b"sub").unwrap();
        fs::write(tmp.path().join("Other.Movie.srt"), b"sub").unwrap();

        let companions = find_companions(&video);
        assert_eq!(companions.len(), 3);

        let suffixes: Vec<&str> = companions.iter().map(|c| c.suffix.as_str()).collect();
        assert!(suffixes.contains(&""));
        assert!(suffixes.contains(&".en"));
        assert!(suffixes.contains(&".zh.forced"));
    }

    #[test]
    fn test_find_companions_in_subs_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let video = tmp.path().join("Movie.mkv");
        fs::write(&video, b"video").unwrap();

        let subs_dir = tmp.path().join("Subs");
        fs::create_dir(&subs_dir).unwrap();
        fs::write(subs_dir.join("Movie.en.srt"), b"sub").unwrap();

        let companions = find_companions(&video);
        assert_eq!(companions.len(), 1);
        assert_eq!(companions[0].suffix, ".en");
    }
}
