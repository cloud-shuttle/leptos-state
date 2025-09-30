//! Storage factory for creating storage instances

use super::core::MachineStorage;
use super::{FileSystemStorage, LocalStorage, MemoryStorage};
use crate::machine::persistence_core::PersistenceError;

/// Storage factory for creating storage instances
#[derive(Debug)]
pub struct StorageFactory;

impl StorageFactory {
    /// Create a new storage factory
    pub fn new() -> Self {
        Self
    }

    /// Create storage based on type string
    pub fn create_storage(&self, storage_type: &str, config: StorageConfig) -> Result<Box<dyn MachineStorage>, PersistenceError> {
        match storage_type.to_lowercase().as_str() {
            "memory" => {
                let storage = if let Some(capacity) = config.capacity {
                    MemoryStorage::with_capacity(capacity)
                } else {
                    MemoryStorage::new()
                };
                Ok(Box::new(storage))
            }
            "local" => {
                if LocalStorage::is_available() {
                    Ok(Box::new(LocalStorage::new()))
                } else {
                    Err(PersistenceError::StorageUnavailable("Local storage not available".to_string()))
                }
            }
            "filesystem" => {
                if let Some(path) = config.path {
                    let storage = if let Some(capacity) = config.capacity {
                        FileSystemStorage::with_capacity(path, capacity)?
                    } else {
                        FileSystemStorage::new(path)?
                    };
                    Ok(Box::new(storage))
                } else {
                    Err(PersistenceError::ConfigError("Path required for filesystem storage".to_string()))
                }
            }
            _ => Err(PersistenceError::ConfigError(format!("Unknown storage type: {}", storage_type))),
        }
    }

    /// Create memory storage
    pub fn create_memory(&self) -> Box<dyn MachineStorage> {
        Box::new(MemoryStorage::new())
    }

    /// Create memory storage with capacity
    pub fn create_memory_with_capacity(&self, capacity: u64) -> Box<dyn MachineStorage> {
        Box::new(MemoryStorage::with_capacity(capacity))
    }

    /// Create local storage (if available)
    pub fn create_local(&self) -> Result<Box<dyn MachineStorage>, PersistenceError> {
        if LocalStorage::is_available() {
            Ok(Box::new(LocalStorage::new()))
        } else {
            Err(PersistenceError::StorageUnavailable("Local storage not available".to_string()))
        }
    }

    /// Create filesystem storage
    pub fn create_filesystem<P: AsRef<std::path::Path>>(&self, path: P) -> Result<Box<dyn MachineStorage>, PersistenceError> {
        Ok(Box::new(FileSystemStorage::new(path)?))
    }

    /// Create filesystem storage with capacity
    pub fn create_filesystem_with_capacity<P: AsRef<std::path::Path>>(&self, path: P, capacity: u64) -> Result<Box<dyn MachineStorage>, PersistenceError> {
        Ok(Box::new(FileSystemStorage::with_capacity(path, capacity)?))
    }

    /// Get available storage types
    pub fn available_types(&self) -> Vec<&'static str> {
        let mut types = vec!["memory"];

        if LocalStorage::is_available() {
            types.push("local");
        }

        types.push("filesystem");
        types
    }

    /// Check if a storage type is available
    pub fn is_available(&self, storage_type: &str) -> bool {
        match storage_type.to_lowercase().as_str() {
            "memory" => true,
            "local" => LocalStorage::is_available(),
            "filesystem" => true, // Available as long as we have filesystem access
            _ => false,
        }
    }
}

/// Configuration for storage creation
#[derive(Debug, Clone, Default)]
pub struct StorageConfig {
    /// Maximum capacity (0 = unlimited)
    pub capacity: Option<u64>,
    /// Path for filesystem storage
    pub path: Option<std::path::PathBuf>,
}

impl StorageConfig {
    /// Create new storage config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set capacity
    pub fn with_capacity(mut self, capacity: u64) -> Self {
        self.capacity = Some(capacity);
        self
    }

    /// Set path
    pub fn with_path<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl Default for StorageFactory {
    fn default() -> Self {
        Self::new()
    }
}
