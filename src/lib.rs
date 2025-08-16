//! Plex Media Organizer - Intelligent media file organization
//! 
//! This library provides intelligent parsing and organization of media files
//! following Plex naming conventions.

pub mod types;
pub mod config;
pub mod movie_parser;
pub mod tmdb_client;
pub mod cli;
pub mod scanner;

pub use types::*;
pub use config::*;
pub use movie_parser::*;
pub use tmdb_client::*;
pub use cli::*;
pub use scanner::*;

/// Main result type for the library
pub type Result<T> = anyhow::Result<T>;

/// Main error type for the library
pub type Error = anyhow::Error;
