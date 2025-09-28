//! Configuration structures for stores and machines

use std::fmt;

/// Type alias for store identifiers
pub type StoreId = String;

/// Type alias for machine identifiers
pub type MachineId = String;

/// Type alias for state identifiers
pub type StateId = String;

/// Type alias for event identifiers
pub type EventId = String;

/// Configuration for stores and machines
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// Enable debug mode
    pub debug: bool,
    /// Enable strict mode (fail on warnings)
    pub strict: bool,
    /// Maximum number of concurrent operations
    pub max_concurrent: usize,
    /// Default timeout for operations
    pub default_timeout: std::time::Duration,
    /// Enable performance monitoring
    pub performance_monitoring: bool,
    /// Enable error reporting
    pub error_reporting: bool,
    /// Log level
    pub log_level: LogLevel,
    /// Custom configuration values
    pub custom: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            strict: false,
            max_concurrent: 10,
            default_timeout: std::time::Duration::from_secs(30),
            performance_monitoring: false,
            error_reporting: true,
            log_level: LogLevel::Info,
            custom: std::collections::HashMap::new(),
        }
    }
}

impl Config {
    /// Create a new configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable debug mode
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Enable strict mode
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Set maximum concurrent operations
    pub fn max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Set default timeout
    pub fn default_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Enable performance monitoring
    pub fn performance_monitoring(mut self, enable: bool) -> Self {
        self.performance_monitoring = enable;
        self
    }

    /// Enable error reporting
    pub fn error_reporting(mut self, enable: bool) -> Self {
        self.error_reporting = enable;
        self
    }

    /// Set log level
    pub fn log_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }

    /// Add a custom configuration value
    pub fn custom(mut self, key: String, value: serde_json::Value) -> Self {
        self.custom.insert(key, value);
        self
    }

    /// Get a custom configuration value
    pub fn get_custom<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.custom.get(key)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    /// Check if a feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.get_custom(feature).unwrap_or(false)
    }

    /// Merge with another configuration (other takes precedence)
    pub fn merge(mut self, other: &Config) -> Self {
        if other.debug { self.debug = true; }
        if other.strict { self.strict = true; }
        if other.max_concurrent > 0 { self.max_concurrent = other.max_concurrent; }
        if other.default_timeout > std::time::Duration::from_secs(0) { self.default_timeout = other.default_timeout; }
        if other.performance_monitoring { self.performance_monitoring = true; }
        if !other.error_reporting { self.error_reporting = false; }

        if other.log_level != LogLevel::Info { self.log_level = other.log_level; }

        // Merge custom values
        for (key, value) in &other.custom {
            self.custom.insert(key.clone(), value.clone());
        }

        self
    }

    /// Create a development configuration
    pub fn development() -> Self {
        Self {
            debug: true,
            strict: false,
            max_concurrent: 5,
            default_timeout: std::time::Duration::from_secs(60),
            performance_monitoring: true,
            error_reporting: true,
            log_level: LogLevel::Debug,
            custom: std::collections::HashMap::new(),
        }
    }

    /// Create a production configuration
    pub fn production() -> Self {
        Self {
            debug: false,
            strict: true,
            max_concurrent: 50,
            default_timeout: std::time::Duration::from_secs(10),
            performance_monitoring: false,
            error_reporting: true,
            log_level: LogLevel::Warn,
            custom: std::collections::HashMap::new(),
        }
    }

    /// Load configuration from a JSON string
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config: Config = serde_json::from_str(json)?;
        // Validate the configuration
        config.validate()?;
        Ok(config)
    }

    /// Save configuration to a JSON string
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.max_concurrent == 0 {
            return Err("max_concurrent must be greater than 0".into());
        }

        if self.default_timeout == std::time::Duration::from_secs(0) {
            return Err("default_timeout must be greater than 0".into());
        }

        Ok(())
    }
}

/// Log levels for debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// No logging
    Off = 0,
    /// Error messages only
    Error = 1,
    /// Warning and error messages
    Warn = 2,
    /// Info, warning, and error messages
    Info = 3,
    /// Debug, info, warning, and error messages
    Debug = 4,
    /// All messages including trace
    Trace = 5,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Off => write!(f, "off"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Trace => write!(f, "trace"),
        }
    }
}

impl LogLevel {
    /// Parse log level from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "off" => Some(LogLevel::Off),
            "error" => Some(LogLevel::Error),
            "warn" => Some(LogLevel::Warn),
            "info" => Some(LogLevel::Info),
            "debug" => Some(LogLevel::Debug),
            "trace" => Some(LogLevel::Trace),
            _ => None,
        }
    }

    /// Check if this log level should log messages at the given level
    pub fn should_log(&self, level: LogLevel) -> bool {
        *self >= level
    }
}

/// Environment-specific configuration
#[derive(Debug, Clone)]
pub enum Environment {
    /// Development environment
    Development,
    /// Testing environment
    Testing,
    /// Staging environment
    Staging,
    /// Production environment
    Production,
    /// Custom environment
    Custom(String),
}

