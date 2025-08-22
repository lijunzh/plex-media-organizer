//! Database caching operations for parsing and TMDB results

use anyhow::Result;
use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};

use std::time::{SystemTime, UNIX_EPOCH};

/// Cached parsing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingCacheEntry {
    pub id: Option<i64>,
    pub filename_hash: String,
    pub parsed_data: String, // JSON string
    pub created_at: Option<String>,
}

/// Cached TMDB search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbCacheEntry {
    pub id: Option<i64>,
    pub search_key: String,   // "title:year" format
    pub tmdb_results: String, // JSON string
    pub created_at: Option<String>,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub parsing_cache_size: i64,
    pub tmdb_cache_size: i64,
    pub parsing_cache_hits: i64,
    pub tmdb_cache_hits: i64,
    pub parsing_cache_misses: i64,
    pub tmdb_cache_misses: i64,
}

impl CacheStats {
    /// Calculate cache hit rates
    pub fn parsing_hit_rate(&self) -> f64 {
        let total = self.parsing_cache_hits + self.parsing_cache_misses;
        if total == 0 {
            0.0
        } else {
            self.parsing_cache_hits as f64 / total as f64
        }
    }

    pub fn tmdb_hit_rate(&self) -> f64 {
        let total = self.tmdb_cache_hits + self.tmdb_cache_misses;
        if total == 0 {
            0.0
        } else {
            self.tmdb_cache_hits as f64 / total as f64
        }
    }
}

/// Cache repository for database operations
pub struct CacheRepository<'a> {
    conn: &'a Connection,
}

impl<'a> CacheRepository<'a> {
    /// Create a new cache repository
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Store parsing result in cache
    pub fn store_parsing_result(&self, filename_hash: &str, parsed_data: &str) -> Result<i64> {
        let sql = r#"
            INSERT OR REPLACE INTO parsing_cache (filename_hash, parsed_data)
            VALUES (?, ?)
        "#;

        let id = self.conn.execute(sql, [filename_hash, parsed_data])?;
        Ok(id as i64)
    }

    /// Retrieve parsing result from cache
    pub fn get_parsing_result(&self, filename_hash: &str) -> Result<Option<ParsingCacheEntry>> {
        let sql = "SELECT * FROM parsing_cache WHERE filename_hash = ?";

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([filename_hash])?;

        if let Some(row) = rows.next()? {
            Ok(Some(ParsingCacheEntry::from_row(row)?))
        } else {
            Ok(None)
        }
    }

    /// Store TMDB search result in cache
    pub fn store_tmdb_result(&self, search_key: &str, tmdb_results: &str) -> Result<i64> {
        let sql = r#"
            INSERT OR REPLACE INTO tmdb_cache (search_key, tmdb_results)
            VALUES (?, ?)
        "#;

        let id = self.conn.execute(sql, [search_key, tmdb_results])?;
        Ok(id as i64)
    }

    /// Retrieve TMDB search result from cache
    pub fn get_tmdb_result(&self, search_key: &str) -> Result<Option<TmdbCacheEntry>> {
        let sql = "SELECT * FROM tmdb_cache WHERE search_key = ?";

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([search_key])?;

        if let Some(row) = rows.next()? {
            Ok(Some(TmdbCacheEntry::from_row(row)?))
        } else {
            Ok(None)
        }
    }

    /// Clear expired cache entries
    pub fn clear_expired_entries(&self, max_age_hours: i64) -> Result<usize> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let cutoff_time = current_time - (max_age_hours * 3600);

        // Clear expired parsing cache entries
        let parsing_sql = "DELETE FROM parsing_cache WHERE created_at < datetime(?, 'unixepoch')";
        let parsing_deleted = self.conn.execute(parsing_sql, [cutoff_time])?;

        // Clear expired TMDB cache entries
        let tmdb_sql = "DELETE FROM tmdb_cache WHERE created_at < datetime(?, 'unixepoch')";
        let tmdb_deleted = self.conn.execute(tmdb_sql, [cutoff_time])?;

