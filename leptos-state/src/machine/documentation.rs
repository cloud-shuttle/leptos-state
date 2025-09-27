//! State Machine Documentation Generator
//!
//! This module provides comprehensive automatic documentation generation
//! for state machines, including multiple formats, templates, and diagrams.
//!
//! The documentation module has been split into multiple modules for better organization:
//! - `doc_config`: Documentation configuration and formats
//! - `doc_styling`: Documentation styling and templates
//! - `doc_generator`: Documentation generator implementation
//! - `doc_data`: Documentation data structures and output
//! - `doc_builder`: Documentation builder for fluent configuration

// Re-export all documentation functionality from the split modules
pub use super::doc_config::*;
pub use super::doc_styling::*;
pub use super::doc_generator::*;
pub use super::doc_data::*;
pub use super::doc_builder::*;
