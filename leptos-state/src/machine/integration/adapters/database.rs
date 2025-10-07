//! Database integration adapter

use super::traits::{IntegrationAdapterTrait, AdapterType, AdapterStats, ConnectionConfig, AdapterFeature};
use super::super::events::{IntegrationEvent, IntegrationError};
use async_trait::async_trait;

/// Database adapter for storing integration events
#[derive(Debug)]
pub struct DatabaseAdapter {
    /// Configuration
    pub config: ConnectionConfig,
    /// Table name for storing events
    pub table_name: String,
    /// Statistics
    stats: std::sync::Mutex<AdapterStats>,
}

impl DatabaseAdapter {
    /// Create a new database adapter
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            table_name: "integration_events".to_string(),
            config,
            stats: std::sync::Mutex::new(AdapterStats::default()),
        }
    }

    /// Set the table name for storing events
    pub fn with_table_name(mut self, table_name: impl Into<String>) -> Self {
        self.table_name = table_name.into();
        self
    }

    /// Get the database type from connection string
    pub fn database_type(&self) -> &str {
        if self.config.url.contains("postgresql://") || self.config.url.contains("postgres://") {
            "postgresql"
        } else if self.config.url.contains("mysql://") {
            "mysql"
        } else if self.config.url.contains("sqlite://") {
            "sqlite"
        } else {
            "unknown"
        }
    }

    /// Create the events table (placeholder implementation)
    pub async fn create_table(&self) -> Result<(), IntegrationError> {
        // This is a placeholder. In a real implementation, you would:
        // 1. Connect to the database
        // 2. Execute CREATE TABLE IF NOT EXISTS statement
        // 3. Handle different database types

        match self.database_type() {
            "postgresql" => {
                // PostgreSQL table creation
                Err(IntegrationError::NotImplemented("PostgreSQL table creation not implemented".into()))
            }
            "mysql" => {
                // MySQL table creation
                Err(IntegrationError::NotImplemented("MySQL table creation not implemented".into()))
            }
            "sqlite" => {
                // SQLite table creation
                Err(IntegrationError::NotImplemented("SQLite table creation not implemented".into()))
            }
            _ => {
                Err(IntegrationError::ConnectionError(format!(
                    "Unsupported database type: {}", self.database_type()
                )))
            }
        }
    }

    /// Store an event in the database (placeholder implementation)
    async fn store_event(&self, event: &IntegrationEvent) -> Result<(), IntegrationError> {
        // This is a placeholder. In a real implementation, you would:
        // 1. Connect to the database
        // 2. Prepare an INSERT statement
        // 3. Execute the statement with event data
        // 4. Handle transactions if needed

        let event_json = serde_json::to_string(event)
            .map_err(|e| IntegrationError::SerializationError(format!("Failed to serialize event: {}", e)))?;

        match self.database_type() {
            "postgresql" => {
                // PostgreSQL insert
                Err(IntegrationError::NotImplemented(format!(
                    "PostgreSQL event storage not implemented. Event: {}", event_json
                )))
            }
            "mysql" => {
                // MySQL insert
                Err(IntegrationError::NotImplemented(format!(
                    "MySQL event storage not implemented. Event: {}", event_json
                )))
            }
            "sqlite" => {
                // SQLite insert
                Err(IntegrationError::NotImplemented(format!(
                    "SQLite event storage not implemented. Event: {}", event_json
                )))
            }
            _ => {
                Err(IntegrationError::ConnectionError(format!(
                    "Unsupported database type: {}. Event: {}", self.database_type(), event_json
                )))
            }
        }
    }

    /// Retrieve events from the database (placeholder implementation)
    async fn retrieve_events(&self, limit: Option<usize>) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        // This is a placeholder. In a real implementation, you would:
        // 1. Connect to the database
        // 2. Execute a SELECT query
        // 3. Parse results into IntegrationEvent structs

        match self.database_type() {
            "postgresql" => {
                Err(IntegrationError::NotImplemented("PostgreSQL event retrieval not implemented".into()))
            }
            "mysql" => {
                Err(IntegrationError::NotImplemented("MySQL event retrieval not implemented".into()))
            }
            "sqlite" => {
                Err(IntegrationError::NotImplemented("SQLite event retrieval not implemented".into()))
            }
            _ => {
                Err(IntegrationError::ConnectionError(format!(
                    "Unsupported database type: {}", self.database_type()
                )))
            }
        }
    }
}

#[async_trait]
impl IntegrationAdapterTrait for DatabaseAdapter {
    fn adapter_type(&self) -> AdapterType {
        AdapterType::Database
    }

    fn name(&self) -> &str {
        "Database Adapter"
    }

    fn config(&self) -> serde_json::Value {
        serde_json::json!({
            "connection_url": self.config.url,
            "timeout_seconds": self.config.timeout.as_secs(),
            "max_retries": self.config.max_retries,
            "table_name": self.table_name,
            "database_type": self.database_type(),
            "extra_config": self.config.extra_config
        })
    }

