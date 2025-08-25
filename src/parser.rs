//! Parser types and logic for the TMDB-first approach

use crate::config::AppConfig;
use crate::tmdb::UnifiedTmdbClient;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Parsed filename components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilenameComponents {
    pub title: String,
    pub original_title: Option<String>,
    pub year: Option<u32>,
    pub quality: Option<String>,
    pub source: Option<String>,
    pub language: Option<String>,
    pub audio: Option<String>,
    pub codec: Option<String>,
    pub group: Option<String>,
    pub confidence: f32,
}

/// Parser result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserResult<T> {
    pub data: T,
    pub confidence: f32,
    pub parsing_method: String,
}

impl<T> ParserResult<T> {
    pub fn new(data: T, confidence: f32, parsing_method: String) -> Self {
        Self {
            data,
            confidence,
            parsing_method,
        }
    }
}

/// TMDB-first parser implementation
#[derive(Debug, Clone)]
pub struct TmdbFirstParser {
    config: AppConfig,
    tmdb_client: Option<UnifiedTmdbClient>,
}

impl TmdbFirstParser {
    /// Create a new TMDB-first parser
    pub fn new(config: AppConfig) -> Self {
        let tmdb_client = config
            .apis
            .tmdb_api_key
            .as_ref()
            .and_then(|key| UnifiedTmdbClient::new(key.clone()).ok());

        Self {
            config,
            tmdb_client,
        }
    }

    /// Parse a filename using the TMDB-first strategy
    pub async fn parse(&self, filename: &str) -> Result<ParserResult<FilenameComponents>> {
        if filename.is_empty() {
            anyhow::bail!("Cannot parse empty filename");
        }

        // Step 1: Remove file extension
        let filename_without_ext = self.remove_extension(filename);

        // Step 2: Tokenize the filename
        let tokens = self.tokenize(&filename_without_ext);

        // Step 3: Extract clean title and metadata
        let (clean_title, year, quality, source) = self.extract_clean_title_and_metadata(&tokens);

        // Step 4: TMDB search and language detection
        let (final_title, original_language) = self.enhance_with_tmdb(&clean_title, year).await;

        // Step 5: Format final output
        let formatted_title =
            self.format_title(&final_title, year, quality.clone(), &original_language);

        // Step 6: Create result
        let components = FilenameComponents {
            title: formatted_title,
            original_title: None, // Will be set based on language detection
            year,
            quality,
            source,
            language: Some(original_language.clone()),
            audio: None,
            codec: None,
            group: None,
            confidence: self.calculate_confidence(&tokens, &clean_title, year),
        };

        let result = ParserResult::new(
            components.clone(),
            components.confidence,
            "tmdb-first".to_string(),
        );
        Ok(result)
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
        // First, protect audio channels and codec names from being split
        let protected_filename = filename
            .replace("x265", "CODECX265")
            .replace("x264", "CODECX264")
            .replace("H265", "CODECH265")
            .replace("H264", "CODECH264")
            .replace("10bit", "CODEC10BIT")
            .replace("8bit", "CODEC8BIT")
            .replace(".7.1", ".AUDIO71")
            .replace(".5.1", ".AUDIO51")
            .replace(".2.0", ".AUDIO20")
            .replace(".6.1", ".AUDIO61")
            .replace(".7.2", ".AUDIO72");

        let tokens: Vec<String> = protected_filename
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.is_empty())
            .map(|part| part.to_string())
            .collect();

