//! State Machine Visualization & DevTools
//!
//! This module provides comprehensive visualization and debugging capabilities
//! for state machines, including visual state diagrams, real-time monitoring,
//! and advanced debugging tools.
//!
//! The visualization module has been split into multiple modules for better organization:
//! - `visualization_config`: Configuration and export formats
//! - `visualization_events`: Event structures for visualization
//! - `visualization_core`: Core visualizer functionality
//! - `visualization_data`: Data structures for diagrams and snapshots
//! - `visualization_debug`: Debugging tools (time travel, breakpoints)
//! - `visualization_monitor`: Real-time monitoring and health checks
//! - `visualization_ext`: Extension traits and fluent APIs

// Re-export all visualization functionality from the split modules
pub use super::visualization_config::*;
pub use super::visualization_events::*;
pub use super::visualization_core::*;
pub use super::visualization_data::*;
pub use super::visualization_debug::*;
pub use super::visualization_monitor::*;
pub use super::visualization_ext::*;
