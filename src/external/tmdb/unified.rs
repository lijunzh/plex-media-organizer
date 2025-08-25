//! Unified TMDB client that combines API client and search functionality

use crate::types::{TmdbMatchResult, TmdbMovie};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::client::TmdbApiClient;
use super::search::TmdbSearchEngine;

/// Cache entry for TMDB search results
#[derive(Debug, Clone)]
struct CacheEntry {
    results: Vec<TmdbMovie>,
    expires_at: Instant,
}

/// Unified TMDB client that combines API client and search functionality
#[derive(Debug)]
pub struct UnifiedTmdbClient {
    api_client: TmdbApiClient,
    search_engine: TmdbSearchEngine,
    cache: Mutex<HashMap<String, CacheEntry>>,
    cache_ttl: Duration,
}

impl UnifiedTmdbClient {
    /// Create a new unified TMDB client
    pub fn new(api_key: String) -> Result<Self> {
        let api_client = TmdbApiClient::new(api_key.clone())?;
        let search_engine = TmdbSearchEngine::default();

        Ok(Self {
            api_client,
            search_engine,
            cache: Mutex::new(HashMap::new()),
            cache_ttl: Duration::from_secs(3600), // 1 hour default
        })
    }

    /// Create a new unified TMDB client with custom cache TTL
    pub fn with_cache_ttl(api_key: String, cache_ttl: Duration) -> Result<Self> {
        let api_client = TmdbApiClient::new(api_key.clone())?;
        let search_engine = TmdbSearchEngine::default();

        Ok(Self {
            api_client,
            search_engine,
            cache: Mutex::new(HashMap::new()),
            cache_ttl,
        })
    }

    /// Create a new unified TMDB client with custom search parameters
    pub fn with_search_params(
        api_key: String,
        min_similarity: f32,
        title_weight: f32,
        year_weight: f32,
        popularity_weight: f32,
    ) -> Result<Self> {
        let api_client = TmdbApiClient::new(api_key.clone())?;
        let search_engine =
            TmdbSearchEngine::new(min_similarity, title_weight, year_weight, popularity_weight);

        Ok(Self {
            api_client,
            search_engine,
            cache: Mutex::new(HashMap::new()),
            cache_ttl: Duration::from_secs(3600), // 1 hour default
        })
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, Duration) {
        let cache = self.cache.lock().unwrap();
        (cache.len(), self.cache_ttl)
    }

