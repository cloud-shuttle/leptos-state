//! File-based configuration sources

use super::traits::{ConfigSourceTrait, ConfigError, SourceMetadata};
use async_trait::async_trait;
use std::path::Path;

/// File-based configuration source
#[derive(Debug, Clone)]
pub enum FileSource {
    /// JSON file configuration
    Json { path: String },
    /// TOML file configuration
    #[cfg(feature = "toml")]
    Toml { path: String },
    /// YAML file configuration
    Yaml { path: String },
    /// Auto-detect format from extension
    Auto { path: String },
}

impl FileSource {
    /// Create a JSON file source
    pub fn json(path: impl Into<String>) -> Self {
        Self::Json { path: path.into() }
    }

    /// Create a TOML file source
    #[cfg(feature = "toml")]
    pub fn toml(path: impl Into<String>) -> Self {
        Self::Toml { path: path.into() }
    }

    /// Create a YAML file source
    pub fn yaml(path: impl Into<String>) -> Self {
        Self::Yaml { path: path.into() }
    }

    /// Create an auto-detecting file source
    pub fn auto(path: impl Into<String>) -> Self {
        Self::Auto { path: path.into() }
    }

    /// Get the file path
    pub fn path(&self) -> &str {
        match self {
            Self::Json { path } => path,
            #[cfg(feature = "toml")]
            Self::Toml { path } => path,
            Self::Yaml { path } => path,
            Self::Auto { path } => path,
        }
    }

    /// Determine file format from path extension
    pub fn detect_format(path: &str) -> Result<Self, ConfigError> {
        let path_obj = Path::new(path);
        match path_obj.extension().and_then(|ext| ext.to_str()) {
            Some("json") => Ok(Self::json(path)),
            Some("toml") => {
                #[cfg(feature = "toml")]
                { Ok(Self::toml(path)) }
                #[cfg(not(feature = "toml"))]
                { Err(ConfigError::Validation("TOML support not enabled".into())) }
            }
            Some("yaml") | Some("yml") => Ok(Self::yaml(path)),
            _ => Err(ConfigError::Validation(format!("Unsupported file extension for: {}", path))),
        }
    }
}

#[async_trait]
impl ConfigSourceTrait for FileSource {
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        let path = self.path();
        if !Path::new(path).exists() {
            return Err(ConfigError::NotAvailable(format!("File not found: {}", path)));
        }

        match self {
            Self::Json { .. } => load_json_file(path).await,
            #[cfg(feature = "toml")]
            Self::Toml { .. } => load_toml_file(path).await,
            Self::Yaml { .. } => load_yaml_file(path).await,
            Self::Auto { .. } => load_auto_file(path).await,
        }
    }

    async fn is_available(&self) -> bool {
        Path::new(self.path()).exists()
    }

    fn priority(&self) -> u8 {
        90 // High priority for local files
    }

    fn validate(&self) -> Result<(), ConfigError> {
        let path = self.path();
        if path.trim().is_empty() {
            return Err(ConfigError::InvalidPath("File path cannot be empty".into()));
        }

        // Check if path contains invalid characters
        if path.contains("..") || path.contains("\\") {
            return Err(ConfigError::InvalidPath("Invalid path characters".into()));
        }

        Ok(())
    }

    fn metadata(&self) -> SourceMetadata {
        let (format, extension) = match self {
            Self::Json { .. } => ("JSON", "json"),
            #[cfg(feature = "toml")]
            Self::Toml { .. } => ("TOML", "toml"),
            Self::Yaml { .. } => ("YAML", "yaml"),
            Self::Auto { .. } => ("Auto", "auto"),
        };

        SourceMetadata {
            name: format!("{} File Source", format),
            source_type: "file".to_string(),
            priority: self.priority(),
            description: format!("Loads configuration from {} files (.{})", format.to_lowercase(), extension),
        }
    }
}

/// Load configuration from a JSON file
async fn load_json_file(path: &str) -> Result<serde_json::Value, ConfigError> {
    let content = tokio::fs::read_to_string(path).await?;
    let value: serde_json::Value = serde_json::from_str(&content)?;
    Ok(value)
}

/// Load configuration from a TOML file
#[cfg(feature = "toml")]
async fn load_toml_file(path: &str) -> Result<serde_json::Value, ConfigError> {
    let content = tokio::fs::read_to_string(path).await?;
    let value: toml::Value = toml::from_str(&content)?;
    let json_value = serde_json::to_value(value)?;
    Ok(json_value)
}

/// Load configuration from a YAML file
async fn load_yaml_file(path: &str) -> Result<serde_json::Value, ConfigError> {
    let content = tokio::fs::read_to_string(path).await?;
    let value: serde_json::Value = serde_yaml::from_str(&content)?;
    Ok(value)
}

/// Load configuration from a file with auto-detected format
async fn load_auto_file(path: &str) -> Result<serde_json::Value, ConfigError> {
    let source = FileSource::detect_format(path)?;
    source.load().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_json_file_source() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let json_content = r#"{"key": "value", "number": 42}"#;
        temp_file.write_all(json_content.as_bytes()).unwrap();
        let temp_path = temp_file.path().to_string_lossy().to_string();

        let source = FileSource::json(&temp_path);
        assert!(source.is_available().await);
        assert_eq!(source.priority(), 90);

        let result = source.load().await.unwrap();
        assert_eq!(result["key"], "value");
        assert_eq!(result["number"], 42);
    }

    #[tokio::test]
    async fn test_missing_file() {
        let source = FileSource::json("/nonexistent/file.json");
        assert!(!source.is_available().await);

        let result = source.load().await;
        assert!(matches!(result, Err(ConfigError::NotAvailable(_))));
    }
}
