//! Unified movie parser that integrates all modular parsing components

use crate::config::AppConfig;
use crate::database::{DatabaseManager, cache::CacheRepository};
use crate::external::tmdb::UnifiedTmdbClient;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
pub struct UnifiedMovieParser {
    pattern_detector: UnifiedPatternDetector,
    series_detector: SeriesDetector,
    anime_detector: AnimeDetector,
    legacy_title_extractor: TitleExtractor,
    config: Option<AppConfig>,
    database: Option<DatabaseManager>,
    pub tmdb_client: Option<UnifiedTmdbClient>,
}

impl Clone for UnifiedMovieParser {
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

impl Default for UnifiedMovieParser {
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

impl UnifiedMovieParser {
    /// Create a new unified movie parser
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new unified movie parser with configuration
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

    /// Create a new unified movie parser with database caching
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

    /// Create a new unified movie parser with TMDB integration
    pub fn with_tmdb(tmdb_client: UnifiedTmdbClient) -> Self {
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

    /// Create a new unified movie parser with configuration and TMDB integration
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

    /// Create a new unified movie parser with configuration, database, and TMDB integration
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

    /// Create a new unified movie parser with both config and database
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

    /// Parse a filename using the unified parser with optional database caching
    pub async fn parse_async(&self, filename: &str) -> Result<ParserResult<FilenameComponents>> {
        if filename.is_empty() {
            anyhow::bail!("Cannot parse empty filename");
        }

        // Try to get from cache first
        if let Some(cached_result) = self.get_from_cache(filename).await? {
            return Ok(cached_result);
        }
        // Remove file extension once
        let filename_without_ext = self.remove_extension(filename);

        // Tokenize the filename once
        let tokens = self.tokenize(&filename_without_ext);

        // Extract technical components in one pass
        let technical_detector = self.pattern_detector.technical();
        let year = technical_detector.detect_year(&filename_without_ext);
        let quality = technical_detector.detect_quality(&filename_without_ext);
        let source = technical_detector.detect_source(&filename_without_ext);
        let audio = technical_detector.detect_audio(&filename_without_ext);
        let codec = technical_detector.detect_codec(&filename_without_ext);

        // Extract language information
        let language_info = self
            .pattern_detector
            .language()
            .detect_language(&filename_without_ext);
        let language = language_info.primary_language;

        // Group detection (simplified for now)
        let group = None; // Could be enhanced later

        // Get configuration parameters once
        let language_codes = self.get_language_codes();
        let common_words = self.get_common_words();
        let known_titles = self.get_known_titles();
        let technical_japanese_terms = self.get_technical_japanese_terms();
        let release_groups = self.get_release_groups();
        let technical_terms = self.get_technical_terms();

        // Extract title and original title using legacy method for compatibility
        let (title, original_title) = self
            .legacy_title_extractor
            .extract_title_and_original_with_params(
                &tokens,
                &year,
                &quality,
                &source,
                &audio,
                &codec,
                &group,
                &language,
                filename,
                &common_words,
                &known_titles,
                &technical_japanese_terms,
                &language_codes,
                &technical_terms,
                &release_groups,
            );

        // Calculate confidence score based on extracted information
        let confidence = self.calculate_confidence(&tokens, &year, &quality, &source, &title);

        // Create filename components
        let components = FilenameComponents {
            title,
            original_title,
            year,
            quality,
            source,
            language,
            audio,
            codec,
            group,
            confidence,
        };

        // Create parser result
        let result = ParserResult::new(components, confidence, "unified".to_string());

        // Store in cache if database is available
        if let Err(e) = self.store_in_cache(filename, &result).await {
            // Log cache error but don't fail the parse
            eprintln!("Warning: Failed to cache parsing result: {}", e);
        }

        Ok(result)
    }

    /// Parse a filename using the unified parser with TMDB enhancement (async version)
    /// This version uses TMDB integration for improved confidence scoring
    pub async fn parse_with_tmdb(
        &self,
        filename: &str,
    ) -> Result<ParserResult<FilenameComponents>> {
        if filename.is_empty() {
            anyhow::bail!("Cannot parse empty filename");
        }

        // First, do the basic parsing
        let mut result = self.parse(filename)?;

        // Enhance with TMDB data if available
        if let Some(tmdb_client) = &self.tmdb_client {
            if let Ok(Some(tmdb_match)) = tmdb_client
                .find_best_match(&result.data.title, result.data.year)
                .await
            {
            // Boost confidence based on TMDB match quality
            let enhanced_confidence =
                (result.data.confidence + tmdb_match.confidence_score).min(1.0);
            result.data.confidence = enhanced_confidence;

            // Update parsing method to indicate TMDB enhancement
            result.parsing_method = "unified+tmdb".to_string();
        }
        }

        Ok(result)
    }

    /// Parse a filename using the unified parser (synchronous version for backward compatibility)
    /// This version does not use database caching
    pub fn parse(&self, filename: &str) -> Result<ParserResult<FilenameComponents>> {
        if filename.is_empty() {
            anyhow::bail!("Cannot parse empty filename");
        }

        // Remove file extension once
        let filename_without_ext = self.remove_extension(filename);

        // Tokenize the filename once
        let tokens = self.tokenize(&filename_without_ext);

        // Extract technical components in one pass
        let technical_detector = self.pattern_detector.technical();
        let year = technical_detector.detect_year(&filename_without_ext);
        let quality = technical_detector.detect_quality(&filename_without_ext);
        let source = technical_detector.detect_source(&filename_without_ext);
        let audio = technical_detector.detect_audio(&filename_without_ext);
        let codec = technical_detector.detect_codec(&filename_without_ext);

        // Extract language information
        let language_info = self
            .pattern_detector
            .language()
            .detect_language(&filename_without_ext);
        let language = language_info.primary_language;

        // Group detection (simplified for now)
        let group = None; // Could be enhanced later

        // Get configuration parameters once
        let language_codes = self.get_language_codes();
        let common_words = self.get_common_words();
        let known_titles = self.get_known_titles();
        let technical_japanese_terms = self.get_technical_japanese_terms();
        let release_groups = self.get_release_groups();
        let technical_terms = self.get_technical_terms();

        // Extract title and original title using legacy method for compatibility
        let (title, original_title) = self
            .legacy_title_extractor
            .extract_title_and_original_with_params(
                &tokens,
                &year,
                &quality,
                &source,
                &audio,
                &codec,
                &group,
                &language,
                filename,
                &common_words,
                &known_titles,
                &technical_japanese_terms,
                &language_codes,
                &technical_terms,
                &release_groups,
            );

        // Calculate confidence score based on extracted information
        let confidence = self.calculate_confidence(&tokens, &year, &quality, &source, &title);

        // Create filename components
        let components = FilenameComponents {
            title,
            original_title,
            year,
            quality,
            source,
            language,
            audio,
            codec,
            group,
            confidence,
        };

        // Create parser result
        let result = ParserResult::new(components, confidence, "unified".to_string());

        Ok(result)
    }

    /// Parse a movie file and return a full ParsingResult (compatibility with MovieParser)
    pub async fn parse_movie(
        &self,
        file_path: &std::path::Path,
    ) -> Result<crate::types::ParsingResult> {
        // Validate input
        if !file_path.exists() {
            anyhow::bail!("File does not exist: {}", file_path.display());
        }

        let filename = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .with_context(|| format!("Invalid filename for path: {}", file_path.display()))?;

        // Parse the filename using the unified parser
        let parser_result = self
            .parse_async(filename)
            .await
            .with_context(|| format!("Failed to parse filename: {}", filename))?;
        let components = parser_result.data;

        // Extract additional metadata using the detection capabilities
        let filename_without_ext = self.remove_extension(filename);

        // Detect series information
        let series_info = self.series_detector.detect_series(filename);
        let is_series = series_info.is_series;
        let series_number = series_info.series_number;
        let series_name = if is_series {
            // Extract series name from the title (simplified approach)
            Some(components.title.clone())
        } else {
            None
        };

        // Detect anime information
        let anime_info = self
            .anime_detector
            .detect_anime_pattern(&components.title, filename);
        let (is_anime, anime_movie_number, has_japanese_title, has_chinese_title) =
            if let Some(info) = anime_info {
                (
                    info.is_anime,
                    info.movie_number,
                    info.has_japanese_title,
                    info.has_chinese_title,
                )
            } else {
                (false, None, false, false)
            };

        // Detect language information
        let language_info = self
            .pattern_detector
            .language()
            .detect_language(&filename_without_ext);
        let has_japanese = language_info.has_japanese || has_japanese_title;
        let has_chinese = language_info.has_chinese || has_chinese_title;

        // Convert to MovieInfo
        let movie_info = crate::media::MovieInfo {
            title: components.title,
            original_title: components.original_title,
            original_language: components.language.clone(),
            year: components.year,
            part_number: None, // Could be extracted from series detection if needed
            is_collection: false, // Could be extracted from series detection if needed
            collection_name: None, // Could be extracted from series detection if needed
            is_series,
            series_name,
            series_number,
            is_anime,
            anime_movie_number,
            has_japanese_title: has_japanese,
            has_chinese_title: has_chinese,
            quality: components.quality,
            source: components.source,
            language: components.language,
        };

        // Create MediaFile
        let metadata = std::fs::metadata(file_path)
            .with_context(|| format!("Failed to get file metadata for: {}", file_path.display()))?;

        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let media_file = crate::types::MediaFile {
            id: format!("movie_{}", uuid::Uuid::new_v4()),
            file_path: file_path.to_path_buf(),
            file_name,
            file_size: metadata.len(),
            media_type: crate::types::MediaType::Video,
            content_hash: format!("{:x}", md5::compute(filename.as_bytes())),
            last_modified: chrono::DateTime::from(
                metadata
                    .modified()
                    .unwrap_or_else(|_| std::time::SystemTime::now()),
            ),
            metadata: crate::types::MediaMetadata::default(), // Will be filled below
        };

        // Create MediaMetadata
        let parsed_metadata = crate::types::MediaMetadata {
            title: Some(movie_info.title.clone()),
            original_title: movie_info.original_title.clone(),
            year: movie_info.year,
            language: movie_info
                .language
                .as_ref()
                .map(|l| vec![l.clone()])
                .unwrap_or_default(),
            quality: movie_info.quality,
            source: movie_info.source,
            duration: None,
            resolution: None,
            codec: components.codec,
            audio_tracks: Vec::new(),
            subtitle_tracks: Vec::new(),
        };

        // Create ParsingResult
        let result = crate::types::ParsingResult {
            media_file,
            parsed_metadata,
            confidence_score: components.confidence,
            parsing_strategy: crate::types::ParsingStrategy::FilenameOnly,
            external_sources: Vec::new(),
            user_corrections: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        Ok(result)
    }

    /// Get cached parsing result from database
    async fn get_from_cache(
        &self,
        filename: &str,
    ) -> Result<Option<ParserResult<FilenameComponents>>> {
        if let Some(db) = &self.database {
            let filename_hash = self.create_filename_hash(filename);
            let conn = db.connection().await;
            let cache_repo = CacheRepository::new(&conn);

            if let Some(cache_entry) = cache_repo.get_parsing_result(&filename_hash)? {
                // Deserialize the cached data
                let components: FilenameComponents = serde_json::from_str(&cache_entry.parsed_data)
                    .with_context(|| "Failed to deserialize cached parsing result")?;

                let result = ParserResult::new(
                    components.clone(),
                    components.confidence,
                    "unified_cached".to_string(),
                );
                return Ok(Some(result));
            }
        }
        Ok(None)
    }

    /// Store parsing result in database cache
    async fn store_in_cache(
        &self,
        filename: &str,
        result: &ParserResult<FilenameComponents>,
    ) -> Result<()> {
        if let Some(db) = &self.database {
            let filename_hash = self.create_filename_hash(filename);
            let serialized_data = serde_json::to_string(&result.data)
                .with_context(|| "Failed to serialize parsing result for cache")?;

            let conn = db.connection().await;
            let cache_repo = CacheRepository::new(&conn);
            cache_repo.store_parsing_result(&filename_hash, &serialized_data)?;
        }
        Ok(())
    }

    /// Create a hash for the filename to use as cache key
    fn create_filename_hash(&self, filename: &str) -> String {
        let mut hasher = DefaultHasher::new();
        filename.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Clear the parser cache (if database is available)
    pub async fn clear_cache(&self) -> Result<usize> {
        if let Some(db) = &self.database {
            let conn = db.connection().await;
            let cache_repo = CacheRepository::new(&conn);
            cache_repo.clear_all_cache()
        } else {
            Ok(0)
        }
    }

    /// Get cache statistics (if database is available)
    pub async fn get_cache_stats(&self) -> Result<Option<crate::database::cache::CacheStats>> {
        if let Some(db) = &self.database {
            let conn = db.connection().await;
            let cache_repo = CacheRepository::new(&conn);
            Ok(Some(cache_repo.get_cache_stats()?))
        } else {
            Ok(None)
        }
    }

    /// Remove file extension from filename
    fn remove_extension(&self, filename: &str) -> String {
        if let Some(dot_pos) = filename.rfind('.') {
            filename[..dot_pos].to_string()
        } else {
            filename.to_string()
        }
    }

    /// Tokenize filename into parts
    fn tokenize(&self, filename: &str) -> Vec<String> {
        filename
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.is_empty())
            .map(|part| part.to_string())
            .collect()
    }

    /// Calculate confidence score for parsing result
    fn calculate_confidence(
        &self,
        _tokens: &[String],
        year: &Option<u32>,
        quality: &Option<String>,
        source: &Option<String>,
        title: &str,
    ) -> f32 {
        let mut confidence: f32 = 0.0;

        // Base confidence
        confidence += 0.2;

        // Year found (strong indicator)
        if year.is_some() {
            confidence += 0.3;
        }

        // Quality found (good indicator)
        if quality.is_some() {
            confidence += 0.2;
        }

        // Source found (good indicator)
        if source.is_some() {
            confidence += 0.2;
        }

        // Title quality assessment
        if !title.is_empty() {
            confidence += 0.1;

            // Title length (reasonable length)
            let title_words = title.split_whitespace().count();
            if (1..=8).contains(&title_words) {
                confidence += 0.1;
            }

            // Title doesn't contain too many technical terms
            let technical_term_count =
                ["x264", "x265", "BluRay", "WEB-DL", "1080p", "720p", "2160p"]
                    .iter()
                    .filter(|&term| title.contains(term))
                    .count();
            if technical_term_count <= 1 {
                confidence += 0.1;
            }
        }

        confidence.clamp(0.0, 1.0)
    }

    /// Get language codes from config or default
    fn get_language_codes(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_language_codes())
            .unwrap_or_else(|| {
                vec![
                    "JPN".to_string(),
                    "ENG".to_string(),
                    "CHI".to_string(),
                    "KOR".to_string(),
                    "JAP".to_string(),
                    "EN".to_string(),
                    "CN".to_string(),
                ]
            })
    }

    /// Get common words from config or default
    fn get_common_words(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_common_words())
            .unwrap_or_else(|| vec!["The".to_string(), "A".to_string(), "An".to_string()])
    }

