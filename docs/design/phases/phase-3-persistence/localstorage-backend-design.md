# LocalStorage Backend Design

## Overview
Implement browser LocalStorage persistence backend for client-side state persistence, enabling automatic state saving and restoration across browser sessions.

## Current State
```rust
// No persistence capabilities
impl<S: State> Store<S> {
    pub fn new(initial: S) -> Self { /* ... */ }
}
```

## Proposed Enhancement
```rust
#[cfg(feature = "localstorage")]
impl<S: SerializableState> Store<S> {
    pub fn with_persistence(key: &str, initial: S) -> Self {
        // Create store with LocalStorage backend
    }
}
```

## Motivation

### Client-Side Persistence
- **Session Continuity**: Maintain state across browser refreshes
- **User Experience**: Preserve user progress and preferences
- **Offline Capability**: Work without server connectivity
- **Development**: Easier debugging with persisted state

### Use Cases
- User preferences and settings
- Form data preservation
- Application state across page reloads
- Shopping cart contents
- User authentication state
- UI layout and customization

## Implementation Details

### Storage Backend Trait
```rust
#[cfg(feature = "web")]
pub trait StorageBackend: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError>;
    fn set(&self, key: &str, value: &str) -> Result<(), StorageError>;
    fn remove(&self, key: &str) -> Result<(), StorageError>;
    fn clear(&self) -> Result<(), StorageError>;
    fn keys(&self) -> Result<Vec<String>, StorageError>;
}
```

### LocalStorage Implementation
```rust
#[cfg(feature = "web")]
pub struct LocalStorageBackend {
    prefix: String,
}

#[cfg(feature = "web")]
impl LocalStorageBackend {
    pub fn new() -> Result<Self, StorageError> {
        // Check if LocalStorage is available
        if Self::is_available() {
            Ok(Self {
                prefix: "leptos-state".to_string(),
            })
        } else {
            Err(StorageError::NotAvailable)
        }
    }

    pub fn with_prefix(prefix: String) -> Result<Self, StorageError> {
        if Self::is_available() {
            Ok(Self { prefix })
        } else {
            Err(StorageError::NotAvailable)
        }
    }

    fn is_available() -> bool {
        // Check if running in browser environment
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

    fn prefixed_key(&self, key: &str) -> String {
        format!("{}:{}", self.prefix, key)
    }
}

#[cfg(feature = "web")]
impl StorageBackend for LocalStorageBackend {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        let prefixed_key = self.prefixed_key(key);

        #[cfg(target_arch = "wasm32")]
        {
            let storage = web_sys::window()
                .ok_or(StorageError::NotAvailable)?
                .local_storage()
                .map_err(|_| StorageError::NotAvailable)?
                .ok_or(StorageError::NotAvailable)?;

            match storage.get_item(&prefixed_key) {
                Ok(Some(value)) => Ok(Some(value)),
                Ok(None) => Ok(None),
                Err(_) => Err(StorageError::AccessDenied),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(StorageError::NotAvailable)
        }
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        let prefixed_key = self.prefixed_key(key);

        #[cfg(target_arch = "wasm32")]
        {
            let storage = web_sys::window()
                .ok_or(StorageError::NotAvailable)?
                .local_storage()
                .map_err(|_| StorageError::NotAvailable)?
                .ok_or(StorageError::NotAvailable)?;

            storage.set_item(&prefixed_key, value)
                .map_err(|_| StorageError::StorageFull)?;

            Ok(())
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(StorageError::NotAvailable)
        }
    }

    fn remove(&self, key: &str) -> Result<(), StorageError> {
        let prefixed_key = self.prefixed_key(key);

        #[cfg(target_arch = "wasm32")]
        {
            let storage = web_sys::window()
                .ok_or(StorageError::NotAvailable)?
                .local_storage()
                .map_err(|_| StorageError::NotAvailable)?
                .ok_or(StorageError::NotAvailable)?;

            storage.remove_item(&prefixed_key)
                .map_err(|_| StorageError::AccessDenied)?;

            Ok(())
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(StorageError::NotAvailable)
        }
    }

    fn clear(&self) -> Result<(), StorageError> {
        #[cfg(target_arch = "wasm32")]
        {
            let storage = web_sys::window()
                .ok_or(StorageError::NotAvailable)?
                .local_storage()
                .map_err(|_| StorageError::NotAvailable)?
                .ok_or(StorageError::NotAvailable)?;

            storage.clear()
                .map_err(|_| StorageError::AccessDenied)?;

            Ok(())
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(StorageError::NotAvailable)
        }
    }

    fn keys(&self) -> Result<Vec<String>, StorageError> {
        #[cfg(target_arch = "wasm32")]
        {
            let storage = web_sys::window()
                .ok_or(StorageError::NotAvailable)?
                .local_storage()
                .map_err(|_| StorageError::NotAvailable)?
                .ok_or(StorageError::NotAvailable)?;

            let length = storage.length()
                .map_err(|_| StorageError::AccessDenied)?;

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

        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(StorageError::NotAvailable)
        }
    }
}
```

