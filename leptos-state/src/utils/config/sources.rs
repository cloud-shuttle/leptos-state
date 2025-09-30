//! Configuration sources and loading

use super::core::Config;

/// Configuration source
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConfigSource {
    /// Configuration from environment variables
    Environment,
    /// Configuration from a JSON file
    JsonFile(String),
    /// Configuration from a TOML file
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
            Self::TomlFile(path) => Some(path),
            Self::RemoteUrl(url) => Some(url),
            Self::Database(conn) => Some(conn),
            Self::Custom(details) => Some(details),
            _ => None,
        }
    }

    /// Check if the source requires network access
    pub fn requires_network(&self) -> bool {
        matches!(self, Self::RemoteUrl(_))
    }

    /// Check if the source requires file system access
    pub fn requires_filesystem(&self) -> bool {
        matches!(self, Self::JsonFile(_) | Self::TomlFile(_))
    }

    /// Check if the source is dynamic (can change at runtime)
    pub fn is_dynamic(&self) -> bool {
        matches!(self, Self::Database(_) | Self::RemoteUrl(_))
    }

    /// Check if the source is static (loaded once)
    pub fn is_static(&self) -> bool {
        !self.is_dynamic()
    }

    /// Get priority order (higher numbers = higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            Self::CommandLine => 100,       // Highest priority
            Self::Environment => 90,
            Self::JsonFile(_) => 80,
            Self::TomlFile(_) => 80,
            Self::RemoteUrl(_) => 70,
            Self::Database(_) => 60,
            Self::InMemory => 50,
            Self::Custom(_) => 40,         // Lowest priority
        }
    }

    /// Validate the source configuration
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::JsonFile(path) | Self::TomlFile(path) => {
                if path.trim().is_empty() {
                    return Err("File path cannot be empty".to_string());
                }
                // Could add more validation here (check if path exists, etc.)
            }
            Self::RemoteUrl(url) => {
                if url.trim().is_empty() {
                    return Err("URL cannot be empty".to_string());
                }
                if !url.starts_with("http://") && !url.starts_with("https://") {
                    return Err("URL must start with http:// or https://".to_string());
                }
            }
            Self::Database(conn) => {
                if conn.trim().is_empty() {
                    return Err("Database connection string cannot be empty".to_string());
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl std::fmt::Display for ConfigSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = self.location() {
            write!(f, "{} ({})", self.as_str(), location)
        } else {
            write!(f, "{}", self.as_str())
        }
    }
}

impl Default for ConfigSource {
    fn default() -> Self {
        Self::InMemory
    }
}

/// Configuration loader for multiple sources
pub struct ConfigLoader {
    /// Configuration sources in priority order (highest first)
    sources: Vec<ConfigSource>,
    /// Cache for loaded configurations
    cache: std::collections::HashMap<ConfigSource, Config>,
    /// Enable caching
    caching_enabled: bool,
}

