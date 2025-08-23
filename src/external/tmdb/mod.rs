//! TMDB API integration

pub mod client;
pub mod search;
pub mod unified;

// Re-export focused modules
pub use client::TmdbApiClient;
pub use search::TmdbSearchEngine;
pub use unified::UnifiedTmdbClient;

// TODO: Gradually migrate to focused modules:
// - client.rs: API client and authentication
// - search.rs: Search algorithms and result processing
