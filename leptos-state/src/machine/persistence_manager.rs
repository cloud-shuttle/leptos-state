//! Persistence manager implementation

use super::*;
use super::persistence_core::PersistenceError;

/// Persistence manager for state machines
pub struct MachinePersistence<C: Send + Sync, E> {
    /// Storage backend
    storage: Box<dyn MachineStorage>,
    /// Configuration
    config: PersistenceConfig,
    /// Active machine IDs
    active_machines: std::sync::RwLock<std::collections::HashSet<String>>,
    /// Auto-save handles
    auto_save_handles: std::sync::Mutex<Vec<tokio::task::JoinHandle<()>>>,
    /// Backup manager
    backup_manager: Option<BackupManager>,
}

impl<C: Send + Sync, E> MachinePersistence<C, E> {
    /// Create a new persistence manager
    pub fn new(storage: Box<dyn MachineStorage>, config: PersistenceConfig) -> Self {
        let backup_manager = if config.backup_config.enabled {
            Some(BackupManager::new(storage.info().storage_type, config.backup_config.clone()))
        } else {
            None
        };

        Self {
            storage,
            config,
            active_machines: std::sync::RwLock::new(std::collections::HashSet::new()),
            auto_save_handles: std::sync::Mutex::new(Vec::new()),
            backup_manager,
        }
    }

    /// Persist a machine
    pub async fn persist_machine(&self, machine: &Machine<C, E, C>) -> Result<(), PersistenceError> {
        let machine_id = machine.id().to_string();
        let serialized = self.serialize_machine(machine).await?;
        let data = self.encode_data(&serialized)?;

        self.storage.store(&machine_id, &data).await?;
        self.add_active_machine(&machine_id);

        // Create backup if configured
        if let Some(ref backup_mgr) = self.backup_manager {
            backup_mgr.create_backup(&machine_id, &data).await?;
        }

        Ok(())
    }

    /// Restore a machine
    pub async fn restore_machine(&self, machine_id: &str) -> Result<Machine<C, E, C>, PersistenceError> {
        let data: Vec<u8> = self.storage.retrieve(machine_id).await?;
        let decoded: Vec<u8> = self.decode_data(&data)?;
        let machine: Machine<C, E, C> = self.deserialize_machine(&decoded).await?;

        self.add_active_machine(machine_id);
        Ok(machine)
    }

    /// Delete a persisted machine
    pub async fn delete_machine(&self, machine_id: &str) -> Result<(), PersistenceError> {
        self.storage.delete(machine_id).await?;
        self.remove_active_machine(machine_id);
        Ok(())
    }

    /// List all persisted machines
    pub async fn list_machines(&self) -> Result<Vec<String>, PersistenceError> {
        self.storage.list_keys().await
    }

    /// Check if a machine exists
    pub async fn machine_exists(&self, machine_id: &str) -> Result<bool, PersistenceError> {
        self.storage.exists(machine_id).await
    }

    /// Get machine info
    pub async fn get_machine_info(&self, machine_id: &str) -> Result<MachineInfo, PersistenceError> {
        let data = self.storage.retrieve(machine_id).await?;
        let decoded = self.decode_data(&data)?;
        let size = data.len();

        // Parse metadata from serialized data
        let metadata = if let Ok(serialized) = serde_json::from_slice::<SerializedMachine<C, E, C>>(&decoded) {
            serialized.metadata
        } else {
            MachineMetadata::new()
        };

        Ok(MachineInfo {
            id: machine_id.to_string(),
            size,
            metadata,
            last_modified: std::time::SystemTime::now(), // Would need to track this properly
        })
    }

