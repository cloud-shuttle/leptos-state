//! Health monitoring and status for integrations

/// Health status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HealthStatus {
    /// System is healthy
    Healthy,
    /// System is degraded but functional
    Degraded,
    /// System is unhealthy
    Unhealthy,
    /// System status is unknown
    Unknown,
}

impl HealthStatus {
    /// Check if the status indicates healthy operation
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// Check if the status indicates degraded operation
    pub fn is_degraded(&self) -> bool {
        matches!(self, Self::Degraded)
    }

    /// Check if the status indicates unhealthy operation
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, Self::Unhealthy)
    }

    /// Check if the status is unknown
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Get a numeric score for the health status (higher is better)
    pub fn score(&self) -> u8 {
        match self {
            Self::Healthy => 100,
            Self::Degraded => 50,
            Self::Unhealthy => 0,
            Self::Unknown => 25,
        }
    }

    /// Get the status as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unhealthy => "unhealthy",
            Self::Unknown => "unknown",
        }
    }

    /// Get the status color for UI display
    pub fn color(&self) -> &'static str {
        match self {
            Self::Healthy => "green",
            Self::Degraded => "yellow",
            Self::Unhealthy => "red",
            Self::Unknown => "gray",
        }
    }

    /// Get an emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Healthy => "✅",
            Self::Degraded => "⚠️",
            Self::Unhealthy => "❌",
            Self::Unknown => "❓",
        }
    }

    /// Combine multiple health statuses
    pub fn combine(statuses: &[Self]) -> Self {
        if statuses.is_empty() {
            return Self::Unknown;
        }

        let mut worst_score = 100;
        for status in statuses {
            worst_score = worst_score.min(status.score());
        }

        match worst_score {
            100 => Self::Healthy,
            50 => Self::Degraded,
            25 => Self::Unknown,
            0 => Self::Unhealthy,
            _ => Self::Unknown,
        }
    }

    /// Check if the status allows operation
    pub fn allows_operation(&self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded)
    }

    /// Get a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Healthy => "All systems operational",
            Self::Degraded => "System is operational but with reduced performance",
            Self::Unhealthy => "System is not operational",
            Self::Unknown => "System status cannot be determined",
        }
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji(), self.as_str())
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl std::str::FromStr for HealthStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "healthy" => Ok(Self::Healthy),
            "degraded" => Ok(Self::Degraded),
            "unhealthy" => Ok(Self::Unhealthy),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Invalid health status: {}", s)),
        }
    }
}

