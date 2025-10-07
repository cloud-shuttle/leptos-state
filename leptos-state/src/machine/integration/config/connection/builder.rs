//! Connection configuration builder

use super::{ConnectionConfig, Credentials};

/// Builder for connection configuration
#[derive(Debug)]
pub struct ConnectionConfigBuilder {
    config: ConnectionConfig,
}

impl ConnectionConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: ConnectionConfig::new(),
        }
    }

    /// Set URL
    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.config.url = url.into();
        self
    }

    /// Set timeout in seconds
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.config.timeout = std::time::Duration::from_secs(secs);
        self
    }

    /// Set timeout duration
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set keep alive in seconds
    pub fn keep_alive_secs(mut self, secs: Option<u64>) -> Self {
        self.config.keep_alive = secs.map(std::time::Duration::from_secs);
        self
    }

    /// Set keep alive duration
    pub fn keep_alive(mut self, interval: Option<std::time::Duration>) -> Self {
        self.config.keep_alive = interval;
        self
    }

    /// Set max idle time in seconds
    pub fn max_idle_time_secs(mut self, secs: Option<u64>) -> Self {
        self.config.max_idle_time = secs.map(std::time::Duration::from_secs);
        self
    }

    /// Set max idle time duration
    pub fn max_idle_time(mut self, duration: Option<std::time::Duration>) -> Self {
        self.config.max_idle_time = duration;
        self
    }

    /// Set pool size
    pub fn pool_size(mut self, size: usize) -> Self {
        self.config.pool_size = size;
        self
    }

    /// Set basic auth
    pub fn basic_auth<S: Into<String>>(mut self, username: S, password: S) -> Self {
        self.config.credentials = Some(Credentials::basic(username, password));
        self
    }

    /// Set bearer token
    pub fn bearer_token<S: Into<String>>(mut self, token: S) -> Self {
        self.config.credentials = Some(Credentials::bearer(token));
        self
    }

    /// Set API key
    pub fn api_key<S: Into<String>>(mut self, key: S, header: S) -> Self {
        self.config.credentials = Some(Credentials::api_key(key, header));
        self
    }

    /// Set custom credentials
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.config.credentials = Some(credentials);
        self
    }

    /// Enable TLS
    pub fn with_tls(mut self) -> Self {
        self.config.tls_enabled = true;
        self
    }

    /// Disable TLS
    pub fn without_tls(mut self) -> Self {
        self.config.tls_enabled = false;
        self
    }

    /// Add parameter
    pub fn parameter<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.config.parameters.insert(key.into(), value.into());
        self
    }

    /// Add multiple parameters
    pub fn parameters<I, K, V>(mut self, params: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.config.parameters.extend(params.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Build the configuration
    pub fn build(self) -> ConnectionConfig {
        self.config
    }

    /// Build and validate the configuration
    pub fn build_validated(self) -> Result<ConnectionConfig, String> {
        let config = self.build();
        config.validate()?;
        Ok(config)
    }
}

impl Default for ConnectionConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ConnectionConfig> for ConnectionConfigBuilder {
    fn from(config: ConnectionConfig) -> Self {
        Self { config }
    }
}

/// Convenience functions for connection configurations
pub mod factories {
    use super::*;

    /// Create default connection configuration
    pub fn default() -> ConnectionConfig {
        ConnectionConfig::default()
    }

    /// Create secure connection configuration
    pub fn secure() -> ConnectionConfig {
        ConnectionConfig::secure()
    }

    /// Create insecure connection configuration
    pub fn insecure() -> ConnectionConfig {
        ConnectionConfig::insecure()
    }

    /// Create high-performance connection configuration
    pub fn high_performance() -> ConnectionConfig {
        ConnectionConfig::high_performance()
    }

    /// Create custom connection configuration
    pub fn custom<F>(f: F) -> ConnectionConfig
    where
        F: FnOnce(ConnectionConfigBuilder) -> ConnectionConfigBuilder,
    {
        let builder = ConnectionConfigBuilder::new();
        f(builder).build()
    }

    /// Create database connection configuration
    pub fn database<S: Into<String>>(url: S) -> ConnectionConfig {
        ConnectionConfigBuilder::new()
            .url(url)
            .pool_size(20)
            .timeout_secs(60)
            .max_idle_time_secs(Some(300))
            .build()
    }

    /// Create API client configuration
    pub fn api_client<S: Into<String>>(base_url: S) -> ConnectionConfig {
        ConnectionConfigBuilder::new()
            .url(base_url)
            .timeout_secs(30)
            .keep_alive_secs(Some(60))
            .pool_size(10)
            .with_tls()
            .build()
    }

    /// Create WebSocket configuration
    pub fn websocket<S: Into<String>>(url: S) -> ConnectionConfig {
        ConnectionConfigBuilder::new()
            .url(url)
            .timeout_secs(10)
            .keep_alive_secs(Some(30))
            .pool_size(5)
            .with_tls()
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = ConnectionConfigBuilder::new();
        let config = builder.build();

        assert_eq!(config.url, "http://localhost:8080");
        assert_eq!(config.pool_size, 10);
        assert!(config.tls_enabled);
    }

    #[test]
    fn test_builder_fluent_api() {
        let config = ConnectionConfigBuilder::new()
            .url("https://api.example.com")
            .timeout_secs(60)
            .pool_size(20)
            .basic_auth("user", "pass")
            .without_tls()
            .parameter("retries", "3")
            .build();

        assert_eq!(config.url, "https://api.example.com");
        assert_eq!(config.timeout, std::time::Duration::from_secs(60));
        assert_eq!(config.pool_size, 20);
        assert!(!config.tls_enabled);
        assert_eq!(config.parameters["retries"], "3");
        assert!(config.credentials.is_some());
    }

    #[test]
    fn test_builder_validation() {
        let result = ConnectionConfigBuilder::new()
            .url("https://example.com")
            .pool_size(0) // Invalid
            .build_validated();

        assert!(result.is_err());
    }

    #[test]
    fn test_factories() {
        let default = factories::default();
        assert_eq!(default.url, "http://localhost:8080");

        let secure = factories::secure();
        assert!(secure.is_secure());

        let insecure = factories::insecure();
        assert!(!insecure.is_secure());

        let high_perf = factories::high_performance();
        assert_eq!(high_perf.pool_size, 50); // Assuming this is set in core

        let database = factories::database("postgresql://localhost/db");
        assert!(database.url.contains("postgresql"));
        assert_eq!(database.pool_size, 20);

        let api = factories::api_client("https://api.example.com");
        assert!(api.url.contains("api.example.com"));
        assert!(api.is_secure());

        let ws = factories::websocket("wss://ws.example.com");
        assert!(ws.url.contains("ws.example.com"));
        assert_eq!(ws.pool_size, 5);
    }

    #[test]
    fn test_custom_factory() {
        let config = factories::custom(|builder| {
            builder
                .url("https://custom.example.com")
                .timeout_secs(120)
                .bearer_token("custom-token")
                .parameter("version", "v2")
        });

        assert_eq!(config.url, "https://custom.example.com");
        assert_eq!(config.timeout, std::time::Duration::from_secs(120));
        assert_eq!(config.parameters["version"], "v2");
        assert!(config.credentials.is_some());
    }
}
