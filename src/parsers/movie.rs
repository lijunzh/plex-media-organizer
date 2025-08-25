//! Movie parsing module that handles intelligent parsing of movie filenames
//!
//! This module provides comprehensive movie parsing capabilities with support for:
//! - Multiple filename patterns and conventions
//! - External API integration (TMDB)
//! - Fuzzy matching and fallback strategies
//! - CJK (Chinese/Japanese/Korean) title handling
//! - Quality and source detection

use crate::config::AppConfig;
use crate::database::DatabaseManager;
use crate::external::tmdb::UnifiedTmdbClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::{
    extraction::TitleExtractor,
    patterns::{AnimeDetector, SeriesDetector, UnifiedPatternDetector},
    types::{FilenameComponents, ParserResult},
};

/// Database-backed cache entry for parsing results
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    filename: String,
    title: String,
    original_title: Option<String>,
    year: Option<u32>,
    quality: Option<String>,
    source: Option<String>,
    audio: Option<String>,
    codec: Option<String>,
    group: Option<String>,
    language: Option<String>,
    confidence: f32,
    created_at: DateTime<Utc>,
    last_accessed: DateTime<Utc>,
}

/// Unified movie parser that combines all parsing components
#[derive(Debug)]
pub struct MovieParser {
    pattern_detector: UnifiedPatternDetector,
    series_detector: SeriesDetector,
    anime_detector: AnimeDetector,
    legacy_title_extractor: TitleExtractor,
    config: Option<AppConfig>,
    database: Option<DatabaseManager>,
    pub tmdb_client: Option<UnifiedTmdbClient>,
}

impl Clone for MovieParser {
    fn clone(&self) -> Self {
        Self {
            pattern_detector: self.pattern_detector.clone(),
            series_detector: self.series_detector.clone(),
            anime_detector: self.anime_detector.clone(),
            legacy_title_extractor: self.legacy_title_extractor.clone(),
            config: self.config.clone(),
            database: self.database.clone(),
            tmdb_client: self.tmdb_client.clone(),
        }
    }
}

impl Default for MovieParser {
    fn default() -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: None,
            database: None,
            tmdb_client: None,
        }
    }
}

