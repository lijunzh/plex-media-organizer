//! Technical terms filtering for title extraction

use regex::Regex;

/// Technical terms filter
#[derive(Clone, Debug)]
pub struct TechnicalTermsFilter {
    technical_terms: Vec<String>,
    release_groups: Vec<String>,
    codecs: Vec<String>,
    qualities: Vec<String>,
    sources: Vec<String>,
    audio_formats: Vec<String>,
}

impl Default for TechnicalTermsFilter {
    fn default() -> Self {
        Self {
            technical_terms: vec![
                "mkv".to_string(),
                "mp4".to_string(),
                "avi".to_string(),
                "web".to_string(),
                "dl".to_string(),
                "rip".to_string(),
                "bluray".to_string(),
                "hdtv".to_string(),
                "dvdrip".to_string(),
                "brrip".to_string(),
                "hdrip".to_string(),
                "webrip".to_string(),
                "remux".to_string(),
                "atvp".to_string(),
                "it".to_string(),
                "netflix".to_string(),
                "amazon".to_string(),
                "part".to_string(),
                "cd".to_string(),
                "disc".to_string(),
                "volume".to_string(),
                "vol".to_string(),
                "collection".to_string(),
                "trilogy".to_string(),
                "saga".to_string(),
                "series".to_string(),
                "ova".to_string(),
                "oav".to_string(),
                "special".to_string(),
                "anime".to_string(),
                "dovi".to_string(),
            ],
            release_groups: vec![
                "YIFY".to_string(),
                "YTS".to_string(),
                "RARBG".to_string(),
                "ETRG".to_string(),
                "FUM".to_string(),
                "DIMENSION".to_string(),
                "SPARKS".to_string(),
                "FGT".to_string(),
                "HALCYON".to_string(),
                "REPACK".to_string(),
                "PROPER".to_string(),
                "INTERNAL".to_string(),
                "EXTENDED".to_string(),
                "DIRFIX".to_string(),
                "3L".to_string(),
            ],
            codecs: vec![
                "x264".to_string(),
                "x265".to_string(),
                "H264".to_string(),
                "H265".to_string(),
                "AVC".to_string(),
                "HEVC".to_string(),
                "10bit".to_string(),
                "8bit".to_string(),
            ],
            qualities: vec![
                "720p".to_string(),
                "1080p".to_string(),
                "2160p".to_string(),
                "4K".to_string(),
                "HDR".to_string(),
                "UHD".to_string(),
                "HD".to_string(),
                "SD".to_string(),
            ],
            sources: vec![
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
            audio_formats: vec![
                "DTS".to_string(),
                "AC3".to_string(),
                "AAC".to_string(),
                "FLAC".to_string(),
                "DD5.1".to_string(),
                "DTS-HD".to_string(),
                "MA".to_string(),
                "THD".to_string(),
                "TrueHD".to_string(),
                "7.1".to_string(),
                "5.1".to_string(),
                "2.0".to_string(),
            ],
        }
    }
}

impl TechnicalTermsFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_terms(technical_terms: Vec<String>) -> Self {
        let mut filter = Self::default();
        filter.technical_terms.extend(technical_terms);
        filter
    }

    /// Filter technical terms from title
    pub fn filter(&self, title: &str) -> String {
        let mut filtered = title.to_string();

        // Filter technical terms
        for term in &self.technical_terms {
            filtered = self.remove_term(&filtered, term);
        }

        // Filter release groups
        for group in &self.release_groups {
            filtered = self.remove_term(&filtered, group);
        }

        // Filter codecs
        for codec in &self.codecs {
            filtered = self.remove_term(&filtered, codec);
        }

        // Filter qualities
        for quality in &self.qualities {
            filtered = self.remove_term(&filtered, quality);
        }

        // Filter sources
        for source in &self.sources {
            filtered = self.remove_term(&filtered, source);
        }

        // Filter audio formats
        for audio in &self.audio_formats {
            filtered = self.remove_term(&filtered, audio);
        }

        // Clean up extra whitespace and separators
        filtered = self.clean_title(&filtered);

        filtered
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

    /// Clean up title by removing extra whitespace and separators
    fn clean_title(&self, title: &str) -> String {
        // Split by common separators and filter out empty parts
        let parts: Vec<&str> = title
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.trim().is_empty())
            .collect();

        // Join parts with spaces
        parts.join(" ")
    }

    /// Check if a term is a technical term
    pub fn is_technical_term(&self, term: &str) -> bool {
        let term_lower = term.to_lowercase();

        // Don't treat common words as technical terms
        let common_words = [
            "movie", "film", "the", "and", "or", "of", "in", "on", "at", "to", "for",
        ];
        if common_words.contains(&term_lower.as_str()) {
            return false;
        }

        self.technical_terms
            .iter()
            .any(|t| t.to_lowercase() == term_lower)
            || self
                .release_groups
                .iter()
                .any(|t| t.to_lowercase() == term_lower)
            || self.codecs.iter().any(|t| t.to_lowercase() == term_lower)
            || self
                .qualities
                .iter()
                .any(|t| t.to_lowercase() == term_lower)
            || self.sources.iter().any(|t| t.to_lowercase() == term_lower)
            || self
                .audio_formats
                .iter()
                .any(|t| t.to_lowercase() == term_lower)
    }

    /// Get all technical terms
    pub fn get_all_terms(&self) -> Vec<String> {
        let mut all_terms = Vec::new();
        all_terms.extend(self.technical_terms.clone());
        all_terms.extend(self.release_groups.clone());
        all_terms.extend(self.codecs.clone());
        all_terms.extend(self.qualities.clone());
        all_terms.extend(self.sources.clone());
        all_terms.extend(self.audio_formats.clone());
        all_terms
    }

    /// Add custom technical terms
    pub fn add_terms(&mut self, terms: Vec<String>) {
        self.technical_terms.extend(terms);
    }

    /// Remove technical terms from list
    pub fn remove_terms(&mut self, terms: Vec<String>) {
        for term in terms {
            self.technical_terms.retain(|t| t != &term);
            self.release_groups.retain(|t| t != &term);
            self.codecs.retain(|t| t != &term);
            self.qualities.retain(|t| t != &term);
            self.sources.retain(|t| t != &term);
            self.audio_formats.retain(|t| t != &term);
        }
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
