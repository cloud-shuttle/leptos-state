//! # Persistence System
//! 
//! This module provides the persistence system for state machines and stores.

use super::traits::{StateMachineContext, StateMachineState, StateMachineEvent, StoreState};
use super::error::{StateMachineError, PersistenceError};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt::Debug;

/// Trait for storage backends
#[cfg(feature = "persist")]
pub trait StorageBackend: Send + Sync + Debug {
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Save data to storage
    fn save<K, V>(&self, key: K, value: &V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize;
        
    /// Load data from storage
    fn load<K, V>(&self, key: K) -> Result<Option<V>, Self::Error>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>;
        
    /// Remove data from storage
    fn remove<K>(&self, key: K) -> Result<(), Self::Error>
    where
        K: AsRef<str>;
        
    /// Check if key exists in storage
    fn exists<K>(&self, key: K) -> Result<bool, Self::Error>
    where
        K: AsRef<str>;
        
    /// List all keys in storage
    fn list_keys(&self) -> Result<Vec<String>, Self::Error>;
}

/// LocalStorage backend for WASM environments
#[cfg(all(feature = "persist", feature = "wasm"))]
pub struct LocalStorageBackend {
    storage: web_sys::Storage,
}

#[cfg(all(feature = "persist", feature = "wasm"))]
impl LocalStorageBackend {
    pub fn new() -> Result<Self, PersistenceError> {
        let window = web_sys::window()
            .ok_or_else(|| PersistenceError::StorageFailed("Window not available".to_string()))?;
        let storage = window
            .local_storage()
            .map_err(|_| PersistenceError::StorageFailed("LocalStorage not available".to_string()))?
            .ok_or_else(|| PersistenceError::StorageFailed("LocalStorage not available".to_string()))?;
        
        Ok(Self { storage })
    }
}

#[cfg(all(feature = "persist", feature = "wasm"))]
impl StorageBackend for LocalStorageBackend {
    type Error = PersistenceError;
    
    fn save<K, V>(&self, key: K, value: &V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        let serialized = serde_json::to_string(value)
            .map_err(|e| PersistenceError::SerializationFailed(e.to_string()))?;
        
        self.storage
            .set_item(key.as_ref(), &serialized)
            .map_err(|_| PersistenceError::StorageFailed("Failed to save to LocalStorage".to_string()))?;
        
        Ok(())
    }
    
