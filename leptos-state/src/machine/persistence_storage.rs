//! Storage backends for machine persistence

use super::*;
use super::persistence_core::PersistenceError;

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
    pub fn max_capacity(mut self, capacity: u64) -> Self {
        self.max_capacity = capacity;
        self
    }

    /// Set current usage
    pub fn current_usage(mut self, usage: u64) -> Self {
        self.current_usage = usage;
        self
    }

    /// Enable compression support
    pub fn supports_compression(mut self, supports: bool) -> Self {
        self.supports_compression = supports;
        self
    }

    /// Enable encryption support
    pub fn supports_encryption(mut self, supports: bool) -> Self {
        self.supports_encryption = supports;
        self
    }

    /// Get usage percentage (0-100)
    pub fn usage_percentage(&self) -> f64 {
        if self.max_capacity == 0 {
            0.0
        } else {
            (self.current_usage as f64 / self.max_capacity as f64) * 100.0
        }
    }

    /// Check if storage is near capacity
    pub fn is_near_capacity(&self, threshold_percent: f64) -> bool {
        self.usage_percentage() >= threshold_percent
    }
}

/// Local storage implementation using web storage
pub struct LocalStorage;

impl LocalStorage {
    /// Create a new local storage instance
    pub fn new() -> Self {
        Self
    }

    /// Check if localStorage is available
    pub fn is_available() -> bool {
        #[cfg(feature = "hydrate")]
        {
            leptos::window().local_storage().is_ok()
        }
        #[cfg(not(feature = "hydrate"))]
        {
            false
        }
    }
}

#[async_trait::async_trait]
impl MachineStorage for LocalStorage {
    async fn store(&self, _key: &str, _data: &[u8]) -> Result<(), PersistenceError> {
        #[cfg(feature = "hydrate")]
        {
            let window = leptos::window();
            match window.local_storage() {
                Ok(Some(storage)) => {
                    // Convert bytes to base64 string
                    let encoded = base64::encode(_data);
                    storage.set_item(_key, &encoded)
                        .map_err(|e| PersistenceError::StorageError(format!("Failed to store: {:?}", e)))?;
                    Ok(())
                }
                _ => Err(PersistenceError::StorageError("localStorage not available".to_string())),
            }
        }
        #[cfg(not(feature = "hydrate"))]
        {
            Err(PersistenceError::StorageError("localStorage not available in SSR".to_string()))
        }
    }

    async fn retrieve(&self, _key: &str) -> Result<Vec<u8>, PersistenceError> {
        #[cfg(feature = "hydrate")]
        {
            let window = leptos::window();
            match window.local_storage() {
                Ok(Some(storage)) => {
                    match storage.get_item(_key) {
                        Ok(Some(encoded)) => {
                            base64::decode(&encoded)
                                .map_err(|e| PersistenceError::DeserializationError(format!("Failed to decode: {}", e)))
                        }
                        Ok(None) => Err(PersistenceError::StorageError(format!("Key '{}' not found", _key))),
                        Err(e) => Err(PersistenceError::StorageError(format!("Failed to retrieve: {:?}", e))),
                    }
                }
                _ => Err(PersistenceError::StorageError("localStorage not available".to_string())),
            }
        }
        #[cfg(not(feature = "hydrate"))]
        {
            Err(PersistenceError::StorageError("localStorage not available in SSR".to_string()))
        }
    }

    async fn delete(&self, _key: &str) -> Result<(), PersistenceError> {
        #[cfg(feature = "hydrate")]
        {
            let window = leptos::window();
            match window.local_storage() {
                Ok(Some(storage)) => {
                    storage.remove_item(_key)
                        .map_err(|e| PersistenceError::StorageError(format!("Failed to delete: {:?}", e)))
                }
                _ => Err(PersistenceError::StorageError("localStorage not available".to_string())),
            }
        }
        #[cfg(not(feature = "hydrate"))]
        {
            Err(PersistenceError::StorageError("localStorage not available in SSR".to_string()))
        }
    }

    async fn list_keys(&self) -> Result<Vec<String>, PersistenceError> {
        #[cfg(feature = "hydrate")]
        {
            let window = leptos::window();
            match window.local_storage() {
                Ok(Some(storage)) => {
                    // In a real implementation, we'd need to iterate through all keys
                    // For now, return empty vec
                    Ok(Vec::new())
                }
                _ => Err(PersistenceError::StorageError("localStorage not available".to_string())),
            }
        }
        #[cfg(not(feature = "hydrate"))]
        {
            Err(PersistenceError::StorageError("localStorage not available in SSR".to_string()))
        }
    }

    async fn exists(&self, _key: &str) -> Result<bool, PersistenceError> {
        #[cfg(feature = "hydrate")]
        {
            let window = leptos::window();
            match window.local_storage() {
                Ok(Some(storage)) => {
                    storage.get_item(_key)
                        .map(|result| result.is_some())
                        .map_err(|e| PersistenceError::StorageError(format!("Failed to check: {:?}", e)))
                }
                _ => Err(PersistenceError::StorageError("localStorage not available".to_string())),
            }
        }
        #[cfg(not(feature = "hydrate"))]
        {
            Err(PersistenceError::StorageError("localStorage not available in SSR".to_string()))
        }
    }

