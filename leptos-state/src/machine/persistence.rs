//! State Machine Persistence & Serialization
//!
//! This module provides comprehensive persistence capabilities for state machines,
//! including serialization, storage, and restoration of machine states and contexts.

use super::*;
use crate::machine::states::StateValue;
use crate::utils::types::{StateError, StateResult};
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Mutex;

#[cfg(feature = "serde_json")]
use serde_json;
#[cfg(feature = "serde_yaml")]


/// Trait for serializing state machine data
pub trait MachineSerialize {
    /// Serialize the machine state to a string
    fn serialize(&self) -> StateResult<String>;

    /// Get a version identifier for the serialized format
    fn version(&self) -> &str {
        "1.0"
    }
}

/// Trait for deserializing state machine data
pub trait MachineDeserialize<T> {
    /// Deserialize machine state from a string
    fn deserialize(data: &str) -> StateResult<T>;

    /// Get the expected version for deserialization
    fn expected_version(&self) -> &str {
        "1.0"
    }
}

/// Persistence configuration for state machines
#[derive(Debug, Clone)]
pub struct PersistenceConfig {
    /// Whether persistence is enabled
    pub enabled: bool,
    /// Storage key for the machine state
    pub storage_key: String,
    /// Whether to auto-save on every transition
    pub auto_save: bool,
    /// Whether to auto-restore on initialization
    pub auto_restore: bool,
    /// Maximum size of stored data in bytes
    pub max_size: usize,
    /// Compression level (0-9, 0 = no compression)
    pub compression_level: u8,
    /// Whether to encrypt stored data
    pub encrypt: bool,
    /// Encryption key (if encryption is enabled)
    pub encryption_key: Option<String>,
    /// Backup configuration
    pub backup_config: BackupConfig,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            storage_key: "leptos_state_machine".to_string(),
            auto_save: true,
            auto_restore: true,
            max_size: 1024 * 1024, // 1MB
            compression_level: 0,
            encrypt: false,
            encryption_key: None,
            backup_config: BackupConfig::default(),
        }
    }
}

/// Backup configuration for state machine persistence
#[derive(Debug, Clone)]
pub struct BackupConfig {
    /// Number of backups to keep
    pub max_backups: usize,
    /// Whether to create backups automatically
    pub auto_backup: bool,
    /// Backup interval in seconds
    pub backup_interval: u64,
    /// Whether to compress backups
    pub compress_backups: bool,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            max_backups: 5,
            auto_backup: true,
            backup_interval: 3600, // 1 hour
            compress_backups: true,
        }
    }
}

/// Serialized state machine data
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SerializedMachine<C, E> {
    /// Version of the serialization format
    pub version: String,
    /// Timestamp when the data was serialized
    pub timestamp: u64,
    /// Current state value
    pub state_value: StateValue,
    /// Context data
    pub context: C,
    /// Machine metadata
    pub metadata: MachineMetadata,
    /// Checksum for data integrity
    pub checksum: String,
    _phantom: std::marker::PhantomData<E>,
}

/// Machine metadata for persistence
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MachineMetadata {
    /// Machine ID
    pub machine_id: String,
    /// Number of transitions performed
    pub transition_count: usize,
    /// Last transition timestamp
    pub last_transition: u64,
    /// Machine creation timestamp
    pub created_at: u64,
    /// Version of the machine definition
    pub machine_version: String,
    /// Custom metadata
    pub custom_data: HashMap<String, String>,
}

impl MachineMetadata {
    pub fn new(machine_id: impl Into<String>) -> Self {
        Self {
            machine_id: machine_id.into(),
            transition_count: 0,
            last_transition: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            machine_version: "1.0".to_string(),
            custom_data: HashMap::new(),
        }
    }

    pub fn with_custom_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_data.insert(key.into(), value.into());
        self
    }
}

