//! Operation history management for database-backed rollback functionality

use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[allow(unused_imports)]
use crate::cli::handlers::organize::{
    OperationType, OrganizationResult, OrganizationStats, OrganizedFile,
};

/// Operation record stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub operation_id: String,
    pub operation_type: String,
    pub source_directory: PathBuf,
    pub output_directory: Option<PathBuf>,
    pub total_files: usize,
    pub successful_files: usize,
    pub skipped_files: usize,
    pub failed_files: usize,
    pub total_size_moved: Option<u64>,
    pub average_confidence: Option<f32>,
    pub operation_data: Option<String>, // JSON string of the full operation result
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: String,
}

/// Operation file record stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationFileRecord {
    pub operation_id: String,
    pub original_path: PathBuf,
    pub new_path: PathBuf,
    pub file_size: u64,
    pub operation_type: String,
    pub confidence: Option<f32>,
    pub created_at: DateTime<Utc>,
}

/// Operation history manager
pub struct OperationHistoryManager {
    conn: Connection,
}

impl OperationHistoryManager {
    /// Create a new operation history manager
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Store an organization operation in the database
    pub fn store_operation(
        &self,
        result: &OrganizationResult,
        source_dir: &Path,
        output_dir: Option<&Path>,
    ) -> Result<String> {
        let operation_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Serialize the result to JSON
        let json_data = serde_json::to_string(result)?;

        // Store main operation record
        self.conn.execute(
            "INSERT INTO operations (
                operation_id, operation_type, source_directory, output_directory,
                total_files, successful_files, skipped_files, failed_files,
                total_size_moved, average_confidence, operation_data, created_at, completed_at, status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                operation_id,
                "organize",
                source_dir.to_string_lossy(),
                output_dir.map(|p| p.to_string_lossy().to_string()),
                result.total_files,
                result.organized_files.len(),
                result.skipped_files.len(),
                result.failed_files.len(),
                result.organization_stats.total_size_moved,
                result.organization_stats.average_confidence,
                json_data,
                now.to_rfc3339(),
                now.to_rfc3339(),
                "completed"
            ],
        )?;

