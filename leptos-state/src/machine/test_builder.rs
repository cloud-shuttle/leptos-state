//! Test builder for fluent test creation

use super::*;
use std::hash::Hash;
use std::time::Duration;

/// Test builder for fluent test creation
pub struct TestBuilder<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
> {
    /// Machine being tested
    pub machine: Machine<C, E, C>,
    /// Test configuration
    pub config: TestConfig,
    /// Test cases
    pub test_cases: Vec<TestCase<C, E>>,
    /// Properties
    pub properties: Vec<Property<C, E>>,
    /// Integration scenarios
    pub scenarios: Vec<IntegrationScenario<C, E>>,
}

impl<
        C: Send + Sync + Clone + PartialEq + 'static,
        E: Clone + Send + Sync + Hash + Eq + 'static,
    > TestBuilder<C, E>
{
    /// Create a new test builder
    pub fn new(machine: Machine<C, E, C>) -> Self {
        Self {
            machine,
            config: TestConfig::default(),
            test_cases: Vec::new(),
            properties: Vec::new(),
            scenarios: Vec::new(),
        }
    }

    /// Set test configuration
    pub fn with_config(mut self, config: TestConfig) -> Self {
        self.config = config;
        self
    }

    /// Add a test case
    pub fn add_test_case(mut self, test_case: TestCase<C, E>) -> Self {
        self.test_cases.push(test_case);
        self
    }

    /// Add a property
    pub fn add_property(mut self, property: Property<C, E>) -> Self {
        self.properties.push(property);
        self
    }

    /// Add an integration scenario
    pub fn add_scenario(mut self, scenario: IntegrationScenario<C, E>) -> Self {
        self.scenarios.push(scenario);
        self
    }

    /// Build and run all tests
    pub fn build_and_run(self) -> TestSuiteResult {
        let mut runner = MachineTestRunner::new(self.machine, self.config);
        let mut results = Vec::new();

        // Run unit tests
        if let Some(unit_results) = runner.run_unit_tests() {
            results.extend(unit_results);
        }

        // Run property tests
        if !self.properties.is_empty() {
            let mut property_runner = PropertyTestRunner::new(runner.machine, runner.config);
            for property in self.properties {
                property_runner.add_property(property);
            }
            if let Some(property_results) = property_runner.run_all_tests() {
                // Convert property results to test results
                for property_result in property_results {
                    results.push(TestResult {
                        passed: property_result.passed,
                        execution_time: property_result.total_execution_time,
                        transitions_executed: 0, // Property tests don't track transitions
                        final_state: "property_tested".to_string(),
                        error_message: if property_result.passed {
                            None
                        } else {
                            Some("Property test failed".to_string())
                        },
                        coverage: TestCoverage {
                            state_coverage: 0.0,
                            transition_coverage: 0.0,
                            guard_coverage: 0.0,
                            action_coverage: 0.0,
                            covered_states: Vec::new(),
                            covered_transitions: Vec::new(),
                        },
                        performance: PerformanceMetrics {
                            avg_transition_time: Duration::from_nanos(0),
                            max_transition_time: Duration::from_nanos(0),
                            memory_usage: 0,
                            allocations: 0,
                        },
                    });
                }
            }
        }

        // Run integration tests
        if !self.scenarios.is_empty() {
            let mut integration_runner = IntegrationTestRunner::new(runner.machine, runner.config);
            for scenario in self.scenarios {
                integration_runner.add_scenario(scenario);
            }
            if let Some(integration_results) = integration_runner.run_all_tests() {
                // Convert integration results to test results
                for integration_result in integration_results {
                    results.push(TestResult {
                        passed: integration_result.passed,
                        execution_time: integration_result.execution_time,
                        transitions_executed: 0, // Integration tests don't track transitions
                        final_state: integration_result.final_state,
                        error_message: integration_result.error_message,
                        coverage: integration_result.coverage,
                        performance: integration_result.performance,
                    });
                }
            }
        }

        TestSuiteResult {
            total_tests: results.len(),
            passed_tests: results.iter().filter(|r| r.passed).count(),
            failed_tests: results.iter().filter(|r| !r.passed).count(),
            results,
            total_execution_time: results.iter().map(|r| r.execution_time).sum(),
        }
    }
}

/// Test suite result
#[derive(Debug, Clone, PartialEq)]
pub struct TestSuiteResult {
    /// Total number of tests
    pub total_tests: usize,
    /// Number of tests that passed
    pub passed_tests: usize,
    /// Number of tests that failed
    pub failed_tests: usize,
    /// Individual test results
    pub results: Vec<TestResult>,
    /// Total execution time
    pub total_execution_time: Duration,
}

impl TestSuiteResult {
    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.failed_tests == 0
    }

    /// Get pass rate
    pub fn pass_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            self.passed_tests as f64 / self.total_tests as f64
        }
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Test Suite Summary:\n\
            Total Tests: {}\n\
            Passed: {}\n\
            Failed: {}\n\
            Pass Rate: {:.1}%\n\
            Total Time: {:?}",
            self.total_tests,
            self.passed_tests,
            self.failed_tests,
            self.pass_rate() * 100.0,
            self.total_execution_time
        )
    }
}

/// Extension trait for adding testing to machines
pub trait MachineTestingExt<
    C: Send + Sync + Clone + PartialEq + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
>
{
    /// Create a test builder for this machine
    fn test(&self) -> TestBuilder<C, E>;
}

impl<
        C: Send + Sync + Clone + PartialEq + 'static,
        E: Clone + Send + Sync + Hash + Eq + 'static,
    > MachineTestingExt<C, E> for Machine<C, E, C>
{
    fn test(&self) -> TestBuilder<C, E> {
        TestBuilder::new(self.clone())
    }
}
