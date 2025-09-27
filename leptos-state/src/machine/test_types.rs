//! Core test types and configurations for state machine testing

use super::*;
use std::collections::HashMap;
use std::time::Duration;

/// Test configuration for state machine testing
#[derive(Debug, Clone, PartialEq)]
pub struct TestConfig {
    /// Maximum number of test iterations
    pub max_iterations: usize,
    /// Timeout for individual tests
    pub timeout: Duration,
    /// Whether to run performance tests
    pub run_performance_tests: bool,
    /// Whether to run integration tests
    pub run_integration_tests: bool,
    /// Whether to run property-based tests
    pub run_property_tests: bool,
    /// Data generation strategy
    pub data_strategy: DataStrategy,
    /// Coverage threshold (0.0 to 1.0)
    pub coverage_threshold: f64,
    /// Whether to generate test reports
    pub generate_reports: bool,
    /// Output directory for test reports
    pub output_dir: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            timeout: Duration::from_secs(30),
            run_performance_tests: true,
            run_integration_tests: true,
            run_property_tests: true,
            data_strategy: DataStrategy::Random,
            coverage_threshold: 0.8,
            generate_reports: true,
            output_dir: "test_reports".to_string(),
        }
    }
}

/// Data generation strategy for testing
#[derive(Debug, Clone, PartialEq)]
pub enum DataStrategy {
    /// Generate random test data
    Random,
    /// Generate data based on patterns
    Pattern,
    /// Use predefined test data
    Predefined,
    /// Generate data based on coverage analysis
    CoverageBased,
}

/// Test result for state machine tests
#[derive(Debug, Clone, PartialEq)]
pub struct TestResult {
    /// Whether the test passed
    pub passed: bool,
    /// Test execution time
    pub execution_time: Duration,
    /// Number of transitions executed
    pub transitions_executed: usize,
    /// Final state reached
    pub final_state: String,
    /// Error message if test failed
    pub error_message: Option<String>,
    /// Coverage information
    pub coverage: TestCoverage,
    /// Performance metrics
    pub performance: PerformanceMetrics,
}

/// Test coverage information
#[derive(Debug, Clone, PartialEq)]
pub struct TestCoverage {
    /// Percentage of states covered
    pub state_coverage: f64,
    /// Percentage of transitions covered
    pub transition_coverage: f64,
    /// Percentage of guards covered
    pub guard_coverage: f64,
    /// Percentage of actions covered
    pub action_coverage: f64,
    /// States that were covered
    pub covered_states: Vec<String>,
    /// Transitions that were covered
    pub covered_transitions: Vec<String>,
}

/// Performance metrics for tests
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceMetrics {
    /// Average transition time
    pub avg_transition_time: Duration,
    /// Maximum transition time
    pub max_transition_time: Duration,
    /// Memory usage during test
    pub memory_usage: usize,
    /// Number of allocations
    pub allocations: usize,
}

/// Test step in a test path
#[derive(Debug, Clone, PartialEq)]
pub struct TestStep {
    /// Event that triggered the step
    pub event: String,
    /// State before the step
    pub from_state: String,
    /// State after the step
    pub to_state: String,
    /// Context before the step
    pub context_before: String,
    /// Context after the step
    pub context_after: String,
    /// Guards that were evaluated
    pub guards_evaluated: Vec<String>,
    /// Actions that were executed
    pub actions_executed: Vec<String>,
    /// Time taken for this step
    pub duration: Duration,
}