impl Environment {
    /// Get the default configuration for this environment
    pub fn default_config(&self) -> Config {
        match self {
            Environment::Development => Config::development(),
            Environment::Testing => Config {
                debug: true,
                strict: true,
                max_concurrent: 1,
                default_timeout: std::time::Duration::from_secs(5),
                performance_monitoring: true,
                error_reporting: true,
                log_level: LogLevel::Debug,
                custom: std::collections::HashMap::new(),
            },
            Environment::Staging => Config {
                debug: false,
                strict: true,
                max_concurrent: 20,
                default_timeout: std::time::Duration::from_secs(15),
                performance_monitoring: true,
                error_reporting: true,
                log_level: LogLevel::Info,
                custom: std::collections::HashMap::new(),
            },
            Environment::Production => Config::production(),
            Environment::Custom(_) => Config::default(),
        }
    }

    /// Check if this is a development environment
    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Development)
    }

    /// Check if this is a production environment
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }

    /// Check if this is a testing environment
    pub fn is_testing(&self) -> bool {
        matches!(self, Environment::Testing)
    }
}

/// Configuration source
#[derive(Debug, Clone)]
pub enum ConfigSource {
    /// Default configuration
    Default,
    /// Configuration from file
    File(std::path::PathBuf),
    /// Configuration from environment variables
    Environment,
    /// Configuration from remote service
    Remote(String),
    /// Inline configuration
    Inline(String),
}

impl ConfigSource {
    /// Load configuration from this source
    pub async fn load(&self) -> Result<Config, Box<dyn std::error::Error>> {
        match self {
            ConfigSource::Default => Ok(Config::default()),
            ConfigSource::File(path) => {
                let content = tokio::fs::read_to_string(path).await?;
                Config::from_json(&content)
            },
            ConfigSource::Environment => {
                // Load from environment variables
                let mut config = Config::default();

                if let Ok(debug) = std::env::var("LEPTOS_STATE_DEBUG") {
                    config.debug = debug.parse().unwrap_or(false);
                }

                if let Ok(strict) = std::env::var("LEPTOS_STATE_STRICT") {
                    config.strict = strict.parse().unwrap_or(false);
                }

                if let Ok(max_concurrent) = std::env::var("LEPTOS_STATE_MAX_CONCURRENT") {
                    config.max_concurrent = max_concurrent.parse().unwrap_or(10);
                }

                if let Ok(log_level) = std::env::var("LEPTOS_STATE_LOG_LEVEL") {
                    if let Some(level) = LogLevel::from_str(&log_level) {
                        config.log_level = level;
                    }
                }

                Ok(config)
            },
            ConfigSource::Remote(url) => {
                // In a real implementation, this would fetch from a remote service
                Err(format!("Remote configuration not implemented for URL: {}", url).into())
            },
            ConfigSource::Inline(json) => Config::from_json(json),
        }
    }
}

/// Configuration manager
pub struct ConfigManager {
    /// Current configuration
    pub current: std::sync::RwLock<Config>,
    /// Configuration sources
    pub sources: Vec<ConfigSource>,
    /// Auto-reload enabled
    pub auto_reload: bool,
    /// Reload interval
    pub reload_interval: std::time::Duration,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Self {
        Self {
            current: std::sync::RwLock::new(Config::default()),
            sources: vec![ConfigSource::Default],
            auto_reload: false,
            reload_interval: std::time::Duration::from_secs(60),
        }
    }

    /// Add a configuration source
    pub fn add_source(&mut self, source: ConfigSource) {
        self.sources.push(source);
    }

    /// Load configuration from all sources
    pub async fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::default();

        for source in &self.sources {
            let source_config = source.load().await?;
            config = config.merge(&source_config);
        }

        *self.current.write().unwrap() = config;
        Ok(())
    }

    /// Get the current configuration
    pub fn get(&self) -> Config {
        self.current.read().unwrap().clone()
    }

    /// Update the configuration
    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut Config),
    {
        let mut config = self.current.write().unwrap();
        updater(&mut config);
    }

    /// Reload configuration
    pub async fn reload(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.load().await
    }

    /// Start auto-reload in the background
    pub fn start_auto_reload(&self) {
        if !self.auto_reload {
            return;
        }

        let manager = std::sync::Arc::new(self.clone());

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(manager.reload_interval);
            loop {
                interval.tick().await;
                if let Err(e) = manager.reload().await {
                    eprintln!("Failed to reload configuration: {}", e);
                }
            }
        });
    }
}

impl Clone for ConfigManager {
    fn clone(&self) -> Self {
        Self {
            current: std::sync::RwLock::new(self.current.read().unwrap().clone()),
            sources: self.sources.clone(),
            auto_reload: self.auto_reload,
            reload_interval: self.reload_interval,
        }
    }
}