    /// Clear the cache
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get a cached result if available and not expired
    fn get_cached_result(&self, cache_key: &str) -> Option<Vec<TmdbMovie>> {
        let mut cache = self.cache.lock().unwrap();

        if let Some(entry) = cache.get(cache_key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.results.clone());
            } else {
                // Remove expired entry
                cache.remove(cache_key);
            }
        }

        None
    }

    /// Store a result in the cache
    fn store_cached_result(&self, cache_key: String, results: Vec<TmdbMovie>) {
        let mut cache = self.cache.lock().unwrap();
        let entry = CacheEntry {
            results,
            expires_at: Instant::now() + self.cache_ttl,
        };
        cache.insert(cache_key, entry);
    }

    /// Search for movies by title with caching and enhanced matching
    pub async fn search_movie(&self, title: &str, year: Option<u32>) -> Result<Vec<TmdbMovie>> {
        // Create cache key
        let cache_key = format!("{}:{}", title, year.unwrap_or(0));

        // Check cache first
        if let Some(cached_results) = self.get_cached_result(&cache_key) {
            return Ok(cached_results);
        }

        // Use API client to search
        let results = self.api_client.search_movies(title, year).await?;

        // Store results in cache
        self.store_cached_result(cache_key, results.clone());

        Ok(results)
    }

    /// Find the best match for a movie title and year
    pub async fn find_best_match(
        &self,
        title: &str,
        year: Option<u32>,
    ) -> Result<Option<TmdbMatchResult>> {
        // Search for candidates
        let candidates = self.search_movie(title, year).await?;

        // Use search engine to find best match
        let best_match = self.search_engine.find_best_match(title, year, &candidates);

        Ok(best_match)
    }

    /// Get movie details by ID
    pub async fn get_movie(&self, movie_id: u32) -> Result<TmdbMovie> {
        self.api_client.get_movie(movie_id).await
    }

    /// Get collection details by ID
    pub async fn get_collection(&self, collection_id: u32) -> Result<serde_json::Value> {
        self.api_client.get_collection(collection_id).await
    }

    /// Make a generic API request
    pub async fn api_request(&self, endpoint: &str) -> Result<serde_json::Value> {
        self.api_client.get(endpoint).await
    }

    /// Enhanced search with multiple fallback strategies and config parameters
    pub async fn enhanced_search_with_config(
        &self,
        title: &str,
        year: Option<u32>,
        _problematic_patterns: &[String],
    ) -> Result<Option<crate::types::TmdbMatchResult>> {
        // Validate input - reject empty or whitespace-only titles
        if title.trim().is_empty() {
            return Ok(None);
        }

        // Strategy 1: Try exact search with year
        if let Some(result) = self.find_best_match(title, year).await? {
            return Ok(Some(result));
        }

        // Strategy 2: Try search without year (broader search)
        if let Some(result) = self.find_best_match(title, None).await? {
            return Ok(Some(result));
        }

        // Strategy 3: Try with cleaned title (remove common suffixes/prefixes)
        let cleaned_title = self.clean_title_for_search(title);
        if cleaned_title != title {
            if let Some(result) = self.find_best_match(&cleaned_title, year).await? {
                return Ok(Some(result));
            }
        }

        // Strategy 4: Try with alternative title variations
        for alt_title in self.generate_title_variations(title) {
            if let Some(result) = self.find_best_match(&alt_title, year).await? {
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    /// Enhanced search with multiple fallback strategies (legacy method)
    pub async fn enhanced_search(
        &self,
        title: &str,
        year: Option<u32>,
    ) -> Result<Option<crate::types::TmdbMatchResult>> {
        self.enhanced_search_with_config(title, year, &[]).await
    }

    /// Clean title for better search matching
    fn clean_title_for_search(&self, title: &str) -> String {
        let mut cleaned = title.to_string();

        // Remove common file suffixes
        let suffixes_to_remove = [
            " (director's cut)",
            " (extended)",
            " (uncut)",
            " (unrated)",
            " (rated)",
            " (special edition)",
            " (collector's edition)",
            " (deluxe edition)",
            " (limited edition)",
            " (anniversary edition)",
        ];

        for suffix in &suffixes_to_remove {
            if cleaned.to_lowercase().ends_with(&suffix.to_lowercase()) {
                cleaned = cleaned[..cleaned.len() - suffix.len()].trim().to_string();
                break;
            }
        }

        // Remove common prefixes
        let prefixes_to_remove = ["the ", "a ", "an "];
        for prefix in &prefixes_to_remove {
            if cleaned.to_lowercase().starts_with(prefix) {
                cleaned = cleaned[prefix.len()..].trim().to_string();
                break;
            }
        }

        cleaned
    }

    /// Generate title variations for better matching
    fn generate_title_variations(&self, title: &str) -> Vec<String> {
        let mut variations = Vec::new();
        let title_lower = title.to_lowercase();

        // Add "The" prefix if not present
        if !title_lower.starts_with("the ") {
            variations.push(format!("The {}", title));
        }

        // Remove "The" prefix if present
        if title_lower.starts_with("the ") {
            variations.push(title[4..].trim().to_string());
        }

        // Handle sequel numbers (e.g., "Matrix 2" -> "Matrix")
        if let Some(last_space) = title.rfind(' ') {
            if let Ok(_number) = title[last_space + 1..].parse::<u32>() {
                variations.push(title[..last_space].trim().to_string());
            }
        }

        variations
    }

    /// Convert TMDB movie to our MovieInfo format
    pub fn tmdb_to_movie_info(
        &self,
        tmdb_movie: &crate::types::TmdbMovie,
    ) -> crate::types::MovieInfo {
        let year = tmdb_movie.release_date.as_ref().and_then(|date| {
            if date.len() >= 4 {
                date[..4].parse::<u32>().ok()
            } else {
                None
            }
        });

        // Check if movie belongs to a collection
        let is_collection = tmdb_movie.belongs_to_collection.is_some();
        let collection_name = tmdb_movie
            .belongs_to_collection
            .as_ref()
            .map(|c| c.name.clone());

        crate::types::MovieInfo {
            title: tmdb_movie.title.clone(),
            original_title: tmdb_movie.original_title.clone(),
            original_language: tmdb_movie.original_language.clone(),
            year,
            part_number: None,
            is_collection,
            collection_name,
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
        }
    }
}

impl Clone for UnifiedTmdbClient {
    fn clone(&self) -> Self {
        Self {
            api_client: self.api_client.clone(),
            search_engine: self.search_engine.clone(),
            cache: Mutex::new(HashMap::new()), // Start with empty cache
            cache_ttl: self.cache_ttl,
        }
    }
}
