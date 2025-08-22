//! TMDB API integration

// Re-export existing TMDB client for now
pub use crate::tmdb_client::TmdbClient;

// TODO: Gradually migrate to focused modules:
// - client.rs: API client and authentication
// - search.rs: Search algorithms and result processing
