//! Debugging tools for state machines

use super::visualization_data::MachineSnapshot;
use super::*;

/// Time travel debugger for state machines
#[derive(Debug)]
pub struct TimeTravelDebugger<C: Send + Sync + std::fmt::Debug, E: std::fmt::Debug> {
    /// Snapshots in chronological order
    pub snapshots: Vec<MachineSnapshot<C, E>>,
    /// Current position in time travel
    pub current_position: usize,
    /// Event history for replay
    pub event_history: Vec<TransitionEvent<C, E>>,
    /// Maximum number of snapshots to keep
    pub max_snapshots: usize,
}

impl<C: Send + Sync, E> TimeTravelDebugger<C, E> {
    /// Create a new time travel debugger
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            current_position: 0,
            event_history: Vec::new(),
            max_snapshots: 100,
        }
    }

    /// Add a snapshot
    pub fn add_snapshot(&mut self, snapshot: MachineSnapshot<C, E>) {
        self.snapshots.push(snapshot);

        // Keep snapshot count manageable
        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
            if self.current_position > 0 {
                self.current_position -= 1;
            }
        } else {
            self.current_position = self.snapshots.len() - 1;
        }
    }

    /// Add an event to the history
    pub fn add_event(&mut self, event: TransitionEvent<C, E>) {
        self.event_history.push(event);
    }

    /// Get current snapshot
    pub fn current_snapshot(&self) -> Option<&MachineSnapshot<C, E>> {
        self.snapshots.get(self.current_position)
    }

    /// Go to specific position
    pub fn go_to_position(&mut self, position: usize) -> Result<(), String> {
        if position >= self.snapshots.len() {
            return Err(format!(
                "Position {} is out of range (max: {})",
                position,
                self.snapshots.len() - 1
            ));
        }
        self.current_position = position;
        Ok(())
    }

    /// Go to next snapshot
    pub fn next(&mut self) -> Result<(), String> {
        if self.current_position >= self.snapshots.len() - 1 {
            return Err("Already at the latest snapshot".to_string());
        }
        self.current_position += 1;
        Ok(())
    }

    /// Go to previous snapshot
    pub fn previous(&mut self) -> Result<(), String> {
        if self.current_position == 0 {
            return Err("Already at the earliest snapshot".to_string());
        }
        self.current_position -= 1;
        Ok(())
    }

    /// Go to first snapshot
    pub fn go_to_first(&mut self) -> Result<(), String> {
        if self.snapshots.is_empty() {
            return Err("No snapshots available".to_string());
        }
        self.current_position = 0;
        Ok(())
    }

    /// Go to last snapshot
    pub fn go_to_last(&mut self) -> Result<(), String> {
        if self.snapshots.is_empty() {
            return Err("No snapshots available".to_string());
        }
        self.current_position = self.snapshots.len() - 1;
        Ok(())
    }

    /// Get position info
    pub fn position_info(&self) -> TimeTravelPosition {
        TimeTravelPosition {
            current: self.current_position,
            total: self.snapshots.len(),
            has_previous: self.current_position > 0,
            has_next: self.current_position < self.snapshots.len() - 1,
        }
    }

    /// Clear all snapshots and history
    pub fn clear(&mut self) {
        self.snapshots.clear();
        self.event_history.clear();
        self.current_position = 0;
    }
}

/// Time travel position information
#[derive(Debug, Clone, PartialEq)]
pub struct TimeTravelPosition {
    /// Current position (0-based)
    pub current: usize,
    /// Total number of snapshots
    pub total: usize,
    /// Whether there are previous snapshots
    pub has_previous: bool,
    /// Whether there are next snapshots
    pub has_next: bool,
}

impl TimeTravelPosition {
    /// Check if at the beginning
    pub fn at_start(&self) -> bool {
        self.current == 0
    }

    /// Check if at the end
    pub fn at_end(&self) -> bool {
        self.current == self.total.saturating_sub(1)
    }

    /// Get progress as percentage (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.current as f64 / (self.total - 1) as f64
        }
    }
}

