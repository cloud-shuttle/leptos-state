//! Persistence layer for state management
//!
//! This module provides browser LocalStorage persistence capabilities
//! for both stores and state machines.

#[cfg(feature = "web")]
use crate::{SerializableState, StoreError};
#[cfg(feature = "web")]
use web_sys::Storage;

/// Errors that can occur during persistence operations
#[cfg(feature = "web")]
#[derive(Debug, Clone, thiserror::Error)]
pub enum StorageError {
    #[error("Storage not available in this environment")]
    NotAvailable,

    #[error("Storage access denied")]
    AccessDenied,

    #[error("Storage quota exceeded")]
    StorageFull,

    #[error("Serialization error: {message}")]
    Serialization { message: String },

    #[error("Deserialization error: {message}")]
    Deserialization { message: String },

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Invalid key format: {0}")]
    InvalidKey(String),
}

/// Storage backend trait for different persistence mechanisms
#[cfg(feature = "web")]
pub trait StorageBackend: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError>;
    fn set(&self, key: &str, value: &str) -> Result<(), StorageError>;
    fn remove(&self, key: &str) -> Result<(), StorageError>;
    fn clear(&self) -> Result<(), StorageError>;
    fn keys(&self) -> Result<Vec<String>, StorageError>;
}

/// LocalStorage backend for browser persistence
#[cfg(feature = "web")]
pub struct LocalStorageBackend {
    prefix: String,
}

#[cfg(feature = "web")]
impl LocalStorageBackend {
    /// Create a new LocalStorage backend with default prefix
    pub fn new() -> Result<Self, StorageError> {
        if Self::is_available() {
            Ok(Self {
                prefix: "leptos-state".to_string(),
            })
        } else {
            Err(StorageError::NotAvailable)
        }
    }

    /// Create a new LocalStorage backend with custom prefix
    pub fn with_prefix(prefix: String) -> Result<Self, StorageError> {
        if Self::is_available() {
            Ok(Self { prefix })
        } else {
            Err(StorageError::NotAvailable)
        }
    }

    /// Check if LocalStorage is available in the current environment
    fn is_available() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
                .is_some()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    }

    /// Get the prefixed key for storage
    fn prefixed_key(&self, key: &str) -> String {
        format!("{}:{}", self.prefix, key)
    }

    /// Get access to the LocalStorage
    fn get_storage() -> Result<Storage, StorageError> {
        #[cfg(target_arch = "wasm32")]
        {
            let window: Window = web_sys::window().ok_or(StorageError::NotAvailable)?;
            let storage = window.local_storage()
                .map_err(|_| StorageError::NotAvailable)?
                .ok_or(StorageError::NotAvailable)?;
            Ok(storage)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(StorageError::NotAvailable)
        }
    }
}

#[cfg(feature = "web")]
impl StorageBackend for LocalStorageBackend {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        let prefixed_key = self.prefixed_key(key);
        let storage = Self::get_storage()?;

        match storage.get_item(&prefixed_key) {
            Ok(Some(value)) => Ok(Some(value)),
            Ok(None) => Ok(None),
            Err(_) => Err(StorageError::AccessDenied),
        }
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        let prefixed_key = self.prefixed_key(key);
        let storage = Self::get_storage()?;

        storage.set_item(&prefixed_key, value)
            .map_err(|_| StorageError::StorageFull)
    }

    fn remove(&self, key: &str) -> Result<(), StorageError> {
        let prefixed_key = self.prefixed_key(key);
        let storage = Self::get_storage()?;

        storage.remove_item(&prefixed_key)
            .map_err(|_| StorageError::AccessDenied)
    }

    fn clear(&self) -> Result<(), StorageError> {
        let storage = Self::get_storage()?;
        storage.clear().map_err(|_| StorageError::AccessDenied)
    }

    fn keys(&self) -> Result<Vec<String>, StorageError> {
        let storage = Self::get_storage()?;
        let length = storage.length().map_err(|_| StorageError::AccessDenied)?;

        let mut keys = Vec::new();
        let prefix = &self.prefix;

        for i in 0..length {
            if let Ok(Some(key)) = storage.key(i) {
                if key.starts_with(&format!("{}:", prefix)) {
                    // Remove prefix from key
                    if let Some(stripped) = key.strip_prefix(&format!("{}:", prefix)) {
                        keys.push(stripped.to_string());
                    }
                }
            }
        }

        Ok(keys)
    }
}