### Persistent Store Implementation
```rust
#[cfg(feature = "web")]
pub struct PersistentStore<S: SerializableState> {
    store: Store<S>,
    backend: Box<dyn StorageBackend>,
    key: String,
    auto_save: bool,
}

#[cfg(feature = "web")]
impl<S: SerializableState> PersistentStore<S> {
    pub fn new(
        key: String,
        initial: S,
        backend: Box<dyn StorageBackend>
    ) -> Result<Self, StoreError> {
        let mut store = Store::new(initial);

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

    pub fn with_auto_save(mut self, auto_save: bool) -> Self {
        self.auto_save = auto_save;
        self
    }

    pub fn save(&self) -> Result<(), StoreError> {
        let json = self.store.to_json()?;
        self.backend.set(&self.key, &json)?;
        Ok(())
    }

    pub fn load(&self) -> Result<(), StoreError> {
        if let Some(json) = self.backend.get(&self.key)? {
            self.store.from_json(&json)?;
        }
        Ok(())
    }

    pub fn clear(&self) -> Result<(), StoreError> {
        self.backend.remove(&self.key)?;
        Ok(())
    }

    pub fn get(&self) -> ReadSignal<S> {
        self.store.get()
    }

    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        self.store.update(updater)?;
        if self.auto_save {
            self.save()?;
        }
        Ok(())
    }

    pub fn set(&self, state: S) -> Result<(), StoreError> {
        self.store.set(state)?;
        if self.auto_save {
            self.save()?;
        }
        Ok(())
    }
}
```

### Hook Integration
```rust
#[cfg(feature = "web")]
pub fn use_persistent_store<S: SerializableState + Default>(
    key: &str
) -> Result<(ReadSignal<S>, PersistentStoreActions<S>), StoreError> {
    let backend = LocalStorageBackend::new()?;
    let persistent_store = PersistentStore::new(key.to_string(), S::default(), Box::new(backend))?;
    let signal = persistent_store.get();
    let actions = PersistentStoreActions {
        store: persistent_store,
    };

    Ok((signal, actions))
}

#[cfg(feature = "web")]
#[derive(Clone)]
pub struct PersistentStoreActions<S: SerializableState> {
    store: PersistentStore<S>,
}

#[cfg(feature = "web")]
impl<S: SerializableState> PersistentStoreActions<S> {
    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        self.store.update(updater)
    }

    pub fn set(&self, state: S) -> Result<(), StoreError> {
        self.store.set(state)
    }

    pub fn save(&self) -> Result<(), StoreError> {
        self.store.save()
    }

    pub fn load(&self) -> Result<(), StoreError> {
        self.store.load()
    }

    pub fn clear(&self) -> Result<(), StoreError> {
        self.store.clear()
    }
}
```

## Error Handling

### Storage Errors
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum StorageError {
    #[error("Storage not available in this environment")]
    NotAvailable,

    #[error("Storage access denied")]
    AccessDenied,

    #[error("Storage quota exceeded")]
    StorageFull,

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Invalid key format: {0}")]
    InvalidKey(String),
}
```

### Graceful Degradation
```rust
#[cfg(feature = "web")]
pub fn use_persistent_store_graceful<S: SerializableState + Default>(
    key: &str
) -> (ReadSignal<S>, PersistentStoreActions<S>) {
    match use_persistent_store(key) {
        Ok((signal, actions)) => (signal, actions),
        Err(_) => {
            // Fallback to regular store if persistence fails
            let (signal, actions) = use_store::<S>();
            let graceful_actions = PersistentStoreActions::graceful(actions);
            (signal, graceful_actions)
        }
    }
}

#[cfg(feature = "web")]
impl<S: SerializableState> PersistentStoreActions<S> {
    pub fn graceful(store_actions: StoreActions<S>) -> Self {
        // Wrap regular store actions with no-op persistence methods
        todo!()
    }
}
```

## Storage Limits and Quotas

### Browser Quotas
```rust
#[cfg(feature = "web")]
impl LocalStorageBackend {
    pub fn get_quota_info(&self) -> Result<QuotaInfo, StorageError> {
        // Estimate used space (not directly available in web APIs)
        // This is approximate and browser-dependent
        let keys = self.keys()?;
        let mut total_size = 0;

        for key in keys {
            if let Some(value) = self.get(&key)? {
                total_size += key.len() + value.len();
            }
        }

        Ok(QuotaInfo {
            used_bytes: total_size,
            // Browsers typically allow 5-10MB, but this varies
            max_bytes: Some(5 * 1024 * 1024), // 5MB conservative estimate
        })
    }
}

