//! # Persistence System
//! 
//! This module provides the persistence system for state machines and stores.

#[cfg(feature = "persist")]
mod persistence_impl;

#[cfg(feature = "persist")]
pub use persistence_impl::*;

#[cfg(not(feature = "persist"))]
pub mod persistence_impl {
    // Placeholder module when persist feature is disabled
    pub struct PersistenceManager;
    
    impl PersistenceManager {
        pub fn with_memory_backend() -> Self {
            Self
        }
    }
}
