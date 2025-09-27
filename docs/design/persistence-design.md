# 💾 Persistence Layer Design

## Overview
Design document for the persistence system - enabling automatic state persistence across browser sessions and platforms.

## Core Persistence Principles

### 1. Platform Agnostic
Support multiple storage backends (localStorage, IndexedDB, file system, remote storage).

### 2. Automatic Synchronization
State changes automatically saved without manual intervention.

### 3. Error Resilient
Graceful handling of storage failures and corrupted data.

### 4. Performance Optimized
Minimal overhead with intelligent batching and caching.

## Persistence Architecture

### Storage Backend Trait

```rust
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    /// Save data with given key
    async fn save(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError>;
    
    /// Load data by key
    async fn load(&self, key: &str) -> Result<Vec<u8>, PersistenceError>;
    
    /// Delete data by key
    async fn delete(&self, key: &str) -> Result<(), PersistenceError>;
    
    /// List all keys
    async fn list_keys(&self) -> Result<Vec<String>, PersistenceError>;
    
    /// Check if key exists
    async fn exists(&self, key: &str) -> Result<bool, PersistenceError>;
    
    /// Get storage info (size, capacity, etc.)
    async fn info(&self) -> Result<StorageInfo, PersistenceError>;
    
    /// Clear all data
    async fn clear(&self) -> Result<(), PersistenceError>;
}

#[derive(Debug, Clone)]
pub struct StorageInfo {
    pub used_bytes: u64,
    pub available_bytes: Option<u64>, // None for unlimited
    pub entry_count: usize,
}
```

### Persistence Manager

```rust
pub struct PersistenceManager<T> {
    backend: Arc<dyn StorageBackend>,
    config: PersistenceConfig,
    serializer: Box<dyn Serializer<T>>,
    cache: Arc<RwLock<LruCache<String, CachedEntry<T>>>>,
    pending_writes: Arc<Mutex<HashMap<String, PendingWrite<T>>>>,
}

impl<T> PersistenceManager<T> 
where T: Clone + Send + Sync + 'static
{
    pub fn new(
        backend: Arc<dyn StorageBackend>,
        config: PersistenceConfig,
        serializer: Box<dyn Serializer<T>>,
    ) -> Self {
        Self {
            backend,
            config,
            serializer,
            cache: Arc::new(RwLock::new(LruCache::new(config.cache_size))),
            pending_writes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn save(&self, key: &str, value: &T) -> Result<(), PersistenceError> {
        // Serialize the value
        let data = self.serializer.serialize(value)?;
        
        // Handle batching if enabled
        if self.config.batch_writes {
            self.add_to_batch(key.to_string(), value.clone(), data).await?;
        } else {
            self.backend.save(key, &data).await?;
        }
        
        // Update cache
        self.update_cache(key, value.clone());
        
        Ok(())
    }
    
    pub async fn load(&self, key: &str) -> Result<Option<T>, PersistenceError> {
        // Check cache first
        if let Some(cached) = self.get_from_cache(key) {
            return Ok(Some(cached));
        }
        
        // Load from backend
        match self.backend.load(key).await {
            Ok(data) => {
                let value = self.serializer.deserialize(&data)?;
                self.update_cache(key, value.clone());
                Ok(Some(value))
            }
            Err(PersistenceError::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
```

## Storage Backend Implementations

### 1. LocalStorage Backend (WASM)

