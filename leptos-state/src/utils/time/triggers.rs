//! Time-based triggers for event scheduling

use super::core::TimeUtils;

/// Time-based trigger for scheduling events
#[derive(Debug, Clone)]
pub struct TimeTrigger<F> {
    /// Trigger name
    pub name: String,
    /// Trigger schedule
    pub schedule: TriggerSchedule,
    /// Action to execute
    action: Option<F>,
    /// Last execution time
    last_execution: Option<std::time::Instant>,
    /// Next execution time
    next_execution: Option<std::time::Instant>,
    /// Execution count
    execution_count: u64,
    /// Whether trigger is enabled
    enabled: bool,
    /// Maximum executions (None for unlimited)
    max_executions: Option<u64>,
}

impl<F> TimeTrigger<F>
where
    F: Fn() + Send + Sync + 'static,
{
    /// Create a new time trigger
    pub fn new(name: String, schedule: TriggerSchedule, action: F) -> Self {
        let mut trigger = Self {
            name,
            schedule,
            action: Some(action),
            last_execution: None,
            next_execution: None,
            execution_count: 0,
            enabled: true,
            max_executions: None,
        };
        trigger.calculate_next_execution();
        trigger
    }

    /// Create a trigger that executes once at a specific time
    pub fn at_time(name: String, time: std::time::Instant, action: F) -> Self {
        Self::new(name, TriggerSchedule::AtTime(time), action)
    }

    /// Create a trigger that executes after a delay
    pub fn after_delay(name: String, delay: std::time::Duration, action: F) -> Self {
        Self::new(name, TriggerSchedule::AfterDelay(delay), action)
    }

    /// Create a trigger that executes at regular intervals
    pub fn every(name: String, interval: std::time::Duration, action: F) -> Self {
        Self::new(name, TriggerSchedule::Every(interval), action)
    }

    /// Check if the trigger should execute
    pub fn should_execute(&self) -> bool {
        if !self.enabled || self.action.is_none() {
            return false;
        }

        if self.has_reached_max_executions() {
            return false;
        }

        if let Some(next) = self.next_execution {
            TimeUtils::now() >= next
        } else {
            false
        }
    }

    /// Execute the trigger if ready
    pub fn execute_if_ready(&mut self) -> bool {
        if self.should_execute() {
            if let Some(ref action) = self.action {
                action();
                self.last_execution = Some(TimeUtils::now());
                self.execution_count += 1;
                self.calculate_next_execution();
            }
            true
        } else {
            false
        }
    }

    /// Force execute the trigger
    pub fn execute_now(&mut self) -> bool {
        if self.enabled && self.action.is_some() && !self.has_reached_max_executions() {
            if let Some(ref action) = self.action {
                action();
                self.last_execution = Some(TimeUtils::now());
                self.execution_count += 1;
                self.calculate_next_execution();
            }
            true
        } else {
            false
        }
    }

    /// Enable the trigger
    pub fn enable(&mut self) {
        self.enabled = true;
        self.calculate_next_execution();
    }

    /// Disable the trigger
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set maximum executions
    pub fn with_max_executions(mut self, max: u64) -> Self {
        self.max_executions = Some(max);
        self
    }

    /// Check if maximum executions reached
    pub fn has_reached_max_executions(&self) -> bool {
        self.max_executions.map_or(false, |max| self.execution_count >= max)
    }

    /// Calculate next execution time
    fn calculate_next_execution(&mut self) {
        if !self.enabled {
            self.next_execution = None;
            return;
        }

        self.next_execution = match &self.schedule {
            TriggerSchedule::AtTime(time) => {
                if self.execution_count == 0 {
                    Some(*time)
                } else {
                    None // One-time trigger
                }
            }
            TriggerSchedule::AfterDelay(delay) => {
                if self.execution_count == 0 {
                    Some(TimeUtils::now() + *delay)
                } else {
                    None // One-time trigger
                }
            }
            TriggerSchedule::Every(interval) => {
                let base_time = self.last_execution.unwrap_or_else(TimeUtils::now);
                Some(base_time + *interval)
            }
            TriggerSchedule::Cron(_) => {
                // Simplified: execute every hour for cron (would need proper cron parsing)
                let base_time = self.last_execution.unwrap_or_else(TimeUtils::now);
                Some(base_time + std::time::Duration::from_secs(3600))
            }
        };
    }

    /// Get time until next execution
    pub fn time_until_next(&self) -> Option<std::time::Duration> {
        self.next_execution.map(|next| {
            let now = TimeUtils::now();
            if now >= next {
                std::time::Duration::from_nanos(0)
            } else {
                next - now
            }
        })
    }

    /// Get execution count
    pub fn execution_count(&self) -> u64 {
        self.execution_count
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get trigger name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get schedule
    pub fn schedule(&self) -> &TriggerSchedule {
        &self.schedule
    }

    /// Get last execution time
    pub fn last_execution(&self) -> Option<std::time::Instant> {
        self.last_execution
    }

    /// Get next execution time
    pub fn next_execution(&self) -> Option<std::time::Instant> {
        self.next_execution
    }
}

impl<F> std::fmt::Display for TimeTrigger<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if !self.enabled {
            "disabled"
        } else if self.has_reached_max_executions() {
            "completed"
        } else {
            "active"
        };

        write!(f, "TimeTrigger '{}' ({}, {} executions", self.name, status, self.execution_count)?;

        if let Some(remaining) = self.time_until_next() {
            if remaining.as_nanos() > 0 {
                write!(f, ", next in {}", TimeUtils::format_duration(remaining))?;
            } else {
                write!(f, ", ready now")?;
            }
        }

        write!(f, ")")
    }
}