    async fn clear(&self) -> Result<(), PersistenceError> {
        #[cfg(feature = "hydrate")]
        {
            let window = leptos::window();
            match window.local_storage() {
                Ok(Some(storage)) => {
                    storage.clear()
                        .map_err(|e| PersistenceError::StorageError(format!("Failed to clear: {:?}", e)))
                }
                _ => Err(PersistenceError::StorageError("localStorage not available".to_string())),
            }
        }
        #[cfg(not(feature = "hydrate"))]
        {
            Err(PersistenceError::StorageError("localStorage not available in SSR".to_string()))
        }
    }

    fn info(&self) -> StorageInfo {
        StorageInfo::new("localStorage")
            .max_capacity(5 * 1024 * 1024) // 5MB typical limit
            .supports_compression(false)
            .supports_encryption(false)
    }
}

/// Memory storage implementation for testing
pub struct MemoryStorage {
    /// In-memory storage
    storage: std::sync::RwLock<std::collections::HashMap<String, Vec<u8>>>,
}

impl MemoryStorage {
    /// Create a new memory storage instance
    pub fn new() -> Self {
        Self {
            storage: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Get current memory usage
    pub fn memory_usage(&self) -> usize {
        let storage = self.storage.read().unwrap();
        storage.values().map(|v| v.len()).sum()
    }
}

#[async_trait::async_trait]
impl MachineStorage for MemoryStorage {
    async fn store(&self, _key: &str, _data: &[u8]) -> Result<(), PersistenceError> {
        let mut storage = self.storage.write().unwrap();
        storage.insert(_key.to_string(), _data.to_vec());
        Ok(())
    }

    async fn retrieve(&self, _key: &str) -> Result<Vec<u8>, PersistenceError> {
        let storage = self.storage.read().unwrap();
        storage.get(_key)
            .cloned()
            .ok_or_else(|| PersistenceError::StorageError(format!("Key '{}' not found", _key)))
    }

    async fn delete(&self, _key: &str) -> Result<(), PersistenceError> {
        let mut storage = self.storage.write().unwrap();
        storage.remove(_key);
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
            .max_capacity(100 * 1024 * 1024) // 100MB limit
            .current_usage(self.memory_usage() as u64)
            .supports_compression(true)
            .supports_encryption(true)
    }
}

/// File system storage implementation
pub struct FileSystemStorage {
    /// Base directory for storage
    base_dir: std::path::PathBuf,
}

impl FileSystemStorage {
    /// Create a new file system storage instance
    pub fn new(base_dir: std::path::PathBuf) -> Self {
        Self { base_dir }
    }

    /// Get file path for key
    fn get_file_path(&self, key: &str) -> std::path::PathBuf {
        // Sanitize key for filesystem
        let sanitized_key = key.replace("/", "_").replace("\\", "_").replace("..", "_");
        self.base_dir.join(format!("{}.data", sanitized_key))
    }
}

#[async_trait::async_trait]
impl MachineStorage for FileSystemStorage {
    async fn store(&self, _key: &str, _data: &[u8]) -> Result<(), PersistenceError> {
        let file_path = self.get_file_path(_key);

        // Ensure directory exists
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| PersistenceError::StorageError(format!("Failed to create directory: {}", e)))?;
        }

        tokio::fs::write(&file_path, _data).await
            .map_err(|e| PersistenceError::StorageError(format!("Failed to write file: {}", e)))
    }

    async fn retrieve(&self, _key: &str) -> Result<Vec<u8>, PersistenceError> {
        let file_path = self.get_file_path(_key);
        tokio::fs::read(&file_path).await
            .map_err(|e| PersistenceError::StorageError(format!("Failed to read file: {}", e)))
    }

    async fn delete(&self, _key: &str) -> Result<(), PersistenceError> {
        let file_path = self.get_file_path(_key);
        tokio::fs::remove_file(&file_path).await
            .map_err(|e| PersistenceError::StorageError(format!("Failed to delete file: {}", e)))
    }

    async fn list_keys(&self) -> Result<Vec<String>, PersistenceError> {
        let mut keys = Vec::new();

        let mut entries = tokio::fs::read_dir(&self.base_dir).await
            .map_err(|e| PersistenceError::StorageError(format!("Failed to read directory: {}", e)))?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| PersistenceError::StorageError(format!("Failed to read entry: {}", e)))? {

            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "data" {
                    if let Some(file_stem) = path.file_stem() {
                        if let Some(key) = file_stem.to_str() {
                            keys.push(key.to_string());
                        }
                    }
                }
            }
        }

        Ok(keys)
    }

    async fn exists(&self, _key: &str) -> Result<bool, PersistenceError> {
        let file_path = self.get_file_path(_key);
        let exists = tokio::fs::try_exists(&file_path).await
            .map_err(|e| PersistenceError::StorageError(format!("Failed to check file: {}", e)))?;
        Ok(exists)
    }

    async fn clear(&self) -> Result<(), PersistenceError> {
        let keys = self.list_keys().await?;
        for key in keys {
            self.delete(&key).await?;
        }
        Ok(())
    }

    fn info(&self) -> StorageInfo {
        StorageInfo::new("filesystem")
            .supports_compression(true)
            .supports_encryption(true)
    }
}

