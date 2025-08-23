//! Unified parsing logic for all media types

pub mod extraction;
pub mod patterns;
pub mod types;
pub mod unified;

// Create compatibility wrappers for old parsers
// Note: These wrappers provide backward compatibility while using the new unified parser underneath

// Re-export legacy parsers for backward compatibility (will be deprecated)
#[deprecated(
    since = "0.2.0",
    note = "Use UnifiedMovieParser or CompatFilenameParser instead"
)]
#[allow(deprecated)] // Allow deprecated during migration phase
pub use crate::filename_parser::FilenameParser;
#[deprecated(
    since = "0.2.0",
    note = "Use UnifiedMovieParser or CompatMovieParser instead"
)]
#[allow(deprecated)] // Allow deprecated during migration phase
pub use crate::movie_parser::MovieParser;

// Note: CompatFilenameParser and CompatMovieParser are defined below

// Re-export new parser types
pub use extraction::{ExtractionResult, UnifiedTitleExtractor};
pub use patterns::{PatternMatch, PatternMatcher, UnifiedPatternDetector};
pub use types::{
    AnimeInfo, FilenameComponents, LanguageInfo, ParserConfig, ParserResult, SeriesInfo,
};

// Re-export legacy pattern detection methods for backward compatibility
pub use patterns::anime::AnimeDetector;
pub use patterns::series::SeriesDetector;

// Re-export legacy title extraction methods for backward compatibility
pub use extraction::title::TitleExtractor;

// Re-export unified movie parser
pub use unified::UnifiedMovieParser;

/// Compatibility wrapper for FilenameParser using the new unified parser
#[allow(deprecated)] // Allow deprecated during migration phase
pub struct CompatFilenameParser {
    unified_parser: UnifiedMovieParser,
}

impl CompatFilenameParser {
    pub fn new() -> Self {
        Self {
            unified_parser: UnifiedMovieParser::new(),
        }
    }

    pub fn with_technical_terms(technical_terms: Vec<String>) -> Self {
        // For now, we create the unified parser with default config
        // TODO: Pass technical terms to the unified parser when config support is added
        let _ = technical_terms; // Suppress unused warning
        Self::new()
    }

    #[allow(deprecated)] // Allow deprecated during migration phase
    pub fn parse(
        &self,
        filename: &str,
    ) -> anyhow::Result<crate::filename_parser::FilenameComponents> {
        let result = self.unified_parser.parse(filename)?;

        // Convert unified parser result to legacy format
        Ok(crate::filename_parser::FilenameComponents {
            title: result.data.title,
            original_title: result.data.original_title,
            year: result.data.year,
            quality: result.data.quality,
            source: result.data.source,
            language: result.data.language,
            audio: result.data.audio,
            codec: result.data.codec,
            group: result.data.group,
            confidence: result.data.confidence,
        })
    }
}

impl Default for CompatFilenameParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Compatibility wrapper for MovieParser using the new unified parser
#[allow(deprecated)] // Allow deprecated during migration phase
#[allow(dead_code)] // Allow dead code during migration phase
pub struct CompatMovieParser {
    unified_parser: UnifiedMovieParser,
    tmdb_client: Option<crate::external::tmdb::UnifiedTmdbClient>,
}

impl CompatMovieParser {
    pub fn new(tmdb_client: Option<crate::external::tmdb::UnifiedTmdbClient>) -> Self {
        Self {
            unified_parser: UnifiedMovieParser::new(),
            tmdb_client,
        }
    }

    pub fn parse_filename(&self, filename: &str) -> anyhow::Result<crate::types::MovieInfo> {
        let result = self.unified_parser.parse(filename)?;

        // Convert unified parser result to legacy MovieInfo format
        Ok(crate::types::MovieInfo {
            title: result.data.title.clone(),
            original_title: result.data.original_title.clone(),
            original_language: None, // TODO: Detect original language from unified parser
            year: result.data.year,
            part_number: None,    // TODO: Extract part number from series detection
            is_collection: false, // TODO: Extract from series detection
            collection_name: None, // TODO: Extract from series detection
            is_series: false,     // TODO: Extract from series detection
            series_name: None,    // TODO: Extract from series detection
            series_number: None,  // TODO: Extract from series detection
            is_anime: false,      // TODO: Extract from anime detection
            anime_movie_number: None, // TODO: Extract from anime detection
            has_japanese_title: false, // TODO: Extract from language detection
            has_chinese_title: false, // TODO: Extract from language detection
            quality: result.data.quality.clone(),
            source: result.data.source.clone(),
            language: result.data.language.clone(),
        })
    }
}

// TODO: Gradually migrate to unified parser structure
// Phase 1.4: Create unified movie parser - COMPLETED
// Phase 1.5: Remove old parser files - IN PROGRESS
