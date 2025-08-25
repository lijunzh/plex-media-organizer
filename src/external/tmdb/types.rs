//! TMDB-specific types and data structures

use serde::{Deserialize, Serialize};
// use std::collections::HashMap; // Unused import

/// TMDB movie information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbMovie {
    pub id: i32,
    pub title: String,
    pub original_title: Option<String>,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub popularity: f64,
    pub vote_average: f64,
    pub vote_count: i32,
    pub adult: bool,
    pub video: bool,
    pub genre_ids: Vec<i32>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub original_language: Option<String>,
}

/// TMDB search result with confidence score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbMatchResult {
    pub movie: TmdbMovie,
    pub confidence_score: f32,
    pub match_type: TmdbMatchType,
    pub search_strategy: String,
}

/// Type of TMDB match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TmdbMatchType {
    Exact,
    Fuzzy,
    YearAdjusted,
    TitleVariation,
}

/// TMDB search parameters
#[derive(Debug, Clone)]
pub struct TmdbSearchParams {
    pub title: String,
    pub year: Option<u32>,
    pub min_similarity: f32,
    pub title_weight: f32,
    pub year_weight: f32,
    pub popularity_weight: f32,
}

/// TMDB API configuration
#[derive(Debug, Clone)]
pub struct TmdbConfig {
    pub api_key: String,
    pub base_url: String,
    pub cache_ttl_seconds: u64,
    pub rate_limit_requests_per_second: u32,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

impl Default for TmdbConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.themoviedb.org/3".to_string(),
            cache_ttl_seconds: 3600, // 1 hour
            rate_limit_requests_per_second: 10,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}

/// TMDB cache entry
#[derive(Debug, Clone)]
pub struct TmdbCacheEntry {
    pub results: Vec<TmdbMovie>,
    pub expires_at: std::time::Instant,
}

/// TMDB search statistics
#[derive(Debug, Clone, Default)]
pub struct TmdbSearchStats {
    pub total_searches: u32,
    pub cache_hits: u32,
    pub api_calls: u32,
    pub average_response_time_ms: f64,
    pub error_count: u32,
}

/// TMDB API error types
#[derive(Debug, thiserror::Error)]
pub enum TmdbError {
    #[error("API request failed: {0}")]
    ApiError(String),
    #[error("Authentication failed: {0}")]
    AuthError(String),
    #[error("Rate limit exceeded")]
    RateLimitError,
    #[error("Invalid response format: {0}")]
    ParseError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Cache error: {0}")]
    CacheError(String),
}