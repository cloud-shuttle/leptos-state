//! Store implementation inspired by Zustand
//!
//! Provides simple, reactive stores with minimal boilerplate.

pub mod async_store;
pub mod async_store_cached;
pub mod async_store_core;
pub mod async_store_hooks;
pub mod async_store_infinite;
#[cfg(feature = "devtools")]
pub mod devtools;
#[cfg(feature = "devtools")]
pub mod devtools_connectors;
#[cfg(feature = "devtools")]
pub mod devtools_core;
#[cfg(feature = "devtools")]
pub mod devtools_timeline;
pub mod middleware;
pub mod store;
pub mod store_core;
// Memoized selectors are now in memoized module
pub mod store_persistence;
pub mod store_selectors;
pub mod store_simple;
pub mod memoized;

pub use async_store::*;
#[cfg(feature = "devtools")]
pub use devtools::*;
pub use middleware::*;
pub use store::*;
pub use store_core::{create_store, Store, StoreContext};
pub use memoized::{
    factory, DependencyTrackedSelector, LazySelector, PerformanceSelector,
    SelectorStats,
};
pub use memoized::combined::composition;
pub use store_persistence::{
    clear_from_local_storage, is_local_storage_available, load_from_local_storage,
    persist_to_local_storage, save_to_local_storage, MigrationManager, PersistenceMiddleware,
    VersionedPersistentStore,
};
pub use store_selectors::{
    selectors, CombinedSelector, FieldSelector, MemoizedSelector, PathSelector, StoreSlice,
};
pub use store_simple::{
    AsyncStore, LoggingMiddleware, MiddlewareStore, ReactiveStore, SimpleStore, StoreMiddleware,
    ValidationMiddleware,
};