    fn load<K, V>(&self, key: K) -> Result<Option<V>, Self::Error>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>,
    {
        let item = self.storage
            .get_item(key.as_ref())
            .map_err(|_| PersistenceError::StorageFailed("Failed to load from LocalStorage".to_string()))?;
        
        match item {
            Some(data) => {
                let deserialized = serde_json::from_str(&data)
                    .map_err(|e| PersistenceError::DeserializationFailed(e.to_string()))?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }
    
    fn remove<K>(&self, key: K) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
    {
        self.storage
            .remove_item(key.as_ref())
            .map_err(|_| PersistenceError::StorageFailed("Failed to remove from LocalStorage".to_string()))?;
        
        Ok(())
    }
    
    fn exists<K>(&self, key: K) -> Result<bool, Self::Error>
    where
        K: AsRef<str>,
    {
        let item = self.storage
            .get_item(key.as_ref())
            .map_err(|_| PersistenceError::StorageFailed("Failed to check existence in LocalStorage".to_string()))?;
        
        Ok(item.is_some())
    }
    
    fn list_keys(&self) -> Result<Vec<String>, Self::Error> {
        // LocalStorage doesn't have a direct way to list keys
        // This is a limitation of the web API
        Err(PersistenceError::StorageFailed("Key listing not supported in LocalStorage".to_string()))
    }
}

/// Memory backend for testing and native environments
#[cfg(feature = "persist")]
#[derive(Debug)]
pub struct MemoryBackend {
    storage: std::sync::Arc<std::sync::Mutex<HashMap<String, String>>>,
}

#[cfg(feature = "persist")]
impl MemoryBackend {
    pub fn new() -> Self {
        Self {
            storage: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }
}

#[cfg(feature = "persist")]
impl StorageBackend for MemoryBackend {
    type Error = PersistenceError;
    
    fn save<K, V>(&self, key: K, value: &V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        let serialized = serde_json::to_string(value)
            .map_err(|e| PersistenceError::SerializationFailed(e.to_string()))?;
        
        let mut storage = self.storage.lock()
            .map_err(|_| PersistenceError::StorageFailed("Failed to acquire lock".to_string()))?;
        
        storage.insert(key.as_ref().to_string(), serialized);
        Ok(())
    }
    
    fn load<K, V>(&self, key: K) -> Result<Option<V>, Self::Error>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>,
    {
        let storage = self.storage.lock()
            .map_err(|_| PersistenceError::StorageFailed("Failed to acquire lock".to_string()))?;
        
        match storage.get(key.as_ref()) {
            Some(data) => {
                let deserialized = serde_json::from_str(data)
                    .map_err(|e| PersistenceError::DeserializationFailed(e.to_string()))?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }
    
    fn remove<K>(&self, key: K) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
    {
        let mut storage = self.storage.lock()
            .map_err(|_| PersistenceError::StorageFailed("Failed to acquire lock".to_string()))?;
        
        storage.remove(key.as_ref());
        Ok(())
    }
    
    fn exists<K>(&self, key: K) -> Result<bool, Self::Error>
    where
        K: AsRef<str>,
    {
        let storage = self.storage.lock()
            .map_err(|_| PersistenceError::StorageFailed("Failed to acquire lock".to_string()))?;
        
        Ok(storage.contains_key(key.as_ref()))
    }
    
    fn list_keys(&self) -> Result<Vec<String>, Self::Error> {
        let storage = self.storage.lock()
            .map_err(|_| PersistenceError::StorageFailed("Failed to acquire lock".to_string()))?;
        
        Ok(storage.keys().cloned().collect())
    }
}

/// Storage backend enum for different implementations
#[cfg(feature = "persist")]
#[derive(Debug)]
pub enum StorageBackendImpl {
    Memory(MemoryBackend),
    #[cfg(feature = "wasm")]
    LocalStorage(LocalStorageBackend),
}

#[cfg(feature = "persist")]
impl StorageBackend for StorageBackendImpl {
    type Error = PersistenceError;
    
    fn save<K, V>(&self, key: K, value: &V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        match self {
            StorageBackendImpl::Memory(backend) => backend.save(key, value),
            #[cfg(feature = "wasm")]
            StorageBackendImpl::LocalStorage(backend) => backend.save(key, value),
        }
    }
    
    fn load<K, V>(&self, key: K) -> Result<Option<V>, Self::Error>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>,
    {
        match self {
            StorageBackendImpl::Memory(backend) => backend.load(key),
            #[cfg(feature = "wasm")]
            StorageBackendImpl::LocalStorage(backend) => backend.load(key),
        }
    }
    
    fn remove<K>(&self, key: K) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
    {
        match self {
            StorageBackendImpl::Memory(backend) => backend.remove(key),
            #[cfg(feature = "wasm")]
            StorageBackendImpl::LocalStorage(backend) => backend.remove(key),
        }
    }
    
    fn exists<K>(&self, key: K) -> Result<bool, Self::Error>
    where
        K: AsRef<str>,
    {
        match self {
            StorageBackendImpl::Memory(backend) => backend.exists(key),
            #[cfg(feature = "wasm")]
            StorageBackendImpl::LocalStorage(backend) => backend.exists(key),
        }
    }
    
    fn list_keys(&self) -> Result<Vec<String>, Self::Error> {
        match self {
            StorageBackendImpl::Memory(backend) => backend.list_keys(),
            #[cfg(feature = "wasm")]
            StorageBackendImpl::LocalStorage(backend) => backend.list_keys(),
        }
    }
}

/// Persistence manager for state machines and stores
#[cfg(feature = "persist")]
pub struct PersistenceManager {
    backend: StorageBackendImpl,
    prefix: String,
}

#[cfg(feature = "persist")]
impl PersistenceManager {
    pub fn new(backend: StorageBackendImpl) -> Self {
        Self {
            backend,
            prefix: "leptos_state".to_string(),
        }
    }
    
    pub fn with_memory_backend() -> Self {
        Self::new(StorageBackendImpl::Memory(MemoryBackend::new()))
    }
    
    #[cfg(feature = "wasm")]
    pub fn with_local_storage_backend() -> Result<Self, PersistenceError> {
        let backend = LocalStorageBackend::new()?;
        Ok(Self::new(StorageBackendImpl::LocalStorage(backend)))
    }
    
    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }
    
    /// Save state machine state
    pub fn save_state_machine<C, E, S>(
        &self,
        id: &str,
        state: &S,
        context: &C,
    ) -> Result<(), PersistenceError>
    where
        C: StateMachineContext + Serialize,
        E: StateMachineEvent,
        S: StateMachineState<Context = C, Event = E> + Serialize,
    {
        let data = StateMachineData {
            state: state.clone(),
            context: context.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        let key = format!("{}:state_machine:{}", self.prefix, id);
        self.backend.save(key, &data)
    }
    
    /// Load state machine state
    pub fn load_state_machine<C, E, S>(
        &self,
        id: &str,
    ) -> Result<Option<(S, C)>, PersistenceError>
    where
        C: StateMachineContext + for<'de> Deserialize<'de>,
        E: StateMachineEvent,
        S: StateMachineState<Context = C, Event = E> + for<'de> Deserialize<'de>,
    {
        let key = format!("{}:state_machine:{}", self.prefix, id);
        match self.backend.load::<_, StateMachineData<S, C>>(key)? {
            Some(data) => Ok(Some((data.state, data.context))),
            None => Ok(None),
        }
    }
    
    /// Save store state
    pub fn save_store<S>(
        &self,
        id: &str,
        state: &S,
    ) -> Result<(), PersistenceError>
    where
        S: StoreState + Serialize,
    {
        let data = StoreData {
            state: state.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        let key = format!("{}:store:{}", self.prefix, id);
        self.backend.save(key, &data)
    }
    
    /// Load store state
    pub fn load_store<S>(
        &self,
        id: &str,
    ) -> Result<Option<S>, PersistenceError>
    where
        S: StoreState + for<'de> Deserialize<'de>,
    {
        let key = format!("{}:store:{}", self.prefix, id);
        match self.backend.load::<_, StoreData<S>>(key)? {
            Some(data) => Ok(Some(data.state)),
            None => Ok(None),
        }
    }
    
    /// Remove persisted data
    pub fn remove(&self, id: &str, data_type: PersistenceDataType) -> Result<(), PersistenceError> {
        let key = match data_type {
            PersistenceDataType::StateMachine => format!("{}:state_machine:{}", self.prefix, id),
            PersistenceDataType::Store => format!("{}:store:{}", self.prefix, id),
        };
        
        self.backend.remove(key)
    }
    
    /// Check if data exists
    pub fn exists(&self, id: &str, data_type: PersistenceDataType) -> Result<bool, PersistenceError> {
        let key = match data_type {
            PersistenceDataType::StateMachine => format!("{}:state_machine:{}", self.prefix, id),
            PersistenceDataType::Store => format!("{}:store:{}", self.prefix, id),
        };
        
        self.backend.exists(key)
    }
    
    /// List all persisted items
    pub fn list_items(&self) -> Result<Vec<PersistenceItem>, PersistenceError> {
        let keys = self.backend.list_keys()?;
        let mut items = Vec::new();
        
        for key in keys {
            if key.starts_with(&self.prefix) {
                let parts: Vec<&str> = key.split(':').collect();
                if parts.len() >= 3 {
                    let data_type = match parts[1] {
                        "state_machine" => PersistenceDataType::StateMachine,
                        "store" => PersistenceDataType::Store,
                        _ => continue,
                    };
                    
                    let id = parts[2..].join(":");
                    items.push(PersistenceItem {
                        id,
                        data_type,
                        key: key.clone(),
                    });
                }
            }
        }
        
        Ok(items)
    }
}

/// Data type for persistence
#[derive(Debug, Clone, PartialEq)]
pub enum PersistenceDataType {
    StateMachine,
    Store,
}

/// Persistence item information
#[derive(Debug, Clone)]
pub struct PersistenceItem {
    pub id: String,
    pub data_type: PersistenceDataType,
    pub key: String,
}

/// Serialized state machine data
#[derive(Serialize, Deserialize, Debug, Clone)]
struct StateMachineData<S, C> {
    state: S,
    context: C,
    timestamp: u64,
}

/// Serialized store data
#[derive(Serialize, Deserialize, Debug, Clone)]
struct StoreData<S> {
    state: S,
    timestamp: u64,
}

#[cfg(all(test, feature = "persist"))]
mod tests {
    use super::{MemoryBackend, PersistenceManager, PersistenceDataType, PersistenceItem};
    use crate::v1::traits::*;
    use serde::{Serialize, Deserialize};
    
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct TestContext {
        value: i32,
    }
    
    impl Default for TestContext {
        fn default() -> Self {
            Self { value: 0 }
        }
    }
    
    impl StateMachineContext for TestContext {}
    
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum TestEvent {
        Increment,
        Decrement,
    }
    
    impl Default for TestEvent {
        fn default() -> Self {
            TestEvent::Increment
        }
    }
    
    impl StateMachineEvent for TestEvent {}
    
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum TestState {
        Idle,
        Active,
    }
    
    impl Default for TestState {
        fn default() -> Self {
            TestState::Idle
        }
    }
    
    impl StateMachineState for TestState {
        type Context = TestContext;
        type Event = TestEvent;
    }
    
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct TestStore {
        count: i32,
    }
    
    impl Default for TestStore {
        fn default() -> Self {
            Self { count: 0 }
        }
    }
    
    impl StoreState for TestStore {}
    
    #[test]
    fn test_memory_backend_basic_operations() {
        let backend = MemoryBackend::new();
        
        // Test save and load
        let data = TestStore { count: 42 };
        backend.save("test_key", &data).unwrap();
        
        let loaded: Option<TestStore> = backend.load("test_key").unwrap();
        assert_eq!(loaded, Some(data));
        
        // Test exists
        assert!(backend.exists("test_key").unwrap());
        assert!(!backend.exists("nonexistent").unwrap());
        
        // Test remove
        backend.remove("test_key").unwrap();
        assert!(!backend.exists("test_key").unwrap());
        
        // Test list keys
        backend.save("key1", &TestStore { count: 1 }).unwrap();
        backend.save("key2", &TestStore { count: 2 }).unwrap();
        
        let keys = backend.list_keys().unwrap();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
    }
    
    #[test]
    fn test_persistence_manager_state_machine() {
        let manager = PersistenceManager::with_memory_backend();
        
        let state = TestState::Active;
        let context = TestContext { value: 100 };
        
        // Save state machine
        manager.save_state_machine("test_machine", &state, &context).unwrap();
        
        // Check existence
        assert!(manager.exists("test_machine", PersistenceDataType::StateMachine).unwrap());
        
        // Load state machine
        let loaded = manager.load_state_machine::<TestContext, TestEvent, TestState>("test_machine").unwrap();
        assert_eq!(loaded, Some((state, context)));
        
        // Remove
        manager.remove("test_machine", PersistenceDataType::StateMachine).unwrap();
        assert!(!manager.exists("test_machine", PersistenceDataType::StateMachine).unwrap());
    }
    
    #[test]
    fn test_persistence_manager_store() {
        let manager = PersistenceManager::with_memory_backend();
        
        let store = TestStore { count: 200 };
        
        // Save store
        manager.save_store("test_store", &store).unwrap();
        
        // Check existence
        assert!(manager.exists("test_store", PersistenceDataType::Store).unwrap());
        
        // Load store
        let loaded = manager.load_store::<TestStore>("test_store").unwrap();
        assert_eq!(loaded, Some(store));
        
        // Remove
        manager.remove("test_store", PersistenceDataType::Store).unwrap();
        assert!(!manager.exists("test_store", PersistenceDataType::Store).unwrap());
    }
    
    #[test]
    fn test_persistence_manager_list_items() {
        let manager = PersistenceManager::with_memory_backend();
        
        // Save some data
        manager.save_state_machine("machine1", &TestState::Idle, &TestContext::default()).unwrap();
        manager.save_store("store1", &TestStore::default()).unwrap();
        
        // List items
        let items = manager.list_items().unwrap();
        assert_eq!(items.len(), 2);
        
        let machine_item = items.iter().find(|item| item.id == "machine1").unwrap();
        assert_eq!(machine_item.data_type, PersistenceDataType::StateMachine);
        
        let store_item = items.iter().find(|item| item.id == "store1").unwrap();
        assert_eq!(store_item.data_type, PersistenceDataType::Store);
    }
    
    #[test]
    fn test_persistence_manager_with_prefix() {
        let manager = PersistenceManager::with_memory_backend().with_prefix("custom_prefix".to_string());
        
        let store = TestStore { count: 300 };
        manager.save_store("test", &store).unwrap();
        
        // Check that the key has the custom prefix
        let items = manager.list_items().unwrap();
        assert_eq!(items.len(), 1);
        assert!(items[0].key.starts_with("custom_prefix:store:"));
    }
}
