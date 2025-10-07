//! Contract test runner implementation

use super::framework::*;
use std::time::Instant;

/// Contract test runner implementation
impl ContractTestRunner {
    /// Run contract tests with detailed reporting
    pub async fn run_tests_with_reporting(&self) -> ContractTestResults {
        println!("🔍 Starting Contract Testing Framework");
        println!("=====================================");

        let start_time = Instant::now();
        let results = self.run_all_tests().await;
        let total_time = start_time.elapsed();

        println!("\n📊 Contract Test Results");
        println!("=======================");
        println!("{}", results.summary());
        println!("\n⏱️  Total execution time: {:.2}s", total_time.as_secs_f64());

        if results.has_violations() {
            println!("\n❌ Contract Violations Found:");
            println!("============================");
            for (i, violation) in results.contract_violations.iter().enumerate() {
                println!("{}. {}", i + 1, violation);
            }
        } else {
            println!("\n✅ All contracts verified successfully!");
        }

        // Component breakdown
        println!("\n📈 Component Breakdown:");
        println!("======================");
        let component_stats = self.analyze_component_results(&results);
        for (component, stats) in component_stats {
            let success_rate = if stats.total == 0 { 0.0 } else { (stats.passed as f64 / stats.total as f64) * 100.0 };
            let status = if stats.failed == 0 { "✅" } else { "❌" };
            println!("{} {}: {}/{} tests passed ({:.1}%)", status, component, stats.passed, stats.total, success_rate);
        }

        results
    }

    /// Analyze results by component
    fn analyze_component_results(&self, results: &ContractTestResults) -> std::collections::HashMap<String, ComponentStats> {
        let mut component_stats = std::collections::HashMap::new();

        for result in &results.results {
            let stats = component_stats.entry(result.component.clone()).or_insert(ComponentStats::default());
            stats.total += 1;
            if result.passed {
                stats.passed += 1;
            } else {
                stats.failed += 1;
            }
        }

        component_stats
    }

    /// Run contract tests in parallel for better performance
    pub async fn run_tests_parallel(&self) -> ContractTestResults {
        let mut tasks = Vec::new();

        for (component_name, suite) in &self.test_suites {
            let component_name = component_name.clone();
            let suite = suite.as_ref(); // We'll need to clone or share the suite

            // For now, run sequentially - in practice you'd want Arc<dyn ContractTestSuite>
            // to enable parallel execution
            tasks.push(async move {
                let results = suite.run_tests().await;
                (component_name, results)
            });
        }

        let mut combined_results = ContractTestResults::new();

        for task in tasks {
            let (_component_name, results) = task.await;
            combined_results.results.extend(results.results);
            combined_results.contract_violations.extend(results.contract_violations);
            combined_results.total_tests += results.total_tests;
            combined_results.passed_tests += results.passed_tests;
            combined_results.failed_tests += results.failed_tests;
            combined_results.total_execution_time += results.total_execution_time;
        }

        combined_results
    }

