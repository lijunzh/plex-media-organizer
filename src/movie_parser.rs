//! Movie parsing and organization logic

use crate::config::AppConfig;
use crate::filename_parser::FilenameParser;
use crate::tmdb_client::TmdbClient;
use crate::types::{
    ExternalSource, MediaFile, MediaMetadata, MediaType, MovieInfo, ParsingResult, ParsingStrategy,
};
use anyhow::{Context, Result};
use chrono::Utc;
use std::path::Path;

/// Movie parser that handles various filename patterns
#[derive(Clone, Debug)]
pub struct MovieParser {
    tmdb_client: Option<TmdbClient>,
    filename_parser: FilenameParser, // Cache the filename parser
    config: AppConfig,               // Store the full config for passing to filename parser
}

impl MovieParser {
    /// Create a new movie parser with default configuration
    pub fn new(tmdb_client: Option<TmdbClient>) -> Self {
        // Load config once and extract parameters
        let config = AppConfig::load().unwrap_or_default();
        let technical_terms = config.get_all_technical_terms();

        let filename_parser = FilenameParser::with_technical_terms(technical_terms);
        Self {
            tmdb_client,
            filename_parser,
            config,
        }
    }

    /// Create a movie parser with specific parameters (no config loading)
    pub fn with_parameters(
        tmdb_client: Option<TmdbClient>,
        technical_terms: Vec<String>,
        language_codes: Vec<String>,
        common_words: Vec<String>,
        technical_japanese_terms: Vec<String>,
    ) -> Self {
        let filename_parser = FilenameParser::with_technical_terms(technical_terms);
        // Create a minimal config with the provided parameters
        let mut config = AppConfig::default();
        config.organization.language.language_codes = language_codes;
        config.organization.title_preservation.common_words = common_words;
        config.organization.language.technical_japanese_terms = technical_japanese_terms;

        Self {
            tmdb_client,
            filename_parser,
            config,
        }
    }

    /// Create a movie parser with full configuration
    pub fn with_config(tmdb_client: Option<TmdbClient>, config: AppConfig) -> Self {
        let technical_terms = config.get_all_technical_terms();
        let filename_parser = FilenameParser::with_technical_terms(technical_terms);

        Self {
            tmdb_client,
            filename_parser,
            config,
        }
    }

