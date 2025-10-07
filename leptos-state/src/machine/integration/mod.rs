//! Integration functionality for state machines

pub mod events;
pub mod config;
pub mod metrics;
pub mod core;
pub mod adapters;

// Re-export modules for convenience
pub use events::*;
pub use config::*;
pub use metrics::*;
pub use core::*;
pub use adapters::*;
