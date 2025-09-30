//! Memory storage implementation for testing

use super::core::{MachineStorage, StorageInfo};
use crate::machine::persistence_core::PersistenceError;
use std::collections::HashMap;
use std::sync::RwLock;

/// Memory storage implementation for testing
#[derive(Debug)]
pub struct MemoryStorage {
    /// In-memory storage
    storage: RwLock<HashMap<String, Vec<u8>>>,
    /// Maximum capacity
    max_capacity: u64,
}

impl MemoryStorage {
    /// Create a new memory storage with unlimited capacity
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
            max_capacity: 0, // 0 = unlimited
        }
    }

    /// Create a new memory storage with capacity limit
    pub fn with_capacity(max_capacity: u64) -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
            max_capacity,
        }
    }

    /// Get current memory usage
    pub fn current_usage(&self) -> u64 {
        let storage = self.storage.read().unwrap();
        storage.values().map(|data| data.len() as u64).sum()
    }

    /// Check if adding data would exceed capacity
    fn check_capacity(&self, additional_size: u64) -> Result<(), PersistenceError> {
        if self.max_capacity > 0 {
            let current_usage = self.current_usage();
            if current_usage + additional_size > self.max_capacity {
                return Err(PersistenceError::StorageFull(format!(
                    "Would exceed capacity: {} + {} > {}",
                    current_usage, additional_size, self.max_capacity
                )));
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl MachineStorage for MemoryStorage {
    async fn store(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError> {
        self.check_capacity(data.len() as u64)?;

        let mut storage = self.storage.write().unwrap();
        storage.insert(key.to_string(), data.to_vec());
        Ok(())
    }

    async fn retrieve(&self, key: &str) -> Result<Vec<u8>, PersistenceError> {
        let storage = self.storage.read().unwrap();
        storage
            .get(key)
            .cloned()
            .ok_or_else(|| PersistenceError::KeyNotFound(key.to_string()))
    }

    async fn delete(&self, key: &str) -> Result<(), PersistenceError> {
        let mut storage = self.storage.write().unwrap();
        if storage.remove(key).is_none() {
            return Err(PersistenceError::KeyNotFound(key.to_string()));
        }
        Ok(())
    }

    async fn list_keys(&self) -> Result<Vec<String>, PersistenceError> {
        let storage = self.storage.read().unwrap();
        Ok(storage.keys().cloned().collect())
    }

    async fn exists(&self, key: &str) -> Result<bool, PersistenceError> {
        let storage = self.storage.read().unwrap();
        Ok(storage.contains_key(key))
    }

    async fn clear(&self) -> Result<(), PersistenceError> {
        let mut storage = self.storage.write().unwrap();
        storage.clear();
        Ok(())
    }

    fn info(&self) -> StorageInfo {
        StorageInfo::new("memory")
            .with_max_capacity(self.max_capacity)
            .with_current_usage(self.current_usage())
            .with_compression_support(true)
            .with_encryption_support(true)
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}
