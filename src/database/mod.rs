//! Database module for Plex Media Organizer
//!
//! This module provides SQLite database functionality for:
//! - Storing parsed movie information
//! - Caching parsing results
//! - Caching TMDB API results
//! - Tracking organization decisions

pub mod cache;
pub mod connection;
pub mod movies;
pub mod operations;
pub mod schema;

use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Database manager that handles all database operations
#[derive(Debug, Clone)]
pub struct DatabaseManager {
    connection: Arc<Mutex<Connection>>,
}

impl DatabaseManager {
    /// Create a new database manager with the given database path
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let connection = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;

        // Initialize the database schema
        schema::init_schema(&connection)?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    /// Get a reference to the database connection
    pub async fn connection(&self) -> tokio::sync::MutexGuard<'_, Connection> {
        self.connection.lock().await
    }

    /// Initialize the database with all required tables
    pub async fn initialize(&self) -> Result<()> {
        let conn = self.connection().await;
        schema::init_schema(&conn)?;
        Ok(())
    }

    /// Check if the database is properly initialized
    pub async fn is_initialized(&self) -> Result<bool> {
        let conn = self.connection().await;
        schema::check_schema(&conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_database_manager_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_manager = DatabaseManager::new(temp_file.path()).await.unwrap();
        assert!(db_manager.is_initialized().await.unwrap());
    }

    #[tokio::test]
    async fn test_database_initialization() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_manager = DatabaseManager::new(temp_file.path()).await.unwrap();
        db_manager.initialize().await.unwrap();
        assert!(db_manager.is_initialized().await.unwrap());
    }
}
