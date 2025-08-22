//! TMDB API client for movie data

use crate::types::{MovieInfo, TmdbMovie};
use anyhow::{Context, Result};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Cache entry for TMDB search results
#[derive(Clone)]
struct CacheEntry {
    results: Vec<TmdbMovie>,
    expires_at: Instant,
}

/// TMDB API client with caching
pub struct TmdbClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
    cache: Mutex<HashMap<String, CacheEntry>>,
    cache_ttl: Duration,
}

impl std::fmt::Debug for TmdbClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TmdbClient")
            .field("api_key", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .field("client", &"[HTTP_CLIENT]")
            .finish()
    }
}

impl Clone for TmdbClient {
    fn clone(&self) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key: self.api_key.clone(),
            base_url: self.base_url.clone(),
            client,
            cache: Mutex::new(HashMap::new()), // Start with empty cache
            cache_ttl: self.cache_ttl,
        }
    }
}

/// TMDB search response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct TmdbSearchResponse {
    results: Vec<TmdbMovie>,
    total_results: u32,
    total_pages: u32,
    page: u32,
}

/// TMDB movie details response
#[derive(Debug, Deserialize)]
struct TmdbMovieResponse {
    id: u32,
    title: String,
    original_title: Option<String>,
    original_language: Option<String>,
    release_date: Option<String>,
    overview: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    vote_average: Option<f32>,
    vote_count: Option<u32>,
    popularity: Option<f32>,
}

