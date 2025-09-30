//! Async store integration with Leptos Resources
//!
//! The async_store module has been split into specialized modules for better organization:
//! - `async_store_core`: Core AsyncStore trait and ResourceStore implementation
//! - `async_store_hooks`: Hooks for async store integration (use_async_store, etc.)
//! - `async_store_cached`: Cached async store implementation
//! - `async_store_infinite`: Infinite loading store for paginated data

// Re-export all async store functionality from the split modules
pub use super::async_store_cached::*;
pub use super::async_store_core::*;
pub use super::async_store_hooks::*;
pub use super::async_store_infinite::*;
