//! Environment-specific configuration

use super::core::Config;
use super::logging::{LogLevel, LoggerConfig};

/// Environment-specific configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    Custom(&'static str),
}

impl Environment {
    /// Get the environment as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Testing => "testing",
            Self::Staging => "staging",
            Self::Production => "production",
            Self::Custom(name) => name,
        }
    }

    /// Check if this is a development environment
    pub fn is_development(&self) -> bool {
        matches!(self, Self::Development)
    }

    /// Check if this is a testing environment
    pub fn is_testing(&self) -> bool {
        matches!(self, Self::Testing)
    }

    /// Check if this is a staging environment
    pub fn is_staging(&self) -> bool {
        matches!(self, Self::Staging)
    }

    /// Check if this is a production environment
    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production)
    }

    /// Check if this is a custom environment
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    /// Get the default configuration for this environment
    pub fn default_config(&self) -> Config {
        match self {
            Self::Development => Config::development(),
            Self::Testing => Config::test(),
            Self::Production => Config::production(),
            Self::Staging => Config::production(), // Similar to production but may have some debug features
            Self::Custom(_) => Config::default(),
        }
    }

    /// Get the default logger configuration for this environment
    pub fn default_logger_config(&self) -> LoggerConfig {
        match self {
            Self::Development => LoggerConfig::development(),
            Self::Testing => LoggerConfig::test(),
            Self::Production => LoggerConfig::production(),
            Self::Staging => LoggerConfig::production(), // Similar to production
            Self::Custom(_) => LoggerConfig::default(),
        }
    }

    /// Check if debug features should be enabled
    pub fn debug_enabled(&self) -> bool {
        matches!(self, Self::Development | Self::Testing)
    }

    /// Check if strict mode should be enabled
    pub fn strict_enabled(&self) -> bool {
        matches!(self, Self::Production | Self::Staging)
    }

    /// Check if performance monitoring should be enabled
    pub fn performance_monitoring_enabled(&self) -> bool {
        matches!(self, Self::Development | Self::Staging)
    }

    /// Get the appropriate log level for this environment
    pub fn log_level(&self) -> LogLevel {
        match self {
            Self::Development => LogLevel::Debug,
            Self::Testing => LogLevel::Debug,
            Self::Staging => LogLevel::Info,
            Self::Production => LogLevel::Warn,
            Self::Custom(_) => LogLevel::Info,
        }
    }

    /// Get the maximum concurrent operations for this environment
    pub fn max_concurrent(&self) -> usize {
        match self {
            Self::Development => 5,
            Self::Testing => 1,
            Self::Staging => 20,
            Self::Production => 50,
            Self::Custom(_) => 10,
        }
    }

    /// Get the default timeout for this environment
    pub fn default_timeout(&self) -> std::time::Duration {
        match self {
            Self::Development => std::time::Duration::from_secs(60),
            Self::Testing => std::time::Duration::from_secs(10),
            Self::Staging => std::time::Duration::from_secs(30),
            Self::Production => std::time::Duration::from_secs(30),
            Self::Custom(_) => std::time::Duration::from_secs(30),
        }
    }

    /// Check if error reporting should be enabled
    pub fn error_reporting_enabled(&self) -> bool {
        !matches!(self, Self::Testing) // Usually disabled in testing
    }

    /// Get all environments
    pub fn all() -> Vec<Self> {
        vec![
            Self::Development,
            Self::Testing,
            Self::Staging,
            Self::Production,
        ]
    }

    /// Parse from string (case-insensitive)
    pub fn from_str_ignore_case(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "dev" | "development" => Some(Self::Development),
            "test" | "testing" => Some(Self::Testing),
            "stage" | "staging" => Some(Self::Staging),
            "prod" | "production" => Some(Self::Production),
            custom => Some(Self::Custom(Box::leak(custom.to_string().into_boxed_str()))),
        }
    }

    /// Detect environment from environment variables
    pub fn detect_from_env() -> Self {
        if let Ok(env_var) = std::env::var("LEPTOS_STATE_ENV") {
            Self::from_str_ignore_case(&env_var).unwrap_or(Self::Development)
        } else if let Ok(node_env) = std::env::var("NODE_ENV") {
            // Support Node.js style environment variables
            Self::from_str_ignore_case(&node_env).unwrap_or(Self::Development)
        } else if cfg!(debug_assertions) {
            Self::Development
        } else {
            Self::Production
        }
    }

    /// Get environment variables that should be set for this environment
    pub fn env_vars(&self) -> Vec<(String, String)> {
        let mut vars = Vec::new();
        vars.push(("LEPTOS_STATE_ENV".to_string(), self.as_str().to_string()));

        // Add common environment variables
        match self {
            Self::Development => {
                vars.push(("RUST_BACKTRACE".to_string(), "1".to_string()));
                vars.push(("RUST_LOG".to_string(), "debug".to_string()));
            }
            Self::Testing => {
                vars.push(("RUST_TEST_THREADS".to_string(), "1".to_string()));
            }
            Self::Production => {
                vars.push(("RUST_BACKTRACE".to_string(), "0".to_string()));
            }
            _ => {}
        }

        vars
    }

    /// Validate that the current system is suitable for this environment
    pub fn validate_system(&self) -> Result<(), Vec<String>> {
        let mut issues = Vec::new();

        match self {
            Self::Production => {
                // In production, we might want to check for certain conditions
                if cfg!(debug_assertions) {
                    issues.push("Production environment detected but debug assertions are enabled".to_string());
                }

                // Check if we're running as root (Unix systems)
                #[cfg(unix)]
                {
                    if unsafe { libc::getuid() } == 0 {
                        issues.push("Running as root in production environment".to_string());
                    }
                }
            }
            Self::Testing => {
                // In testing, we might want to ensure we're not in production
                if std::env::var("CI").is_ok() {
                    // We're in CI, that's fine
                } else if std::env::var("PRODUCTION").is_ok() {
                    issues.push("Testing environment detected but PRODUCTION variable is set".to_string());
                }
            }
            _ => {}
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(issues)
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_ignore_case(s)
            .ok_or_else(|| format!("Invalid environment: {}", s))
    }
}

