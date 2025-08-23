//! Pattern detection for filename parsing

pub mod anime;
pub mod language;
pub mod series;
pub mod technical;

pub use anime::AnimeDetector;
pub use language::LanguageDetector;
pub use series::SeriesDetector;
pub use technical::{PatternDetector, TechnicalPatterns};

/// Common pattern detection traits and utilities
pub trait PatternMatcher {
    fn matches(&self, text: &str) -> bool;
    fn extract(&self, text: &str) -> Option<String>;
}

/// Pattern detection result
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern_type: String,
    pub value: String,
    pub confidence: f32,
    pub start: usize,
    pub end: usize,
}

/// Unified pattern detection for all pattern types
#[derive(Clone, Debug)]
pub struct UnifiedPatternDetector {
    technical: TechnicalPatterns,
    series: SeriesDetector,
    anime: AnimeDetector,
    language: LanguageDetector,
}

impl Default for UnifiedPatternDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl UnifiedPatternDetector {
    pub fn new() -> Self {
        Self {
            technical: TechnicalPatterns::new(),
            series: SeriesDetector::new(),
            anime: AnimeDetector::new(),
            language: LanguageDetector::new(),
        }
    }

    pub fn with_technical_terms(technical_terms: Vec<String>) -> Self {
        Self {
            technical: TechnicalPatterns::with_technical_terms(technical_terms),
            series: SeriesDetector::new(),
            anime: AnimeDetector::new(),
            language: LanguageDetector::new(),
        }
    }

    /// Get technical patterns detector
    pub fn technical(&self) -> &TechnicalPatterns {
        &self.technical
    }

    /// Get series detector
    pub fn series(&self) -> &SeriesDetector {
        &self.series
    }

    /// Get anime detector
    pub fn anime(&self) -> &AnimeDetector {
        &self.anime
    }

    /// Get language detector
    pub fn language(&self) -> &LanguageDetector {
        &self.language
    }
}
