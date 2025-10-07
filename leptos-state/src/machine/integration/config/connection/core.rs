//! Core connection configuration structures

use super::credentials::Credentials;
use super::pool::PoolConfig;

/// Connection configuration for adapters
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionConfig {
    /// Base URL for connections
    pub url: String,
    /// Connection timeout
    pub timeout: std::time::Duration,
    /// Keep-alive interval
    pub keep_alive: Option<std::time::Duration>,
    /// Maximum idle time
    pub max_idle_time: Option<std::time::Duration>,
    /// Connection pool size
    pub pool_size: usize,
    /// Authentication credentials
    pub credentials: Option<Credentials>,
    /// TLS configuration
    pub tls_enabled: bool,
    /// Custom connection parameters
    pub parameters: std::collections::HashMap<String, String>,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:8080".to_string(),
            timeout: std::time::Duration::from_secs(30),
            keep_alive: Some(std::time::Duration::from_secs(60)),
            max_idle_time: Some(std::time::Duration::from_secs(300)),
            pool_size: 10,
            credentials: None,
            tls_enabled: true,
            parameters: std::collections::HashMap::new(),
        }
    }
}

impl ConnectionConfig {
    /// Create a new connection config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set connection URL
    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = url.into();
        self
    }

    /// Set connection timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set keep-alive interval
    pub fn keep_alive(mut self, interval: Option<std::time::Duration>) -> Self {
        self.keep_alive = interval;
        self
    }

    /// Set maximum idle time
    pub fn max_idle_time(mut self, duration: Option<std::time::Duration>) -> Self {
        self.max_idle_time = duration;
        self
    }

    /// Set connection pool size
    pub fn pool_size(mut self, size: usize) -> Self {
        self.pool_size = size;
        self
    }

    /// Set authentication credentials
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Enable or disable TLS
    pub fn tls_enabled(mut self, enabled: bool) -> Self {
        self.tls_enabled = enabled;
        self
    }

    /// Add a custom parameter
    pub fn parameter<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }

    /// Add multiple custom parameters
    pub fn parameters<I, K, V>(mut self, params: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.parameters.extend(params.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Validate the connection configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate URL format
        if self.url.trim().is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        // Basic URL validation
        if !self.url.starts_with("http://") && !self.url.starts_with("https://") &&
           !self.url.starts_with("ws://") && !self.url.starts_with("wss://") &&
           !self.url.starts_with("tcp://") && !self.url.starts_with("unix://") {
            return Err("URL must start with a valid protocol (http, https, ws, wss, tcp, unix)".to_string());
        }

        // Validate timeout
        if self.timeout.as_secs() == 0 {
            return Err("timeout must be greater than 0".to_string());
        }

        // Validate pool size
        if self.pool_size == 0 {
            return Err("pool_size must be greater than 0".to_string());
        }

        // Validate credentials if present
        if let Some(creds) = &self.credentials {
            creds.validate()?;
        }

        // Validate custom parameters
        for (key, value) in &self.parameters {
            if key.trim().is_empty() {
                return Err("parameter keys cannot be empty".to_string());
            }
            if value.contains('\n') || value.contains('\r') {
                return Err(format!("parameter '{}' contains invalid characters", key));
            }
        }

        Ok(())
    }

    /// Get effective timeout (considering all timeout settings)
    pub fn effective_timeout(&self) -> std::time::Duration {
        self.timeout
    }

    /// Check if connection requires authentication
    pub fn requires_auth(&self) -> bool {
        self.credentials.is_some()
    }

    /// Get connection summary for debugging
    pub fn summary(&self) -> String {
        format!(
            "ConnectionConfig {{ url: {}, timeout: {:?}, pool_size: {}, tls: {}, auth: {} }}",
            self.url,
            self.timeout,
            self.pool_size,
            self.tls_enabled,
            self.requires_auth()
        )
    }

    /// Check if this is a secure connection
    pub fn is_secure(&self) -> bool {
        self.tls_enabled && (
            self.url.starts_with("https://") ||
            self.url.starts_with("wss://") ||
            self.url.starts_with("unix://") // Unix sockets are inherently secure
        )
    }

    /// Create a pool configuration based on this connection config
    pub fn create_pool_config(&self) -> PoolConfig {
        PoolConfig::new()
            .max_size(self.pool_size)
            .max_idle_time(self.max_idle_time)
            .keep_alive(self.keep_alive)
    }

    /// Create a secure connection configuration preset
    pub fn secure() -> Self {
        Self::new()
            .url("https://secure.example.com")
            .tls_enabled(true)
            .timeout(std::time::Duration::from_secs(30))
            .pool_size(10)
    }

    /// Create an insecure connection configuration preset
    pub fn insecure() -> Self {
        Self::new()
            .url("http://insecure.example.com")
            .tls_enabled(false)
            .timeout(std::time::Duration::from_secs(60))
            .pool_size(5)
    }

    /// Create a high-performance connection configuration preset
    pub fn high_performance() -> Self {
        Self::new()
            .timeout(std::time::Duration::from_secs(10))
            .pool_size(50)
            .keep_alive(Some(std::time::Duration::from_secs(30)))
            .max_idle_time(Some(std::time::Duration::from_secs(60)))
    }
}

impl std::fmt::Display for ConnectionConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_config_defaults() {
        let config = ConnectionConfig::new();
        assert_eq!(config.url, "http://localhost:8080");
        assert_eq!(config.timeout, std::time::Duration::from_secs(30));
        assert_eq!(config.pool_size, 10);
        assert!(config.tls_enabled);
        assert!(!config.requires_auth());
    }

    #[test]
    fn test_connection_config_building() {
        let config = ConnectionConfig::new()
            .url("https://api.example.com")
            .timeout(std::time::Duration::from_secs(60))
            .pool_size(20)
            .tls_enabled(true)
            .parameter("retries", "3");

        assert_eq!(config.url, "https://api.example.com");
        assert_eq!(config.timeout, std::time::Duration::from_secs(60));
        assert_eq!(config.pool_size, 20);
        assert!(config.tls_enabled);
        assert_eq!(config.parameters["retries"], "3");
    }

    #[test]
    fn test_connection_config_validation() {
        // Valid config
        let valid_config = ConnectionConfig::new().url("https://example.com");
        assert!(valid_config.validate().is_ok());

        // Invalid URL
        let invalid_url = ConnectionConfig::new().url("");
        assert!(invalid_url.validate().is_err());

        // Invalid timeout
        let invalid_timeout = ConnectionConfig::new()
            .url("https://example.com")
            .timeout(std::time::Duration::from_secs(0));
        assert!(invalid_timeout.validate().is_err());

        // Invalid pool size
        let invalid_pool = ConnectionConfig::new()
            .url("https://example.com")
            .pool_size(0);
        assert!(invalid_pool.validate().is_err());
    }

    #[test]
    fn test_connection_config_security() {
        let secure_config = ConnectionConfig::new().url("https://secure.example.com");
        assert!(secure_config.is_secure());

        let insecure_config = ConnectionConfig::new()
            .url("http://insecure.example.com")
            .tls_enabled(false);
        assert!(!insecure_config.is_secure());
    }

    #[test]
    fn test_connection_config_summary() {
        let config = ConnectionConfig::new().url("https://api.example.com");
        let summary = config.summary();
        assert!(summary.contains("https://api.example.com"));
        assert!(summary.contains("tls: true"));
        assert!(summary.contains("auth: false"));
    }
}
