# Testing Utilities Design

## Overview
Implement comprehensive testing utilities for state management, including property-based testing, state machine testing DSL, integration testing helpers, and performance benchmarking tools.

## Current State
```rust
// Basic unit tests only
#[test]
fn store_basic_functionality() {
    let store = Store::new(0);
    store.update(|s| *s = 42).unwrap();
    assert_eq!(store.get().get_untracked(), 42);
}
```

## Proposed Enhancement
```rust
#[cfg(feature = "testing")]
use leptos_state_minimal::testing::*;

#[test]
fn store_property_based_test() {
    property_test_store(|store: TestStore<i32>| {
        // Property-based testing for stores
        prop_assert!(store.get().get_untracked() >= 0);
    });
}

#[test]
fn state_machine_model_checking() {
    let machine = create_test_machine();
    model_check_machine(machine, |model| {
        // Verify state machine properties
        prop_assert!(model.reachable_states().len() > 0);
        prop_assert!(model.has_no_deadlocks());
    });
}
```

## Motivation

### Quality Assurance
- **Property-Based Testing**: Test stateful systems with generated inputs
- **Model Checking**: Verify state machine correctness and invariants
- **Integration Testing**: Test complete state management workflows
- **Performance Testing**: Benchmark state operations under load

### Developer Productivity
- **Testing DSL**: Expressive, readable test specifications
- **Debugging Tools**: Better test failure diagnostics
- **Test Generation**: Automated test case generation
- **CI/CD Integration**: Comprehensive test reporting

### Use Cases
- Verifying state machine invariants and properties
- Testing complex state transitions and edge cases
- Performance regression testing
- Integration testing across multiple stores/machines
- Property-based testing of state operations

## Implementation Details

### Property-Based Testing Framework
```rust
#[cfg(feature = "testing")]
pub mod property_testing {
    use super::*;
    use proptest::prelude::*;

    pub trait StatePropertyTest<S: State>: Send + Sync {
        fn test_property(&self, state: &S) -> bool;
        fn name(&self) -> &'static str;
        fn description(&self) -> &'static str;
    }

    pub struct PropertyTestSuite<S: State> {
        properties: Vec<Box<dyn StatePropertyTest<S>>>,
        generators: Vec<Box<dyn Fn() -> S + Send + Sync>>,
    }

    impl<S: State> PropertyTestSuite<S> {
        pub fn new() -> Self {
            Self {
                properties: Vec::new(),
                generators: Vec::new(),
            }
        }

        pub fn add_property<P: StatePropertyTest<S> + 'static>(mut self, property: P) -> Self {
            self.properties.push(Box::new(property));
            self
        }

        pub fn add_generator<G: Fn() -> S + Send + Sync + 'static>(mut self, generator: G) -> Self {
            self.generators.push(Box::new(generator));
            self
        }

        pub fn run(&self, cases: u32) -> PropertyTestResult {
            let mut results = Vec::new();

            for case in 0..cases {
                // Generate test state
                let state = if let Some(generator) = self.generators.get(case as usize % self.generators.len()) {
                    generator()
                } else {
                    // Fallback to default if no generators provided
                    continue;
                };

                // Test all properties
                for property in &self.properties {
                    let passed = property.test_property(&state);
                    results.push(PropertyTestCase {
                        case_number: case,
                        property_name: property.name(),
                        state: format!("{:?}", state),
                        passed,
                        failure_reason: if !passed {
                            Some(format!("Property '{}' failed for state {:?}", property.name(), state))
                        } else {
                            None
                        },
                    });
                }
            }

            PropertyTestResult { cases: results }
        }
    }

    #[derive(Clone, Debug)]
    pub struct PropertyTestCase {
        pub case_number: u32,
        pub property_name: &'static str,
        pub state: String,
        pub passed: bool,
        pub failure_reason: Option<String>,
    }

    #[derive(Clone, Debug)]
    pub struct PropertyTestResult {
        pub cases: Vec<PropertyTestCase>,
    }

    impl PropertyTestResult {
        pub fn passed(&self) -> bool {
            self.cases.iter().all(|c| c.passed)
        }

        pub fn failure_count(&self) -> usize {
            self.cases.iter().filter(|c| !c.passed).count()
        }

        pub fn summary(&self) -> String {
            let total = self.cases.len();
            let passed = self.cases.iter().filter(|c| c.passed).count();
            let failed = total - passed;

            format!("Property tests: {} passed, {} failed out of {} total", passed, failed, total)
        }
    }
}
```

