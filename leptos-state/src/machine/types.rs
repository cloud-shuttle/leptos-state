//! Machine types module - split into specialized modules for better organization
//!
//! The types module has been split into multiple modules for better organization:
//! - `types_basic`: Basic types, errors, and state types
//! - `types_config`: Configuration structures and settings
//! - `types_context`: Context-related types and values
//! - `types_history`: History tracking and logging types

// Re-export all type functionality from the split modules
pub use super::types_basic::*;
pub use super::types_config::*;
pub use super::types_context::*;
pub use super::types_history::*;
