//! Anime detection for Japanese and Chinese animation patterns

use regex::Regex;

use crate::parsers::types::AnimeInfo;

/// Anime detection patterns and logic
#[derive(Clone, Debug)]
pub struct AnimeDetector {
    anime_patterns: Vec<String>,
    movie_number_patterns: Vec<String>,
}

impl Default for AnimeDetector {
    fn default() -> Self {
        Self {
            anime_patterns: vec![
                "Anime".to_string(),
                "OVA".to_string(),
                "OAV".to_string(),
                "Movie".to_string(),
                "Special".to_string(),
            ],
            movie_number_patterns: vec![
                r"Movie\s+(\d+)".to_string(),
                r"(\d+)\s*\.mkv".to_string(),
                r"(\d+)\s*\.mp4".to_string(),
                r"(\d+)\s*\.avi".to_string(),
            ],
        }
    }
}

impl AnimeDetector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Detect if a filename contains anime information
    pub fn detect_anime(&self, filename: &str) -> AnimeInfo {
        let mut anime_info = AnimeInfo {
            is_anime: false,
            movie_number: None,
            has_japanese_title: false,
            has_chinese_title: false,
            is_movie_series: false,
        };

        // Check for anime patterns
        for pattern in &self.anime_patterns {
            if filename.to_uppercase().contains(&pattern.to_uppercase()) {
                anime_info.is_anime = true;
                break;
            }
        }

        // Check for Japanese or Chinese characters
        anime_info.has_japanese_title = self.has_japanese_characters(filename);
        anime_info.has_chinese_title = self.has_chinese_characters(filename);

        // If it has CJK characters, it's likely anime
        if anime_info.has_japanese_title || anime_info.has_chinese_title {
            anime_info.is_anime = true;
        }

        // Extract movie number
        anime_info.movie_number = self.detect_movie_number(filename);

        // Check if it's part of a movie series
        anime_info.is_movie_series = anime_info.movie_number.is_some();

        anime_info
    }

    /// Detect movie number in anime filenames
    pub fn detect_movie_number(&self, filename: &str) -> Option<u32> {
        for pattern in &self.movie_number_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(filename) {
                    if let Some(number_str) = captures.get(1) {
                        if let Ok(number) = number_str.as_str().parse::<u32>() {
                            return Some(number);
                        }
                    }
                }
            }
        }

        // Also check for simple number patterns
        let simple_number_regex = Regex::new(r"\b(\d+)\b").unwrap();
        if let Some(captures) = simple_number_regex.captures(filename) {
            if let Some(number_str) = captures.get(1) {
                if let Ok(number) = number_str.as_str().parse::<u32>() {
                    // Only return if it's a reasonable movie number (1-20)
                    if number >= 1 && number <= 20 {
                        return Some(number);
                    }
                }
            }
        }

        None
    }

    /// Extract anime title from filename
    pub fn extract_anime_title(&self, filename: &str) -> String {
        let mut title = filename.to_string();

        // Remove file extension
        if let Some(dot_pos) = title.rfind('.') {
            title = title[..dot_pos].to_string();
        }

        // Remove movie number patterns
        for pattern in &self.movie_number_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                title = regex.replace_all(&title, "").to_string();
            }
        }

        // Remove common technical terms (but preserve the actual anime title)
        let technical_terms = vec![
            "1080p", "720p", "4K", "HDR", "UHD", "HD", "SD", "BluRay", "WEB-DL", "HDTV", "DVDRip",
            "BRRip", "HDRip", "x264", "x265", "H264", "H265", "AVC", "HEVC", "DTS", "AC3", "AAC",
            "FLAC", "DD5.1", "DTS-HD",
            "Movie", // Remove "Movie" as it's a technical term in anime context
        ];

        for term in technical_terms {
            let term_upper = term.to_uppercase();
            if title.to_uppercase().contains(&term_upper) {
                let regex_pattern = format!(r"(?i){}", regex::escape(term));
                if let Ok(regex) = Regex::new(&regex_pattern) {
                    title = regex.replace_all(&title, "").to_string();
                }
            }
        }

        // Remove standalone numbers that might be movie numbers
        let number_regex = Regex::new(r"\b\d+\b").unwrap();
        title = number_regex.replace_all(&title, "").to_string();

        // Clean up extra whitespace and separators
        title = title
            .split(|c| c == '.' || c == '_' || c == '-')
            .filter(|part| !part.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        title.trim().to_string()
    }

    /// Check if text contains Japanese characters (Hiragana, Katakana)
    pub fn has_japanese_characters(&self, text: &str) -> bool {
        // Hiragana: \u3040-\u309F
        // Katakana: \u30A0-\u30FF
        let japanese_regex = Regex::new(r"[\u3040-\u309F\u30A0-\u30FF]").unwrap();
        japanese_regex.is_match(text)
    }

    /// Check if text contains Chinese characters (Han)
    pub fn has_chinese_characters(&self, text: &str) -> bool {
        // Han characters: \u4E00-\u9FFF
        let chinese_regex = Regex::new(r"[\u4E00-\u9FFF]").unwrap();
        chinese_regex.is_match(text)
    }

    /// Check if text contains Korean characters (Hangul)
    pub fn has_korean_characters(&self, text: &str) -> bool {
        // Hangul: \uAC00-\uD7AF
        let korean_regex = Regex::new(r"[\uAC00-\uD7AF]").unwrap();
        korean_regex.is_match(text)
    }

    /// Check if text contains any CJK characters
    pub fn has_cjk_characters(&self, text: &str) -> bool {
        self.has_japanese_characters(text)
            || self.has_chinese_characters(text)
            || self.has_korean_characters(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_anime_basic() {
        let detector = AnimeDetector::new();
        let result = detector.detect_anime("Anime.Movie.1080p.mkv");

        assert!(result.is_anime);
        assert_eq!(result.movie_number, None);
    }

    #[test]
    fn test_detect_anime_movie_number() {
        let detector = AnimeDetector::new();
        let result = detector.detect_anime("Anime.Movie.2.1080p.mkv");

        assert!(result.is_anime);
        assert_eq!(result.movie_number, Some(2));
        assert!(result.is_movie_series);
    }

    #[test]
    fn test_detect_anime_japanese() {
        let detector = AnimeDetector::new();
        let result = detector.detect_anime("アニメ.Movie.1080p.mkv");

        assert!(result.is_anime);
        assert!(result.has_japanese_title);
    }

    #[test]
    fn test_detect_anime_chinese() {
        let detector = AnimeDetector::new();
        let result = detector.detect_anime("动画.Movie.1080p.mkv");

        assert!(result.is_anime);
        assert!(result.has_chinese_title);
    }

    #[test]
    fn test_extract_anime_title() {
        let detector = AnimeDetector::new();
        let title = detector.extract_anime_title("Anime.Movie.2.1080p.mkv");

        assert_eq!(title, "Anime");
    }

    #[test]
    fn test_has_japanese_characters() {
        let detector = AnimeDetector::new();
        assert!(detector.has_japanese_characters("アニメ"));
        assert!(!detector.has_japanese_characters("Anime"));
    }

    #[test]
    fn test_has_chinese_characters() {
        let detector = AnimeDetector::new();
        assert!(detector.has_chinese_characters("动画"));
        assert!(!detector.has_chinese_characters("Anime"));
    }

    #[test]
    fn test_has_korean_characters() {
        let detector = AnimeDetector::new();
        assert!(detector.has_korean_characters("애니메이션"));
        assert!(!detector.has_korean_characters("Anime"));
    }
}
