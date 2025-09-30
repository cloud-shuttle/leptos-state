//! Event structures for visualization

use super::*;

/// State transition event for visualization
#[derive(Debug, Clone)]
pub struct TransitionEvent<C, E> {
    /// Source state name
    pub from_state: String,
    /// Target state name
    pub to_state: String,
    /// Event that triggered the transition
    pub event: Option<E>,
    /// Context at the time of transition
    pub context: Option<C>,
    /// Guards that were evaluated
    pub guard_results: Vec<GuardResult>,
    /// Actions that were executed
    pub action_results: Vec<ActionResult>,
    /// Timestamp of the transition
    pub timestamp: std::time::Instant,
    /// Whether the transition succeeded
    pub success: bool,
    /// Error message if transition failed
    pub error_message: Option<String>,
}

impl<C, E> TransitionEvent<C, E> {
    /// Create a new successful transition event
    pub fn success(
        from_state: String,
        to_state: String,
        event: Option<E>,
        context: Option<C>,
    ) -> Self {
        Self {
            from_state,
            to_state,
            event,
            context,
            guard_results: Vec::new(),
            action_results: Vec::new(),
            timestamp: std::time::Instant::now(),
            success: true,
            error_message: None,
        }
    }

    /// Create a new failed transition event
    pub fn failure(
        from_state: String,
        to_state: String,
        event: Option<E>,
        context: Option<C>,
        error_message: String,
    ) -> Self {
        Self {
            from_state,
            to_state,
            event,
            context,
            guard_results: Vec::new(),
            action_results: Vec::new(),
            timestamp: std::time::Instant::now(),
            success: false,
            error_message: Some(error_message),
        }
    }

    /// Add a guard result
    pub fn with_guard_result(mut self, result: GuardResult) -> Self {
        self.guard_results.push(result);
        self
    }

    /// Add multiple guard results
    pub fn with_guard_results(mut self, results: Vec<GuardResult>) -> Self {
        self.guard_results.extend(results);
        self
    }

    /// Add an action result
    pub fn with_action_result(mut self, result: ActionResult) -> Self {
        self.action_results.push(result);
        self
    }

    /// Add multiple action results
    pub fn with_action_results(mut self, results: Vec<ActionResult>) -> Self {
        self.action_results.extend(results);
        self
    }

    /// Get the duration since the transition occurred
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
    }

    /// Check if all guards passed
    pub fn all_guards_passed(&self) -> bool {
        self.guard_results.iter().all(|r| r.passed)
    }

    /// Check if all actions succeeded
    pub fn all_actions_succeeded(&self) -> bool {
        self.action_results.iter().all(|r| r.success)
    }
}