/// A persistent store that automatically saves to LocalStorage
#[cfg(feature = "web")]
pub struct PersistentStore<S: SerializableState> {
    store: crate::Store<S>,
    backend: Box<dyn StorageBackend>,
    key: String,
    auto_save: bool,
}

#[cfg(feature = "web")]
impl<S: SerializableState> PersistentStore<S> {
    /// Create a new persistent store
    pub fn new(
        key: String,
        initial: S,
        backend: Box<dyn StorageBackend>
    ) -> Result<Self, StoreError> {
        let store = crate::Store::new(initial);

        // Try to load existing state
        if let Ok(Some(json)) = backend.get(&key) {
            if let Ok(loaded_state) = serde_json::from_str::<S>(&json) {
                store.set(loaded_state)?;
            }
        }

        Ok(Self {
            store,
            backend,
            key,
            auto_save: true,
        })
    }

    /// Create a persistent store with auto-save disabled
    pub fn with_auto_save(mut self, auto_save: bool) -> Self {
        self.auto_save = auto_save;
        self
    }

    /// Manually save the current state to storage
    pub fn save(&self) -> Result<(), StoreError> {
        let json = self.store.to_json()?;
        self.backend.set(&self.key, &json)?;
        Ok(())
    }

    /// Manually load state from storage
    pub fn load(&self) -> Result<(), StoreError> {
        if let Some(json) = self.backend.get(&self.key)? {
            self.store.from_json(&json)?;
        }
        Ok(())
    }

    /// Clear the stored state
    pub fn clear(&self) -> Result<(), StoreError> {
        self.backend.remove(&self.key)?;
        Ok(())
    }

    /// Get a read signal for the current state
    pub fn get(&self) -> leptos::prelude::ReadSignal<S> {
        self.store.get()
    }

    /// Update the state (will auto-save if enabled)
    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + 'static,
    {
        self.store.update(updater)?;
        if self.auto_save {
            self.save()?;
        }
        Ok(())
    }

    /// Set the state to a new value (will auto-save if enabled)
    pub fn set(&self, state: S) -> Result<(), StoreError> {
        self.store.set(state)?;
        if self.auto_save {
            self.save()?;
        }
        Ok(())
    }
}

/// Actions for interacting with a persistent store
#[cfg(feature = "web")]
#[derive(Clone)]
pub struct PersistentStoreActions<S: SerializableState> {
    store: std::rc::Rc<PersistentStore<S>>,
}

#[cfg(feature = "web")]
impl<S: SerializableState> PersistentStoreActions<S> {
    /// Create new actions for a persistent store
    pub fn new(store: PersistentStore<S>) -> Self {
        Self {
            store: std::rc::Rc::new(store),
        }
    }

    /// Update the store state
    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + 'static,
    {
        self.store.update(updater)
    }

    /// Set the store state to a new value
    pub fn set(&self, state: S) -> Result<(), StoreError> {
        self.store.set(state)
    }

    /// Manually save the current state
    pub fn save(&self) -> Result<(), StoreError> {
        self.store.save()
    }

    /// Manually load state from storage
    pub fn load(&self) -> Result<(), StoreError> {
        self.store.load()
    }

    /// Clear the stored state
    pub fn clear(&self) -> Result<(), StoreError> {
        self.store.clear()
    }
}

/// Graceful fallback actions that wrap a regular store but don't persist
#[cfg(feature = "web")]
#[derive(Clone)]
pub struct GracefulStoreActions<S: SerializableState> {
    actions: crate::StoreActions<S>,
}

