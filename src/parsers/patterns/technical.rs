//! Technical pattern detection for quality, source, audio, and codec information

use anyhow::Result;
use regex::Regex;

use crate::parsers::types::FilenameComponents;

/// Technical patterns for detecting quality, source, audio, and codec information
#[derive(Clone, Debug)]
pub struct TechnicalPatterns {
    quality_patterns: Vec<String>,
    source_patterns: Vec<String>,
    audio_patterns: Vec<String>,
    codec_patterns: Vec<String>,
    technical_terms: Option<Vec<String>>,
}

impl Default for TechnicalPatterns {
    fn default() -> Self {
        Self {
            quality_patterns: vec![
                "720p".to_string(),
                "1080p".to_string(),
                "2160p".to_string(),
                "4K".to_string(),
                "HDR".to_string(),
                "UHD".to_string(),
                "HD".to_string(),
                "SD".to_string(),
            ],
            source_patterns: vec![
                "BluRay".to_string(),
                "WEB-DL".to_string(),
                "HDTV".to_string(),
                "DVDRip".to_string(),
                "BRRip".to_string(),
                "HDRip".to_string(),
                "WEBRip".to_string(),
                "REMUX".to_string(),
                "ATVP".to_string(),
                "iT".to_string(),
                "Netflix".to_string(),
                "Amazon".to_string(),
            ],
            audio_patterns: vec![
                "DTS".to_string(),
                "AC3".to_string(),
                "AAC".to_string(),
                "FLAC".to_string(),
                "DD5.1".to_string(),
                "DTS-HD".to_string(),
                "MA".to_string(),
                "THD".to_string(),
            ],
            codec_patterns: vec![
                "x264".to_string(),
                "x265".to_string(),
                "H264".to_string(),
                "H265".to_string(),
                "AVC".to_string(),
                "HEVC".to_string(),
                "10bit".to_string(),
            ],
            technical_terms: None,
        }
    }
}

impl TechnicalPatterns {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_technical_terms(technical_terms: Vec<String>) -> Self {
        Self {
            technical_terms: Some(technical_terms),
            ..Default::default()
        }
    }

    /// Detect quality information in filename
    pub fn detect_quality(&self, filename: &str) -> Option<String> {
        for pattern in &self.quality_patterns {
            if filename.to_uppercase().contains(&pattern.to_uppercase()) {
                return Some(pattern.clone());
            }
        }
        None
    }

    /// Detect source information in filename
    pub fn detect_source(&self, filename: &str) -> Option<String> {
        for pattern in &self.source_patterns {
            if filename.to_uppercase().contains(&pattern.to_uppercase()) {
                return Some(pattern.clone());
            }
        }
        None
    }

    /// Detect audio information in filename
    pub fn detect_audio(&self, filename: &str) -> Option<String> {
        for pattern in &self.audio_patterns {
            // Use word boundaries to avoid false positives
            let pattern_regex = format!(r"\b{}\b", regex::escape(pattern));
            if let Ok(regex) = Regex::new(&pattern_regex) {
                                if regex.is_match(filename) {
                    return Some(pattern.clone());
                }
            }
        }
        None
    }

    /// Detect codec information in filename
    pub fn detect_codec(&self, filename: &str) -> Option<String> {
        for pattern in &self.codec_patterns {
            if filename.to_uppercase().contains(&pattern.to_uppercase()) {
                return Some(pattern.clone());
            }
        }
        None
    }

    /// Detect year in filename
    pub fn detect_year(&self, filename: &str) -> Option<u32> {
        // Look for 4-digit years (19xx or 20xx)
        let year_regex = Regex::new(r"\b(19|20)\d{2}\b").unwrap();

        if let Some(captures) = year_regex.captures(filename) {
            if let Some(year_str) = captures.get(0) {
                if let Ok(year) = year_str.as_str().parse::<u32>() {
                    return Some(year);
                }
            }
        }
        None
    }

    /// Check if a term is a technical term that should be filtered out
    pub fn is_technical_term(&self, term: &str) -> bool {
        if let Some(ref technical_terms) = self.technical_terms {
            technical_terms.iter().any(|t| t.eq_ignore_ascii_case(term))
        } else {
            false
        }
    }
}

/// Main pattern detector that combines all pattern detection logic
#[derive(Clone, Debug)]
pub struct PatternDetector {
    technical_patterns: TechnicalPatterns,
}

impl Default for PatternDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternDetector {
    pub fn new() -> Self {
        Self {
            technical_patterns: TechnicalPatterns::new(),
        }
    }

    pub fn with_technical_terms(technical_terms: Vec<String>) -> Self {
        Self {
            technical_patterns: TechnicalPatterns::with_technical_terms(technical_terms),
        }
    }

