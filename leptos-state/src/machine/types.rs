use std::collections::HashMap;
use std::fmt::Debug;

/// Common types and enums used across the machine module
pub type StateValue = String;

/// Result type for machine operations
pub type MachineResult<T> = Result<T, MachineError>;

/// Errors that can occur during machine operations
#[derive(Debug, Clone)]
pub enum MachineError {
    InvalidState(String),
    InvalidTransition,
    GuardFailed(String),
    MissingGuard(String),
    MissingAction(String),
    ContextError(String),
    StateNotFound(String),
    EventNotFound(String),
    CircularDependency(String),
}

impl std::fmt::Display for MachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineError::InvalidState(s) => write!(f, "Invalid state: {}", s),
            MachineError::InvalidTransition => write!(f, "Invalid transition"),
            MachineError::GuardFailed(s) => write!(f, "Guard failed: {}", s),
            MachineError::MissingGuard(s) => write!(f, "Missing guard: {}", s),
            MachineError::MissingAction(s) => write!(f, "Missing action: {}", s),
            MachineError::ContextError(s) => write!(f, "Context error: {}", s),
            MachineError::StateNotFound(s) => write!(f, "State not found: {}", s),
            MachineError::EventNotFound(s) => write!(f, "Event not found: {}", s),
            MachineError::CircularDependency(s) => write!(f, "Circular dependency: {}", s),
        }
    }
}

impl std::error::Error for MachineError {}

/// State types in the state machine hierarchy
#[derive(Debug, Clone, PartialEq)]
pub enum StateType {
    Atomic,
    Compound,
    Parallel,
    History,
    Final,
}

/// Machine configuration options
#[derive(Clone, Debug)]
pub struct MachineConfig {
    pub strict_mode: bool,
    pub auto_cleanup: bool,
    pub max_history_size: usize,
    pub enable_guards: bool,
    pub enable_actions: bool,
}

impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            auto_cleanup: true,
            max_history_size: 100,
            enable_guards: true,
            enable_actions: true,
        }
    }
}

/// Context for machine execution
#[derive(Clone, Debug, Default)]
pub struct Context {
    pub data: HashMap<String, ContextValue>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T: Into<ContextValue>>(&mut self, key: String, value: T) {
        self.data.insert(key, value.into());
    }

    pub fn get(&self, key: &str) -> Option<&ContextValue> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut ContextValue> {
        self.data.get_mut(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<ContextValue> {
        self.data.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Values that can be stored in machine context
#[derive(Clone, Debug, PartialEq)]
pub enum ContextValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ContextValue>),
    Object(HashMap<String, ContextValue>),
}

impl From<String> for ContextValue {
    fn from(s: String) -> Self {
        ContextValue::String(s)
    }
}

impl From<&str> for ContextValue {
    fn from(s: &str) -> Self {
        ContextValue::String(s.to_string())
    }
}

impl From<f64> for ContextValue {
    fn from(n: f64) -> Self {
        ContextValue::Number(n)
    }
}

impl From<i32> for ContextValue {
    fn from(n: i32) -> Self {
        ContextValue::Number(n as f64)
    }
}

impl From<bool> for ContextValue {
    fn from(b: bool) -> Self {
        ContextValue::Boolean(b)
    }
}

impl From<Vec<ContextValue>> for ContextValue {
    fn from(v: Vec<ContextValue>) -> Self {
        ContextValue::Array(v)
    }
}

impl From<HashMap<String, ContextValue>> for ContextValue {
    fn from(m: HashMap<String, ContextValue>) -> Self {
        ContextValue::Object(m)
    }
}

/// History entry for machine transitions
#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub from_state: String,
    pub to_state: String,
    pub event: String,
    pub timestamp: std::time::SystemTime,
    pub context_snapshot: Context,
}

/// Machine history tracking
#[derive(Clone, Debug, Default)]
pub struct MachineHistory {
    pub entries: Vec<HistoryEntry>,
    pub max_size: usize,
}

impl MachineHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
        }
    }

    pub fn record_transition(
        &mut self,
        from_state: String,
        to_state: String,
        event: String,
        context: &Context,
    ) {
        let entry = HistoryEntry {
            from_state,
            to_state,
            event,
            timestamp: std::time::SystemTime::now(),
            context_snapshot: context.clone(),
        };

        self.entries.push(entry);

        // Trim history if it exceeds max size
        if self.entries.len() > self.max_size {
            let excess = self.entries.len() - self.max_size;
            self.entries.drain(0..excess);
        }
    }

    pub fn get_entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn get_latest(&self) -> Option<&HistoryEntry> {
        self.entries.last()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Event routing configuration
#[derive(Clone, Debug)]
pub struct EventRoutingConfig {
    pub priority: i32,
    pub async_processing: bool,
    pub retry_count: u32,
    pub timeout_ms: u64,
}

impl Default for EventRoutingConfig {
    fn default() -> Self {
        Self {
            priority: 0,
            async_processing: false,
            retry_count: 3,
            timeout_ms: 5000,
        }
    }
}

/// State validation configuration
#[derive(Clone, Debug)]
pub struct StateValidationConfig {
    pub validate_on_entry: bool,
    pub validate_on_exit: bool,
    pub validate_transitions: bool,
    pub strict_validation: bool,
}

impl Default for StateValidationConfig {
    fn default() -> Self {
        Self {
            validate_on_entry: true,
            validate_on_exit: true,
            validate_transitions: true,
            strict_validation: false,
        }
    }
}

/// Performance monitoring configuration
#[derive(Clone, Debug)]
pub struct PerformanceConfig {
    pub enable_metrics: bool,
    pub enable_profiling: bool,
    pub metrics_interval_ms: u64,
    pub max_samples: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_metrics: false,
            enable_profiling: false,
            metrics_interval_ms: 1000,
            max_samples: 1000,
        }
    }
}

