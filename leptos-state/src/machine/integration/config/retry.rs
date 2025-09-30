//! Retry configuration structures

/// Retry configuration
#[derive(Debug, Clone, PartialEq)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries
    pub initial_delay: std::time::Duration,
    /// Maximum delay between retries
    pub max_delay: std::time::Duration,
    /// Backoff multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Jitter factor to add randomness to delays
    pub jitter_factor: f64,
    /// Whether to retry on specific error types
    pub retry_on_errors: Vec<String>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: std::time::Duration::from_millis(100),
            max_delay: std::time::Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
            retry_on_errors: vec![
                "network".to_string(),
                "timeout".to_string(),
                "service_unavailable".to_string(),
            ],
        }
    }
}

impl RetryConfig {
    /// Create a new retry config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum attempts
    pub fn max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Set initial delay
    pub fn initial_delay(mut self, delay: std::time::Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Set maximum delay
    pub fn max_delay(mut self, delay: std::time::Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Set backoff multiplier
    pub fn backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Set jitter factor
    pub fn jitter_factor(mut self, factor: f64) -> Self {
        self.jitter_factor = factor;
        self
    }

    /// Set retry error types
    pub fn retry_on_errors<I, S>(mut self, errors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.retry_on_errors = errors.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Add a retry error type
    pub fn add_retry_error<S: Into<String>>(mut self, error: S) -> Self {
        self.retry_on_errors.push(error.into());
        self
    }

    /// Calculate delay for a specific attempt
    pub fn delay_for_attempt(&self, attempt: u32) -> std::time::Duration {
        if attempt == 0 {
            return std::time::Duration::from_nanos(0);
        }

        let base_delay = self.initial_delay.as_millis() as f64 * self.backoff_multiplier.powi(attempt as i32 - 1);
        let mut delay_ms = base_delay.min(self.max_delay.as_millis() as f64);

        // Add jitter
        if self.jitter_factor > 0.0 {
            let jitter_range = delay_ms * self.jitter_factor;
            let jitter = (rand::random::<f64>() - 0.5) * 2.0 * jitter_range;
            delay_ms += jitter;
            delay_ms = delay_ms.max(0.0);
        }

        std::time::Duration::from_millis(delay_ms as u64)
    }

    /// Get total estimated delay for all retries
    pub fn estimated_total_delay(&self) -> std::time::Duration {
        let mut total = std::time::Duration::from_nanos(0);
        for attempt in 1..=self.max_attempts {
            total += self.delay_for_attempt(attempt);
        }
        total
    }

    /// Check if should retry for a specific error type
    pub fn should_retry_for_error(&self, error_type: &str) -> bool {
        self.retry_on_errors.iter().any(|e| e == error_type)
    }

    /// Check if more attempts are available
    pub fn can_retry(&self, current_attempts: u32) -> bool {
        current_attempts < self.max_attempts
    }

    /// Validate the retry configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.max_attempts == 0 {
            return Err("max_attempts must be greater than 0".to_string());
        }

        if self.backoff_multiplier <= 1.0 {
            return Err("backoff_multiplier must be greater than 1.0".to_string());
        }

        if self.jitter_factor < 0.0 || self.jitter_factor > 1.0 {
            return Err("jitter_factor must be between 0.0 and 1.0".to_string());
        }

        if self.max_delay < self.initial_delay {
            return Err("max_delay must be greater than or equal to initial_delay".to_string());
        }

        Ok(())
    }

    /// Merge with another retry config (self takes precedence)
    pub fn merge(&mut self, other: &RetryConfig) {
        self.max_attempts = self.max_attempts.max(other.max_attempts);
        self.initial_delay = self.initial_delay.min(other.initial_delay);
        self.max_delay = self.max_delay.max(other.max_delay);
        self.backoff_multiplier = self.backoff_multiplier.max(other.backoff_multiplier);
        self.jitter_factor = self.jitter_factor.max(other.jitter_factor);

        // Merge retry error types
        for error in &other.retry_on_errors {
            if !self.retry_on_errors.contains(error) {
                self.retry_on_errors.push(error.clone());
            }
        }
    }

    /// Get retry config summary
    pub fn summary(&self) -> String {
        format!(
            "RetryConfig {{ max_attempts: {}, initial_delay: {:.0}ms, max_delay: {:.0}s, multiplier: {:.1} }}",
            self.max_attempts,
            self.initial_delay.as_millis(),
            self.max_delay.as_secs_f64(),
            self.backoff_multiplier
        )
    }

    /// Create aggressive retry config (quick retries)
    pub fn aggressive() -> Self {
        Self::new()
            .max_attempts(5)
            .initial_delay(std::time::Duration::from_millis(50))
            .max_delay(std::time::Duration::from_secs(5))
            .backoff_multiplier(1.5)
    }

    /// Create conservative retry config (slow retries)
    pub fn conservative() -> Self {
        Self::new()
            .max_attempts(2)
            .initial_delay(std::time::Duration::from_secs(1))
            .max_delay(std::time::Duration::from_secs(60))
            .backoff_multiplier(3.0)
    }

    /// Create disabled retry config
    pub fn disabled() -> Self {
        Self::new().max_attempts(0)
    }
}

impl std::fmt::Display for RetryConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Builder for retry configuration
pub struct RetryConfigBuilder {
    config: RetryConfig,
}

impl RetryConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: RetryConfig::new(),
        }
    }

    /// Set max attempts
    pub fn max_attempts(mut self, attempts: u32) -> Self {
        self.config.max_attempts = attempts;
        self
    }

    /// Set initial delay in milliseconds
    pub fn initial_delay_ms(mut self, ms: u64) -> Self {
        self.config.initial_delay = std::time::Duration::from_millis(ms);
        self
    }

    /// Set initial delay
    pub fn initial_delay(mut self, delay: std::time::Duration) -> Self {
        self.config.initial_delay = delay;
        self
    }

    /// Set max delay in seconds
    pub fn max_delay_secs(mut self, secs: u64) -> Self {
        self.config.max_delay = std::time::Duration::from_secs(secs);
        self
    }

    /// Set max delay
    pub fn max_delay(mut self, delay: std::time::Duration) -> Self {
        self.config.max_delay = delay;
        self
    }

    /// Set backoff multiplier
    pub fn backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.config.backoff_multiplier = multiplier;
        self
    }

    /// Set jitter factor
    pub fn jitter_factor(mut self, factor: f64) -> Self {
        self.config.jitter_factor = factor;
        self
    }

    /// Set retry error types
    pub fn retry_on_errors<I, S>(mut self, errors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.config.retry_on_errors = errors.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Add retry error type
    pub fn add_retry_error<S: Into<String>>(mut self, error: S) -> Self {
        self.config.retry_on_errors.push(error.into());
        self
    }

    /// Build the configuration
    pub fn build(self) -> RetryConfig {
        self.config
    }

    /// Build aggressive config
    pub fn aggressive() -> RetryConfig {
        RetryConfig::aggressive()
    }

    /// Build conservative config
    pub fn conservative() -> RetryConfig {
        RetryConfig::conservative()
    }

    /// Build disabled config
    pub fn disabled() -> RetryConfig {
        RetryConfig::disabled()
    }
}

impl Default for RetryConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for retry configurations
pub mod factories {
    use super::*;

    /// Create default retry configuration
    pub fn default() -> RetryConfig {
        RetryConfig::default()
    }

    /// Create aggressive retry configuration
    pub fn aggressive() -> RetryConfig {
        RetryConfig::aggressive()
    }

    /// Create conservative retry configuration
    pub fn conservative() -> RetryConfig {
        RetryConfig::conservative()
    }

    /// Create disabled retry configuration
    pub fn disabled() -> RetryConfig {
        RetryConfig::disabled()
    }

    /// Create custom retry configuration
    pub fn custom<F>(f: F) -> RetryConfig
    where
        F: FnOnce(RetryConfigBuilder) -> RetryConfigBuilder,
    {
        let builder = RetryConfigBuilder::new();
        f(builder).build()
    }
}