### State Machine Testing DSL
```rust
#[cfg(feature = "testing")]
pub mod state_machine_testing {
    use super::*;

    pub struct StateMachineTest<S: State, E: Event> {
        machine: Machine<S, E>,
        test_scenario: TestScenario<S, E>,
        assertions: Vec<Box<dyn TestAssertion<S, E>>>,
    }

    #[derive(Clone, Debug)]
    pub struct TestScenario<S: State, E: Event> {
        pub initial_state: String,
        pub initial_context: S,
        pub steps: Vec<TestStep<E>>,
        pub expected_final_state: Option<String>,
    }

    #[derive(Clone, Debug)]
    pub enum TestStep<E> {
        SendEvent(E),
        Wait(Duration),
        AssertState(String),
        AssertContext(Box<dyn Fn(&S) -> bool + Send + Sync>),
    }

    pub trait TestAssertion<S: State, E: Event> {
        fn check(&self, machine: &Machine<S, E>, step: usize) -> TestAssertionResult;
        fn name(&self) -> &'static str;
    }

    #[derive(Clone, Debug)]
    pub enum TestAssertionResult {
        Pass,
        Fail { reason: String },
        Skip { reason: String },
    }

    impl<S: State, E: Event> StateMachineTest<S, E> {
        pub fn new(machine: Machine<S, E>, scenario: TestScenario<S, E>) -> Self {
            Self {
                machine,
                test_scenario: scenario,
                assertions: Vec::new(),
            }
        }

        pub fn add_assertion<A: TestAssertion<S, E> + 'static>(mut self, assertion: A) -> Self {
            self.assertions.push(Box::new(assertion));
            self
        }

        pub fn run(&mut self) -> StateMachineTestResult {
            let mut results = Vec::new();

            // Reset machine to initial state
            self.machine = Machine::new(&self.test_scenario.initial_state, self.test_scenario.initial_context.clone());

            for (step_index, step) in self.test_scenario.steps.iter().enumerate() {
                match step {
                    TestStep::SendEvent(event) => {
                        let result = self.machine.send(event.clone());
                        results.push(TestStepResult {
                            step: step_index,
                            step_type: "send_event".to_string(),
                            success: result.is_ok(),
                            error: result.err().map(|e| e.to_string()),
                            state_after: self.machine.current_state().to_string(),
                        });
                    }
                    TestStep::Wait(duration) => {
                        // In real implementation, this would handle async waiting
                        results.push(TestStepResult {
                            step: step_index,
                            step_type: "wait".to_string(),
                            success: true,
                            error: None,
                            state_after: self.machine.current_state().to_string(),
                        });
                    }
                    TestStep::AssertState(expected_state) => {
                        let success = self.machine.current_state() == expected_state;
                        results.push(TestStepResult {
                            step: step_index,
                            step_type: "assert_state".to_string(),
                            success,
                            error: if !success {
                                Some(format!("Expected state '{}', got '{}'",
                                           expected_state, self.machine.current_state()))
                            } else {
                                None
                            },
                            state_after: self.machine.current_state().to_string(),
                        });
                    }
                    TestStep::AssertContext(predicate) => {
                        let success = predicate(self.machine.context());
                        results.push(TestStepResult {
                            step: step_index,
                            step_type: "assert_context".to_string(),
                            success,
                            error: if !success {
                                Some("Context assertion failed".to_string())
                            } else {
                                None
                            },
                            state_after: self.machine.current_state().to_string(),
                        });
                    }
                }

                // Run assertions
                for assertion in &self.assertions {
                    let assertion_result = assertion.check(&self.machine, step_index);
                    results.push(TestStepResult {
                        step: step_index,
                        step_type: format!("assertion_{}", assertion.name()),
                        success: matches!(assertion_result, TestAssertionResult::Pass),
                        error: match assertion_result {
                            TestAssertionResult::Fail { reason } => Some(reason),
                            TestAssertionResult::Skip { reason } => Some(format!("Skipped: {}", reason)),
                            TestAssertionResult::Pass => None,
                        },
                        state_after: self.machine.current_state().to_string(),
                    });
                }
            }

            // Check final state if specified
            if let Some(expected_final) = &self.test_scenario.expected_final_state {
                let success = self.machine.current_state() == expected_final;
                results.push(TestStepResult {
                    step: self.test_scenario.steps.len(),
                    step_type: "final_state_check".to_string(),
                    success,
                    error: if !success {
                        Some(format!("Expected final state '{}', got '{}'",
                                   expected_final, self.machine.current_state()))
                    } else {
                        None
                    },
                    state_after: self.machine.current_state().to_string(),
                });
            }

            StateMachineTestResult {
                scenario_name: "test_scenario".to_string(), // Would be configurable
                steps: results,
                total_duration: Duration::from_millis(0), // Would track actual duration
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct TestStepResult {
        pub step: usize,
        pub step_type: String,
        pub success: bool,
        pub error: Option<String>,
        pub state_after: String,
    }

    #[derive(Clone, Debug)]
    pub struct StateMachineTestResult {
        pub scenario_name: String,
        pub steps: Vec<TestStepResult>,
        pub total_duration: Duration,
    }

    impl StateMachineTestResult {
        pub fn passed(&self) -> bool {
            self.steps.iter().all(|step| step.success)
        }

        pub fn failure_count(&self) -> usize {
            self.steps.iter().filter(|step| !step.success).count()
        }

        pub fn summary(&self) -> String {
            let total_steps = self.steps.len();
            let passed_steps = self.steps.iter().filter(|s| s.success).count();
            let failed_steps = total_steps - passed_steps;

            format!("State machine test '{}': {} passed, {} failed out of {} steps",
                   self.scenario_name, passed_steps, failed_steps, total_steps)
        }
    }
}
```

