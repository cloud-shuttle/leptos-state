//! Leptos State Minimal - Simple, maintainable state management for Leptos
//!
//! This is a fresh implementation with minimal trait bounds for better usability.

pub mod error;
pub mod store;
pub mod machine;
pub mod hooks;

pub use error::{MachineError, MachineResult, StoreError, StoreResult};
pub use store::{Store, StoreActions};
pub use machine::{Machine, StateNode, Transition};
pub use hooks::{use_store, use_machine};

// Re-export leptos for convenience
pub use leptos;

/// Marker trait for state types
///
/// This trait requires Send + Sync + Clone + 'static for reactive signal compatibility.
/// Clone is needed for signal operations, Send + Sync for concurrent contexts.
pub trait State: Send + Sync + Clone + 'static {}

/// Auto-implement State for any type with the required bounds
impl<T: Send + Sync + Clone + 'static> State for T {}

/// Marker trait for event types
///
/// This trait requires Send + Sync + Clone + 'static for machine compatibility.
/// Clone is needed for event handling, Send + Sync for concurrent contexts.
pub trait Event: Send + Sync + Clone + 'static {
    /// Optional: provide a string identifier for this event type
    fn event_type(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Auto-implement Event for any type with the required bounds
impl<T: Send + Sync + Clone + 'static> Event for T {}
