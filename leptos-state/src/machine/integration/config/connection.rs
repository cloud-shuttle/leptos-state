//! Connection configuration structures

/// Connection configuration for adapters
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionConfig {
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

    /// Set connection timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set keep-alive interval
    pub fn keep_alive(mut self, keep_alive: Option<std::time::Duration>) -> Self {
        self.keep_alive = keep_alive;
        self
    }

    /// Set maximum idle time
    pub fn max_idle_time(mut self, max_idle_time: Option<std::time::Duration>) -> Self {
        self.max_idle_time = max_idle_time;
        self
    }

    /// Set connection pool size
    pub fn pool_size(mut self, pool_size: usize) -> Self {
        self.pool_size = pool_size;
        self
    }

    /// Set credentials
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

    /// Get a parameter value
    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(key)
    }

    /// Validate the connection configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.pool_size == 0 {
            return Err("pool_size must be greater than 0".to_string());
        }

        if let Some(keep_alive) = self.keep_alive {
            if keep_alive.as_secs() == 0 {
                return Err("keep_alive must be greater than 0".to_string());
            }
        }

        if let Some(max_idle) = self.max_idle_time {
            if max_idle.as_secs() == 0 {
                return Err("max_idle_time must be greater than 0".to_string());
            }
        }

        if let Some(creds) = &self.credentials {
            creds.validate()?;
        }

        Ok(())
    }

    /// Get connection summary
    pub fn summary(&self) -> String {
        let auth_type = self.credentials.as_ref()
            .map(|c| c.auth_type())
            .unwrap_or("none");

        format!(
            "ConnectionConfig {{ timeout: {:.1}s, pool: {}, tls: {}, auth: {} }}",
            self.timeout.as_secs_f64(),
            self.pool_size,
            self.tls_enabled,
            auth_type
        )
    }

    /// Check if connection requires authentication
    pub fn requires_auth(&self) -> bool {
        self.credentials.is_some()
    }

    /// Create a secure connection config (with TLS and auth)
    pub fn secure() -> Self {
        Self::new().tls_enabled(true)
    }

    /// Create an insecure connection config (no TLS)
    pub fn insecure() -> Self {
        Self::new().tls_enabled(false)
    }

    /// Create a high-performance connection config
    pub fn high_performance() -> Self {
        Self::new()
            .pool_size(100)
            .timeout(std::time::Duration::from_secs(10))
            .keep_alive(Some(std::time::Duration::from_secs(30)))
    }
}

impl std::fmt::Display for ConnectionConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Authentication credentials
#[derive(Debug, Clone, PartialEq)]
pub enum Credentials {
    /// No authentication
    None,
    /// Basic authentication (username/password)
    Basic { username: String, password: String },
    /// Bearer token authentication
    Bearer { token: String },
    /// API key authentication
    ApiKey { key: String, header_name: String },
    /// Custom authentication
    Custom { auth_type: String, parameters: std::collections::HashMap<String, String> },
}

impl Credentials {
    /// Create basic auth credentials
    pub fn basic<S: Into<String>>(username: S, password: S) -> Self {
        Self::Basic {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Create bearer token credentials
    pub fn bearer<S: Into<String>>(token: S) -> Self {
        Self::Bearer { token: token.into() }
    }

    /// Create API key credentials
    pub fn api_key<S: Into<String>>(key: S, header_name: S) -> Self {
        Self::ApiKey {
            key: key.into(),
            header_name: header_name.into(),
        }
    }

    /// Create custom credentials
    pub fn custom<S: Into<String>>(auth_type: S, parameters: std::collections::HashMap<String, String>) -> Self {
        Self::Custom {
            auth_type: auth_type.into(),
            parameters,
        }
    }

    /// Get authentication type
    pub fn auth_type(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Basic { .. } => "basic",
            Self::Bearer { .. } => "bearer",
            Self::ApiKey { .. } => "api_key",
            Self::Custom { .. } => "custom",
        }
    }

    /// Validate credentials
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::None => Ok(()),
            Self::Basic { username, password } => {
                if username.trim().is_empty() {
                    return Err("username cannot be empty".to_string());
                }
                if password.trim().is_empty() {
                    return Err("password cannot be empty".to_string());
                }
                Ok(())
            }
            Self::Bearer { token } => {
                if token.trim().is_empty() {
                    return Err("token cannot be empty".to_string());
                }
                Ok(())
            }
            Self::ApiKey { key, header_name } => {
                if key.trim().is_empty() {
                    return Err("API key cannot be empty".to_string());
                }
                if header_name.trim().is_empty() {
                    return Err("header name cannot be empty".to_string());
                }
                Ok(())
            }
            Self::Custom { auth_type, parameters } => {
                if auth_type.trim().is_empty() {
                    return Err("auth_type cannot be empty".to_string());
                }
                if parameters.is_empty() {
                    return Err("custom auth must have parameters".to_string());
                }
                Ok(())
            }
        }
    }

    /// Check if credentials are secure (have actual authentication)
    pub fn is_secure(&self) -> bool {
        !matches!(self, Self::None)
    }

    /// Get credential summary (without sensitive data)
    pub fn summary(&self) -> String {
        match self {
            Self::None => "No authentication".to_string(),
            Self::Basic { username, .. } => format!("Basic auth for user '{}'", username),
            Self::Bearer { .. } => "Bearer token authentication".to_string(),
            Self::ApiKey { header_name, .. } => format!("API key authentication (header: {})", header_name),
            Self::Custom { auth_type, .. } => format!("Custom authentication: {}", auth_type),
        }
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Connection pool configuration
#[derive(Debug, Clone, PartialEq)]
pub struct PoolConfig {
    /// Maximum pool size
    pub max_size: usize,
    /// Minimum idle connections
    pub min_idle: usize,
    /// Maximum idle time before closing connection
    pub max_idle_time: std::time::Duration,
    /// Maximum lifetime of a connection
    pub max_lifetime: Option<std::time::Duration>,
    /// Connection acquire timeout
    pub acquire_timeout: std::time::Duration,
    /// Whether to test connections on acquire
    pub test_on_acquire: bool,
    /// Test query for connection validation
    pub test_query: Option<String>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_size: 10,
            min_idle: 1,
            max_idle_time: std::time::Duration::from_secs(300),
            max_lifetime: Some(std::time::Duration::from_secs(1800)),
            acquire_timeout: std::time::Duration::from_secs(30),
            test_on_acquire: true,
            test_query: None,
        }
    }
}

