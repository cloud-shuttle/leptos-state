//! Configuration manager for centralized configuration management

use super::core::Config;
use super::environment::Environment;
use super::logging::LogLevel;

/// Configuration manager
#[derive(Debug)]
pub struct ConfigManager {
    /// Current configuration
    config: Config,
    /// Environment
    environment: Environment,
    /// Configuration history
    history: Vec<(std::time::SystemTime, Config)>,
    /// Maximum history size
    max_history: usize,
    /// Listeners for configuration changes
    listeners: Vec<Box<dyn Fn(&Config, &Config) + Send + Sync>>,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            environment: Environment::Development,
            history: Vec::new(),
            max_history: 10,
            listeners: Vec::new(),
        }
    }

    /// Create with initial configuration
    pub fn with_config(config: Config) -> Self {
        let mut manager = Self::new();
        manager.config = config.clone();
        manager.history.push((std::time::SystemTime::now(), config));
        manager
    }

    /// Get the current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get a mutable reference to the configuration
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, new_config: Config) -> Result<(), String> {
        // Validate the new configuration
        new_config.validate()?;

        let old_config = self.config.clone();
        self.config = new_config.clone();

        // Add to history
        self.history.push((std::time::SystemTime::now(), new_config));
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        // Notify listeners
        for listener in &self.listeners {
            listener(&old_config, &self.config);
        }

        Ok(())
    }

    /// Merge configuration updates
    pub fn merge_config(&mut self, updates: Config) -> Result<(), String> {
        let mut new_config = self.config.clone();
        new_config.merge(&updates);
        self.update_config(new_config)
    }

    /// Get configuration value
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.get_custom(key)
    }

    /// Set configuration value
    pub fn set(&mut self, key: String, value: serde_json::Value) -> Result<(), String> {
        let mut new_config = self.config.clone();
        new_config.custom.insert(key, value);
        self.update_config(new_config)
    }

    /// Remove configuration value
    pub fn remove(&mut self, key: &str) -> Result<Option<serde_json::Value>, String> {
        let mut new_config = self.config.clone();
        let value = new_config.custom.remove(key);
        self.update_config(new_config)?;
        Ok(value)
    }

    /// Get environment
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    /// Set environment
    pub fn set_environment(&mut self, environment: Environment) {
        self.environment = environment;
    }

    /// Check if debug mode is enabled
    pub fn is_debug_enabled(&self) -> bool {
        self.config.debug
    }

    /// Check if strict mode is enabled
    pub fn is_strict_enabled(&self) -> bool {
        self.config.strict
    }

    /// Get log level
    pub fn log_level(&self) -> LogLevel {
        self.config.log_level
    }

    /// Get maximum concurrent operations
    pub fn max_concurrent(&self) -> usize {
        self.config.max_concurrent
    }

    /// Get default timeout
    pub fn default_timeout(&self) -> std::time::Duration {
        self.config.default_timeout
    }

    /// Check if performance monitoring is enabled
    pub fn performance_monitoring_enabled(&self) -> bool {
        self.config.performance_monitoring
    }

    /// Check if error reporting is enabled
    pub fn error_reporting_enabled(&self) -> bool {
        self.config.error_reporting
    }

    /// Add a configuration change listener
    pub fn add_listener<F>(&mut self, listener: F)
    where
        F: Fn(&Config, &Config) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    /// Remove all listeners
    pub fn clear_listeners(&mut self) {
        self.listeners.clear();
    }

    /// Get configuration history
    pub fn history(&self) -> &[(std::time::SystemTime, Config)] {
        &self.history
    }

    /// Get the last configuration change
    pub fn last_change(&self) -> Option<&(std::time::SystemTime, Config)> {
        self.history.last()
    }

    /// Revert to previous configuration
    pub fn revert(&mut self) -> Result<(), String> {
        if self.history.len() < 2 {
            return Err("No previous configuration to revert to".to_string());
        }

        // Remove current config from history
        self.history.pop();

        // Get the previous config
        if let Some((_, prev_config)) = self.history.last() {
            let old_config = self.config.clone();
            self.config = prev_config.clone();

            // Notify listeners
            for listener in &self.listeners {
                listener(&old_config, &self.config);
            }

            Ok(())
        } else {
            Err("Failed to revert configuration".to_string())
        }
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), String> {
        let content = self.config.to_json()
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// Load configuration from file
    pub fn load_from_file<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: Config = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        self.update_config(config)
    }

    /// Export configuration as environment variables
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        self.config.to_env_vars()
    }

    /// Import configuration from environment variables
    pub fn from_env_vars(&mut self) -> Result<(), String> {
        let config = Config::from_env_vars();
        self.update_config(config)
    }

    /// Get configuration summary
    pub fn summary(&self) -> String {
        format!(
            "ConfigManager(env: {}, config: {}, history: {})",
            self.environment,
            self.config.summary(),
            self.history.len()
        )
    }

    /// Validate current configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        self.config.validate()
    }

    /// Check if configuration is valid
    pub fn is_valid(&self) -> bool {
        self.config.is_valid()
    }

    /// Reset to default configuration
    pub fn reset(&mut self) -> Result<(), String> {
        let default_config = self.environment.default_config();
        self.update_config(default_config)
    }

    /// Get configuration statistics
    pub fn stats(&self) -> ConfigStats {
        ConfigStats {
            custom_values: self.config.custom.len(),
            history_size: self.history.len(),
            listeners_count: self.listeners.len(),
            is_valid: self.is_valid(),
            last_change: self.last_change().map(|(time, _)| *time),
        }
    }
}

