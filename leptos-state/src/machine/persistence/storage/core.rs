//! Core storage traits and data structures

use crate::machine::persistence_core::PersistenceError;

/// Trait for machine storage backends
#[async_trait::async_trait]
pub trait MachineStorage: Send + Sync {
    /// Store data with key
    async fn store(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError>;

    /// Retrieve data by key
    async fn retrieve(&self, key: &str) -> Result<Vec<u8>, PersistenceError>;

    /// Delete data by key
    async fn delete(&self, key: &str) -> Result<(), PersistenceError>;

    /// List all keys
    async fn list_keys(&self) -> Result<Vec<String>, PersistenceError>;

    /// Check if key exists
    async fn exists(&self, key: &str) -> Result<bool, PersistenceError>;

    /// Clear all data
    async fn clear(&self) -> Result<(), PersistenceError>;

    /// Get storage info
    fn info(&self) -> StorageInfo;
}

/// Storage information
#[derive(Debug, Clone)]
pub struct StorageInfo {
    /// Storage type name
    pub storage_type: String,
    /// Maximum capacity (bytes, 0 = unlimited)
    pub max_capacity: u64,
    /// Current usage (bytes)
    pub current_usage: u64,
    /// Whether compression is supported
    pub supports_compression: bool,
    /// Whether encryption is supported
    pub supports_encryption: bool,
}

impl StorageInfo {
    /// Create new storage info
    pub fn new(storage_type: &str) -> Self {
        Self {
            storage_type: storage_type.to_string(),
            max_capacity: 0,
            current_usage: 0,
            supports_compression: false,
            supports_encryption: false,
        }
    }

    /// Set maximum capacity
    pub fn with_max_capacity(mut self, capacity: u64) -> Self {
        self.max_capacity = capacity;
        self
    }

    /// Set current usage
    pub fn with_current_usage(mut self, usage: u64) -> Self {
        self.current_usage = usage;
        self
    }

    /// Enable compression support
    pub fn with_compression_support(mut self, supports: bool) -> Self {
        self.supports_compression = supports;
        self
    }

    /// Enable encryption support
    pub fn with_encryption_support(mut self, supports: bool) -> Self {
        self.supports_encryption = supports;
        self
    }

    /// Check if storage has available capacity
    pub fn has_capacity(&self, size: u64) -> bool {
        if self.max_capacity == 0 {
            true // Unlimited capacity
        } else {
            self.current_usage + size <= self.max_capacity
        }
    }

    /// Calculate available capacity
    pub fn available_capacity(&self) -> u64 {
        if self.max_capacity == 0 {
            u64::MAX // Unlimited
        } else if self.current_usage >= self.max_capacity {
            0
        } else {
            self.max_capacity - self.current_usage
        }
    }

    /// Calculate usage percentage
    pub fn usage_percentage(&self) -> f64 {
        if self.max_capacity == 0 {
            0.0 // Unlimited, so 0% usage
        } else {
            (self.current_usage as f64 / self.max_capacity as f64) * 100.0
        }
    }

    /// Check if storage is nearly full (90%+ usage)
    pub fn is_nearly_full(&self) -> bool {
        self.usage_percentage() >= 90.0
    }

    /// Check if storage is full
    pub fn is_full(&self) -> bool {
        self.max_capacity > 0 && self.current_usage >= self.max_capacity
    }
}

impl Default for StorageInfo {
    fn default() -> Self {
        Self::new("unknown")
    }
}
