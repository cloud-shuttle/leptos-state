//! Integration functionality for state machines

pub mod events;
pub mod config;
pub mod metrics;

// Re-export modules for convenience
pub use events::*;
pub use config::*;
pub use metrics::*;
