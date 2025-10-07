//! Common traits for configuration sources

use async_trait::async_trait;

/// Common trait for all configuration sources
#[async_trait]
pub trait ConfigSourceTrait: Send + Sync {
    /// Load configuration from this source
    async fn load(&self) -> Result<serde_json::Value, ConfigError>;

    /// Check if this source is available
    async fn is_available(&self) -> bool;

    /// Get source priority (higher = more preferred)
    fn priority(&self) -> u8;

    /// Validate source configuration
    fn validate(&self) -> Result<(), ConfigError>;

    /// Get source metadata
    fn metadata(&self) -> SourceMetadata;
}

/// Configuration source metadata
#[derive(Debug, Clone)]
pub struct SourceMetadata {
    pub name: String,
    pub source_type: String,
    pub priority: u8,
    pub description: String,
}

/// Configuration error types
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML parsing error: {0}")]
    #[cfg(feature = "toml")]
    Toml(#[from] toml::de::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Source not available: {0}")]
    NotAvailable(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid connection: {0}")]
    InvalidConnection(String),

    #[error("Invalid name: {0}")]
    InvalidName(String),
}
