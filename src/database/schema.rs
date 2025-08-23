//! Database schema definitions and initialization

use anyhow::Result;
use rusqlite::Connection;

/// SQL statements for creating database tables
const CREATE_MOVIES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS movies (
    id INTEGER PRIMARY KEY,
    original_filename TEXT NOT NULL,
    parsed_title TEXT NOT NULL,
    year INTEGER,
    quality TEXT,
    source TEXT,
    release_group TEXT,
    language TEXT,
    collection_name TEXT,
    part_number INTEGER,
    tmdb_id INTEGER,
    tmdb_title TEXT,
    tmdb_year INTEGER,
    confidence_score REAL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"#;

const CREATE_PARSING_CACHE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS parsing_cache (
    id INTEGER PRIMARY KEY,
    filename_hash TEXT UNIQUE NOT NULL,
    parsed_data TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"#;

const CREATE_TMDB_CACHE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS tmdb_cache (
    id INTEGER PRIMARY KEY,
    search_key TEXT UNIQUE NOT NULL,
    tmdb_results TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"#;

const CREATE_OPERATIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS operations (
    id INTEGER PRIMARY KEY,
    operation_id TEXT UNIQUE NOT NULL,
    operation_type TEXT NOT NULL,
    source_directory TEXT NOT NULL,
    output_directory TEXT,
    total_files INTEGER NOT NULL,
    successful_files INTEGER NOT NULL,
    skipped_files INTEGER NOT NULL,
    failed_files INTEGER NOT NULL,
    total_size_moved INTEGER,
    average_confidence REAL,
    operation_data TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    status TEXT DEFAULT 'completed'
);
"#;

const CREATE_OPERATION_FILES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS operation_files (
    id INTEGER PRIMARY KEY,
    operation_id TEXT NOT NULL,
    original_path TEXT NOT NULL,
    new_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    operation_type TEXT NOT NULL,
    confidence REAL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (operation_id) REFERENCES operations(operation_id)
);
"#;

const CREATE_INDEXES: &[&str] = &[
    "CREATE INDEX IF NOT EXISTS idx_movies_original_filename ON movies(original_filename);",
    "CREATE INDEX IF NOT EXISTS idx_movies_parsed_title ON movies(parsed_title);",
    "CREATE INDEX IF NOT EXISTS idx_movies_year ON movies(year);",
    "CREATE INDEX IF NOT EXISTS idx_movies_tmdb_id ON movies(tmdb_id);",
    "CREATE INDEX IF NOT EXISTS idx_movies_collection ON movies(collection_name);",
    "CREATE INDEX IF NOT EXISTS idx_parsing_cache_hash ON parsing_cache(filename_hash);",
    "CREATE INDEX IF NOT EXISTS idx_tmdb_cache_key ON tmdb_cache(search_key);",
    "CREATE INDEX IF NOT EXISTS idx_parsing_cache_created ON parsing_cache(created_at);",
    "CREATE INDEX IF NOT EXISTS idx_tmdb_cache_created ON tmdb_cache(created_at);",
    "CREATE INDEX IF NOT EXISTS idx_operations_id ON operations(operation_id);",
    "CREATE INDEX IF NOT EXISTS idx_operations_created ON operations(created_at);",
    "CREATE INDEX IF NOT EXISTS idx_operations_type ON operations(operation_type);",
    "CREATE INDEX IF NOT EXISTS idx_operation_files_operation_id ON operation_files(operation_id);",
    "CREATE INDEX IF NOT EXISTS idx_operation_files_original_path ON operation_files(original_path);",
];

/// Initialize the database schema by creating all required tables
pub fn init_schema(conn: &Connection) -> Result<()> {
    // Create tables
    conn.execute(CREATE_MOVIES_TABLE, [])?;
    conn.execute(CREATE_PARSING_CACHE_TABLE, [])?;
    conn.execute(CREATE_TMDB_CACHE_TABLE, [])?;
    conn.execute(CREATE_OPERATIONS_TABLE, [])?;
    conn.execute(CREATE_OPERATION_FILES_TABLE, [])?;

    // Create indexes
    for index_sql in CREATE_INDEXES {
        conn.execute(index_sql, [])?;
    }

    Ok(())
}

/// Check if the database schema is properly initialized
pub fn check_schema(conn: &Connection) -> Result<bool> {
    // Check if all required tables exist
    let tables = vec![
        "movies",
        "parsing_cache",
        "tmdb_cache",
        "operations",
        "operation_files",
    ];

    for table in tables {
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?",
            [table],
            |row| row.get(0),
        )?;

        if count == 0 {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Get database schema version (for future migrations)
pub fn get_schema_version(_conn: &Connection) -> Result<i32> {
    // For now, we'll use a simple versioning approach
    // In the future, this could be stored in a separate table
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use tempfile::NamedTempFile;

    #[test]
    fn test_schema_initialization() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();

        // Initialize schema
        init_schema(&conn).unwrap();

        // Check if tables exist
        assert!(check_schema(&conn).unwrap());

        // Check schema version
        assert_eq!(get_schema_version(&conn).unwrap(), 1);
    }

    #[test]
    fn test_schema_check_empty_database() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();

        // Should return false for empty database
        assert!(!check_schema(&conn).unwrap());
    }

    #[test]
    fn test_table_structure() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();

        init_schema(&conn).unwrap();

        // Check movies table structure
        let mut movies_columns = conn.prepare("PRAGMA table_info(movies)").unwrap();
        let movies_count = movies_columns.query_map([], |_| Ok(())).unwrap().count();
        assert_eq!(movies_count, 16); // 16 columns in movies table

        // Check parsing_cache table structure
        let mut cache_columns = conn.prepare("PRAGMA table_info(parsing_cache)").unwrap();
        let cache_count = cache_columns.query_map([], |_| Ok(())).unwrap().count();
        assert_eq!(cache_count, 4); // 4 columns in parsing_cache table

        // Check tmdb_cache table structure
        let mut tmdb_columns = conn.prepare("PRAGMA table_info(tmdb_cache)").unwrap();
        let tmdb_count = tmdb_columns.query_map([], |_| Ok(())).unwrap().count();
        assert_eq!(tmdb_count, 4); // 4 columns in tmdb_cache table
    }
}
