//! Core contract testing framework

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Result of a single contract test
#[derive(Debug, Clone)]
pub struct ContractTestResult {
    pub test_name: String,
    pub component: String,
    pub passed: bool,
    pub execution_time: Duration,
    pub error_message: Option<String>,
    pub contract_violations: Vec<String>,
}

/// Results from running multiple contract tests
#[derive(Debug, Clone)]
pub struct ContractTestResults {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_execution_time: Duration,
    pub results: Vec<ContractTestResult>,
    pub contract_violations: Vec<String>,
}

impl ContractTestResults {
    pub fn new() -> Self {
        Self {
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            total_execution_time: Duration::default(),
            results: Vec::new(),
            contract_violations: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: ContractTestResult) {
        self.total_tests += 1;
        self.total_execution_time += result.execution_time;

        if result.passed {
            self.passed_tests += 1;
        } else {
            self.failed_tests += 1;
            self.contract_violations.extend(result.contract_violations.clone());
        }

        self.results.push(result);
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }

    pub fn has_violations(&self) -> bool {
        !self.contract_violations.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "Contract Test Summary:\n\
            Total Tests: {}\n\
            Passed: {} ({:.1}%)\n\
            Failed: {}\n\
            Total Time: {:.2}s\n\
            Contract Violations: {}",
            self.total_tests,
            self.passed_tests,
            self.success_rate(),
            self.failed_tests,
            self.total_execution_time.as_secs_f64(),
            self.contract_violations.len()
        )
    }
}

/// Contract test runner
pub struct ContractTestRunner {
    test_suites: HashMap<String, Box<dyn ContractTestSuite>>,
}

impl ContractTestRunner {
    pub fn new() -> Self {
        let mut runner = Self {
            test_suites: HashMap::new(),
        };

        // Register built-in test suites
        runner.register_suite("config_sources", Box::new(crate::utils::config::sources::contract_tests::ConfigSourceContractTestRunner::new()));
        runner.register_suite("integration_adapters", Box::new(crate::machine::integration::adapters::contract_tests::IntegrationAdapterContractTestRunner::new()));

        runner
    }

    pub fn register_suite(&mut self, name: &str, suite: Box<dyn ContractTestSuite>) {
        self.test_suites.insert(name.to_string(), suite);
    }

    pub async fn run_all_tests(&self) -> ContractTestResults {
        let mut results = ContractTestResults::new();

        for (component_name, suite) in &self.test_suites {
            let component_results = suite.run_tests().await;
            results.results.extend(component_results.results);
            results.contract_violations.extend(component_results.contract_violations);
            results.total_tests += component_results.total_tests;
            results.passed_tests += component_results.passed_tests;
            results.failed_tests += component_results.failed_tests;
            results.total_execution_time += component_results.total_execution_time;
        }

        results
    }

    pub async fn run_component_tests(&self, component: &str) -> ContractTestResults {
        if let Some(suite) = self.test_suites.get(component) {
            suite.run_tests().await
        } else {
            ContractTestResults::new()
        }
    }

    pub fn list_components(&self) -> Vec<String> {
        self.test_suites.keys().cloned().collect()
    }
}

/// Trait for contract test suites
#[async_trait::async_trait]
pub trait ContractTestSuite: Send + Sync {
    async fn run_tests(&self) -> ContractTestResults;
}

/// Helper for creating contract test results
pub struct ContractTestBuilder {
    results: Vec<ContractTestResult>,
}

impl ContractTestBuilder {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_test<F, Fut>(&mut self, test_name: &str, component: &str, test_fn: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), String>>,
    {
        let start_time = Instant::now();

        // We need to make this synchronous for the builder pattern
        // In practice, you'd want to collect futures and await them
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                test_fn().await
            })
        });

        let execution_time = start_time.elapsed();

        let test_result = match result {
            Ok(()) => ContractTestResult {
                test_name: test_name.to_string(),
                component: component.to_string(),
                passed: true,
                execution_time,
                error_message: None,
                contract_violations: Vec::new(),
            },
            Err(error) => ContractTestResult {
                test_name: test_name.to_string(),
                component: component.to_string(),
                passed: false,
                execution_time,
                error_message: Some(error.clone()),
                contract_violations: vec![error],
            },
        };

        self.results.push(test_result);
    }

    pub fn build(self) -> ContractTestResults {
        let mut results = ContractTestResults::new();
        let total_execution_time: Duration = self.results.iter().map(|r| r.execution_time).sum();

        for result in self.results {
            results.add_result(result);
        }

        results.total_execution_time = total_execution_time;
        results
    }
}

/// Contract violation types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractViolation {
    PreConditionFailure(String),
    PostConditionFailure(String),
    InvariantViolation(String),
    PerformanceViolation(String),
    ErrorHandlingViolation(String),
}

/// Contract assertion helpers
pub struct ContractAssertions;

impl ContractAssertions {
    /// Assert that a pre-condition is met
    pub fn assert_precondition(condition: bool, message: &str) -> Result<(), ContractViolation> {
        if !condition {
            Err(ContractViolation::PreConditionFailure(message.to_string()))
        } else {
            Ok(())
        }
    }

    /// Assert that a post-condition is met
    pub fn assert_postcondition(condition: bool, message: &str) -> Result<(), ContractViolation> {
        if !condition {
            Err(ContractViolation::PostConditionFailure(message.to_string()))
        } else {
            Ok(())
        }
    }

    /// Assert that an invariant is maintained
    pub fn assert_invariant(condition: bool, message: &str) -> Result<(), ContractViolation> {
        if !condition {
            Err(ContractViolation::InvariantViolation(message.to_string()))
        } else {
            Ok(())
        }
    }

    /// Assert that performance requirements are met
    pub fn assert_performance(max_duration: Duration, actual_duration: Duration, operation: &str) -> Result<(), ContractViolation> {
        if actual_duration > max_duration {
            Err(ContractViolation::PerformanceViolation(
                format!("{} took {:?}, exceeded limit of {:?}", operation, actual_duration, max_duration)
            ))
        } else {
            Ok(())
        }
    }
}

/// Macro for defining contract tests
#[macro_export]
macro_rules! contract_test {
    ($test_name:ident, $component:expr, $body:block) => {
        #[tokio::test]
        async fn $test_name() {
            let result = async $body;
            result.await.expect("Contract test failed");
        }
    };
}

/// Macro for contract assertions in tests
#[macro_export]
macro_rules! assert_contract {
    ($condition:expr, $violation:expr) => {
        if !$condition {
            return Err($violation.into());
        }
    };
    ($condition:expr, $violation:expr, $($arg:tt)*) => {
        if !$condition {
            return Err(format!($violation, $($arg)*).into());
        }
    };
}
