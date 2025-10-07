//! File system integration adapter

use super::traits::{IntegrationAdapterTrait, AdapterType, AdapterStats, ConnectionConfig, AdapterFeature};
use super::super::events::{IntegrationEvent, IntegrationError};
use async_trait::async_trait;

/// File system adapter for storing integration events as files
#[derive(Debug)]
pub struct FileSystemAdapter {
    /// Configuration
    pub config: ConnectionConfig,
    /// Base directory for storing events
    pub base_directory: String,
    /// File naming pattern
    pub file_pattern: String,
    /// File format (json, csv, etc.)
    pub file_format: FileFormat,
    /// Statistics
    stats: std::sync::Mutex<AdapterStats>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FileFormat {
    Json,
    Csv,
    Text,
}

impl FileSystemAdapter {
    /// Create a new file system adapter
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            base_directory: "./integration_events".to_string(),
            file_pattern: "{timestamp}_{event_type}_{id}".to_string(),
            file_format: FileFormat::Json,
            config,
            stats: std::sync::Mutex::new(AdapterStats::default()),
        }
    }

    /// Set the base directory for storing events
    pub fn with_base_directory(mut self, directory: impl Into<String>) -> Self {
        self.base_directory = directory.into();
        self
    }

    /// Set the file naming pattern
    pub fn with_file_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.file_pattern = pattern.into();
        self
    }

    /// Set the file format
    pub fn with_file_format(mut self, format: FileFormat) -> Self {
        self.file_format = format;
        self
    }

    /// Generate filename for an event
    pub fn generate_filename(&self, event: &IntegrationEvent) -> String {
        let timestamp = event.timestamp;
        let event_type = &event.event_type;
        let id = &event.id;

        let filename = self.file_pattern
            .replace("{timestamp}", &timestamp.to_string())
            .replace("{event_type}", event_type)
            .replace("{id}", id);

        match self.file_format {
            FileFormat::Json => format!("{}.json", filename),
            FileFormat::Csv => format!("{}.csv", filename),
            FileFormat::Text => format!("{}.txt", filename),
        }
    }

    /// Generate full file path for an event
    pub fn generate_file_path(&self, event: &IntegrationEvent) -> std::path::PathBuf {
        let filename = self.generate_filename(event);
        std::path::Path::new(&self.base_directory).join(filename)
    }

    /// Ensure the base directory exists
    pub fn ensure_directory_exists(&self) -> Result<(), IntegrationError> {
        std::fs::create_dir_all(&self.base_directory)
            .map_err(|e| IntegrationError::IoError(format!("Failed to create directory {}: {}", self.base_directory, e)))
    }

    /// Write an event to a file
    async fn write_event_to_file(&self, event: &IntegrationEvent) -> Result<(), IntegrationError> {
        self.ensure_directory_exists()?;

        let file_path = self.generate_file_path(event);

        let content = match self.file_format {
            FileFormat::Json => {
                serde_json::to_string_pretty(event)
                    .map_err(|e| IntegrationError::SerializationError(format!("Failed to serialize event: {}", e)))?
            }
            FileFormat::Csv => {
                // Simple CSV format - in a real implementation, you'd want proper CSV handling
                format!(
                    "id,timestamp,event_type,data\n{},{},{},{}",
                    event.id,
                    event.timestamp,
                    event.event_type,
                    serde_json::to_string(&event.data)
                        .unwrap_or_else(|_| "\"\"".to_string())
                )
            }
            FileFormat::Text => {
                format!(
                    "Event ID: {}\nTimestamp: {}\nType: {}\nData: {}\n",
                    event.id,
                    event.timestamp,
                    event.event_type,
                    serde_json::to_string_pretty(&event.data)
                        .unwrap_or_else(|_| "{}".to_string())
                )
            }
        };

        tokio::fs::write(&file_path, content).await
            .map_err(|e| IntegrationError::IoError(format!("Failed to write file {}: {}", file_path.display(), e)))?;

        Ok(())
    }

    /// Read events from files (placeholder implementation)
    async fn read_events_from_files(&self) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        use tokio::fs;
        use std::path::Path;

        let mut events = Vec::new();
        let base_path = Path::new(&self.base_directory);

        if !base_path.exists() {
            return Ok(events);
        }

        let mut entries = fs::read_dir(base_path).await
            .map_err(|e| IntegrationError::IoError(format!("Failed to read directory {}: {}", self.base_directory, e)))?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| IntegrationError::IoError(format!("Failed to read directory entry: {}", e)))? {

            let path = entry.path();
            if path.is_file() {
                match self.read_event_from_file(&path).await {
                    Ok(event) => events.push(event),
                    Err(e) => {
                        // Log error but continue reading other files
                        eprintln!("Failed to read event from {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(events)
    }

    /// Read a single event from a file
    async fn read_event_from_file(&self, path: &std::path::Path) -> Result<IntegrationEvent, IntegrationError> {
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| IntegrationError::IoError(format!("Failed to read file {}: {}", path.display(), e)))?;

        match self.file_format {
            FileFormat::Json => {
                serde_json::from_str(&content)
                    .map_err(|e| IntegrationError::SerializationError(format!("Failed to parse JSON from {}: {}", path.display(), e)))
            }
            FileFormat::Csv => {
                // Simple CSV parsing - in a real implementation, you'd use a proper CSV library
                Err(IntegrationError::NotImplemented(format!("CSV reading not implemented for {}", path.display())))
            }
            FileFormat::Text => {
                Err(IntegrationError::NotImplemented(format!("Text reading not implemented for {}", path.display())))
            }
        }
    }
}

#[async_trait]
impl IntegrationAdapterTrait for FileSystemAdapter {
    fn adapter_type(&self) -> AdapterType {
        AdapterType::FileSystem
    }

    fn name(&self) -> &str {
        "File System Adapter"
    }

    fn config(&self) -> serde_json::Value {
        serde_json::json!({
            "base_directory": self.base_directory,
            "file_pattern": self.file_pattern,
            "file_format": format!("{:?}", self.file_format).to_lowercase(),
            "timeout_seconds": self.config.timeout.as_secs(),
            "max_retries": self.config.max_retries,
            "extra_config": self.config.extra_config
        })
    }

    async fn health_check(&self) -> Result<(), IntegrationError> {
        // Check if we can write to the directory
        self.ensure_directory_exists()?;

        // Try to write a test file
        let test_file = std::path::Path::new(&self.base_directory).join(".health_check");
        tokio::fs::write(&test_file, b"test").await
            .map_err(|e| IntegrationError::IoError(format!("Health check failed: {}", e)))?;

        // Clean up test file
        let _ = tokio::fs::remove_file(&test_file).await;

        Ok(())
    }

    async fn send_event(&self, event: &IntegrationEvent) -> Result<(), IntegrationError> {
        let result = self.write_event_to_file(event).await;

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
        let events = self.read_events_from_files().await?;

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
            AdapterFeature::Batching => false, // File system operations are typically individual
            AdapterFeature::RetryLogic => false, // Basic file operations don't need complex retry
            AdapterFeature::Authentication => false, // File system permissions handle access control
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::TempDir;

    #[test]
    fn test_file_system_adapter_creation() {
        let config = ConnectionConfig::default();
        let adapter = FileSystemAdapter::new(config);

        assert_eq!(adapter.adapter_type(), AdapterType::FileSystem);
        assert_eq!(adapter.name(), "File System Adapter");
        assert_eq!(adapter.base_directory, "./integration_events");
        assert_eq!(adapter.file_format, FileFormat::Json);
    }

    #[test]
    fn test_configuration_methods() {
        let config = ConnectionConfig::default();
        let adapter = FileSystemAdapter::new(config)
            .with_base_directory("/tmp/events")
            .with_file_pattern("{event_type}_{id}")
            .with_file_format(FileFormat::Csv);

        assert_eq!(adapter.base_directory, "/tmp/events");
        assert_eq!(adapter.file_pattern, "{event_type}_{id}");
        assert_eq!(adapter.file_format, FileFormat::Csv);
    }

    #[test]
    fn test_filename_generation() {
        let config = ConnectionConfig::default();
        let adapter = FileSystemAdapter::new(config).with_file_pattern("{event_type}_{id}");

        let event = IntegrationEvent {
            id: "test-123".to_string(),
            timestamp: 1640995200, // 2022-01-01 00:00:00 UTC
            event_type: "user.created".to_string(),
            data: serde_json::json!({"user_id": 123}),
            correlation_id: None,
            metadata: std::collections::HashMap::new(),
        };

        let filename = adapter.generate_filename(&event);
        assert_eq!(filename, "user.created_test-123.json");

        let adapter_csv = adapter.with_file_format(FileFormat::Csv);
        let filename_csv = adapter_csv.generate_filename(&event);
        assert_eq!(filename_csv, "user.created_test-123.csv");
    }

    #[test]
    fn test_feature_support() {
        let adapter = FileSystemAdapter::new(ConnectionConfig::default());

        assert!(adapter.supports_feature(AdapterFeature::SendEvents));
        assert!(adapter.supports_feature(AdapterFeature::ReceiveEvents));
        assert!(adapter.supports_feature(AdapterFeature::HealthCheck));
        assert!(!adapter.supports_feature(AdapterFeature::Batching));
        assert!(!adapter.supports_feature(AdapterFeature::RetryLogic));
        assert!(!adapter.supports_feature(AdapterFeature::Authentication));
    }

    #[test]
    fn test_config_serialization() {
        let config = ConnectionConfig {
            url: "file:///tmp/events".to_string(),
            timeout: Duration::from_secs(60),
            max_retries: 0,
            extra_config: serde_json::json!({"compression": true}),
        };

        let adapter = FileSystemAdapter::new(config)
            .with_base_directory("/custom/path")
            .with_file_format(FileFormat::Text);

        let config_json = adapter.config();
        assert_eq!(config_json["base_directory"], "/custom/path");
        assert_eq!(config_json["file_pattern"], "{timestamp}_{event_type}_{id}");
        assert_eq!(config_json["file_format"], "text");
        assert_eq!(config_json["timeout_seconds"], 60);
        assert_eq!(config_json["max_retries"], 0);
        assert_eq!(config_json["extra_config"]["compression"], true);
    }

    #[tokio::test]
    async fn test_directory_creation() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().join("events");

        let config = ConnectionConfig::default();
        let adapter = FileSystemAdapter::new(config)
            .with_base_directory(temp_path.to_string_lossy().to_string());

        // Directory shouldn't exist yet
        assert!(!temp_path.exists());

        // This should create the directory
        adapter.ensure_directory_exists().unwrap();

        // Now it should exist
        assert!(temp_path.exists());
        assert!(temp_path.is_dir());
    }

    #[tokio::test]
    async fn test_health_check() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().join("events");

        let config = ConnectionConfig::default();
        let adapter = FileSystemAdapter::new(config)
            .with_base_directory(temp_path.to_string_lossy().to_string());

        // Health check should pass (creates directory and test file)
        adapter.health_check().await.unwrap();

        // Directory should now exist
        assert!(temp_path.exists());
    }
}
