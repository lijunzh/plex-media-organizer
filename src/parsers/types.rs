//! Parser-specific types and data structures

/// Tokenized components of a filename
#[derive(Debug, Clone)]
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

/// Anime movie information extracted from filename
#[derive(Debug, Clone)]
pub struct AnimeInfo {
    pub is_anime: bool,
    pub movie_number: Option<u32>,
    pub has_japanese_title: bool,
    pub has_chinese_title: bool,
    pub is_movie_series: bool,
}

/// Series information extracted from filename
#[derive(Debug, Clone)]
pub struct SeriesInfo {
    pub is_series: bool,
    pub series_number: Option<u32>,
    pub series_type: Option<String>,
    pub total_parts: Option<u32>,
}

/// Language information extracted from filename
#[derive(Debug, Clone)]
pub struct LanguageInfo {
    pub primary_language: Option<String>,
    pub has_japanese: bool,
    pub has_chinese: bool,
    pub has_korean: bool,
    pub is_bilingual: bool,
}

/// Parser configuration options
#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub enable_tmdb_integration: bool,
    pub enable_anime_detection: bool,
    pub enable_series_detection: bool,
    pub confidence_threshold: f32,
    pub max_title_length: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            enable_tmdb_integration: true,
            enable_anime_detection: true,
            enable_series_detection: true,
            confidence_threshold: 0.3,
            max_title_length: 200,
        }
    }
}

/// Parser result with metadata
#[derive(Debug, Clone)]
pub struct ParserResult<T> {
    pub data: T,
    pub confidence: f32,
    pub parsing_method: String,
    pub warnings: Vec<String>,
}

impl<T> ParserResult<T> {
    pub fn new(data: T, confidence: f32, parsing_method: String) -> Self {
        Self {
            data,
            confidence,
            parsing_method,
            warnings: Vec::new(),
        }
    }

    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn is_high_confidence(&self, threshold: f32) -> bool {
        self.confidence >= threshold
    }
}
