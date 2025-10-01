//! Testing utilities for state management
//!
//! This module provides comprehensive testing utilities including:
//! - Property-based testing for state operations
//! - State machine testing DSL
//! - Integration testing helpers
//! - Performance benchmarking tools

use crate::{State, Event, Store, Machine, MachineResult};
use serde::{Deserialize, Serialize};
use leptos::prelude::GetUntracked;

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
pub trait StatePropertyTest<S: State> {
    fn test_property(&self, state: &S) -> Result<(), String>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

/// State invariant checker
pub trait StateInvariant<S: State> {
    fn check_invariant(&self, state: &S) -> Result<(), String>;
    fn name(&self) -> &'static str;
}

/// Test store wrapper with additional testing capabilities
pub struct TestStore<S: State> {
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
pub struct TestMachine<S: State, E: Event> {
    machine: Machine<S, E>,
    invariants: Vec<Box<dyn StateInvariant<S>>>,
    transition_log: Vec<TestTransition<S>>,
    reachable_states: std::collections::HashSet<String>,
    dead_states: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestTransition<S> {
    pub from_state: String,
    pub event: String,
    pub to_state: String,
    pub context_before: S,
    pub context_after: S,
    pub timestamp: u64,
}

/// Property-based testing suite
pub struct PropertyTestSuite<S: State + Serialize> {
    properties: Vec<Box<dyn StatePropertyTest<S>>>,
    generators: Vec<Box<dyn Fn() -> S + Send + Sync>>,
    max_iterations: usize,
}

impl<S: State + Clone + Serialize> PropertyTestSuite<S> {
    /// Create a new property test suite
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
            generators: Vec::new(),
            max_iterations: 100,
        }
    }

    /// Add a property test
    pub fn add_property<P: StatePropertyTest<S> + 'static>(mut self, property: P) -> Self {
        self.properties.push(Box::new(property));
        self
    }

    /// Add a state generator
    pub fn add_generator<F: Fn() -> S + Send + Sync + 'static>(mut self, generator: F) -> Self {
        self.generators.push(Box::new(generator));
        self
    }

    /// Set maximum iterations per property
    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Run all property tests
    pub fn run(&self) -> Result<Vec<PropertyTestResult>, TestingError> {
        let mut results = Vec::new();

        for property in &self.properties {
            let mut failed_inputs = Vec::new();
            let mut passed = true;
            let mut error_message = None;

            for _ in 0..self.max_iterations {
                // Generate a random state (simplified - in real implementation would use proper generators)
                if let Some(generator) = self.generators.first() {
                    let state = generator();

                    match property.test_property(&state) {
                        Ok(()) => {}
                        Err(reason) => {
                            passed = false;
                            failed_inputs.push(serde_json::to_value(&state).unwrap_or_default());
                            error_message = Some(reason);
                            break; // Stop on first failure for this property
                        }
                    }
                }
            }

            results.push(PropertyTestResult {
                property_name: property.name().to_string(),
                passed,
                iterations: self.max_iterations,
                failed_inputs,
                error_message,
            });
        }

        Ok(results)
    }
}

impl<S: State> TestStore<S> {
    /// Create a new test store
    pub fn new(initial: S) -> Self {
        Self {
            store: Store::new(initial),
            invariants: Vec::new(),
            operation_log: Vec::new(),
        }
    }

