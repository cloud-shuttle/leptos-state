//! State Machine Testing Framework
//!
//! This module provides comprehensive testing utilities and frameworks
//! for state machines, including unit testing, integration testing,
//! property-based testing, and automated test generation.
//!
//! The testing framework has been split into multiple modules for better organization:
//! - `test_types`: Core test types and configurations
//! - `test_runner`: Test execution and running
//! - `test_cases`: Test case management
//! - `property_testing`: Property-based testing
//! - `integration_testing`: Integration testing
//! - `test_data_generation`: Test data generation
//! - `coverage_tracking`: Coverage analysis
//! - `performance_tracking`: Performance metrics
//! - `test_builder`: Fluent test creation
//! - `test_macros`: Test macros and utilities

// Re-export all testing functionality from the split modules
pub use super::coverage_tracking::*;
pub use super::integration_testing::*;
pub use super::performance_tracking::*;
pub use super::property_testing::*;
pub use super::test_builder::*;
pub use super::test_cases::*;
pub use super::test_data_generation::*;
pub use super::test_macros::*;
pub use super::test_runner::*;
pub use super::test_types::*;