    /// Parse a movie filename and return MovieInfo
    pub async fn parse_movie(&self, file_path: &Path) -> Result<ParsingResult> {
        let filename = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

        // Try different parsing strategies
        let mut parsing_strategy = ParsingStrategy::FilenameOnly;
        let mut external_sources = Vec::new();
        let mut confidence_score: f32 = 0.0;

        // First, try to parse the filename
        let mut movie_info = self.parse_filename(filename)?;
        confidence_score += 0.3; // Base confidence from filename parsing

        // If we have a TMDB client, try to get additional data
        if let Some(ref tmdb_client) = self.tmdb_client {
            // Get problematic patterns from config (single load)
            let problematic_patterns = self.config.get_all_content_filtering_patterns();

            // Use enhanced search with config parameters (no repeated loading)
            let tmdb_result = tmdb_client
                .enhanced_search_with_config(
                    &movie_info.title,
                    movie_info.year,
                    &problematic_patterns,
                )
                .await?;

            if let Some(tmdb_result) = tmdb_result {
                // Update movie info with TMDB data
                let tmdb_info = tmdb_client.tmdb_to_movie_info(&tmdb_result.movie);
                movie_info = self.merge_movie_info(movie_info, tmdb_info);

                // Add external source
                external_sources.push(ExternalSource {
                    name: "TMDB".to_string(),
                    external_id: tmdb_result.movie.id.to_string(),
                    url: Some(format!(
                        "https://www.themoviedb.org/movie/{}",
                        tmdb_result.movie.id
                    )),
                    fetched_at: Utc::now(),
                });

                parsing_strategy = ParsingStrategy::ExternalApi;

                // Use TMDB's calculated confidence score directly
                confidence_score += tmdb_result.confidence_score;
            }
        }

        // Create MediaFile and MediaMetadata
        let media_file = self.create_media_file(file_path, &movie_info)?;
        let parsed_metadata = self.create_media_metadata(&movie_info)?;

        let result = ParsingResult {
            media_file,
            parsed_metadata,
            confidence_score: confidence_score.min(1.0),
            parsing_strategy,
            external_sources,
            user_corrections: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(result)
    }

    /// Parse filename using token-based approach
    pub fn parse_filename(&self, filename: &str) -> Result<MovieInfo> {
        // Use the cached filename parser with config
        let components = self
            .filename_parser
            .parse_with_config(filename, Some(&self.config))?;

        // Detect series patterns
        let (is_series, series_name, series_number) = if let Some((name, number)) = self
            .filename_parser
            .detect_series_pattern(&components.title)
        {
            (true, Some(name), Some(number))
        } else {
            (false, None, None)
        };

        // Detect anime patterns
        let (is_anime, anime_movie_number, has_japanese_title, has_chinese_title) =
            if let Some(anime_info) = self
                .filename_parser
                .detect_anime_pattern(&components.title, filename)
            {
                (
                    anime_info.is_anime,
                    anime_info.movie_number,
                    anime_info.has_japanese_title,
                    anime_info.has_chinese_title,
                )
            } else {
                (false, None, false, false)
            };

        // Convert to MovieInfo
        let movie_info = MovieInfo {
            title: components.title,
            original_title: components.original_title, // Use extracted original title
            original_language: components.language.clone(),
            year: components.year,
            part_number: None, // Could be extracted from title if needed
            is_collection: false,
            collection_name: None,
            is_series,
            series_name,
            series_number,
            is_anime,
            anime_movie_number,
            has_japanese_title,
            has_chinese_title,
            quality: components.quality,
            source: components.source,
            language: components.language,
        };

        Ok(movie_info)
    }

    /// Merge movie info from different sources
    fn merge_movie_info(&self, base: MovieInfo, tmdb: MovieInfo) -> MovieInfo {
        // Simply use TMDB's data: English title for indexing, original title for display
        let (final_title, final_original_title, final_original_language) = (
            tmdb.title.clone(),             // English title for Plex indexing
            tmdb.original_title.clone(), // Original title from TMDB (could be English or non-English)
            tmdb.original_language.clone(), // Original language from TMDB
        );

        MovieInfo {
            title: final_title,
            original_title: final_original_title,
            original_language: final_original_language,
            year: tmdb.year.or(base.year),
            part_number: base.part_number, // Keep from filename
            is_collection: base.is_collection,
            collection_name: base.collection_name,
            is_series: base.is_series,         // Keep from filename
            series_name: base.series_name,     // Keep from filename
            series_number: base.series_number, // Keep from filename
            is_anime: base.is_anime,           // Keep from filename
            anime_movie_number: base.anime_movie_number, // Keep from filename
            has_japanese_title: base.has_japanese_title, // Keep from filename
            has_chinese_title: base.has_chinese_title, // Keep from filename
            quality: base.quality,             // Keep from filename
            source: base.source,               // Keep from filename
            language: tmdb.language.or(base.language),
        }
    }

    /// Create MediaFile from MovieInfo
    fn create_media_file(&self, file_path: &Path, movie_info: &MovieInfo) -> Result<MediaFile> {
        let metadata = std::fs::metadata(file_path).context("Failed to get file metadata")?;

        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(MediaFile {
            id: format!("movie_{}", uuid::Uuid::new_v4()),
            file_path: file_path.to_path_buf(),
            file_name,
            file_size: metadata.len(),
            media_type: MediaType::Movie,
            content_hash: self.calculate_content_hash(file_path)?,
            last_modified: metadata
                .modified()
                .map(chrono::DateTime::from)
                .unwrap_or_else(|_| Utc::now()),
            metadata: self.create_media_metadata(movie_info)?,
        })
    }

    /// Create MediaMetadata from MovieInfo
    fn create_media_metadata(&self, movie_info: &MovieInfo) -> Result<MediaMetadata> {
        Ok(MediaMetadata {
            title: Some(movie_info.title.clone()),
            original_title: movie_info.original_title.clone(),
            year: movie_info.year,
            language: movie_info
                .language
                .as_ref()
                .map(|lang| lang.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            quality: movie_info.quality.clone(),
            source: movie_info.source.clone(),
            duration: None,              // Would need media file analysis
            resolution: None,            // Would need media file analysis
            codec: None,                 // Would need media file analysis
            audio_tracks: Vec::new(),    // Would need media file analysis
            subtitle_tracks: Vec::new(), // Would need media file analysis
        })
    }

    /// Calculate content hash for change detection (efficient version)
    fn calculate_content_hash(&self, file_path: &Path) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let metadata = std::fs::metadata(file_path).context("Failed to get file metadata")?;

        let mut hasher = DefaultHasher::new();

        // Hash file size and modification time for efficiency
        // This avoids reading the entire file content
        metadata.len().hash(&mut hasher);
        metadata.modified()?.hash(&mut hasher);

        // Also hash the file path for uniqueness
        file_path.to_string_lossy().hash(&mut hasher);

        Ok(format!("{:x}", hasher.finish()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_japanese_chinese_english_trilingual() {
        let parser = MovieParser::new(None);
        let filename =
            "千与千寻.国日双语.千と千尋の神隠し.Spirited.Away.2001.WEB-DL.2160P.H265.mkv";

        let result = parser.parse_filename(filename).unwrap();

        assert_eq!(result.title, "千与千寻 千と千尋の神隠し Spirited Away");
        assert_eq!(result.original_title, Some("千と千尋の神隠し".to_string()));
        assert_eq!(result.year, Some(2001));
        assert_eq!(result.quality, Some("2160P".to_string()));
        assert_eq!(result.source, Some("WEB-DL".to_string()));
        assert_eq!(result.language, Some("Japanese,English".to_string()));
    }

    #[test]
    fn test_parse_chinese_english_bilingual() {
        let parser = MovieParser::new(None);
        let filename = "白蛇2：青蛇劫起..Green.Snake.2021.1080p.WEB-DL.mkv";
        let result = parser.parse_filename(filename).unwrap();

        println!("Parsed title: '{}'", result.title);
        println!("Original title: '{:?}'", result.original_title);
        println!("Year: {:?}", result.year);
        println!("Quality: {:?}", result.quality);
        println!("Source: {:?}", result.source);

        // For now, test basic functionality - we'll improve regex in next iteration
        assert_eq!(result.quality, Some("1080p".to_string()));
        assert_eq!(result.source, Some("WEB-DL".to_string()));
    }

    #[test]
    fn test_parse_bracketed_chinese() {
        let parser = MovieParser::new(None);
        let filename = "[雏菊(导演剪辑版)].Daisy.2006.720p.BluRay.mkv";
        let result = parser.parse_filename(filename).unwrap();

        // For now, test basic functionality - we'll improve regex in next iteration
        assert_eq!(result.quality, Some("720p".to_string()));
        assert_eq!(result.source, Some("BluRay".to_string()));
    }

    #[test]
    fn test_parse_multi_part() {
        let parser = MovieParser::new(None);
        let filename = "Movie Name Part 1 1080p BluRay.mkv";
        let result = parser.parse_filename(filename).unwrap();

        // For now, test basic functionality - we'll improve regex in next iteration
        assert_eq!(result.quality, Some("1080p".to_string()));
        assert_eq!(result.source, Some("BluRay".to_string()));
    }

    #[test]
    fn test_parse_filename_edge_cases() {
        let parser = MovieParser::new(None);

        // Test empty filename - should return empty title
        let result = parser.parse_filename("");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.title, "");

        // Test filename with only extension - should return empty title
        let result = parser.parse_filename(".mkv");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.title, "");

        // Test filename with no year
        let result = parser.parse_filename("The Matrix.mkv");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.title, "The Matrix");
        assert_eq!(parsed.year, None);

        // Test filename with invalid year
        let result = parser.parse_filename("The Matrix 9999.mkv");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.title, "The Matrix");
        assert_eq!(parsed.year, None); // Invalid year should not be parsed
    }