    /// Get known titles from config or default
    fn get_known_titles(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_known_titles())
            .unwrap_or_else(|| {
                vec![
                    "灌篮高手".to_string(),
                    "灌篮".to_string(),
                    "Slam".to_string(),
                    "Dunk".to_string(),
                ]
            })
    }

    /// Get technical Japanese terms from config or default
    fn get_technical_japanese_terms(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_technical_japanese_terms())
            .unwrap_or_else(|| {
                vec![
                    "国日双语".to_string(),
                    "双语".to_string(),
                    "国日".to_string(),
                    "日英".to_string(),
                    "英日".to_string(),
                    "中日".to_string(),
                    "日中".to_string(),
                ]
            })
    }

    /// Get release groups from config or default
    fn get_release_groups(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_release_groups())
            .unwrap_or_default()
    }

    /// Get technical terms from config or default
    fn get_technical_terms(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_all_technical_terms())
            .unwrap_or_else(|| {
                vec![
                    // Video/audio codecs and quality
                    "10bit".to_string(),
                    "10bits".to_string(),
                    "bit".to_string(),
                    "bits".to_string(),
                    "DDP".to_string(),
                    "DTS".to_string(),
                    "AC3".to_string(),
                    "AAC".to_string(),
                    "FLAC".to_string(),
                    "THD".to_string(),
                    "MA".to_string(),
                    "HD".to_string(),
                    "x264".to_string(),
                    "x265".to_string(),
                    "H264".to_string(),
                    "H265".to_string(),
                    "AVC".to_string(),
                    "HEVC".to_string(),
                    "Atmos".to_string(),
                    "TrueHD".to_string(),
                    "DualAudio".to_string(),
                    "2Audio".to_string(),
                    "2Audios".to_string(),
                    "4Audios".to_string(),
                    "60fps".to_string(),
                    "HQ".to_string(),
                    "AAC(5".to_string(),
                    "1)".to_string(),
                    "Hi10P".to_string(),
                    "DD5".to_string(),
                    "TrueHD7".to_string(),
                    "H".to_string(),
                    "264".to_string(),
                    "265".to_string(),
                    "4Audio".to_string(),
                    "3Audio".to_string(),
                    "5Audio".to_string(),
                    "REPACK".to_string(),
                    "Remux".to_string(),
                    "VC-1".to_string(),
                    "DoVi".to_string(),
                    "HDR10".to_string(),
                    "EDR".to_string(),
                    "MULTi".to_string(),
                    "HDTS".to_string(),
                    "IMAX".to_string(),
                    "DSNP".to_string(),
                    "DTS-HD".to_string(),
                    "HDR".to_string(),
                    "120FPS".to_string(),
                    "4K".to_string(),
                    "WEB".to_string(),
                    "WEBRip".to_string(),
                    "UHD".to_string(),
                    "Blu-ray".to_string(),
                    "Bluray".to_string(),
                    "BluRay".to_string(),
                    "DD5".to_string(),
                    "DD+".to_string(),
                    "AC3".to_string(),
                    "AAC5".to_string(),
                    "AAC1".to_string(),
                    "10bit".to_string(),
                    "DV".to_string(),
                    "MP4".to_string(),
                    "MKV".to_string(),
                    // Source/platform names
                    "NF".to_string(),
                    "AMZN".to_string(),
                    "HKG".to_string(),
                    "ESP".to_string(),
                    "GBR".to_string(),
                    "INT".to_string(),
                    "JPN".to_string(),
                    "CHN".to_string(),
                    "CCTV6HD".to_string(),
                    "CHC".to_string(),
                    "Movie-HD".to_string(),
                    "AKA".to_string(),
                    "Chinese".to_string(),
                    "iTunes".to_string(),
                    "AMZN".to_string(),
                    "NF".to_string(),
                    "Netflix".to_string(),
                    "HMAX".to_string(),
                    "NOW".to_string(),
                    "ATVP".to_string(),
                    "HULU".to_string(),
                    "DSNP".to_string(),
                    // File formats and containers
                    "HDTVRip".to_string(),
                    "DVDRip".to_string(),
                    "BDRip".to_string(),
                    "HDRip".to_string(),
                    "WEBRip".to_string(),
                    "HDTV".to_string(),
                    "MP3".to_string(),
                    // Special editions and versions
                    "EXTENDED".to_string(),
                    "修复加长版".to_string(),
                    "导演剪辑版".to_string(),
                    "Extended".to_string(),
                    "RERIP".to_string(),
                    "Hybrid".to_string(),
                    "ES".to_string(),
                    // Release groups
                    "CMCT".to_string(),
                    "WiKi".to_string(),
                    "FRDS".to_string(),
                    "HDS".to_string(),
                    "ADWeb".to_string(),
                    "TLF".to_string(),
                    "CHDWEB".to_string(),
                    "PTerWEB".to_string(),
                    "GREENOTEA".to_string(),
                    "ZmWeb".to_string(),
                    "HDVWEB".to_string(),
                    "NukeHD".to_string(),
                    "TJUPT".to_string(),
                    "CMCTV".to_string(),
                    "NTG".to_string(),
                    "HDWTV".to_string(),
                    "NowOur".to_string(),
                    "PandaQT".to_string(),
                    "HANDJOB".to_string(),
                    "npuer".to_string(),
                    "BYRHD".to_string(),
                    "c0kE".to_string(),
                    "TBMovies".to_string(),
                    "MNHD".to_string(),
                    "YTS".to_string(),
                    "MX".to_string(),
                    "HDWinG".to_string(),
                    "NYPAD".to_string(),
                    "ZigZag".to_string(),
                    "NTb".to_string(),
                    "REMUX".to_string(),
                    "iT".to_string(),
                    "mUHD".to_string(),
                    "IAMABLE".to_string(),
                    "KRaLiMaRKo".to_string(),
                    "HDChina".to_string(),
                    "CtrlHD".to_string(),
                    "SWTYBLZ".to_string(),
                    "ADE".to_string(),
                    "PHOBOS".to_string(),
                    "PTHOME".to_string(),
                    "SyncUP".to_string(),
                    "YIFY".to_string(),
                    "SPARKS".to_string(),
                    "HiDt".to_string(),
                    "Geek".to_string(),
                    "TayTO".to_string(),
                    "nikt0".to_string(),
                    "beAst".to_string(),
                    "FoRM".to_string(),
                    "CRiME".to_string(),
                    "HVAC".to_string(),
                    "MaoZhan".to_string(),
                    "VietHD".to_string(),
                    "JYK".to_string(),
                    "GalaxyRG265".to_string(),
                    "PaODEQUEiJO".to_string(),
                    "SA89".to_string(),
                    "FANDANGO".to_string(),
                    "PTer".to_string(),
                    "ABM".to_string(),
                    "MZABI".to_string(),
                    "BYRPAD".to_string(),
                    "NCmt".to_string(),
                    "MTeam".to_string(),
                    "playWEB".to_string(),
                    "FLUX".to_string(),
                    "CMRG".to_string(),
                    "MZABARBiE".to_string(),
                    "SMURF".to_string(),
                    "AREY".to_string(),
                    "RABiDS".to_string(),
                    "ETHEL".to_string(),
                    "RightSiZE".to_string(),
                    "CiNEPHiLES".to_string(),
                    "Kitsune".to_string(),
                    "KBTV".to_string(),
                    "EbP".to_string(),
                ]
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_parser_basic() {
        let parser = UnifiedMovieParser::new();
        let result = parser
            .parse("The.Matrix.1999.1080p.BluRay.x264.mkv")
            .unwrap();

        assert_eq!(result.data.title, "The Matrix");
        assert_eq!(result.data.year, Some(1999));
        assert_eq!(result.data.quality, Some("1080p".to_string()));
        assert_eq!(result.data.source, Some("BluRay".to_string()));
        assert!(result.data.confidence > 0.5);
    }

    #[test]
    fn test_unified_parser_chinese_bilingual() {
        let parser = UnifiedMovieParser::new();
        let result = parser.parse("[BD-1080P] [名探偵コナン 緋色の弾丸] Detective Conan The Scarlet Bullet (2021) [BDRip][HEVC-10bit][1080p][CHS&CHT&ENG].mkv").unwrap();

        assert!(result.data.title.contains("Detective Conan"));
        assert_eq!(result.data.year, Some(2021));
        assert_eq!(result.data.quality, Some("1080p".to_string()));
    }

    #[test]
    fn test_unified_parser_series() {
        let parser = UnifiedMovieParser::new();
        let result = parser.parse("Iron.Man.Part.2.2010.1080p.mkv").unwrap();

        assert_eq!(result.data.title, "Iron Man Part");
        assert_eq!(result.data.year, Some(2010));
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_unified_parser_anime() {
        let parser = UnifiedMovieParser::new();
        let result = parser.parse("アニメ.Movie.2.1080p.mkv").unwrap();

        assert!(result.data.title.contains("アニメ"));
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_unified_parser_language_detection() {
        let parser = UnifiedMovieParser::new();
        let result = parser.parse("Movie.ENG.JPN.1080p.mkv").unwrap();

        assert!(result.data.title.contains("Movie"));
        assert!(result.confidence > 0.3);
    }

    #[tokio::test]
    async fn test_unified_parser_with_caching() {
        use crate::database::DatabaseManager;
        use tempfile::NamedTempFile;

        // Create a temporary database for testing
        let temp_file = NamedTempFile::new().unwrap();
        let db_manager = DatabaseManager::new(temp_file.path()).await.unwrap();

        // Create parser with database
        let parser = UnifiedMovieParser::with_database(db_manager);

        let filename = "The.Matrix.1999.1080p.BluRay.x264.mkv";

        // First parse - should store in cache
        let result1 = parser.parse_async(filename).await.unwrap();
        assert_eq!(result1.data.title, "The Matrix");
        assert_eq!(result1.parsing_method, "unified");

        // Second parse - should retrieve from cache
        let result2 = parser.parse_async(filename).await.unwrap();
        assert_eq!(result2.data.title, "The Matrix");
        assert_eq!(result2.parsing_method, "unified_cached");

        // Results should be identical except for parsing method
        assert_eq!(result1.data.year, result2.data.year);
        assert_eq!(result1.data.quality, result2.data.quality);
        assert_eq!(result1.data.source, result2.data.source);
    }
}
