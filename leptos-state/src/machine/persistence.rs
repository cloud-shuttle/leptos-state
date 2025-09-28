//! State Machine Persistence & Serialization
//!
//! This module provides comprehensive persistence capabilities for state machines,
//! including serialization, storage, and restoration of machine states and contexts.
//!
//! The persistence module has been split into multiple modules for better organization:
//! - `persistence_core`: Core traits, configurations, and error types
//! - `persistence_serialization`: Serialization/deserialization for state machines
//! - `persistence_metadata`: Machine metadata and statistics
//! - `persistence_storage`: Storage backends (LocalStorage, MemoryStorage, FileSystemStorage)
//! - `persistence_manager`: Persistence manager and backup management
//! - `persistence_ext`: Extension traits and persistent machine wrappers

// Re-export all persistence functionality from the split modules
pub use super::persistence_core::*;
pub use super::persistence_serialization::*;
pub use super::persistence_metadata::*;
pub use super::persistence_storage::*;
pub use super::persistence_manager::*;
pub use super::persistence_ext::*;
