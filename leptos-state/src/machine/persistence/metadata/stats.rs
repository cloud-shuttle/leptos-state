//! Machine statistics and metrics

/// Machine statistics
#[derive(Debug, Clone)]
pub struct MachineStats {
    /// Total transitions executed
    pub transitions_total: u64,
    /// Transitions per second
    pub transitions_per_second: f64,
    /// Total time spent in transitions (nanoseconds)
    pub transition_time_total: u64,
    /// Average transition time (nanoseconds)
    pub avg_transition_time: u64,
    /// Maximum transition time (nanoseconds)
    pub max_transition_time: u64,
    /// Minimum transition time (nanoseconds)
    pub min_transition_time: u64,
    /// Total errors encountered
    pub errors_total: u64,
    /// Error rate (errors per transition)
    pub error_rate: f64,
    /// Last transition timestamp
    pub last_transition_at: Option<u64>,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Memory usage estimate (bytes)
    pub memory_usage: u64,
    /// State count
    pub state_count: u32,
    /// Event count
    pub event_count: u32,
}

impl MachineStats {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
            transitions_total: 0,
            transitions_per_second: 0.0,
            transition_time_total: 0,
            avg_transition_time: 0,
            max_transition_time: 0,
            min_transition_time: u64::MAX,
            errors_total: 0,
            error_rate: 0.0,
            last_transition_at: None,
            uptime_seconds: 0,
            memory_usage: 0,
            state_count: 0,
            event_count: 0,
        }
    }

    /// Record a transition
    pub fn record_transition(&mut self, duration_ns: u64) {
        self.transitions_total += 1;
        self.transition_time_total += duration_ns;
        self.last_transition_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        );

        // Update min/max
        if duration_ns > self.max_transition_time {
            self.max_transition_time = duration_ns;
        }
        if duration_ns < self.min_transition_time {
            self.min_transition_time = duration_ns;
        }

        // Update averages
        self.update_averages();
        self.update_rates();
    }

    /// Record an error
    pub fn record_error(&mut self) {
        self.errors_total += 1;
        self.update_rates();
    }

    /// Update uptime
    pub fn update_uptime(&mut self, uptime_seconds: u64) {
        self.uptime_seconds = uptime_seconds;
        self.update_rates();
    }

    /// Set memory usage
    pub fn set_memory_usage(&mut self, memory_bytes: u64) {
        self.memory_usage = memory_bytes;
    }

    /// Set state and event counts
    pub fn set_counts(&mut self, state_count: u32, event_count: u32) {
        self.state_count = state_count;
        self.event_count = event_count;
    }

    /// Update average calculations
    fn update_averages(&mut self) {
        if self.transitions_total > 0 {
            self.avg_transition_time = self.transition_time_total / self.transitions_total;
        }
    }

    /// Update rate calculations
    fn update_rates(&mut self) {
        if self.transitions_total > 0 {
            self.error_rate = self.errors_total as f64 / self.transitions_total as f64;
        }

        if self.uptime_seconds > 0 {
            self.transitions_per_second = self.transitions_total as f64 / self.uptime_seconds as f64;
        }
    }

    /// Get transitions per minute
    pub fn transitions_per_minute(&self) -> f64 {
        self.transitions_per_second * 60.0
    }

    /// Get transitions per hour
    pub fn transitions_per_hour(&self) -> f64 {
        self.transitions_per_second * 3600.0
    }

    /// Get success rate (1.0 - error_rate)
    pub fn success_rate(&self) -> f64 {
        1.0 - self.error_rate
    }

    /// Get average transition time as duration
    pub fn avg_transition_duration(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.avg_transition_time)
    }

    /// Get max transition time as duration
    pub fn max_transition_duration(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.max_transition_time)
    }

    /// Get min transition time as duration
    pub fn min_transition_duration(&self) -> std::time::Duration {
        if self.min_transition_time == u64::MAX {
            std::time::Duration::from_nanos(0)
        } else {
            std::time::Duration::from_nanos(self.min_transition_time)
        }
    }

    /// Get total transition time as duration
    pub fn total_transition_duration(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.transition_time_total)
    }

    /// Get time since last transition
    pub fn time_since_last_transition(&self) -> Option<std::time::Duration> {
        self.last_transition_at.map(|timestamp| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            std::time::Duration::from_secs(now.saturating_sub(timestamp))
        })
    }

    /// Check if machine is active (had transitions recently)
    pub fn is_active(&self, threshold_seconds: u64) -> bool {
        self.time_since_last_transition()
            .map(|duration| duration.as_secs() < threshold_seconds)
            .unwrap_or(false)
    }

    /// Get performance score (0.0 to 1.0, higher is better)
    pub fn performance_score(&self) -> f64 {
        let success_weight = 0.6;
        let speed_weight = 0.4;

        let success_score = self.success_rate();
        let speed_score = if self.avg_transition_time > 0 {
            // Lower is better for transition time, normalize to 0-1 scale
            // Assume 1ms is excellent (score 1.0), 100ms is poor (score 0.0)
            let avg_ms = self.avg_transition_time as f64 / 1_000_000.0;
            (100.0 - avg_ms.min(100.0)) / 100.0
        } else {
            1.0
        };

        success_score * success_weight + speed_score * speed_weight
    }

    /// Get health status based on statistics
    pub fn health_status(&self) -> HealthStatus {
        let error_rate_threshold = 0.1; // 10%
        let avg_time_threshold_ns = 50_000_000; // 50ms

        if self.error_rate > error_rate_threshold || self.avg_transition_time > avg_time_threshold_ns {
            HealthStatus::Unhealthy
        } else if self.error_rate > error_rate_threshold * 0.5 || self.avg_transition_time > avg_time_threshold_ns / 2 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Get statistics summary
    pub fn summary(&self) -> String {
        format!(
            "MachineStats {{ transitions: {}, success: {:.1}%, avg_time: {:.2}ms, health: {} }}",
            self.transitions_total,
            self.success_rate() * 100.0,
            self.avg_transition_time as f64 / 1_000_000.0,
            self.health_status()
        )
    }
}

impl Default for MachineStats {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MachineStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Health status of the machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Machine is healthy
    Healthy,
    /// Machine is degraded but functional
    Degraded,
    /// Machine is unhealthy
    Unhealthy,
}

impl HealthStatus {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unhealthy => "unhealthy",
        }
    }

    /// Check if healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// Check if degraded
    pub fn is_degraded(&self) -> bool {
        matches!(self, Self::Degraded)
    }

    /// Check if unhealthy
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, Self::Unhealthy)
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