        // Restore audio channels and codecs
        tokens
            .into_iter()
            .map(|token| {
                if token == "AUDIO71" {
                    "7.1".to_string()
                } else if token == "AUDIO51" {
                    "5.1".to_string()
                } else if token == "AUDIO20" {
                    "2.0".to_string()
                } else if token == "AUDIO61" {
                    "6.1".to_string()
                } else if token == "AUDIO72" {
                    "7.2".to_string()
                } else if token == "CODECX264" {
                    "x264".to_string()
                } else if token == "CODECX265" {
                    "x265".to_string()
                } else if token == "CODECH264" {
                    "H264".to_string()
                } else if token == "CODECH265" {
                    "H265".to_string()
                } else if token == "CODEC10BIT" {
                    "10bit".to_string()
                } else if token == "CODEC8BIT" {
                    "8bit".to_string()
                } else {
                    token
                }
            })
            .collect()
    }

    /// Extract clean title and metadata from tokens
    fn extract_clean_title_and_metadata(
        &self,
        tokens: &[String],
    ) -> (String, Option<u32>, Option<String>, Option<String>) {
        let mut title_parts = Vec::new();
        let mut year = None;
        let mut quality = None;
        let mut source = None;

        let technical_terms = self.config.get_all_technical_terms();

        for token in tokens {
            let token_lower = token.to_lowercase();

            // Check for year
            if let Ok(y) = token.parse::<u32>()
                && (1900..=2030).contains(&y)
            {
                year = Some(y);
                continue;
            }

            // Check for quality
            if self.is_quality_token(&token_lower) {
                quality = Some(token.clone());
                continue;
            }

            // Check for source
            if self.is_source_token(&token_lower) {
                source = Some(token.clone());
                continue;
            }

            // Check if it's a technical term to filter out
            if technical_terms.iter().any(|term| {
                let term_lower = term.to_lowercase();
                token_lower == term_lower
            }) || token_lower == "7 1 audio"
                || token_lower == "5 1 audio"
                || token_lower == "2 0 audio"
                || token_lower == "6 1 audio"
                || token_lower == "7 2 audio"
            {
                continue;
            }

            // Keep as title part
            title_parts.push(token.clone());
        }

        let clean_title = title_parts.join(" ");
        (clean_title, year, quality, source)
    }

    /// Check if token is a quality indicator
    fn is_quality_token(&self, token: &str) -> bool {
        let quality_patterns = [
            "1080p", "720p", "2160p", "4k", "uhd", "hd", "sd", "480p", "576p",
        ];
        quality_patterns.contains(&token)
    }

    /// Check if token is a source indicator
    fn is_source_token(&self, token: &str) -> bool {
        let source_patterns = [
            "bluray", "blu-ray", "web-dl", "hdtv", "dvdrip", "brrip", "hdrip", "webrip", "remux",
            "itunes", "web",
        ];
        source_patterns.contains(&token)
    }

    /// Enhance parsing with TMDB data
    async fn enhance_with_tmdb(&self, clean_title: &str, year: Option<u32>) -> (String, String) {
        if let Some(tmdb_client) = &self.tmdb_client
            && let Ok(Some(tmdb_match)) = tmdb_client.find_best_match(clean_title, year).await
            && tmdb_match.confidence_score >= self.config.parsing.tmdb.min_confidence
        {
            let original_language = tmdb_match
                .movie
                .original_language
                .unwrap_or_else(|| "en".to_string());

            // Use TMDB title if confidence is high
            let final_title = if tmdb_match.confidence_score >= 0.7 {
                tmdb_match.movie.title
            } else {
                clean_title.to_string()
            };

            return (final_title, original_language);
        }

        // Fallback: use clean title and detect language
        let original_language = self.detect_language(clean_title);
        (clean_title.to_string(), original_language)
    }

    /// Detect language from title
    fn detect_language(&self, title: &str) -> String {
        // Simple language detection based on character sets
        let has_chinese = Regex::new(r"[\u4e00-\u9fff]").unwrap().is_match(title);
        let has_japanese = Regex::new(r"[\u3040-\u309f\u30a0-\u30ff]")
            .unwrap()
            .is_match(title);
        let has_korean = Regex::new(r"[\uac00-\ud7af]").unwrap().is_match(title);

        if has_chinese {
            "zh".to_string()
        } else if has_japanese {
            "ja".to_string()
        } else if has_korean {
            "ko".to_string()
        } else {
            "en".to_string()
        }
    }

    /// Format final title based on language and configuration
    fn format_title(
        &self,
        title: &str,
        year: Option<u32>,
        quality: Option<String>,
        _original_language: &str,
    ) -> String {
        let mut formatted_title = title.to_string();

        // Add year if configured and available
        if self.config.parsing.output.include_year
            && let Some(y) = year
        {
            formatted_title = format!("{} ({})", formatted_title, y);
        }

        // Add quality if configured and available
        if self.config.parsing.output.include_quality
            && let Some(q) = quality
        {
            formatted_title = format!("{} [{}]", formatted_title, q);
        }

        formatted_title
    }

    /// Calculate confidence score
    fn calculate_confidence(&self, tokens: &[String], clean_title: &str, year: Option<u32>) -> f32 {
        let mut confidence: f32 = 0.5; // Base confidence

        // Boost confidence for longer titles
        if clean_title.len() > 5 {
            confidence += 0.2;
        }

        // Boost confidence if year is found
        if year.is_some() {
            confidence += 0.2;
        }

        // Boost confidence for reasonable token count
        if tokens.len() >= 3 && tokens.len() <= 15 {
            confidence += 0.1;
        }

        confidence.min(1.0)
    }
}

/// Simple parser wrapper for synchronous usage
#[derive(Debug, Clone)]
pub struct MovieParser {
    tmdb_parser: TmdbFirstParser,
}

impl MovieParser {
    /// Create a new movie parser
    pub fn new(config: AppConfig) -> Self {
        Self {
            tmdb_parser: TmdbFirstParser::new(config),
        }
    }

    /// Parse a filename (synchronous wrapper around async parser)
    #[allow(dead_code)]
    pub fn parse(&self, filename: &str) -> Result<ParserResult<FilenameComponents>, anyhow::Error> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async { self.tmdb_parser.parse(filename).await })
    }

    /// Parse a filename (async version)
    pub async fn parse_async(
        &self,
        filename: &str,
    ) -> Result<ParserResult<FilenameComponents>, anyhow::Error> {
        self.tmdb_parser.parse(filename).await
    }
}

// Re-export for compatibility
pub use MovieParser as UnifiedMovieParser;