    /// Start auto-save for a machine
    pub fn start_auto_save(&self, machine: std::sync::Arc<Machine<C, E, C>>) {
        if !self.config.auto_save_enabled() {
            return;
        }

        let interval = self.config.auto_save_interval;
        let persistence = std::sync::Arc::new(self.clone());

        let handle = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(std::time::Duration::from_secs(interval));

            loop {
                interval_timer.tick().await;

                // Clone the machine for persistence
                let machine_clone = machine.clone();
                let persistence_clone = persistence.clone();

                tokio::spawn(async move {
                    if let Err(e) = persistence_clone.persist_machine(&machine_clone).await {
                        eprintln!("Auto-save failed: {:?}", e);
                    }
                });
            }
        });

        self.auto_save_handles.lock().unwrap().push(handle);
    }

    /// Stop auto-save
    pub async fn stop_auto_save(&self) {
        let mut handles = self.auto_save_handles.lock().unwrap();
        for handle in handles.drain(..) {
            handle.abort();
        }
    }

    /// Create a backup
    pub async fn create_backup(&self, machine_id: &str) -> Result<String, PersistenceError> {
        if let Some(ref backup_mgr) = self.backup_manager {
            let data = self.storage.retrieve(machine_id).await?;
            backup_mgr.create_backup(machine_id, &data).await
        } else {
            Err(PersistenceError::ConfigError("Backup not configured".to_string()))
        }
    }

    /// Restore from backup
    pub async fn restore_from_backup(&self, backup_id: &str) -> Result<Machine<C, E, C>, PersistenceError> {
        if let Some(ref backup_mgr) = self.backup_manager {
            let data: Vec<u8> = backup_mgr.restore_backup(backup_id).await?;
            let decoded: Vec<u8> = self.decode_data(&data)?;
            self.deserialize_machine(&decoded).await
        } else {
            Err(PersistenceError::ConfigError("Backup not configured".to_string()))
        }
    }

    /// List backups
    pub async fn list_backups(&self) -> Result<Vec<BackupEntry>, PersistenceError> {
        if let Some(ref backup_mgr) = self.backup_manager {
            backup_mgr.list_backups().await
        } else {
            Ok(Vec::new())
        }
    }

    /// Cleanup old data
    pub async fn cleanup(&self) -> Result<(), PersistenceError> {
        // Cleanup old backups
        if let Some(ref backup_mgr) = self.backup_manager {
            backup_mgr.cleanup_old_backups().await?;
        }

        // Additional cleanup could be implemented here
        Ok(())
    }

    /// Get statistics
    pub async fn get_stats(&self) -> Result<PersistenceStats, PersistenceError> {
        let keys = self.storage.list_keys().await?;
        let mut total_size = 0u64;
        let mut machine_count = 0;
        let mut oldest_machine: Option<MachineInfo> = None;
        let mut newest_machine: Option<MachineInfo> = None;

        for key in &keys {
            if let Ok(data) = self.storage.retrieve(key).await {
                total_size += data.len() as u64;
                machine_count += 1;

                // Parse timestamp from metadata (simplified)
                // In practice, you'd extract this from the serialized data
            }
        }

        Ok(PersistenceStats {
            machine_count,
            total_size,
            active_machines: self.active_machines.read().unwrap().len(),
            storage_info: self.storage.info(),
        })
    }

    /// Serialize a machine
    async fn serialize_machine(&self, machine: &Machine<C, E, C>) -> Result<SerializedMachine<C, E, C>, PersistenceError> {
        // This would implement the actual serialization logic
        // For now, return a placeholder
        Err(PersistenceError::SerializationError("Not implemented".to_string()))
    }

    /// Deserialize a machine
    async fn deserialize_machine(&self, data: &SerializedMachine<C, E, C>) -> Result<Machine<C, E, C>, PersistenceError> {
        // This would implement the actual deserialization logic
        // For now, return a placeholder
        Err(PersistenceError::DeserializationError("Not implemented".to_string()))
    }

    /// Encode data for storage
    fn encode_data(&self, data: &SerializedMachine<C, E, C>) -> Result<Vec<u8>, PersistenceError> {
        let json = serde_json::to_string(data)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;

        if self.config.compression_level > 0 {
            persistence_storage::utils::compress_data(json.as_bytes())
        } else {
            Ok(json.into_bytes())
        }
    }

    /// Decode data from storage
    fn decode_data(&self, data: &[u8]) -> Result<SerializedMachine<C, E, C>, PersistenceError> {
        let json_bytes = if self.config.compression_level > 0 {
            persistence_storage::utils::decompress_data(data)?
        } else {
            data.to_vec()
        };

        let json = String::from_utf8(json_bytes)
            .map_err(|e| PersistenceError::DeserializationError(e.to_string()))?;

        serde_json::from_str(&json)
            .map_err(|e| PersistenceError::DeserializationError(e.to_string()))
    }

    /// Add an active machine
    fn add_active_machine(&self, machine_id: &str) {
        self.active_machines.write().unwrap().insert(machine_id.to_string());
    }

    /// Remove an active machine
    fn remove_active_machine(&self, machine_id: &str) {
        self.active_machines.write().unwrap().remove(machine_id);
    }
}

