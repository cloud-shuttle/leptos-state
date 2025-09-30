//! Integration functionality for state machines

pub mod events;
pub mod config;
pub mod metrics;
pub mod core;

// Re-export modules for convenience
pub use events::*;
pub use config::*;
pub use metrics::*;
pub use core::*;
