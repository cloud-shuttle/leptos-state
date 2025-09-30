//! Collection utilities and data structures

pub mod core;
pub mod registry;
pub mod cache;

// Re-export the most commonly used items
pub use core::CollectionUtils;
pub use registry::{Registry, ObservableRegistry, RegistryEvent};
pub use cache::{Cache, CacheEntry, CacheStats, PriorityQueue};
