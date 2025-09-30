//! Test macros and utilities for state machine testing

use super::*;

/// Macro for creating test cases
#[macro_export]
macro_rules! test_case {
    ($name:expr, $description:expr, $initial_context:expr) => {
        TestCase::new(
            $name.to_string(),
            $description.to_string(),
            $initial_context,
        )
    };
}

/// Macro for creating properties
#[macro_export]
macro_rules! property {
    ($name:expr, $description:expr, $check_fn:expr) => {
        Property {
            name: $name.to_string(),
            description: $description.to_string(),
            check_fn: Box::new($check_fn),
            critical: false,
            test_count: 100,
        }
    };
}

/// Macro for creating integration scenarios
#[macro_export]
macro_rules! integration_scenario {
    ($name:expr, $description:expr, $initial_context:expr, $event_sequence:expr, $expected_final_state:expr, $expected_final_context:expr) => {
        IntegrationScenario {
            name: $name.to_string(),
            description: $description.to_string(),
            initial_context: $initial_context,
            event_sequence: $event_sequence,
            expected_final_state: $expected_final_state.to_string(),
            expected_final_context: $expected_final_context,
            timeout: Duration::from_secs(30),
        }
    };
}

/// Macro for creating test steps
#[macro_export]
macro_rules! test_step {
    ($event:expr) => {
        TestCaseStep::new($event)
    };
    ($event:expr, $expected_state:expr) => {{
        let mut step = TestCaseStep::new($event);
        step.expect_state($expected_state.to_string());
        step
    }};
    ($event:expr, $expected_state:expr, $expected_context:expr) => {{
        let mut step = TestCaseStep::new($event);
        step.expect_state($expected_state.to_string());
        step.expect_context($expected_context.to_string());
        step
    }};
}

/// Macro for creating test configurations
#[macro_export]
macro_rules! test_config {
    () => {
        TestConfig::default()
    };
    (max_iterations: $max_iterations:expr) => {{
        let mut config = TestConfig::default();
        config.max_iterations = $max_iterations;
        config
    }};
    (timeout: $timeout:expr) => {{
        let mut config = TestConfig::default();
        config.timeout = $timeout;
        config
    }};
    (coverage_threshold: $threshold:expr) => {{
        let mut config = TestConfig::default();
        config.coverage_threshold = $threshold;
        config
    }};
}

/// Macro for creating data generation configurations
#[macro_export]
macro_rules! data_gen_config {
    () => {
        DataGenerationConfig::default()
    };
    (strategy: $strategy:expr) => {{
        let mut config = DataGenerationConfig::default();
        config.strategy = $strategy;
        config
    }};
    (test_case_count: $count:expr) => {{
        let mut config = DataGenerationConfig::default();
        config.test_case_count = $count;
        config
    }};
}

/// Macro for creating test suites
#[macro_export]
macro_rules! test_suite {
    ($machine:expr) => {
        $machine.test()
    };
    ($machine:expr, $config:expr) => {
        $machine.test().with_config($config)
    };
}

/// Macro for running tests
#[macro_export]
macro_rules! run_tests {
    ($builder:expr) => {
        $builder.build_and_run()
    };
}

/// Macro for asserting test results
#[macro_export]
macro_rules! assert_test_passed {
    ($result:expr) => {
        assert!($result.passed, "Test failed: {:?}", $result.error_message);
    };
}

/// Macro for asserting test results with message
#[macro_export]
macro_rules! assert_test_passed_with_message {
    ($result:expr, $message:expr) => {
        assert!($result.passed, "{}: {:?}", $message, $result.error_message);
    };
}

/// Macro for asserting coverage
#[macro_export]
macro_rules! assert_coverage {
    ($result:expr, $threshold:expr) => {
        assert!(
            $result.coverage.overall_coverage() >= $threshold,
            "Coverage {} below threshold {}",
            $result.coverage.overall_coverage(),
            $threshold
        );
    };
}

/// Macro for asserting performance
#[macro_export]
macro_rules! assert_performance {
    ($result:expr, $max_time:expr, $max_memory:expr) => {
        assert!(
            $result.performance.max_transition_time <= $max_time,
            "Transition time {} exceeds maximum {}",
            $result.performance.max_transition_time,
            $max_time
        );
        assert!(
            $result.performance.memory_usage <= $max_memory,
            "Memory usage {} exceeds maximum {}",
            $result.performance.memory_usage,
            $max_memory
        );
    };
}

/// Macro for creating test data generators
#[macro_export]
macro_rules! test_data_generator {
    () => {
        DefaultTestDataGenerator
    };
    ($machine:expr) => {
        MachineTestDataGenerator::new($machine)
    };
}

/// Macro for creating coverage trackers
#[macro_export]
macro_rules! coverage_tracker {
    () => {
        CoverageTracker::new()
    };
    ($states:expr, $transitions:expr, $guards:expr, $actions:expr) => {{
        let mut tracker = CoverageTracker::new();
        tracker.set_totals($states, $transitions, $guards, $actions);
        tracker
    }};
}

/// Macro for creating performance trackers
#[macro_export]
macro_rules! performance_tracker {
    () => {
        PerformanceTracker::new()
    };
}

/// Macro for creating test reports
#[macro_export]
macro_rules! test_report {
    ($results:expr) => {{
        let total_tests = $results.len();
        let passed_tests = $results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        let total_time: Duration = $results.iter().map(|r| r.execution_time).sum();

        format!(
            "Test Report:\n\
                Total Tests: {}\n\
                Passed: {}\n\
                Failed: {}\n\
                Pass Rate: {:.1}%\n\
                Total Time: {:?}",
            total_tests,
            passed_tests,
            failed_tests,
            (passed_tests as f64 / total_tests as f64) * 100.0,
            total_time
        )
    }};
}
