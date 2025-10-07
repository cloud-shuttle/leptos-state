//! Environment variable configuration sources

use super::traits::{ConfigSourceTrait, ConfigError, SourceMetadata};
use async_trait::async_trait;

/// Environment variable configuration source
#[derive(Debug, Clone)]
pub struct EnvironmentSource {
    /// Optional prefix for environment variables
    pub prefix: Option<String>,
    /// Separator for nested keys (e.g., "__" for APP__DATABASE__HOST)
    pub separator: String,
    /// Whether to treat keys as case-sensitive
    pub case_sensitive: bool,
    /// Whether to include system environment variables
    pub include_system_vars: bool,
}

impl EnvironmentSource {
    /// Create a new environment source
    pub fn new() -> Self {
        Self {
            prefix: None,
            separator: "__".to_string(),
            case_sensitive: false,
            include_system_vars: true,
        }
    }

    /// Set a prefix for environment variables
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set the separator for nested keys
    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Set case sensitivity
    pub fn case_sensitive(mut self, sensitive: bool) -> Self {
        self.case_sensitive = sensitive;
        self
    }

    /// Include or exclude system environment variables
    pub fn include_system_vars(mut self, include: bool) -> Self {
        self.include_system_vars = include;
        self
    }

    /// Create an environment source with a common prefix
    pub fn prefixed(prefix: impl Into<String>) -> Self {
        Self::new().with_prefix(prefix)
    }

    /// Check if an environment variable matches the source criteria
    pub fn matches_key(&self, key: &str) -> bool {
        // Check prefix
        if let Some(ref prefix) = self.prefix {
            if !key.starts_with(prefix) {
                return false;
            }
        }

        // Check for system variables (starting with underscore or common system prefixes)
        if !self.include_system_vars {
            if key.starts_with('_') ||
               key.starts_with("HOME") ||
               key.starts_with("PATH") ||
               key.starts_with("USER") ||
               key.starts_with("SHELL") ||
               key.starts_with("PWD") ||
               key.starts_with("OLDPWD") ||
               key.starts_with("LANG") ||
               key.starts_with("LC_") {
                return false;
            }
        }

        true
    }

    /// Transform an environment variable key to a config key
    pub fn transform_key(&self, key: &str) -> String {
        let mut config_key = key.to_string();

        // Remove prefix if present
        if let Some(ref prefix) = self.prefix {
            if config_key.starts_with(prefix) {
                config_key = config_key[prefix.len()..].to_string();
                // Remove leading separator if present
                if config_key.starts_with(&self.separator) {
                    config_key = config_key[self.separator.len()..].to_string();
                }
            }
        }

        // Convert separator to dots for nested structure
        config_key = config_key.replace(&self.separator, ".");

        // Convert to lowercase if not case sensitive
        if !self.case_sensitive {
            config_key = config_key.to_lowercase();
        }

        config_key
    }
}

impl Default for EnvironmentSource {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ConfigSourceTrait for EnvironmentSource {
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        let mut config = serde_json::Map::new();

        // Collect matching environment variables
        for (key, value) in std::env::vars() {
            if self.matches_key(&key) {
                let config_key = self.transform_key(&key);
                self.insert_nested_value(&mut config, &config_key, &value);
            }
        }

        Ok(serde_json::Value::Object(config))
    }

    async fn is_available(&self) -> bool {
        // Environment variables are always available
        true
    }

    fn priority(&self) -> u8 {
        50 // Lower priority than file and HTTP sources
    }

    fn validate(&self) -> Result<(), ConfigError> {
        // Check separator is not empty
        if self.separator.trim().is_empty() {
            return Err(ConfigError::Validation("Separator cannot be empty".into()));
        }

        // Check prefix doesn't contain separator if separator is set
        if let Some(ref prefix) = self.prefix {
            if prefix.contains(&self.separator) {
                return Err(ConfigError::Validation(
                    "Prefix cannot contain the separator character".into()
                ));
            }
        }

        Ok(())
    }

    fn metadata(&self) -> SourceMetadata {
        let prefix_desc = self.prefix
            .as_ref()
            .map(|p| format!(" with prefix '{}'", p))
            .unwrap_or_default();

        SourceMetadata {
            name: "Environment Variables Source".to_string(),
            source_type: "environment".to_string(),
            priority: self.priority(),
            description: format!(
                "Loads configuration from environment variables{} (separator: '{}')",
                prefix_desc,
                self.separator
            ),
        }
    }
}

