//! TMDB API client for movie data

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// TMDB movie data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbMovie {
    pub id: i32,
    pub title: String,
    pub original_title: Option<String>,
    pub original_language: Option<String>,
    pub release_date: Option<String>,
    pub overview: Option<String>,
}

/// TMDB search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbSearchResult {
    pub page: i32,
    pub results: Vec<TmdbMovie>,
    pub total_pages: i32,
    pub total_results: i32,
}

/// TMDB match with confidence score
#[derive(Debug, Clone)]
pub struct TmdbMatch {
    pub movie: TmdbMovie,
    pub confidence_score: f32,
}

/// TMDB API client
#[derive(Debug, Clone)]
pub struct TmdbClient {
    api_key: String,
    client: reqwest::Client,
}

impl TmdbClient {
    /// Create a new TMDB client
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Search for movies by title and year
    pub async fn search_movies(&self, title: &str, year: Option<u32>) -> Result<Vec<TmdbMovie>> {
        let mut url = format!(
            "https://api.themoviedb.org/3/search/movie?api_key={}&query={}",
            self.api_key,
            urlencoding::encode(title)
        );

        if let Some(y) = year {
            url.push_str(&format!("&year={}", y));
        }

        let response = self.client.get(&url).send().await?;
        let search_result: TmdbSearchResult = response.json().await?;

        Ok(search_result.results)
    }

    /// Find the best match for a movie title
    pub async fn find_best_match(
        &self,
        title: &str,
        year: Option<u32>,
    ) -> Result<Option<TmdbMatch>> {
        let movies = self.search_movies(title, year).await?;

        if movies.is_empty() {
            return Ok(None);
        }

        // Simple matching: take the first result for now
        // In a more sophisticated implementation, you'd use fuzzy matching
        let movie = movies[0].clone();
        let confidence_score = self.calculate_confidence(title, &movie, year);

        Ok(Some(TmdbMatch {
            movie,
            confidence_score,
        }))
    }

    /// Calculate confidence score for a match
    fn calculate_confidence(
        &self,
        search_title: &str,
        movie: &TmdbMovie,
        year: Option<u32>,
    ) -> f32 {
        let mut confidence: f32 = 0.0;

        // Title similarity (simple case-insensitive comparison)
        let search_lower = search_title.to_lowercase();
        let movie_lower = movie.title.to_lowercase();

        if search_lower == movie_lower {
            confidence += 0.8;
        } else if movie_lower.contains(&search_lower) || search_lower.contains(&movie_lower) {
            confidence += 0.6;
        } else {
            confidence += 0.3; // Basic match
        }

        // Year match
        if let Some(search_year) = year
            && let Some(release_date) = &movie.release_date
            && let Some(movie_year) = release_date
                .split('-')
                .next()
                .and_then(|y| y.parse::<u32>().ok())
            && search_year == movie_year
        {
            confidence += 0.2;
        }

        confidence.min(1.0_f32)
    }
}

/// Unified TMDB client that can be None if no API key is provided
#[derive(Debug, Clone)]
pub struct UnifiedTmdbClient {
    client: Option<TmdbClient>,
}

impl UnifiedTmdbClient {
    /// Create a new unified TMDB client
    pub fn new(api_key: String) -> Result<Self> {
        if api_key.is_empty() {
            Ok(Self { client: None })
        } else {
            Ok(Self {
                client: Some(TmdbClient::new(api_key)),
            })
        }
    }

    /// Find the best match for a movie title
    pub async fn find_best_match(
        &self,
        title: &str,
        year: Option<u32>,
    ) -> Result<Option<TmdbMatch>> {
        if let Some(client) = &self.client {
            client.find_best_match(title, year).await
        } else {
            Ok(None)
        }
    }
}
