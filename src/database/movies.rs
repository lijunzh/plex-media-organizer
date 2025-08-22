//! Movie database operations

use anyhow::Result;
use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Movie record stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieRecord {
    pub id: Option<i64>,
    pub original_filename: String,
    pub parsed_title: String,
    pub year: Option<i32>,
    pub quality: Option<String>,
    pub source: Option<String>,
    pub release_group: Option<String>,
    pub language: Option<String>,
    pub collection_name: Option<String>,
    pub part_number: Option<i32>,
    pub tmdb_id: Option<i32>,
    pub tmdb_title: Option<String>,
    pub tmdb_year: Option<i32>,
    pub confidence_score: Option<f64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl MovieRecord {
    /// Create a new movie record
    pub fn new(original_filename: String, parsed_title: String, year: Option<i32>) -> Self {
        Self {
            id: None,
            original_filename,
            parsed_title,
            year,
            quality: None,
            source: None,
            release_group: None,
            language: None,
            collection_name: None,
            part_number: None,
            tmdb_id: None,
            tmdb_title: None,
            tmdb_year: None,
            confidence_score: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Create a movie record from a database row
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            original_filename: row.get(1)?,
            parsed_title: row.get(2)?,
            year: row.get(3)?,
            quality: row.get(4)?,
            source: row.get(5)?,
            release_group: row.get(6)?,
            language: row.get(7)?,
            collection_name: row.get(8)?,
            part_number: row.get(9)?,
            tmdb_id: row.get(10)?,
            tmdb_title: row.get(11)?,
            tmdb_year: row.get(12)?,
            confidence_score: row.get(13)?,
            created_at: row.get(14)?,
            updated_at: row.get(15)?,
        })
    }
}

/// Movie database operations
pub struct MovieRepository<'a> {
    conn: &'a Connection,
}

