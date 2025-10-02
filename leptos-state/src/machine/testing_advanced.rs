use crate::{Store, Machine};
use serde::{Deserialize, Serialize};

/// Errors that can occur during testing
#[derive(Debug, Clone, thiserror::Error)]
pub enum TestingError {
    #[error("Property test failed: {property} - {reason}")]
    PropertyTestFailed { property: String, reason: String },
    #[error("State machine test failed: {test} - {reason}")]
    StateMachineTestFailed { test: String, reason: String },
    #[error("Invariant violation: {invariant} - {reason}")]
    InvariantViolation { invariant: String, reason: String },
    #[error("Performance test failed: {reason}")]
    PerformanceTestFailed { reason: String },
}

/// Test result for property-based testing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyTestResult {
    pub property_name: String,
    pub passed: bool,
    pub iterations: usize,
    pub failed_inputs: Vec<serde_json::Value>,
    pub error_message: Option<String>,
}

/// Test result for state machine testing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateMachineTestResult {
    pub test_name: String,
    pub passed: bool,
    pub transitions_tested: usize,
    pub invariants_checked: usize,
    pub error_message: Option<String>,
}

/// Property test trait for state operations
pub trait StatePropertyTest<S> where S: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static {
    fn test_property(&self, state: &S) -> Result<(), String>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

/// State invariant checker
pub trait StateInvariant<S> where S: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static {
    fn check_invariant(&self, state: &S) -> Result<(), String>;
    fn name(&self) -> &'static str;
}

/// Test store wrapper with additional testing capabilities
pub struct TestStore<S> where S: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static {
    store: Store<S>,
    invariants: Vec<Box<dyn StateInvariant<S>>>,
    operation_log: Vec<TestOperation<S>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestOperation<S> {
    pub operation: String,
    pub before_state: S,
    pub after_state: S,
    pub timestamp: u64,
}

/// Test machine wrapper with state machine testing capabilities
pub struct TestMachine<C, E> where C: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static, E: Clone + std::fmt::Debug + Send + Sync + 'static {
    machine: Machine<C, E, C>,
    invariants: Vec<Box<dyn StateInvariant<C>>>,
    transition_log: Vec<TestTransition<C>>,
    reachable_states: std::collections::HashSet<String>,
    dead_states: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestTransition<C> {
    pub from_state: String,
    pub event: String,
    pub to_state: String,
    pub context_before: C,
    pub context_after: C,
    pub timestamp: u64,
}

/// Property-based testing suite
pub struct PropertyTestSuite<S> where S: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static + Serialize {
    properties: Vec<Box<dyn StatePropertyTest<S>>>,
    generators: Vec<Box<dyn Fn() -> S + Send + Sync>>,
    max_iterations: usize,
}

impl<S> PropertyTestSuite<S> where S: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static + Serialize {
    /// Create a new property test suite
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
            generators: Vec::new(),
            max_iterations: 100,
        }
    }
}

impl<S where S: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static> TestStore<S> {
    /// Create a new test store
    pub fn new(initial: S) -> Self {
        Self {
            store: Store::new(initial),
            invariants: Vec::new(),
            operation_log: Vec::new(),
        }
    }
}

impl<C, E> TestMachine<C, E> where C: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static, E: Clone + std::fmt::Debug + Send + Sync + 'static {
    /// Create a new test machine
    pub fn new(initial_state: &str, context: C) -> Self {
        Self {
            machine: Machine::new(initial_state, context),
            invariants: Vec::new(),
            transition_log: Vec::new(),
            reachable_states: std::collections::HashSet::new(),
            dead_states: Vec::new(),
        }
    }
}
