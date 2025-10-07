//! Custom configuration sources

use super::traits::{ConfigSourceTrait, ConfigError, SourceMetadata};
use async_trait::async_trait;

/// Custom configuration source with user-provided loader function
#[derive(Debug)]
pub struct CustomSource<F> {
    /// The loader function that provides configuration
    pub loader: F,
    /// Source name for identification
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Source priority
    pub priority: u8,
    /// Expected configuration schema (for documentation)
    pub schema: Option<serde_json::Value>,
}

impl<F> CustomSource<F>
where
    F: Fn() -> Result<serde_json::Value, ConfigError> + Send + Sync,
{
    /// Create a new custom source
    pub fn new(name: impl Into<String>, loader: F) -> Self {
        Self {
            loader,
            name: name.into(),
            description: None,
            priority: 40, // Low priority by default
            schema: None,
        }
    }

    /// Set a description for the custom source
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the priority for this source
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Set the expected schema for this source
    pub fn with_schema(mut self, schema: serde_json::Value) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Create a custom source that loads from a static JSON string
    pub fn from_json_string(name: impl Into<String>, json_str: &'static str) -> Result<CustomSource<impl Fn() -> Result<serde_json::Value, ConfigError>>, ConfigError> {
        let loader = move || {
            serde_json::from_str(json_str)
                .map_err(|e| ConfigError::Json(e))
        };

        Ok(Self::new(name, loader))
    }

    /// Create a custom source that loads from a static value
    pub fn from_value(name: impl Into<String>, value: serde_json::Value) -> CustomSource<impl Fn() -> Result<serde_json::Value, ConfigError>> {
        let loader = move || Ok(value.clone());
        Self::new(name, loader)
    }

    /// Create a custom source that loads from a file path (similar to FileSource but with custom logic)
    pub fn from_file_path(name: impl Into<String>, path: String) -> CustomSource<impl Fn() -> Result<serde_json::Value, ConfigError>> {
        let loader = move || {
            std::fs::read_to_string(&path)
                .map_err(|e| ConfigError::Io(e))
                .and_then(|content| {
                    serde_json::from_str(&content)
                        .map_err(|e| ConfigError::Json(e))
                })
        };

        Self::new(name, loader)
    }

    /// Create a custom source with validation
    pub fn with_validation<G>(self, validator: G) -> CustomSourceWithValidation<F, G>
    where
        G: Fn(&serde_json::Value) -> Result<(), ConfigError> + Send + Sync,
    {
        CustomSourceWithValidation {
            source: self,
            validator,
        }
    }
}

#[async_trait]
impl<F> ConfigSourceTrait for CustomSource<F>
where
    F: Fn() -> Result<serde_json::Value, ConfigError> + Send + Sync,
{
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        (self.loader)()
    }

    async fn is_available(&self) -> bool {
        // Custom sources are available by definition
        // The loader function will return an error if not available
        true
    }

    fn priority(&self) -> u8 {
        self.priority
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.name.trim().is_empty() {
            return Err(ConfigError::Validation("Custom source name cannot be empty".into()));
        }

        if self.priority == 0 {
            return Err(ConfigError::Validation("Priority cannot be zero".into()));
        }

        Ok(())
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            name: self.name.clone(),
            source_type: "custom".to_string(),
            priority: self.priority,
            description: self.description.clone()
                .unwrap_or_else(|| format!("Custom configuration source '{}'", self.name)),
        }
    }
}

/// Custom source with validation
#[derive(Debug)]
pub struct CustomSourceWithValidation<F, G> {
    pub source: CustomSource<F>,
    pub validator: G,
}