### Model Checking Framework
```rust
#[cfg(feature = "testing")]
pub mod model_checking {
    use super::*;
    use std::collections::{HashSet, VecDeque};

    pub struct StateMachineModel<S: State + Clone + Eq + Hash, E: Event + Clone> {
        machine: Machine<S, E>,
        explored_states: HashSet<(String, S)>,
        state_queue: VecDeque<(String, S, Vec<E>)>,
        invariants: Vec<Box<dyn Invariant<S, E>>>,
        max_depth: usize,
    }

    pub trait Invariant<S: State, E: Event> {
        fn check(&self, state: &str, context: &S, history: &[E]) -> InvariantResult;
        fn name(&self) -> &'static str;
    }

    #[derive(Clone, Debug)]
    pub enum InvariantResult {
        Hold,
        Violated { reason: String },
    }

    impl<S: State + Clone + Eq + Hash, E: Event + Clone> StateMachineModel<S, E> {
        pub fn new(machine: Machine<S, E>, max_depth: usize) -> Self {
            let initial_state = (machine.current_state().to_string(), machine.context().clone());
            let mut explored_states = HashSet::new();
            explored_states.insert(initial_state.clone());

            let mut state_queue = VecDeque::new();
            state_queue.push_back((machine.current_state().to_string(), machine.context().clone(), Vec::new()));

            Self {
                machine,
                explored_states,
                state_queue,
                invariants: Vec::new(),
                max_depth,
            }
        }

        pub fn add_invariant<I: Invariant<S, E> + 'static>(mut self, invariant: I) -> Self {
            self.invariants.push(Box::new(invariant));
            self
        }

        pub fn check_model(&mut self) -> ModelCheckResult {
            let mut violations = Vec::new();
            let mut reachable_states = HashSet::new();

            while let Some((state_name, context, history)) = self.state_queue.pop_front() {
                reachable_states.insert((state_name.clone(), context.clone()));

                // Check invariants
                for invariant in &self.invariants {
                    match invariant.check(&state_name, &context, &history) {
                        InvariantResult::Violated { reason } => {
                            violations.push(ModelViolation {
                                invariant: invariant.name(),
                                state: state_name.clone(),
                                context: format!("{:?}", context),
                                history: history.iter().map(|e| format!("{:?}", e)).collect(),
                                reason,
                            });
                        }
                        InvariantResult::Hold => {}
                    }
                }

                // Explore transitions if under depth limit
                if history.len() < self.max_depth {
                    // Get available events (this would need to be implemented)
                    let available_events = self.get_available_events(&state_name, &context);

                    for event in available_events {
                        let mut test_machine = Machine::new(&state_name, context.clone());
                        let _ = test_machine.send(event.clone());

                        let new_state = (test_machine.current_state().to_string(), test_machine.context().clone());
                        let new_history = [history.as_slice(), &[event]].concat();

                        if self.explored_states.insert(new_state.clone()) {
                            self.state_queue.push_back((new_state.0, new_state.1, new_history));
                        }
                    }
                }
            }

            ModelCheckResult {
                total_states_explored: self.explored_states.len(),
                reachable_states: reachable_states.len(),
                violations,
                has_deadlocks: self.check_for_deadlocks(),
                invariants_checked: self.invariants.len(),
            }
        }

        fn get_available_events(&self, _state: &str, _context: &S) -> Vec<E> {
            // This would need to be implemented based on the specific event types
            // For now, return empty vec
            Vec::new()
        }

        fn check_for_deadlocks(&self) -> bool {
            // Check if any reachable state has no outgoing transitions
            // This is a simplified check
            false
        }

        pub fn reachable_states(&self) -> HashSet<String> {
            self.explored_states.iter().map(|(state, _)| state.clone()).collect()
        }

        pub fn has_no_deadlocks(&self) -> bool {
            !self.check_for_deadlocks()
        }
    }

    #[derive(Clone, Debug)]
    pub struct ModelViolation {
        pub invariant: &'static str,
        pub state: String,
        pub context: String,
        pub history: Vec<String>,
        pub reason: String,
    }

    #[derive(Clone, Debug)]
    pub struct ModelCheckResult {
        pub total_states_explored: usize,
        pub reachable_states: usize,
        pub violations: Vec<ModelViolation>,
        pub has_deadlocks: bool,
        pub invariants_checked: usize,
    }

    impl ModelCheckResult {
        pub fn passed(&self) -> bool {
            self.violations.is_empty() && !self.has_deadlocks
        }

        pub fn summary(&self) -> String {
            format!(
                "Model check: {} states explored, {} violations, {} invariants checked{}",
                self.total_states_explored,
                self.violations.len(),
                self.invariants_checked,
                if self.has_deadlocks { " (deadlocks detected)" } else { "" }
            )
        }
    }
}
```