```rust
pub struct LocalStorageBackend {
    prefix: String,
}

impl LocalStorageBackend {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self { prefix: prefix.into() }
    }
    
    fn get_storage(&self) -> Result<web_sys::Storage, PersistenceError> {
        web_sys::window()
            .ok_or(PersistenceError::BackendUnavailable)?
            .local_storage()
            .map_err(|_| PersistenceError::BackendUnavailable)?
            .ok_or(PersistenceError::BackendUnavailable)
    }
    
    fn prefixed_key(&self, key: &str) -> String {
        format!("{}:{}", self.prefix, key)
    }
}

#[async_trait::async_trait]
impl StorageBackend for LocalStorageBackend {
    async fn save(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError> {
        let storage = self.get_storage()?;
        let prefixed_key = self.prefixed_key(key);
        
        // Convert bytes to base64 for localStorage
        let encoded = base64::encode(data);
        
        storage
            .set_item(&prefixed_key, &encoded)
            .map_err(|_| PersistenceError::WriteError)?;
            
        Ok(())
    }
    
    async fn load(&self, key: &str) -> Result<Vec<u8>, PersistenceError> {
        let storage = self.get_storage()?;
        let prefixed_key = self.prefixed_key(key);
        
        let encoded = storage
            .get_item(&prefixed_key)
            .map_err(|_| PersistenceError::ReadError)?
            .ok_or(PersistenceError::NotFound)?;
            
        let data = base64::decode(encoded)
            .map_err(|_| PersistenceError::CorruptedData)?;
            
        Ok(data)
    }
    
    async fn delete(&self, key: &str) -> Result<(), PersistenceError> {
        let storage = self.get_storage()?;
        let prefixed_key = self.prefixed_key(key);
        
        storage
            .remove_item(&prefixed_key)
            .map_err(|_| PersistenceError::WriteError)?;
            
        Ok(())
    }
    
    async fn info(&self) -> Result<StorageInfo, PersistenceError> {
        let storage = self.get_storage()?;
        let mut used_bytes = 0u64;
        let mut entry_count = 0usize;
        
        // Estimate storage usage
        for i in 0..storage.length().unwrap_or(0) {
            if let Ok(Some(key)) = storage.key(i) {
                if key.starts_with(&self.prefix) {
                    if let Ok(Some(value)) = storage.get_item(&key) {
                        used_bytes += (key.len() + value.len()) as u64;
                        entry_count += 1;
                    }
                }
            }
        }
        
        Ok(StorageInfo {
            used_bytes,
            available_bytes: Some(5_242_880), // ~5MB typical localStorage limit
            entry_count,
        })
    }
}
```

### 2. IndexedDB Backend (WASM)

```rust
pub struct IndexedDBBackend {
    db_name: String,
    store_name: String,
    version: u32,
}

impl IndexedDBBackend {
    pub fn new(db_name: impl Into<String>, store_name: impl Into<String>) -> Self {
        Self {
            db_name: db_name.into(),
            store_name: store_name.into(),
            version: 1,
        }
    }
    
    async fn get_database(&self) -> Result<web_sys::IdbDatabase, PersistenceError> {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen_futures::JsFuture;
        
        let window = web_sys::window().ok_or(PersistenceError::BackendUnavailable)?;
        let idb = window
            .indexed_db()
            .map_err(|_| PersistenceError::BackendUnavailable)?
            .ok_or(PersistenceError::BackendUnavailable)?;
            
        let request = idb
            .open_with_u32(&self.db_name, self.version)
            .map_err(|_| PersistenceError::BackendUnavailable)?;
            
        // Handle database creation/upgrade
        let request_clone = request.clone();
        let store_name = self.store_name.clone();
        let onupgradeneeded = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let request: web_sys::IdbOpenDbRequest = target.dyn_into().unwrap();
            let db = request.result().unwrap().dyn_into::<web_sys::IdbDatabase>().unwrap();
            
            if !db.object_store_names().contains(&store_name) {
                let _ = db.create_object_store(&store_name);
            }
        }) as Box<dyn FnMut(_)>);
        
        request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));
        onupgradeneeded.forget();
        
        let future = JsFuture::from(request);
        let result = future.await.map_err(|_| PersistenceError::BackendUnavailable)?;
        let db: web_sys::IdbDatabase = result.dyn_into().unwrap();
        
        Ok(db)
    }
}

#[async_trait::async_trait]
impl StorageBackend for IndexedDBBackend {
    async fn save(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError> {
        use wasm_bindgen_futures::JsFuture;
        
        let db = self.get_database().await?;
        let transaction = db
            .transaction_with_str_and_mode(&self.store_name, web_sys::IdbTransactionMode::Readwrite)
            .map_err(|_| PersistenceError::WriteError)?;
            
        let store = transaction
            .object_store(&self.store_name)
            .map_err(|_| PersistenceError::WriteError)?;
            
        // Convert data to Uint8Array
        let array = js_sys::Uint8Array::new_with_length(data.len() as u32);
        array.copy_from(data);
        
        let request = store
            .put_with_key(&array, &JsValue::from_str(key))
            .map_err(|_| PersistenceError::WriteError)?;
            
        JsFuture::from(request).await
            .map_err(|_| PersistenceError::WriteError)?;
            
        Ok(())
    }
    
    async fn load(&self, key: &str) -> Result<Vec<u8>, PersistenceError> {
        use wasm_bindgen_futures::JsFuture;
        
        let db = self.get_database().await?;
        let transaction = db
            .transaction_with_str(&self.store_name)
            .map_err(|_| PersistenceError::ReadError)?;
            
        let store = transaction
            .object_store(&self.store_name)
            .map_err(|_| PersistenceError::ReadError)?;
            
        let request = store
            .get(&JsValue::from_str(key))
            .map_err(|_| PersistenceError::ReadError)?;
            
        let result = JsFuture::from(request).await
            .map_err(|_| PersistenceError::ReadError)?;
            
        if result.is_undefined() || result.is_null() {
            return Err(PersistenceError::NotFound);
        }
        
        let array: js_sys::Uint8Array = result.dyn_into()
            .map_err(|_| PersistenceError::CorruptedData)?;
            
        let mut data = vec![0u8; array.length() as usize];
        array.copy_to(&mut data);
        
        Ok(data)
    }
}
```

