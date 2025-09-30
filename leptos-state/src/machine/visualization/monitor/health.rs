//! Health checking and monitoring for state machines

use crate::machine::{Machine, MachineStateImpl};

/// Health checker for state machines
pub struct HealthChecker<C: Send + Sync, E> {
    /// Machine being checked
    machine: Option<Machine<C, E, C>>,
    /// Last health check time
    last_check: Option<std::time::SystemTime>,
    /// Health check interval
    check_interval: std::time::Duration,
    /// Maximum allowed errors
    max_errors: u32,
    /// Current error count
    error_count: u32,
    /// Health check history
    history: Vec<HealthCheckResult>,
    /// Maximum history size
    max_history: usize,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> HealthChecker<C, E> {
    /// Create a new health checker
    pub fn new() -> Self {
        Self {
            machine: None,
            last_check: None,
            check_interval: std::time::Duration::from_secs(60), // Check every minute
            max_errors: 5,
            error_count: 0,
            history: Vec::new(),
            max_history: 100,
        }
    }

    /// Create with custom configuration
    pub fn with_config(check_interval: std::time::Duration, max_errors: u32) -> Self {
        Self {
            check_interval,
            max_errors,
            ..Self::new()
        }
    }

    /// Set the machine to check
    pub fn set_machine(&mut self, machine: Machine<C, E, C>) {
        self.machine = Some(machine);
        self.reset();
    }

    /// Perform a health check
    pub fn check_health(&mut self) -> HealthCheckResult {
        let now = std::time::SystemTime::now();

        // Check if enough time has passed since last check
        if let Some(last) = self.last_check {
            if now.duration_since(last).unwrap_or_default() < self.check_interval {
                return self.last_result().unwrap_or_else(|| HealthCheckResult::unknown("No previous check"));
            }
        }

        self.last_check = Some(now);

        let result = if let Some(ref machine) = self.machine {
            self.perform_machine_check(machine)
        } else {
            HealthCheckResult::error("No machine configured")
        };

        // Update error count
        if result.status.is_error() {
            self.error_count += 1;
        } else if result.status.is_healthy() {
            self.error_count = 0; // Reset on successful check
        }

        // Add to history
        self.history.push(result.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        result
    }

    /// Perform actual machine health check
    fn perform_machine_check(&self, machine: &Machine<C, E, C>) -> HealthCheckResult {
        let start_time = std::time::Instant::now();

        // Try to get initial state
        let initial_result = std::panic::catch_unwind(|| {
            machine.initial_state()
        });

        let mut status = HealthStatus::Healthy;
        let mut message = "Machine is healthy".to_string();
        let mut metadata = std::collections::HashMap::new();

        match initial_result {
            Ok(initial_state) => {
                metadata.insert("initial_state".to_string(), serde_json::json!(initial_state.value()));

                // Try a simple transition (if possible)
                // This is a basic check - in practice you'd want more comprehensive checks
                metadata.insert("state_count".to_string(), serde_json::json!(1));
            }
            Err(_) => {
                status = HealthStatus::Unhealthy;
                message = "Failed to get initial state".to_string();
                self.error_count += 1;
            }
        }

        // Check error threshold
        if self.error_count >= self.max_errors {
            status = HealthStatus::Unhealthy;
            message = format!("Too many errors: {}", self.error_count);
        }

        let response_time = start_time.elapsed();

        HealthCheckResult {
            status,
            message,
            timestamp: std::time::SystemTime::now(),
            response_time_ms: Some(response_time.as_millis() as u64),
            metadata,
        }
    }

    /// Get the last health check result
    pub fn last_result(&self) -> Option<&HealthCheckResult> {
        self.history.last()
    }

    /// Get health status
    pub fn status(&self) -> HealthStatus {
        self.last_result()
            .map(|r| r.status)
            .unwrap_or(HealthStatus::Unknown)
    }

    /// Check if healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.status(), HealthStatus::Healthy)
    }

    /// Get error count
    pub fn error_count(&self) -> u32 {
        self.error_count
    }

    /// Get health history
    pub fn history(&self) -> &[HealthCheckResult] {
        &self.history
    }

    /// Get health score (0-100)
    pub fn health_score(&self) -> u8 {
        if self.history.is_empty() {
            50 // Unknown
        } else {
            let healthy_count = self.history.iter().filter(|r| r.status.is_healthy()).count();
            let total_count = self.history.len();
            ((healthy_count as f64 / total_count as f64) * 100.0) as u8
        }
    }

    /// Reset the health checker
    pub fn reset(&mut self) {
        self.last_check = None;
        self.error_count = 0;
        self.history.clear();
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Set check interval
    pub fn set_check_interval(&mut self, interval: std::time::Duration) {
        self.check_interval = interval;
    }

    /// Set maximum errors
    pub fn set_max_errors(&mut self, max_errors: u32) {
        self.max_errors = max_errors;
    }

    /// Get statistics
    pub fn stats(&self) -> HealthStats {
        let total_checks = self.history.len();
        let healthy_checks = self.history.iter().filter(|r| r.status.is_healthy()).count();
        let unhealthy_checks = self.history.iter().filter(|r| r.status.is_unhealthy()).count();
        let unknown_checks = self.history.iter().filter(|r| r.status.is_unknown()).count();

        let avg_response_time = if !self.history.is_empty() {
            let total_time: u64 = self.history.iter()
                .filter_map(|r| r.response_time_ms)
                .sum();
            total_time / self.history.len() as u64
        } else {
            0
        };

        HealthStats {
            total_checks: total_checks as u64,
            healthy_checks: healthy_checks as u64,
            unhealthy_checks: unhealthy_checks as u64,
            unknown_checks: unknown_checks as u64,
            current_errors: self.error_count,
            avg_response_time_ms: avg_response_time,
            health_score: self.health_score(),
        }
    }
}

impl<C: Send + Sync, E> Default for HealthChecker<C, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> std::fmt::Debug for HealthChecker<C, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HealthChecker")
            .field("last_check", &self.last_check)
            .field("error_count", &self.error_count)
            .field("history_size", &self.history.len())
            .field("status", &self.status())
            .finish()
    }
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> std::fmt::Display for HealthChecker<C, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HealthChecker(status: {}, errors: {}, checks: {})",
            self.status(),
            self.error_count,
            self.history.len()
        )
    }
}

