//! State Machine Performance Optimization
//!
//! This module provides advanced performance optimization features
//! for state machines, including caching, lazy evaluation, performance
//! profiling, and optimization strategies.
//!
//! The performance module has been split into multiple modules for better organization:
//! - `performance_config`: Performance configuration and optimization strategies
//! - `performance_metrics`: Performance metrics, bottlenecks, and optimization suggestions
//! - `performance_profiler`: Performance profiler and monitoring
//! - `cache_system`: Cache statistics, memory tracker, transition cache, cache key, and cached transition
//! - `lazy_evaluation`: Lazy evaluation wrapper for expensive operations
//! - `optimized_machine`: Performance-optimized state machine
//! - `performance_builder`: Performance builder for fluent configuration

// Re-export all performance functionality from the split modules
pub use super::cache_system::*;
pub use super::lazy_evaluation::*;
pub use super::optimized_machine::*;
pub use super::performance_builder::*;
pub use super::performance_config::*;
pub use super::performance_metrics::*;
pub use super::performance_profiler::*;
