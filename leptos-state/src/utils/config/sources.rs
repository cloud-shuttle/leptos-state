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
            ConfigSource::Custom(details) => Err(format!("Custom source not implemented: {}", details)),
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
        // Simplified implementation - in practice you'd use reqwest or similar
        Err("Remote URL loading not implemented".to_string())
    }

    /// Load configuration from a database
    async fn load_from_database(&self, connection_string: &str) -> Result<Config, String> {
        // Simplified implementation - in practice you'd use a database client
        Err("Database loading not implemented".to_string())
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