/// Persistence manager for state machines
pub struct MachinePersistence<C: Send + Sync, E> {
    config: PersistenceConfig,
    storage: Arc<dyn MachineStorage>,
    backups: Arc<Mutex<Vec<BackupEntry>>>,
    last_save: Arc<Mutex<Option<u64>>>,
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C: Send + Sync + 'static, E> MachinePersistence<C, E>
where
    C: Clone + Send + Sync + 'static,
    E: Clone,
{
    pub fn new(config: PersistenceConfig) -> Self {
        Self {
            storage: Arc::new(LocalStorage::new()),
            backups: Arc::new(Mutex::new(Vec::new())),
            last_save: Arc::new(Mutex::new(None)),
            config,
            _phantom: PhantomData::<(C, E)>,
        }
    }

    pub fn with_storage(mut self, storage: Arc<dyn MachineStorage>) -> Self {
        self.storage = storage;
        self
    }

    /// Save the current machine state
    pub fn save(&self, machine: &Machine<C, E>, state: &MachineStateImpl<C>) -> StateResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let _serialized = self.serialize_machine(machine, state)?;

        #[cfg(feature = "serde_json")]
        {
            let data = serde_json::to_string(&serialized)?;

            // Check size limit
            if data.len() > self.config.max_size {
                return Err(StateError::new("Serialized data exceeds maximum size"));
            }

            // Compress if enabled
            let final_data = if self.config.compression_level > 0 {
                self.compress_data(&data)?
            } else {
                data
            };

            // Encrypt if enabled
            let final_data = if self.config.encrypt {
                self.encrypt_data(&final_data)?
            } else {
                final_data
            };

            // Save to storage
            self.storage.save(&self.config.storage_key, &final_data)?;

            // Update last save time
            if let Ok(mut last_save) = self.last_save.lock() {
                *last_save = Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                );
            }

            // Create backup if needed
            if self.config.backup_config.auto_backup {
                self.create_backup(&final_data)?;
            }
        }

        #[cfg(not(feature = "serde_json"))]
        {
            return Err(StateError::new("Serialization requires serde_json feature"));
        }
    }

    /// Load and restore machine state
    pub fn load(&self, _machine: &Machine<C, E>) -> StateResult<MachineStateImpl<C>> {
        if !self.config.enabled || !self.config.auto_restore {
            return Err(StateError::new(
                "Persistence not enabled or auto-restore disabled",
            ));
        }

        // Load from storage
        let _data = self.storage.load(&self.config.storage_key)?;

        // Decrypt if needed
        let _data = if self.config.encrypt {
            self.decrypt_data(&_data)?
        } else {
            _data
        };

        // Decompress if needed
        let _data = if self.config.compression_level > 0 {
            self.decompress_data(&_data)?
        } else {
            _data
        };

        // Deserialize
        #[cfg(feature = "serde_json")]
        {
            let serialized: SerializedMachine<C, E> = serde_json::from_str(&_data)?;

            // Validate checksum
            self.validate_checksum(&serialized)?;

            // Create machine state
            let state = MachineStateImpl {
                value: serialized.state_value,
                context: serialized.context,
            };

            Ok(state)
        }

        #[cfg(not(feature = "serde_json"))]
        Err(StateError::new(
            "Deserialization requires serde_json feature",
        ))
    }

    /// Create a backup of the current state
    pub fn create_backup(&self, data: &str) -> StateResult<()> {
        let backup_key = format!(
            "{}_backup_{}",
            self.config.storage_key,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        let backup_data = if self.config.backup_config.compress_backups {
            self.compress_data(data)?
        } else {
            data.to_string()
        };

        self.storage.save(&backup_key, &backup_data)?;

        // Update backup list
        if let Ok(mut backups) = self.backups.lock() {
            backups.push(BackupEntry {
                key: backup_key,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });

            // Remove old backups if we exceed the limit
            if backups.len() > self.config.backup_config.max_backups {
                if let Some(oldest) = backups.iter().min_by_key(|b| b.timestamp) {
                    let oldest_key = oldest.key.clone();
                    let _ = self.storage.delete(&oldest_key);
                    backups.retain(|b| b.key != oldest_key);
                }
            }
        }

        Ok(())
    }

    /// Restore from a specific backup
    pub fn restore_from_backup(&self, backup_timestamp: u64) -> StateResult<String> {
        let backup_key = format!("{}_backup_{}", self.config.storage_key, backup_timestamp);
        let data = self.storage.load(&backup_key)?;

        if self.config.backup_config.compress_backups {
            self.decompress_data(&data)
        } else {
            Ok(data)
        }
    }

    /// List available backups
    pub fn list_backups(&self) -> Vec<BackupEntry> {
        self.backups.lock().unwrap().clone()
    }

    /// Clear all persisted data
    pub fn clear(&self) -> StateResult<()> {
        self.storage.delete(&self.config.storage_key)?;

        // Clear backups
        if let Ok(backups) = self.backups.lock() {
            for backup in backups.iter() {
                let _ = self.storage.delete(&backup.key);
            }
        }

        if let Ok(mut backups) = self.backups.lock() {
            backups.clear();
        }

        Ok(())
    }

    /// Check if auto-save should be triggered
    pub fn should_auto_save(&self) -> bool {
        if !self.config.enabled || !self.config.auto_save {
            return false;
        }

        if let Ok(last_save) = self.last_save.lock() {
            if let Some(last) = *last_save {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                return now - last >= self.config.backup_config.backup_interval;
            }
        }

        true
    }

    /// Serialize machine state
    fn serialize_machine(
        &self,
        _machine: &Machine<C, E>,
        state: &MachineStateImpl<C>,
    ) -> StateResult<SerializedMachine<C, E>> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let metadata = MachineMetadata::new("machine")
            .with_custom_data("machine_type", "state_machine")
            .with_custom_data("context_type", std::any::type_name::<C>())
            .with_custom_data("event_type", std::any::type_name::<E>());

        #[allow(unused_variables)]
        let serialized = SerializedMachine {
            version: "1.0".to_string(),
            timestamp,
            state_value: state.value().clone(),
            context: state.context().clone(),
            metadata,
            checksum: String::new(), // Will be calculated below
            _phantom: PhantomData::<E>,
        };

        // Calculate checksum
        #[cfg(feature = "serde_json")]
        {
            let data = serde_json::to_string(&serialized)?;
            let checksum = self.calculate_checksum(&data);

            Ok(SerializedMachine {
                checksum,
                _phantom: PhantomData::<E>,
                ..serialized
            })
        }

        #[cfg(not(feature = "serde_json"))]
        {
            Err(StateError::new(
                "Checksum calculation requires serde_json feature",
            ))
        }
    }

    /// Calculate checksum for data integrity
    #[allow(dead_code)]
    fn calculate_checksum(&self, data: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Validate checksum
    #[allow(dead_code)]
    fn validate_checksum(&self, serialized: &SerializedMachine<C, E>) -> StateResult<()> {
        let mut temp_serialized = serialized.clone();
        temp_serialized.checksum = String::new();

        #[cfg(feature = "serde_json")]
        {
            let data = serde_json::to_string(&temp_serialized)?;
            let expected_checksum = self.calculate_checksum(&data);

            if serialized.checksum != expected_checksum {
                return Err(StateError::new("Checksum validation failed"));
            }

            Ok(())
        }

        #[cfg(not(feature = "serde_json"))]
        Err(StateError::new(
            "Checksum validation requires serde_json feature",
        ))
    }

    /// Compress data
    fn compress_data(&self, data: &str) -> StateResult<String> {
        // Simple compression - in a real implementation, you'd use a proper compression library
        Ok(data.to_string())
    }

    /// Decompress data
    fn decompress_data(&self, data: &str) -> StateResult<String> {
        // Simple decompression - in a real implementation, you'd use a proper compression library
        Ok(data.to_string())
    }

    /// Encrypt data
    #[allow(dead_code)]
    fn encrypt_data(&self, data: &str) -> StateResult<String> {
        // Simple encryption - in a real implementation, you'd use a proper encryption library
        Ok(data.to_string())
    }

    /// Decrypt data
    fn decrypt_data(&self, data: &str) -> StateResult<String> {
        // Simple decryption - in a real implementation, you'd use a proper decryption library
        Ok(data.to_string())
    }
}

