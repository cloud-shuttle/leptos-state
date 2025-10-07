//! Database-based configuration sources

use super::traits::{ConfigSourceTrait, ConfigError, SourceMetadata};
use async_trait::async_trait;
use std::time::Duration;

/// Database configuration source
#[derive(Debug, Clone)]
pub struct DatabaseSource {
    /// Database connection string
    pub connection_string: String,
    /// Table name containing configuration
    pub table: String,
    /// Column name for configuration keys
    pub key_column: String,
    /// Column name for configuration values
    pub value_column: String,
    /// Query timeout
    pub query_timeout: Duration,
    /// Maximum number of connection retries
    pub max_retries: u32,
}

impl DatabaseSource {
    /// Create a new database source
    pub fn new(connection_string: impl Into<String>) -> Self {
        Self {
            connection_string: connection_string.into(),
            table: "configuration".to_string(),
            key_column: "key".to_string(),
            value_column: "value".to_string(),
            query_timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }

    /// Set the configuration table name
    pub fn with_table(mut self, table: impl Into<String>) -> Self {
        self.table = table.into();
        self
    }

    /// Set the key column name
    pub fn with_key_column(mut self, column: impl Into<String>) -> Self {
        self.key_column = column.into();
        self
    }

    /// Set the value column name
    pub fn with_value_column(mut self, column: impl Into<String>) -> Self {
        self.value_column = column.into();
        self
    }

    /// Set the query timeout
    pub fn with_query_timeout(mut self, timeout: Duration) -> Self {
        self.query_timeout = timeout;
        self
    }

    /// Set the maximum retry count
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Extract database type from connection string
    pub fn database_type(&self) -> &str {
        if self.connection_string.contains("postgresql://") || self.connection_string.contains("postgres://") {
            "postgresql"
        } else if self.connection_string.contains("mysql://") {
            "mysql"
        } else if self.connection_string.contains("sqlite://") {
            "sqlite"
        } else if self.connection_string.contains("mongodb://") {
            "mongodb"
        } else {
            "unknown"
        }
    }
}

impl Default for DatabaseSource {
    fn default() -> Self {
        Self::new("")
    }
}

#[async_trait]
impl ConfigSourceTrait for DatabaseSource {
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        if self.connection_string.trim().is_empty() {
            return Err(ConfigError::InvalidConnection("Connection string cannot be empty".into()));
        }

        // This is a placeholder implementation
        // In a real implementation, you would:
        // 1. Connect to the database based on the connection string
        // 2. Execute a query to fetch configuration data
        // 3. Parse the results into a JSON structure

        match self.database_type() {
            "postgresql" | "mysql" | "sqlite" => {
                // Simulate database connection and query
                // This would normally use database-specific crates
                Err(ConfigError::Database(format!(
                    "Database type '{}' not yet implemented. Connection: {}",
                    self.database_type(),
                    self.connection_string.chars().take(50).collect::<String>()
                )))
            }
            "mongodb" => {
                Err(ConfigError::Database("MongoDB support not yet implemented".into()))
            }
            _ => {
                Err(ConfigError::InvalidConnection(format!(
                    "Unsupported database type in connection string: {}",
                    self.connection_string.chars().take(30).collect::<String>()
                )))
            }
        }
    }

    async fn is_available(&self) -> bool {
        if self.connection_string.trim().is_empty() {
            return false;
        }

        // This is a placeholder implementation
        // In a real implementation, you would attempt to connect to the database
        // and verify the connection is successful

        // For now, just check if the connection string looks valid
        let db_type = self.database_type();
        !matches!(db_type, "unknown") && self.connection_string.len() > 10
    }