impl PoolConfig {
    /// Create a new pool config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum pool size
    pub fn max_size(mut self, size: usize) -> Self {
        self.max_size = size;
        self
    }

    /// Set minimum idle connections
    pub fn min_idle(mut self, min: usize) -> Self {
        self.min_idle = min;
        self
    }

    /// Set maximum idle time
    pub fn max_idle_time(mut self, time: std::time::Duration) -> Self {
        self.max_idle_time = time;
        self
    }

    /// Set maximum lifetime
    pub fn max_lifetime(mut self, lifetime: Option<std::time::Duration>) -> Self {
        self.max_lifetime = lifetime;
        self
    }

    /// Set acquire timeout
    pub fn acquire_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.acquire_timeout = timeout;
        self
    }

    /// Enable/disable test on acquire
    pub fn test_on_acquire(mut self, test: bool) -> Self {
        self.test_on_acquire = test;
        self
    }

    /// Set test query
    pub fn test_query<S: Into<String>>(mut self, query: S) -> Self {
        self.test_query = Some(query.into());
        self
    }

    /// Validate pool configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.max_size == 0 {
            return Err("max_size must be greater than 0".to_string());
        }

        if self.min_idle > self.max_size {
            return Err("min_idle cannot be greater than max_size".to_string());
        }

        if self.acquire_timeout.as_secs() == 0 {
            return Err("acquire_timeout must be greater than 0".to_string());
        }

        if self.max_idle_time.as_secs() == 0 {
            return Err("max_idle_time must be greater than 0".to_string());
        }

        if let Some(max_life) = self.max_lifetime {
            if max_life.as_secs() == 0 {
                return Err("max_lifetime must be greater than 0 if set".to_string());
            }
        }

        Ok(())
    }

    /// Get pool summary
    pub fn summary(&self) -> String {
        format!(
            "PoolConfig {{ max_size: {}, min_idle: {}, idle_time: {:.0}s, test_on_acquire: {} }}",
            self.max_size,
            self.min_idle,
            self.max_idle_time.as_secs_f64(),
            self.test_on_acquire
        )
    }

    /// Create high-performance pool config
    pub fn high_performance() -> Self {
        Self::new()
            .max_size(100)
            .min_idle(10)
            .max_idle_time(std::time::Duration::from_secs(60))
            .acquire_timeout(std::time::Duration::from_secs(5))
    }

    /// Create conservative pool config
    pub fn conservative() -> Self {
        Self::new()
            .max_size(5)
            .min_idle(0)
            .max_idle_time(std::time::Duration::from_secs(600))
            .acquire_timeout(std::time::Duration::from_secs(60))
    }
}

impl std::fmt::Display for PoolConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Builder for connection configuration
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

    /// Set timeout in seconds
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.config.timeout = std::time::Duration::from_secs(secs);
        self
    }

    /// Set keep alive in seconds
    pub fn keep_alive_secs(mut self, secs: Option<u64>) -> Self {
        self.config.keep_alive = secs.map(std::time::Duration::from_secs);
        self
    }

    /// Set max idle time in seconds
    pub fn max_idle_time_secs(mut self, secs: Option<u64>) -> Self {
        self.config.max_idle_time = secs.map(std::time::Duration::from_secs);
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

    /// Build the configuration
    pub fn build(self) -> ConnectionConfig {
        self.config
    }
}

impl Default for ConnectionConfigBuilder {
    fn default() -> Self {
        Self::new()
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
}