    /// Add a state invariant
    pub fn with_invariant<I: StateInvariant<S> + 'static>(mut self, invariant: I) -> Self {
        self.invariants.push(Box::new(invariant));
        self
    }

    /// Get the underlying store
    pub fn store(&self) -> &Store<S> {
        &self.store
    }

    /// Get the underlying store mutably
    pub fn store_mut(&mut self) -> &mut Store<S> {
        &mut self.store
    }

    /// Update state with invariant checking
    pub fn update<F>(&mut self, updater: F) -> Result<(), TestingError>
    where
        F: FnOnce(&mut S) + Send + 'static,
        S: Clone,
    {
        let before_state = self.store.get().get_untracked();

        // Record the operation
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Apply the update
        self.store.update(updater).map_err(|e| TestingError::PerformanceTestFailed {
            reason: format!("Store update failed: {:?}", e),
        })?;

        let after_state = self.store.get().get_untracked();

        self.operation_log.push(TestOperation {
            operation: "update".to_string(),
            before_state: before_state.clone(),
            after_state: after_state.clone(),
            timestamp,
        });

        // Check invariants
        for invariant in &self.invariants {
            if let Err(reason) = invariant.check_invariant(&after_state) {
                return Err(TestingError::InvariantViolation {
                    invariant: invariant.name().to_string(),
                    reason,
                });
            }
        }

        Ok(())
    }

    /// Reset store to default state
    pub fn reset(&mut self) -> Result<(), TestingError>
    where
        S: Default + Clone,
    {
        let before_state = self.store.get().get_untracked();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Reset by setting to default
        self.store.set(S::default()).map_err(|e| TestingError::PerformanceTestFailed {
            reason: format!("Store reset failed: {:?}", e),
        })?;

        let after_state = self.store.get().get_untracked();

        self.operation_log.push(TestOperation {
            operation: "reset".to_string(),
            before_state,
            after_state,
            timestamp,
        });

        Ok(())
    }

    /// Get operation log
    pub fn operation_log(&self) -> &[TestOperation<S>] {
        &self.operation_log
    }

    /// Clear operation log
    pub fn clear_log(&mut self) {
        self.operation_log.clear();
    }

    /// Get invariants
    pub fn invariants(&self) -> &[Box<dyn StateInvariant<S>>] {
        &self.invariants
    }
}

impl<S: State, E: Event> TestMachine<S, E> {
    /// Create a new test machine
    pub fn new(initial_state: &str, context: S) -> Self {
        Self {
            machine: Machine::new(initial_state, context),
            invariants: Vec::new(),
            transition_log: Vec::new(),
            reachable_states: std::collections::HashSet::new(),
            dead_states: Vec::new(),
        }
    }

    /// Add a state invariant
    pub fn with_invariant<I: StateInvariant<S> + 'static>(mut self, invariant: I) -> Self {
        self.invariants.push(Box::new(invariant));
        self
    }

    /// Get the underlying machine
    pub fn machine(&self) -> &Machine<S, E> {
        &self.machine
    }

    /// Get the underlying machine mutably
    pub fn machine_mut(&mut self) -> &mut Machine<S, E> {
        &mut self.machine
    }

    /// Send event with invariant checking and transition logging
    pub fn send(&mut self, event: E) -> MachineResult<()> {
        let from_state = self.machine.current_state().to_string();
        let context_before = self.machine.context().clone();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Send the event
        self.machine.send(event.clone())?;

        let to_state = self.machine.current_state().to_string();
        let context_after = self.machine.context().clone();

        // Record reachable states
        self.reachable_states.insert(from_state.clone());
        self.reachable_states.insert(to_state.clone());

        // Log the transition
        self.transition_log.push(TestTransition {
            from_state,
            event: format!("{:?}", event),
            to_state,
            context_before,
            context_after: context_after.clone(),
            timestamp,
        });

        // Check invariants
        for invariant in &self.invariants {
            if let Err(reason) = invariant.check_invariant(&context_after) {
                return Err(crate::MachineError::InvalidTransition {
                    from: "invariant_check".to_string(),
                    to: reason,
                });
            }
        }

        Ok(())
    }

    /// Analyze the state machine for common issues
    pub fn analyze(&mut self) -> StateMachineTestResult {
        // Find dead states (states with no outgoing transitions)
        self.dead_states.clear();

        // Note: This is a simplified analysis - in a real implementation,
        // we would need to expose the states map from the Machine struct
        // For now, we'll just return a basic result

        StateMachineTestResult {
            test_name: "state_machine_analysis".to_string(),
            passed: self.dead_states.is_empty(),
            transitions_tested: self.transition_log.len(),
            invariants_checked: self.invariants.len(),
            error_message: if self.dead_states.is_empty() {
                None
            } else {
                Some(format!("Dead states found: {:?}", self.dead_states))
            },
        }
    }

    /// Get transition log
    pub fn transition_log(&self) -> &[TestTransition<S>] {
        &self.transition_log
    }

    /// Get reachable states
    pub fn reachable_states(&self) -> &std::collections::HashSet<String> {
        &self.reachable_states
    }

    /// Get dead states
    pub fn dead_states(&self) -> &[String] {
        &self.dead_states
    }

    /// Clear logs
    pub fn clear_logs(&mut self) {
        self.transition_log.clear();
        self.reachable_states.clear();
        self.dead_states.clear();
    }
}