/// Backup entry information
#[derive(Debug, Clone)]
pub struct BackupEntry {
    pub key: String,
    pub timestamp: u64,
}

/// Trait for machine storage backends
pub trait MachineStorage: Send + Sync {
    /// Save data with the given key
    fn save(&self, key: &str, data: &str) -> StateResult<()>;

    /// Load data with the given key
    fn load(&self, key: &str) -> StateResult<String>;

    /// Delete data with the given key
    fn delete(&self, key: &str) -> StateResult<()>;

    /// Check if data exists for the given key
    fn exists(&self, key: &str) -> bool;
}

/// Local storage implementation using web storage
pub struct LocalStorage;

impl LocalStorage {
    pub fn new() -> Self {
        Self
    }
}

impl MachineStorage for LocalStorage {
    fn save(&self, _key: &str, _data: &str) -> StateResult<()> {
        // TODO: Implement localStorage when web-sys features are properly configured
        tracing::warn!("LocalStorage not yet implemented - web-sys features need configuration");
        Ok(())
    }

    fn load(&self, _key: &str) -> StateResult<String> {
        // TODO: Implement localStorage when web-sys features are properly configured
        Err(StateError::new("LocalStorage not yet implemented"))
    }

    fn delete(&self, _key: &str) -> StateResult<()> {
        // TODO: Implement localStorage when web-sys features are properly configured
        Ok(())
    }

    fn exists(&self, _key: &str) -> bool {
        // TODO: Implement localStorage when web-sys features are properly configured
        false
    }
}

