//! Connection pool configuration

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

    /// Set keep-alive interval (maps to max_idle_time)
    pub fn keep_alive(mut self, interval: Option<std::time::Duration>) -> Self {
        if let Some(interval) = interval {
            self.max_idle_time = interval;
        }
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
            if max_life <= self.max_idle_time {
                return Err("max_lifetime must be greater than max_idle_time".to_string());
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

    /// Create low-latency pool config
    pub fn low_latency() -> Self {
        Self::new()
            .max_size(50)
            .min_idle(5)
            .max_idle_time(std::time::Duration::from_secs(30))
            .acquire_timeout(std::time::Duration::from_millis(100))
            .test_on_acquire(false) // Skip health checks for speed
    }

    /// Calculate effective pool utilization
    pub fn utilization_rate(&self, active_connections: usize) -> f64 {
        if self.max_size == 0 {
            0.0
        } else {
            active_connections as f64 / self.max_size as f64
        }
    }

    /// Check if pool is at capacity
    pub fn is_at_capacity(&self, active_connections: usize) -> bool {
        active_connections >= self.max_size
    }

    /// Get recommended pool size based on expected load
    pub fn recommended_size(expected_rps: usize, avg_request_time_ms: u64) -> usize {
        let concurrent_requests = (expected_rps as f64 * avg_request_time_ms as f64 / 1000.0).ceil() as usize;
        std::cmp::max(concurrent_requests, 1)
    }

    /// Estimate memory usage for this pool configuration
    pub fn estimate_memory_usage(&self, connection_memory_kb: usize) -> usize {
        // Estimate based on max connections and per-connection memory
        // This is a rough estimate and depends on the actual connection type
        let max_connections = self.max_size;
        let buffer_overhead = 1024; // KB for connection buffers
        (max_connections * (connection_memory_kb + buffer_overhead)) / 1024 // Convert to MB
    }
}

impl std::fmt::Display for PoolConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_config_defaults() {
        let config = PoolConfig::new();
        assert_eq!(config.max_size, 10);
        assert_eq!(config.min_idle, 1);
        assert_eq!(config.max_idle_time, std::time::Duration::from_secs(300));
        assert_eq!(config.max_lifetime, Some(std::time::Duration::from_secs(1800)));
        assert_eq!(config.acquire_timeout, std::time::Duration::from_secs(30));
        assert!(config.test_on_acquire);
        assert!(config.test_query.is_none());
    }

    #[test]
    fn test_pool_config_building() {
        let config = PoolConfig::new()
            .max_size(50)
            .min_idle(5)
            .max_idle_time(std::time::Duration::from_secs(120))
            .acquire_timeout(std::time::Duration::from_secs(10))
            .test_query("SELECT 1");

        assert_eq!(config.max_size, 50);
        assert_eq!(config.min_idle, 5);
        assert_eq!(config.max_idle_time, std::time::Duration::from_secs(120));
        assert_eq!(config.acquire_timeout, std::time::Duration::from_secs(10));
        assert_eq!(config.test_query, Some("SELECT 1".to_string()));
    }

    #[test]
    fn test_pool_config_validation() {
        // Valid config
        let valid_config = PoolConfig::new();
        assert!(valid_config.validate().is_ok());

        // Invalid max_size
        let invalid_size = PoolConfig::new().max_size(0);
        assert!(invalid_size.validate().is_err());

        // Invalid min_idle > max_size
        let invalid_idle = PoolConfig::new().max_size(5).min_idle(10);
        assert!(invalid_idle.validate().is_err());

        // Invalid timeouts
        let invalid_timeout = PoolConfig::new().acquire_timeout(std::time::Duration::from_secs(0));
        assert!(invalid_timeout.validate().is_err());

        let invalid_idle_time = PoolConfig::new().max_idle_time(std::time::Duration::from_secs(0));
        assert!(invalid_idle_time.validate().is_err());

        // Invalid lifetime <= idle time
        let invalid_lifetime = PoolConfig::new()
            .max_lifetime(Some(std::time::Duration::from_secs(100)))
            .max_idle_time(std::time::Duration::from_secs(200));
        assert!(invalid_lifetime.validate().is_err());
    }

    #[test]
    fn test_pool_config_presets() {
        let high_perf = PoolConfig::high_performance();
        assert_eq!(high_perf.max_size, 100);
        assert_eq!(high_perf.min_idle, 10);
        assert_eq!(high_perf.acquire_timeout, std::time::Duration::from_secs(5));

        let conservative = PoolConfig::conservative();
        assert_eq!(conservative.max_size, 5);
        assert_eq!(conservative.min_idle, 0);
        assert_eq!(conservative.acquire_timeout, std::time::Duration::from_secs(60));

        let low_latency = PoolConfig::low_latency();
        assert_eq!(low_latency.max_size, 50);
        assert_eq!(low_latency.acquire_timeout, std::time::Duration::from_millis(100));
        assert!(!low_latency.test_on_acquire);
    }

    #[test]
    fn test_pool_utilization() {
        let config = PoolConfig::new().max_size(10);

        assert_eq!(config.utilization_rate(5), 0.5);
        assert_eq!(config.utilization_rate(10), 1.0);
        assert_eq!(config.utilization_rate(0), 0.0);

        assert!(!config.is_at_capacity(5));
        assert!(config.is_at_capacity(10));
        assert!(config.is_at_capacity(15));
    }

    #[test]
    fn test_recommended_pool_size() {
        // 100 RPS with 100ms avg response time = 10 concurrent connections
        assert_eq!(PoolConfig::recommended_size(100, 100), 10);

        // 50 RPS with 50ms avg response time = 3 concurrent connections
        assert_eq!(PoolConfig::recommended_size(50, 50), 3);

        // Minimum of 1
        assert_eq!(PoolConfig::recommended_size(1, 1), 1);
    }

    #[test]
    fn test_memory_estimation() {
        let config = PoolConfig::new().max_size(10);

        // Assuming 64KB per connection
        let memory_mb = config.estimate_memory_usage(64);
        assert!(memory_mb > 0);

        // Larger pool should use more memory
        let large_config = PoolConfig::new().max_size(100);
        let large_memory = large_config.estimate_memory_usage(64);
        assert!(large_memory > memory_mb);
    }

    #[test]
    fn test_pool_config_display() {
        let config = PoolConfig::new();
        let display = format!("{}", config);
        assert!(display.contains("max_size: 10"));
        assert!(display.contains("min_idle: 1"));
        assert!(display.contains("test_on_acquire: true"));
    }
}
