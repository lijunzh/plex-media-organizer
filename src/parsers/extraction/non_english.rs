//! Non-English character processing for multilingual content

use regex::Regex;

/// Non-English processing result
#[derive(Debug, Clone)]
pub struct NonEnglishProcessingResult {
    pub original_title: Option<String>,
    pub has_non_english: bool,
    pub primary_language: Option<String>,
    pub detected_scripts: Vec<String>,
    pub warnings: Vec<String>,
}

/// Non-English title processor (formerly CJK processor)
#[derive(Clone, Debug)]
pub struct NonEnglishProcessor {
    // Specific script detection
    japanese_regex: Regex,
    chinese_regex: Regex,
    korean_regex: Regex,
    arabic_regex: Regex,
    cyrillic_regex: Regex,
    devanagari_regex: Regex,
    thai_regex: Regex,
    hebrew_regex: Regex,
    greek_regex: Regex,
    // General non-Latin detection
    non_latin_regex: Regex,
}

impl Default for NonEnglishProcessor {
    fn default() -> Self {
        Self {
            japanese_regex: Regex::new(r"[\u3040-\u309F\u30A0-\u30FF]").unwrap(),
            chinese_regex: Regex::new(r"[\u4E00-\u9FFF]").unwrap(),
            korean_regex: Regex::new(r"[\uAC00-\uD7AF]").unwrap(),
            arabic_regex: Regex::new(r"[\u0600-\u06FF]").unwrap(),
            cyrillic_regex: Regex::new(r"[\u0400-\u04FF]").unwrap(),
            devanagari_regex: Regex::new(r"[\u0900-\u097F]").unwrap(),
            thai_regex: Regex::new(r"[\u0E00-\u0E7F]").unwrap(),
            hebrew_regex: Regex::new(r"[\u0590-\u05FF]").unwrap(),
            greek_regex: Regex::new(r"[\u0370-\u03FF]").unwrap(),
            non_latin_regex: Regex::new(r"[^\x00-\x7F]").unwrap(),
        }
    }
}