impl<C: Send + Sync, E> Clone for MachinePersistence<C, E> {
    fn clone(&self) -> Self {
        Self {
            storage: persistence_storage::StorageFactory::create_storage(&StorageType::Memory).unwrap(),
            config: self.config.clone(),
            active_machines: std::sync::RwLock::new(std::collections::HashSet::new()),
            auto_save_handles: std::sync::Mutex::new(Vec::new()),
            backup_manager: None,
        }
    }
}

/// Machine information
#[derive(Debug, Clone)]
pub struct MachineInfo {
    /// Machine ID
    pub id: String,
    /// Size in bytes
    pub size: usize,
    /// Metadata
    pub metadata: MachineMetadata,
    /// Last modified timestamp
    pub last_modified: std::time::SystemTime,
}

/// Persistence statistics
#[derive(Debug, Clone)]
pub struct PersistenceStats {
    /// Number of machines
    pub machine_count: usize,
    /// Total size in bytes
    pub total_size: u64,
    /// Number of active machines
    pub active_machines: usize,
    /// Storage information
    pub storage_info: StorageInfo,
}

/// Backup manager
pub struct BackupManager {
    /// Storage type
    storage_type: String,
    /// Backup configuration
    config: BackupConfig,
    /// Backup storage
    backup_storage: Box<dyn MachineStorage>,
}

impl BackupManager {
    /// Create a new backup manager
    pub fn new(storage_type: String, config: BackupConfig) -> Self {
        // Use memory storage for backups by default
        let backup_storage = persistence_storage::StorageFactory::create_storage(&StorageType::Memory).unwrap();

        Self {
            storage_type,
            config,
            backup_storage,
        }
    }

    /// Create a backup
    pub async fn create_backup(&self, machine_id: &str, data: &[u8]) -> Result<String, PersistenceError> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let backup_id = format!("{}-{}-{}", machine_id, timestamp, self.config.naming_pattern);

        self.backup_storage.store(&backup_id, data).await?;

        Ok(backup_id)
    }

    /// Restore from backup
    pub async fn restore_backup(&self, backup_id: &str) -> Result<Vec<u8>, PersistenceError> {
        self.backup_storage.retrieve(backup_id).await
    }

    /// List backups
    pub async fn list_backups(&self) -> Result<Vec<BackupEntry>, PersistenceError> {
        let keys = self.backup_storage.list_keys().await?;
        let mut backups = Vec::new();

        for key in keys {
            // Parse backup information from key
            // This is a simplified implementation
            let entry = BackupEntry {
                id: key.clone(),
                machine_id: key.split('-').next().unwrap_or("").to_string(),
                timestamp: std::time::SystemTime::now(), // Would parse from key
                size: 0, // Would get from storage
                compressed: self.config.compress,
            };
            backups.push(entry);
        }

        Ok(backups)
    }

    /// Cleanup old backups
    pub async fn cleanup_old_backups(&self) -> Result<(), PersistenceError> {
        let backups = self.list_backups().await?;
        let mut to_delete = Vec::new();

        // Sort by timestamp (newest first)
        let mut sorted_backups = backups;
        sorted_backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Keep only the most recent backups
        if sorted_backups.len() > self.config.max_backups {
            let delete_count = sorted_backups.len() - self.config.max_backups;
            for backup in sorted_backups.iter().skip(self.config.max_backups) {
                to_delete.push(backup.id.clone());
            }
        }

        // Delete old backups
        for backup_id in to_delete {
            self.backup_storage.delete(&backup_id).await?;
        }

        Ok(())
    }
}

