//! Unified parsing logic for all media types

pub mod extraction;
pub mod patterns;
pub mod types;
pub mod movie;

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

// Re-export movie parser
pub use movie::MovieParser;
pub use movie::UnifiedMovieParser; // Backward compatibility

// TODO: Gradually migrate to unified parser structure
// Phase 1.4: Create unified movie parser - COMPLETED
// Phase 1.5: Remove old parser files - COMPLETED
// Phase 4: Clean up compatibility wrappers - COMPLETED