#[cfg(feature = "web")]
impl<S: SerializableState> GracefulStoreActions<S> {
    /// Create graceful actions that wrap regular store actions
    pub fn new(actions: crate::StoreActions<S>) -> Self {
        Self { actions }
    }

    /// Update the store state (no-op for persistence)
    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + 'static,
    {
        self.actions.update(updater)
    }

    /// Set the store state to a new value (no-op for persistence)
    pub fn set(&self, state: S) -> Result<(), StoreError> {
        self.actions.set(state)
    }

    /// Save operation (no-op for graceful fallback)
    pub fn save(&self) -> Result<(), StoreError> {
        Ok(()) // No-op
    }

    /// Load operation (no-op for graceful fallback)
    pub fn load(&self) -> Result<(), StoreError> {
        Ok(()) // No-op
    }

    /// Clear operation (no-op for graceful fallback)
    pub fn clear(&self) -> Result<(), StoreError> {
        Ok(()) // No-op
    }
}

// Implement the same trait as PersistentStoreActions for polymorphism
#[cfg(feature = "web")]
pub trait StoreActionsTrait<S: SerializableState> {
    fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + 'static;
    fn set(&self, state: S) -> Result<(), StoreError>;
    fn save(&self) -> Result<(), StoreError>;
    fn load(&self) -> Result<(), StoreError>;
    fn clear(&self) -> Result<(), StoreError>;
}

#[cfg(feature = "web")]
impl<S: SerializableState> StoreActionsTrait<S> for PersistentStoreActions<S> {
    fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + 'static,
    {
        PersistentStoreActions::update(self, updater)
    }

    fn set(&self, state: S) -> Result<(), StoreError> {
        PersistentStoreActions::set(self, state)
    }

    fn save(&self) -> Result<(), StoreError> {
        PersistentStoreActions::save(self)
    }

    fn load(&self) -> Result<(), StoreError> {
        PersistentStoreActions::load(self)
    }

    fn clear(&self) -> Result<(), StoreError> {
        PersistentStoreActions::clear(self)
    }
}

#[cfg(feature = "web")]
impl<S: SerializableState> StoreActionsTrait<S> for GracefulStoreActions<S> {
    fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + 'static,
    {
        GracefulStoreActions::update(self, updater)
    }

    fn set(&self, state: S) -> Result<(), StoreError> {
        GracefulStoreActions::set(self, state)
    }

    fn save(&self) -> Result<(), StoreError> {
        GracefulStoreActions::save(self)
    }

    fn load(&self) -> Result<(), StoreError> {
        GracefulStoreActions::load(self)
    }

    fn clear(&self) -> Result<(), StoreError> {
        GracefulStoreActions::clear(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "web")]
    mod web_tests {
        use super::*;
        use wasm_bindgen_test::*;

        wasm_bindgen_test_configure!(run_in_browser);

        #[wasm_bindgen_test]
        fn localstorage_backend_basic_operations() {
            let backend = LocalStorageBackend::new().unwrap();

            // Test set/get
            backend.set("test_key", "test_value").unwrap();
            assert_eq!(backend.get("test_key").unwrap(), Some("test_value".to_string()));

            // Test remove
            backend.remove("test_key").unwrap();
            assert_eq!(backend.get("test_key").unwrap(), None);
        }

        #[wasm_bindgen_test]
        fn persistent_store_integration() {
            #[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Debug)]
            struct TestState { count: i32 }

            impl crate::SerializableState for TestState {}

            let backend = LocalStorageBackend::new().unwrap();

            let store = PersistentStore::new(
                "test_store".to_string(),
                TestState { count: 0 },
                Box::new(backend)
            ).unwrap();

            // Update state
            store.update(|s| s.count = 42).unwrap();

            // Create new store - should load persisted state
            let backend2 = LocalStorageBackend::new().unwrap();
            let new_store = PersistentStore::new(
                "test_store".to_string(),
                TestState { count: 0 },
                Box::new(backend2)
            ).unwrap();

            // Check that state was loaded
            let loaded_state = new_store.get().get_untracked();
            assert_eq!(loaded_state.count, 42);
        }
    }
}
