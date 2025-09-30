//! Integration event structures and utilities

pub mod core;
pub mod priority;
pub mod errors;
pub mod batch;
pub mod filter;

// Re-export the most commonly used items
pub use core::IntegrationEvent;
pub use priority::EventPriority;
pub use errors::{ErrorHandlingStrategy, ErrorAction, IntegrationError, IntegrationErrorType};
pub use batch::EventBatch;
pub use filter::{EventFilter, CombinedEventFilter};