        Ok(parsing_deleted + tmdb_deleted)
    }

    /// Clear all cache entries
    pub fn clear_all_cache(&self) -> Result<usize> {
        let parsing_deleted = self.conn.execute("DELETE FROM parsing_cache", [])?;
        let tmdb_deleted = self.conn.execute("DELETE FROM tmdb_cache", [])?;
        Ok(parsing_deleted + tmdb_deleted)
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> Result<CacheStats> {
        // Get cache sizes
        let parsing_cache_size: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM parsing_cache", [], |row| row.get(0))?;

        let tmdb_cache_size: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM tmdb_cache", [], |row| row.get(0))?;

        // For now, we'll return placeholder values for hit/miss counts
        // In a real implementation, these would be tracked separately
        Ok(CacheStats {
            parsing_cache_size,
            tmdb_cache_size,
            parsing_cache_hits: 0,   // TODO: Implement hit tracking
            tmdb_cache_hits: 0,      // TODO: Implement hit tracking
            parsing_cache_misses: 0, // TODO: Implement miss tracking
            tmdb_cache_misses: 0,    // TODO: Implement miss tracking
        })
    }

    /// Get cache entry by ID
    pub fn get_parsing_cache_by_id(&self, id: i64) -> Result<Option<ParsingCacheEntry>> {
        let sql = "SELECT * FROM parsing_cache WHERE id = ?";

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(ParsingCacheEntry::from_row(row)?))
        } else {
            Ok(None)
        }
    }

    /// Get TMDB cache entry by ID
    pub fn get_tmdb_cache_by_id(&self, id: i64) -> Result<Option<TmdbCacheEntry>> {
        let sql = "SELECT * FROM tmdb_cache WHERE id = ?";

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(TmdbCacheEntry::from_row(row)?))
        } else {
            Ok(None)
        }
    }
}

impl ParsingCacheEntry {
    /// Create a new parsing cache entry
    pub fn new(filename_hash: String, parsed_data: String) -> Self {
        Self {
            id: None,
            filename_hash,
            parsed_data,
            created_at: None,
        }
    }

    /// Create from database row
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            filename_hash: row.get(1)?,
            parsed_data: row.get(2)?,
            created_at: row.get(3)?,
        })
    }
}

impl TmdbCacheEntry {
    /// Create a new TMDB cache entry
    pub fn new(search_key: String, tmdb_results: String) -> Self {
        Self {
            id: None,
            search_key,
            tmdb_results,
            created_at: None,
        }
    }

    /// Create from database row
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            search_key: row.get(1)?,
            tmdb_results: row.get(2)?,
            created_at: row.get(3)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema;
    use rusqlite::Connection;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, Connection) {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();
        schema::init_schema(&conn).unwrap();
        (temp_file, conn)
    }

    #[test]
    fn test_parsing_cache_operations() {
        let (_temp_file, conn) = create_test_db();
        let cache_repo = CacheRepository::new(&conn);

        let filename_hash = "abc123";
        let parsed_data = r#"{"title": "Test Movie", "year": 2020}"#;

        // Store cache entry
        let id = cache_repo
            .store_parsing_result(filename_hash, parsed_data)
            .unwrap();
        assert!(id > 0);

        // Retrieve cache entry
        let entry = cache_repo
            .get_parsing_result(filename_hash)
            .unwrap()
            .unwrap();
        assert_eq!(entry.filename_hash, filename_hash);
        assert_eq!(entry.parsed_data, parsed_data);

        // Test cache miss
        let miss = cache_repo.get_parsing_result("nonexistent").unwrap();
        assert!(miss.is_none());
    }

    #[test]
    fn test_tmdb_cache_operations() {
        let (_temp_file, conn) = create_test_db();
        let cache_repo = CacheRepository::new(&conn);

        let search_key = "Test Movie:2020";
        let tmdb_results = r#"[{"id": 123, "title": "Test Movie"}]"#;

        // Store cache entry
        let id = cache_repo
            .store_tmdb_result(search_key, tmdb_results)
            .unwrap();
        assert!(id > 0);

        // Retrieve cache entry
        let entry = cache_repo.get_tmdb_result(search_key).unwrap().unwrap();
        assert_eq!(entry.search_key, search_key);
        assert_eq!(entry.tmdb_results, tmdb_results);

        // Test cache miss
        let miss = cache_repo.get_tmdb_result("nonexistent:2020").unwrap();
        assert!(miss.is_none());
    }

    #[test]
    fn test_cache_stats() {
        let (_temp_file, conn) = create_test_db();
        let cache_repo = CacheRepository::new(&conn);

        // Add some test data
        cache_repo.store_parsing_result("hash1", "data1").unwrap();
        cache_repo.store_parsing_result("hash2", "data2").unwrap();
        cache_repo.store_tmdb_result("key1", "results1").unwrap();

        let stats = cache_repo.get_cache_stats().unwrap();
        assert_eq!(stats.parsing_cache_size, 2);
        assert_eq!(stats.tmdb_cache_size, 1);
    }

    #[test]
    fn test_clear_cache() {
        let (_temp_file, conn) = create_test_db();
        let cache_repo = CacheRepository::new(&conn);

        // Add test data
        cache_repo.store_parsing_result("hash1", "data1").unwrap();
        cache_repo.store_tmdb_result("key1", "results1").unwrap();

        // Clear all cache
        let deleted = cache_repo.clear_all_cache().unwrap();
        assert_eq!(deleted, 2);

        // Verify cache is empty
        let stats = cache_repo.get_cache_stats().unwrap();
        assert_eq!(stats.parsing_cache_size, 0);
        assert_eq!(stats.tmdb_cache_size, 0);
    }
}