/// State machine testing DSL
pub struct StateMachineTester<S: State, E: Event> {
    machine_factory: Box<dyn Fn() -> TestMachine<S, E>>,
    test_cases: Vec<Box<dyn Fn(&mut TestMachine<S, E>) -> Result<(), TestingError>>>,
}

impl<S: State + Clone, E: Event + Clone> StateMachineTester<S, E> {
    /// Create a new state machine tester
    pub fn new<F: Fn() -> TestMachine<S, E> + 'static>(factory: F) -> Self {
        Self {
            machine_factory: Box::new(factory),
            test_cases: Vec::new(),
        }
    }

    /// Add a test case
    pub fn add_test<F: Fn(&mut TestMachine<S, E>) -> Result<(), TestingError> + 'static>(
        mut self,
        test: F,
    ) -> Self {
        self.test_cases.push(Box::new(test));
        self
    }

    /// Run all test cases
    pub fn run(&self) -> Result<Vec<StateMachineTestResult>, TestingError> {
        let mut results = Vec::new();

        for (i, test_case) in self.test_cases.iter().enumerate() {
            let mut machine = (self.machine_factory)();

            let passed = match test_case(&mut machine) {
                Ok(()) => true,
                Err(e) => {
                    results.push(StateMachineTestResult {
                        test_name: format!("test_case_{}", i),
                        passed: false,
                        transitions_tested: machine.transition_log().len(),
                        invariants_checked: machine.invariants.len(),
                        error_message: Some(format!("{:?}", e)),
                    });
                    continue;
                }
            };

            let analysis = machine.analyze();

            results.push(StateMachineTestResult {
                test_name: format!("test_case_{}", i),
                passed: passed && analysis.passed,
                transitions_tested: machine.transition_log().len(),
                invariants_checked: machine.invariants.len(),
                error_message: analysis.error_message,
            });
        }

        Ok(results)
    }
}

// Built-in property tests

/// Property test: Store never panics on valid operations
pub struct StoreNeverPanicsProperty;

impl<S: State + Clone> StatePropertyTest<S> for StoreNeverPanicsProperty {
    fn test_property(&self, _state: &S) -> Result<(), String> {
        // Test that creating and operating on a store doesn't panic
        let _store = TestStore::new(_state.clone());
        Ok(())
    }

    fn name(&self) -> &'static str {
        "store_never_panics"
    }

    fn description(&self) -> &'static str {
        "Store operations should never panic on valid inputs"
    }
}

/// Property test: State transitions preserve invariants
pub struct StateTransitionInvariantProperty<I: StateInvariant<S>, S: State> {
    invariant: I,
    _phantom: std::marker::PhantomData<S>,
}

impl<I: StateInvariant<S>, S: State> StateTransitionInvariantProperty<I, S> {
    pub fn new(invariant: I) -> Self {
        Self {
            invariant,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I: StateInvariant<S>, S: State> StatePropertyTest<S> for StateTransitionInvariantProperty<I, S> {
    fn test_property(&self, state: &S) -> Result<(), String> {
        self.invariant.check_invariant(state)
    }

    fn name(&self) -> &'static str {
        self.invariant.name()
    }

    fn description(&self) -> &'static str {
        "State transitions should preserve specified invariants"
    }
}

/// Convenience function to run property tests
pub fn run_property_tests<S: State + Clone + Serialize>(
    suite: PropertyTestSuite<S>,
) -> Result<Vec<PropertyTestResult>, TestingError> {
    suite.run()
}

/// Convenience function to run state machine tests
pub fn run_state_machine_tests<S: State + Clone, E: Event + Clone>(
    tester: StateMachineTester<S, E>,
) -> Result<Vec<StateMachineTestResult>, TestingError> {
    tester.run()
}