/// Environment configuration loader
pub struct EnvironmentLoader {
    /// Base configuration directory
    config_dir: std::path::PathBuf,
    /// Environment-specific overrides
    overrides: std::collections::HashMap<String, serde_json::Value>,
}

impl EnvironmentLoader {
    /// Create a new environment loader
    pub fn new() -> Self {
        Self {
            config_dir: std::path::PathBuf::from("config"),
            overrides: std::collections::HashMap::new(),
        }
    }

    /// Set the configuration directory
    pub fn with_config_dir<P: Into<std::path::PathBuf>>(mut self, dir: P) -> Self {
        self.config_dir = dir.into();
        self
    }

    /// Add an environment-specific override
    pub fn with_override<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.overrides.insert(key.into(), value.into());
        self
    }

    /// Load configuration for the given environment
    pub fn load_config(&self, environment: &Environment) -> Result<Config, String> {
        let mut config = environment.default_config();

        // Load base configuration
        let base_config_path = self.config_dir.join("base.json");
        if base_config_path.exists() {
            let base_config: Config = self.load_config_file(&base_config_path)?;
            config.merge(&base_config);
        }

        // Load environment-specific configuration
        let env_config_path = self.config_dir.join(format!("{}.json", environment.as_str()));
        if env_config_path.exists() {
            let env_config: Config = self.load_config_file(&env_config_path)?;
            config.merge(&env_config);
        }

        // Apply overrides
        for (key, value) in &self.overrides {
            config.custom.insert(key.clone(), value.clone());
        }

        // Validate the final configuration
        config.validate().map_err(|errors| {
            format!("Configuration validation failed: {}", errors.join(", "))
        })?;

        Ok(config)
    }

    /// Load a configuration file
    fn load_config_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<Config, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))
    }

    /// Save configuration for the given environment
    pub fn save_config(&self, environment: &Environment, config: &Config) -> Result<(), String> {
        std::fs::create_dir_all(&self.config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;

        let env_config_path = self.config_dir.join(format!("{}.json", environment.as_str()));
        let content = config.to_json()
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        std::fs::write(&env_config_path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// List available configuration files
    pub fn list_configs(&self) -> Result<Vec<String>, String> {
        if !self.config_dir.exists() {
            return Ok(Vec::new());
        }

        let mut configs = Vec::new();
        for entry in std::fs::read_dir(&self.config_dir)
            .map_err(|e| format!("Failed to read config directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                    configs.push(filename.to_string());
                }
            }
        }

        Ok(configs)
    }
}

impl Default for EnvironmentLoader {
    fn default() -> Self {
        Self::new()
    }
}
