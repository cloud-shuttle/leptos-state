//! Core integration configuration structures

use super::routing::EventRoutingConfig;
use super::retry::RetryConfig;

/// Integration configuration for state machines
#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationConfig {
    /// Whether integration is enabled
    pub enabled: bool,
    /// Maximum concurrent integrations
    pub max_concurrent: usize,
    /// Timeout for integration operations
    pub timeout: std::time::Duration,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Event routing configuration
    pub event_routing: EventRoutingConfig,
    /// Whether to collect metrics
    pub collect_metrics: bool,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_concurrent: 10,
            timeout: std::time::Duration::from_secs(30),
            retry_config: RetryConfig::default(),
            event_routing: EventRoutingConfig::default(),
            collect_metrics: true,
        }
    }
}

impl IntegrationConfig {
    /// Create a new integration config
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable integration
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set maximum concurrent integrations
    pub fn max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Set timeout for operations
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set retry configuration
    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Set event routing configuration
    pub fn event_routing(mut self, config: EventRoutingConfig) -> Self {
        self.event_routing = config;
        self
    }

    /// Enable or disable metrics collection
    pub fn collect_metrics(mut self, collect: bool) -> Self {
        self.collect_metrics = collect;
        self
    }

    /// Create a disabled configuration
    pub fn disabled() -> Self {
        Self::new().enabled(false)
    }

    /// Create a high-performance configuration
    pub fn high_performance() -> Self {
        Self::new()
            .max_concurrent(100)
            .timeout(std::time::Duration::from_secs(10))
            .collect_metrics(false)
    }

    /// Create a conservative configuration
    pub fn conservative() -> Self {
        Self::new()
            .max_concurrent(2)
            .timeout(std::time::Duration::from_secs(60))
            .retry_config(RetryConfig::new().max_attempts(5))
    }

    /// Check if configuration is valid
    pub fn validate(&self) -> Result<(), String> {
        if self.max_concurrent == 0 {
            return Err("max_concurrent must be greater than 0".to_string());
        }

        if self.timeout.as_secs() == 0 {
            return Err("timeout must be greater than 0".to_string());
        }

        self.retry_config.validate()?;
        self.event_routing.validate()?;

        Ok(())
    }

    /// Get a summary of the configuration
    pub fn summary(&self) -> String {
        format!(
            "IntegrationConfig {{ enabled: {}, max_concurrent: {}, timeout: {:.1}s, metrics: {} }}",
            self.enabled,
            self.max_concurrent,
            self.timeout.as_secs_f64(),
            self.collect_metrics
        )
    }

    /// Merge with another configuration (self takes precedence)
    pub fn merge(&mut self, other: &IntegrationConfig) {
        if !self.enabled && other.enabled {
            self.enabled = true;
        }

        self.max_concurrent = self.max_concurrent.max(other.max_concurrent);

        if other.timeout < self.timeout {
            self.timeout = other.timeout;
        }

        self.retry_config.merge(&other.retry_config);
        self.event_routing.merge(&other.event_routing);

        if !self.collect_metrics && other.collect_metrics {
            self.collect_metrics = true;
        }
    }

    /// Check if integration is ready to use
    pub fn is_ready(&self) -> bool {
        self.enabled && self.max_concurrent > 0
    }

    /// Get effective timeout (considering retry delays)
    pub fn effective_timeout(&self) -> std::time::Duration {
        let total_retry_time = self.retry_config.estimated_total_delay();
        self.timeout.saturating_add(total_retry_time)
    }
}

impl std::fmt::Display for IntegrationConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Builder for integration configuration
pub struct IntegrationConfigBuilder {
    config: IntegrationConfig,
}

impl IntegrationConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: IntegrationConfig::new(),
        }
    }

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self
    }

    /// Set max concurrent
    pub fn max_concurrent(mut self, max: usize) -> Self {
        self.config.max_concurrent = max;
        self
    }

    /// Set timeout in seconds
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.config.timeout = std::time::Duration::from_secs(secs);
        self
    }

    /// Set timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Configure retries
    pub fn with_retry<F>(mut self, f: F) -> Self
    where
        F: FnOnce(super::retry::RetryConfigBuilder) -> super::retry::RetryConfigBuilder,
    {
        let builder = super::retry::RetryConfigBuilder::new();
        self.config.retry_config = f(builder).build();
        self
    }

    /// Configure routing
    pub fn with_routing<F>(mut self, f: F) -> Self
    where
        F: FnOnce(super::routing::EventRoutingConfigBuilder) -> super::routing::EventRoutingConfigBuilder,
    {
        let builder = super::routing::EventRoutingConfigBuilder::new();
        self.config.event_routing = f(builder).build();
        self
    }

    /// Set metrics collection
    pub fn collect_metrics(mut self, collect: bool) -> Self {
        self.config.collect_metrics = collect;
        self
    }

    /// Build the configuration
    pub fn build(self) -> IntegrationConfig {
        self.config
    }

    /// Build and validate the configuration
    pub fn build_validated(self) -> Result<IntegrationConfig, String> {
        let config = self.build();
        config.validate()?;
        Ok(config)
    }
}

impl Default for IntegrationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<IntegrationConfig> for IntegrationConfigBuilder {
    fn from(config: IntegrationConfig) -> Self {
        Self { config }
    }
}

/// Convenience functions for creating integration configs
pub mod factories {
    use super::*;

    /// Create a default integration configuration
    pub fn default() -> IntegrationConfig {
        IntegrationConfig::default()
    }

    /// Create a disabled configuration
    pub fn disabled() -> IntegrationConfig {
        IntegrationConfig::disabled()
    }

    /// Create a high-performance configuration
    pub fn high_performance() -> IntegrationConfig {
        IntegrationConfig::high_performance()
    }

    /// Create a conservative configuration
    pub fn conservative() -> IntegrationConfig {
        IntegrationConfig::conservative()
    }

    /// Create a custom configuration using builder pattern
    pub fn custom<F>(f: F) -> IntegrationConfig
    where
        F: FnOnce(IntegrationConfigBuilder) -> IntegrationConfigBuilder,
    {
        let builder = IntegrationConfigBuilder::new();
        f(builder).build()
    }
}