impl<'a> MovieRepository<'a> {
    /// Create a new movie repository
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Insert a new movie record
    pub fn insert(&self, movie: &MovieRecord) -> Result<i64> {
        let sql = r#"
            INSERT INTO movies (
                original_filename, parsed_title, year, quality, source,
                release_group, language, collection_name, part_number,
                tmdb_id, tmdb_title, tmdb_year, confidence_score
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        let id = self.conn.execute(
            sql,
            rusqlite::params![
                movie.original_filename,
                movie.parsed_title,
                movie.year,
                movie.quality,
                movie.source,
                movie.release_group,
                movie.language,
                movie.collection_name,
                movie.part_number,
                movie.tmdb_id,
                movie.tmdb_title,
                movie.tmdb_year,
                movie.confidence_score,
            ],
        )?;

        Ok(id as i64)
    }

    /// Update an existing movie record
    pub fn update(&self, movie: &MovieRecord) -> Result<usize> {
        let sql = r#"
            UPDATE movies SET
                parsed_title = ?, year = ?, quality = ?, source = ?,
                release_group = ?, language = ?, collection_name = ?, part_number = ?,
                tmdb_id = ?, tmdb_title = ?, tmdb_year = ?, confidence_score = ?,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        "#;

        let rows_affected = self.conn.execute(
            sql,
            rusqlite::params![
                movie.parsed_title,
                movie.year,
                movie.quality,
                movie.source,
                movie.release_group,
                movie.language,
                movie.collection_name,
                movie.part_number,
                movie.tmdb_id,
                movie.tmdb_title,
                movie.tmdb_year,
                movie.confidence_score,
                movie.id,
            ],
        )?;

        Ok(rows_affected)
    }

    /// Find a movie by its original filename
    pub fn find_by_filename(&self, filename: &str) -> Result<Option<MovieRecord>> {
        let sql = "SELECT * FROM movies WHERE original_filename = ?";

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([filename])?;

        if let Some(row) = rows.next()? {
            Ok(Some(MovieRecord::from_row(row)?))
        } else {
            Ok(None)
        }
    }

    /// Find a movie by its ID
    pub fn find_by_id(&self, id: i64) -> Result<Option<MovieRecord>> {
        let sql = "SELECT * FROM movies WHERE id = ?";

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(MovieRecord::from_row(row)?))
        } else {
            Ok(None)
        }
    }

    /// Find movies by parsed title
    pub fn find_by_title(&self, title: &str) -> Result<Vec<MovieRecord>> {
        let sql = "SELECT * FROM movies WHERE parsed_title LIKE ?";

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([format!("%{}%", title)])?;

        let mut movies = Vec::new();
        while let Some(row) = rows.next()? {
            movies.push(MovieRecord::from_row(row)?);
        }

        Ok(movies)
    }

    /// Find movies by collection name
    pub fn find_by_collection(&self, collection: &str) -> Result<Vec<MovieRecord>> {
        let sql = "SELECT * FROM movies WHERE collection_name = ? ORDER BY part_number";

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([collection])?;

        let mut movies = Vec::new();
        while let Some(row) = rows.next()? {
            movies.push(MovieRecord::from_row(row)?);
        }

        Ok(movies)
    }

    /// Get all movies
    pub fn find_all(&self) -> Result<Vec<MovieRecord>> {
        let sql = "SELECT * FROM movies ORDER BY created_at DESC";

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([])?;

        let mut movies = Vec::new();
        while let Some(row) = rows.next()? {
            movies.push(MovieRecord::from_row(row)?);
        }

        Ok(movies)
    }

    /// Delete a movie by ID
    pub fn delete_by_id(&self, id: i64) -> Result<usize> {
        let sql = "DELETE FROM movies WHERE id = ?";
        let rows_affected = self.conn.execute(sql, [id])?;
        Ok(rows_affected)
    }

    /// Get movie statistics
    pub fn get_statistics(&self) -> Result<HashMap<String, i64>> {
        let mut stats = HashMap::new();

        // Total movies
        let total: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM movies", [], |row| row.get(0))?;
        stats.insert("total_movies".to_string(), total);

        // Movies with TMDB data
        let with_tmdb: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM movies WHERE tmdb_id IS NOT NULL",
            [],
            |row| row.get(0),
        )?;
        stats.insert("movies_with_tmdb".to_string(), with_tmdb);

        // Movies in collections
        let in_collections: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM movies WHERE collection_name IS NOT NULL",
            [],
            |row| row.get(0),
        )?;
        stats.insert("movies_in_collections".to_string(), in_collections);

        // Unique collections
        let unique_collections: i64 = self.conn.query_row(
            "SELECT COUNT(DISTINCT collection_name) FROM movies WHERE collection_name IS NOT NULL",
            [],
            |row| row.get(0),
        )?;
        stats.insert("unique_collections".to_string(), unique_collections);

        Ok(stats)
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
    fn test_insert_and_find_movie() {
        let (_temp_file, conn) = create_test_db();
        let repo = MovieRepository::new(&conn);

        let movie = MovieRecord::new(
            "test.movie.2020.1080p.mkv".to_string(),
            "Test Movie".to_string(),
            Some(2020),
        );

        let id = repo.insert(&movie).unwrap();
        assert!(id > 0);

        let found = repo.find_by_id(id).unwrap().unwrap();
        assert_eq!(found.original_filename, "test.movie.2020.1080p.mkv");
        assert_eq!(found.parsed_title, "Test Movie");
        assert_eq!(found.year, Some(2020));
    }

    #[test]
    fn test_find_by_filename() {
        let (_temp_file, conn) = create_test_db();
        let repo = MovieRepository::new(&conn);

        let movie = MovieRecord::new(
            "unique.filename.2021.mkv".to_string(),
            "Unique Movie".to_string(),
            Some(2021),
        );

        repo.insert(&movie).unwrap();

        let found = repo
            .find_by_filename("unique.filename.2021.mkv")
            .unwrap()
            .unwrap();
        assert_eq!(found.parsed_title, "Unique Movie");

        let not_found = repo.find_by_filename("nonexistent.mkv").unwrap();
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update_movie() {
        let (_temp_file, conn) = create_test_db();
        let repo = MovieRepository::new(&conn);

        let mut movie = MovieRecord::new(
            "update.test.mkv".to_string(),
            "Original Title".to_string(),
            Some(2020),
        );

        let id = repo.insert(&movie).unwrap();
        movie.id = Some(id);
        movie.parsed_title = "Updated Title".to_string();
        movie.year = Some(2021);

        let rows_affected = repo.update(&movie).unwrap();
        assert_eq!(rows_affected, 1);

        let updated = repo.find_by_id(id).unwrap().unwrap();
        assert_eq!(updated.parsed_title, "Updated Title");
        assert_eq!(updated.year, Some(2021));
    }

    #[test]
    fn test_get_statistics() {
        let (_temp_file, conn) = create_test_db();
        let repo = MovieRepository::new(&conn);

        // Insert some test data
        let movie1 = MovieRecord::new("movie1.mkv".to_string(), "Movie 1".to_string(), Some(2020));
        let movie2 = MovieRecord::new("movie2.mkv".to_string(), "Movie 2".to_string(), Some(2021));

        repo.insert(&movie1).unwrap();
        repo.insert(&movie2).unwrap();

        let stats = repo.get_statistics().unwrap();
        assert_eq!(stats["total_movies"], 2);
        assert_eq!(stats["movies_with_tmdb"], 0);
        assert_eq!(stats["movies_in_collections"], 0);
    }
}
