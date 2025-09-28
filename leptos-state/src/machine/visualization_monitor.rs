//! Real-time monitoring for state machines

use super::*;

/// Real-time state monitor
pub struct StateMonitor<C: Send + Sync, E> {
    /// Monitored machine
    pub machine: Option<Machine<C, E, C>>,
    /// Current state information
    pub current_state: Option<StateInfo<C, E>>,
    /// State change listeners
    pub state_change_listeners: Vec<Box<dyn Fn(&StateChangeEvent<C, E>) + Send + Sync>>,
    /// Error listeners
    pub error_listeners: Vec<Box<dyn Fn(&ErrorEvent) + Send + Sync>>,
    /// Performance listeners
    pub performance_listeners: Vec<Box<dyn Fn(&PerformanceEvent) + Send + Sync>>,
    /// Monitoring enabled flag
    pub enabled: bool,
    /// Monitoring statistics
    pub stats: MonitoringStats,
}

impl<C: Send + Sync, E> StateMonitor<C, E> {
    /// Create a new state monitor
    pub fn new() -> Self {
        Self {
            machine: None,
            current_state: None,
            state_change_listeners: Vec::new(),
            error_listeners: Vec::new(),
            performance_listeners: Vec::new(),
            enabled: true,
            stats: MonitoringStats::default(),
        }
    }

    /// Set the machine to monitor
    pub fn with_machine(mut self, machine: Machine<C, E, C>) -> Self {
        self.machine = Some(machine);
        self
    }

    /// Add a state change listener
    pub fn add_state_change_listener<F>(&mut self, listener: F)
    where
        F: Fn(&StateChangeEvent<C, E>) + Send + Sync + 'static,
    {
        self.state_change_listeners.push(Box::new(listener));
    }

    /// Add an error listener
    pub fn add_error_listener<F>(&mut self, listener: F)
    where
        F: Fn(&ErrorEvent) + Send + Sync + 'static,
    {
        self.error_listeners.push(Box::new(listener));
    }

    /// Add a performance listener
    pub fn add_performance_listener<F>(&mut self, listener: F)
    where
        F: Fn(&PerformanceEvent) + Send + Sync + 'static,
    {
        self.performance_listeners.push(Box::new(listener));
    }

    /// Enable or disable monitoring
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Notify state change
    pub fn notify_state_change(&mut self, event: &StateChangeEvent<C, E>) {
        if !self.enabled {
            return;
        }

        self.stats.total_state_changes += 1;

        for listener in &self.state_change_listeners {
            listener(event);
        }
    }

    /// Notify error
    pub fn notify_error(&mut self, error: &ErrorEvent) {
        if !self.enabled {
            return;
        }

        self.stats.total_errors += 1;
        *self.stats.error_counts.entry(error.error_type.clone()).or_insert(0) += 1;

        for listener in &self.error_listeners {
            listener(error);
        }
    }

    /// Notify performance event
    pub fn notify_performance(&mut self, event: &PerformanceEvent) {
        if !self.enabled {
            return;
        }

        self.stats.total_performance_events += 1;

        for listener in &self.performance_listeners {
            listener(event);
        }
    }

    /// Get current monitoring statistics
    pub fn get_stats(&self) -> &MonitoringStats {
        &self.stats
    }

    /// Reset monitoring statistics
    pub fn reset_stats(&mut self) {
        self.stats = MonitoringStats::default();
    }

    /// Clear all listeners
    pub fn clear_listeners(&mut self) {
        self.state_change_listeners.clear();
        self.error_listeners.clear();
        self.performance_listeners.clear();
    }
}

/// Monitoring statistics
#[derive(Debug, Clone, Default)]
pub struct MonitoringStats {
    /// Total state changes observed
    pub total_state_changes: usize,
    /// Total errors observed
    pub total_errors: usize,
    /// Total performance events observed
    pub total_performance_events: usize,
    /// Error counts by type
    pub error_counts: std::collections::HashMap<ErrorEventType, usize>,
    /// Monitoring start time
    pub start_time: std::time::Instant,
}

impl MonitoringStats {
    /// Get monitoring duration
    pub fn duration(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Get state changes per second
    pub fn state_changes_per_second(&self) -> f64 {
        let duration_secs = self.duration().as_secs_f64();
        if duration_secs == 0.0 {
            0.0
        } else {
            self.total_state_changes as f64 / duration_secs
        }
    }

    /// Get error rate (errors per minute)
    pub fn error_rate_per_minute(&self) -> f64 {
        let duration_mins = self.duration().as_secs_f64() / 60.0;
        if duration_mins == 0.0 {
            0.0
        } else {
            self.total_errors as f64 / duration_mins
        }
    }
}

/// Real-time state information
#[derive(Debug, Clone)]
pub struct StateInfo<C: Send + Sync, E> {
    /// State name
    pub name: String,
    /// Entry timestamp
    pub entered_at: std::time::Instant,
    /// Exit timestamp (if state was exited)
    pub exited_at: Option<std::time::Instant>,
    /// Context when entering the state
    pub entry_context: Option<C>,
    /// Events received while in this state
    pub events_received: Vec<E>,
    /// Transitions attempted from this state
    pub transitions_attempted: usize,
    /// Transitions succeeded from this state
    pub transitions_succeeded: usize,
    /// Errors occurred while in this state
    pub errors_occurred: Vec<ErrorEvent>,
    /// Current status
    pub status: StateStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StateStatus {
    /// Currently active
    Active,
    /// Previously active, now exited
    Exited,
    /// Never entered
    NeverEntered,
}

impl<C: Send + Sync, E> StateInfo<C, E> {
    /// Create a new state info for a state that was just entered
    pub fn entered(name: String, context: Option<C>) -> Self {
        Self {
            name,
            entered_at: std::time::Instant::now(),
            exited_at: None,
            entry_context: context,
            events_received: Vec::new(),
            transitions_attempted: 0,
            transitions_succeeded: 0,
            errors_occurred: Vec::new(),
            status: StateStatus::Active,
        }
    }

