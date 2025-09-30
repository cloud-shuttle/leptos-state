//! Repeating timer functionality

use super::core::TimeUtils;

/// Repeating timer
pub struct RepeatingTimer<F> {
    /// Interval between executions
    pub interval: std::time::Duration,
    /// Action to execute
    action: Option<F>,
    /// Last execution time
    last_execution: std::time::Instant,
    /// Execution count
    execution_count: u64,
    /// Whether the timer is active
    active: bool,
    /// Maximum number of executions (None for unlimited)
    max_executions: Option<u64>,
}

impl<F> RepeatingTimer<F>
where
    F: Fn() + Send + Sync + 'static,
{
    /// Create a new repeating timer
    pub fn new(interval: std::time::Duration, action: F) -> Self {
        Self {
            interval,
            action: Some(action),
            last_execution: TimeUtils::now(),
            execution_count: 0,
            active: true,
            max_executions: None,
        }
    }

    /// Check if the timer should execute
    pub fn should_execute(&self) -> bool {
        self.active &&
        self.action.is_some() &&
        TimeUtils::has_elapsed(self.last_execution, self.interval) &&
        !self.has_reached_max_executions()
    }

    /// Execute the timer action if ready
    pub fn execute_if_ready(&mut self) -> bool {
        if self.should_execute() {
            if let Some(ref action) = self.action {
                action();
                self.last_execution = TimeUtils::now();
                self.execution_count += 1;
            }
            true
        } else {
            false
        }
    }

    /// Force execute the timer (ignore timing)
    pub fn execute_now(&mut self) -> bool {
        if self.active && self.action.is_some() && !self.has_reached_max_executions() {
            if let Some(ref action) = self.action {
                action();
                self.last_execution = TimeUtils::now();
                self.execution_count += 1;
            }
            true
        } else {
            false
        }
    }

    /// Start the timer
    pub fn start(&mut self) {
        self.active = true;
        self.last_execution = TimeUtils::now();
    }

    /// Stop the timer
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Pause the timer (stop but remember timing)
    pub fn pause(&mut self) {
        self.active = false;
    }

    /// Resume the timer
    pub fn resume(&mut self) {
        self.active = true;
    }

    /// Reset the timer
    pub fn reset(&mut self) {
        self.last_execution = TimeUtils::now();
        self.execution_count = 0;
    }

    /// Set maximum number of executions
    pub fn with_max_executions(mut self, max: u64) -> Self {
        self.max_executions = Some(max);
        self
    }

    /// Check if maximum executions reached
    pub fn has_reached_max_executions(&self) -> bool {
        self.max_executions.map_or(false, |max| self.execution_count >= max)
    }

    /// Get time until next execution
    pub fn time_until_next(&self) -> std::time::Duration {
        if !self.active {
            std::time::Duration::from_nanos(0) // Not active
        } else {
            let elapsed = self.last_execution.elapsed();
            if elapsed >= self.interval {
                std::time::Duration::from_nanos(0) // Ready now
            } else {
                self.interval - elapsed
            }
        }
    }

    /// Get execution count
    pub fn execution_count(&self) -> u64 {
        self.execution_count
    }

    /// Check if timer is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get interval
    pub fn interval(&self) -> std::time::Duration {
        self.interval
    }

    /// Get last execution time
    pub fn last_execution(&self) -> std::time::Instant {
        self.last_execution
    }

    /// Get uptime (time since first execution or creation)
    pub fn uptime(&self) -> std::time::Duration {
        self.last_execution.elapsed()
    }

    /// Get executions per second
    pub fn executions_per_second(&self) -> f64 {
        let uptime_secs = self.uptime().as_secs_f64();
        if uptime_secs > 0.0 {
            self.execution_count as f64 / uptime_secs
        } else {
            0.0
        }
    }

    /// Change interval
    pub fn set_interval(&mut self, interval: std::time::Duration) {
        self.interval = interval;
    }

    /// Replace the action
    pub fn set_action(&mut self, action: F) {
        self.action = Some(action);
    }

    /// Take the action (for ownership transfer)
    pub fn take_action(&mut self) -> Option<F> {
        self.action.take()
    }

    /// Get reference to action
    pub fn action(&self) -> Option<&F> {
        self.action.as_ref()
    }
}

