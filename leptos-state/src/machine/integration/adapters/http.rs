//! HTTP API integration adapter

use super::traits::{IntegrationAdapterTrait, AdapterType, AdapterStats, ConnectionConfig, AdapterFeature};
use super::super::events::{IntegrationEvent, IntegrationError};
use async_trait::async_trait;

/// HTTP API adapter for REST API integration
#[derive(Debug)]
pub struct HttpApiAdapter {
    /// Configuration
    pub config: ConnectionConfig,
    /// HTTP client
    pub client: reqwest::Client,
    /// Endpoint mappings for different event types
    pub endpoints: std::collections::HashMap<String, String>,
    /// Statistics
    stats: std::sync::Mutex<AdapterStats>,
}

impl HttpApiAdapter {
    /// Create a new HTTP API adapter
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            endpoints: std::collections::HashMap::new(),
            stats: std::sync::Mutex::new(AdapterStats::default()),
        }
    }

    /// Add an endpoint mapping for a specific event type
    pub fn add_endpoint(&mut self, event_type: impl Into<String>, endpoint: impl Into<String>) {
        self.endpoints.insert(event_type.into(), endpoint.into());
    }

    /// Remove an endpoint mapping
    pub fn remove_endpoint(&mut self, event_type: &str) {
        self.endpoints.remove(event_type);
    }

    /// Get endpoint for event type, falling back to base URL
    pub fn get_endpoint(&self, event_type: &str) -> String {
        self.endpoints
            .get(event_type)
            .cloned()
            .unwrap_or_else(|| self.config.url.clone())
    }

    /// Set custom headers for the HTTP client
    pub fn with_headers(mut self, headers: std::collections::HashMap<String, String>) -> Self {
        let mut client_builder = reqwest::Client::builder();

        for (key, value) in headers {
            client_builder = client_builder.default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::HeaderName::try_from(key).unwrap(),
                    reqwest::header::HeaderValue::try_from(value).unwrap(),
                );
                headers
            });
        }

        self.client = client_builder.build().unwrap_or_else(|_| reqwest::Client::new());
        self
    }

    /// Set authentication token
    pub fn with_auth_token(mut self, token: impl Into<String>) -> Self {
        let token = token.into();
        let mut headers = std::collections::HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        self.with_headers(headers)
    }
}

#[async_trait]
impl IntegrationAdapterTrait for HttpApiAdapter {
    fn adapter_type(&self) -> AdapterType {
        AdapterType::RestApi
    }

    fn name(&self) -> &str {
        "HTTP API Adapter"
    }

    fn config(&self) -> serde_json::Value {
        serde_json::json!({
            "base_url": self.config.url,
            "timeout_seconds": self.config.timeout.as_secs(),
            "max_retries": self.config.max_retries,
            "endpoints": self.endpoints,
            "extra_config": self.config.extra_config
        })
    }

    async fn health_check(&self) -> Result<(), IntegrationError> {
        // Simple health check - try to connect to base URL
        let response = self.client
            .head(&self.config.url)
            .timeout(self.config.timeout)
            .send()
            .await
            .map_err(|e| IntegrationError::ConnectionError(format!("Health check failed: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(IntegrationError::ConnectionError(format!("Health check returned status: {}", response.status())))
        }
    }

    async fn send_event(&self, event: &IntegrationEvent) -> Result<(), IntegrationError> {
        let endpoint = self.get_endpoint(&event.event_type);

        let json_payload = serde_json::to_value(event)
            .map_err(|e| IntegrationError::SerializationError(format!("Failed to serialize event: {}", e)))?;

        let mut retries = 0;
        let mut last_error = None;

        while retries <= self.config.max_retries {
            let result = self.client
                .post(&endpoint)
                .json(&json_payload)
                .timeout(self.config.timeout)
                .send()
                .await;

            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        let mut stats = self.stats.lock().unwrap();
                        stats.record_success();
                        return Ok(());
                    } else {
                        let error = IntegrationError::ConnectionError(format!(
                            "HTTP {}: {}",
                            response.status(),
                            response.text().await.unwrap_or_default()
                        ));
                        last_error = Some(error);
                    }
                }
                Err(e) => {
                    last_error = Some(IntegrationError::ConnectionError(format!("Request failed: {}", e)));
                }
            }

            retries += 1;
            if retries <= self.config.max_retries {
                // Simple exponential backoff
                tokio::time::sleep(std::time::Duration::from_millis(100 * 2u64.pow(retries - 1))).await;
            }
        }

        let mut stats = self.stats.lock().unwrap();
        stats.record_error();
        Err(last_error.unwrap_or_else(|| IntegrationError::ConnectionError("Max retries exceeded".into())))
    }

    fn stats(&self) -> AdapterStats {
        self.stats.lock().unwrap().clone()
    }

    fn supports_feature(&self, feature: AdapterFeature) -> bool {
        match feature {
            AdapterFeature::SendEvents => true,
            AdapterFeature::ReceiveEvents => false, // HTTP is typically one-way
            AdapterFeature::HealthCheck => true,
            AdapterFeature::Batching => false, // Could be implemented but not currently
            AdapterFeature::RetryLogic => true,
            AdapterFeature::Authentication => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_http_adapter_creation() {
        let config = ConnectionConfig {
            url: "https://api.example.com".to_string(),
            timeout: Duration::from_secs(10),
            max_retries: 2,
            extra_config: serde_json::Value::Null,
        };

        let adapter = HttpApiAdapter::new(config);
        assert_eq!(adapter.adapter_type(), AdapterType::RestApi);
        assert_eq!(adapter.name(), "HTTP API Adapter");
        assert!(adapter.get_endpoint("test").contains("api.example.com"));
    }

    #[test]
    fn test_endpoint_mapping() {
        let config = ConnectionConfig::default();
        let mut adapter = HttpApiAdapter::new(config);

        adapter.add_endpoint("user.created", "/api/users");
        adapter.add_endpoint("order.placed", "/api/orders");

        assert_eq!(adapter.get_endpoint("user.created"), "/api/users");
        assert_eq!(adapter.get_endpoint("order.placed"), "/api/orders");
        assert_eq!(adapter.get_endpoint("unknown.event"), ""); // falls back to base URL
    }

    #[test]
    fn test_feature_support() {
        let adapter = HttpApiAdapter::new(ConnectionConfig::default());

        assert!(adapter.supports_feature(AdapterFeature::SendEvents));
        assert!(adapter.supports_feature(AdapterFeature::HealthCheck));
        assert!(adapter.supports_feature(AdapterFeature::RetryLogic));
        assert!(adapter.supports_feature(AdapterFeature::Authentication));
        assert!(!adapter.supports_feature(AdapterFeature::ReceiveEvents));
        assert!(!adapter.supports_feature(AdapterFeature::Batching));
    }

    #[test]
    fn test_config_serialization() {
        let config = ConnectionConfig {
            url: "https://api.example.com".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            extra_config: serde_json::json!({"api_version": "v1"}),
        };

        let mut adapter = HttpApiAdapter::new(config);
        adapter.add_endpoint("test", "/test");

        let config_json = adapter.config();
        assert_eq!(config_json["base_url"], "https://api.example.com");
        assert_eq!(config_json["timeout_seconds"], 30);
        assert_eq!(config_json["max_retries"], 3);
        assert_eq!(config_json["endpoints"]["test"], "/test");
        assert_eq!(config_json["extra_config"]["api_version"], "v1");
    }
}
