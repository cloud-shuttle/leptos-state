//! Advanced action system for state machine effects
//!
//! This module provides a comprehensive action system that allows side effects
//! during state transitions, including context updates, logging, async operations,
//! and complex action compositions.
//!
//! The actions module has been split into multiple modules for better organization:
//! - `action_core`: Core Action trait and basic action implementations
//! - `action_composite`: Composite actions (conditional, sequential, parallel)
//! - `action_control`: Control flow actions (retry, timer, metrics, timeout, circuit breaker)
//! - `action_builder`: Action builder for fluent action construction
//! - `action_executor`: Action executor with advanced features

// Re-export all action functionality from the split modules
pub use super::action_builder::*;
pub use super::action_composite::*;
pub use super::action_control::*;
pub use super::action_core::*;
pub use super::action_executor::*;