### Performance Testing Framework
```rust
#[cfg(feature = "testing")]
pub mod performance_testing {
    use super::*;
    use std::time::Instant;

    pub struct PerformanceTest<S: State, E: Event> {
        setup: Box<dyn Fn() -> (Store<S>, Option<Machine<S, E>>) + Send + Sync>,
        operations: Vec<PerformanceOperation<S, E>>,
        iterations: usize,
        warmup_iterations: usize,
    }

    #[derive(Clone)]
    pub enum PerformanceOperation<S: State, E: Event> {
        StoreUpdate(Box<dyn Fn(&mut S) + Send + Sync>),
        StoreGet,
        MachineSend(E),
        Custom(Box<dyn Fn(&mut Store<S>, &mut Option<&mut Machine<S, E>>) + Send + Sync>),
    }

    impl<S: State, E: Event> PerformanceTest<S, E> {
        pub fn new<F>(setup: F, iterations: usize) -> Self
        where
            F: Fn() -> (Store<S>, Option<Machine<S, E>>) + Send + Sync + 'static,
        {
            Self {
                setup: Box::new(setup),
                operations: Vec::new(),
                iterations,
                warmup_iterations: iterations / 10,
            }
        }

        pub fn add_operation(mut self, operation: PerformanceOperation<S, E>) -> Self {
            self.operations.push(operation);
            self
        }

        pub fn with_warmup_iterations(mut self, warmup: usize) -> Self {
            self.warmup_iterations = warmup;
            self
        }

        pub fn run(&self) -> PerformanceTestResult {
            let mut results = Vec::new();

            // Warmup phase
            for _ in 0..self.warmup_iterations {
                let (mut store, mut machine) = (self.setup)();
                self.run_operations(&mut store, &mut machine.as_mut());
            }

            // Measurement phase
            for iteration in 0..self.iterations {
                let start_time = Instant::now();
                let (mut store, mut machine) = (self.setup)();
                self.run_operations(&mut store, &mut machine.as_mut());
                let duration = start_time.elapsed();

                results.push(IterationResult {
                    iteration,
                    duration,
                    memory_usage: self.measure_memory_usage(),
                });
            }

            PerformanceTestResult::new(results)
        }

        fn run_operations(&self, store: &mut Store<S>, machine: &mut Option<&mut Machine<S, E>>) {
            for operation in &self.operations {
                match operation {
                    PerformanceOperation::StoreUpdate(updater) => {
                        let _ = store.update(|s| updater(s));
                    }
                    PerformanceOperation::StoreGet => {
                        let _ = store.get().get_untracked();
                    }
                    PerformanceOperation::MachineSend(event) => {
                        if let Some(ref mut m) = machine {
                            let _ = m.send(event.clone());
                        }
                    }
                    PerformanceOperation::Custom(operation_fn) => {
                        operation_fn(store, machine);
                    }
                }
            }
        }

        fn measure_memory_usage(&self) -> usize {
            // Platform-specific memory measurement
            // This is a placeholder
            0
        }
    }

    #[derive(Clone, Debug)]
    pub struct IterationResult {
        pub iteration: usize,
        pub duration: Duration,
        pub memory_usage: usize,
    }

    #[derive(Clone, Debug)]
    pub struct PerformanceTestResult {
        pub results: Vec<IterationResult>,
        pub summary: PerformanceSummary,
    }

    #[derive(Clone, Debug)]
    pub struct PerformanceSummary {
        pub total_iterations: usize,
        pub average_duration: Duration,
        pub min_duration: Duration,
        pub max_duration: Duration,
        pub p95_duration: Duration,
        pub p99_duration: Duration,
        pub average_memory_usage: usize,
        pub max_memory_usage: usize,
    }

    impl PerformanceTestResult {
        pub fn new(results: Vec<IterationResult>) -> Self {
            let mut durations: Vec<Duration> = results.iter().map(|r| r.duration).collect();
            durations.sort();

            let total_iterations = results.len();
            let total_duration: Duration = durations.iter().sum();
            let average_duration = total_duration / total_iterations as u32;

            let min_duration = durations.first().cloned().unwrap_or_default();
            let max_duration = durations.last().cloned().unwrap_or_default();

            let p95_index = (total_iterations as f64 * 0.95) as usize;
            let p99_index = (total_iterations as f64 * 0.99) as usize;

            let p95_duration = durations.get(p95_index).cloned().unwrap_or_default();
            let p99_duration = durations.get(p99_index).cloned().unwrap_or_default();

            let memory_usages: Vec<usize> = results.iter().map(|r| r.memory_usage).collect();
            let average_memory = memory_usages.iter().sum::<usize>() / memory_usages.len();
            let max_memory = *memory_usages.iter().max().unwrap_or(&0);

            Self {
                results,
                summary: PerformanceSummary {
                    total_iterations,
                    average_duration,
                    min_duration,
                    max_duration,
                    p95_duration,
                    p99_duration,
                    average_memory_usage: average_memory,
                    max_memory_usage: max_memory,
                },
            }
        }

        pub fn summary(&self) -> &PerformanceSummary {
            &self.summary
        }

        pub fn report(&self) -> String {
            format!(
                "Performance Test Results:\n\
                 Total Iterations: {}\n\
                 Average Duration: {:.2}ms\n\
                 Min Duration: {:.2}ms\n\
                 Max Duration: {:.2}ms\n\
                 95th Percentile: {:.2}ms\n\
                 99th Percentile: {:.2}ms\n\
                 Average Memory: {} bytes\n\
                 Max Memory: {} bytes",
                self.summary.total_iterations,
                self.summary.average_duration.as_millis(),
                self.summary.min_duration.as_millis(),
                self.summary.max_duration.as_millis(),
                self.summary.p95_duration.as_millis(),
                self.summary.p99_duration.as_millis(),
                self.summary.average_memory_usage,
                self.summary.max_memory_usage
            )
        }
    }
}
```

