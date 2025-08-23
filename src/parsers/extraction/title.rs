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
            if let Ok(year) = part.parse::<u32>()
                && (1900..=2030).contains(&year)
            {
                continue;
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
}