impl NonEnglishProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Process filename for non-English content
    pub fn process(&self, filename: &str) -> NonEnglishProcessingResult {
        let mut result = NonEnglishProcessingResult {
            original_title: None,
            has_non_english: false,
            primary_language: None,
            detected_scripts: Vec::new(),
            warnings: Vec::new(),
        };

        // Check for non-Latin characters
        result.has_non_english = self.non_latin_regex.is_match(filename);

        if result.has_non_english {
            // Detect specific scripts
            result.detected_scripts = self.get_detected_scripts(filename);

            // Determine primary language based on detected scripts
            result.primary_language = self.determine_primary_language(&result.detected_scripts);

            // Extract original non-English title
            result.original_title = self.extract_non_english_title(filename);
        }

        result
    }

    /// Get all detected scripts in the text
    pub fn get_detected_scripts(&self, text: &str) -> Vec<String> {
        let mut scripts = Vec::new();

        // Check for Latin characters first
        if text.chars().any(|c| c.is_ascii_alphabetic()) {
            scripts.push("Latin".to_string());
        }

        if self.japanese_regex.is_match(text) {
            scripts.push("Japanese".to_string());
        }
        if self.chinese_regex.is_match(text) {
            scripts.push("Chinese".to_string());
        }
        if self.korean_regex.is_match(text) {
            scripts.push("Korean".to_string());
        }
        if self.arabic_regex.is_match(text) {
            scripts.push("Arabic".to_string());
        }
        if self.cyrillic_regex.is_match(text) {
            scripts.push("Cyrillic".to_string());
        }
        if self.devanagari_regex.is_match(text) {
            scripts.push("Devanagari".to_string());
        }
        if self.thai_regex.is_match(text) {
            scripts.push("Thai".to_string());
        }
        if self.hebrew_regex.is_match(text) {
            scripts.push("Hebrew".to_string());
        }
        if self.greek_regex.is_match(text) {
            scripts.push("Greek".to_string());
        }

        scripts
    }

    /// Determine primary language based on detected scripts
    fn determine_primary_language(&self, scripts: &[String]) -> Option<String> {
        // Priority order for language detection
        let priority_languages = vec![
            "Japanese",
            "Chinese",
            "Korean",
            "Arabic",
            "Cyrillic",
            "Devanagari",
            "Thai",
            "Hebrew",
            "Greek",
        ];

        for language in priority_languages {
            if scripts.iter().any(|s| s == language) {
                return Some(language.to_string());
            }
        }

        if !scripts.is_empty() {
            Some("Non-English".to_string())
        } else {
            None
        }
    }

    /// Extract non-English title from filename
    pub fn extract_non_english_title(&self, filename: &str) -> Option<String> {
        let parts: Vec<&str> = filename
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.is_empty())
            .collect();

        let mut non_english_parts = Vec::new();

        for part in parts {
            if self.non_latin_regex.is_match(part) {
                non_english_parts.push(part);
            }
        }

        if non_english_parts.is_empty() {
            None
        } else {
            Some(non_english_parts.join(" "))
        }
    }

    /// Check if text contains Japanese characters
    pub fn has_japanese(&self, text: &str) -> bool {
        self.japanese_regex.is_match(text)
    }

    /// Check if text contains Chinese characters
    pub fn has_chinese(&self, text: &str) -> bool {
        self.chinese_regex.is_match(text)
    }

    /// Check if text contains Korean characters
    pub fn has_korean(&self, text: &str) -> bool {
        self.korean_regex.is_match(text)
    }

    /// Check if text contains Arabic characters
    pub fn has_arabic(&self, text: &str) -> bool {
        self.arabic_regex.is_match(text)
    }

    /// Check if text contains Cyrillic characters
    pub fn has_cyrillic(&self, text: &str) -> bool {
        self.cyrillic_regex.is_match(text)
    }

    /// Check if text contains Devanagari characters
    pub fn has_devanagari(&self, text: &str) -> bool {
        self.devanagari_regex.is_match(text)
    }

    /// Check if text contains Thai characters
    pub fn has_thai(&self, text: &str) -> bool {
        self.thai_regex.is_match(text)
    }

    /// Check if text contains Hebrew characters
    pub fn has_hebrew(&self, text: &str) -> bool {
        self.hebrew_regex.is_match(text)
    }

    /// Check if text contains Greek characters
    pub fn has_greek(&self, text: &str) -> bool {
        self.greek_regex.is_match(text)
    }

    /// Check if text contains any non-Latin characters
    pub fn has_non_latin(&self, text: &str) -> bool {
        self.non_latin_regex.is_match(text)
    }

    /// Extract language-specific title
    pub fn extract_language_title(&self, filename: &str, language: &str) -> Option<String> {
        let parts: Vec<&str> = filename
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.is_empty())
            .collect();

        let mut title_parts = Vec::new();

        for part in parts {
            match language {
                "Japanese" => {
                    if self.has_japanese(part) {
                        title_parts.push(part);
                    }
                }
                "Chinese" => {
                    if self.has_chinese(part) {
                        title_parts.push(part);
                    }
                }
                "Korean" => {
                    if self.has_korean(part) {
                        title_parts.push(part);
                    }
                }
                "Arabic" => {
                    if self.has_arabic(part) {
                        title_parts.push(part);
                    }
                }
                "Russian" => {
                    if self.has_cyrillic(part) {
                        title_parts.push(part);
                    }
                }
                "Hindi" => {
                    if self.has_devanagari(part) {
                        title_parts.push(part);
                    }
                }
                "Thai" => {
                    if self.has_thai(part) {
                        title_parts.push(part);
                    }
                }
                "Hebrew" => {
                    if self.has_hebrew(part) {
                        title_parts.push(part);
                    }
                }
                "Greek" => {
                    if self.has_greek(part) {
                        title_parts.push(part);
                    }
                }
                "Non-English" => {
                    if self.has_non_latin(part) {
                        title_parts.push(part);
                    }
                }
                "English" => {
                    if !self.has_non_latin(part) && part.chars().any(|c| c.is_alphabetic()) {
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

    /// Clean non-English title by removing technical terms
    pub fn clean_non_english_title(&self, title: &str) -> String {
        let mut cleaned = title.to_string();

        // Remove common technical terms that might appear in non-English titles
        let technical_terms = vec![
            "1080p", "720p", "4K", "HDR", "UHD", "HD", "SD", "BluRay", "WEB-DL", "HDTV", "DVDRip",
            "BRRip", "HDRip", "x264", "x265", "H264", "H265", "AVC", "HEVC", "DTS", "AC3", "AAC",
            "FLAC", "DD5.1", "DTS-HD",
        ];

        for term in technical_terms {
            let term_upper = term.to_uppercase();
            if cleaned.to_uppercase().contains(&term_upper) {
                let regex_pattern = format!(r"(?i){}", regex::escape(term));
                if let Ok(regex) = Regex::new(&regex_pattern) {
                    cleaned = regex.replace_all(&cleaned, "").to_string();
                }
            }
        }

        // Clean up extra whitespace and separators
        cleaned = cleaned
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        cleaned.trim().to_string()
    }

    /// Legacy method for backward compatibility
    #[deprecated(since = "1.0.0", note = "Use process() instead")]
    pub fn process_cjk(&self, filename: &str) -> NonEnglishProcessingResult {
        self.process(filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_japanese() {
        let processor = NonEnglishProcessor::new();
        let result = processor.process("アニメ.Movie.1080p.mkv");

        assert!(result.has_non_english);
        assert_eq!(result.primary_language, Some("Japanese".to_string()));
        assert_eq!(result.original_title, Some("アニメ".to_string()));
        assert!(result.detected_scripts.contains(&"Japanese".to_string()));
    }

    #[test]
    fn test_process_chinese() {
        let processor = NonEnglishProcessor::new();
        let result = processor.process("电影.Movie.1080p.mkv");

        assert!(result.has_non_english);
        assert_eq!(result.primary_language, Some("Chinese".to_string()));
        assert_eq!(result.original_title, Some("电影".to_string()));
        assert!(result.detected_scripts.contains(&"Chinese".to_string()));
    }

    #[test]
    fn test_process_arabic() {
        let processor = NonEnglishProcessor::new();
        let result = processor.process("فيلم.Movie.1080p.mkv");

        assert!(result.has_non_english);
        assert_eq!(result.primary_language, Some("Arabic".to_string()));
        assert_eq!(result.original_title, Some("فيلم".to_string()));
        assert!(result.detected_scripts.contains(&"Arabic".to_string()));
    }

    #[test]
    fn test_process_russian() {
        let processor = NonEnglishProcessor::new();
        let result = processor.process("фильм.Movie.1080p.mkv");

        assert!(result.has_non_english);
        assert_eq!(result.primary_language, Some("Cyrillic".to_string()));
        assert_eq!(result.original_title, Some("фильм".to_string()));
        assert!(result.detected_scripts.contains(&"Cyrillic".to_string()));
    }

    #[test]
    fn test_process_english_only() {
        let processor = NonEnglishProcessor::new();
        let result = processor.process("Movie.1080p.mkv");

        assert!(!result.has_non_english);
        assert_eq!(result.primary_language, None);
        assert_eq!(result.original_title, None);
        assert!(result.detected_scripts.is_empty());
    }

    #[test]
    fn test_extract_language_title_japanese() {
        let processor = NonEnglishProcessor::new();
        let title = processor.extract_language_title("アニメ.Movie.1080p.mkv", "Japanese");

        assert_eq!(title, Some("アニメ".to_string()));
    }

    #[test]
    fn test_extract_language_title_english() {
        let processor = NonEnglishProcessor::new();
        let title = processor.extract_language_title("アニメ.Movie.1080p.mkv", "English");

        assert_eq!(title, Some("Movie".to_string()));
    }

    #[test]
    fn test_extract_language_title_non_english() {
        let processor = NonEnglishProcessor::new();
        let title = processor.extract_language_title("Movie.电影.1080p.mkv", "Non-English");

        assert_eq!(title, Some("电影".to_string()));
    }

    #[test]
    fn test_clean_non_english_title() {
        let processor = NonEnglishProcessor::new();
        let cleaned = processor.clean_non_english_title("アニメ.1080p.BluRay");

        assert_eq!(cleaned, "アニメ");
    }

    #[test]
    fn test_get_detected_scripts() {
        let processor = NonEnglishProcessor::new();
        let scripts = processor.get_detected_scripts("Movie电影فيلم");

        assert!(scripts.contains(&"Latin".to_string()));
        assert!(scripts.contains(&"Chinese".to_string()));
        assert!(scripts.contains(&"Arabic".to_string()));
    }

    #[test]
    fn test_has_japanese() {
        let processor = NonEnglishProcessor::new();
        assert!(processor.has_japanese("アニメ"));
        assert!(!processor.has_japanese("Anime"));
    }

    #[test]
    fn test_has_chinese() {
        let processor = NonEnglishProcessor::new();
        assert!(processor.has_chinese("电影"));
        assert!(!processor.has_chinese("Movie"));
    }

    #[test]
    fn test_has_arabic() {
        let processor = NonEnglishProcessor::new();
        assert!(processor.has_arabic("فيلم"));
        assert!(!processor.has_arabic("Movie"));
    }

    #[test]
    fn test_has_cyrillic() {
        let processor = NonEnglishProcessor::new();
        assert!(processor.has_cyrillic("фильм"));
        assert!(!processor.has_cyrillic("Movie"));
    }
}