## Error Handling

### Testing Errors
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum TestingError {
    #[error("Property test failed: {reason}")]
    PropertyTestFailed { reason: String },

    #[error("State machine test failed: {reason}")]
    StateMachineTestFailed { reason: String },

    #[error("Model checking failed: {reason}")]
    ModelCheckFailed { reason: String },

    #[error("Performance test failed: {reason}")]
    PerformanceTestFailed { reason: String },

    #[error("Test setup error: {reason}")]
    SetupError { reason: String },

    #[error("Test assertion failed: {assertion}")]
    AssertionFailed { assertion: String },

    #[error("Test timeout exceeded: {duration:?}")]
    Timeout { duration: Duration },
}
```

### Safe Testing
```rust
#[cfg(feature = "testing")]
pub fn run_test_safely<F, R>(test_fn: F) -> Result<R, TestingError>
where
    F: FnOnce() -> Result<R, TestingError>,
{
    std::panic::catch_unwind(|| test_fn())
        .map_err(|_| TestingError::SetupError {
            reason: "Test panicked".to_string(),
        })?
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(feature = "testing")]
mod tests {
    use super::*;

    #[test]
    fn property_test_basic_functionality() {
        struct PositiveProperty;
        impl<S: State + PartialOrd + From<i32>> StatePropertyTest<S> for PositiveProperty {
            fn test_property(&self, state: &S) -> bool {
                let num: i32 = (*state).clone().into();
                num >= 0
            }

            fn name(&self) -> &'static str { "positive" }
            fn description(&self) -> &'static str { "State value should be positive" }
        }

        let mut suite = PropertyTestSuite::<i32>::new()
            .add_property(PositiveProperty)
            .add_generator(|| rand::random::<i32>().abs());

        let result = suite.run(100);
        assert!(result.passed());
    }