/// Storage factory for creating storage instances
pub struct StorageFactory;

impl StorageFactory {
    /// Create storage based on type
    pub fn create_storage(storage_type: &StorageType) -> Result<Box<dyn MachineStorage>, PersistenceError> {
        match storage_type {
            StorageType::LocalStorage => {
                if LocalStorage::is_available() {
                    Ok(Box::new(LocalStorage::new()))
                } else {
                    Err(PersistenceError::StorageError("localStorage not available".to_string()))
                }
            }
            StorageType::Memory => Ok(Box::new(MemoryStorage::new())),
            StorageType::FileSystem => {
                // Use current directory as default
                let base_dir = std::env::current_dir()
                    .map_err(|e| PersistenceError::ConfigError(format!("Failed to get current dir: {}", e)))?;
                Ok(Box::new(FileSystemStorage::new(base_dir)))
            }
            _ => Err(PersistenceError::ConfigError(format!("Unsupported storage type: {:?}", storage_type))),
        }
    }

    /// Create storage with custom configuration
    pub fn create_storage_with_config(
        storage_type: &StorageType,
        config: &std::collections::HashMap<String, String>
    ) -> Result<Box<dyn MachineStorage>, PersistenceError> {
        match storage_type {
            StorageType::FileSystem => {
                let base_dir = config.get("base_dir")
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
                Ok(Box::new(FileSystemStorage::new(base_dir)))
            }
            _ => Self::create_storage(storage_type),
        }
    }
}

/// Storage utilities
pub mod utils {
    use super::*;

    /// Test storage connectivity
    pub async fn test_storage(storage: &dyn MachineStorage) -> Result<(), PersistenceError> {
        let test_key = "__test__";
        let test_data = b"test data";

        // Store test data
        storage.store(test_key, test_data).await?;

        // Retrieve test data
        let retrieved = storage.retrieve(test_key).await?;
        if retrieved != test_data {
            return Err(PersistenceError::StorageError("Data integrity check failed".to_string()));
        }

        // Check exists
        let exists = storage.exists(test_key).await?;
        if !exists {
            return Err(PersistenceError::StorageError("Exists check failed".to_string()));
        }

        // List keys (should contain our test key)
        let keys = storage.list_keys().await?;
        if !keys.contains(&test_key.to_string()) {
            return Err(PersistenceError::StorageError("List keys failed".to_string()));
        }

        // Clean up
        storage.delete(test_key).await?;

        Ok(())
    }

    /// Get storage statistics
    pub async fn get_storage_stats(storage: &dyn MachineStorage) -> Result<StorageStats, PersistenceError> {
        let keys = storage.list_keys().await?;
        let mut total_size = 0u64;
        let mut largest_key = String::new();
        let mut largest_size = 0u64;

        for key in &keys {
            if let Ok(data) = storage.retrieve(key).await {
                let size = data.len() as u64;
                total_size += size;

                if size > largest_size {
                    largest_size = size;
                    largest_key = key.clone();
                }
            }
        }

        Ok(StorageStats {
            key_count: keys.len(),
            total_size,
            average_size: if keys.is_empty() { 0.0 } else { total_size as f64 / keys.len() as f64 },
            largest_key,
            largest_size,
        })
    }

    /// Storage statistics
    #[derive(Debug, Clone)]
    pub struct StorageStats {
        /// Number of keys
        pub key_count: usize,
        /// Total size in bytes
        pub total_size: u64,
        /// Average size per key
        pub average_size: f64,
        /// Largest key
        pub largest_key: String,
        /// Largest size
        pub largest_size: u64,
    }

    /// Compress data using gzip
    pub fn compress_data(data: &[u8]) -> Result<Vec<u8>, PersistenceError> {
        use std::io::Write;
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(data)
            .map_err(|e| PersistenceError::SerializationError(format!("Compression failed: {}", e)))?;
        encoder.finish()
            .map_err(|e| PersistenceError::SerializationError(format!("Compression finish failed: {}", e)))
    }

    /// Decompress data using gzip
    pub fn decompress_data(data: &[u8]) -> Result<Vec<u8>, PersistenceError> {
        use std::io::Read;
        let mut decoder = flate2::read::GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| PersistenceError::DeserializationError(format!("Decompression failed: {}", e)))?;
        Ok(decompressed)
    }
}
