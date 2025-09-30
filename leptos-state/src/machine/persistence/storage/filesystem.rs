//! File system storage implementation

use super::core::{MachineStorage, StorageInfo};
use crate::machine::persistence_core::PersistenceError;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

/// File system storage implementation
#[derive(Debug)]
pub struct FileSystemStorage {
    /// Base directory for storage
    base_path: PathBuf,
    /// Maximum capacity
    max_capacity: u64,
}

impl FileSystemStorage {
    /// Create a new file system storage with the given base path
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self, PersistenceError> {
        let base_path = base_path.as_ref().to_path_buf();

        // Create directory if it doesn't exist
        if !base_path.exists() {
            fs::create_dir_all(&base_path)
                .map_err(|e| PersistenceError::StorageError(format!("Failed to create directory: {}", e)))?;
        }

        Ok(Self {
            base_path,
            max_capacity: 0, // Unlimited by default
        })
    }

    /// Create a new file system storage with capacity limit
    pub fn with_capacity<P: AsRef<Path>>(base_path: P, max_capacity: u64) -> Result<Self, PersistenceError> {
        let mut storage = Self::new(base_path)?;
        storage.max_capacity = max_capacity;
        Ok(storage)
    }

    /// Get the file path for a key
    fn get_file_path(&self, key: &str) -> PathBuf {
        // Sanitize key to prevent directory traversal
        let sanitized_key = key.replace("..", "").replace("/", "_").replace("\\", "_");
        self.base_path.join(format!("{}.dat", sanitized_key))
    }

    /// Calculate current directory size
    fn calculate_usage(&self) -> Result<u64, PersistenceError> {
        let mut total_size = 0u64;

        fn calculate_dir_size(dir: &Path, total_size: &mut u64) -> Result<(), PersistenceError> {
            let entries = fs::read_dir(dir)
                .map_err(|e| PersistenceError::StorageError(format!("Failed to read directory: {}", e)))?;

            for entry in entries {
                let entry = entry
                    .map_err(|e| PersistenceError::StorageError(format!("Failed to read entry: {}", e)))?;
                let path = entry.path();

                if path.is_file() {
                    let metadata = entry.metadata()
                        .map_err(|e| PersistenceError::StorageError(format!("Failed to get metadata: {}", e)))?;
                    *total_size += metadata.len();
                } else if path.is_dir() {
                    calculate_dir_size(&path, total_size)?;
                }
            }

            Ok(())
        }

        calculate_dir_size(&self.base_path, &mut total_size)?;
        Ok(total_size)
    }

    /// Check if storing data would exceed capacity
    fn check_capacity(&self, additional_size: u64) -> Result<(), PersistenceError> {
        if self.max_capacity > 0 {
            let current_usage = self.calculate_usage()?;
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
impl MachineStorage for FileSystemStorage {
    async fn store(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError> {
        self.check_capacity(data.len() as u64)?;

        let file_path = self.get_file_path(key);

        // Create parent directories if needed
        if let Some(parent) = file_path.parent() {
            async_fs::create_dir_all(parent)
                .await
                .map_err(|e| PersistenceError::StorageError(format!("Failed to create directories: {}", e)))?;
        }

        async_fs::write(&file_path, data)
            .await
            .map_err(|e| PersistenceError::StorageError(format!("Failed to write file: {}", e)))
    }

    async fn retrieve(&self, key: &str) -> Result<Vec<u8>, PersistenceError> {
        let file_path = self.get_file_path(key);

        if !file_path.exists() {
            return Err(PersistenceError::KeyNotFound(key.to_string()));
        }

        async_fs::read(&file_path)
            .await
            .map_err(|e| PersistenceError::StorageError(format!("Failed to read file: {}", e)))
    }

    async fn delete(&self, key: &str) -> Result<(), PersistenceError> {
        let file_path = self.get_file_path(key);

        if !file_path.exists() {
            return Err(PersistenceError::KeyNotFound(key.to_string()));
        }

        async_fs::remove_file(&file_path)
            .await
            .map_err(|e| PersistenceError::StorageError(format!("Failed to delete file: {}", e)))
    }

    async fn list_keys(&self) -> Result<Vec<String>, PersistenceError> {
        let mut keys = Vec::new();

        fn read_dir_recursive(dir: &Path, keys: &mut Vec<String>, base_path: &Path) -> Result<(), PersistenceError> {
            let entries = fs::read_dir(dir)
                .map_err(|e| PersistenceError::StorageError(format!("Failed to read directory: {}", e)))?;

            for entry in entries {
                let entry = entry
                    .map_err(|e| PersistenceError::StorageError(format!("Failed to read entry: {}", e)))?;
                let path = entry.path();

                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "dat" {
                            if let Some(stem) = path.file_stem() {
                                if let Some(stem_str) = stem.to_str() {
                                    // Convert back from sanitized form
                                    keys.push(stem_str.to_string());
                                }
                            }
                        }
                    }
                } else if path.is_dir() {
                    read_dir_recursive(&path, keys, base_path)?;
                }
            }

            Ok(())
        }

        read_dir_recursive(&self.base_path, &mut keys, &self.base_path)?;
        Ok(keys)
    }

    async fn exists(&self, key: &str) -> Result<bool, PersistenceError> {
        let file_path = self.get_file_path(key);
        Ok(file_path.exists())
    }

    async fn clear(&self) -> Result<(), PersistenceError> {
        // Remove all .dat files in the directory
        fn remove_dat_files(dir: &Path) -> Result<(), PersistenceError> {
            let entries = fs::read_dir(dir)
                .map_err(|e| PersistenceError::StorageError(format!("Failed to read directory: {}", e)))?;

            for entry in entries {
                let entry = entry
                    .map_err(|e| PersistenceError::StorageError(format!("Failed to read entry: {}", e)))?;
                let path = entry.path();

                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "dat" {
                            fs::remove_file(&path)
                                .map_err(|e| PersistenceError::StorageError(format!("Failed to remove file: {}", e)))?;
                        }
                    }
                } else if path.is_dir() {
                    remove_dat_files(&path)?;
                }
            }

            Ok(())
        }

        remove_dat_files(&self.base_path)
    }

    fn info(&self) -> StorageInfo {
        let current_usage = self.calculate_usage().unwrap_or(0);

        StorageInfo::new("filesystem")
            .with_max_capacity(self.max_capacity)
            .with_current_usage(current_usage)
            .with_compression_support(true)
            .with_encryption_support(true)
    }
}