    #[test]
    fn state_machine_test_basic_flow() {
        #[derive(Clone)]
        enum TestEvent { Start, Stop }
        impl Event for TestEvent {}

        let machine = Machine::new("idle", ());
        // Add states and transitions...

        let scenario = TestScenario {
            initial_state: "idle".to_string(),
            initial_context: (),
            steps: vec![
                TestStep::SendEvent(TestEvent::Start),
                TestStep::AssertState("running".to_string()),
                TestStep::SendEvent(TestEvent::Stop),
                TestStep::AssertState("idle".to_string()),
            ],
            expected_final_state: Some("idle".to_string()),
        };

        let mut test = StateMachineTest::new(machine, scenario);
        let result = test.run();

        assert!(result.passed());
    }

    #[test]
    fn performance_test_measurement() {
        let test = PerformanceTest::new(
            || (Store::new(0), None),
            100
        ).add_operation(PerformanceOperation::StoreUpdate(Box::new(|s| *s += 1)));

        let result = test.run();

        assert!(result.summary().total_iterations == 100);
        assert!(result.summary().average_duration > Duration::from_nanos(0));
    }
}
```

### Integration Tests
```rust
#[cfg(feature = "testing")]
#[test]
fn comprehensive_state_machine_test() {
    // Create complex state machine
    let machine = create_complex_test_machine();

    // Create comprehensive test scenario
    let scenario = TestScenario {
        initial_state: "idle".to_string(),
        initial_context: TestContext::default(),
        steps: vec![
            TestStep::SendEvent(TestEvent::Start),
            TestStep::AssertState("running".to_string()),
            TestStep::Wait(Duration::from_millis(100)),
            TestStep::AssertContext(Box::new(|ctx| ctx.started_at.is_some())),
            TestStep::SendEvent(TestEvent::Pause),
            TestStep::AssertState("paused".to_string()),
            TestStep::SendEvent(TestEvent::Resume),
            TestStep::AssertState("running".to_string()),
            TestStep::SendEvent(TestEvent::Stop),
            TestStep::AssertState("idle".to_string()),
        ],
        expected_final_state: Some("idle".to_string()),
    };

    // Add custom assertions
    let mut test = StateMachineTest::new(machine, scenario)
        .add_assertion(NoInvalidTransitionsAssertion)
        .add_assertion(ContextConsistencyAssertion);

    let result = test.run();
    assert!(result.passed());
}
```

## Performance Impact

### Test Execution Cost
- **Property Tests**: High cost due to many generated test cases
- **Model Checking**: Exponential cost with state space size
- **Performance Tests**: Direct measurement cost
- **State Machine Tests**: Linear with test scenario length

### Optimization Strategies
```rust
#[cfg(feature = "testing")]
impl<S: State, E: Event> PropertyTestSuite<S> {
    pub fn with_parallel_execution(mut self) -> Self {
        // Run property tests in parallel
        self.parallel_execution = true;
        self
    }

