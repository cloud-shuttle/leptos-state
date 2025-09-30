//! Core configuration structures and functionality

use super::logging::LogLevel;

/// Configuration for stores and machines
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    /// Create a new configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable debug mode
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Enable strict mode
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Set maximum concurrent operations
    pub fn with_max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Set default timeout
    pub fn with_default_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Enable performance monitoring
    pub fn with_performance_monitoring(mut self, enable: bool) -> Self {
        self.performance_monitoring = enable;
        self
    }

    /// Enable error reporting
    pub fn with_error_reporting(mut self, enable: bool) -> Self {
        self.error_reporting = enable;
        self
    }

    /// Set log level
    pub fn with_log_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }

    /// Add a custom configuration value
    pub fn with_custom<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }

    /// Get a custom configuration value
    pub fn get_custom(&self, key: &str) -> Option<&serde_json::Value> {
        self.custom.get(key)
    }

    /// Check if a custom configuration value exists
    pub fn has_custom(&self, key: &str) -> bool {
        self.custom.contains_key(key)
    }

    /// Remove a custom configuration value
    pub fn remove_custom(&mut self, key: &str) -> Option<serde_json::Value> {
        self.custom.remove(key)
    }

    /// Clear all custom configuration values
    pub fn clear_custom(&mut self) {
        self.custom.clear();
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.max_concurrent == 0 {
            errors.push("max_concurrent must be greater than 0".to_string());
        }

        if self.max_concurrent > 1000 {
            errors.push("max_concurrent seems too high (> 1000)".to_string());
        }

        if self.default_timeout.as_secs() == 0 {
            errors.push("default_timeout must be greater than 0".to_string());
        }

        if self.default_timeout.as_secs() > 3600 {
            errors.push("default_timeout seems too high (> 1 hour)".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Check if the configuration is valid
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Create a development configuration
    pub fn development() -> Self {
        Self::new()
            .with_debug(true)
            .with_strict(false)
            .with_performance_monitoring(true)
            .with_log_level(LogLevel::Debug)
    }

    /// Create a production configuration
    pub fn production() -> Self {
        Self::new()
            .with_debug(false)
            .with_strict(true)
            .with_performance_monitoring(false)
            .with_log_level(LogLevel::Warn)
    }

    /// Create a test configuration
    pub fn test() -> Self {
        Self::new()
            .with_debug(true)
            .with_strict(true)
            .with_max_concurrent(1)
            .with_log_level(LogLevel::Debug)
    }

    /// Merge with another configuration (self takes precedence)
    pub fn merge(&mut self, other: &Config) {
        if other.debug && !self.debug {
            self.debug = true;
        }

        if other.strict {
            self.strict = true;
        }

        if other.max_concurrent > self.max_concurrent {
            self.max_concurrent = other.max_concurrent;
        }

        if other.default_timeout > self.default_timeout {
            self.default_timeout = other.default_timeout;
        }

        if other.performance_monitoring {
            self.performance_monitoring = true;
        }

        if !other.error_reporting {
            self.error_reporting = false;
        }

        if other.log_level < self.log_level {
            self.log_level = other.log_level;
        }

        // Merge custom values (self takes precedence for existing keys)
        for (key, value) in &other.custom {
            if !self.custom.contains_key(key) {
                self.custom.insert(key.clone(), value.clone());
            }
        }
    }

    /// Create a merged configuration
    pub fn merged_with(mut self, other: &Config) -> Self {
        self.merge(other);
        self
    }

    /// Get a summary of the configuration
    pub fn summary(&self) -> String {
        format!(
            "Config(debug: {}, strict: {}, concurrent: {}, timeout: {:.1}s, log: {})",
            self.debug,
            self.strict,
            self.max_concurrent,
            self.default_timeout.as_secs_f64(),
            self.log_level
        )
    }

    /// Export configuration as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Import configuration from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Export configuration as environment variables
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        let mut vars = Vec::new();

        vars.push(("LEPTOS_STATE_DEBUG".to_string(), self.debug.to_string()));
        vars.push(("LEPTOS_STATE_STRICT".to_string(), self.strict.to_string()));
        vars.push(("LEPTOS_STATE_MAX_CONCURRENT".to_string(), self.max_concurrent.to_string()));
        vars.push(("LEPTOS_STATE_DEFAULT_TIMEOUT".to_string(), self.default_timeout.as_secs().to_string()));
        vars.push(("LEPTOS_STATE_PERFORMANCE_MONITORING".to_string(), self.performance_monitoring.to_string()));
        vars.push(("LEPTOS_STATE_ERROR_REPORTING".to_string(), self.error_reporting.to_string()));
        vars.push(("LEPTOS_STATE_LOG_LEVEL".to_string(), self.log_level.to_string()));

        for (key, value) in &self.custom {
            if let Some(str_value) = value.as_str() {
                vars.push((format!("LEPTOS_STATE_CUSTOM_{}", key.to_uppercase()), str_value.to_string()));
            }
        }

        vars
    }

    /// Load configuration from environment variables
    pub fn from_env_vars() -> Self {
        let mut config = Self::default();

        if let Ok(debug) = std::env::var("LEPTOS_STATE_DEBUG") {
            config.debug = debug.parse().unwrap_or(false);
        }

        if let Ok(strict) = std::env::var("LEPTOS_STATE_STRICT") {
            config.strict = strict.parse().unwrap_or(false);
        }

        if let Ok(max_concurrent) = std::env::var("LEPTOS_STATE_MAX_CONCURRENT") {
            config.max_concurrent = max_concurrent.parse().unwrap_or(10);
        }

        if let Ok(timeout_secs) = std::env::var("LEPTOS_STATE_DEFAULT_TIMEOUT") {
            if let Ok(secs) = timeout_secs.parse::<u64>() {
                config.default_timeout = std::time::Duration::from_secs(secs);
            }
        }

        if let Ok(perf) = std::env::var("LEPTOS_STATE_PERFORMANCE_MONITORING") {
            config.performance_monitoring = perf.parse().unwrap_or(false);
        }

        if let Ok(reporting) = std::env::var("LEPTOS_STATE_ERROR_REPORTING") {
            config.error_reporting = reporting.parse().unwrap_or(true);
        }

        if let Ok(log_level) = std::env::var("LEPTOS_STATE_LOG_LEVEL") {
            if let Ok(level) = log_level.parse::<LogLevel>() {
                config.log_level = level;
            }
        }

        config
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}