/// Trigger schedule types
#[derive(Debug, Clone)]
pub enum TriggerSchedule {
    /// Execute once at a specific time
    AtTime(std::time::Instant),
    /// Execute once after a delay
    AfterDelay(std::time::Duration),
    /// Execute repeatedly at intervals
    Every(std::time::Duration),
    /// Execute according to cron expression (simplified)
    Cron(String),
}

impl std::fmt::Display for TriggerSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AtTime(_) => write!(f, "at specific time"),
            Self::AfterDelay(duration) => write!(f, "after {}", TimeUtils::format_duration(*duration)),
            Self::Every(interval) => write!(f, "every {}", TimeUtils::format_duration(*interval)),
            Self::Cron(expr) => write!(f, "cron: {}", expr),
        }
    }
}

/// Trigger manager for coordinating multiple triggers
pub struct TriggerManager<F> {
    triggers: Vec<TimeTrigger<F>>,
}

impl<F> TriggerManager<F>
where
    F: Fn() + Send + Sync + 'static,
{
    /// Create a new trigger manager
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
        }
    }

    /// Add a trigger
    pub fn add_trigger(&mut self, trigger: TimeTrigger<F>) {
        self.triggers.push(trigger);
    }

    /// Create and add a trigger
    pub fn create_trigger(&mut self, name: String, schedule: TriggerSchedule, action: F) {
        self.add_trigger(TimeTrigger::new(name, schedule, action));
    }

    /// Execute all ready triggers
    pub fn execute_ready_triggers(&mut self) -> usize {
        let mut executed = 0;
        for trigger in &mut self.triggers {
            if trigger.execute_if_ready() {
                executed += 1;
            }
        }
        executed
    }

    /// Get triggers by name
    pub fn get_trigger(&self, name: &str) -> Option<&TimeTrigger<F>> {
        self.triggers.iter().find(|t| t.name() == name)
    }

    /// Get mutable trigger by name
    pub fn get_trigger_mut(&mut self, name: &str) -> Option<&mut TimeTrigger<F>> {
        self.triggers.iter_mut().find(|t| t.name() == name)
    }

    /// Enable trigger by name
    pub fn enable_trigger(&mut self, name: &str) -> bool {
        if let Some(trigger) = self.get_trigger_mut(name) {
            trigger.enable();
            true
        } else {
            false
        }
    }

    /// Disable trigger by name
    pub fn disable_trigger(&mut self, name: &str) -> bool {
        if let Some(trigger) = self.get_trigger_mut(name) {
            trigger.disable();
            true
        } else {
            false
        }
    }

    /// Remove trigger by name
    pub fn remove_trigger(&mut self, name: &str) -> bool {
        let initial_len = self.triggers.len();
        self.triggers.retain(|t| t.name() != name);
        self.triggers.len() < initial_len
    }

    /// Get active trigger count
    pub fn active_triggers(&self) -> usize {
        self.triggers.iter().filter(|t| t.is_enabled()).count()
    }

    /// Get ready trigger count
    pub fn ready_triggers(&self) -> usize {
        self.triggers.iter().filter(|t| t.should_execute()).count()
    }

    /// Get total execution count
    pub fn total_executions(&self) -> u64 {
        self.triggers.iter().map(|t| t.execution_count()).sum()
    }

    /// Clear all triggers
    pub fn clear(&mut self) {
        self.triggers.clear();
    }

    /// Get trigger names
    pub fn trigger_names(&self) -> Vec<&str> {
        self.triggers.iter().map(|t| t.name()).collect()
    }
}

impl<F> Default for TriggerManager<F> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for triggers
pub mod factories {
    use super::*;

    /// Create a trigger that executes once after N milliseconds
    pub fn after_ms<F>(name: String, ms: u64, action: F) -> TimeTrigger<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        TimeTrigger::after_delay(name, std::time::Duration::from_millis(ms), action)
    }

    /// Create a trigger that executes once after N seconds
    pub fn after_secs<F>(name: String, secs: u64, action: F) -> TimeTrigger<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        TimeTrigger::after_delay(name, std::time::Duration::from_secs(secs), action)
    }

    /// Create a trigger that executes every N milliseconds
    pub fn every_ms<F>(name: String, ms: u64, action: F) -> TimeTrigger<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        TimeTrigger::every(name, std::time::Duration::from_millis(ms), action)
    }

    /// Create a trigger that executes every N seconds
    pub fn every_secs<F>(name: String, secs: u64, action: F) -> TimeTrigger<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        TimeTrigger::every(name, std::time::Duration::from_secs(secs), action)
    }

    /// Create a trigger that executes every N minutes
    pub fn every_mins<F>(name: String, mins: u64, action: F) -> TimeTrigger<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        TimeTrigger::every(name, std::time::Duration::from_secs(mins * 60), action)
    }
}