impl ConfigLoader {
    /// Create a new configuration loader
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            cache: std::collections::HashMap::new(),
            caching_enabled: true,
        }
    }

    /// Add a configuration source
    pub fn add_source(mut self, source: ConfigSource) -> Self {
        // Validate the source
        if let Err(error) = source.validate() {
            eprintln!("Warning: Invalid config source {}: {}", source, error);
            return self;
        }

        self.sources.push(source);
        // Sort by priority (highest first)
        self.sources.sort_by(|a, b| b.priority().cmp(&a.priority()));
        self
    }

    /// Enable or disable caching
    pub fn with_caching(mut self, enabled: bool) -> Self {
        self.caching_enabled = enabled;
        self
    }

    /// Load configuration from all sources
    pub async fn load_config(&mut self) -> Result<Config, Vec<String>> {
        let mut final_config = Config::default();
        let mut errors = Vec::new();

        for source in &self.sources {
            match self.load_from_source(source).await {
                Ok(config) => {
                    final_config.merge(&config);
                }
                Err(error) => {
                    errors.push(format!("Failed to load from {}: {}", source, error));
                }
            }
        }

        if errors.is_empty() {
            Ok(final_config)
        } else {
            Err(errors)
        }
    }

    /// Load configuration from a specific source
    async fn load_from_source(&mut self, source: &ConfigSource) -> Result<Config, String> {
        // Check cache first
        if self.caching_enabled && source.is_static() {
            if let Some(cached) = self.cache.get(source) {
                return Ok(cached.clone());
            }
        }

        let config = match source {
            ConfigSource::Environment => self.load_from_environment(),
            ConfigSource::JsonFile(path) => self.load_from_json_file(path).await,
            ConfigSource::TomlFile(path) => self.load_from_toml_file(path).await,
            ConfigSource::CommandLine => self.load_from_command_line(),
            ConfigSource::RemoteUrl(url) => self.load_from_remote_url(url).await,
            ConfigSource::Database(conn) => self.load_from_database(conn).await,
            ConfigSource::InMemory => Ok(Config::default()),
            ConfigSource::Custom(details) => self.load_from_custom_source(details).await,
        }?;

        // Cache the result
        if self.caching_enabled && source.is_static() {
            self.cache.insert(source.clone(), config.clone());
        }

        Ok(config)
    }

    /// Load configuration from environment variables
    fn load_from_environment(&self) -> Result<Config, String> {
        Ok(Config::from_env_vars())
    }

    /// Load configuration from a JSON file
    async fn load_from_json_file(&self, path: &str) -> Result<Config, String> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| format!("Failed to read JSON file: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Load configuration from a TOML file
    async fn load_from_toml_file(&self, path: &str) -> Result<Config, String> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| format!("Failed to read TOML file: {}", e))?;

        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse TOML: {}", e))
    }

    /// Load configuration from command line arguments
    fn load_from_command_line(&self) -> Result<Config, String> {
        // Simplified implementation - in practice you'd use clap or similar
        Ok(Config::default())
    }

    /// Load configuration from a remote URL
    async fn load_from_remote_url(&self, url: &str) -> Result<Config, String> {
        // Basic HTTP GET implementation using standard library
        // In a real application, you'd use a proper HTTP client like reqwest

        #[cfg(feature = "http_client")]
        {
            // If reqwest or similar is available, use it here
            // For now, we'll return an error indicating the feature is needed
            Err("HTTP client not available - enable http_client feature".to_string())
        }

        #[cfg(not(feature = "http_client"))]
        {
            // Fallback: try to load from local file if URL looks like a file path
            if url.starts_with("file://") {
                let path = url.trim_start_matches("file://");
                self.load_from_json_file(path).await
            } else {
                // For HTTP URLs, return a helpful error
                Err(format!("Remote URL loading requires http_client feature. URL: {}", url))
            }
        }
    }

    /// Load configuration from a database
    async fn load_from_database(&self, connection_string: &str) -> Result<Config, String> {
        // Parse connection string to determine database type
        // Format: type://user:pass@host:port/database?params

        let scheme_end = connection_string.find("://")
            .ok_or_else(|| "Invalid connection string format - missing '://'".to_string())?;
        let scheme = &connection_string[..scheme_end];

        match scheme {
            "sqlite" => {
                // SQLite database
                #[cfg(feature = "sqlite")]
                {
                    self.load_from_sqlite(connection_string).await
                }
                #[cfg(not(feature = "sqlite"))]
                {
                    Err("SQLite support not enabled - enable sqlite feature".to_string())
                }
            }
            "postgres" | "postgresql" => {
                // PostgreSQL database
                #[cfg(feature = "postgres")]
                {
                    self.load_from_postgres(connection_string).await
                }
                #[cfg(not(feature = "postgres"))]
                {
                    Err("PostgreSQL support not enabled - enable postgres feature".to_string())
                }
            }
            "mysql" => {
                // MySQL database
                #[cfg(feature = "mysql")]
                {
                    self.load_from_mysql(connection_string).await
                }
                #[cfg(not(feature = "mysql"))]
                {
                    Err("MySQL support not enabled - enable mysql feature".to_string())
                }
            }
            "redis" => {
                // Redis (key-value store)
                #[cfg(feature = "redis")]
                {
                    self.load_from_redis(connection_string).await
                }
                #[cfg(not(feature = "redis"))]
                {
                    Err("Redis support not enabled - enable redis feature".to_string())
                }
            }
            _ => Err(format!("Unsupported database type: {}", scheme)),
        }
    }

    /// Load configuration from SQLite
    #[cfg(feature = "sqlite")]
    async fn load_from_sqlite(&self, connection_string: &str) -> Result<Config, String> {
        // Implementation would use rusqlite or similar
        Err("SQLite config loading not yet implemented".to_string())
    }

    /// Load configuration from PostgreSQL
    #[cfg(feature = "postgres")]
    async fn load_from_postgres(&self, connection_string: &str) -> Result<Config, String> {
        // Implementation would use tokio-postgres or similar
        Err("PostgreSQL config loading not yet implemented".to_string())
    }

    /// Load configuration from MySQL
    #[cfg(feature = "mysql")]
    async fn load_from_mysql(&self, connection_string: &str) -> Result<Config, String> {
        // Implementation would use mysql_async or similar
        Err("MySQL config loading not yet implemented".to_string())
    }

    /// Load configuration from Redis
    #[cfg(feature = "redis")]
    async fn load_from_redis(&self, connection_string: &str) -> Result<Config, String> {
        // Implementation would use redis crate
        Err("Redis config loading not yet implemented".to_string())
    }

    /// Load configuration from a custom source
    async fn load_from_custom_source(&self, details: &str) -> Result<Config, String> {
        // Parse custom source format: "format:path" or "format://data"
        // Examples:
        //   "env:MYAPP_*" - load environment variables with prefix
        //   "inline:debug=true,strict=false" - inline key-value pairs
        //   "yaml:/path/to/config.yaml" - YAML file
        //   "ini:/path/to/config.ini" - INI file

        let colon_pos = details.find(':')
            .ok_or_else(|| format!("Invalid custom source format: {}", details))?;

        let format = &details[..colon_pos];
        let data = &details[colon_pos + 1..];

        match format {
            "env" => self.load_from_env_pattern(data).await,
            "inline" => self.load_from_inline_kv(data).await,
            "yaml" => self.load_from_yaml_file(data).await,
            "yml" => self.load_from_yaml_file(data).await,
            "ini" => self.load_from_ini_file(data).await,
            "toml" => self.load_from_toml_file(data).await,
            "json" => self.load_from_json_file(data).await,
            _ => Err(format!("Unsupported custom format: {}", format)),
        }
    }

    /// Load environment variables matching a pattern
    async fn load_from_env_pattern(&self, pattern: &str) -> Result<Config, String> {
        let mut config = Config::default();

        // If pattern ends with *, treat it as a prefix
        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            for (key, value) in std::env::vars() {
                if key.starts_with(prefix) {
                    let config_key = key.trim_start_matches(prefix).to_lowercase();
                    if let Ok(json_value) = serde_json::from_str(&value) {
                        config.custom.insert(config_key, json_value);
                    } else {
                        config.custom.insert(config_key, serde_json::Value::String(value));
                    }
                }
            }
        } else {
            // Exact match
            if let Ok(value) = std::env::var(pattern) {
                if let Ok(json_value) = serde_json::from_str(&value) {
                    config.custom.insert(pattern.to_string(), json_value);
                } else {
                    config.custom.insert(pattern.to_string(), serde_json::Value::String(value));
                }
            }
        }

        Ok(config)
    }

    /// Load from inline key-value pairs
    async fn load_from_inline_kv(&self, data: &str) -> Result<Config, String> {
        let mut config = Config::default();

        for pair in data.split(',') {
            let pair = pair.trim();
            if let Some(eq_pos) = pair.find('=') {
                let key = pair[..eq_pos].trim();
                let value_str = pair[eq_pos + 1..].trim();

                if let Ok(json_value) = serde_json::from_str(value_str) {
                    config.custom.insert(key.to_string(), json_value);
                } else {
                    config.custom.insert(key.to_string(), serde_json::Value::String(value_str.to_string()));
                }
            }
        }

        Ok(config)
    }

    /// Load from YAML file
    async fn load_from_yaml_file(&self, path: &str) -> Result<Config, String> {
        #[cfg(feature = "yaml")]
        {
            let content = tokio::fs::read_to_string(path)
                .await
                .map_err(|e| format!("Failed to read YAML file: {}", e))?;

            serde_yaml::from_str(&content)
                .map_err(|e| format!("Failed to parse YAML: {}", e))
        }

        #[cfg(not(feature = "yaml"))]
        {
            Err("YAML support not enabled - enable yaml feature".to_string())
        }
    }

    /// Load from INI file
    async fn load_from_ini_file(&self, path: &str) -> Result<Config, String> {
        #[cfg(feature = "ini")]
        {
            let content = tokio::fs::read_to_string(path)
                .await
                .map_err(|e| format!("Failed to read INI file: {}", e))?;

            let ini_config: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
                ini::Ini::load_from_str(&content)
                    .map_err(|e| format!("Failed to parse INI: {}", e))?
                    .into_iter()
                    .collect();

            // Convert to Config (simplified - assumes default section)
            let mut config = Config::default();
            if let Some(default_section) = ini_config.get("") {
                for (key, value) in default_section {
                    if let Ok(json_value) = serde_json::from_str(value) {
                        config.custom.insert(key.clone(), json_value);
                    } else {
                        config.custom.insert(key.clone(), serde_json::Value::String(value.clone()));
                    }
                }
            }

            Ok(config)
        }

        #[cfg(not(feature = "ini"))]
        {
            Err("INI support not enabled - enable ini feature".to_string())
        }
    }

    /// Get all configured sources
    pub fn sources(&self) -> &[ConfigSource] {
        &self.sources
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.cache.len(), self.sources.len())
    }

    /// Create a loader with default sources
    pub fn with_defaults() -> Self {
        Self::new()
            .add_source(ConfigSource::Environment)
            .add_source(ConfigSource::JsonFile("config.json".to_string()))
    }

    /// Create a loader for development
    pub fn for_development() -> Self {
        Self::new()
            .add_source(ConfigSource::Environment)
            .add_source(ConfigSource::JsonFile("config.development.json".to_string()))
            .add_source(ConfigSource::JsonFile("config.json".to_string()))
    }

    /// Create a loader for production
    pub fn for_production() -> Self {
        Self::new()
            .add_source(ConfigSource::Environment)
            .add_source(ConfigSource::JsonFile("config.production.json".to_string()))
            .add_source(ConfigSource::JsonFile("config.json".to_string()))
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl std::fmt::Display for ConfigLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigLoader({} sources)", self.sources.len())
    }
}