impl MovieParser {
    /// Create a new movie parser
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new movie parser with configuration
    pub fn with_config(config: AppConfig) -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: Some(config),
            database: None,
            tmdb_client: None,
        }
    }

    /// Create a new movie parser with database caching
    pub fn with_database(database: DatabaseManager) -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: None,
            database: Some(database),
            tmdb_client: None,
        }
    }

    /// Create a new movie parser with TMDB client
    pub fn with_tmdb_client(tmdb_client: UnifiedTmdbClient) -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: None,
            database: None,
            tmdb_client: Some(tmdb_client),
        }
    }

    /// Create a new movie parser with all components
    pub fn with_all(
        config: AppConfig,
        database: DatabaseManager,
        tmdb_client: UnifiedTmdbClient,
    ) -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: Some(config),
            database: Some(database),
            tmdb_client: Some(tmdb_client),
        }
    }

    /// Create a new movie parser with configuration and database
    pub fn with_config_and_database(config: AppConfig, database: DatabaseManager) -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: Some(config),
            database: Some(database),
            tmdb_client: None,
        }
    }

    /// Create a new movie parser with configuration and TMDB client
    pub fn with_config_and_tmdb(config: AppConfig, tmdb_client: UnifiedTmdbClient) -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: Some(config),
            database: None,
            tmdb_client: Some(tmdb_client),
        }
    }

    /// Create a new movie parser with configuration, database, and TMDB integration
    pub fn with_config_and_database_and_tmdb(
        config: AppConfig,
        database: DatabaseManager,
        tmdb_client: UnifiedTmdbClient,
    ) -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: Some(config),
            database: Some(database),
            tmdb_client: Some(tmdb_client),
        }
    }

    /// Parse a movie file path
    pub async fn parse_movie<P: AsRef<Path>>(&self, file_path: P) -> Result<ParserResult<FilenameComponents>> {
        let file_path = file_path.as_ref();
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

        self.parse_async(filename).await
    }

    /// Parse a filename asynchronously
    pub async fn parse_async(&self, filename: &str) -> Result<ParserResult<FilenameComponents>> {
        if filename.is_empty() {
            anyhow::bail!("Cannot parse empty filename");
        }

        // Check cache first
        if let Some(cached_result) = self.check_cache(filename).await? {
            return Ok(cached_result);
        }

        // Extract filename components
        let components = self.extract_components(filename)?;

        // Parse using multiple strategies
        let result = self.parse_with_strategies(&components, filename).await?;

        // Cache the result
        if let Some(db) = &self.database {
            self.cache_result(filename, &result, db).await?;
        }

        Ok(result)
    }

    /// Parse a filename synchronously
    pub fn parse(&self, filename: &str) -> Result<ParserResult<FilenameComponents>> {
        // For now, just call the async version and block
        // In a full implementation, this would be a synchronous version
        tokio::runtime::Runtime::new()?.block_on(self.parse_async(filename))
    }

    /// Parse with TMDB integration
    pub async fn parse_with_tmdb(&self, filename: &str) -> Result<ParserResult<FilenameComponents>> {
        // This would use TMDB client for enhanced parsing
        self.parse_async(filename).await
    }

    /// Extract components from filename
    fn extract_components(&self, filename: &str) -> Result<FilenameComponents> {
        // For now, create a basic FilenameComponents structure
        // In a full implementation, this would use the pattern detectors
        let mut components = FilenameComponents::default();
        
        // Extract basic information from filename
        if let Some(title_result) = self.legacy_title_extractor.extract(filename).ok() {
            components.title = title_result.title;
        }
        
        // Extract year using technical patterns
        if let Some(year) = self.pattern_detector.technical().detect_year(filename) {
            components.year = Some(year);
        }
        
        // Extract quality using technical patterns
        if let Some(quality) = self.pattern_detector.technical().detect_quality(filename) {
            components.quality = Some(quality);
        }
        
        // Extract source using technical patterns
        if let Some(source) = self.pattern_detector.technical().detect_source(filename) {
            components.source = Some(source);
        }
        
        Ok(components)
    }

    /// Parse using multiple strategies
    async fn parse_with_strategies(
        &self,
        components: &FilenameComponents,
        filename: &str,
    ) -> Result<ParserResult<FilenameComponents>> {
        let mut best_result = None;

        // Strategy 1: TMDB API lookup
        if let Some(tmdb_client) = &self.tmdb_client {
            if let Ok(result) = self.try_tmdb_lookup(components, tmdb_client).await {
                best_result = Some(result);
            }
        }

        // Strategy 2: Pattern-based parsing
        if best_result.is_none() {
            if let Ok(result) = self.try_pattern_parsing(components) {
                best_result = Some(result);
            }
        }

        // Strategy 3: Legacy parsing
        if best_result.is_none() {
            if let Ok(result) = self.try_legacy_parsing(components) {
                best_result = Some(result);
            }
        }

        best_result.ok_or_else(|| anyhow::anyhow!("Failed to parse movie: {}", filename))
    }

    /// Try TMDB API lookup
    async fn try_tmdb_lookup(
        &self,
        _components: &FilenameComponents,
        _tmdb_client: &UnifiedTmdbClient,
    ) -> Result<ParserResult<FilenameComponents>> {
        // Implementation will be moved from unified.rs
        todo!("Implement TMDB lookup strategy")
    }

    /// Try pattern-based parsing
    fn try_pattern_parsing(&self, _components: &FilenameComponents) -> Result<ParserResult<FilenameComponents>> {
        // Implementation will be moved from unified.rs
        todo!("Implement pattern parsing strategy")
    }

    /// Try legacy parsing
    fn try_legacy_parsing(&self, _components: &FilenameComponents) -> Result<ParserResult<FilenameComponents>> {
        // Implementation will be moved from unified.rs
        todo!("Implement legacy parsing strategy")
    }

    /// Check cache for existing result
    async fn check_cache(&self, _filename: &str) -> Result<Option<ParserResult<FilenameComponents>>> {
        if let Some(_db) = &self.database {
            // Implementation will be moved from unified.rs
            todo!("Implement cache checking")
        }
        Ok(None)
    }

    /// Cache parsing result
    async fn cache_result(
        &self,
        _filename: &str,
        _result: &ParserResult<FilenameComponents>,
        _database: &DatabaseManager,
    ) -> Result<()> {
        // Implementation will be moved from unified.rs
        todo!("Implement result caching")
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> Result<Option<(usize, std::time::Duration)>> {
        // Implementation will be moved from unified.rs
        todo!("Implement cache statistics")
    }
}

// Re-export for backward compatibility
pub use MovieParser as UnifiedMovieParser;