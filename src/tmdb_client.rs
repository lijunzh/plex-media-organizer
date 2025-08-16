//! TMDB API client for movie data

use anyhow::{Context, Result};
use serde::Deserialize;
use crate::types::{MovieInfo, TmdbMovie};
use std::time::Duration;


/// TMDB API client
pub struct TmdbClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
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
        
        let response = self.client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .context("Failed to send TMDB search request")?;
        
        if !response.status().is_success() {
            anyhow::bail!("TMDB API request failed with status: {}", response.status());
        }
        
        let search_response: TmdbSearchResponse = response.json().await
            .context("Failed to parse TMDB search response")?;
        
        Ok(search_response.results)
    }
    
    /// Get movie details by TMDB ID
    pub async fn get_movie(&self, tmdb_id: u32) -> Result<TmdbMovie> {
        let url = format!("{}/movie/{}", self.base_url, tmdb_id);
        let query_params = vec![
            ("api_key", self.api_key.as_str()),
            ("language", "en-US"),
        ];
        
        let response = self.client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .context("Failed to send TMDB movie request")?;
        
        if !response.status().is_success() {
            anyhow::bail!("TMDB API request failed with status: {}", response.status());
        }
        
        let movie_response: TmdbMovieResponse = response.json().await
            .context("Failed to parse TMDB movie response")?;
        
        let movie = TmdbMovie {
            id: movie_response.id,
            title: movie_response.title,
            original_title: movie_response.original_title,
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
    
    /// Find the best matching movie from search results
    pub async fn find_best_match(&self, title: &str, year: Option<u32>) -> Result<Option<TmdbMovie>> {
        let search_results = self.search_movie(title, year).await?;
        
        if search_results.is_empty() {
            return Ok(None);
        }
        
        // Simple scoring: prefer exact title matches and year matches
        let mut best_match = None;
        let mut best_score = 0.0;
        
        for movie in search_results {
            let mut score = 0.0;
            
            // Title similarity (simple string matching for now)
            let title_lower = title.to_lowercase();
            let movie_title_lower = movie.title.to_lowercase();
            
            if title_lower == movie_title_lower {
                score += 100.0; // Exact match
            } else if movie_title_lower.contains(&title_lower) || title_lower.contains(&movie_title_lower) {
                score += 50.0; // Partial match
            }
            
            // Year bonus
            if let Some(search_year) = year {
                if let Some(release_date) = &movie.release_date {
                    if let Ok(movie_year) = release_date[..4].parse::<u32>() {
                        if movie_year == search_year {
                            score += 25.0;
                        }
                    }
                }
            }
            
            // Popularity bonus
            if let Some(popularity) = movie.popularity {
                score += popularity.min(50.0); // Cap at 50
            }
            
            if score > best_score {
                best_score = score;
                best_match = Some(movie);
            }
        }
        
        Ok(best_match)
    }
    
    /// Convert TMDB movie to our MovieInfo format
    pub fn tmdb_to_movie_info(&self, tmdb_movie: &TmdbMovie) -> MovieInfo {
        let year = tmdb_movie.release_date
            .as_ref()
            .and_then(|date| {
                if date.len() >= 4 {
                    date[..4].parse::<u32>().ok()
                } else {
                    None
                }
            });
        
        MovieInfo {
            title: tmdb_movie.title.clone(),
            original_title: tmdb_movie.original_title.clone(),
            year,
            part_number: None, // TMDB doesn't provide this
            is_collection: false, // Would need additional API call
            collection_name: None,
            quality: None, // Not provided by TMDB
            source: None, // Not provided by TMDB
            language: Some("en-US".to_string()), // Default assumption
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
}