    /// Parse a filename and extract all components
    pub fn parse_filename(&self, filename: &str) -> Result<FilenameComponents> {
        let mut components = FilenameComponents {
            title: String::new(),
            original_title: None,
            year: None,
            quality: None,
            source: None,
            language: None,
            audio: None,
            codec: None,
            group: None,
            confidence: 0.0,
        };

        // Extract technical information
        components.quality = self.technical_patterns.detect_quality(filename);
        components.source = self.technical_patterns.detect_source(filename);
        components.audio = self.technical_patterns.detect_audio(filename);
        components.codec = self.technical_patterns.detect_codec(filename);
        components.year = self.technical_patterns.detect_year(filename);

        // Extract title (this will be handled by the title extractor)
        components.title = self.extract_title(filename)?;

        // Calculate confidence based on what we found
        components.confidence = self.calculate_confidence(&components);

        Ok(components)
    }

    /// Extract the movie title from filename
    fn extract_title(&self, filename: &str) -> Result<String> {
        // Remove file extension
        let filename = filename.rsplit('.').skip(1).collect::<Vec<_>>().join(".");

        // Split by common separators
        let parts: Vec<&str> = filename
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.is_empty())
            .collect();

        let mut title_parts = Vec::new();

        for part in parts.iter().rev() {
            // Process in reverse order to get correct title order
            // Skip if it's a year
            if let Ok(year) = part.parse::<u32>() {
                                if (1900..=2030).contains(&year) {
                    continue;
                }
            }

            // Skip technical terms (but be more conservative)
            if self.technical_patterns.is_technical_term(part) {
                continue;
            }

            // Skip quality, source, audio, codec patterns
            if self.technical_patterns.detect_quality(part).is_some()
                || self.technical_patterns.detect_source(part).is_some()
                || self.technical_patterns.detect_audio(part).is_some()
                || self.technical_patterns.detect_codec(part).is_some()
            {
                continue;
            }

            // Skip common file extensions and technical terms
            let part_lower = part.to_lowercase();
            if part_lower == "mkv"
                || part_lower == "mp4"
                || part_lower == "avi"
                || part_lower == "web"
                || part_lower == "dl"
                || part_lower == "rip"
            {
                continue;
            }

            // Skip parts that contain only Chinese characters
            if part.chars().all(|c| !c.is_ascii() && c.is_alphabetic()) {
                continue;
            }

            title_parts.push(*part);
        }

        if title_parts.is_empty() {
            return Err(anyhow::anyhow!("Could not extract title from filename"));
        }

        Ok(title_parts.join(" "))
    }

    /// Calculate confidence score based on extracted components
    fn calculate_confidence(&self, components: &FilenameComponents) -> f32 {
        let mut confidence: f32 = 0.0;

        // Base confidence for having a title
        if !components.title.is_empty() {
            confidence += 0.3;
        }

        // Additional confidence for technical information
        if components.quality.is_some() {
            confidence += 0.1;
        }
        if components.source.is_some() {
            confidence += 0.1;
        }
        if components.year.is_some() {
            confidence += 0.2;
        }

        confidence.min(1.0)
    }

    /// Get the technical patterns
    pub fn technical_patterns(&self) -> &TechnicalPatterns {
        &self.technical_patterns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_quality() {
        let patterns = TechnicalPatterns::new();
        assert_eq!(
            patterns.detect_quality("movie.1080p.mkv"),
            Some("1080p".to_string())
        );
        assert_eq!(
            patterns.detect_quality("movie.4K.mkv"),
            Some("4K".to_string())
        );
        assert_eq!(patterns.detect_quality("movie.mkv"), None);
    }

    #[test]
    fn test_detect_source() {
        let patterns = TechnicalPatterns::new();
        assert_eq!(
            patterns.detect_source("movie.BluRay.mkv"),
            Some("BluRay".to_string())
        );
        assert_eq!(
            patterns.detect_source("movie.WEB-DL.mkv"),
            Some("WEB-DL".to_string())
        );
        assert_eq!(patterns.detect_source("movie.mkv"), None);
    }

    #[test]
    fn test_detect_year() {
        let patterns = TechnicalPatterns::new();
        assert_eq!(patterns.detect_year("movie.1999.mkv"), Some(1999));
        assert_eq!(patterns.detect_year("movie.2021.mkv"), Some(2021));
        assert_eq!(patterns.detect_year("movie.mkv"), None);
    }

    #[test]
    fn test_parse_filename() {
        let detector = PatternDetector::new();
        let result = detector
            .parse_filename("The.Matrix.1999.1080p.BluRay.mkv")
            .unwrap();

        assert_eq!(result.title, "The Matrix");
        assert_eq!(result.year, Some(1999));
        assert_eq!(result.quality, Some("1080p".to_string()));
        assert_eq!(result.source, Some("BluRay".to_string()));
    }

    #[test]
    fn test_technical_terms_filtering() {
        let patterns = TechnicalPatterns::new();
        // Test that "Matrix" is not considered a technical term
        assert!(!patterns.is_technical_term("Matrix"));
        assert!(!patterns.is_technical_term("The"));
        assert!(!patterns.is_technical_term("Matrix"));
    }
}
