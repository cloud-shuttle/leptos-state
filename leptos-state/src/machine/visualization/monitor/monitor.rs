//! Core state monitoring functionality

use crate::machine::visualization_events::{ErrorEvent, PerformanceEvent, StateChangeEvent};
use crate::machine::{Machine, MachineStateImpl};
use super::state_info::StateInfo;
use super::stats::MonitoringStats;

/// Real-time state monitor
pub struct StateMonitor<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> {
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

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> Clone for StateMonitor<C, E> {
    fn clone(&self) -> Self {
        Self {
            machine: self.machine.clone(),
            current_state: self.current_state.clone(),
            state_change_listeners: Vec::new(), // Can't clone trait objects
            error_listeners: Vec::new(), // Can't clone trait objects
            performance_listeners: Vec::new(), // Can't clone trait objects
            enabled: self.enabled,
            stats: self.stats,
        }
    }
}

// Manual Debug implementation for StateMonitor since it contains trait objects
impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> std::fmt::Debug for StateMonitor<C, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateMonitor")
            .field("machine", &self.machine)
            .field("current_state", &self.current_state)
            .field("state_change_listeners_count", &self.state_change_listeners.len())
            .field("error_listeners_count", &self.error_listeners.len())
            .field("performance_listeners_count", &self.performance_listeners.len())
            .field("enabled", &self.enabled)
            .field("stats", &self.stats)
            .finish()
    }
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> StateMonitor<C, E> {
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
        let initial_state = StateInfo::from_machine(&machine);
        self.machine = Some(machine);
        self.current_state = Some(initial_state);
        self
    }

    /// Enable monitoring
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable monitoring
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if monitoring is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
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

    /// Remove all listeners
    pub fn clear_listeners(&mut self) {
        self.state_change_listeners.clear();
        self.error_listeners.clear();
        self.performance_listeners.clear();
    }

    /// Notify state change listeners
    pub fn notify_state_change(&self, event: &StateChangeEvent<C, E>) {
        if !self.enabled {
            return;
        }

        self.stats.state_changes += 1;
        for listener in &self.state_change_listeners {
            listener(event);
        }
    }

    /// Notify error listeners
    pub fn notify_error(&self, event: &ErrorEvent) {
        if !self.enabled {
            return;
        }

        self.stats.errors += 1;
        for listener in &self.error_listeners {
            listener(event);
        }
    }

    /// Notify performance listeners
    pub fn notify_performance(&self, event: &PerformanceEvent) {
        if !self.enabled {
            return;
        }

        self.stats.performance_events += 1;
        for listener in &self.performance_listeners {
            listener(event);
        }
    }

    /// Update current state
    pub fn update_state(&mut self, new_state: MachineStateImpl<C>) {
        if let Some(ref mut current) = self.current_state {
            let old_state_value = current.state.value().to_string();
            current.state = new_state;
            current.last_updated = std::time::SystemTime::now();

            let new_state_value = current.state.value().to_string();
            if old_state_value != new_state_value {
                self.notify_state_change(&StateChangeEvent {
                    machine_id: current.machine_id.clone(),
                    from_state: old_state_value,
                    to_state: new_state_value,
                    timestamp: current.last_updated,
                    context: None,
                });
            }
        }
    }

    /// Record an event
    pub fn record_event(&mut self, event_type: &str) {
        match event_type {
            "transition" => self.stats.transitions += 1,
            "action" => self.stats.actions += 1,
            "guard" => self.stats.guards += 1,
            _ => {}
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> &MonitoringStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = MonitoringStats::default();
    }

    /// Get current state info
    pub fn get_current_state(&self) -> Option<&StateInfo<C, E>> {
        self.current_state.as_ref()
    }

    /// Check if the monitored machine is healthy
    pub fn is_healthy(&self) -> bool {
        if let Some(ref state_info) = self.current_state {
            matches!(state_info.status, super::state_info::StateStatus::Active)
        } else {
            false
        }
    }

    /// Get uptime
    pub fn uptime(&self) -> Option<std::time::Duration> {
        self.current_state.as_ref().and_then(|state| {
            std::time::SystemTime::now()
                .duration_since(state.created_at)
                .ok()
        })
    }

    /// Export monitoring data as JSON
    pub fn export_data(&self) -> serde_json::Value {
        serde_json::json!({
            "enabled": self.enabled,
            "stats": self.stats,
            "current_state": self.current_state,
            "healthy": self.is_healthy(),
            "uptime_seconds": self.uptime().map(|d| d.as_secs()).unwrap_or(0),
        })
    }
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> Default for StateMonitor<C, E> {
    fn default() -> Self {
        Self::new()
    }
}