impl EnvironmentSource {
    /// Insert a value into nested JSON structure
    fn insert_nested_value(&self, config: &mut serde_json::Map<String, serde_json::Value>, key: &str, value: &str) {
        let parts: Vec<&str> = key.split('.').collect();

        let mut current_path = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            current_path.push(*part);
            if i == parts.len() - 1 {
                // Last part - insert the value
                let json_value = self.parse_value(value);
                self.insert_at_path(config, &current_path, json_value);
            } else {
                // Intermediate part - ensure nested object exists
                self.ensure_path_exists(config, &current_path);
            }
        }
    }

    /// Insert a value at the specified path
    fn insert_at_path(&self, root: &mut serde_json::Map<String, serde_json::Value>, path: &[&str], value: serde_json::Value) {
        let mut current = root;
        for (i, part) in path.iter().enumerate() {
            if i == path.len() - 1 {
                current.insert(part.to_string(), value);
                break;
            } else {
                if let Some(serde_json::Value::Object(ref mut obj)) = current.get_mut(*part) {
                    current = obj;
                } else {
                    break;
                }
            }
        }
    }

    /// Ensure a path exists in the nested structure
    fn ensure_path_exists(&self, root: &mut serde_json::Map<String, serde_json::Value>, path: &[&str]) {
        let mut current = root;
        for part in path {
            if !current.contains_key(*part) {
                current.insert(part.to_string(), serde_json::Value::Object(serde_json::Map::new()));
            }
            if let Some(serde_json::Value::Object(ref mut obj)) = current.get_mut(*part) {
                current = obj;
            } else {
                break;
            }
        }
    }

    /// Parse a string value into appropriate JSON type
    fn parse_value(&self, value: &str) -> serde_json::Value {
        // Try to parse as boolean
        if value.eq_ignore_ascii_case("true") {
            return serde_json::Value::Bool(true);
        }
        if value.eq_ignore_ascii_case("false") {
            return serde_json::Value::Bool(false);
        }

        // Try to parse as number
        if let Ok(num) = value.parse::<i64>() {
            return serde_json::Value::Number(num.into());
        }
        if let Ok(num) = value.parse::<f64>() {
            if let Some(num_val) = serde_json::Number::from_f64(num) {
                return serde_json::Value::Number(num_val);
            }
        }

        // Default to string
        serde_json::Value::String(value.to_string())
    }
}

/// Environment variable configuration source (alias for EnvironmentSource)
pub type Environment = EnvironmentSource;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_environment_source_validation() {
        // Valid source
        let source = EnvironmentSource::new()
            .with_prefix("APP")
            .with_separator("__");
        assert!(source.validate().is_ok());

        // Invalid separator
        let source = EnvironmentSource::new().with_separator("");
        assert!(matches!(source.validate(), Err(ConfigError::Validation(_))));

        // Invalid prefix with separator
        let source = EnvironmentSource::new()
            .with_prefix("APP__CONFIG")
            .with_separator("__");
        assert!(matches!(source.validate(), Err(ConfigError::Validation(_))));
    }

    #[test]
    fn test_key_transformation() {
        let source = EnvironmentSource::new()
            .with_prefix("APP")
            .with_separator("__");

        assert_eq!(source.transform_key("APP__DATABASE__HOST"), "database.host");
        assert_eq!(source.transform_key("APP_DEBUG"), "debug");
        assert_eq!(source.transform_key("OTHER_VAR"), "other_var");
    }

    #[test]
    fn test_case_sensitivity() {
        let source = EnvironmentSource::new().case_sensitive(true);
        assert_eq!(source.transform_key("APP_DATABASE_HOST"), "APP_DATABASE_HOST");

        let source = EnvironmentSource::new().case_sensitive(false);
        assert_eq!(source.transform_key("APP_DATABASE_HOST"), "app_database_host");
    }

    #[test]
    fn test_key_matching() {
        let source = EnvironmentSource::new().with_prefix("APP");

        assert!(source.matches_key("APP_DATABASE_HOST"));
        assert!(source.matches_key("APP_DEBUG"));
        assert!(!source.matches_key("DATABASE_HOST"));
        assert!(!source.matches_key("_SYSTEM_VAR"));

        let source_no_system = EnvironmentSource::new().include_system_vars(false);
        assert!(!source_no_system.matches_key("HOME"));
        assert!(!source_no_system.matches_key("_PRIVATE_VAR"));
    }

    #[test]
    fn test_value_parsing() {
        let source = EnvironmentSource::new();

        assert_eq!(source.parse_value("true"), serde_json::Value::Bool(true));
        assert_eq!(source.parse_value("false"), serde_json::Value::Bool(false));
        assert_eq!(source.parse_value("42"), serde_json::Value::Number(42.into()));
        assert_eq!(source.parse_value("3.14"), serde_json::Value::Number(3.14.into()));
        assert_eq!(source.parse_value("hello"), serde_json::Value::String("hello".into()));
    }

    #[tokio::test]
    async fn test_environment_loading() {
        // Set up test environment variables
        env::set_var("TEST_APP_NAME", "myapp");
        env::set_var("TEST_APP_VERSION", "1.0.0");
        env::set_var("TEST_DATABASE_HOST", "localhost");

        let source = EnvironmentSource::new().with_prefix("TEST");

        let result = source.load().await.unwrap();
        let config = result.as_object().unwrap();

        assert_eq!(config["app"]["name"], "myapp");
        assert_eq!(config["app"]["version"], "1.0.0");
        assert_eq!(config["database"]["host"], "localhost");

        // Clean up
        env::remove_var("TEST_APP_NAME");
        env::remove_var("TEST_APP_VERSION");
        env::remove_var("TEST_DATABASE_HOST");
    }

    #[test]
    fn test_metadata() {
        let source = EnvironmentSource::new().with_prefix("MYAPP");

        let metadata = source.metadata();
        assert_eq!(metadata.source_type, "environment");
        assert_eq!(metadata.priority, 50);
        assert!(metadata.description.contains("MYAPP"));
        assert!(metadata.description.contains("__"));
    }
}
