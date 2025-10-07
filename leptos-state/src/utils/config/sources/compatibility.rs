//! Compatibility layer for legacy ConfigSource, ConfigLoader, and ConfigBuilder

use super::traits::{ConfigSourceTrait, ConfigError, SourceMetadata};
use super::{FileSource, HttpSource, DatabaseSource, EnvironmentSource};
use async_trait::async_trait;
use std::collections::HashMap;

/// Legacy ConfigSource enum for backward compatibility
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConfigSource {
    /// Configuration from environment variables
    Environment,
    /// Configuration from a JSON file
    JsonFile(String),
    /// Configuration from a TOML file
    #[cfg(feature = "toml")]
    TomlFile(String),
    /// Configuration from command line arguments
    CommandLine,
    /// Configuration from a remote URL
    RemoteUrl(String),
    /// Configuration from a database
    Database(String),
    /// In-memory configuration
    InMemory,
    /// Custom configuration source
    Custom(String),
}

impl ConfigSource {
    /// Get the source type as a string
    pub fn as_str(&self) -> &str {
        match self {
            Self::Environment => "environment",
            Self::JsonFile(_) => "json_file",
            #[cfg(feature = "toml")]
            Self::TomlFile(_) => "toml_file",
            Self::CommandLine => "command_line",
            Self::RemoteUrl(_) => "remote_url",
            Self::Database(_) => "database",
            Self::InMemory => "in_memory",
            Self::Custom(_) => "custom",
        }
    }

    /// Get the source location/details
    pub fn location(&self) -> Option<&str> {
        match self {
            Self::JsonFile(path) => Some(path),
            #[cfg(feature = "toml")]
            Self::TomlFile(path) => Some(path),
            Self::RemoteUrl(url) => Some(url),
            Self::Database(conn) => Some(conn),
            Self::Custom(data) => Some(data),
            _ => None,
        }
    }
}

impl std::fmt::Display for ConfigSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.as_str(), self.location().unwrap_or("default"))
    }
}

impl Default for ConfigSource {
    fn default() -> Self {
        Self::Environment
    }
}

/// Convert legacy ConfigSource to new trait-based sources
impl ConfigSource {
    pub fn to_trait_source(&self) -> Result<Box<dyn ConfigSourceTrait + Send + Sync>, ConfigError> {
        match self {
            ConfigSource::Environment => {
                Ok(Box::new(EnvironmentSource::new()))
            }
            ConfigSource::JsonFile(path) => {
                Ok(Box::new(FileSource::json(path)))
            }
            #[cfg(feature = "toml")]
            ConfigSource::TomlFile(path) => {
                Ok(Box::new(FileSource::toml(path)))
            }
            ConfigSource::RemoteUrl(url) => {
                Ok(Box::new(HttpSource::new(url)))
            }
            ConfigSource::Database(conn) => {
                Ok(Box::new(DatabaseSource::new(conn)))
            }
            ConfigSource::InMemory => {
                // For in-memory, we'll create a custom source with empty config
                Ok(super::custom::boxed(
                    super::CustomSource::from_value("in-memory", serde_json::Value::Object(serde_json::Map::new()))
                ))
            }
            ConfigSource::Custom(data) => {
                // Try to parse as JSON, otherwise treat as a simple string value
                match serde_json::from_str(data) {
                    Ok(value) => Ok(super::custom::boxed(
                        super::CustomSource::from_value("custom", value)
                    )),
                    Err(_) => Ok(super::custom::boxed(
                        super::CustomSource::from_value("custom", serde_json::Value::String(data.clone()))
                    )),
                }
            }
            ConfigSource::CommandLine => {
                // Command line args would need special handling
                Err(ConfigError::Validation("CommandLine source not yet implemented in new system".into()))
            }
        }
    }
}

/// Legacy ConfigLoader for backward compatibility
#[derive(Debug)]
pub struct ConfigLoader {
    sources: Vec<ConfigSource>,
}

impl ConfigLoader {
    /// Create a new config loader
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    /// Add a configuration source
    pub fn add_source(mut self, source: ConfigSource) -> Self {
        self.sources.push(source);
        self
    }

    /// Load configuration from all sources
    pub async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        let mut merged_config = serde_json::Value::Object(serde_json::Map::new());

        for source in &self.sources {
            let source_config = source.to_trait_source()?.load().await?;
            if let Some(obj) = source_config.as_object() {
                merge_json(&mut merged_config, obj);
            }
        }

        Ok(merged_config)
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ConfigLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigLoader with {} sources", self.sources.len())
    }
}

/// Legacy ConfigBuilder for backward compatibility
#[derive(Debug)]
pub struct ConfigBuilder {
    loader: ConfigLoader,
}

impl ConfigBuilder {
    /// Create a new config builder
    pub fn new() -> Self {
        Self {
            loader: ConfigLoader::new(),
        }
    }

    /// Add a configuration source
    pub fn add_source(mut self, source: ConfigSource) -> Self {
        self.loader = self.loader.add_source(source);
        self
    }

    /// Build the configuration
    pub async fn build(self) -> Result<serde_json::Value, ConfigError> {
        self.loader.load().await
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ConfigBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigBuilder ({})", self.loader)
    }
}

/// Merge two JSON objects (simple implementation)
fn merge_json(target: &mut serde_json::Map<String, serde_json::Value>, source: &serde_json::Map<String, serde_json::Value>) {
    for (key, value) in source {
        if target.contains_key(key) {
            // If both are objects, merge recursively
            if let (Some(serde_json::Value::Object(ref mut target_child)), serde_json::Value::Object(ref source_child)) =
                (target.get_mut(key), value) {
                merge_json(target_child, source_child);
            } else {
                // Otherwise, source overwrites target
                target.insert(key.clone(), value.clone());
            }
        } else {
            target.insert(key.clone(), value.clone());
        }
    }
}