/// Visualization statistics
#[derive(Debug, Clone)]
pub struct VisualizationStats {
    /// Total events recorded
    pub total_events: usize,
    /// Total transitions recorded
    pub total_transitions: usize,
    /// Total errors recorded
    pub total_errors: usize,
    /// Total performance events recorded
    pub total_performance_events: usize,
    /// Average event processing time
    pub avg_event_time: std::time::Duration,
    /// Peak memory usage
    pub peak_memory_usage: usize,
    /// State distribution (state -> count)
    pub state_distribution: std::collections::HashMap<String, usize>,
    /// Error distribution (error type -> count)
    pub error_distribution: std::collections::HashMap<ErrorEventType, usize>,
    /// Last updated timestamp
    pub last_updated: std::time::Instant,
}

impl Default for VisualizationStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            total_transitions: 0,
            total_errors: 0,
            total_performance_events: 0,
            avg_event_time: std::time::Duration::from_nanos(0),
            peak_memory_usage: 0,
            state_distribution: std::collections::HashMap::new(),
            error_distribution: std::collections::HashMap::new(),
            last_updated: std::time::Instant::now(),
        }
    }
}

impl VisualizationStats {
    /// Update statistics with new data
    pub fn update<C: Clone + Send + Sync, E: Clone>(&mut self, visualizer: &MachineVisualizer<C, E>) {
        self.total_events = visualizer.event_history.len();
        self.total_transitions = visualizer.state_history.len();
        self.total_errors = visualizer.error_log.len();
        self.total_performance_events = visualizer.performance_metrics.len();

        // Update state distribution
        self.state_distribution.clear();
        for event in &visualizer.state_history {
            *self
                .state_distribution
                .entry(event.new_state.clone())
                .or_insert(0) += 1;
        }

        // Update error distribution
        self.error_distribution.clear();
        for error in &visualizer.error_log {
            *self
                .error_distribution
                .entry(error.error_type.clone())
                .or_insert(0) += 1;
        }

        // Calculate average event time
        if !visualizer.performance_metrics.is_empty() {
            let total_time: std::time::Duration = visualizer
                .performance_metrics
                .iter()
                .map(|e| e.duration)
                .sum();
            self.avg_event_time = total_time / visualizer.performance_metrics.len() as u32;
        }

        // Update peak memory
        self.peak_memory_usage = visualizer
            .performance_metrics
            .iter()
            .map(|e| e.memory_after)
            .max()
            .unwrap_or(0);

        self.last_updated = std::time::Instant::now();
    }

    /// Get most frequent state
    pub fn most_frequent_state(&self) -> Option<&str> {
        self.state_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(state, _)| state.as_str())
    }

    /// Get most common error type
    pub fn most_common_error(&self) -> Option<&ErrorEventType> {
        self.error_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(error_type, _)| error_type)
    }

    /// Get error rate as percentage
    pub fn error_rate(&self) -> f64 {
        if self.total_events == 0 {
            0.0
        } else {
            (self.total_errors as f64 / self.total_events as f64) * 100.0
        }
    }

    /// Check if statistics are stale (older than 5 minutes)
    pub fn is_stale(&self) -> bool {
        self.last_updated.elapsed() > std::time::Duration::from_secs(300)
    }
}

/// Breakpoint system for debugging
pub struct Breakpoint<C: Send + Sync, E> {
    /// Breakpoint ID
    pub id: String,
    /// Breakpoint type
    pub breakpoint_type: BreakpointType<C, E>,
    /// Whether the breakpoint is enabled
    pub enabled: bool,
    /// Hit count
    pub hit_count: usize,
    /// Condition for the breakpoint
    pub condition: Option<Box<dyn Fn(&TransitionEvent<C, E>) -> bool + Send + Sync>>,
}

impl<C: Send + Sync, E> Clone for Breakpoint<C, E> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            breakpoint_type: self.breakpoint_type.clone(),
            enabled: self.enabled,
            hit_count: self.hit_count,
            condition: None, // Can't clone trait objects
        }
    }
}

