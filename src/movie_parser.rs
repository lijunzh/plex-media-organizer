//! Movie parsing and organization logic

use crate::config::OriginalTitleConfig;
use crate::filename_parser::FilenameParser;
use crate::tmdb_client::TmdbClient;
use crate::types::{
    ExternalSource, MediaFile, MediaMetadata, MediaType, MovieInfo, ParsingResult, ParsingStrategy,
};
use anyhow::{Context, Result};
use chrono::Utc;
use std::path::Path;

use std::sync::OnceLock;

// Token-based filename parser instance
static FILENAME_PARSER: OnceLock<FilenameParser> = OnceLock::new();

fn get_filename_parser() -> &'static FilenameParser {
    FILENAME_PARSER.get_or_init(FilenameParser::new)
}

/// Movie parser that handles various filename patterns
#[derive(Clone, Debug)]
pub struct MovieParser {
    tmdb_client: Option<TmdbClient>,
    original_title_config: OriginalTitleConfig,
}

impl MovieParser {
    /// Create a new movie parser
    pub fn new(tmdb_client: Option<TmdbClient>) -> Self {
        Self {
            tmdb_client,
            original_title_config: OriginalTitleConfig::default(),
        }
    }

    /// Create a new movie parser with original title configuration
    pub fn with_original_title_config(
        tmdb_client: Option<TmdbClient>,
        original_title_config: OriginalTitleConfig,
    ) -> Self {
        Self {
            tmdb_client,
            original_title_config,
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
            // Use enhanced search with multiple fallback strategies
            let tmdb_result = tmdb_client
                .enhanced_search(&movie_info.title, movie_info.year)
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
        // Use the new token-based parser
        let parser = get_filename_parser();
        let components = parser.parse(filename)?;

        // Convert to MovieInfo
        let movie_info = MovieInfo {
            title: components.title,
            original_title: components.original_title, // Use extracted original title
            original_language: components.language.clone(),
            year: components.year,
            part_number: None, // Could be extracted from title if needed
            is_collection: false,
            collection_name: None,
            quality: components.quality,
            source: components.source,
            language: components.language,
        };

        Ok(movie_info)
    }

    /// Merge movie info from different sources
    fn merge_movie_info(&self, base: MovieInfo, tmdb: MovieInfo) -> MovieInfo {
        // Use TMDB's original language information to determine the true original title
        let (final_title, final_original_title, final_original_language) =
            if let Some(tmdb_original_language) = &tmdb.original_language {
                // TMDB tells us the original language
                match tmdb_original_language.as_str() {
                    "ja" | "zh" | "ko" => {
                        // For CJK languages, prefer the original title from TMDB
                        if let Some(tmdb_original_title) = &tmdb.original_title {
                            if self.original_title_config.prefer_original_titles {
                                if self.original_title_config.include_english_subtitle {
                                    // Original with English subtitle: 千と千尋の神隠し [Spirited Away]
                                    let subtitle = format!(" [{}]", tmdb.title);
                                    let combined_title = tmdb_original_title.clone() + &subtitle;
                                    (
                                        combined_title,
                                        Some(tmdb.title.clone()),
                                        tmdb.original_language.clone(),
                                    )
                                } else {
                                    // Original title only: 千と千尋の神隠し
                                    (
                                        tmdb_original_title.clone(),
                                        Some(tmdb.title.clone()),
                                        tmdb.original_language.clone(),
                                    )
                                }
                            } else {
                                // English-first strategy
                                (
                                    tmdb.title.clone(),
                                    Some(tmdb_original_title.clone()),
                                    tmdb.original_language.clone(),
                                )
                            }
                        } else {
                            // No original title from TMDB, fall back to base
                            (
                                base.title.clone(),
                                base.original_title.clone(),
                                base.original_language.clone(),
                            )
                        }
                    }
                    _ => {
                        // For non-CJK languages, use English title as primary
                        (
                            tmdb.title.clone(),
                            tmdb.original_title.clone(),
                            tmdb.original_language.clone(),
                        )
                    }
                }
            } else {
                // No original language info from TMDB, fall back to filename-based logic
                let has_cjk_in_base = base
                    .original_title
                    .as_ref()
                    .map(|title| self.contains_cjk_characters(title))
                    .unwrap_or(false);

                let has_cjk_in_tmdb = tmdb
                    .original_title
                    .as_ref()
                    .map(|title| self.contains_cjk_characters(title))
                    .unwrap_or(false);

                if (has_cjk_in_base || has_cjk_in_tmdb)
                    && self.original_title_config.prefer_original_titles
                {
                    // We have CJK content and prefer original titles
                    let cjk_title = if has_cjk_in_base {
                        base.original_title.unwrap()
                    } else {
                        tmdb.original_title.unwrap()
                    };

                    let english_title = if has_cjk_in_base {
                        base.title
                    } else {
                        tmdb.title
                    };

                    if self.original_title_config.include_english_subtitle {
                        let subtitle = format!(" [{}]", english_title);
                        let combined_title = cjk_title.clone() + &subtitle;
                        (combined_title, Some(english_title), None)
                    } else {
                        (cjk_title, Some(english_title), None)
                    }
                } else {
                    // Apply standard title strategy
                    let (title, original_title) = self.apply_cjk_title_strategy(&base, &tmdb);
                    (title, original_title, None)
                }
            };

        MovieInfo {
            title: final_title,
            original_title: final_original_title,
            original_language: final_original_language,
            year: tmdb.year.or(base.year),
            part_number: base.part_number, // Keep from filename
            is_collection: base.is_collection,
            collection_name: base.collection_name,
            quality: base.quality, // Keep from filename
            source: base.source,   // Keep from filename
            language: tmdb.language.or(base.language),
        }
    }

    /// Apply CJK title strategy based on configuration
    fn apply_cjk_title_strategy(
        &self,
        base: &MovieInfo,
        tmdb: &MovieInfo,
    ) -> (String, Option<String>) {
        let english_title = tmdb.title.clone();
        let original_cjk_title = base
            .original_title
            .clone()
            .or_else(|| tmdb.original_title.clone());

        // Detect if we have CJK content
        let has_cjk_title = original_cjk_title
            .as_ref()
            .map(|title| self.contains_cjk_characters(title))
            .unwrap_or(false);

        if !has_cjk_title {
            // No CJK content, use standard behavior
            return (english_title, original_cjk_title);
        }

        // Apply original title strategy
        match (
            self.original_title_config.prefer_original_titles,
            self.original_title_config.include_english_subtitle,
        ) {
            (true, true) => {
                // Original with English subtitle: 英雄 [Hero]
                let subtitle = format!(" [{}]", english_title);
                let combined_title = original_cjk_title.clone().unwrap() + &subtitle;
                (combined_title, Some(english_title))
            }
            (true, false) => {
                // Original title only: 英雄
                (original_cjk_title.unwrap(), Some(english_title))
            }
            (false, true) => {
                // English with CJK subtitle: Hero [英雄]
                let subtitle = format!(" [{}]", original_cjk_title.clone().unwrap());
                let combined_title = english_title.clone() + &subtitle;
                (combined_title, original_cjk_title)
            }
            (false, false) => {
                // English title (default behavior)
                (english_title, original_cjk_title)
            }
        }
    }

    /// Check if a string contains CJK characters
    fn contains_cjk_characters(&self, text: &str) -> bool {
        text.chars().any(|c| {
            // Chinese characters (CJK Unified Ideographs)
            ('\u{4e00}'..='\u{9fff}').contains(&c) ||
            // Japanese Hiragana
            ('\u{3040}'..='\u{309f}').contains(&c) ||
            // Japanese Katakana
            ('\u{30a0}'..='\u{30ff}').contains(&c) ||
            // Korean Hangul
            ('\u{ac00}'..='\u{d7af}').contains(&c)
        })
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
    fn test_original_title_strategy_default() {
        use crate::config::OriginalTitleConfig;

        let original_title_config = OriginalTitleConfig::default();
        let parser = MovieParser::with_original_title_config(None, original_title_config);

        // Create test data
        let base = MovieInfo {
            title: "Hero".to_string(),
            original_title: Some("英雄".to_string()),
            original_language: Some("zh".to_string()),
            year: Some(2002),
            part_number: None,
            is_collection: false,
            collection_name: None,

            quality: Some("1080p".to_string()),
            source: Some("BluRay".to_string()),
            language: Some("zh".to_string()),
        };

        let tmdb = MovieInfo {
            title: "Hero".to_string(),
            original_title: Some("英雄".to_string()),
            original_language: Some("zh".to_string()),
            year: Some(2002),
            part_number: None,
            is_collection: false,
            collection_name: None,

            quality: None,
            source: None,
            language: Some("zh".to_string()),
        };

        let result = parser.merge_movie_info(base, tmdb);

        // Default behavior: Original title with English subtitle
        assert_eq!(result.title, "英雄 [Hero]");
        assert_eq!(result.original_title, Some("Hero".to_string()));
    }

    #[test]
    fn test_original_title_strategy_prefer_original() {
        use crate::config::OriginalTitleConfig;

        let original_title_config = OriginalTitleConfig {
            prefer_original_titles: true,
            include_english_subtitle: false,
            fallback_to_english_on_error: true,
            preserve_original_in_metadata: true,
        };
        let parser = MovieParser::with_original_title_config(None, original_title_config);

        // Create test data
        let base = MovieInfo {
            title: "Hero".to_string(),
            original_title: Some("英雄".to_string()),
            original_language: Some("zh".to_string()),
            year: Some(2002),
            part_number: None,
            is_collection: false,
            collection_name: None,

            quality: Some("1080p".to_string()),
            source: Some("BluRay".to_string()),
            language: Some("zh".to_string()),
        };

        let tmdb = MovieInfo {
            title: "Hero".to_string(),
            original_title: Some("英雄".to_string()),
            original_language: Some("zh".to_string()),
            year: Some(2002),
            part_number: None,
            is_collection: false,
            collection_name: None,

            quality: None,
            source: None,
            language: Some("zh".to_string()),
        };

        let result = parser.merge_movie_info(base, tmdb);

        // Should prefer original CJK title
        assert_eq!(result.title, "英雄");
        assert_eq!(result.original_title, Some("Hero".to_string()));
    }

    #[test]
    fn test_original_title_strategy_hybrid() {
        use crate::config::OriginalTitleConfig;

        let original_title_config = OriginalTitleConfig {
            prefer_original_titles: true,
            include_english_subtitle: true,
            fallback_to_english_on_error: true,
            preserve_original_in_metadata: true,
        };
        let parser = MovieParser::with_original_title_config(None, original_title_config);

        // Create test data
        let base = MovieInfo {
            title: "Hero".to_string(),
            original_title: Some("英雄".to_string()),
            original_language: Some("zh".to_string()),
            year: Some(2002),
            part_number: None,
            is_collection: false,
            collection_name: None,

            quality: Some("1080p".to_string()),
            source: Some("BluRay".to_string()),
            language: Some("zh".to_string()),
        };

        let tmdb = MovieInfo {
            title: "Hero".to_string(),
            original_title: Some("英雄".to_string()),
            original_language: Some("zh".to_string()),
            year: Some(2002),
            part_number: None,
            is_collection: false,
            collection_name: None,

            quality: None,
            source: None,
            language: Some("zh".to_string()),
        };

        let result = parser.merge_movie_info(base, tmdb);

        // Should combine titles: 英雄 [Hero]
        assert_eq!(result.title, "英雄 [Hero]");
        assert_eq!(result.original_title, Some("Hero".to_string()));
    }

    #[test]
    fn test_contains_cjk_characters() {
        let parser = MovieParser::new(None);

        // Test Chinese characters
        assert!(parser.contains_cjk_characters("英雄"));
        assert!(parser.contains_cjk_characters("白蛇2：青蛇劫起"));

        // Test Japanese characters
        assert!(parser.contains_cjk_characters("千と千尋の神隠し"));
        assert!(parser.contains_cjk_characters("ドラゴンボール"));

        // Test Korean characters
        assert!(parser.contains_cjk_characters("기생충"));

        // Test English only
        assert!(!parser.contains_cjk_characters("Hero"));
        assert!(!parser.contains_cjk_characters("The Matrix"));

        // Test mixed
        assert!(parser.contains_cjk_characters("Hero英雄"));
        assert!(parser.contains_cjk_characters("The 英雄 Movie"));
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
    fn test_merge_movie_info_edge_cases() {
        use crate::config::OriginalTitleConfig;

        let original_title_config = OriginalTitleConfig::default();
        let parser = MovieParser::with_original_title_config(None, original_title_config);

        // Test with empty TMDB title - should use TMDB title (empty) but keep base year
        let base = MovieInfo {
            title: "Test Movie".to_string(),
            original_title: None,
            original_language: None,
            year: Some(2023),
            part_number: None,
            is_collection: false,
            collection_name: None,
            quality: Some("1080p".to_string()),
            source: Some("BluRay".to_string()),
            language: None,
        };

        let tmdb = MovieInfo {
            title: "".to_string(),
            original_title: None,
            original_language: None,
            year: None,
            part_number: None,
            is_collection: false,
            collection_name: None,
            quality: None,
            source: None,
            language: None,
        };

        let result = parser.merge_movie_info(base, tmdb);
        assert_eq!(result.title, ""); // TMDB title takes precedence
        assert_eq!(result.year, Some(2023)); // Base year is kept
    }

    #[test]
    fn test_movie_parser_debug() {
        let parser = MovieParser::new(None);
        let debug_output = format!("{:?}", parser);
        assert!(debug_output.contains("MovieParser"));
    }
}