#[async_trait]
impl<F, G> ConfigSourceTrait for CustomSourceWithValidation<F, G>
where
    F: Fn() -> Result<serde_json::Value, ConfigError> + Send + Sync,
    G: Fn(&serde_json::Value) -> Result<(), ConfigError> + Send + Sync,
{
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        let value = self.source.load().await?;
        (self.validator)(&value)?;
        Ok(value)
    }

    async fn is_available(&self) -> bool {
        self.source.is_available().await
    }

    fn priority(&self) -> u8 {
        self.source.priority
    }

    fn validate(&self) -> Result<(), ConfigError> {
        self.source.validate()
    }

    fn metadata(&self) -> SourceMetadata {
        let mut metadata = self.source.metadata();
        metadata.description = format!("{} (with validation)", metadata.description);
        metadata
    }
}

/// Type alias for boxed custom source (useful for dynamic dispatch)
pub type BoxedCustomSource = Box<dyn ConfigSourceTrait + Send + Sync>;

/// Create a boxed custom source from any ConfigSourceTrait implementor
pub fn boxed<T: ConfigSourceTrait + Send + Sync + 'static>(source: T) -> BoxedCustomSource {
    Box::new(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_custom_source_validation() {
        let source = CustomSource::from_value("test", json!({"key": "value"}));
        assert!(source.validate().is_ok());

        // Test empty name
        let source = CustomSource::new("", || Ok(json!({})));
        assert!(matches!(source.validate(), Err(ConfigError::Validation(_))));
    }

    #[tokio::test]
    async fn test_custom_source_from_value() {
        let test_data = json!({
            "app": {
                "name": "test-app",
                "version": "1.0.0"
            },
            "database": {
                "host": "localhost",
                "port": 5432
            }
        });

        let source = CustomSource::from_value("test-config", test_data.clone());
        let loaded = source.load().await.unwrap();
        assert_eq!(loaded, test_data);
    }

    #[tokio::test]
    async fn test_custom_source_from_json_string() {
        let json_str = r#"{"service": "api", "port": 8080}"#;
        let source = CustomSource::from_json_string("api-config", json_str).unwrap();

        let loaded = source.load().await.unwrap();
        assert_eq!(loaded["service"], "api");
        assert_eq!(loaded["port"], 8080);
    }

    #[tokio::test]
    async fn test_custom_source_with_validation() {
        let test_data = json!({"value": 42});

        let source = CustomSource::from_value("validated", test_data)
            .with_validation(|value| {
                if let Some(num) = value.get("value").and_then(|v| v.as_i64()) {
                    if num >= 0 {
                        Ok(())
                    } else {
                        Err(ConfigError::Validation("Value must be non-negative".into()))
                    }
                } else {
                    Err(ConfigError::Validation("Value must be a number".into()))
                }
            });

        // Valid value
        let loaded = source.load().await.unwrap();
        assert_eq!(loaded["value"], 42);

        // Test with invalid data
        let invalid_source = CustomSource::from_value("invalid", json!({"value": -1}))
            .with_validation(|value| {
                if let Some(num) = value.get("value").and_then(|v| v.as_i64()) {
                    if num >= 0 {
                        Ok(())
                    } else {
                        Err(ConfigError::Validation("Value must be non-negative".into()))
                    }
                } else {
                    Err(ConfigError::Validation("Value must be a number".into()))
                }
            });

        let result = invalid_source.load().await;
        assert!(matches!(result, Err(ConfigError::Validation(_))));
    }

    #[test]
    fn test_custom_source_metadata() {
        let source = CustomSource::from_value("my-custom-source", json!({}))
            .with_description("A custom configuration source")
            .with_priority(75);

        let metadata = source.metadata();
        assert_eq!(metadata.name, "my-custom-source");
        assert_eq!(metadata.source_type, "custom");
        assert_eq!(metadata.priority, 75);
        assert!(metadata.description.contains("custom configuration source"));
    }

    #[test]
    fn test_custom_source_with_schema() {
        let schema = json!({
            "type": "object",
            "properties": {
                "host": {"type": "string"},
                "port": {"type": "integer"}
            }
        });

        let source = CustomSource::from_value("schema-test", json!({}))
            .with_schema(schema);

        // The schema is stored but not currently used for validation
        // This could be extended in the future
        assert!(source.schema.is_some());
    }
}
