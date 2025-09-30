//! Advanced guard system for state machine transitions
//!
//! This module provides a comprehensive guard system that allows conditional
//! transitions based on context, events, and state conditions.
//!
//! The guards module has been split into multiple modules for better organization:
//! - `guard_core`: Core GuardEvaluator trait and basic guards
//! - `guard_logical`: Logical guards (AND, OR, NOT, XOR, majority)
//! - `guard_context`: Context-based guards (field equality, range, comparison)
//! - `guard_state`: State and event guards (state-based, event type, transitions)
//! - `guard_temporal`: Time-based guards (time, counter, rate limit, cooldown)
//! - `guard_composite`: Composite guards with custom logic
//! - `guard_builder`: Guard builder for fluent guard construction

// Re-export all guard functionality from the split modules
pub use super::guard_builder::*;
pub use super::guard_composite::*;
pub use super::guard_context::*;
pub use super::guard_core::*;
pub use super::guard_logical::*;
pub use super::guard_state::*;
pub use super::guard_temporal::*;