    pub fn with_early_termination(mut self) -> Self {
        // Stop on first failure
        self.early_termination = true;
        self
    }
}

#[cfg(feature = "testing")]
impl<S: State + Clone + Eq + Hash, E: Event + Clone> StateMachineModel<S, E> {
    pub fn with_bfs_search(mut self) -> Self {
        // Use breadth-first search for state exploration
        self.search_strategy = SearchStrategy::BFS;
        self
    }

    pub fn with_state_pruning(mut self) -> Self {
        // Prune uninteresting state space
        self.state_pruning = true;
        self
    }
}
```

## Security Considerations

### Test Data Safety
- Avoid exposing sensitive data in test failures
- Sanitize error messages and state dumps
- Control access to test execution and results

### Resource Limits
- Prevent infinite loops in property tests
- Limit model checking state space exploration
- Timeout mechanisms for long-running tests

## Future Extensions

### Fuzz Testing Integration
```rust
#[cfg(all(feature = "testing", feature = "fuzzing"))]
pub mod fuzz_testing {
    use super::*;

    pub struct FuzzTest<S: State, E: Event> {
        store: Store<S>,
        machine: Option<Machine<S, E>>,
        fuzz_strategy: Box<dyn FuzzStrategy<S, E>>,
    }

    impl<S: State, E: Event> FuzzTest<S, E> {
        pub fn fuzz_operations(&mut self, iterations: usize) -> FuzzResult {
            // Generate random operations and apply them
            // Look for crashes, panics, or invariant violations
            todo!()
        }
    }
}
```

### Continuous Testing
```rust
#[cfg(feature = "testing")]
pub mod continuous_testing {
    use super::*;