        // Store individual file records
        for organized_file in &result.organized_files {
            self.conn.execute(
                "INSERT INTO operation_files (
                    operation_id, original_path, new_path, file_size, operation_type, confidence
                ) VALUES (?, ?, ?, ?, ?, ?)",
                rusqlite::params![
                    operation_id,
                    organized_file.original_path.to_string_lossy(),
                    organized_file.new_path.to_string_lossy(),
                    organized_file.media_file.file_size,
                    format!("{:?}", organized_file.operation_type),
                    None::<f32> // MediaMetadata doesn't have confidence field, we'll use None for now
                ],
            )?;
        }

        Ok(operation_id)
    }

    /// Get an operation by ID
    pub fn get_operation(&self, operation_id: &str) -> Result<Option<OperationRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT operation_id, operation_type, source_directory, output_directory,
                    total_files, successful_files, skipped_files, failed_files,
                    total_size_moved, average_confidence, operation_data, created_at, completed_at, status
             FROM operations WHERE operation_id = ?",
        )?;

        let mut rows = stmt.query([operation_id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_operation_record(row)?))
        } else {
            Ok(None)
        }
    }

    /// Get all operations, optionally filtered by type
    pub fn get_operations(&self, operation_type: Option<&str>) -> Result<Vec<OperationRecord>> {
        let sql = if let Some(_op_type) = operation_type {
            "SELECT operation_id, operation_type, source_directory, output_directory,
                    total_files, successful_files, skipped_files, failed_files,
                    total_size_moved, average_confidence, operation_data, created_at, completed_at, status
             FROM operations WHERE operation_type = ? ORDER BY created_at DESC"
        } else {
            "SELECT operation_id, operation_type, source_directory, output_directory,
                    total_files, successful_files, skipped_files, failed_files,
                    total_size_moved, average_confidence, operation_data, created_at, completed_at, status
             FROM operations ORDER BY created_at DESC"
        };

        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = if let Some(op_type) = operation_type {
            stmt.query([op_type])?
        } else {
            stmt.query([])?
        };

        let mut operations = Vec::new();
        while let Some(row) = rows.next()? {
            operations.push(self.row_to_operation_record(row)?);
        }

        Ok(operations)
    }

    /// Get operation files for a specific operation
    pub fn get_operation_files(&self, operation_id: &str) -> Result<Vec<OperationFileRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT operation_id, original_path, new_path, file_size, operation_type, confidence, created_at
             FROM operation_files WHERE operation_id = ? ORDER BY created_at"
        )?;

        let mut rows = stmt.query([operation_id])?;
        let mut files = Vec::new();

        while let Some(row) = rows.next()? {
            files.push(self.row_to_operation_file_record(row)?);
        }

        Ok(files)
    }

    /// Get recent operations (last N operations)
    pub fn get_recent_operations(&self, limit: usize) -> Result<Vec<OperationRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT operation_id, operation_type, source_directory, output_directory,
                    total_files, successful_files, skipped_files, failed_files,
                    total_size_moved, average_confidence, operation_data, created_at, completed_at, status
             FROM operations ORDER BY created_at DESC LIMIT ?",
        )?;

        let mut rows = stmt.query([limit as i64])?;
        let mut operations = Vec::new();

        while let Some(row) = rows.next()? {
            operations.push(self.row_to_operation_record(row)?);
        }

        Ok(operations)
    }

    /// Delete old operations (older than specified days)
    pub fn cleanup_old_operations(&self, days: u32) -> Result<usize> {
        let cutoff_date = Utc::now() - chrono::Duration::days(days as i64);

        // Delete operation files first (due to foreign key constraint)
        let _files_deleted = self.conn.execute(
            "DELETE FROM operation_files WHERE operation_id IN (
                SELECT operation_id FROM operations WHERE created_at < ?
            )",
            [cutoff_date.to_rfc3339()],
        )?;

        // Delete operations
        let operations_deleted = self.conn.execute(
            "DELETE FROM operations WHERE created_at < ?",
            [cutoff_date.to_rfc3339()],
        )?;

        Ok(operations_deleted)
    }

    /// Get operation statistics
    pub fn get_operation_stats(&self) -> Result<OperationStats> {
        let total_operations: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM operations", [], |row| row.get(0))?;

        let total_files_processed: i64 = self
            .conn
            .query_row("SELECT SUM(total_files) FROM operations", [], |row| {
                row.get(0)
            })
            .unwrap_or(0);

        let total_size_moved: i64 = self
            .conn
            .query_row(
                "SELECT SUM(total_size_moved) FROM operations WHERE total_size_moved IS NOT NULL",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let avg_confidence: f64 = self.conn.query_row(
            "SELECT AVG(average_confidence) FROM operations WHERE average_confidence IS NOT NULL",
            [],
            |row| row.get(0),
        ).unwrap_or(0.0);

        Ok(OperationStats {
            total_operations: total_operations as usize,
            total_files_processed: total_files_processed as usize,
            total_size_moved: total_size_moved as u64,
            average_confidence: avg_confidence as f32,
        })
    }

    /// Convert database row to OperationRecord
    fn row_to_operation_record(&self, row: &Row) -> Result<OperationRecord> {
        Ok(OperationRecord {
            operation_id: row.get(0)?,
            operation_type: row.get(1)?,
            source_directory: PathBuf::from(row.get::<_, String>(2)?),
            output_directory: row.get::<_, Option<String>>(3)?.map(PathBuf::from),
            total_files: row.get(4)?,
            successful_files: row.get(5)?,
            skipped_files: row.get(6)?,
            failed_files: row.get(7)?,
            total_size_moved: row.get(8)?,
            average_confidence: row.get(9)?,
            operation_data: row.get(10)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(11)?)?
                .with_timezone(&Utc),
            completed_at: row.get::<_, Option<String>>(12)?.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
            status: row.get(13)?,
        })
    }

    /// Convert database row to OperationFileRecord
    fn row_to_operation_file_record(&self, row: &Row) -> Result<OperationFileRecord> {
        let created_at_str: String = row.get(6)?;

        // Parse SQLite timestamp format (YYYY-MM-DD HH:MM:SS)
        let created_at = if created_at_str.contains('T') {
            // RFC3339 format
            DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc)
        } else {
            // SQLite format (YYYY-MM-DD HH:MM:SS)
            let naive_dt =
                chrono::NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")?;
            DateTime::from_naive_utc_and_offset(naive_dt, Utc)
        };

        Ok(OperationFileRecord {
            operation_id: row.get(0)?,
            original_path: PathBuf::from(row.get::<_, String>(1)?),
            new_path: PathBuf::from(row.get::<_, String>(2)?),
            file_size: row.get(3)?,
            operation_type: row.get(4)?,
            confidence: row.get(5)?,
            created_at,
        })
    }
}

