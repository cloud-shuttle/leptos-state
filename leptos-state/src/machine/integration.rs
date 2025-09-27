//! State Machine Integration Patterns
//!
//! This module provides integration capabilities for state machines
//! with external systems, APIs, databases, and message queues.
//!
//! The integration module has been split into multiple modules for better organization:
//! - `integration_config`: Configuration structures and connection settings
//! - `integration_events`: Event structures and error handling
//! - `integration_core`: Core integration manager and adapter traits
//! - `integration_adapters`: Adapter implementations for different systems
//! - `integration_metrics`: Metrics collection and performance monitoring
//! - `integration_ext`: Extension traits and fluent APIs

// Re-export all integration functionality from the split modules
pub use super::integration_config::*;
pub use super::integration_events::*;
pub use super::integration_core::*;
pub use super::integration_adapters::*;
pub use super::integration_metrics::*;
pub use super::integration_ext::*;