// Manual Debug implementation for Breakpoint since it contains trait objects
impl<C: Send + Sync, E> std::fmt::Debug for Breakpoint<C, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Breakpoint")
            .field("id", &self.id)
            .field("breakpoint_type", &self.breakpoint_type)
            .field("enabled", &self.enabled)
            .field("hit_count", &self.hit_count)
            .field("has_condition", &self.condition.is_some())
            .field("last_updated", &self.last_updated)
            .finish()
    }
}

pub enum BreakpointType<C, E> {
    /// Break on state entry
    StateEntry(String),
    /// Break on state exit
    StateExit(String),
    /// Break on transition
    Transition { from: String, to: String },
    /// Break on event
    Event(String),
    /// Break on error
    Error(ErrorEventType),
    /// Break on guard failure
    GuardFailure,
    /// Custom breakpoint
    Custom(Box<dyn Fn(&TransitionEvent<C, E>) -> bool + Send + Sync>),
}

impl<C, E> Clone for BreakpointType<C, E> {
    fn clone(&self) -> Self {
        match self {
            Self::StateEntry(s) => Self::StateEntry(s.clone()),
            Self::StateExit(s) => Self::StateExit(s.clone()),
            Self::Transition { from, to } => Self::Transition {
                from: from.clone(),
                to: to.clone(),
            },
            Self::Event(s) => Self::Event(s.clone()),
            Self::Error(e) => Self::Error(*e),
            Self::GuardFailure => Self::GuardFailure,
            Self::Custom(_) => Self::GuardFailure, // Can't clone trait objects, fallback
        }
    }
}

// Manual Debug implementation for BreakpointType since it contains trait objects
impl<C, E> std::fmt::Debug for BreakpointType<C, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateEntry(s) => f.debug_tuple("StateEntry").field(s).finish(),
            Self::StateExit(s) => f.debug_tuple("StateExit").field(s).finish(),
            Self::Transition { from, to } => f.debug_struct("Transition").field("from", from).field("to", to).finish(),
            Self::Event(s) => f.debug_tuple("Event").field(s).finish(),
            Self::Error(e) => f.debug_tuple("Error").field(e).finish(),
            Self::GuardFailure => f.debug_tuple("GuardFailure").finish(),
            Self::Custom(_) => f.debug_tuple("Custom").field(&"<function>").finish(),
        }
    }
}

impl<C: Send + Sync, E> Breakpoint<C, E> {
    /// Create a new breakpoint
    pub fn new(id: String, breakpoint_type: BreakpointType<C, E>) -> Self {
        Self {
            id,
            breakpoint_type,
            enabled: true,
            hit_count: 0,
            condition: None,
        }
    }

    /// Enable or disable the breakpoint
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Add a condition
    pub fn with_condition<F>(mut self, condition: F) -> Self
    where
        F: Fn(&TransitionEvent<C, E>) -> bool + Send + Sync + 'static,
    {
        self.condition = Some(Box::new(condition));
        self
    }

    /// Check if breakpoint should trigger
    pub fn should_trigger(&self, event: &TransitionEvent<C, E>) -> bool {
        if !self.enabled {
            return false;
        }

        // Check condition first
        if let Some(ref condition) = self.condition {
            if !condition(event) {
                return false;
            }
        }

        // Check breakpoint type
        match &self.breakpoint_type {
            BreakpointType::StateEntry(state) => event.to_state == *state,
            BreakpointType::StateExit(state) => event.from_state == *state,
            BreakpointType::Transition { from, to } => {
                event.from_state == *from && event.to_state == *to
            }
            BreakpointType::Event(event_type) => {
                // Would need to check event type - simplified for now
                false
            }
            BreakpointType::Error(_) => !event.success,
            BreakpointType::GuardFailure => !event.all_guards_passed(),
            BreakpointType::Custom(checker) => checker(event),
        }
    }

    /// Increment hit count
    pub fn hit(&mut self) {
        self.hit_count += 1;
    }

    /// Reset hit count
    pub fn reset_hit_count(&mut self) {
        self.hit_count = 0;
    }
}