/// Memory storage implementation for testing
pub struct MemoryStorage {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl MachineStorage for MemoryStorage {
    fn save(&self, key: &str, data: &str) -> StateResult<()> {
        if let Ok(mut storage) = self.data.lock() {
            storage.insert(key.to_string(), data.to_string());
            Ok(())
        } else {
            Err(StateError::new("Failed to acquire storage lock"))
        }
    }

    fn load(&self, key: &str) -> StateResult<String> {
        if let Ok(storage) = self.data.lock() {
            storage
                .get(key)
                .cloned()
                .ok_or_else(|| StateError::new("Data not found"))
        } else {
            Err(StateError::new("Failed to acquire storage lock"))
        }
    }

    fn delete(&self, key: &str) -> StateResult<()> {
        if let Ok(mut storage) = self.data.lock() {
            storage.remove(key);
            Ok(())
        } else {
            Err(StateError::new("Failed to acquire storage lock"))
        }
    }

    fn exists(&self, key: &str) -> bool {
        if let Ok(storage) = self.data.lock() {
            storage.contains_key(key)
        } else {
            false
        }
    }
}

/// Extension trait for adding persistence to machines
pub trait MachinePersistenceExt<C: Send + Sync, E> {
    /// Add persistence to the machine
    fn with_persistence(self, config: PersistenceConfig) -> PersistentMachine<C, E>;
}

impl<C: Send + Sync, E> MachinePersistenceExt<C, E> for Machine<C, E>
where
    C: Clone + std::default::Default + 'static + std::fmt::Debug + Send + Sync,
    E: Clone + std::cmp::PartialEq + 'static + std::fmt::Debug,
{
    fn with_persistence(self, config: PersistenceConfig) -> PersistentMachine<C, E> {
        PersistentMachine::new(self, config)
    }
}

/// A state machine with persistence capabilities
pub struct PersistentMachine<C: Send + Sync, E> {
    machine: Machine<C, E>,
    persistence: MachinePersistence<C, E>,
    current_state: Option<MachineStateImpl<C>>,
}

impl<C: Send + Sync, E> PersistentMachine<C, E>
where
    C: Clone + std::default::Default + 'static + std::fmt::Debug + Send + Sync,
    E: Clone + std::cmp::PartialEq + 'static + std::fmt::Debug,
{
    pub fn new(machine: Machine<C, E>, config: PersistenceConfig) -> Self {
        let persistence = MachinePersistence::new(config);
        let current_state = None;

        Self {
            machine,
            persistence,
            current_state,
        }
    }

    /// Initialize the machine, optionally restoring from persistence
    pub fn initialize(mut self) -> StateResult<Self> {
        if self.persistence.config.auto_restore {
            match self.persistence.load(&self.machine) {
                Ok(state) => {
                    self.current_state = Some(state);
                    tracing::info!("Machine state restored from persistence");
                }
                Err(e) => {
                    tracing::warn!("Failed to restore machine state: {:?}", e);
                    self.current_state = Some(self.machine.initial_state());
                }
            }
        } else {
            self.current_state = Some(self.machine.initial_state());
        }

        Ok(self)
    }

    /// Get the current state
    pub fn current_state(&self) -> Option<&MachineStateImpl<C>> {
        self.current_state.as_ref()
    }

    /// Transition the machine and persist if enabled
    pub fn transition(&mut self, event: E) -> StateResult<MachineStateImpl<C>> {
        let current = self
            .current_state
            .as_ref()
            .ok_or_else(|| StateError::new("Machine not initialized"))?;

        let new_state = Machine::transition(&self.machine, current, event);

        // Auto-save if enabled
        if self.persistence.should_auto_save() {
            if let Err(e) = self.persistence.save(&self.machine, &new_state) {
                tracing::warn!("Failed to auto-save machine state: {:?}", e);
            }
        }

        self.current_state = Some(new_state.clone());
        Ok(new_state)
    }

    /// Manually save the current state
    pub fn save(&self) -> StateResult<()> {
        let current = self
            .current_state
            .as_ref()
            .ok_or_else(|| StateError::new("Machine not initialized"))?;

        self.persistence.save(&self.machine, current)
    }

    /// Manually restore from persistence
    pub fn restore(&mut self) -> StateResult<()> {
        let state = self.persistence.load(&self.machine)?;
        self.current_state = Some(state);
        Ok(())
    }

    /// Clear all persisted data
    pub fn clear_persistence(&self) -> StateResult<()> {
        self.persistence.clear()
    }

    /// Get persistence information
    pub fn persistence_info(&self) -> PersistenceInfo {
        PersistenceInfo {
            enabled: self.persistence.config.enabled,
            auto_save: self.persistence.config.auto_save,
            auto_restore: self.persistence.config.auto_restore,
            storage_key: self.persistence.config.storage_key.clone(),
            backups: self.persistence.list_backups(),
        }
    }
}

/// Information about machine persistence
#[derive(Debug, Clone)]
pub struct PersistenceInfo {
    pub enabled: bool,
    pub auto_save: bool,
    pub auto_restore: bool,
    pub storage_key: String,
    pub backups: Vec<BackupEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::*;