impl<F> Drop for RepeatingTimer<F> {
    fn drop(&mut self) {
        // Ensure timer is stopped when dropped
        self.active = false;
    }
}

impl<F> std::fmt::Debug for RepeatingTimer<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepeatingTimer")
            .field("interval", &self.interval)
            .field("active", &self.active)
            .field("execution_count", &self.execution_count)
            .field("max_executions", &self.max_executions)
            .field("time_until_next", &self.time_until_next())
            .finish()
    }
}

impl<F> std::fmt::Display for RepeatingTimer<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.active {
            write!(f, "RepeatingTimer(stopped, {} executions)", self.execution_count)
        } else if self.has_reached_max_executions() {
            write!(f, "RepeatingTimer(completed, {} executions)", self.execution_count)
        } else {
            write!(
                f,
                "RepeatingTimer(active, {} executions, next in {})",
                self.execution_count,
                TimeUtils::format_duration(self.time_until_next())
            )
        }
    }
}

/// Timer manager for coordinating multiple timers
pub struct TimerManager<F> {
    timers: Vec<RepeatingTimer<F>>,
}

impl<F> TimerManager<F>
where
    F: Fn() + Send + Sync + 'static,
{
    /// Create a new timer manager
    pub fn new() -> Self {
        Self {
            timers: Vec::new(),
        }
    }

    /// Add a timer to the manager
    pub fn add_timer(&mut self, timer: RepeatingTimer<F>) {
        self.timers.push(timer);
    }

    /// Create and add a timer
    pub fn create_timer(&mut self, interval: std::time::Duration, action: F) {
        self.add_timer(RepeatingTimer::new(interval, action));
    }

    /// Execute all ready timers
    pub fn execute_ready_timers(&mut self) -> usize {
        let mut executed = 0;
        for timer in &mut self.timers {
            if timer.execute_if_ready() {
                executed += 1;
            }
        }
        executed
    }

    /// Get number of active timers
    pub fn active_timers(&self) -> usize {
        self.timers.iter().filter(|t| t.is_active()).count()
    }

    /// Get total execution count across all timers
    pub fn total_executions(&self) -> u64 {
        self.timers.iter().map(|t| t.execution_count()).sum()
    }

    /// Stop all timers
    pub fn stop_all(&mut self) {
        for timer in &mut self.timers {
            timer.stop();
        }
    }

    /// Clear all timers
    pub fn clear(&mut self) {
        self.timers.clear();
    }

    /// Get timers that are ready
    pub fn ready_timers(&self) -> Vec<usize> {
        self.timers
            .iter()
            .enumerate()
            .filter_map(|(i, timer)| if timer.should_execute() { Some(i) } else { None })
            .collect()
    }
}

impl<F> Default for TimerManager<F> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for timers
pub mod factories {
    use super::*;

    /// Create a timer that executes every N milliseconds
    pub fn every_ms<F>(ms: u64, action: F) -> RepeatingTimer<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        RepeatingTimer::new(std::time::Duration::from_millis(ms), action)
    }

    /// Create a timer that executes every N seconds
    pub fn every_secs<F>(secs: u64, action: F) -> RepeatingTimer<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        RepeatingTimer::new(std::time::Duration::from_secs(secs), action)
    }

    /// Create a timer that executes every N minutes
    pub fn every_mins<F>(mins: u64, action: F) -> RepeatingTimer<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        RepeatingTimer::new(std::time::Duration::from_secs(mins * 60), action)
    }

    /// Create a timer that executes every N hours
    pub fn every_hours<F>(hours: u64, action: F) -> RepeatingTimer<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        RepeatingTimer::new(std::time::Duration::from_secs(hours * 3600), action)
    }

    /// Create a timer with limited executions
    pub fn limited<F>(interval: std::time::Duration, max_executions: u64, action: F) -> RepeatingTimer<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        RepeatingTimer::new(interval, action).with_max_executions(max_executions)
    }
}