    /// Mark the state as exited
    pub fn exit(&mut self) {
        self.exited_at = Some(std::time::Instant::now());
        self.status = StateStatus::Exited;
    }

    /// Record an event received
    pub fn record_event(&mut self, event: E) {
        self.events_received.push(event);
    }

    /// Record a transition attempt
    pub fn record_transition_attempt(&mut self, success: bool) {
        self.transitions_attempted += 1;
        if success {
            self.transitions_succeeded += 1;
        }
    }

    /// Record an error
    pub fn record_error(&mut self, error: ErrorEvent) {
        self.errors_occurred.push(error);
    }

    /// Get the time spent in this state
    pub fn time_in_state(&self) -> std::time::Duration {
        match self.status {
            StateStatus::Active => self.entered_at.elapsed(),
            StateStatus::Exited => {
                self.exited_at.unwrap().duration_since(self.entered_at)
            }
            StateStatus::NeverEntered => std::time::Duration::from_nanos(0),
        }
    }

    /// Get transition success rate
    pub fn transition_success_rate(&self) -> f64 {
        if self.transitions_attempted == 0 {
            0.0
        } else {
            self.transitions_succeeded as f64 / self.transitions_attempted as f64
        }
    }

    /// Check if this state has errors
    pub fn has_errors(&self) -> bool {
        !self.errors_occurred.is_empty()
    }

    /// Get the most recent error
    pub fn latest_error(&self) -> Option<&ErrorEvent> {
        self.errors_occurred.last()
    }
}

/// Health checker for state machines
pub struct HealthChecker<C: Send + Sync, E> {
    /// Health checks to perform
    pub checks: Vec<Box<dyn HealthCheck<C, E> + Send + Sync>>,
    /// Last health check results
    pub last_results: Vec<HealthCheckResult>,
    /// Health check interval
    pub check_interval: std::time::Duration,
    /// Last check time
    pub last_check: Option<std::time::Instant>,
}

impl<C: Send + Sync, E> HealthChecker<C, E> {
    /// Create a new health checker
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            last_results: Vec::new(),
            check_interval: std::time::Duration::from_secs(60), // Check every minute
            last_check: None,
        }
    }

    /// Add a health check
    pub fn add_check(&mut self, check: Box<dyn HealthCheck<C, E> + Send + Sync>) {
        self.checks.push(check);
    }

    /// Perform health checks
    pub fn perform_checks(&mut self, machine: &Machine<C, E, C>, monitor: &StateMonitor<C, E>) -> Vec<HealthCheckResult> {
        let now = std::time::Instant::now();

        // Check if enough time has passed since last check
        if let Some(last) = self.last_check {
            if now.duration_since(last) < self.check_interval {
                return self.last_results.clone();
            }
        }

        let mut results = Vec::new();

        for check in &self.checks {
            let result = check.perform_check(machine, monitor);
            results.push(result);
        }

        self.last_results = results.clone();
        self.last_check = Some(now);

        results
    }

    /// Check if the machine is healthy
    pub fn is_healthy(&self) -> bool {
        self.last_results.iter().all(|r| r.status == HealthStatus::Healthy)
    }

    /// Get overall health status
    pub fn overall_status(&self) -> HealthStatus {
        if self.last_results.is_empty() {
            HealthStatus::Unknown
        } else if self.last_results.iter().any(|r| r.status == HealthStatus::Critical) {
            HealthStatus::Critical
        } else if self.last_results.iter().any(|r| r.status == HealthStatus::Warning) {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        }
    }
}

/// Health check trait
pub trait HealthCheck<C: Send + Sync, E> {
    /// Perform a health check
    fn perform_check(&self, machine: &Machine<C, E, C>, monitor: &StateMonitor<C, E>) -> HealthCheckResult;

    /// Get the name of this health check
    fn name(&self) -> &str;
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// Check name
    pub check_name: String,
    /// Health status
    pub status: HealthStatus,
    /// Status message
    pub message: String,
    /// Additional details
    pub details: std::collections::HashMap<String, String>,
    /// Timestamp
    pub timestamp: std::time::Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    /// Everything is healthy
    Healthy,
    /// Warning condition
    Warning,
    /// Critical issue
    Critical,
    /// Unknown status
    Unknown,
}

impl HealthCheckResult {
    /// Create a healthy result
    pub fn healthy(check_name: String, message: String) -> Self {
        Self {
            check_name,
            status: HealthStatus::Healthy,
            message,
            details: std::collections::HashMap::new(),
            timestamp: std::time::Instant::now(),
        }
    }

    /// Create a warning result
    pub fn warning(check_name: String, message: String) -> Self {
        Self {
            check_name,
            status: HealthStatus::Warning,
            message,
            details: std::collections::HashMap::new(),
            timestamp: std::time::Instant::now(),
        }
    }

    /// Create a critical result
    pub fn critical(check_name: String, message: String) -> Self {
        Self {
            check_name,
            status: HealthStatus::Critical,
            message,
            details: std::collections::HashMap::new(),
            timestamp: std::time::Instant::now(),
        }
    }

    /// Add detail
    pub fn with_detail(mut self, key: String, value: String) -> Self {
        self.details.insert(key, value);
        self
    }
}