/// Integration configuration for external systems
#[derive(Clone, Debug)]
pub struct IntegrationConfig {
    pub enable_external_events: bool,
    pub enable_state_sync: bool,
    pub sync_interval_ms: u64,
    pub event_routing: EventRoutingConfig,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enable_external_events: false,
            enable_state_sync: false,
            sync_interval_ms: 1000,
            event_routing: EventRoutingConfig::default(),
        }
    }
}

/// Machine configuration combining all settings
#[derive(Clone, Debug)]
pub struct CompleteMachineConfig {
    pub machine: MachineConfig,
    pub validation: StateValidationConfig,
    pub performance: PerformanceConfig,
    pub integration: IntegrationConfig,
}

impl Default for CompleteMachineConfig {
    fn default() -> Self {
        Self {
            machine: MachineConfig::default(),
            validation: StateValidationConfig::default(),
            performance: PerformanceConfig::default(),
            integration: IntegrationConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_operations_work() {
        let mut context = Context::new();
        context.set("key1".to_string(), "value1".to_string());
        context.set("key2".to_string(), 42.0);

        assert_eq!(context.get("key1"), Some(&ContextValue::String("value1".to_string())));
        assert_eq!(context.get("key2"), Some(&ContextValue::Number(42.0)));
        assert_eq!(context.len(), 2);
        assert!(!context.is_empty());
    }

    #[test]
    fn context_value_conversions_work() {
        let string_val: ContextValue = "hello".into();
        let number_val: ContextValue = 42.0.into();
        let bool_val: ContextValue = true.into();

        match (string_val, number_val, bool_val) {
            (ContextValue::String(s), ContextValue::Number(n), ContextValue::Boolean(b)) => {
                assert_eq!(s, "hello");
                assert_eq!(n, 42.0);
                assert!(b);
            }
            _ => panic!("Type conversion failed"),
        }
    }

    #[test]
    fn machine_history_tracking_works() {
        let mut history = MachineHistory::with_max_size(3);

        history.record_transition(
            "state1".to_string(),
            "state2".to_string(),
            "event1".to_string(),
            &Context::new(),
        );

        history.record_transition(
            "state2".to_string(),
            "state3".to_string(),
            "event2".to_string(),
            &Context::new(),
        );

        assert_eq!(history.len(), 2);

        let latest = history.get_latest().unwrap();
        assert_eq!(latest.from_state, "state2");
        assert_eq!(latest.to_state, "state3");
    }

    #[test]
    fn machine_history_trimming_works() {
        let mut history = MachineHistory::with_max_size(2);

        history.record_transition("s1".to_string(), "s2".to_string(), "e1".to_string(), &Context::new());
        history.record_transition("s2".to_string(), "s3".to_string(), "e2".to_string(), &Context::new());
        history.record_transition("s3".to_string(), "s4".to_string(), "e3".to_string(), &Context::new());

        // Should only keep the last 2 entries
        assert_eq!(history.len(), 2);
        let entries = history.get_entries();
        assert_eq!(entries[0].from_state, "s2");
        assert_eq!(entries[1].from_state, "s3");
    }
}