### 3. File System Backend (Native)

```rust
pub struct FileSystemBackend {
    base_path: PathBuf,
}

impl FileSystemBackend {
    pub fn new(base_path: impl Into<PathBuf>) -> Result<Self, PersistenceError> {
        let base_path = base_path.into();
        
        // Create directory if it doesn't exist
        std::fs::create_dir_all(&base_path)
            .map_err(|_| PersistenceError::BackendUnavailable)?;
            
        Ok(Self { base_path })
    }
    
    fn key_to_path(&self, key: &str) -> PathBuf {
        // Sanitize key for filesystem
        let safe_key = key.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        self.base_path.join(format!("{}.dat", safe_key))
    }
}

#[async_trait::async_trait]
impl StorageBackend for FileSystemBackend {
    async fn save(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError> {
        let path = self.key_to_path(key);
        
        // Write to temporary file first, then rename for atomicity
        let temp_path = path.with_extension("tmp");
        
        tokio::fs::write(&temp_path, data).await
            .map_err(|_| PersistenceError::WriteError)?;
            
        tokio::fs::rename(&temp_path, &path).await
            .map_err(|_| PersistenceError::WriteError)?;
            
        Ok(())
    }
    
    async fn load(&self, key: &str) -> Result<Vec<u8>, PersistenceError> {
        let path = self.key_to_path(key);
        
        if !path.exists() {
            return Err(PersistenceError::NotFound);
        }
        
        tokio::fs::read(&path).await
            .map_err(|_| PersistenceError::ReadError)
    }
    
    async fn delete(&self, key: &str) -> Result<(), PersistenceError> {
        let path = self.key_to_path(key);
        
        if path.exists() {
            tokio::fs::remove_file(&path).await
                .map_err(|_| PersistenceError::WriteError)?;
        }
        
        Ok(())
    }
    
    async fn info(&self) -> Result<StorageInfo, PersistenceError> {
        let mut used_bytes = 0u64;
        let mut entry_count = 0usize;
        
        let mut dir = tokio::fs::read_dir(&self.base_path).await
            .map_err(|_| PersistenceError::ReadError)?;
            
        while let Ok(Some(entry)) = dir.next_entry().await {
            if let Ok(metadata) = entry.metadata().await {
                if metadata.is_file() && entry.file_name().to_string_lossy().ends_with(".dat") {
                    used_bytes += metadata.len();
                    entry_count += 1;
                }
            }
        }
        
        Ok(StorageInfo {
            used_bytes,
            available_bytes: None, // Filesystem limit varies
            entry_count,
        })
    }
}
```

## Serialization Layer

### Serializer Trait

```rust
pub trait Serializer<T>: Send + Sync {
    fn serialize(&self, value: &T) -> Result<Vec<u8>, SerializationError>;
    fn deserialize(&self, data: &[u8]) -> Result<T, SerializationError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SerializationError {
    #[error("Serialization failed: {0}")]
    SerializeFailed(String),
    
    #[error("Deserialization failed: {0}")]
    DeserializeFailed(String),
    
    #[error("Unsupported format")]
    UnsupportedFormat,
}
```

### Built-in Serializers

