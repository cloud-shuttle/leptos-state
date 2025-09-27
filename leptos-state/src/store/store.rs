//! Store implementation inspired by Zustand
//!
//! The store module has been split into multiple modules for better organization:
//! - `store_core`: Core Store trait and StoreContext
//! - `store_persistence`: Persistence functionality (localStorage)
//! - `store_selectors`: Selector functionality for slicing state
//! - `store_simple`: SimpleStore implementation and middleware
//! - `store_memoized`: Memoized selectors and advanced patterns

// Re-export all store functionality from the split modules
pub use super::store_core::*;
pub use super::store_persistence::*;
pub use super::store_selectors::*;
pub use super::store_simple::*;
pub use super::store_memoized::*;