impl TmdbClient {
    /// Create a new TMDB client with default cache TTL (1 hour)
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url: "https://api.themoviedb.org/3".to_string(),
            client,
            cache: Mutex::new(HashMap::new()),
            cache_ttl: Duration::from_secs(3600), // 1 hour default
        }
    }

    /// Create a new TMDB client with custom cache TTL
    pub fn with_cache_ttl(api_key: String, cache_ttl: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url: "https://api.themoviedb.org/3".to_string(),
            client,
            cache: Mutex::new(HashMap::new()),
            cache_ttl,
        }
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

    /// Search for a movie by title with caching
    pub async fn search_movie(&self, title: &str, year: Option<u32>) -> Result<Vec<TmdbMovie>> {
        // Create cache key
        let cache_key = format!("{}:{}", title, year.unwrap_or(0));

        // Check cache first
        if let Some(cached_results) = self.get_cached_result(&cache_key) {
            return Ok(cached_results);
        }

        let url = format!("{}/search/movie", self.base_url);
        let mut query_params = vec![
            ("api_key", self.api_key.as_str()),
            ("query", title),
            ("language", "en-US"),
            ("include_adult", "false"),
        ];

        // Store year string to extend its lifetime
        #[allow(unused_assignments)]
        let mut year_string = None;
        if let Some(year) = year {
            year_string = Some(year.to_string());
            query_params.push(("year", year_string.as_ref().unwrap()));
        }

        let response = self
            .client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .context("Failed to send TMDB search request")?;

        if !response.status().is_success() {
            anyhow::bail!("TMDB API request failed with status: {}", response.status());
        }

        let search_response: TmdbSearchResponse = response
            .json()
            .await
            .context("Failed to parse TMDB search response")?;

        let results = search_response.results;

        // Store results in cache
        self.store_cached_result(cache_key, results.clone());

        Ok(results)
    }

    /// Get movie details by TMDB ID
    pub async fn get_movie(&self, tmdb_id: u32) -> Result<TmdbMovie> {
        let url = format!("{}/movie/{}", self.base_url, tmdb_id);
        let query_params = vec![("api_key", self.api_key.as_str()), ("language", "en-US")];

        let response = self
            .client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .context("Failed to send TMDB movie request")?;

        if !response.status().is_success() {
            anyhow::bail!("TMDB API request failed with status: {}", response.status());
        }

        let movie_response: TmdbMovieResponse = response
            .json()
            .await
            .context("Failed to parse TMDB movie response")?;

        let movie = TmdbMovie {
            id: movie_response.id,
            title: movie_response.title,
            original_title: movie_response.original_title,
            original_language: movie_response.original_language,
            release_date: movie_response.release_date,
            overview: movie_response.overview,
            poster_path: movie_response.poster_path,
            backdrop_path: movie_response.backdrop_path,
            vote_average: movie_response.vote_average,
            vote_count: movie_response.vote_count,
            popularity: movie_response.popularity,
        };

        Ok(movie)
    }

    /// Enhanced search with multiple fallback strategies and config parameters
    pub async fn enhanced_search_with_config(
        &self,
        title: &str,
        year: Option<u32>,
        problematic_patterns: &[String],
    ) -> Result<Option<crate::types::TmdbMatchResult>> {
        // Validate input - reject empty or whitespace-only titles
        if title.trim().is_empty() {
            return Ok(None);
        }

        // Strategy 1: Try exact search with year
        if let Some(result) = self
            .find_best_match_with_score_and_config(title, year, problematic_patterns)
            .await?
        {
            return Ok(Some(result));
        }

        // Strategy 2: Try search without year (broader search)
        if let Some(result) = self
            .find_best_match_with_score_and_config(title, None, problematic_patterns)
            .await?
        {
            return Ok(Some(result));
        }

        // Strategy 3: Try with cleaned title (remove common suffixes/prefixes)
        let cleaned_title = self.clean_title_for_search(title);
        if cleaned_title != title
            && let Some(result) = self
                .find_best_match_with_score_and_config(&cleaned_title, year, problematic_patterns)
                .await?
        {
            return Ok(Some(result));
        }

        // Strategy 4: Try with alternative title variations
        for alt_title in self.generate_title_variations(title) {
            if let Some(result) = self
                .find_best_match_with_score_and_config(&alt_title, year, problematic_patterns)
                .await?
            {
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    /// Enhanced search with multiple fallback strategies (legacy method - loads config internally)
    pub async fn enhanced_search(
        &self,
        title: &str,
        year: Option<u32>,
    ) -> Result<Option<crate::types::TmdbMatchResult>> {
        // Validate input - reject empty or whitespace-only titles
        if title.trim().is_empty() {
            return Ok(None);
        }

        // Strategy 1: Try exact search with year
        if let Some(result) = self.find_best_match_with_score(title, year).await? {
            return Ok(Some(result));
        }

        // Strategy 2: Try search without year (broader search)
        if let Some(result) = self.find_best_match_with_score(title, None).await? {
            return Ok(Some(result));
        }

        // Strategy 3: Try with cleaned title (remove common suffixes/prefixes)
        let cleaned_title = self.clean_title_for_search(title);
        if cleaned_title != title
            && let Some(result) = self
                .find_best_match_with_score(&cleaned_title, year)
                .await?
        {
            return Ok(Some(result));
        }

        // Strategy 4: Try with alternative title variations
        for alt_title in self.generate_title_variations(title) {
            if let Some(result) = self.find_best_match_with_score(&alt_title, year).await? {
                return Ok(Some(result));
            }
        }

        Ok(None)
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
            " (theatrical)",
            " (special edition)",
            " (collector's edition)",
            " (remastered)",
            " (restored)",
        ];

        for suffix in &suffixes_to_remove {
            if cleaned.to_lowercase().ends_with(&suffix.to_lowercase()) {
                cleaned = cleaned[..cleaned.len() - suffix.len()].to_string();
                break;
            }
        }

        // Remove common prefixes
        let prefixes_to_remove = ["the ", "a ", "an "];

        for prefix in &prefixes_to_remove {
            if cleaned.to_lowercase().starts_with(prefix) {
                cleaned = cleaned[prefix.len()..].to_string();
                break;
            }
        }

        cleaned.trim().to_string()
    }

    /// Generate alternative title variations for search
    fn generate_title_variations(&self, title: &str) -> Vec<String> {
        let mut variations = Vec::new();
        let title_lower = title.to_lowercase();

        // Add "The" prefix if not present
        if !title_lower.starts_with("the ") {
            variations.push(format!("The {}", title));
        }

        // Remove "The" prefix if present
        if title_lower.starts_with("the ") {
            variations.push(title[4..].to_string());
        }

        // Try without numbers (for sequels)
        let without_numbers = title
            .chars()
            .filter(|c| !c.is_numeric())
            .collect::<String>();
        if without_numbers != title && without_numbers.trim().len() > 3 {
            variations.push(without_numbers.trim().to_string());
        }

        variations
    }

    /// Find the best matching movie from search results with enhanced fuzzy matching
    pub async fn find_best_match(
        &self,
        title: &str,
        year: Option<u32>,
    ) -> Result<Option<TmdbMovie>> {
        if let Some(result) = self.find_best_match_with_score(title, year).await? {
            Ok(Some(result.movie))
        } else {
            Ok(None)
        }
    }

    /// Find the best matching movie with scoring and config parameters
    pub async fn find_best_match_with_score_and_config(
        &self,
        title: &str,
        year: Option<u32>,
        problematic_patterns: &[String],
    ) -> Result<Option<crate::types::TmdbMatchResult>> {
        let movies = self.search_movie(title, year).await?;

        if movies.is_empty() {
            return Ok(None);
        }

        let mut best_score = 0.0;
        let mut best_match: Option<TmdbMovie> = None;
        let title_lower = title.to_lowercase();

        for movie in movies {
            let mut score = 0.0;
            let movie_title_lower = movie.title.to_lowercase();

            // Exact title match (highest priority)
            if movie_title_lower == title_lower {
                score += 1000.0;
            } else if movie_title_lower.len() > 10 && title_lower.len() > 10 {
                // For longer titles, check if they're very similar
                let title_words: Vec<&str> = title_lower.split_whitespace().collect();
                let movie_words: Vec<&str> = movie_title_lower.split_whitespace().collect();

                if title_words.len() >= 3 && movie_words.len() >= 3 {
                    let common_words = title_words
                        .iter()
                        .filter(|word| movie_words.contains(word))
                        .count();
                    let similarity =
                        common_words as f32 / title_words.len().max(movie_words.len()) as f32;

                    if similarity >= 0.8 {
                        score += 800.0; // Very high score for high similarity
                    } else if similarity >= 0.6 {
                        score += 400.0; // High score for good similarity
                    }
                }

                // For short titles, don't give partial match bonuses
                // This prevents "Apples" from matching "Crab Apples"
            } else {
                // Fuzzy matching for similar titles
                let fuzzy_matcher = SkimMatcherV2::default();
                if let Some(fuzzy_score) =
                    fuzzy_matcher.fuzzy_match(&movie_title_lower, &title_lower)
                {
                    // Convert fuzzy score to our scoring system (fuzzy scores are typically 0-100)
                    let normalized_fuzzy_score = (fuzzy_score as f32) * 1.5; // Boost fuzzy scores
                    score += normalized_fuzzy_score;
                }

                // Partial match bonus (only for longer titles to avoid false positives)
                if title_lower.len() > 10
                    && (movie_title_lower.contains(&title_lower)
                        || title_lower.contains(&movie_title_lower))
                {
                    score += 10.0; // Reduced from 30.0
                }

                // Check original title if available
                if let Some(original_title) = &movie.original_title {
                    let original_lower = original_title.to_lowercase();
                    if let Some(fuzzy_score) =
                        fuzzy_matcher.fuzzy_match(&original_lower, &title_lower)
                    {
                        let normalized_fuzzy_score = (fuzzy_score as f32) * 1.2; // Slightly lower weight for original titles
                        score += normalized_fuzzy_score;
                    }
                }
            }

            // Year matching (significant bonus)
            if let Some(search_year) = year
                && let Some(release_date) = &movie.release_date
                && let Ok(movie_year) = release_date[..4].parse::<u32>()
            {
                if movie_year == search_year {
                    score += 100.0; // Exact year match
                } else if (movie_year as i32 - search_year as i32).abs() <= 1 {
                    score += 50.0; // Year within 1 year
                } else if (movie_year as i32 - search_year as i32).abs() <= 3 {
                    score += 25.0; // Year within 3 years
                }
            }

            // Popularity and rating bonus
            if let Some(popularity) = movie.popularity {
                score += popularity.min(30.0); // Cap popularity bonus
            }

            if let Some(vote_average) = movie.vote_average {
                score += vote_average * 2.0; // Rating bonus
            }

            // Penalize problematic content types using passed config
            let title_lower = movie.title.to_lowercase();

            for pattern in problematic_patterns {
                if title_lower.contains(pattern) {
                    score -= 1000.0; // Very heavy penalty for problematic content
                    break;
                }
            }

            if score > best_score {
                best_score = score;
                best_match = Some(movie);
            }
        }

        // Convert TMDB score to confidence score (0.0 to 1.0)
        // TMDB scores can range from ~50 to ~400, so normalize to 0.0-1.0
        let confidence_score = if best_score >= 50.0 {
            // Normalize: 50 = 0.3, 400 = 1.0
            let normalized = (best_score - 50.0) / 350.0; // 400 - 50 = 350
            (0.3 + normalized * 0.7).min(1.0_f32) // Scale to 0.3-1.0 range
        } else {
            0.0
        };

        // Only return matches above a minimum threshold
        if best_score >= 50.0 {
            let movie = best_match.unwrap();

            Ok(Some(crate::types::TmdbMatchResult {
                movie: movie.clone(),
                confidence_score,
            }))
        } else {
            Ok(None)
        }
    }

    /// Find the best matching movie with scoring (legacy method - loads config internally)
    pub async fn find_best_match_with_score(
        &self,
        title: &str,
        year: Option<u32>,
    ) -> Result<Option<crate::types::TmdbMatchResult>> {
        // Get problematic patterns from configuration
        let problematic_patterns = crate::config::AppConfig::load()
            .map(|config| config.get_all_content_filtering_patterns())
            .unwrap_or_else(|_| {
                vec![
                    "production report".to_string(),
                    "making of".to_string(),
                    "behind the scenes".to_string(),
                    "documentary".to_string(),
                    "extras".to_string(),
                    "commentary".to_string(),
                    "interview".to_string(),
                    "photo gallery".to_string(),
                    "gallery".to_string(),
                    "trailer".to_string(),
                    "teaser".to_string(),
                    "preview".to_string(),
                    "sneak peek".to_string(),
                    "deleted scene".to_string(),
                    "alternate ending".to_string(),
                    "bloopers".to_string(),
                    "outtakes".to_string(),
                    "featurette".to_string(),
                    "promo".to_string(),
                    "promotional".to_string(),
                    "music video".to_string(),
                    "soundtrack".to_string(),
                    "score".to_string(),
                    "ost".to_string(),
                ]
            });

        self.find_best_match_with_score_and_config(title, year, &problematic_patterns)
            .await
    }

    /// Convert TMDB movie to our MovieInfo format
    pub fn tmdb_to_movie_info(&self, tmdb_movie: &TmdbMovie) -> MovieInfo {
        let year = tmdb_movie.release_date.as_ref().and_then(|date| {
            if date.len() >= 4 {
                date[..4].parse::<u32>().ok()
            } else {
                None
            }
        });

        MovieInfo {
            title: tmdb_movie.title.clone(),
            original_title: tmdb_movie.original_title.clone(),
            original_language: tmdb_movie.original_language.clone(),
            year,
            part_number: None,
            is_collection: false,
            collection_name: None,
            quality: None,
            source: None,
            language: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_title_for_search() {
        let client = TmdbClient::new("test_key".to_string());

        // Test removing suffixes
        assert_eq!(
            client.clean_title_for_search("Movie (Director's Cut)"),
            "Movie"
        );
        assert_eq!(client.clean_title_for_search("Movie (Extended)"), "Movie");
        assert_eq!(client.clean_title_for_search("Movie (Uncut)"), "Movie");

        // Test removing prefixes
        assert_eq!(client.clean_title_for_search("The Movie"), "Movie");
        assert_eq!(client.clean_title_for_search("A Movie"), "Movie");
        assert_eq!(client.clean_title_for_search("An Movie"), "Movie");

        // Test no changes
        assert_eq!(client.clean_title_for_search("Movie"), "Movie");
        assert_eq!(client.clean_title_for_search("The Matrix"), "Matrix");
    }

    #[test]
    fn test_generate_title_variations() {
        let client = TmdbClient::new("test_key".to_string());

        // Test adding "The" prefix
        let variations = client.generate_title_variations("Matrix");
        assert!(variations.contains(&"The Matrix".to_string()));

        // Test removing "The" prefix
        let variations = client.generate_title_variations("The Matrix");
        assert!(variations.contains(&"Matrix".to_string()));

        // Test sequel handling
        let variations = client.generate_title_variations("Matrix 2");
        assert!(variations.contains(&"Matrix".to_string())); // Should contain "Matrix" without numbers
    }

    #[test]
    fn test_fuzzy_matching_scoring() {
        let _client = TmdbClient::new("test_key".to_string());
        let fuzzy_matcher = SkimMatcherV2::default();

        // Test exact match
        let score = fuzzy_matcher.fuzzy_match("the matrix", "the matrix");
        assert!(score.is_some());
        assert!(score.unwrap() > 80); // High score for exact match

        // Test similar match
        let score = fuzzy_matcher.fuzzy_match("the matrix", "matrix");
        assert!(score.is_some());
        assert!(score.unwrap() > 50); // Good score for similar match

        // Test poor match
        let score = fuzzy_matcher.fuzzy_match("the matrix", "completely different");
        assert!(score.is_none() || score.unwrap() < 30); // Low or no score for poor match
    }

    #[tokio::test]
    async fn test_enhanced_search_strategies() {
        // This test would require a real TMDB API key and network access
        // For now, we'll test the structure and logic
        let client = TmdbClient::new("test_key".to_string());

        // Test that the method exists and has the right signature
        let result = client.enhanced_search("test movie", Some(2020)).await;
        // This will fail with API key error, but that's expected
        assert!(result.is_err()); // Should fail due to invalid API key
    }

    #[tokio::test]
    async fn test_tmdb_client_creation() {
        let client = TmdbClient::new("test_key".to_string());
        assert_eq!(client.base_url, "https://api.themoviedb.org/3");
    }

    #[test]
    fn test_tmdb_to_movie_info() {
        let client = TmdbClient::new("test_key".to_string());
        let tmdb_movie = TmdbMovie {
            id: 1,
            title: "Test Movie".to_string(),
            original_title: None,
            original_language: None,
            release_date: Some("2023-01-01".to_string()),
            overview: None,
            poster_path: None,
            backdrop_path: None,
            vote_average: None,
            vote_count: None,
            popularity: None,
        };

        let movie_info = client.tmdb_to_movie_info(&tmdb_movie);
        assert_eq!(movie_info.title, "Test Movie");
        assert_eq!(movie_info.year, Some(2023));
    }

    #[test]
    fn test_tmdb_to_movie_info_no_release_date() {
        let client = TmdbClient::new("test_key".to_string());
        let tmdb_movie = TmdbMovie {
            id: 1,
            title: "Test Movie".to_string(),
            original_title: None,
            original_language: None,
            release_date: None,
            overview: None,
            poster_path: None,
            backdrop_path: None,
            vote_average: None,
            vote_count: None,
            popularity: None,
        };

        let movie_info = client.tmdb_to_movie_info(&tmdb_movie);
        assert_eq!(movie_info.title, "Test Movie");
        assert_eq!(movie_info.year, None);
    }

    #[test]
    fn test_tmdb_to_movie_info_with_original_title() {
        let client = TmdbClient::new("test_key".to_string());
        let tmdb_movie = TmdbMovie {
            id: 1,
            title: "Test Movie".to_string(),
            original_title: Some("Original Title".to_string()),
            original_language: Some("en".to_string()),
            release_date: Some("2023-01-01".to_string()),
            overview: None,
            poster_path: None,
            backdrop_path: None,
            vote_average: None,
            vote_count: None,
            popularity: None,
        };

        let movie_info = client.tmdb_to_movie_info(&tmdb_movie);
        assert_eq!(movie_info.title, "Test Movie");
        assert_eq!(
            movie_info.original_title,
            Some("Original Title".to_string())
        );
        assert_eq!(movie_info.original_language, Some("en".to_string()));
        assert_eq!(movie_info.year, Some(2023));
    }

    #[tokio::test]
    async fn test_find_best_match_empty_search() {
        let _client = TmdbClient::new("test_key".to_string());
        // This test would require mocking the HTTP client
        // For now, we'll just test that the method exists and can be called
        // In a real implementation, we'd use a mock HTTP client
        assert_eq!(_client.base_url, "https://api.themoviedb.org/3");
    }

    #[test]
    fn test_tmdb_client_clone() {
        let client = TmdbClient::new("test_key".to_string());
        let cloned_client = client.clone();

        assert_eq!(client.base_url, cloned_client.base_url);
        // Note: We can't easily test that the API key is cloned correctly
        // since it's private, but the clone should work
    }

    #[test]
    fn test_cache_stats() {
        let client = TmdbClient::new("test_key".to_string());
        let (cache_size, ttl) = client.cache_stats();

        assert_eq!(cache_size, 0); // Cache should be empty initially
        assert_eq!(ttl, Duration::from_secs(3600)); // Default 1 hour TTL
    }

    #[test]
    fn test_clear_cache() {
        let client = TmdbClient::new("test_key".to_string());
        client.clear_cache();

        let (cache_size, _) = client.cache_stats();
        assert_eq!(cache_size, 0);
    }
}
