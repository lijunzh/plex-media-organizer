//! Language detection for multilingual content

use regex::Regex;

use crate::parsers::types::LanguageInfo;

/// Language detection patterns and logic
#[derive(Clone, Debug)]
pub struct LanguageDetector {
    language_patterns: Vec<String>,
    bilingual_patterns: Vec<String>,
    // Non-Latin script detection
    #[allow(dead_code)]
    latin_regex: Regex,
    non_latin_regex: Regex,
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self {
            language_patterns: vec![
                "English".to_string(),
                "Chinese".to_string(),
                "Japanese".to_string(),
                "Korean".to_string(),
                "Arabic".to_string(),
                "Russian".to_string(),
                "Hindi".to_string(),
                "Thai".to_string(),
                "Hebrew".to_string(),
                "Greek".to_string(),
                "Spanish".to_string(),
                "French".to_string(),
                "German".to_string(),
                "Italian".to_string(),
            ],
            bilingual_patterns: vec![
                "双语".to_string(),
                "Bilingual".to_string(),
                "Dual".to_string(),
                "Multi".to_string(),
                "مزدوج".to_string(), // Arabic for "dual"
                "二重".to_string(),  // Japanese for "dual"
            ],
            // Latin script: Basic Latin + Latin-1 Supplement + Latin Extended
            latin_regex: Regex::new(r"^[a-zA-Z\u00C0-\u00FF\u0100-\u017F\u0180-\u024F]*$").unwrap(),
            // Non-Latin scripts: everything else
            non_latin_regex: Regex::new(r"[^\x00-\x7F]").unwrap(),
        }
    }
}