#[derive(Clone, Debug)]
pub struct QuotaInfo {
    pub used_bytes: usize,
    pub max_bytes: Option<usize>,
}
```

### Compression for Large Data
```rust
#[cfg(feature = "web")]
impl LocalStorageBackend {
    pub fn set_compressed(&self, key: &str, value: &str) -> Result<(), StorageError> {
        use base64::Engine;
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(value.as_bytes())?;
        let compressed = encoder.finish()?;

        let encoded = base64::engine::general_purpose::STANDARD.encode(&compressed);
        self.set(key, &encoded)?;
        Ok(())
    }

    pub fn get_compressed(&self, key: &str) -> Result<Option<String>, StorageError> {
        use base64::Engine;
        use flate2::read::GzDecoder;
        use std::io::Read;

        if let Some(encoded) = self.get(key)? {
            let compressed = base64::engine::general_purpose::STANDARD
                .decode(encoded.as_bytes())
                .map_err(|_| StorageError::Deserialization("Invalid base64".to_string()))?;

            let mut decoder = GzDecoder::new(&compressed[..]);
            let mut decompressed = String::new();
            decoder.read_to_string(&mut decompressed)?;

            Ok(Some(decompressed))
        } else {
            Ok(None)
        }
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(feature = "web")]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn localstorage_basic_operations() {
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
        let backend = LocalStorageBackend::new().unwrap();

        #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
        struct TestState { count: i32 }

        impl SerializableState for TestState {}

        let store = PersistentStore::new(
            "test_store".to_string(),
            TestState { count: 0 },
            Box::new(backend)
        ).unwrap();

        // Update state
        store.update(|s| s.count = 42).unwrap();

        // Create new store - should load persisted state
        let new_store = PersistentStore::new(
            "test_store".to_string(),
            TestState { count: 0 },
            Box::new(LocalStorageBackend::new().unwrap())
        ).unwrap();

        assert_eq!(new_store.store.get().get_untracked().count, 42);
    }
}
```

### Integration Tests
```rust
#[cfg(feature = "web")]
#[wasm_bindgen_test]
fn hook_integration() {
    wasm_bindgen_test::wasm_bindgen_test_block!(async {
        // This would require a Leptos test environment
        // For now, test the core functionality
        let backend = LocalStorageBackend::new().unwrap();

        #[derive(Clone, Serialize, Deserialize, Default)]
        struct TestState { value: String }
        impl SerializableState for TestState {}

        let store = PersistentStore::new(
            "hook_test".to_string(),
            TestState::default(),
            Box::new(backend)
        ).unwrap();

        store.update(|s| s.value = "integrated".to_string()).unwrap();

        // Verify persistence
        let new_store = PersistentStore::new(
            "hook_test".to_string(),
            TestState::default(),
            Box::new(LocalStorageBackend::new().unwrap())
        ).unwrap();

        assert_eq!(new_store.store.get().get_untracked().value, "integrated");
    });
}
```

### Error Handling Tests
```rust
#[cfg(feature = "web")]
#[wasm_bindgen_test]
fn error_handling() {
    // Test behavior when LocalStorage is not available
    // (This is hard to test in a real browser environment)

    // Test serialization errors
    let backend = LocalStorageBackend::new().unwrap();

    // Invalid JSON should not crash
    let result = backend.set("invalid", "{invalid json");
    // This should succeed (LocalStorage doesn't validate JSON)
    assert!(result.is_ok());
}
```

## Performance Impact

### Browser Storage Performance
- **LocalStorage**: Synchronous, blocks main thread
- **Access Time**: Fast for small data (< 5ms typical)
- **Storage Limit**: ~5-10MB depending on browser
- **Parsing**: JSON overhead for complex objects

### Optimization Strategies
```rust
#[cfg(feature = "web")]
impl<S: SerializableState> PersistentStore<S> {
    pub fn with_debounced_save(mut self, delay_ms: u32) -> Self {
        // Implement debounced auto-save to reduce writes
        todo!()
    }

    pub fn with_compression(mut self, enable: bool) -> Self {
        // Enable compression for large state objects
        todo!()
    }

