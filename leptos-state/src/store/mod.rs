//! Store implementation inspired by Zustand
//!
//! Provides simple, reactive stores with minimal boilerplate.

pub mod async_store;
#[cfg(feature = "devtools")]
pub mod devtools;
pub mod middleware;
pub mod store;
pub mod store_core;
pub mod store_persistence;
pub mod store_selectors;
pub mod store_simple;
pub mod store_memoized;

pub use async_store::*;
#[cfg(feature = "devtools")]
pub use devtools::*;
pub use middleware::*;
pub use store::*;
pub use store_core::{Store, StoreContext, create_store};
pub use store_persistence::{load_from_local_storage, save_to_local_storage, persist_to_local_storage, clear_from_local_storage, is_local_storage_available, PersistenceMiddleware, MigrationManager, VersionedPersistentStore};
pub use store_selectors::{StoreSlice, FieldSelector, PathSelector, MemoizedSelector, CombinedSelector, selectors};
pub use store_simple::{SimpleStore, ReactiveStore, AsyncStore, MiddlewareStore, StoreMiddleware, LoggingMiddleware, ValidationMiddleware};
pub use store_memoized::{DependencyTrackedSelector, PerformanceSelector, SelectorStats, LazySelector, composition, factory};