/// Health check result with additional information
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// Overall status
    pub status: HealthStatus,
    /// Detailed message
    pub message: String,
    /// Timestamp of the check
    pub timestamp: std::time::SystemTime,
    /// Response time in milliseconds
    pub response_time_ms: Option<u64>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl HealthCheckResult {
    /// Create a new health check result
    pub fn new(status: HealthStatus, message: String) -> Self {
        Self {
            status,
            message,
            timestamp: std::time::SystemTime::now(),
            response_time_ms: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a healthy result
    pub fn healthy(message: String) -> Self {
        Self::new(HealthStatus::Healthy, message)
    }

    /// Create a degraded result
    pub fn degraded(message: String) -> Self {
        Self::new(HealthStatus::Degraded, message)
    }

    /// Create an unhealthy result
    pub fn unhealthy(message: String) -> Self {
        Self::new(HealthStatus::Unhealthy, message)
    }

    /// Create an unknown result
    pub fn unknown(message: String) -> Self {
        Self::new(HealthStatus::Unknown, message)
    }

    /// Set response time
    pub fn with_response_time(mut self, ms: u64) -> Self {
        self.response_time_ms = Some(ms);
        self
    }

    /// Add metadata
    pub fn with_metadata<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Get age of the result
    pub fn age(&self) -> std::time::Duration {
        std::time::SystemTime::now()
            .duration_since(self.timestamp)
            .unwrap_or_default()
    }

    /// Check if the result is stale
    pub fn is_stale(&self, max_age_seconds: u64) -> bool {
        self.age().as_secs() > max_age_seconds
    }
}

impl std::fmt::Display for HealthCheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

/// Health monitor for tracking integration health
pub struct HealthMonitor {
    /// Health check results by integration name
    results: std::sync::Mutex<std::collections::HashMap<String, HealthCheckResult>>,
    /// Health check interval
    check_interval: std::time::Duration,
    /// Maximum age for cached results
    max_cache_age: std::time::Duration,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(check_interval_seconds: u64, max_cache_age_seconds: u64) -> Self {
        Self {
            results: std::sync::Mutex::new(std::collections::HashMap::new()),
            check_interval: std::time::Duration::from_secs(check_interval_seconds),
            max_cache_age: std::time::Duration::from_secs(max_cache_age_seconds),
        }
    }

    /// Record a health check result
    pub fn record_result(&self, integration_name: String, result: HealthCheckResult) {
        self.results.lock().unwrap().insert(integration_name, result);
    }

    /// Get health status for an integration
    pub fn get_status(&self, integration_name: &str) -> Option<HealthStatus> {
        self.results.lock().unwrap()
            .get(integration_name)
            .map(|result| result.status.clone())
    }

    /// Get detailed health result for an integration
    pub fn get_result(&self, integration_name: &str) -> Option<HealthCheckResult> {
        self.results.lock().unwrap()
            .get(integration_name)
            .cloned()
    }

    /// Get all health statuses
    pub fn get_all_statuses(&self) -> std::collections::HashMap<String, HealthStatus> {
        self.results.lock().unwrap()
            .iter()
            .map(|(name, result)| (name.clone(), result.status.clone()))
            .collect()
    }

    /// Get overall health status
    pub fn get_overall_status(&self) -> HealthStatus {
        let statuses: Vec<HealthStatus> = self.results.lock().unwrap()
            .values()
            .map(|result| result.status.clone())
            .collect();

        HealthStatus::combine(&statuses)
    }

    /// Check if any integration is unhealthy
    pub fn has_unhealthy_integrations(&self) -> bool {
        self.results.lock().unwrap()
            .values()
            .any(|result| result.status.is_unhealthy())
    }

    /// Get unhealthy integrations
    pub fn get_unhealthy_integrations(&self) -> Vec<String> {
        self.results.lock().unwrap()
            .iter()
            .filter(|(_, result)| result.status.is_unhealthy())
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Clear old cached results
    pub fn clear_stale_results(&self) {
        self.results.lock().unwrap()
            .retain(|_, result| !result.is_stale(self.max_cache_age.as_secs()));
    }

    /// Get health statistics
    pub fn get_statistics(&self) -> HealthStatistics {
        let results = self.results.lock().unwrap();
        let total = results.len();

        let mut healthy = 0;
        let mut degraded = 0;
        let mut unhealthy = 0;
        let mut unknown = 0;

        for result in results.values() {
            match result.status {
                HealthStatus::Healthy => healthy += 1,
                HealthStatus::Degraded => degraded += 1,
                HealthStatus::Unhealthy => unhealthy += 1,
                HealthStatus::Unknown => unknown += 1,
            }
        }

        HealthStatistics {
            total_integrations: total,
            healthy_integrations: healthy,
            degraded_integrations: degraded,
            unhealthy_integrations: unhealthy,
            unknown_integrations: unknown,
            overall_status: self.get_overall_status(),
        }
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new(60, 300) // Check every minute, cache for 5 minutes
    }
}

/// Health statistics
#[derive(Debug, Clone)]
pub struct HealthStatistics {
    /// Total number of integrations
    pub total_integrations: usize,
    /// Number of healthy integrations
    pub healthy_integrations: usize,
    /// Number of degraded integrations
    pub degraded_integrations: usize,
    /// Number of unhealthy integrations
    pub unhealthy_integrations: usize,
    /// Number of integrations with unknown status
    pub unknown_integrations: usize,
    /// Overall health status
    pub overall_status: HealthStatus,
}

impl std::fmt::Display for HealthStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Health: {} total ({} healthy, {} degraded, {} unhealthy, {} unknown) - {}",
            self.total_integrations,
            self.healthy_integrations,
            self.degraded_integrations,
            self.unhealthy_integrations,
            self.unknown_integrations,
            self.overall_status
        )
    }
}
