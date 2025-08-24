//! Technical terms filtering for title extraction

use crate::config::TechnicalTermsConfig;
use regex::Regex;
use std::collections::HashSet;

/// Filter for removing technical terms from movie titles
#[derive(Debug, Clone)]
pub struct TechnicalTermsFilter {
    technical_terms: HashSet<String>,
    release_groups: HashSet<String>,
    codecs: HashSet<String>,
    qualities: HashSet<String>,
    sources: HashSet<String>,
    audio_formats: HashSet<String>,
}

impl Default for TechnicalTermsFilter {
    fn default() -> Self {
        Self::with_config(TechnicalTermsConfig::default())
    }
}

impl TechnicalTermsFilter {
    /// Create a new filter with default terms (fallback)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a filter with custom terms
    pub fn with_terms(terms: Vec<String>) -> Self {
        let mut filter = Self::new();
        for term in terms {
            filter.technical_terms.insert(term);
        }
        filter
    }

    /// Create a filter from configuration
    pub fn with_config(config: TechnicalTermsConfig) -> Self {
        let mut technical_terms = HashSet::new();
        let mut release_groups = HashSet::new();
        let mut codecs = HashSet::new();
        let mut qualities = HashSet::new();
        let mut sources = HashSet::new();
        let mut audio_formats = HashSet::new();

        // Add release groups from config
        for term in config.release_groups {
            release_groups.insert(term);
        }

        // Categorize video/audio terms from config
        for term in config.video_audio_terms {
            if Self::is_audio_format(&term) {
                audio_formats.insert(term);
            } else if Self::is_codec(&term) {
                codecs.insert(term);
            } else {
                technical_terms.insert(term);
            }
        }

        // Add source/platform terms from config
        for term in config.source_platform_terms {
            sources.insert(term);
        }

        // Add file format terms from config
        for term in config.file_format_terms {
            technical_terms.insert(term);
        }

        // Add special edition terms from config
        for term in config.special_edition_terms {
            technical_terms.insert(term);
        }

        // Add custom terms from config
        for term in config.custom_terms {
            technical_terms.insert(term);
        }

        // Add essential fallback terms (minimal set that should never change)
        Self::add_essential_fallback_terms(
            &mut technical_terms,
            &mut release_groups,
            &mut codecs,
            &mut qualities,
            &mut sources,
            &mut audio_formats,
        );

        Self {
            technical_terms,
            release_groups,
            codecs,
            qualities,
            sources,
            audio_formats,
        }
    }

    /// Add essential fallback terms that should always be available
    /// These are minimal and should rarely change
    fn add_essential_fallback_terms(
        technical_terms: &mut HashSet<String>,
        release_groups: &mut HashSet<String>,
        codecs: &mut HashSet<String>,
        qualities: &mut HashSet<String>,
        sources: &mut HashSet<String>,
        audio_formats: &mut HashSet<String>,
    ) {
        // Essential technical terms (minimal fallback)
        let essential_technical = vec![
            "mkv",
            "mp4",
            "avi",
            "web",
            "dl",
            "rip",
            "part",
            "cd",
            "disc",
            "volume",
            "vol",
            "collection",
            "trilogy",
            "saga",
            "series",
            "ova",
            "oav",
            "special",
            "anime",
        ];

        // Essential release groups (critical ones that must be filtered)
        let essential_release_groups = vec![
            "YIFY", "YTS", "RARBG", "3L", // Critical ones that must be filtered
        ];

        // Essential codecs
        let essential_codecs = vec![
            "x264", "x265", "H264", "H265", "AVC", "HEVC", "10bit", "8bit",
        ];

        // Essential qualities
        let essential_qualities = vec!["720p", "1080p", "2160p", "4K", "HDR", "UHD", "HD", "SD"];

        // Essential sources
        let essential_sources = vec![
            "BluRay", "WEB-DL", "HDTV", "DVDRip", "BRRip", "HDRip", "WEBRip", "REMUX",
        ];

        // Essential audio formats
        let essential_audio = vec![
            "DTS", "AC3", "AAC", "FLAC", "DD5.1", "DTS-HD", "MA", "THD", "TrueHD", "7.1", "5.1",
            "2.0",
        ];

        for term in essential_technical {
            technical_terms.insert(term.to_string());
        }
        for term in essential_release_groups {
            release_groups.insert(term.to_string());
        }
        for term in essential_codecs {
            codecs.insert(term.to_string());
        }
        for term in essential_qualities {
            qualities.insert(term.to_string());
        }
        for term in essential_sources {
            sources.insert(term.to_string());
        }
        for term in essential_audio {
            audio_formats.insert(term.to_string());
        }
    }

    /// Check if a term is an audio format
    fn is_audio_format(term: &str) -> bool {
        let audio_patterns = [
            "DTS", "AC3", "AAC", "FLAC", "DD", "THD", "TrueHD", "MA", "Atmos", "5.1", "7.1", "2.0",
            "Audio", "Audios",
        ];
        audio_patterns
            .iter()
            .any(|&pattern| term.to_uppercase().contains(pattern))
    }

    /// Check if a term is a codec
    fn is_codec(term: &str) -> bool {
        let codec_patterns = [
            "x264", "x265", "H264", "H265", "AVC", "HEVC", "bit", "bits", "VC-1", "DoVi", "HDR10",
            "EDR",
        ];
        codec_patterns
            .iter()
            .any(|&pattern| term.to_uppercase().contains(pattern))
    }