    async fn health_check(&self) -> Result<(), IntegrationError> {
        // Placeholder health check
        // In a real implementation, you would attempt to connect to the database

        if self.config.url.trim().is_empty() {
            return Err(IntegrationError::ConnectionError("Database URL is empty".into()));
        }

        match self.database_type() {
            "postgresql" | "mysql" | "sqlite" => {
                // In a real implementation, we would try to establish a connection
                // For now, just validate the connection string format
                Err(IntegrationError::NotImplemented(format!(
                    "{} health check not implemented", self.database_type()
                )))
            }
            _ => {
                Err(IntegrationError::ConnectionError(format!(
                    "Unsupported database type: {}", self.database_type()
                )))
            }
        }
    }

    async fn send_event(&self, event: &IntegrationEvent) -> Result<(), IntegrationError> {
        let result = self.store_event(event).await;

        let mut stats = self.stats.lock().unwrap();
        match result {
            Ok(()) => {
                stats.record_success();
                Ok(())
            }
            Err(e) => {
                stats.record_error();
                Err(e)
            }
        }
    }

    async fn receive_events(&self) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        let events = self.retrieve_events(Some(100)).await?;

        let mut stats = self.stats.lock().unwrap();
        for _ in &events {
            stats.record_received();
        }

        Ok(events)
    }

    fn stats(&self) -> AdapterStats {
        self.stats.lock().unwrap().clone()
    }

    fn supports_feature(&self, feature: AdapterFeature) -> bool {
        match feature {
            AdapterFeature::SendEvents => true,
            AdapterFeature::ReceiveEvents => true,
            AdapterFeature::HealthCheck => true,
            AdapterFeature::Batching => true, // Databases naturally support batching
            AdapterFeature::RetryLogic => false, // Would need transaction management
            AdapterFeature::Authentication => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_database_adapter_creation() {
        let config = ConnectionConfig {
            url: "postgresql://user:pass@localhost:5432/db".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            extra_config: serde_json::Value::Null,
        };

        let adapter = DatabaseAdapter::new(config);
        assert_eq!(adapter.adapter_type(), AdapterType::Database);
        assert_eq!(adapter.name(), "Database Adapter");
        assert_eq!(adapter.database_type(), "postgresql");
        assert_eq!(adapter.table_name, "integration_events");
    }

    #[test]
    fn test_database_type_detection() {
        let postgres_config = ConnectionConfig {
            url: "postgresql://localhost/db".to_string(),
            ..Default::default()
        };
        let adapter = DatabaseAdapter::new(postgres_config);
        assert_eq!(adapter.database_type(), "postgresql");

        let mysql_config = ConnectionConfig {
            url: "mysql://localhost/db".to_string(),
            ..Default::default()
        };
        let adapter = DatabaseAdapter::new(mysql_config);
        assert_eq!(adapter.database_type(), "mysql");

        let sqlite_config = ConnectionConfig {
            url: "sqlite://./db.sqlite".to_string(),
            ..Default::default()
        };
        let adapter = DatabaseAdapter::new(sqlite_config);
        assert_eq!(adapter.database_type(), "sqlite");

        let unknown_config = ConnectionConfig {
            url: "unknown://localhost/db".to_string(),
            ..Default::default()
        };
        let adapter = DatabaseAdapter::new(unknown_config);
        assert_eq!(adapter.database_type(), "unknown");
    }

    #[test]
    fn test_table_name_configuration() {
        let config = ConnectionConfig::default();
        let adapter = DatabaseAdapter::new(config).with_table_name("custom_events");
        assert_eq!(adapter.table_name, "custom_events");
    }

    #[test]
    fn test_feature_support() {
        let adapter = DatabaseAdapter::new(ConnectionConfig::default());

        assert!(adapter.supports_feature(AdapterFeature::SendEvents));
        assert!(adapter.supports_feature(AdapterFeature::ReceiveEvents));
        assert!(adapter.supports_feature(AdapterFeature::HealthCheck));
        assert!(adapter.supports_feature(AdapterFeature::Batching));
        assert!(adapter.supports_feature(AdapterFeature::Authentication));
        assert!(!adapter.supports_feature(AdapterFeature::RetryLogic));
    }

    #[test]
    fn test_config_serialization() {
        let config = ConnectionConfig {
            url: "postgresql://user@localhost/db".to_string(),
            timeout: Duration::from_secs(60),
            max_retries: 5,
            extra_config: serde_json::json!({"ssl_mode": "require"}),
        };

        let adapter = DatabaseAdapter::new(config).with_table_name("events");

        let config_json = adapter.config();
        assert_eq!(config_json["connection_url"], "postgresql://user@localhost/db");
        assert_eq!(config_json["timeout_seconds"], 60);
        assert_eq!(config_json["max_retries"], 5);
        assert_eq!(config_json["table_name"], "events");
        assert_eq!(config_json["database_type"], "postgresql");
        assert_eq!(config_json["extra_config"]["ssl_mode"], "require");
    }
}