impl LanguageDetector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Detect language information in filename
    pub fn detect_language(&self, filename: &str) -> LanguageInfo {
        let mut language_info = LanguageInfo {
            primary_language: None,
            has_japanese: false,
            has_chinese: false,
            has_korean: false,
            is_bilingual: false,
        };

        // Check for explicit language patterns
        for pattern in &self.language_patterns {
            if filename.to_uppercase().contains(&pattern.to_uppercase()) {
                language_info.primary_language = Some(pattern.clone());
                break;
            }
        }

        // Check for bilingual patterns
        for pattern in &self.bilingual_patterns {
            if filename.to_uppercase().contains(&pattern.to_uppercase()) {
                language_info.is_bilingual = true;
                break;
            }
        }

        // Detect specific CJK characters (for backward compatibility)
        language_info.has_japanese = self.has_japanese_characters(filename);
        language_info.has_chinese = self.has_chinese_characters(filename);
        language_info.has_korean = self.has_korean_characters(filename);

        // If we have non-Latin characters but no explicit language, set primary language
        if language_info.primary_language.is_none() {
            if language_info.has_japanese {
                language_info.primary_language = Some("Japanese".to_string());
            } else if language_info.has_chinese {
                language_info.primary_language = Some("Chinese".to_string());
            } else if language_info.has_korean {
                language_info.primary_language = Some("Korean".to_string());
            } else if self.has_non_latin_characters(filename) {
                // Generic non-English detection
                language_info.primary_language = Some("Non-English".to_string());
            }
        }

        // Check if it's bilingual based on character presence
        if self.has_non_latin_characters(filename) && self.has_latin_characters(filename) {
            language_info.is_bilingual = true;
        }

        language_info
    }

    /// Check if text contains only Latin characters
    pub fn has_latin_characters(&self, text: &str) -> bool {
        text.chars().any(|c| c.is_ascii_alphabetic())
    }

    /// Check if text contains non-Latin characters
    pub fn has_non_latin_characters(&self, text: &str) -> bool {
        self.non_latin_regex.is_match(text)
    }

    /// Check if text is primarily Latin
    pub fn is_primarily_latin(&self, text: &str) -> bool {
        let latin_count = text.chars().filter(|c| c.is_ascii_alphabetic()).count();
        let total_alpha = text.chars().filter(|c| c.is_alphabetic()).count();

        if total_alpha == 0 {
            false
        } else {
            // If more than 50% of alphabetic characters are Latin, consider it primarily Latin
            (latin_count as f32 / total_alpha as f32) > 0.5
        }
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

    /// Check if text contains Arabic characters
    pub fn has_arabic_characters(&self, text: &str) -> bool {
        // Arabic: \u0600-\u06FF
        let arabic_regex = Regex::new(r"[\u0600-\u06FF]").unwrap();
        arabic_regex.is_match(text)
    }

    /// Check if text contains Cyrillic characters
    pub fn has_cyrillic_characters(&self, text: &str) -> bool {
        // Cyrillic: \u0400-\u04FF
        let cyrillic_regex = Regex::new(r"[\u0400-\u04FF]").unwrap();
        cyrillic_regex.is_match(text)
    }

    /// Check if text contains Devanagari characters
    pub fn has_devanagari_characters(&self, text: &str) -> bool {
        // Devanagari: \u0900-\u097F
        let devanagari_regex = Regex::new(r"[\u0900-\u097F]").unwrap();
        devanagari_regex.is_match(text)
    }

    /// Check if text contains Thai characters
    pub fn has_thai_characters(&self, text: &str) -> bool {
        // Thai: \u0E00-\u0E7F
        let thai_regex = Regex::new(r"[\u0E00-\u0E7F]").unwrap();
        thai_regex.is_match(text)
    }

    /// Check if text contains Hebrew characters
    pub fn has_hebrew_characters(&self, text: &str) -> bool {
        // Hebrew: \u0590-\u05FF
        let hebrew_regex = Regex::new(r"[\u0590-\u05FF]").unwrap();
        hebrew_regex.is_match(text)
    }

    /// Check if text contains Greek characters
    pub fn has_greek_characters(&self, text: &str) -> bool {
        // Greek: \u0370-\u03FF
        let greek_regex = Regex::new(r"[\u0370-\u03FF]").unwrap();
        greek_regex.is_match(text)
    }

    /// Extract language-specific title from multilingual content
    pub fn extract_language_title(&self, filename: &str, target_language: &str) -> Option<String> {
        let parts: Vec<&str> = filename
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.is_empty())
            .collect();

        let mut title_parts = Vec::new();

        for part in parts {
            match target_language {
                "English" => {
                    if self.is_primarily_latin(part) && !self.has_non_latin_characters(part) {
                        // Filter out technical terms for English titles
                        let part_lower = part.to_lowercase();
                        let technical_terms = vec![
                            "1080p", "720p", "4k", "hdr", "uhd", "hd", "sd", "bluray", "web-dl",
                            "hdtv", "dvdrip", "brrip", "hdrip", "x264", "x265", "h264", "h265",
                            "avc", "hevc", "dts", "ac3", "aac", "flac", "dd5.1", "dts-hd", "mkv",
                            "mp4", "avi", "web", "dl", "rip",
                        ];

                        if !technical_terms.contains(&part_lower.as_str()) {
                            title_parts.push(part);
                        }
                    }
                }
                "Non-English" => {
                    if self.has_non_latin_characters(part) {
                        title_parts.push(part);
                    }
                }
                "Chinese" => {
                    if self.has_chinese_characters(part) {
                        title_parts.push(part);
                    }
                }
                "Japanese" => {
                    if self.has_japanese_characters(part) {
                        title_parts.push(part);
                    }
                }
                "Korean" => {
                    if self.has_korean_characters(part) {
                        title_parts.push(part);
                    }
                }
                "Arabic" => {
                    if self.has_arabic_characters(part) {
                        title_parts.push(part);
                    }
                }
                "Russian" => {
                    if self.has_cyrillic_characters(part) {
                        title_parts.push(part);
                    }
                }
                "Hindi" => {
                    if self.has_devanagari_characters(part) {
                        title_parts.push(part);
                    }
                }
                "Thai" => {
                    if self.has_thai_characters(part) {
                        title_parts.push(part);
                    }
                }
                "Hebrew" => {
                    if self.has_hebrew_characters(part) {
                        title_parts.push(part);
                    }
                }
                "Greek" => {
                    if self.has_greek_characters(part) {
                        title_parts.push(part);
                    }
                }
                _ => {
                    // For other languages, include the part
                    title_parts.push(part);
                }
            }
        }

        if title_parts.is_empty() {
            None
        } else {
            Some(title_parts.join(" "))
        }
    }

    /// Determine if content is primarily in a specific language
    pub fn is_primarily_language(&self, text: &str, language: &str) -> bool {
        match language {
            "English" => self.is_primarily_latin(text) && !self.has_non_latin_characters(text),
            "Non-English" => self.has_non_latin_characters(text),
            "Chinese" => self.has_chinese_characters(text),
            "Japanese" => self.has_japanese_characters(text),
            "Korean" => self.has_korean_characters(text),
            "Arabic" => self.has_arabic_characters(text),
            "Russian" => self.has_cyrillic_characters(text),
            "Hindi" => self.has_devanagari_characters(text),
            "Thai" => self.has_thai_characters(text),
            "Hebrew" => self.has_hebrew_characters(text),
            "Greek" => self.has_greek_characters(text),
            _ => false,
        }
    }

    /// Get detected script types in text
    pub fn get_detected_scripts(&self, text: &str) -> Vec<String> {
        let mut scripts = Vec::new();

        if self.has_latin_characters(text) {
            scripts.push("Latin".to_string());
        }
        if self.has_chinese_characters(text) {
            scripts.push("Chinese".to_string());
        }
        if self.has_japanese_characters(text) {
            scripts.push("Japanese".to_string());
        }
        if self.has_korean_characters(text) {
            scripts.push("Korean".to_string());
        }
        if self.has_arabic_characters(text) {
            scripts.push("Arabic".to_string());
        }
        if self.has_cyrillic_characters(text) {
            scripts.push("Cyrillic".to_string());
        }
        if self.has_devanagari_characters(text) {
            scripts.push("Devanagari".to_string());
        }
        if self.has_thai_characters(text) {
            scripts.push("Thai".to_string());
        }
        if self.has_hebrew_characters(text) {
            scripts.push("Hebrew".to_string());
        }
        if self.has_greek_characters(text) {
            scripts.push("Greek".to_string());
        }

        scripts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_language_english() {
        let detector = LanguageDetector::new();
        let result = detector.detect_language("Movie.English.1080p.mkv");

        assert_eq!(result.primary_language, Some("English".to_string()));
        assert!(!result.is_bilingual);
    }

    #[test]
    fn test_detect_language_chinese() {
        let detector = LanguageDetector::new();
        let result = detector.detect_language("电影.1080p.mkv");

        assert_eq!(result.primary_language, Some("Chinese".to_string()));
        assert!(result.has_chinese);
        assert!(!result.has_japanese);
        assert!(!result.has_korean);
    }

    #[test]
    fn test_detect_language_arabic() {
        let detector = LanguageDetector::new();
        let result = detector.detect_language("فيلم.1080p.mkv");

        assert_eq!(result.primary_language, Some("Non-English".to_string()));
        assert!(detector.has_arabic_characters("فيلم"));
    }

    #[test]
    fn test_detect_language_russian() {
        let detector = LanguageDetector::new();
        let result = detector.detect_language("фильм.1080p.mkv");

        assert_eq!(result.primary_language, Some("Non-English".to_string()));
        assert!(detector.has_cyrillic_characters("фильм"));
    }

    #[test]
    fn test_detect_language_bilingual() {
        let detector = LanguageDetector::new();
        let result = detector.detect_language("Movie.电影.1080p.mkv");

        assert!(result.is_bilingual);
        assert!(result.has_chinese);
        assert!(detector.has_latin_characters("Movie"));
    }

    #[test]
    fn test_is_primarily_latin() {
        let detector = LanguageDetector::new();

        assert!(detector.is_primarily_latin("Movie"));
        assert!(detector.is_primarily_latin("The Matrix"));
        assert!(!detector.is_primarily_latin("电影"));
        // "Movie电影" has more Latin characters than non-Latin, so it should be primarily Latin
        assert!(detector.is_primarily_latin("Movie电影"));
    }

    #[test]
    fn test_get_detected_scripts() {
        let detector = LanguageDetector::new();
        let scripts = detector.get_detected_scripts("Movie电影фильм");

        assert!(scripts.contains(&"Latin".to_string()));
        assert!(scripts.contains(&"Chinese".to_string()));
        assert!(scripts.contains(&"Cyrillic".to_string()));
    }

    #[test]
    fn test_extract_language_title_english() {
        let detector = LanguageDetector::new();
        let title = detector.extract_language_title("Movie.电影.1080p.mkv", "English");

        assert_eq!(title, Some("Movie".to_string()));
    }

    #[test]
    fn test_extract_language_title_non_english() {
        let detector = LanguageDetector::new();
        let title = detector.extract_language_title("Movie.电影.1080p.mkv", "Non-English");

        assert_eq!(title, Some("电影".to_string()));
    }
}