/// Backup entry information
#[derive(Debug, Clone)]
pub struct BackupEntry {
    /// Backup ID
    pub id: String,
    /// Machine ID
    pub machine_id: String,
    /// Creation timestamp
    pub timestamp: std::time::SystemTime,
    /// Size in bytes
    pub size: usize,
    /// Whether compressed
    pub compressed: bool,
}

impl BackupEntry {
    /// Get backup age
    pub fn age(&self) -> std::time::Duration {
        std::time::SystemTime::now()
            .duration_since(self.timestamp)
            .unwrap_or_default()
    }

    /// Check if backup is expired
    pub fn is_expired(&self, max_age: std::time::Duration) -> bool {
        self.age() > max_age
    }

    /// Get formatted timestamp
    pub fn formatted_timestamp(&self) -> String {
        // Simplified timestamp formatting
        format!("{:?}", self.timestamp)
    }
}

/// Persistence utilities
pub mod utils {
    use super::*;

    /// Create a persistence manager with default settings
    pub fn create_default_persistence<C: Send + Sync, E>() -> Result<MachinePersistence<C, E>, PersistenceError> {
        let storage = persistence_storage::StorageFactory::create_storage(&StorageType::Memory)?;
        let config = PersistenceConfig::default();
        Ok(MachinePersistence::new(storage, config))
    }

    /// Create a persistence manager with local storage
    pub fn create_local_persistence<C: Send + Sync, E>() -> Result<MachinePersistence<C, E>, PersistenceError> {
        let storage = persistence_storage::StorageFactory::create_storage(&StorageType::LocalStorage)?;
        let config = PersistenceConfig::default();
        Ok(MachinePersistence::new(storage, config))
    }

    /// Create a persistence manager with file system storage
    pub fn create_filesystem_persistence<C: Send + Sync, E>(base_dir: std::path::PathBuf) -> Result<MachinePersistence<C, E>, PersistenceError> {
        let storage = Box::new(persistence_storage::FileSystemStorage::new(base_dir));
        let config = PersistenceConfig::default();
        Ok(MachinePersistence::new(storage, config))
    }

    /// Validate persistence configuration
    pub fn validate_config(config: &PersistenceConfig) -> Result<(), PersistenceError> {
        if config.storage_type == StorageType::LocalStorage && !persistence_storage::LocalStorage::is_available() {
            return Err(PersistenceError::ConfigError("LocalStorage not available".to_string()));
        }

        if config.compression_level > 9 {
            return Err(PersistenceError::ConfigError("Compression level must be 0-9".to_string()));
        }

        Ok(())
    }

    /// Get recommended persistence configuration
    pub fn recommended_config() -> PersistenceConfig {
        PersistenceConfig {
            enabled: true,
            storage_type: StorageType::LocalStorage,
            auto_save_interval: 60, // 1 minute
            max_backups: 20,
            compression_level: 6,
            validate_on_load: true,
            custom_config: std::collections::HashMap::new(),
        }
    }

    /// Test persistence setup
    pub async fn test_persistence(persistence: &MachinePersistence<(), ()>) -> Result<(), PersistenceError> {
        persistence_storage::utils::test_storage(persistence.storage.as_ref()).await
    }
}