/// Operation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationStats {
    pub total_operations: usize,
    pub total_files_processed: usize,
    pub total_size_moved: u64,
    pub average_confidence: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_json_serialization() -> Result<()> {
        // Test JSON serialization/deserialization in isolation
        let result = crate::cli::handlers::organize::OrganizationResult {
            total_files: 2,
            organized_files: vec![crate::cli::handlers::organize::OrganizedFile {
                original_path: PathBuf::from("/test/source/movie1.mkv"),
                new_path: PathBuf::from("/test/output/Movie 1 (2020).mkv"),
                media_file: crate::types::MediaFile {
                    id: "1".to_string(),
                    file_path: PathBuf::from("/test/source/movie1.mkv"),
                    file_name: "movie1.mkv".to_string(),
                    file_size: 1024,
                    media_type: crate::types::MediaType::Movie,
                    content_hash: "hash1".to_string(),
                    last_modified: Utc::now(),
                    metadata: crate::types::MediaMetadata::default(),
                },
                operation_type: crate::cli::handlers::organize::OperationType::Move,
            }],
            skipped_files: vec![],
            failed_files: vec![],
            organization_stats: crate::cli::handlers::organize::OrganizationStats {
                successful_organizations: 1,
                skipped_files: 0,
                failed_files: 0,
                total_size_moved: 1024,
                average_confidence: 0.9,
                operation_duration: std::time::Duration::from_secs(1),
            },
            operation_duration: std::time::Duration::from_secs(1),
        };

        // Test serialization
        let json = serde_json::to_string(&result)?;
        eprintln!("Serialized JSON: {}", json);

        // Test deserialization
        let deserialized: crate::cli::handlers::organize::OrganizationResult =
            serde_json::from_str(&json)?;
        assert_eq!(deserialized.total_files, 2);
        assert_eq!(deserialized.organized_files.len(), 1);

        Ok(())
    }

    #[test]
    fn test_operation_history_manager() -> Result<()> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let conn = Connection::open(&db_path)?;

        // Initialize schema
        crate::database::schema::init_schema(&conn)?;

        let manager = OperationHistoryManager::new(conn);

        // Test storing and retrieving operations
        let _operation_id = "test-op-123".to_string();
        let source_dir = PathBuf::from("/test/source");
        let output_dir = PathBuf::from("/test/output");

        // Create a mock organization result
        let result = OrganizationResult {
            total_files: 2,
            organized_files: vec![OrganizedFile {
                original_path: PathBuf::from("/test/source/movie1.mkv"),
                new_path: PathBuf::from("/test/output/Movie 1 (2020).mkv"),
                media_file: crate::types::MediaFile {
                    id: "1".to_string(),
                    file_path: PathBuf::from("/test/source/movie1.mkv"),
                    file_name: "movie1.mkv".to_string(),
                    file_size: 1024,
                    media_type: crate::types::MediaType::Movie,
                    content_hash: "hash1".to_string(),
                    last_modified: Utc::now(),
                    metadata: crate::types::MediaMetadata::default(),
                },
                operation_type: OperationType::Move,
            }],
            skipped_files: vec![],
            failed_files: vec![],
            organization_stats: OrganizationStats {
                successful_organizations: 1,
                skipped_files: 0,
                failed_files: 0,
                total_size_moved: 1024,
                average_confidence: 0.9,
                operation_duration: std::time::Duration::from_secs(1),
            },
            operation_duration: std::time::Duration::from_secs(1),
        };

        // Store operation
        let stored_id = manager.store_operation(&result, &source_dir, Some(&output_dir))?;

        // Retrieve operation
        let retrieved = manager.get_operation(&stored_id)?.unwrap();
        assert_eq!(retrieved.operation_type, "organize");
        assert_eq!(retrieved.total_files, 2);
        assert_eq!(retrieved.successful_files, 1);

        // Get operation files
        let files = manager.get_operation_files(&stored_id)?;
        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].original_path,
            PathBuf::from("/test/source/movie1.mkv")
        );

        // Test JSON deserialization
        if let Some(json_data) = &retrieved.operation_data {
            let deserialized: crate::cli::handlers::organize::OrganizationResult =
                serde_json::from_str(json_data)?;
            assert_eq!(deserialized.total_files, 2);
            assert_eq!(deserialized.organized_files.len(), 1);
        }

        Ok(())
    }
}
