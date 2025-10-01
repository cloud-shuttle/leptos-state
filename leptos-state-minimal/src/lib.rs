//! Leptos State Minimal - Simple, maintainable state management for Leptos
//!
//! This is a fresh implementation with minimal trait bounds for better usability.

pub mod error;
pub mod store;
pub mod machine;
pub mod hooks;
#[cfg(feature = "web")]
pub mod persistence;
pub mod middleware;
#[cfg(all(feature = "web", feature = "devtools"))]
pub mod devtools;
#[cfg(feature = "performance")]
pub mod performance;
#[cfg(feature = "testing")]
pub mod testing;
#[cfg(feature = "visualization")]
pub mod visualization;

pub use error::{MachineError, MachineResult, StoreError, StoreResult};
pub use store::{Store, StoreActions};
pub use machine::{Machine, StateNode, Transition};
pub use hooks::{use_store, use_machine};
pub use middleware::{Middleware, MiddlewareContext, MiddlewareError, MiddlewarePriority, Operation};
#[cfg(all(feature = "web", feature = "devtools"))]
pub use devtools::{DevToolsIntegration, StateInspector, TimeTravelDebugger};
#[cfg(feature = "performance")]
pub use performance::PerformanceMonitor;
#[cfg(feature = "testing")]
pub use testing::{PropertyTestSuite, StateMachineTester, TestStore, TestMachine};
#[cfg(feature = "visualization")]
pub use visualization::{Visualizable, StateMachineVisualizer};

// Re-export leptos for convenience
pub use leptos;

/// Marker trait for state types
///
/// This trait requires Send + Sync + Clone + std::fmt::Debug + Default + Eq + PartialEq + 'static
/// for reactive signal compatibility and enhanced functionality.
/// Clone is needed for signal operations, Send + Sync for concurrent contexts.
/// Debug enables development debugging, Default provides clean initialization,
/// Eq + PartialEq enable state comparison and reactivity optimization.
pub trait State: Send + Sync + Clone + std::fmt::Debug + Default + Eq + PartialEq + 'static {}

/// Auto-implement State for any type with the required bounds
impl<T: Send + Sync + Clone + std::fmt::Debug + Default + Eq + PartialEq + 'static> State for T {}

/// Marker trait for event types
///
/// This trait requires Send + Sync + Clone + std::fmt::Debug + Default + Eq + PartialEq + 'static
/// for machine compatibility and enhanced functionality.
/// Clone is needed for event handling, Send + Sync for concurrent contexts.
/// Debug enables development debugging, Default provides clean event initialization,
/// Eq + PartialEq enable event comparison and optimization.
pub trait Event: Send + Sync + Clone + std::fmt::Debug + Default + Eq + PartialEq + 'static {
    /// Optional: provide a string identifier for this event type
    fn event_type(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Auto-implement Event for any type with the required bounds
impl<T: Send + Sync + Clone + std::fmt::Debug + Default + Eq + PartialEq + 'static> Event for T {}

/// Marker trait for serializable state types
///
/// This trait extends State with serde serialization capabilities.
/// It requires all State bounds plus Serialize + DeserializeOwned.
#[cfg(feature = "serde")]
pub trait SerializableState: State + serde::Serialize + serde::de::DeserializeOwned {}

/// Auto-implement SerializableState for any type with the required bounds
#[cfg(feature = "serde")]
impl<T> SerializableState for T
where
    T: State + serde::Serialize + serde::de::DeserializeOwned,
{}

/// Marker trait for serializable event types
///
/// This trait extends Event with serde serialization capabilities.
/// It requires all Event bounds plus Serialize + DeserializeOwned.
#[cfg(feature = "serde")]
pub trait SerializableEvent: Event + serde::Serialize + serde::de::DeserializeOwned {}

/// Auto-implement SerializableEvent for any type with the required bounds
#[cfg(feature = "serde")]
impl<T> SerializableEvent for T
where
    T: Event + serde::Serialize + serde::de::DeserializeOwned,
{}

/// Snapshot of state with metadata for persistence and export
#[cfg(feature = "serde")]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct StateSnapshot<S> {
    /// The actual state data
    pub data: S,
    /// Timestamp when snapshot was created
    pub timestamp: std::time::SystemTime,
    /// Version of the application that created this snapshot
    pub version: String,
}

/// Snapshot of machine state with metadata for persistence and export
#[cfg(feature = "serde")]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct MachineSnapshot<S> {
    /// Current state name
    pub current_state: String,
    /// The context/state data
    pub context: S,
    /// Timestamp when snapshot was created
    pub timestamp: std::time::SystemTime,
}
