//! Filename parser — wraps hunch for video, uses simple regex for music.
//!
//! This is the core integration point with hunch. For video files, hunch
//! does all the heavy lifting. For music, we use simple linear regex
//! patterns as a placeholder until a proper music parser is built.

use regex::Regex;
use std::sync::LazyLock;
use tracing::debug;

use crate::models::{MediaFile, MediaType, ParsedMedia};
use crate::scanner::AUDIO_EXTENSIONS;

// ── Music placeholder regex ────────────────────────────────────────────────

/// Matches: "01 - Track Title" or "01. Track Title"
static TRACK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<num>\d{1,3})\s*[.\-]\s*(?P<title>.+)$").unwrap());

/// Matches: "Artist - Album (Year)" directory pattern
static ALBUM_DIR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<artist>.+?)\s*-\s*(?P<album>.+?)(?:\s*\((?P<year>\d{4})\))?$").unwrap()
});

// ── Public API ──────────────────────────────────────────────────────────────

/// Parse a media file into structured metadata.
///
/// Routes to hunch for video content, or simple regex for music.
pub fn parse_media_file(file: &MediaFile) -> ParsedMedia {
    let full_name = file.full_name();

    if AUDIO_EXTENSIONS.contains(&file.extension.as_str()) {
        return parse_music(file);
    }

    parse_video(&full_name)
}

/// Parse a video filename using hunch.
fn parse_video(filename: &str) -> ParsedMedia {
    if filename.is_empty() {
        return ParsedMedia {
            raw_filename: filename.to_string(),
            ..Default::default()
        };
    }

    let result = hunch::hunch(filename);

    let media_type = match result.media_type() {
        Some(hunch::MediaType::Movie) => MediaType::Movie,
        Some(hunch::MediaType::Episode) => MediaType::Tv,
        None => MediaType::Unknown,
    };

    let title = result.title().unwrap_or("").to_string();
    let year = result.year();
    let season = result.season();
    let episode = result.episode();
    let episode_title = result.episode_title().map(String::from);
    let release_group = result.release_group().map(String::from);
    let source_tag = result.source().map(String::from);
    let quality = build_quality_string(&result);

    // Compute confidence from how many fields hunch populated
    let confidence = compute_confidence(&title, year, media_type, season, episode);

    debug!(
        "parsed {filename:?} → title={title:?} type={media_type} year={year:?} \
         S{season:?}E{episode:?} conf={confidence:.0}"
    );

    ParsedMedia {
        title,
        year,
        season,
        episode,
        episode_end: None, // hunch returns single episode; multi-ep is future work
        episode_title,
        media_type,
        release_group,
        quality,
        source_tag,
        language: result
            .first(hunch::matcher::span::Property::Language)
            .map(String::from),
        confidence,
        raw_filename: filename.to_string(),
        artist: None,
        album: None,
        track_number: None,
        track_title: None,
    }
}

/// Parse a music file using simple regex (placeholder).
fn parse_music(file: &MediaFile) -> ParsedMedia {
    let mut parsed = ParsedMedia {
        media_type: MediaType::Music,
        raw_filename: file.full_name(),
        confidence: 40.0,
        ..Default::default()
    };

    // Try to extract track number and title from filename
    if let Some(caps) = TRACK_RE.captures(&file.filename) {
        parsed.track_number = caps
            .name("num")
            .and_then(|m| m.as_str().parse().ok());
        parsed.track_title = caps.name("title").map(|m| m.as_str().to_string());
        parsed.title = parsed
            .track_title
            .clone()
            .unwrap_or_else(|| file.filename.clone());
        parsed.confidence = 50.0;
    } else {
        parsed.title = file.filename.replace(['.', '_'], " ");
    }

    // Try to extract artist/album from parent directory
    if let Some(caps) = ALBUM_DIR_RE.captures(&file.parent_dir) {
        parsed.artist = caps.name("artist").map(|m| m.as_str().trim().to_string());
        parsed.album = caps.name("album").map(|m| m.as_str().trim().to_string());
        parsed.year = caps
            .name("year")
            .and_then(|m| m.as_str().parse().ok());
        parsed.confidence += 20.0;
    }

    debug!(
        "parsed music {:?} → artist={:?} album={:?} track={:?} conf={:.0}",
        file.full_name(),
        parsed.artist,
        parsed.album,
        parsed.track_title,
        parsed.confidence,
    );

    parsed
}

/// Build a human-readable quality string from hunch output.
fn build_quality_string(result: &hunch::HunchResult) -> String {
    let mut parts = Vec::new();
    if let Some(v) = result.screen_size() {
        parts.push(v.to_string());
    }
    if let Some(v) = result.source() {
        parts.push(v.to_string());
    }
    if let Some(v) = result.video_codec() {
        parts.push(v.to_string());
    }
    parts.join(" ")
}

/// Compute a 0–100 confidence score based on populated fields.
///
/// Mirrors the Python version's heuristic. Capped at 85 — DB enrichment
/// is needed for higher confidence.
fn compute_confidence(
    title: &str,
    year: Option<i32>,
    media_type: MediaType,
    season: Option<i32>,
    episode: Option<i32>,
) -> f64 {
    let mut score: f64 = 0.0;

    if !title.is_empty() {
        score += 30.0;
    }
    if year.is_some() {
        score += 20.0;
    }
    if media_type != MediaType::Unknown {
        score += 15.0;
    }
    if media_type == MediaType::Tv && season.is_some() {
        score += 15.0;
    }
    if media_type == MediaType::Tv && episode.is_some() {
        score += 15.0;
    }
    if media_type == MediaType::Movie && year.is_some() {
        score += 15.0;
    }

    score.min(85.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_movie() {
        let result = parse_video("The.Matrix.1999.1080p.BluRay.x264-GROUP.mkv");
        assert_eq!(result.title, "The Matrix");
        assert_eq!(result.year, Some(1999));
        assert_eq!(result.media_type, MediaType::Movie);
        assert!(result.confidence > 60.0);
    }

    #[test]
    fn test_parse_tv_episode() {
        let result = parse_video("The.Walking.Dead.S05E03.720p.BluRay.x264-DEMAND.mkv");
        assert_eq!(result.title, "The Walking Dead");
        assert_eq!(result.season, Some(5));
        assert_eq!(result.episode, Some(3));
        assert_eq!(result.media_type, MediaType::Tv);
        assert!(result.confidence > 70.0);
    }

    #[test]
    fn test_parse_empty_filename() {
        let result = parse_video("");
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_parse_music_track() {
        let file = MediaFile {
            source_path: "/music/Artist - Album (2020)/01 - Song Title.flac".into(),
            filename: "01 - Song Title".to_string(),
            extension: ".flac".to_string(),
            detected_type: MediaType::Music,
            size_bytes: 30_000_000,
            parent_dir: "Artist - Album (2020)".to_string(),
        };
        let result = parse_media_file(&file);
        assert_eq!(result.media_type, MediaType::Music);
        assert_eq!(result.track_number, Some(1));
        assert_eq!(result.track_title.as_deref(), Some("Song Title"));
        assert_eq!(result.artist.as_deref(), Some("Artist"));
        assert_eq!(result.album.as_deref(), Some("Album"));
        assert_eq!(result.year, Some(2020));
    }

    #[test]
    fn test_confidence_caps_at_85() {
        let conf = compute_confidence("Title", Some(2024), MediaType::Movie, None, None);
        assert!(conf <= 85.0);
    }
}