    fn priority(&self) -> u8 {
        60 // Lower priority than HTTP sources
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.connection_string.trim().is_empty() {
            return Err(ConfigError::InvalidConnection("Connection string cannot be empty".into()));
        }

        if self.table.trim().is_empty() {
            return Err(ConfigError::Validation("Table name cannot be empty".into()));
        }

        if self.key_column.trim().is_empty() || self.value_column.trim().is_empty() {
            return Err(ConfigError::Validation("Column names cannot be empty".into()));
        }

        if self.query_timeout.as_secs() == 0 {
            return Err(ConfigError::Validation("Query timeout cannot be zero".into()));
        }

        // Validate database type
        match self.database_type() {
            "unknown" => {
                return Err(ConfigError::InvalidConnection(format!(
                    "Cannot determine database type from connection string: {}",
                    self.connection_string.chars().take(30).collect::<String>()
                )));
            }
            "postgresql" | "mysql" | "sqlite" | "mongodb" => {
                // Valid database types
            }
            _ => {
                return Err(ConfigError::InvalidConnection(format!(
                    "Unsupported database type: {}", self.database_type()
                )));
            }
        }

        Ok(())
    }

    fn metadata(&self) -> SourceMetadata {
        let db_type = self.database_type();
        let host = self.extract_host();

        SourceMetadata {
            name: format!("{} Database Source", db_type),
            source_type: "database".to_string(),
            priority: self.priority(),
            description: format!(
                "Loads configuration from {} database{}{}",
                db_type,
                host.map(|h| format!(" at {}", h)).unwrap_or_default(),
                format!(" (table: {})", self.table)
            ),
        }
    }
}

impl DatabaseSource {
    /// Extract host information from connection string
    fn extract_host(&self) -> Option<String> {
        // Simple host extraction - in a real implementation,
        // you would use proper URL parsing
        if let Some(start) = self.connection_string.find("://") {
            let after_protocol = &self.connection_string[start + 3..];
            if let Some(end) = after_protocol.find('/') {
                Some(after_protocol[..end].to_string())
            } else if let Some(end) = after_protocol.find('?') {
                Some(after_protocol[..end].to_string())
            } else {
                Some(after_protocol.to_string())
            }
        } else {
            None
        }
    }
}

/// Database configuration source (alias for DatabaseSource)
pub type Database = DatabaseSource;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_source_validation() {
        // Valid PostgreSQL connection
        let source = DatabaseSource::new("postgresql://user:pass@localhost:5432/config_db");
        assert!(source.validate().is_ok());
        assert_eq!(source.database_type(), "postgresql");

        // Valid SQLite connection
        let source = DatabaseSource::new("sqlite://./config.db");
        assert!(source.validate().is_ok());
        assert_eq!(source.database_type(), "sqlite");

        // Invalid - empty connection string
        let source = DatabaseSource::new("");
        assert!(matches!(source.validate(), Err(ConfigError::InvalidConnection(_))));

        // Invalid - unknown database type
        let source = DatabaseSource::new("unknown://localhost/db");
        assert!(matches!(source.validate(), Err(ConfigError::InvalidConnection(_))));

        // Invalid - empty table name
        let source = DatabaseSource::new("postgresql://localhost/db")
            .with_table("");
        assert!(matches!(source.validate(), Err(ConfigError::Validation(_))));
    }

    #[test]
    fn test_database_source_metadata() {
        let source = DatabaseSource::new("postgresql://user@localhost:5432/myapp")
            .with_table("app_config");

        let metadata = source.metadata();
        assert_eq!(metadata.source_type, "database");
        assert_eq!(metadata.priority, 60);
        assert!(metadata.name.contains("postgresql"));
        assert!(metadata.description.contains("postgresql"));
        assert!(metadata.description.contains("app_config"));
    }

    #[test]
    fn test_host_extraction() {
        let source = DatabaseSource::new("postgresql://user:pass@localhost:5432/config");
        assert_eq!(source.extract_host(), Some("user:pass@localhost:5432".to_string()));

        let source = DatabaseSource::new("mysql://localhost/db");
        assert_eq!(source.extract_host(), Some("localhost".to_string()));

        let source = DatabaseSource::new("invalid-connection-string");
        assert_eq!(source.extract_host(), None);
    }

    #[tokio::test]
    async fn test_database_source_availability() {
        // Valid connection string should be considered available
        let source = DatabaseSource::new("postgresql://localhost:5432/db");
        assert!(source.is_available().await);

        // Empty connection string should not be available
        let source = DatabaseSource::new("");
        assert!(!source.is_available().await);
    }
}