/// Guard evaluation result for visualization
#[derive(Debug, Clone)]
pub struct GuardResult {
    /// Guard description
    pub description: String,
    /// Whether the guard passed
    pub passed: bool,
    /// Evaluation duration
    pub duration: std::time::Duration,
    /// Error message if evaluation failed
    pub error_message: Option<String>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl GuardResult {
    /// Create a successful guard result
    pub fn success(description: String, duration: std::time::Duration) -> Self {
        Self {
            description,
            passed: true,
            duration,
            error_message: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a failed guard result
    pub fn failure(description: String, duration: std::time::Duration) -> Self {
        Self {
            description,
            passed: false,
            duration,
            error_message: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a guard result with error
    pub fn error(description: String, duration: std::time::Duration, error: String) -> Self {
        Self {
            description,
            passed: false,
            duration,
            error_message: Some(error),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Action execution result for visualization
#[derive(Debug, Clone)]
pub struct ActionResult {
    /// Action description
    pub description: String,
    /// Whether the action succeeded
    pub success: bool,
    /// Execution duration
    pub duration: std::time::Duration,
    /// Error message if execution failed
    pub error_message: Option<String>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl ActionResult {
    /// Create a successful action result
    pub fn success(description: String, duration: std::time::Duration) -> Self {
        Self {
            description,
            success: true,
            duration,
            error_message: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a failed action result
    pub fn failure(description: String, duration: std::time::Duration) -> Self {
        Self {
            description,
            success: false,
            duration,
            error_message: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create an action result with error
    pub fn error(description: String, duration: std::time::Duration, error: String) -> Self {
        Self {
            description,
            success: false,
            duration,
            error_message: Some(error),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// State change event for monitoring
#[derive(Debug, Clone)]
pub struct StateChangeEvent<C, E> {
    /// Previous state
    pub previous_state: String,
    /// New state
    pub new_state: String,
    /// Event that caused the change
    pub triggering_event: Option<E>,
    /// Context when the change occurred
    pub context: Option<C>,
    /// Timestamp of the change
    pub timestamp: std::time::Instant,
    /// Change type
    pub change_type: StateChangeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StateChangeType {
    /// Normal state transition
    Transition,
    /// State machine initialization
    Initialization,
    /// State machine reset
    Reset,
    /// Error recovery
    ErrorRecovery,
    /// Manual state change
    Manual,
}

impl<C, E> StateChangeEvent<C, E> {
    /// Create a new state change event
    pub fn new(previous_state: String, new_state: String, change_type: StateChangeType) -> Self {
        Self {
            previous_state,
            new_state,
            triggering_event: None,
            context: None,
            timestamp: std::time::Instant::now(),
            change_type,
        }
    }

    /// Add triggering event
    pub fn with_event(mut self, event: E) -> Self {
        self.triggering_event = Some(event);
        self
    }

    /// Add context
    pub fn with_context(mut self, context: C) -> Self {
        self.context = Some(context);
        self
    }

    /// Get the age of this event
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
    }
}

/// Performance event for monitoring
#[derive(Debug, Clone)]
pub struct PerformanceEvent {
    /// Event type
    pub event_type: PerformanceEventType,
    /// Duration of the operation
    pub duration: std::time::Duration,
    /// Memory usage before operation
    pub memory_before: usize,
    /// Memory usage after operation
    pub memory_after: usize,
    /// Timestamp
    pub timestamp: std::time::Instant,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceEventType {
    /// State transition
    Transition,
    /// Guard evaluation
    GuardEvaluation,
    /// Action execution
    ActionExecution,
    /// State lookup
    StateLookup,
    /// Serialization
    Serialization,
    /// Deserialization
    Deserialization,
}

impl PerformanceEvent {
    /// Create a new performance event
    pub fn new(event_type: PerformanceEventType, duration: std::time::Duration) -> Self {
        Self {
            event_type,
            duration,
            memory_before: 0,
            memory_after: 0,
            timestamp: std::time::Instant::now(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add memory usage information
    pub fn with_memory(mut self, before: usize, after: usize) -> Self {
        self.memory_before = before;
        self.memory_after = after;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get memory delta
    pub fn memory_delta(&self) -> isize {
        self.memory_after as isize - self.memory_before as isize
    }
}

/// Error event for monitoring
#[derive(Debug, Clone)]
pub struct ErrorEvent {
    /// Error type
    pub error_type: ErrorEventType,
    /// Error message
    pub message: String,
    /// State when error occurred
    pub state: String,
    /// Timestamp
    pub timestamp: std::time::Instant,
    /// Stack trace if available
    pub stack_trace: Option<String>,
    /// Additional context
    pub context: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorEventType {
    /// Guard evaluation error
    GuardError,
    /// Action execution error
    ActionError,
    /// State transition error
    TransitionError,
    /// Serialization error
    SerializationError,
    /// Configuration error
    ConfigurationError,
    /// Internal error
    InternalError,
}

impl ErrorEvent {
    /// Create a new error event
    pub fn new(error_type: ErrorEventType, message: String, state: String) -> Self {
        Self {
            error_type,
            message,
            state,
            timestamp: std::time::Instant::now(),
            stack_trace: None,
            context: std::collections::HashMap::new(),
        }
    }

    /// Add stack trace
    pub fn with_stack_trace(mut self, trace: String) -> Self {
        self.stack_trace = Some(trace);
        self
    }

    /// Add context information
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }

    /// Get the age of this error event
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
    }
}