impl Clone for ConfigManager {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            environment: self.environment,
            history: self.history.clone(),
            max_history: self.max_history,
            listeners: Vec::new(), // Can't clone trait objects
        }
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ConfigManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Configuration statistics
#[derive(Debug, Clone)]
pub struct ConfigStats {
    /// Number of custom configuration values
    pub custom_values: usize,
    /// Size of configuration history
    pub history_size: usize,
    /// Number of registered listeners
    pub listeners_count: usize,
    /// Whether the current configuration is valid
    pub is_valid: bool,
    /// Timestamp of last configuration change
    pub last_change: Option<std::time::SystemTime>,
}

impl std::fmt::Display for ConfigStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConfigStats(custom: {}, history: {}, listeners: {}, valid: {})",
            self.custom_values, self.history_size, self.listeners_count, self.is_valid
        )
    }
}

/// Global configuration manager instance
static mut GLOBAL_CONFIG_MANAGER: Option<ConfigManager> = None;

/// Initialize the global configuration manager
pub fn init_global_manager(config: Config) -> Result<(), String> {
    unsafe {
        if GLOBAL_CONFIG_MANAGER.is_some() {
            return Err("Global config manager already initialized".to_string());
        }
        GLOBAL_CONFIG_MANAGER = Some(ConfigManager::with_config(config));
        Ok(())
    }
}

/// Get the global configuration manager
pub fn global_manager() -> &'static ConfigManager {
    unsafe {
        GLOBAL_CONFIG_MANAGER.as_ref()
            .expect("Global config manager not initialized. Call init_global_manager() first.")
    }
}

/// Get the global configuration manager mutably (unsafe)
pub fn global_manager_mut() -> &'static mut ConfigManager {
    unsafe {
        GLOBAL_CONFIG_MANAGER.as_mut()
            .expect("Global config manager not initialized. Call init_global_manager() first.")
    }
}

/// Convenience function to get current configuration
pub fn current_config() -> &'static Config {
    global_manager().config()
}

/// Convenience function to check if debug is enabled
pub fn is_debug_enabled() -> bool {
    global_manager().is_debug_enabled()
}

/// Convenience function to check if strict mode is enabled
pub fn is_strict_enabled() -> bool {
    global_manager().is_strict_enabled()
}

/// Convenience function to get log level
pub fn log_level() -> LogLevel {
    global_manager().log_level()
}

/// Convenience function to get max concurrent operations
pub fn max_concurrent() -> usize {
    global_manager().max_concurrent()
}

/// Convenience function to get default timeout
pub fn default_timeout() -> std::time::Duration {
    global_manager().default_timeout()
}
