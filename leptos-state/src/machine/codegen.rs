//! State Machine Code Generation
//!
//! This module provides automatic code generation for state machines
//! in multiple programming languages.
//!
//! The codegen module has been split into multiple modules for better organization:
//! - `codegen_config`: Configuration and programming languages
//! - `codegen_core`: Core code generator functionality
//! - `codegen_types`: Generated file types and information structures
//! - `codegen_ext`: Extension traits for machines and builders
//! - `codegen_builder`: Builder pattern for fluent configuration

// Re-export all codegen functionality from the split modules
pub use super::codegen_builder::*;
pub use super::codegen_config::*;
pub use super::codegen_core::*;
pub use super::codegen_ext::*;
pub use super::codegen_types::*;