/// Configuration builder for complex setups
pub struct ConfigBuilder {
    loader: ConfigLoader,
    environment: super::environment::Environment,
    base_config: Config,
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            loader: ConfigLoader::new(),
            environment: super::environment::Environment::Development,
            base_config: Config::default(),
        }
    }

    /// Set the environment
    pub fn environment(mut self, env: super::environment::Environment) -> Self {
        self.environment = env;
        self.base_config = env.default_config();
        self
    }

    /// Add a configuration source
    pub fn add_source(mut self, source: ConfigSource) -> Self {
        self.loader = self.loader.add_source(source);
        self
    }

    /// Set base configuration
    pub fn base_config(mut self, config: Config) -> Self {
        self.base_config = config;
        self
    }

    /// Build the final configuration
    pub async fn build(mut self) -> Result<Config, Vec<String>> {
        let mut final_config = self.base_config;

        // Load from sources
        let loaded_config = self.loader.load_config().await?;
        final_config.merge(&loaded_config);

        // Validate
        final_config.validate().map_err(|errors| {
            vec![format!("Configuration validation failed: {}", errors.join(", "))]
        })?;

        Ok(final_config)
    }

    /// Build with validation disabled
    pub async fn build_unchecked(mut self) -> Result<Config, Vec<String>> {
        let mut final_config = self.base_config;

        // Load from sources (ignore errors)
        if let Ok(loaded_config) = self.loader.load_config().await {
            final_config.merge(&loaded_config);
        }

        Ok(final_config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
