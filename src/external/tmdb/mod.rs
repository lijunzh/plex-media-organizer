//! TMDB API integration

pub mod client;
pub mod search;
pub mod unified;

// Re-export existing TMDB client for now
pub use crate::tmdb_client::TmdbClient;

// Re-export new focused modules
pub use client::TmdbApiClient;
pub use search::TmdbSearchEngine;
pub use unified::UnifiedTmdbClient;

// TODO: Gradually migrate to focused modules:
// - client.rs: API client and authentication
// - search.rs: Search algorithms and result processing