    #[derive(Debug, Clone, PartialEq, Default)]
    #[cfg_attr(feature = "persist", derive(serde::Serialize, serde::Deserialize))]
    struct TestContext {
        count: i32,
        name: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "persist", derive(serde::Serialize, serde::Deserialize))]
    enum TestEvent {
        Increment,
        Decrement,
        SetName(String),
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &str {
            match self {
                TestEvent::Increment => "increment",
                TestEvent::Decrement => "decrement",
                TestEvent::SetName(_) => "set_name",
            }
        }
    }

    #[test]
    fn test_persistence_config_default() {
        let config = PersistenceConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.storage_key, "leptos_state_machine");
        assert!(config.auto_save);
        assert!(config.auto_restore);
    }

    #[test]
    fn test_memory_storage() {
        let storage = MemoryStorage::new();

        // Test save and load
        storage.save("test_key", "test_data").unwrap();
        assert!(storage.exists("test_key"));

        let data = storage.load("test_key").unwrap();
        assert_eq!(data, "test_data");

        // Test delete
        storage.delete("test_key").unwrap();
        assert!(!storage.exists("test_key"));
    }

    #[test]
    fn test_persistent_machine() {
        #[cfg(feature = "serde_json")]
        {
            let machine = MachineBuilder::<TestContext, TestEvent>::new()
                .initial("idle")
                .state("idle")
                .on(TestEvent::Increment, "counting")
                .on(TestEvent::SetName("test".to_string()), "idle")
                .state("counting")
                .on(TestEvent::Decrement, "idle")
                .build();

            let config = PersistenceConfig {
                enabled: true,
                storage_key: "test_machine".to_string(),
                auto_save: false,
                auto_restore: false,
                ..Default::default()
            };

            let mut persistent_machine = machine
                .with_persistence(config.clone())
                .initialize()
                .unwrap();

            // Test initial state
            let initial_state = persistent_machine.current_state().unwrap();
            assert_eq!(
                *initial_state.value(),
                StateValue::Simple("idle".to_string())
            );

            // Test transition
            let new_state = persistent_machine.transition(TestEvent::Increment).unwrap();
            assert_eq!(
                *new_state.value(),
                StateValue::Simple("counting".to_string())
            );

            // Test manual save and restore
            persistent_machine.save().unwrap();

            // Create a new machine and restore
            let new_machine = MachineBuilder::<TestContext, TestEvent>::new()
                .initial("idle")
                .state("idle")
                .on(TestEvent::Increment, "counting")
                .on(TestEvent::SetName("test".to_string()), "idle")
                .state("counting")
                .on(TestEvent::Decrement, "idle")
                .build();

            let mut new_persistent_machine =
                new_machine.with_persistence(config).initialize().unwrap();

            new_persistent_machine.restore().unwrap();
            let restored_state = new_persistent_machine.current_state().unwrap();
            assert_eq!(
                *restored_state.value(),
                StateValue::Simple("counting".to_string())
            );
        }

        #[cfg(not(feature = "serde_json"))]
        {
            // Skip test when serde_json feature is not enabled
            println!("Skipping persistent machine test - serde_json feature not enabled");
        }
    }

    #[test]
    fn test_serialization() {
        #[cfg(feature = "serde_json")]
        {
            let context = TestContext {
                count: 42,
                name: "test".to_string(),
            };

            let state = MachineStateImpl::new(StateValue::Simple("idle".to_string()), context);

            let machine = MachineBuilder::<TestContext, TestEvent>::new()
                .initial("idle")
                .state("idle")
                .build();

            let persistence = MachinePersistence::new(PersistenceConfig {
                enabled: true,
                ..Default::default()
            });

            // Test serialization
            let serialized = persistence.serialize_machine(&machine, &state).unwrap();
            assert_eq!(serialized.version, "1.0");
            assert_eq!(serialized.context.count, 42);
            assert_eq!(serialized.context.name, "test");
            assert!(!serialized.checksum.is_empty());
        }

        #[cfg(not(feature = "serde_json"))]
        {
            // Skip test when serde_json feature is not enabled
            println!("Skipping serialization test - serde_json feature not enabled");
        }
    }
}