    /// Filter technical terms from a title
    pub fn filter(&self, title: &str) -> String {
        let mut result = title.to_string();

        // Filter all term categories
        for term in &self.technical_terms {
            result = self.remove_term(&result, term);
        }
        for term in &self.release_groups {
            result = self.remove_term(&result, term);
        }
        for term in &self.codecs {
            result = self.remove_term(&result, term);
        }
        for term in &self.qualities {
            result = self.remove_term(&result, term);
        }
        for term in &self.sources {
            result = self.remove_term(&result, term);
        }
        for term in &self.audio_formats {
            result = self.remove_term(&result, term);
        }

        // Clean up extra whitespace and punctuation
        self.clean_title(&result)
    }

    /// Check if a term is considered technical
    pub fn is_technical_term(&self, term: &str) -> bool {
        let term_upper = term.to_uppercase();
        self.technical_terms
            .iter()
            .any(|t| t.to_uppercase() == term_upper)
            || self
                .release_groups
                .iter()
                .any(|t| t.to_uppercase() == term_upper)
            || self.codecs.iter().any(|t| t.to_uppercase() == term_upper)
            || self
                .qualities
                .iter()
                .any(|t| t.to_uppercase() == term_upper)
            || self.sources.iter().any(|t| t.to_uppercase() == term_upper)
            || self
                .audio_formats
                .iter()
                .any(|t| t.to_uppercase() == term_upper)
    }

    /// Get all technical terms
    pub fn get_all_terms(&self) -> Vec<String> {
        let mut all_terms = Vec::new();
        all_terms.extend(self.technical_terms.iter().cloned());
        all_terms.extend(self.release_groups.iter().cloned());
        all_terms.extend(self.codecs.iter().cloned());
        all_terms.extend(self.qualities.iter().cloned());
        all_terms.extend(self.sources.iter().cloned());
        all_terms.extend(self.audio_formats.iter().cloned());
        all_terms
    }

    /// Add custom technical terms
    pub fn add_terms(&mut self, terms: Vec<String>) {
        for term in terms {
            self.technical_terms.insert(term);
        }
    }

    /// Remove technical terms from list
    pub fn remove_terms(&mut self, terms: Vec<String>) {
        for term in terms {
            self.technical_terms.remove(&term);
            self.release_groups.remove(&term);
            self.codecs.remove(&term);
            self.qualities.remove(&term);
            self.sources.remove(&term);
            self.audio_formats.remove(&term);
        }
    }

    /// Remove a specific term from title
    fn remove_term(&self, title: &str, term: &str) -> String {
        let term_upper = term.to_uppercase();
        let title_upper = title.to_uppercase();

        if title_upper.contains(&term_upper) {
            // Use word boundaries to avoid false positives, case insensitive
            let pattern = format!(r"(?i)\b{}\b", regex::escape(term));
            if let Ok(regex) = Regex::new(&pattern) {
                return regex.replace_all(title, "").to_string();
            }
        }
        title.to_string()
    }

    /// Clean up title by removing extra whitespace and punctuation
    fn clean_title(&self, title: &str) -> String {
        let mut result = title.to_string();

        // Replace multiple dots with single dot
        while result.contains("..") {
            result = result.replace("..", ".");
        }

        // Replace multiple dashes with single dash
        while result.contains("--") {
            result = result.replace("--", "-");
        }

        // Replace multiple underscores with single underscore
        while result.contains("__") {
            result = result.replace("__", "_");
        }

        // Replace multiple spaces with single space
        while result.contains("  ") {
            result = result.replace("  ", " ");
        }

        // Remove leading/trailing dots, dashes, underscores, and spaces
        result = result
            .trim_matches(|c| c == '.' || c == '-' || c == '_' || c == ' ')
            .to_string();

        // Split by common separators and filter out empty parts
        let parts: Vec<&str> = result
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.trim().is_empty())
            .collect();

        // Join parts with spaces
        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_basic() {
        let filter = TechnicalTermsFilter::new();
        let result = filter.filter("Movie.1080p.BluRay.x264.mkv");

        assert_eq!(result, "Movie");
    }

    #[test]
    fn test_filter_with_release_group() {
        let filter = TechnicalTermsFilter::new();
        let result = filter.filter("Movie.YIFY.1080p.BluRay.mkv");

        assert_eq!(result, "Movie");
    }

    #[test]
    fn test_filter_with_audio() {
        let filter = TechnicalTermsFilter::new();
        let result = filter.filter("Movie.DTS.AC3.1080p.mkv");

        assert_eq!(result, "Movie");
    }

    #[test]
    fn test_filter_preserve_title() {
        let filter = TechnicalTermsFilter::new();
        let result = filter.filter("The.Matrix.1999.1080p.BluRay.mkv");

        assert_eq!(result, "The Matrix 1999");
    }

    #[test]
    fn test_is_technical_term() {
        let filter = TechnicalTermsFilter::new();

        assert!(filter.is_technical_term("1080p"));
        assert!(filter.is_technical_term("BluRay"));
        assert!(filter.is_technical_term("x264"));
        assert!(!filter.is_technical_term("Movie"));
        assert!(!filter.is_technical_term("Matrix"));
    }

    #[test]
    fn test_filter_with_custom_terms() {
        let mut filter = TechnicalTermsFilter::new();
        filter.add_terms(vec!["CustomTerm".to_string()]);

        let result = filter.filter("Movie.CustomTerm.1080p.mkv");
        assert_eq!(result, "Movie");
    }

    #[test]
    fn test_clean_title() {
        let filter = TechnicalTermsFilter::new();
        let result = filter.filter("Movie...1080p...BluRay...mkv");

        assert_eq!(result, "Movie");
    }

    #[test]
    fn test_filter_3l_release_group() {
        let filter = TechnicalTermsFilter::new();
        let result = filter.filter("The.Batman.2022.2160p.Remux.HEVC.DoVi.TrueHD.7.1-3L.mkv");

        assert_eq!(result, "The Batman 2022");
    }
}