    /// Generate a contract compliance report
    pub fn generate_compliance_report(&self, results: &ContractTestResults) -> String {
        let mut report = String::new();
        report.push_str("# Contract Compliance Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        report.push_str("## Executive Summary\n\n");
        report.push_str(&format!("- **Total Tests**: {}\n", results.total_tests));
        report.push_str(&format!("- **Pass Rate**: {:.1}%\n", results.success_rate()));
        report.push_str(&format!("- **Contract Violations**: {}\n", results.contract_violations.len()));
        report.push_str(&format!("- **Total Execution Time**: {:.2}s\n\n", results.total_execution_time.as_secs_f64()));

        if results.has_violations() {
            report.push_str("## Contract Violations\n\n");
            for (i, violation) in results.contract_violations.iter().enumerate() {
                report.push_str(&format!("### Violation {}\n\n", i + 1));
                report.push_str(&format!("{}\n\n", violation));
            }
        }

        report.push_str("## Component Status\n\n");
        report.push_str("| Component | Tests | Passed | Failed | Success Rate |\n");
        report.push_str("|-----------|-------|--------|--------|--------------|\n");

        let component_stats = self.analyze_component_results(results);
        for (component, stats) in component_stats {
            let success_rate = if stats.total == 0 { 0.0 } else { (stats.passed as f64 / stats.total as f64) * 100.0 };
            report.push_str(&format!("| {} | {} | {} | {} | {:.1}% |\n",
                component, stats.total, stats.passed, stats.failed, success_rate));
        }

        report.push_str("\n## Recommendations\n\n");

        if results.success_rate() >= 95.0 {
            report.push_str("✅ **Excellent**: Contract compliance is very high. Continue monitoring.\n\n");
        } else if results.success_rate() >= 80.0 {
            report.push_str("⚠️  **Good**: Contract compliance is acceptable but could be improved.\n\n");
        } else {
            report.push_str("❌ **Critical**: Contract compliance is too low. Immediate action required.\n\n");
        }

        if results.has_violations() {
            report.push_str("### Action Items\n\n");
            report.push_str("1. Review each contract violation in detail\n");
            report.push_str("2. Update component implementations to meet contracts\n");
            report.push_str("3. Add regression tests for fixed violations\n");
            report.push_str("4. Update API documentation if contracts change\n");
        }

        report
    }
}

/// Component statistics for reporting
#[derive(Debug, Default)]
struct ComponentStats {
    total: usize,
    passed: usize,
    failed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_contract_test_runner_initialization() {
        let runner = ContractTestRunner::new();

        // Should have registered some test suites
        let components = runner.list_components();
        assert!(!components.is_empty(), "Should have at least one test suite registered");

        // Should include our main components
        assert!(components.contains(&"config_sources".to_string()), "Should include config_sources");
        assert!(components.contains(&"integration_adapters".to_string()), "Should include integration_adapters");
    }

    #[test]
    fn test_contract_test_results_summary() {
        let mut results = ContractTestResults::new();

        // Add some test results
        results.add_result(ContractTestResult {
            test_name: "test_1".to_string(),
            component: "component_a".to_string(),
            passed: true,
            execution_time: std::time::Duration::from_millis(100),
            error_message: None,
            contract_violations: Vec::new(),
        });

        results.add_result(ContractTestResult {
            test_name: "test_2".to_string(),
            component: "component_a".to_string(),
            passed: false,
            execution_time: std::time::Duration::from_millis(200),
            error_message: Some("Contract violation".to_string()),
            contract_violations: vec!["Violation 1".to_string()],
        });

        assert_eq!(results.total_tests, 2);
        assert_eq!(results.passed_tests, 1);
        assert_eq!(results.failed_tests, 1);
        assert_eq!(results.success_rate(), 50.0);
        assert!(results.has_violations());
        assert_eq!(results.contract_violations.len(), 1);
    }

    #[test]
    fn test_contract_assertions() {
        // Test precondition assertion
        let result = ContractAssertions::assert_precondition(true, "Should pass");
        assert!(result.is_ok());

        let result = ContractAssertions::assert_precondition(false, "Should fail");
        assert!(matches!(result, Err(ContractViolation::PreConditionFailure(_))));

        // Test postcondition assertion
        let result = ContractAssertions::assert_postcondition(5 > 3, "Math works");
        assert!(result.is_ok());

        let result = ContractAssertions::assert_postcondition(3 > 5, "Math broken");
        assert!(matches!(result, Err(ContractViolation::PostConditionFailure(_))));

        // Test performance assertion
        let result = ContractAssertions::assert_performance(
            std::time::Duration::from_secs(1),
            std::time::Duration::from_millis(500),
            "test_operation"
        );
        assert!(result.is_ok());

        let result = ContractAssertions::assert_performance(
            std::time::Duration::from_millis(500),
            std::time::Duration::from_secs(1),
            "test_operation"
        );
        assert!(matches!(result, Err(ContractViolation::PerformanceViolation(_))));
    }

    #[test]
    fn test_component_analysis() {
        let runner = ContractTestRunner::new();
        let results = ContractTestResults::new();

        let component_stats = runner.analyze_component_results(&results);
        assert!(component_stats.is_empty(), "Empty results should have no component stats");

        // Test with actual results would require setting up test data
    }
}