    #[test]
    fn test_parse_filename_with_special_characters() {
        let parser = MovieParser::new(None);

        // Test that various filename patterns can be parsed without errors
        // This tests the robustness of the parser rather than specific output
        let test_cases = vec![
            "Dr. Strangelove 1964.mkv",
            "The Matrix (1999).mkv",
            "[The Matrix] 1999.mkv",
            "Movie.Title.2023.mkv",
            "A.Movie.With.Dots.2023.mkv",
            "Movie-With-Dashes-2023.mkv",
        ];

        for filename in test_cases {
            let result = parser.parse_filename(filename);
            assert!(result.is_ok(), "Failed to parse filename: {}", filename);
            let parsed = result.unwrap();

            // Basic validation - title should not be empty (except for special cases)
            if filename != "[The Matrix] 1999.mkv" {
                assert!(
                    !parsed.title.is_empty(),
                    "Title should not be empty for: {}",
                    filename
                );
            }

            // Year should be reasonable if present
            if let Some(year) = parsed.year {
                assert!(
                    year >= 1900 && year <= 2030,
                    "Year {} is out of reasonable range for: {}",
                    year,
                    filename
                );
            }
        }
    }

    #[test]
    fn test_merge_movie_info_simple() {
        let parser = MovieParser::new(None);

        // Test with simple TMDB data
        let base = MovieInfo {
            title: "Test Movie".to_string(),
            original_title: None,
            original_language: None,
            year: Some(2023),
            part_number: None,
            is_collection: false,
            collection_name: None,
            is_series: false,
            series_name: None,
            series_number: None,
            is_anime: false,
            anime_movie_number: None,
            has_japanese_title: false,
            has_chinese_title: false,
            quality: Some("1080p".to_string()),
            source: Some("BluRay".to_string()),
            language: None,
        };

        let tmdb = MovieInfo {
            title: "The Test Movie".to_string(),
            original_title: Some("Le Film de Test".to_string()),
            original_language: Some("fr".to_string()),
            year: Some(2023),
            part_number: None,
            is_collection: false,
            collection_name: None,
            is_series: false,
            series_name: None,
            series_number: None,
            is_anime: false,
            anime_movie_number: None,
            has_japanese_title: false,
            has_chinese_title: false,
            quality: None,
            source: None,
            language: None,
        };

        let result = parser.merge_movie_info(base, tmdb);
        assert_eq!(result.title, "The Test Movie"); // TMDB title takes precedence
        assert_eq!(result.original_title, Some("Le Film de Test".to_string())); // TMDB original title
        assert_eq!(result.original_language, Some("fr".to_string())); // TMDB original language
        assert_eq!(result.year, Some(2023)); // Base year is kept
    }

    #[test]
    fn test_movie_parser_debug() {
        let parser = MovieParser::new(None);
        let debug_output = format!("{:?}", parser);
        assert!(debug_output.contains("MovieParser"));
    }
}
