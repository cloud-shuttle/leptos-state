//! Contract testing framework for leptos-state
//!
//! This module provides a comprehensive contract testing framework
//! that verifies API contracts across all components.

pub mod framework;
pub mod runner;
pub mod reporters;

pub use framework::*;
pub use runner::*;
pub use reporters::*;

/// Initialize the contract testing framework
pub fn init_contract_testing() -> ContractTestRunner {
    ContractTestRunner::new()
}

/// Run all contract tests
pub async fn run_all_contract_tests() -> ContractTestResults {
    let runner = init_contract_testing();
    runner.run_all_tests().await
}

/// Run contract tests for a specific component
pub async fn run_component_contract_tests(component: &str) -> ContractTestResults {
    let runner = init_contract_testing();
    runner.run_component_tests(component).await
}
