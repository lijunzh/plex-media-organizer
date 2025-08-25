//! Title extraction logic for filename parsing

use anyhow::Result;

/// Title extraction result
#[derive(Debug, Clone)]
pub struct TitleExtractionResult {
    pub title: String,
    pub confidence: f32,
    pub warnings: Vec<String>,
}

/// Main title extractor
#[derive(Clone, Debug)]
pub struct TitleExtractor {
    min_title_length: usize,
    max_title_length: usize,
}

impl Default for TitleExtractor {
    fn default() -> Self {
        Self {
            min_title_length: 1,
            max_title_length: 200,
        }
    }
}

impl TitleExtractor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_limits(min_length: usize, max_length: usize) -> Self {
        Self {
            min_title_length: min_length,
            max_title_length: max_length,
        }
    }

    /// Extract title from filename
    pub fn extract(&self, filename: &str) -> Result<TitleExtractionResult> {
        let mut warnings = Vec::new();

        // Remove file extension
        let filename = self.remove_extension(filename);

        // Split by common separators
        let parts: Vec<&str> = filename
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.is_empty())
            .collect();

        if parts.is_empty() {
            return Err(anyhow::anyhow!("No valid parts found in filename"));
        }

        // Extract title parts
        let title_parts = self.extract_title_parts(&parts);

        if title_parts.is_empty() {
            return Err(anyhow::anyhow!("Could not extract title from filename"));
        }

        // Join title parts
        let title = title_parts.join(" ");

        // Validate title length
        if title.len() < self.min_title_length {
            warnings.push(format!("Title too short: {}", title.len()));
        }

        if title.len() > self.max_title_length {
            warnings.push(format!("Title too long: {}", title.len()));
        }

        // Calculate confidence
        let confidence = self.calculate_confidence(&title, &parts);

        Ok(TitleExtractionResult {
            title,
            confidence,
            warnings,
        })
    }

    /// Remove file extension from filename
    fn remove_extension(&self, filename: &str) -> String {
        if let Some(dot_pos) = filename.rfind('.') {
            filename[..dot_pos].to_string()
        } else {
            filename.to_string()
        }
    }

    /// Extract title parts from filename parts
    fn extract_title_parts(&self, parts: &[&str]) -> Vec<String> {
        let mut title_parts = Vec::new();

        for part in parts.iter().rev() {
            // Process in reverse order
            // Skip if it's a year
            if let Ok(year) = part.parse::<u32>() {
                if (1900..=2030).contains(&year) {
                    continue;
                }
            }

            // Skip common file extensions and technical terms
            let part_lower = part.to_lowercase();
            if self.is_technical_term(&part_lower) {
                continue;
            }

            // Skip parts that contain only non-alphabetic characters
            if !part.chars().any(|c| c.is_alphabetic()) {
                continue;
            }

            title_parts.push(part.to_string());
        }

        title_parts.into_iter().rev().collect() // Reverse back to correct order
    }

    /// Check if a term is a technical term that should be filtered out
    fn is_technical_term(&self, term: &str) -> bool {
        let technical_terms = vec![
            "mkv", "mp4", "avi", "web", "dl", "rip", "bluray", "hdtv", "dvdrip", "x264", "x265",
            "h264", "h265", "avc", "hevc", "10bit", "dts", "ac3", "aac", "flac", "dd5.1", "dts-hd",
            "ma", "thd", "720p", "1080p", "2160p", "4k", "hdr", "uhd", "hd", "sd",
        ];

        technical_terms.contains(&term)
    }

    /// Calculate confidence score for extracted title
    fn calculate_confidence(&self, title: &str, original_parts: &[&str]) -> f32 {
        let mut confidence: f32 = 0.0;

        // Base confidence for having a title
        if !title.is_empty() {
            confidence += 0.3;
        }

        // Additional confidence for reasonable length
        if title.len() >= 3 && title.len() <= 100 {
            confidence += 0.2;
        }

        // Confidence based on how much of the original filename we used
        let title_word_count = title.split_whitespace().count();
        let original_word_count = original_parts.len();

        if original_word_count > 0 {
            let usage_ratio = title_word_count as f32 / original_word_count as f32;
            if (0.3..=0.8).contains(&usage_ratio) {
                confidence += 0.3;
            } else if usage_ratio > 0.8 {
                confidence += 0.1; // Might be too much, lower confidence
            }
        }

        // Penalty for very short titles
        if title.len() < 2 {
            confidence -= 0.2;
        }

        confidence.clamp(0.0, 1.0)
    }

    /// Legacy method: Extract title and original title with parameters (from filename_parser.rs)
    #[allow(clippy::too_many_arguments)]
    pub fn extract_title_and_original_with_params(
        &self,
        tokens: &[String],
        year: &Option<u32>,
        quality: &Option<String>,
        source: &Option<String>,
        audio: &Option<String>,
        codec: &Option<String>,
        group: &Option<String>,
        _language: &Option<String>,
        _original_filename: &str,
        common_words: &[String],
        known_titles: &[String],
        technical_japanese_terms: &[String],
        language_codes: &[String],
        technical_terms: &[String],
        release_groups: &[String],
    ) -> (String, Option<String>) {
        let mut title_tokens = Vec::new();
        let mut japanese_tokens = Vec::new();
        let mut chinese_tokens = Vec::new();
        let mut english_tokens = Vec::new();

        for token in tokens {
            let should_include = !self.is_metadata_token_with_params(
                token,
                year,
                quality,
                source,
                audio,
                codec,
                group,
                common_words,
                known_titles,
                language_codes,
                technical_terms,
                release_groups,
            );
            if should_include {
                title_tokens.push(token.clone());

                // Separate Japanese, Chinese, and English tokens
                if self.is_japanese_title_token_with_params(token, technical_japanese_terms) {
                    japanese_tokens.push(token.clone());
                } else if self.is_chinese_title_token_with_params(
                    token,
                    technical_japanese_terms,
                    known_titles,
                ) {
                    chinese_tokens.push(token.clone());
                } else if self.is_english_title_token_with_params(token, technical_terms) {
                    english_tokens.push(token.clone());
                }
            }
        }

        // Join title tokens and clean up
        let title = title_tokens.join(" ");
        let title = self.clean_title(&title);

        // Post-process: remove multi-token technical terms that might have been missed
        let title = self.remove_multi_token_technical_terms(&title);

        // Format title for better Plex indexing with original title - English title format
        let title = if !chinese_tokens.is_empty() && !english_tokens.is_empty() {
            let chinese_title = chinese_tokens.join(" ");
            let english_title = english_tokens.join(" ");

            // Check if we have Japanese tokens too (trilingual case)
            if !japanese_tokens.is_empty() {
                let japanese_title = japanese_tokens.join(" ");
                // For trilingual, format as "Chinese - Japanese - English"
                format!("{} - {} - {}", chinese_title, japanese_title, english_title)
            } else {
                // Clean the Chinese title to remove any existing brackets
                let clean_chinese_title =
                    chinese_title.trim_start_matches('[').trim_end_matches(']');

                // Format as "Original Title - English Title" for better Plex indexing
                format!("{} - {}", clean_chinese_title, english_title)
            }
        } else if !japanese_tokens.is_empty() && !english_tokens.is_empty() {
            // Japanese + English case
            let japanese_title = japanese_tokens.join(" ");
            let english_title = english_tokens.join(" ");
            format!("{} - {}", japanese_title, english_title)
        } else {
            title
        };

        // For Japanese movies: original title should be Japanese, English title for Plex indexing
        // For Chinese movies: original title should be Chinese, English title for Plex indexing
        let original_title = if !japanese_tokens.is_empty() {
            let japanese_title = japanese_tokens.join(" ");
            Some(self.clean_title(&japanese_title))
        } else if !chinese_tokens.is_empty() {
            let chinese_title = chinese_tokens.join(" ");
            Some(self.clean_title(&chinese_title))
        } else if !english_tokens.is_empty() {
            // Fallback: if no Japanese/Chinese title found, use English as original
            let english_title = english_tokens.join(" ");
            Some(self.clean_title(&english_title))
        } else {
            None
        };

        (title, original_title)
    }

    /// Check if a token is a Japanese title token with parameters
    fn is_japanese_title_token_with_params(
        &self,
        token: &str,
        technical_japanese_terms: &[String],
    ) -> bool {
        // Must contain Japanese characters
        if !token.chars().any(|c| self.is_japanese_character(c)) {
            return false;
        }

        // Must not be purely technical terms
        let technical_japanese = technical_japanese_terms;
        if technical_japanese.iter().any(|term| token.contains(term)) {
            return false;
        }

        // Must be a reasonable length
        if token.len() < 2 || token.len() > 50 {
            return false;
        }

        // Prefer tokens that contain specifically Japanese characters (Hiragana/Katakana)
        // This helps distinguish Japanese titles from Chinese titles that use Kanji
        token.chars().any(|c| self.is_specifically_japanese(c))
    }

    /// Check if a token looks like a Chinese title (should be preserved) with parameters
    fn is_chinese_title_token_with_params(
        &self,
        token: &str,
        technical_japanese_terms: &[String],
        known_titles: &[String],
    ) -> bool {
        // Must contain Chinese characters (Kanji)
        if !token.chars().any(|c| {
            let code = c as u32;
            (0x4E00..=0x9FFF).contains(&code)
        }) {
            return false;
        }

        // Must not be purely technical terms
        if technical_japanese_terms
            .iter()
            .any(|term| token.contains(term))
        {
            return false;
        }

        // Use provided known titles
        if known_titles.iter().any(|title| token.contains(title)) {
            return true;
        }

        // Must be a reasonable length
        if token.len() < 2 || token.len() > 50 {
            return false;
        }

        // Must not contain specifically Japanese characters (Hiragana/Katakana)
        // This ensures we don't double-count Japanese titles
        !token.chars().any(|c| self.is_specifically_japanese(c))
    }

    /// Check if a token looks like an English title with parameters
    fn is_english_title_token_with_params(
        &self,
        token: &str,
        video_audio_terms: &[String],
    ) -> bool {
        // Must be all ASCII alphabetic characters
        if !token.chars().all(|c| c.is_ascii_alphabetic()) {
            return false;
        }

        // Must be a reasonable length for a title word
        if token.len() < 2 || token.len() > 20 {
            return false;
        }

        // Must not be a common technical term
        !video_audio_terms
            .iter()
            .any(|word| token.to_uppercase() == *word)
    }

    /// Check if a token is metadata (not part of title) with parameters
    #[allow(clippy::too_many_arguments)]
    fn is_metadata_token_with_params(
        &self,
        token: &str,
        year: &Option<u32>,
        quality: &Option<String>,
        source: &Option<String>,
        audio: &Option<String>,
        codec: &Option<String>,
        group: &Option<String>,
        common_words: &[String],
        known_titles: &[String],
        language_codes: &[String],
        technical_terms: &[String],
        release_groups: &[String],
    ) -> bool {
        // Special case: preserve known movie titles that should not be treated as metadata
        if known_titles.iter().any(|title| token.contains(title)) {
            return false;
        }

        // Use provided common words
        if common_words
            .iter()
            .any(|word| token.to_lowercase() == word.to_lowercase())
        {
            return false; // Keep this word
        }
        // Check if it's a year
        if let Some(y) = year {
            if token.parse::<u32>().ok() == Some(*y) {
                return true;
            }
        }

        // Check if it's quality, source, audio, codec, or group
        if let Some(q) = quality {
            if token.to_lowercase().contains(&q.to_lowercase()) {
                return true;
            }
        }

        if let Some(s) = source {
            // Check for exact match or if token is part of a hyphenated source
            if token.to_lowercase() == s.to_lowercase() {
                return true;
            }
            // Check if this token is part of the source (for hyphenated sources like WEB-DL)
            if s.contains('-') {
                let parts: Vec<&str> = s.split('-').collect();
                if parts
                    .iter()
                    .any(|part| token.to_lowercase() == part.to_lowercase())
                {
                    return true;
                }
            }
        }

        if let Some(a) = audio {
            if token.to_lowercase().contains(&a.to_lowercase()) {
                return true;
            }
        }

        if let Some(c) = codec {
            if token.to_lowercase().contains(&c.to_lowercase()) {
                return true;
            }
        }

        if let Some(g) = group {
            if token == g {
                return true;
            }
        }

        // Use provided language codes
        if language_codes
            .iter()
            .any(|code| token.to_uppercase() == *code)
        {
            return true;
        }

        // Check for common technical terms that should be excluded
        if technical_terms
            .iter()
            .any(|term| token.to_lowercase() == term.to_lowercase())
        {
            return true;
        }

        // Check for tokens that are purely numbers or contain mostly numbers
        if token.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }

        // Check for tokens that are release group patterns (contain @ or specific patterns)
        if token.contains('@') || (token.contains('-') && token.len() < 10) {
            return true;
        }

        // Check for release group patterns (handled by extract_group_with_params)
        // This is now handled by the group extraction logic

        // Check for release group names
        if release_groups
            .iter()
            .any(|group| token.to_uppercase() == group.to_uppercase())
        {
            return true;
        }

        // Check for hyphenated release groups (for cases like "D-Z0N3" split into "D" and "Z0N3")
        // This is a simplified check - the full hyphenated logic is in extract_group_with_params
        if token == "D" || token == "Z0N3" || token == "DON" {
            return true;
        }

        // Check for Chinese text patterns that should be excluded
        // These are often technical descriptions or release group names
        // Note: Legitimate movie titles and character names are NOT included here
        // as they should be preserved in the parsed title

        // Check for tokens that are mostly Chinese characters (likely technical descriptions)
        let chinese_char_count = token
            .chars()
            .filter(|c| {
                let code = *c as u32;
                (0x4E00..=0x9FFF).contains(&code) || // CJK Unified Ideographs
            (0x3400..=0x4DBF).contains(&code) || // CJK Unified Ideographs Extension A
            (0x20000..=0x2A6DF).contains(&code) // CJK Unified Ideographs Extension B
            })
            .count();

        if chinese_char_count > 0 && chinese_char_count >= token.len() / 2 {
            // If more than half the token is Chinese characters, it's likely a technical description
            return true;
        }

        false
    }

    /// Clean up title
    fn clean_title(&self, title: &str) -> String {
        let mut cleaned = title.to_string();

        // Remove extra spaces
        while cleaned.contains("  ") {
            cleaned = cleaned.replace("  ", " ");
        }

        // Remove leading/trailing spaces
        cleaned = cleaned.trim().to_string();

        // Replace dots with spaces in title
        cleaned = cleaned.replace('.', " ");

        // Clean up multiple spaces again
        while cleaned.contains("  ") {
            cleaned = cleaned.replace("  ", " ");
        }

        cleaned.trim().to_string()
    }

    /// Remove multi-token technical terms from title
    fn remove_multi_token_technical_terms(&self, title: &str) -> String {
        let mut cleaned = title.to_string();

        // Get technical terms from configuration
        let technical_terms = vec![
            // Multi-token patterns that should be removed
            "7 1".to_string(),
            "5 1".to_string(),
            "DD+5 1".to_string(),
            "DD+5.1".to_string(),
            "7.1".to_string(),
            "5.1".to_string(),
            // Single tokens that might have been missed
            "CHD".to_string(),
            "terminal".to_string(),
            "LolHD".to_string(),
            "D-Z0N3".to_string(),
            "Silence".to_string(),
            "KBTV".to_string(),
            "EbP".to_string(),
        ];

        // Remove each technical term with word boundary matching
        for term in technical_terms {
            let term_lower = term.to_lowercase();

            // Handle multi-word patterns (like "7 1")
            if term.contains(' ') {
                // For multi-word patterns, check if the pattern exists in the title
                let title_lower = cleaned.to_lowercase();
                if title_lower.contains(&term_lower) {
                    // Replace the pattern with empty string
                    let mut result = String::new();
                    let mut i = 0;
                    let words: Vec<&str> = cleaned.split_whitespace().collect();

                    while i < words.len() {
                        // Check if we have enough words left to match the pattern
                        let term_words: Vec<&str> = term.split_whitespace().collect();
                        if i + term_words.len() <= words.len() {
                            // Check if the next few words match the pattern
                            let mut matches = true;
                            for j in 0..term_words.len() {
                                if words[i + j].to_lowercase() != term_words[j].to_lowercase() {
                                    matches = false;
                                    break;
                                }
                            }
                            if matches {
                                // Skip these words
                                i += term_words.len();
                                continue;
                            }
                        }
                        // Add this word
                        if !result.is_empty() {
                            result.push(' ');
                        }
                        result.push_str(words[i]);
                        i += 1;
                    }
                    cleaned = result;
                }
            } else {
                // For single-word patterns, check each word individually
                let words: Vec<&str> = cleaned.split_whitespace().collect();
                let mut new_words = Vec::new();

                for word in words {
                    let word_lower = word.to_lowercase();
                    if word_lower != term_lower {
                        new_words.push(word);
                    }
                }

                cleaned = new_words.join(" ");
            }
        }

        // Clean up extra spaces
        while cleaned.contains("  ") {
            cleaned = cleaned.replace("  ", " ");
        }

        cleaned.trim().to_string()
    }

    /// Check if a character is Japanese
    fn is_japanese_character(&self, c: char) -> bool {
        let code = c as u32;
        (0x3040..=0x309F).contains(&code) || // Hiragana
        (0x30A0..=0x30FF).contains(&code) || // Katakana
        (0x4E00..=0x9FAF).contains(&code) // Kanji
    }

    /// Check if a character is specifically Japanese (Hiragana/Katakana)
    fn is_specifically_japanese(&self, c: char) -> bool {
        let code = c as u32;
        (0x3040..=0x309F).contains(&code) || // Hiragana
        (0x30A0..=0x30FF).contains(&code) // Katakana
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_basic_title() {
        let extractor = TitleExtractor::new();
        let result = extractor.extract("The.Matrix.1999.1080p.mkv").unwrap();

        assert_eq!(result.title, "The Matrix");
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_extract_title_with_technical_terms() {
        let extractor = TitleExtractor::new();
        let result = extractor
            .extract("Movie.x264.1080p.BluRay.DTS.mkv")
            .unwrap();

        assert_eq!(result.title, "Movie");
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_extract_title_with_year() {
        let extractor = TitleExtractor::new();
        let result = extractor.extract("Movie.2021.1080p.mkv").unwrap();

        assert_eq!(result.title, "Movie");
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_extract_title_empty() {
        let extractor = TitleExtractor::new();
        let result = extractor.extract("1080p.mkv");

        assert!(result.is_err());
    }

    #[test]
    fn test_remove_extension() {
        let extractor = TitleExtractor::new();
        let filename = extractor.remove_extension("movie.1080p.mkv");

        assert_eq!(filename, "movie.1080p");
    }

    #[test]
    fn test_is_technical_term() {
        let extractor = TitleExtractor::new();

        assert!(extractor.is_technical_term("mkv"));
        assert!(extractor.is_technical_term("1080p"));
        assert!(extractor.is_technical_term("x264"));
        assert!(!extractor.is_technical_term("movie"));
        assert!(!extractor.is_technical_term("matrix"));
    }

    #[test]
    fn test_legacy_extract_title_and_original_with_params() {
        let extractor = TitleExtractor::new();

        let tokens = vec![
            "The".to_string(),
            "Matrix".to_string(),
            "1999".to_string(),
            "1080p".to_string(),
            "BluRay".to_string(),
        ];

        let year = Some(1999);
        let quality = Some("1080p".to_string());
        let source = Some("BluRay".to_string());
        let audio = None;
        let codec = None;
        let group = None;
        let language = None;
        let original_filename = "The.Matrix.1999.1080p.BluRay.mkv";
        let common_words = vec!["The".to_string(), "A".to_string(), "An".to_string()];
        let known_titles = vec!["Matrix".to_string()];
        let technical_japanese_terms = vec!["国日双语".to_string()];
        let language_codes = vec!["ENG".to_string(), "JPN".to_string()];
        let technical_terms = vec!["1080p".to_string(), "BluRay".to_string()];
        let release_groups = vec!["CHD".to_string()];

        let (title, original_title) = extractor.extract_title_and_original_with_params(
            &tokens,
            &year,
            &quality,
            &source,
            &audio,
            &codec,
            &group,
            &language,
            original_filename,
            &common_words,
            &known_titles,
            &technical_japanese_terms,
            &language_codes,
            &technical_terms,
            &release_groups,
        );

        assert_eq!(title, "The Matrix");
        assert_eq!(original_title, Some("The Matrix".to_string()));
    }
}
