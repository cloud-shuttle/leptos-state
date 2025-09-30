//! Local storage implementation using web storage

use super::core::{MachineStorage, StorageInfo};
use crate::machine::persistence_core::PersistenceError;

/// Local storage implementation using web storage
#[derive(Debug)]
pub struct LocalStorage;

impl LocalStorage {
    /// Create a new local storage instance
    pub fn new() -> Self {
        Self
    }

    /// Check if local storage is available
    pub fn is_available() -> bool {
        // In a real implementation, this would check if localStorage is available
        // For now, we'll assume it's available in web environments
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|window| window.local_storage().ok())
                .is_some()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false // Not available in non-web environments
        }
    }

    /// Get the underlying web storage
    #[cfg(target_arch = "wasm32")]
    fn get_storage() -> Result<web_sys::Storage, PersistenceError> {
        web_sys::window()
            .and_then(|window| window.local_storage().ok())
            .flatten()
            .ok_or_else(|| PersistenceError::StorageUnavailable("Local storage not available".to_string()))
    }

    /// Convert web_sys error to PersistenceError
    #[cfg(target_arch = "wasm32")]
    fn convert_error(error: wasm_bindgen::JsValue) -> PersistenceError {
        PersistenceError::StorageError(format!("Local storage error: {:?}", error))
    }
}

#[async_trait::async_trait]
impl MachineStorage for LocalStorage {
    async fn store(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError> {
        #[cfg(target_arch = "wasm32")]
        {
            let storage = Self::get_storage()?;
            let data_str = base64::encode(data);
            storage
                .set_item(key, &data_str)
                .map_err(Self::convert_error)?;
            Ok(())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(PersistenceError::StorageUnavailable("Local storage only available in web environments".to_string()))
        }
    }

    async fn retrieve(&self, key: &str) -> Result<Vec<u8>, PersistenceError> {
        #[cfg(target_arch = "wasm32")]
        {
            let storage = Self::get_storage()?;
            let data_str = storage
                .get_item(key)
                .map_err(Self::convert_error)?
                .ok_or_else(|| PersistenceError::KeyNotFound(key.to_string()))?;

            base64::decode(&data_str)
                .map_err(|e| PersistenceError::SerializationError(format!("Base64 decode error: {}", e)))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(PersistenceError::StorageUnavailable("Local storage only available in web environments".to_string()))
        }
    }

    async fn delete(&self, key: &str) -> Result<(), PersistenceError> {
        #[cfg(target_arch = "wasm32")]
        {
            let storage = Self::get_storage()?;
            storage.remove_item(key).map_err(Self::convert_error)?;
            Ok(())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(PersistenceError::StorageUnavailable("Local storage only available in web environments".to_string()))
        }
    }

    async fn list_keys(&self) -> Result<Vec<String>, PersistenceError> {
        #[cfg(target_arch = "wasm32")]
        {
            let storage = Self::get_storage()?;
            let length = storage.length();
            let mut keys = Vec::new();

            for i in 0..length {
                if let Ok(Some(key)) = storage.key(i) {
                    keys.push(key);
                }
            }

            Ok(keys)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(PersistenceError::StorageUnavailable("Local storage only available in web environments".to_string()))
        }
    }

    async fn exists(&self, key: &str) -> Result<bool, PersistenceError> {
        #[cfg(target_arch = "wasm32")]
        {
            let storage = Self::get_storage()?;
            Ok(storage.get_item(key).map_err(Self::convert_error)?.is_some())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(PersistenceError::StorageUnavailable("Local storage only available in web environments".to_string()))
        }
    }

    async fn clear(&self) -> Result<(), PersistenceError> {
        #[cfg(target_arch = "wasm32")]
        {
            let storage = Self::get_storage()?;
            storage.clear().map_err(Self::convert_error)?;
            Ok(())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(PersistenceError::StorageUnavailable("Local storage only available in web environments".to_string()))
        }
    }

    fn info(&self) -> StorageInfo {
        #[cfg(target_arch = "wasm32")]
        {
            // In web environments, localStorage typically has ~5-10MB limit
            StorageInfo::new("local")
                .with_max_capacity(5 * 1024 * 1024) // 5MB
                .with_compression_support(false)
                .with_encryption_support(false)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            StorageInfo::new("local")
        }
    }
}

impl Default for LocalStorage {
    fn default() -> Self {
        Self::new()
    }
}
