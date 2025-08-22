//! TMDB API client functionality

use crate::types::TmdbMovie;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::time::Duration;

/// TMDB API client for making HTTP requests
#[derive(Clone, Debug)]
pub struct TmdbApiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl TmdbApiClient {
    /// Create a new TMDB API client
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            api_key,
            base_url: "https://api.themoviedb.org/3".to_string(),
        })
    }

    /// Make a GET request to TMDB API
    pub async fn get(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}{}?api_key={}", self.base_url, endpoint, self.api_key);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to make TMDB API request")?;

        if !response.status().is_success() {
            anyhow::bail!("TMDB API request failed with status: {}", response.status());
        }

        let json: Value = response
            .json()
            .await
            .context("Failed to parse TMDB API response")?;

        Ok(json)
    }

    /// Search for movies by title
    pub async fn search_movies(&self, query: &str, year: Option<u32>) -> Result<Vec<TmdbMovie>> {
        let mut url = format!(
            "{}/search/movie?api_key={}&query={}&language=en-US&page=1&include_adult=false",
            self.base_url, self.api_key, query
        );

        if let Some(year) = year {
            url.push_str(&format!("&year={}", year));
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to search TMDB movies")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "TMDB search request failed with status: {}",
                response.status()
            );
        }

        #[derive(Debug, Deserialize)]
        struct TmdbSearchResponse {
            results: Vec<TmdbMovie>,
        }

        let search_result: TmdbSearchResponse = response
            .json()
            .await
            .context("Failed to parse TMDB search response")?;

        Ok(search_result.results)
    }

    /// Get movie details by ID
    pub async fn get_movie(&self, movie_id: u32) -> Result<TmdbMovie> {
        let url = format!(
            "{}/movie/{}?api_key={}&language=en-US&append_to_response=credits,videos,images",
            self.base_url, movie_id, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to get TMDB movie details")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "TMDB movie request failed with status: {}",
                response.status()
            );
        }

        let movie: TmdbMovie = response
            .json()
            .await
            .context("Failed to parse TMDB movie response")?;

        Ok(movie)
    }

    /// Get movie collection details
    pub async fn get_collection(&self, collection_id: u32) -> Result<Value> {
        let url = format!(
            "{}/collection/{}?api_key={}&language=en-US",
            self.base_url, collection_id, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to get TMDB collection details")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "TMDB collection request failed with status: {}",
                response.status()
            );
        }

        let collection: Value = response
            .json()
            .await
            .context("Failed to parse TMDB collection response")?;

        Ok(collection)
    }
}