/// Health check trait
pub trait HealthCheck<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> {
    /// Perform a health check
    fn check_health(&self) -> HealthCheckResult;

    /// Get current health status
    fn health_status(&self) -> HealthStatus {
        self.check_health().status
    }

    /// Check if healthy
    fn is_healthy(&self) -> bool {
        self.health_status().is_healthy()
    }
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> HealthCheck<C, E> for HealthChecker<C, E> {
    fn check_health(&self) -> HealthCheckResult {
        // This is a read-only check, so we need to clone or use a different approach
        // For simplicity, we'll return the last result
        self.last_result().cloned().unwrap_or_else(|| HealthCheckResult::unknown("No health checks performed"))
    }
}

/// Health check result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthCheckResult {
    /// Health status
    pub status: HealthStatus,
    /// Descriptive message
    pub message: String,
    /// Timestamp of the check
    pub timestamp: std::time::SystemTime,
    /// Response time in milliseconds
    pub response_time_ms: Option<u64>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl HealthCheckResult {
    /// Create a healthy result
    pub fn healthy(message: impl Into<String>) -> Self {
        Self {
            status: HealthStatus::Healthy,
            message: message.into(),
            timestamp: std::time::SystemTime::now(),
            response_time_ms: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create an unhealthy result
    pub fn unhealthy(message: impl Into<String>) -> Self {
        Self {
            status: HealthStatus::Unhealthy,
            message: message.into(),
            timestamp: std::time::SystemTime::now(),
            response_time_ms: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create an error result
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            status: HealthStatus::Unhealthy,
            message: message.into(),
            timestamp: std::time::SystemTime::now(),
            response_time_ms: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create an unknown result
    pub fn unknown(message: impl Into<String>) -> Self {
        Self {
            status: HealthStatus::Unknown,
            message: message.into(),
            timestamp: std::time::SystemTime::now(),
            response_time_ms: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set response time
    pub fn with_response_time(mut self, ms: u64) -> Self {
        self.response_time_ms = Some(ms);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get age of the result
    pub fn age(&self) -> std::time::Duration {
        std::time::SystemTime::now()
            .duration_since(self.timestamp)
            .unwrap_or_default()
    }

    /// Check if result is stale
    pub fn is_stale(&self, max_age: std::time::Duration) -> bool {
        self.age() > max_age
    }
}

impl std::fmt::Display for HealthCheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

/// Health status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HealthStatus {
    /// System is healthy
    Healthy,
    /// System is degraded
    Degraded,
    /// System is unhealthy
    Unhealthy,
    /// Health status is unknown
    Unknown,
}

impl HealthStatus {
    /// Check if healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// Check if unhealthy
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, Self::Unhealthy)
    }

    /// Check if unknown
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Get status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unhealthy => "unhealthy",
            Self::Unknown => "unknown",
        }
    }

    /// Get status score (higher is better)
    pub fn score(&self) -> u8 {
        match self {
            Self::Healthy => 100,
            Self::Degraded => 50,
            Self::Unhealthy => 0,
            Self::Unknown => 25,
        }
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Health statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthStats {
    /// Total number of health checks
    pub total_checks: u64,
    /// Number of healthy checks
    pub healthy_checks: u64,
    /// Number of unhealthy checks
    pub unhealthy_checks: u64,
    /// Number of unknown checks
    pub unknown_checks: u64,
    /// Current error count
    pub current_errors: u32,
    /// Average response time in milliseconds
    pub avg_response_time_ms: u64,
    /// Overall health score (0-100)
    pub health_score: u8,
}

impl std::fmt::Display for HealthStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HealthStats(checks: {}, healthy: {}, errors: {}, score: {})",
            self.total_checks,
            self.healthy_checks,
            self.current_errors,
            self.health_score
        )
    }
}
