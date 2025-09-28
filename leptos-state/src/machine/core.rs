//! Core machine module - split into specialized modules for better organization
//!
//! The core module has been split into multiple modules for better organization:
//! - `core_traits`: Core traits like StateMachine, MachineBuilder, MachineState
//! - `core_machine`: Main Machine struct and implementation
//! - `core_state`: StateNode struct and implementation
//! - `core_actions`: Action trait definitions
//! - `core_guards`: Guard trait definitions
//! - `core_errors`: Error types and results
//! - `core_macros`: Helper macros for machine creation

// Re-export all core functionality from the split modules
pub use super::core_traits::*;
pub use super::core_machine::*;
pub use super::core_state::*;
pub use super::core_actions::*;
pub use super::core_guards::*;
pub use super::core_errors::*;
pub use super::core_macros::*;