    pub struct ContinuousTestRunner {
        test_suites: Vec<Box<dyn TestSuite>>,
        schedule: TestSchedule,
        results_reporter: Box<dyn ResultsReporter>,
    }

    impl ContinuousTestRunner {
        pub async fn run_continuously(&self) -> Result<(), TestingError> {
            // Run tests on a schedule
            // Report results and regressions
            todo!()
        }
    }
}
```

### Visual Test Reporting
```rust
#[cfg(all(feature = "testing", feature = "visualization"))]
pub mod visual_reporting {
    use super::*;

    pub struct VisualTestReporter {
        results: Vec<TestResult>,
    }

    impl VisualTestReporter {
        pub fn generate_html_report(&self) -> String {
            // Generate interactive HTML test report
            todo!()
        }

        pub fn generate_coverage_heatmap(&self) -> String {
            // Generate state/transition coverage visualization
            todo!()
        }
    }
}
```

## Migration Guide

### Adding Testing to Existing Code
```rust
// Before - basic unit tests
#[test]
fn basic_store_test() {
    let store = Store::new(0);
    store.update(|s| *s = 42).unwrap();
    assert_eq!(store.get().get_untracked(), 42);
}

// After - comprehensive testing
#[cfg(feature = "testing")]
#[test]
fn comprehensive_store_test() {
    // Property-based testing
    property_test_store(|store: TestStore<i32>| {
        prop_assert!(store.get().get_untracked() >= 0);
    });

    // Performance testing
    let perf_test = PerformanceTest::new(
        || (Store::new(0), None),
        1000
    ).add_operation(PerformanceOperation::StoreUpdate(Box::new(|s| *s += 1)));

    let result = perf_test.run();
    assert!(result.summary().average_duration < Duration::from_millis(1));
}
```

### Configuration-Based Testing
```rust
#[derive(Deserialize)]
pub struct TestingConfig {
    pub enable_property_testing: bool,
    pub property_test_cases: usize,
    pub enable_performance_testing: bool,
    pub performance_test_iterations: usize,
    pub enable_model_checking: bool,
    pub model_check_max_depth: usize,
}

pub fn create_comprehensive_test_suite<S: State, E: Event>(
    config: &TestingConfig
) -> Vec<Box<dyn TestSuite>> {
    let mut suites = Vec::new();

    if config.enable_property_testing {
        suites.push(Box::new(PropertyTestSuite::<S>::new()
            .add_property(PositiveProperty)
            .run(config.property_test_cases)));
    }

    if config.enable_performance_testing {
        suites.push(Box::new(PerformanceTest::new(
            || (Store::new(S::default()), None),
            config.performance_test_iterations
        )));
    }

    if config.enable_model_checking {
        let machine = create_test_machine();
        suites.push(Box::new(StateMachineModel::new(machine, config.model_check_max_depth)));
    }

    suites
}
```

## Risk Assessment

### Likelihood: Medium
- Testing utilities are generally safe when used correctly
- Property-based testing can find edge cases and bugs
- Performance testing may have resource usage concerns
- Model checking can be computationally expensive

### Impact: Low
- Testing is opt-in and doesn't affect production code
- Test failures don't break the application
- Performance impact is limited to test execution
- Comprehensive error handling prevents test-related crashes

### Mitigation
- Clear documentation on testing best practices
- Resource limits and timeout mechanisms
- Safe test execution with panic boundaries
- Comprehensive error reporting and diagnostics
- Opt-in features with sensible defaults
- Performance monitoring of test execution