    pub fn with_change_detection(mut self, enable: bool) -> Self {
        // Only save when state actually changes
        todo!()
    }
}
```

## Security Considerations

### Data Exposure
- LocalStorage is accessible to all scripts on the domain
- Sensitive data should not be stored in LocalStorage
- Use encryption for sensitive persisted data

```rust
#[cfg(feature = "web")]
impl LocalStorageBackend {
    pub fn set_encrypted(&self, key: &str, value: &str, key: &str) -> Result<(), StorageError> {
        // Encrypt value before storing
        let encrypted = encrypt_with_key(value, key)?;
        self.set(key, &encrypted)
    }

    pub fn get_encrypted(&self, key: &str, decryption_key: &str) -> Result<Option<String>, StorageError> {
        if let Some(encrypted) = self.get(key)? {
            let decrypted = decrypt_with_key(&encrypted, decryption_key)?;
            Ok(Some(decrypted))
        } else {
            Ok(None)
        }
    }
}
```

### Cross-Site Scripting (XSS)
- Validate data before storing
- Sanitize data when loading
- Use Content Security Policy (CSP)

### Data Integrity
- Validate data structure on load
- Handle corrupted data gracefully
- Implement data migration for schema changes

## Migration Guide

### Adding Persistence to Existing Stores
```rust
// Before - regular store
let (state, actions) = use_store::<MyState>();

// After - persistent store
let (state, actions) = use_persistent_store::<MyState>("my-app-state")
    .unwrap_or_else(|_| use_store::<MyState>()); // Fallback
```

### Gradual Migration
```rust
// Phase 1: Add optional persistence
#[derive(Clone, Serialize, Deserialize)]
struct MyState { /* ... */ }

pub fn create_store(persist: bool) -> Store<MyState> {
    if persist {
        // Try persistent store
        match use_persistent_store("my-state") {
            Ok((signal, actions)) => return (signal, actions.into()),
            Err(_) => {} // Fall back to regular store
        }
    }

    // Regular store
    use_store::<MyState>()
}

// Phase 2: Make persistence default
pub fn create_store() -> Store<MyState> {
    use_persistent_store("my-state")
        .unwrap_or_else(|_| use_store::<MyState>())
}
```

### Data Migration
```rust
#[cfg(feature = "web")]
pub fn migrate_legacy_data() -> Result<(), StorageError> {
    let backend = LocalStorageBackend::new()?;

    // Check for old data format
    if let Some(old_data) = backend.get("old_key")? {
        // Migrate to new format
        let migrated_data = migrate_old_to_new(&old_data)?;
        backend.set("new_key", &migrated_data)?;
        backend.remove("old_key")?;
    }

    Ok(())
}
```

## Future Extensions

### IndexedDB Fallback
```rust
#[cfg(feature = "web")]
pub enum WebStorageBackend {
    LocalStorage(LocalStorageBackend),
    IndexedDB(IndexedDBBackend),
}

#[cfg(feature = "web")]
impl WebStorageBackend {
    pub fn best_available() -> Result<Self, StorageError> {
        // Try IndexedDB first (better performance), fallback to LocalStorage
        IndexedDBBackend::new()
            .map(Self::IndexedDB)
            .or_else(|_| LocalStorageBackend::new().map(Self::LocalStorage))
    }
}
```

### Sync Across Tabs
```rust
#[cfg(feature = "web")]
pub struct SyncedStore<S: SerializableState> {
    store: PersistentStore<S>,
    storage_listener: Option<Closure<dyn FnMut(web_sys::StorageEvent)>>,
}

#[cfg(feature = "web")]
impl<S: SerializableState> SyncedStore<S> {
    pub fn new(key: String, initial: S) -> Result<Self, StoreError> {
        let store = PersistentStore::new(key.clone(), initial, /* ... */)?;

        // Listen for storage changes from other tabs
        let storage_listener = Self::setup_storage_listener(key, store.clone());

        Ok(Self {
            store,
            storage_listener: Some(storage_listener),
        })
    }

    fn setup_storage_listener(key: String, store: PersistentStore<S>) -> Closure<dyn FnMut(web_sys::StorageEvent)> {
        Closure::wrap(Box::new(move |event: web_sys::StorageEvent| {
            if event.key().as_deref() == Some(&key) {
                if let Some(new_value) = event.new_value() {
                    // Update local store when storage changes
                    if let Ok(state) = serde_json::from_str(&new_value) {
                        let _ = store.set(state);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>)
    }
}
```

## Risk Assessment

### Likelihood: Medium
- Browser storage APIs are stable but can have edge cases
- LocalStorage has known limitations (blocking, size limits)
- Environment detection is critical for non-browser usage

### Impact: Medium
- Graceful degradation when storage unavailable
- Clear error messages guide users
- Feature-gated to avoid mandatory browser dependencies

### Mitigation
- Comprehensive browser testing across different environments
- Fallback strategies for when LocalStorage fails
- Clear documentation on limitations and alternatives
- Environment detection to prevent runtime panics
