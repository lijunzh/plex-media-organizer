//! Database connection management and pooling

use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub path: String,
    pub max_connections: usize,
    pub cache_ttl_hours: i64,
    pub enable_wal: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: "data/movies.db".to_string(),
            max_connections: 10,
            cache_ttl_hours: 24,
            enable_wal: true,
        }
    }
}

/// Database connection pool
pub struct ConnectionPool {
    config: DatabaseConfig,
    connections: Arc<Mutex<Vec<Connection>>>,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        // Ensure the database directory exists
        if let Some(parent) = Path::new(&config.path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(Self {
            config,
            connections: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Get a database connection from the pool
    pub async fn get_connection(&self) -> Result<PooledConnection> {
        let mut connections = self.connections.lock().await;

        // Try to get an existing connection
        if let Some(conn) = connections.pop() {
            return Ok(PooledConnection {
                connection: Some(conn),
                pool: Arc::clone(&self.connections),
            });
        }

        // Create a new connection
        let conn = self.create_connection()?;
        Ok(PooledConnection {
            connection: Some(conn),
            pool: Arc::clone(&self.connections),
        })
    }

    /// Create a new database connection
    fn create_connection(&self) -> Result<Connection> {
        let conn = Connection::open_with_flags(
            &self.config.path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;

        // For now, skip PRAGMA statements to avoid issues
        // TODO: Re-enable PRAGMA statements once we understand the issue

        Ok(conn)
    }

    /// Get the database configuration
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// Close all connections in the pool
    pub async fn close_all(&self) -> Result<()> {
        let mut connections = self.connections.lock().await;
        connections.clear();
        Ok(())
    }

    /// Get pool statistics
    pub async fn stats(&self) -> PoolStats {
        let connections = self.connections.lock().await;
        PoolStats {
            available_connections: connections.len(),
            max_connections: self.config.max_connections,
            cache_ttl_hours: self.config.cache_ttl_hours,
        }
    }
}

/// A pooled database connection that returns to the pool when dropped
pub struct PooledConnection {
    connection: Option<Connection>,
    pool: Arc<Mutex<Vec<Connection>>>,
}

impl PooledConnection {
    /// Get a reference to the underlying connection
    pub fn connection(&self) -> &Connection {
        self.connection.as_ref().unwrap()
    }

    /// Get a mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> &mut Connection {
        self.connection.as_mut().unwrap()
    }

    /// Consume the connection and return the underlying Connection
    pub fn into_inner(mut self) -> Connection {
        self.connection.take().unwrap()
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(conn) = self.connection.take() {
            // Try to return the connection to the pool
            // If the pool is full, just drop the connection
            let pool = Arc::clone(&self.pool);
            tokio::spawn(async move {
                let mut connections = pool.lock().await;
                if connections.len() < 10 {
                    // Max pool size
                    connections.push(conn);
                }
            });
        }
    }
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub available_connections: usize,
    pub max_connections: usize,
    pub cache_ttl_hours: i64,
}

impl PoolStats {
    /// Get the connection pool utilization percentage
    pub fn utilization(&self) -> f64 {
        if self.max_connections == 0 {
            0.0
        } else {
            (self.available_connections as f64 / self.max_connections as f64) * 100.0
        }
    }
}

/// Database manager that uses connection pooling
pub struct DatabaseManager {
    pool: ConnectionPool,
}

impl DatabaseManager {
    /// Create a new database manager with the given configuration
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        let pool = ConnectionPool::new(config)?;
        Ok(Self { pool })
    }

    /// Get a database connection
    pub async fn get_connection(&self) -> Result<PooledConnection> {
        self.pool.get_connection().await
    }

    /// Initialize the database schema
    pub async fn initialize(&self) -> Result<()> {
        let conn = self.get_connection().await?;
        crate::database::schema::init_schema(conn.connection())?;
        Ok(())
    }

    /// Check if the database is properly initialized
    pub async fn is_initialized(&self) -> Result<bool> {
        let conn = self.get_connection().await?;
        crate::database::schema::check_schema(conn.connection())
    }

    /// Get database statistics
    pub async fn stats(&self) -> PoolStats {
        self.pool.stats().await
    }

    /// Close all database connections
    pub async fn close(&self) -> Result<()> {
        self.pool.close_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_connection_pool_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = DatabaseConfig {
            path: temp_file.path().to_string_lossy().to_string(),
            max_connections: 5,
            cache_ttl_hours: 24,
            enable_wal: true,
        };

        let pool = ConnectionPool::new(config).unwrap();
        let stats = pool.stats().await;
        assert_eq!(stats.available_connections, 0);
        assert_eq!(stats.max_connections, 5);
    }

    #[tokio::test]
    async fn test_connection_retrieval() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = DatabaseConfig {
            path: temp_file.path().to_string_lossy().to_string(),
            max_connections: 5,
            cache_ttl_hours: 24,
            enable_wal: true,
        };

        let pool = ConnectionPool::new(config).unwrap();

        // Get a connection
        let conn = pool.get_connection().await.unwrap();
        // Test that we can execute a simple query
        conn.connection()
            .execute("CREATE TABLE test (id INTEGER)", [])
            .unwrap();

        // Check pool stats
        let stats = pool.stats().await;
        assert_eq!(stats.available_connections, 0); // Connection is in use
    }

    #[tokio::test]
    async fn test_connection_return_to_pool() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = DatabaseConfig {
            path: temp_file.path().to_string_lossy().to_string(),
            max_connections: 5,
            cache_ttl_hours: 24,
            enable_wal: true,
        };

        let pool = ConnectionPool::new(config).unwrap();

        // Get and drop a connection
        {
            let conn = pool.get_connection().await.unwrap();
            conn.connection()
                .execute("CREATE TABLE test2 (id INTEGER)", [])
                .unwrap();
        }

        // Wait a bit for the connection to be returned
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Check pool stats
        let stats = pool.stats().await;
        assert_eq!(stats.available_connections, 1); // Connection returned to pool
    }

    #[tokio::test]
    async fn test_database_manager() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = DatabaseConfig {
            path: temp_file.path().to_string_lossy().to_string(),
            max_connections: 5,
            cache_ttl_hours: 24,
            enable_wal: true,
        };

        let manager = DatabaseManager::new(config).unwrap();

        // Initialize database
        manager.initialize().await.unwrap();

        // Check if initialized
        assert!(manager.is_initialized().await.unwrap());

        // Get connection
        let conn = manager.get_connection().await.unwrap();
        // Test that we can execute a simple query
        conn.connection()
            .execute("CREATE TABLE test3 (id INTEGER)", [])
            .unwrap();
    }
}