```rust
pub struct JsonSerializer<T> {
    _phantom: PhantomData<T>,
}

impl<T> JsonSerializer<T> {
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<T> Serializer<T> for JsonSerializer<T>
where T: Serialize + for<'de> Deserialize<'de>
{
    fn serialize(&self, value: &T) -> Result<Vec<u8>, SerializationError> {
        serde_json::to_vec(value)
            .map_err(|e| SerializationError::SerializeFailed(e.to_string()))
    }
    
    fn deserialize(&self, data: &[u8]) -> Result<T, SerializationError> {
        serde_json::from_slice(data)
            .map_err(|e| SerializationError::DeserializeFailed(e.to_string()))
    }
}

pub struct BincodeSerializer<T> {
    _phantom: PhantomData<T>,
}

impl<T> Serializer<T> for BincodeSerializer<T>
where T: Serialize + for<'de> Deserialize<'de>
{
    fn serialize(&self, value: &T) -> Result<Vec<u8>, SerializationError> {
        bincode::serialize(value)
            .map_err(|e| SerializationError::SerializeFailed(e.to_string()))
    }
    
    fn deserialize(&self, data: &[u8]) -> Result<T, SerializationError> {
        bincode::deserialize(data)
            .map_err(|e| SerializationError::DeserializeFailed(e.to_string()))
    }
}
```

## Configuration System

### Persistence Configuration

```rust
#[derive(Debug, Clone)]
pub struct PersistenceConfig {
    /// Storage key prefix
    pub key_prefix: String,
    
    /// Auto-save after every state change
    pub auto_save: bool,
    
    /// Debounce time for auto-save (milliseconds)
    pub save_debounce_ms: u64,
    
    /// Batch multiple writes
    pub batch_writes: bool,
    
    /// Batch flush interval (milliseconds)
    pub batch_flush_interval_ms: u64,
    
    /// Maximum batch size
    pub max_batch_size: usize,
    
    /// Enable compression
    pub compression: CompressionConfig,
    
    /// Encryption settings
    pub encryption: Option<EncryptionConfig>,
    
    /// Backup configuration
    pub backup: BackupConfig,
    
    /// Cache settings
    pub cache_size: usize,
    
    /// Error handling strategy
    pub error_strategy: ErrorStrategy,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            key_prefix: "leptos_state".to_string(),
            auto_save: true,
            save_debounce_ms: 100,
            batch_writes: true,
            batch_flush_interval_ms: 1000,
            max_batch_size: 100,
            compression: CompressionConfig::default(),
            encryption: None,
            backup: BackupConfig::default(),
            cache_size: 1000,
            error_strategy: ErrorStrategy::LogAndContinue,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub algorithm: CompressionAlgorithm,
    pub level: u8,
}

#[derive(Debug, Clone)]
pub enum CompressionAlgorithm {
    Gzip,
    Zstd,
    Lz4,
}

#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    pub algorithm: EncryptionAlgorithm,
    pub key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
}

#[derive(Debug, Clone)]
pub struct BackupConfig {
    pub enabled: bool,
    pub max_backups: usize,
    pub backup_interval_ms: u64,
}

#[derive(Debug, Clone)]
pub enum ErrorStrategy {
    LogAndContinue,
    RetryOnce,
    RetryWithBackoff,
    FailFast,
}
```

## Auto-persistence Integration

### Store Integration

```rust
pub fn create_persistent_store<T>(
    initial: T,
    config: PersistenceConfig,
) -> (ReadSignal<T>, WriteSignal<T>, PersistenceHandle)
where T: Clone + Serialize + for<'de> Deserialize<'de> + 'static
{
    let (read_signal, write_signal) = create_signal(initial.clone());
    
    // Create persistence manager
    let backend = create_backend_for_platform(&config);
    let serializer = Box::new(JsonSerializer::new());
    let manager = PersistenceManager::new(backend, config.clone(), serializer);
    
    // Load persisted value
    let key = format!("{}:store", config.key_prefix);
    spawn_local({
        let manager = manager.clone();
        let write_signal = write_signal.clone();
        let key = key.clone();
        
        async move {
            if let Ok(Some(persisted)) = manager.load(&key).await {
                write_signal.set(persisted);
            }
        }
    });
    
    // Auto-save on changes
    let save_manager = manager.clone();
    let save_key = key.clone();
    create_effect(move |_| {
        let value = read_signal.get();
        let manager = save_manager.clone();
        let key = save_key.clone();
        
        spawn_local(async move {
            if let Err(e) = manager.save(&key, &value).await {
                tracing::error!("Failed to persist store: {}", e);
            }
        });
    });
    
    let handle = PersistenceHandle { manager, key };
    
    (read_signal, write_signal, handle)
}

#[cfg(target_arch = "wasm32")]
fn create_backend_for_platform(config: &PersistenceConfig) -> Arc<dyn StorageBackend> {
    if config.use_indexeddb {
        Arc::new(IndexedDBBackend::new("leptos_state", "main"))
    } else {
        Arc::new(LocalStorageBackend::new(&config.key_prefix))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn create_backend_for_platform(config: &PersistenceConfig) -> Arc<dyn StorageBackend> {
    let path = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".leptos_state");
        
    Arc::new(FileSystemBackend::new(path).unwrap())
}
```

