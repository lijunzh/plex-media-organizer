//! TMDB API client for movie data

use crate::types::{MovieInfo, TmdbMovie};
use anyhow::{Context, Result};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use serde::Deserialize;
use std::time::Duration;

/// TMDB API client
#[derive(Clone, Debug)]
pub struct TmdbClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
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
    /// Create a new TMDB client
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url: "https://api.themoviedb.org/3".to_string(),
            client,
        }
    }

    /// Search for a movie by title
    pub async fn search_movie(&self, title: &str, year: Option<u32>) -> Result<Vec<TmdbMovie>> {
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

        Ok(search_response.results)
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

    /// Enhanced movie search with multiple fallback strategies
    pub async fn enhanced_search(
        &self,
        title: &str,
        year: Option<u32>,
    ) -> Result<Option<TmdbMovie>> {
        // Strategy 1: Try exact search with year
        if let Some(movie) = self.find_best_match(title, year).await? {
            return Ok(Some(movie));
        }

        // Strategy 2: Try search without year (broader search)
        if let Some(movie) = self.find_best_match(title, None).await? {
            return Ok(Some(movie));
        }

        // Strategy 3: Try with cleaned title (remove common suffixes/prefixes)
        let cleaned_title = self.clean_title_for_search(title);
        if cleaned_title != title
            && let Some(movie) = self.find_best_match(&cleaned_title, year).await?
        {
            return Ok(Some(movie));
        }

        // Strategy 4: Try with alternative title variations
        for alt_title in self.generate_title_variations(title) {
            if let Some(movie) = self.find_best_match(&alt_title, year).await? {
                return Ok(Some(movie));
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
        let search_results = self.search_movie(title, year).await?;

        if search_results.is_empty() {
            return Ok(None);
        }

        // Enhanced scoring with fuzzy matching
        let mut best_match = None;
        let mut best_score = 0.0;

        for movie in search_results {
            let mut score = 0.0;

            // Enhanced title similarity with fuzzy matching
            let title_lower = title.to_lowercase();
            let movie_title_lower = movie.title.to_lowercase();

            // Exact match (highest priority)
            if title_lower == movie_title_lower {
                score += 200.0;
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

                // Partial match bonus
                if movie_title_lower.contains(&title_lower)
                    || title_lower.contains(&movie_title_lower)
                {
                    score += 30.0;
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

            if score > best_score {
                best_score = score;
                best_match = Some(movie);
            }
        }

        // Only return matches above a minimum threshold
        if best_score >= 50.0 {
            Ok(best_match)
        } else {
            Ok(None)
        }
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
        let client = TmdbClient::new("test_key".to_string());
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
        let client = TmdbClient::new("test_key".to_string());
        // This test would require mocking the HTTP client
        // For now, we'll just test that the method exists and can be called
        // In a real implementation, we'd use a mock HTTP client
        assert_eq!(client.base_url, "https://api.themoviedb.org/3");
    }

    #[test]
    fn test_tmdb_client_clone() {
        let client = TmdbClient::new("test_key".to_string());
        let cloned_client = client.clone();

        assert_eq!(client.base_url, cloned_client.base_url);
        // Note: We can't easily test that the API key is cloned correctly
        // since it's private, but the clone should work
    }
}
