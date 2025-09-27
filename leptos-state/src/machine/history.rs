//! State machine history support
//!
//! This module provides history tracking capabilities for state machines,
//! allowing them to remember previous states and restore them when entering
//! history states.
//!
//! The history module has been split into multiple modules for better organization:
//! - `history_core`: Core history types and configurations
//! - `history_machine`: HistoryMachine implementation with state tracking
//! - `history_tracker`: HistoryTracker for managing state history
//! - `history_builder`: Builder extensions and fluent APIs

// Re-export all history functionality from the split modules
pub use super::history_core::*;
pub use super::history_machine::*;
pub use super::history_tracker::*;
pub use super::history_builder::*;