### Machine Integration

```rust
impl<S, E, C> Machine<S, E, C> 
where 
    S: Clone + PartialEq + Serialize + for<'de> Deserialize<'de> + 'static,
    E: Clone + 'static,
    C: Clone + Serialize + for<'de> Deserialize<'de> + 'static,
{
    pub fn with_persistence(self, config: PersistenceConfig) -> PersistedMachine<S, E, C> {
        PersistedMachine::new(self, config)
    }
}

pub struct PersistedMachine<S, E, C> {
    machine: Machine<S, E, C>,
    persistence: PersistenceManager<MachineSnapshot<S, C>>,
    config: PersistenceConfig,
}

#[derive(Serialize, Deserialize)]
struct MachineSnapshot<S, C> {
    current_state: S,
    context: C,
    timestamp: u64,
}

impl<S, E, C> PersistedMachine<S, E, C> 
where 
    S: Clone + PartialEq + Serialize + for<'de> Deserialize<'de> + 'static,
    E: Clone + 'static,
    C: Clone + Serialize + for<'de> Deserialize<'de> + 'static,
{
    pub fn new(machine: Machine<S, E, C>, config: PersistenceConfig) -> Self {
        let backend = create_backend_for_platform(&config);
        let serializer = Box::new(JsonSerializer::new());
        let persistence = PersistenceManager::new(backend, config.clone(), serializer);
        
        Self { machine, persistence, config }
    }
    
    pub async fn load_state(&mut self) -> Result<(), PersistenceError> {
        let key = format!("{}:machine", self.config.key_prefix);
        
        if let Some(snapshot) = self.persistence.load(&key).await? {
            self.machine.current_state = snapshot.current_state;
            self.machine.context = snapshot.context;
        }
        
        Ok(())
    }
    
    pub async fn save_state(&self) -> Result<(), PersistenceError> {
        let snapshot = MachineSnapshot {
            current_state: self.machine.current_state.clone(),
            context: self.machine.context.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let key = format!("{}:machine", self.config.key_prefix);
        self.persistence.save(&key, &snapshot).await
    }
}
```

## Error Handling and Recovery

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Storage backend unavailable")]
    BackendUnavailable,
    
    #[error("Key not found: {key}")]
    NotFound { key: String },
    
    #[error("Failed to read data")]
    ReadError,
    
    #[error("Failed to write data")]
    WriteError,
    
    #[error("Data is corrupted")]
    CorruptedData,
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] SerializationError),
    
    #[error("Storage quota exceeded")]
    QuotaExceeded,
    
    #[error("Access denied")]
    AccessDenied,
}
```

### Recovery Strategies

```rust
pub struct RecoveryManager<T> {
    fallback_backends: Vec<Arc<dyn StorageBackend>>,
    validation: Box<dyn Fn(&T) -> bool>,
}

impl<T> RecoveryManager<T> 
where T: Clone + Serialize + for<'de> Deserialize<'de>
{
    pub async fn recover_data(&self, key: &str) -> Result<Option<T>, PersistenceError> {
        // Try each backend in order
        for backend in &self.fallback_backends {
            match backend.load(key).await {
                Ok(data) => {
                    if let Ok(value) = serde_json::from_slice::<T>(&data) {
                        if (self.validation)(&value) {
                            return Ok(Some(value));
                        }
                    }
                }
                Err(PersistenceError::NotFound { .. }) => continue,
                Err(e) => tracing::warn!("Recovery attempt failed: {}", e),
            }
        }
        
        Ok(None)
    }
}
```

This persistence design provides a robust, flexible system for maintaining state across sessions while handling the complexities of different storage backends and error conditions.
