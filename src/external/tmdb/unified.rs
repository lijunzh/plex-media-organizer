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

    /// Enhanced search with multiple strategies
    pub async fn enhanced_search(
        &self,
        title: &str,
        year: Option<u32>,
    ) -> Result<Option<TmdbMatchResult>> {
        // Generate title variations for better matching
        let title_variations = self.search_engine.generate_title_variations(title);

        let mut best_match: Option<TmdbMatchResult> = None;
        let mut best_score = 0.0;

        // Try each title variation
        for variation in title_variations {
            if let Some(match_result) = self.find_best_match(&variation, year).await?
                && match_result.confidence_score > best_score
            {
                best_score = match_result.confidence_score;
                best_match = Some(match_result);
            }
        }

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
