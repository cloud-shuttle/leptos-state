//! HTTP-based configuration sources

use super::traits::{ConfigSourceTrait, ConfigError, SourceMetadata};
use async_trait::async_trait;
use std::time::Duration;

/// HTTP-based configuration source
#[derive(Debug, Clone)]
pub struct HttpSource {
    /// The URL to fetch configuration from
    pub url: String,
    /// HTTP headers to include
    pub headers: std::collections::HashMap<String, String>,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Current retry count
    pub retry_count: u32,
}

impl HttpSource {
    /// Create a new HTTP source
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            headers: std::collections::HashMap::new(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_count: 0,
        }
    }

    /// Add an HTTP header
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the maximum number of retries
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Create an HTTP source with common headers for JSON APIs
    pub fn json_api(url: impl Into<String>) -> Self {
        Self::new(url)
            .with_header("Accept", "application/json")
            .with_header("Content-Type", "application/json")
    }
}

impl Default for HttpSource {
    fn default() -> Self {
        Self::new("")
    }
}

#[async_trait]
impl ConfigSourceTrait for HttpSource {
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        if self.url.trim().is_empty() {
            return Err(ConfigError::InvalidUrl("URL cannot be empty".into()));
        }

        let client = reqwest::Client::new();
        let mut request = client
            .get(&self.url)
            .timeout(self.timeout);

        // Add custom headers
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.send().await
            .map_err(|e| ConfigError::Http(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(ConfigError::Http(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let json_value: serde_json::Value = response.json().await
            .map_err(|e| ConfigError::Http(format!("JSON parsing failed: {}", e)))?;

        Ok(json_value)
    }

    async fn is_available(&self) -> bool {
        if self.url.trim().is_empty() {
            return false;
        }

        // Simple connectivity check
        let client = reqwest::Client::new();
        let request = client
            .head(&self.url)
            .timeout(Duration::from_secs(5));

        match request.send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    fn priority(&self) -> u8 {
        70 // Lower than local files, higher than databases
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.url.trim().is_empty() {
            return Err(ConfigError::InvalidUrl("URL cannot be empty".into()));
        }

        if !self.url.starts_with("http://") && !self.url.starts_with("https://") {
            return Err(ConfigError::InvalidUrl("URL must start with http:// or https://".into()));
        }

        if self.timeout.as_secs() == 0 {
            return Err(ConfigError::Validation("Timeout cannot be zero".into()));
        }

        Ok(())
    }

    fn metadata(&self) -> SourceMetadata {
        let is_secure = self.url.starts_with("https://");
        let host = self.url
            .strip_prefix("http://")
            .or_else(|| self.url.strip_prefix("https://"))
            .and_then(|s| s.split('/').next())
            .unwrap_or("unknown");

        SourceMetadata {
            name: format!("HTTP Source ({})", host),
            source_type: "http".to_string(),
            priority: self.priority(),
            description: format!(
                "Loads configuration from HTTP endpoint{}{}",
                if is_secure { " (HTTPS)" } else { " (HTTP)" },
                if !self.headers.is_empty() {
                    format!(" with {} headers", self.headers.len())
                } else {
                    "".to_string()
                }
            ),
        }
    }
}

/// Remote URL configuration source (alias for HttpSource)
pub type RemoteUrl = HttpSource;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_http_source_validation() {
        // Valid HTTPS URL
        let source = HttpSource::new("https://api.example.com/config");
        assert!(source.validate().is_ok());

        // Valid HTTP URL
        let source = HttpSource::new("http://localhost:8080/config");
        assert!(source.validate().is_ok());

        // Invalid URL - empty
        let source = HttpSource::new("");
        assert!(matches!(source.validate(), Err(ConfigError::InvalidUrl(_))));

        // Invalid URL - no protocol
        let source = HttpSource::new("api.example.com/config");
        assert!(matches!(source.validate(), Err(ConfigError::InvalidUrl(_))));

        // Invalid timeout
        let source = HttpSource::new("https://api.example.com/config")
            .with_timeout(Duration::from_secs(0));
        assert!(matches!(source.validate(), Err(ConfigError::Validation(_))));
    }

    #[test]
    fn test_http_source_metadata() {
        let source = HttpSource::new("https://api.example.com/v1/config")
            .with_header("Authorization", "Bearer token")
            .with_timeout(Duration::from_secs(10));

        let metadata = source.metadata();
        assert_eq!(metadata.source_type, "http");
        assert_eq!(metadata.priority, 70);
        assert!(metadata.description.contains("HTTPS"));
        assert!(metadata.description.contains("1 headers"));
    }

    #[test]
    fn test_json_api_constructor() {
        let source = HttpSource::json_api("https://api.example.com/config");

        assert_eq!(source.url, "https://api.example.com/config");
        assert_eq!(source.headers.get("Accept").unwrap(), "application/json");
        assert_eq!(source.headers.get("Content-Type").unwrap(), "application/json");
    }
}
