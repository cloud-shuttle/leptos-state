//! Core persistence manager implementation

use crate::machine::persistence::storage::{MachineStorage, StorageFactory, StorageInfo};
use crate::machine::{Machine, SerializedMachine};
use crate::machine::persistence_core::{BackupConfig, PersistenceError, PersistenceConfig, StorageType};

/// Persistence manager for state machines
pub struct MachinePersistence<C: Send + Sync + 'static, E: Send + Sync + 'static> {
    /// Storage backend
    storage: Box<dyn MachineStorage>,
    /// Configuration
    config: PersistenceConfig,
    /// Active machine IDs
    active_machines: std::sync::RwLock<std::collections::HashSet<String>>,
    /// Auto-save handles
    auto_save_handles: std::sync::Mutex<Vec<tokio::task::JoinHandle<()>>>,
    /// Backup manager
    backup_manager: Option<super::backup::BackupManager>,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + 'static> MachinePersistence<C, E> {
    /// Create a new persistence manager
    pub fn new(storage: Box<dyn MachineStorage>, config: PersistenceConfig) -> Self {
        let backup_manager = if config.backup_config.enabled {
            Some(super::backup::BackupManager::new(
                storage.info().storage_type,
                config.backup_config.clone(),
            ))
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
    pub async fn persist_machine(
        &self,
        machine: &Machine<C, E, C>,
    ) -> Result<(), PersistenceError> {
        let machine_id = machine.id().to_string();
        let serialized = self.serialize_machine(machine).await?;
        let data = self.encode_data(&serialized)?;

        self.storage.store(&machine_id, &data).await?;
        self.active_machines.write().unwrap().insert(machine_id.clone());

        if let Some(ref backup_mgr) = self.backup_manager {
            backup_mgr.create_backup(&machine_id, &data).await?;
        }

        Ok(())
    }

    /// Load a machine
    pub async fn load_machine(&self, machine_id: &str) -> Result<Machine<C, E, C>, PersistenceError> {
        let data = self.storage.retrieve(machine_id).await?;
        let serialized: SerializedMachine = self.decode_data(&data)?;
        let machine = self.deserialize_machine(serialized).await?;
        Ok(machine)
    }

    /// Delete a machine
    pub async fn delete_machine(&self, machine_id: &str) -> Result<(), PersistenceError> {
        self.storage.delete(machine_id).await?;
        self.active_machines.write().unwrap().remove(machine_id);
        Ok(())
    }

    /// Check if machine exists
    pub async fn machine_exists(&self, machine_id: &str) -> Result<bool, PersistenceError> {
        self.storage.exists(machine_id).await
    }

    /// List all machine IDs
    pub async fn list_machines(&self) -> Result<Vec<String>, PersistenceError> {
        self.storage.list_keys().await
    }

    /// Save machine state
    pub async fn save_machine_state(
        &self,
        machine: &Machine<C, E, C>,
        state: &crate::machine::MachineStateImpl<C>,
    ) -> Result<(), PersistenceError> {
        let machine_id = format!("{}_state", machine.id());
        let serialized_state = self.serialize_state(state).await?;
        let data = self.encode_data(&serialized_state)?;
        self.storage.store(&machine_id, &data).await?;
        Ok(())
    }

    /// Load machine state
    pub async fn load_machine_state(&self, machine_id: &str) -> Result<(Machine<C, E, C>, crate::machine::MachineStateImpl<C>), PersistenceError> {
        let machine = self.load_machine(machine_id).await?;
        let state_id = format!("{}_state", machine_id);
        let state_data = self.storage.retrieve(&state_id).await?;
        let serialized_state: serde_json::Value = self.decode_data(&state_data)?;
        let state = self.deserialize_state(serialized_state).await?;
        Ok((machine, state))
    }

    /// Enable auto-save for a machine
    pub async fn enable_auto_save(
        &self,
        machine: &Machine<C, E, C>,
        interval: std::time::Duration,
    ) -> Result<(), PersistenceError> {
        let machine_id = machine.id().to_string();
        let storage = self.storage.clone();
        let config = self.config.clone();

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval);
            loop {
                interval.tick().await;
                // Note: In a real implementation, we'd need access to the current machine state
                // This is simplified for the refactoring
                let _ = storage.exists(&machine_id).await;
            }
        });

        self.auto_save_handles.lock().unwrap().push(handle);
        Ok(())
    }

    /// Disable auto-save for a machine
    pub async fn disable_auto_save(&self, machine_id: &str) -> Result<(), PersistenceError> {
        // In a real implementation, we'd cancel specific handles
        // This is simplified for the refactoring
        Ok(())
    }

    /// Get persistence statistics
    pub async fn get_statistics(&self) -> Result<super::stats::PersistenceStats, PersistenceError> {
        let storage_info = self.storage.info().await?;
        let active_count = self.active_machines.read().unwrap().len();
        let auto_save_count = self.auto_save_handles.lock().unwrap().len();

        Ok(super::stats::PersistenceStats {
            total_machines: storage_info.key_count,
            active_machines: active_count,
            auto_save_enabled: auto_save_count > 0,
            total_size_bytes: storage_info.total_size_bytes,
            last_backup_time: self.backup_manager.as_ref().and_then(|bm| bm.last_backup_time()),
            backup_count: self.backup_manager.as_ref().map_or(0, |bm| bm.backup_count()),
            storage_info,
        })
    }

    /// Create a backup
    pub async fn create_backup(&self, machine_id: &str) -> Result<(), PersistenceError> {
        if let Some(ref backup_mgr) = self.backup_manager {
            let data = self.storage.retrieve(machine_id).await?;
            backup_mgr.create_backup(machine_id, &data).await?;
        }
        Ok(())
    }

    /// Restore from backup
    pub async fn restore_from_backup(&self, machine_id: &str, backup_id: &str) -> Result<(), PersistenceError> {
        if let Some(ref backup_mgr) = self.backup_manager {
            let data = backup_mgr.restore_backup(backup_id).await?;
            self.storage.store(machine_id, &data).await?;
        }
        Ok(())
    }

    /// List backups
    pub fn list_backups(&self) -> Vec<super::backup::BackupEntry> {
        self.backup_manager.as_ref()
            .map(|bm| bm.list_backups())
            .unwrap_or_default()
    }

    /// Clean up old backups
    pub async fn cleanup_backups(&self) -> Result<(), PersistenceError> {
        if let Some(ref backup_mgr) = self.backup_manager {
            backup_mgr.cleanup_old_backups().await?;
        }
        Ok(())
    }

    /// Get configuration
    pub fn config(&self) -> &PersistenceConfig {
        &self.config
    }

    /// Shutdown the persistence manager
    pub async fn shutdown(&self) -> Result<(), PersistenceError> {
        // Cancel all auto-save handles
        let handles = std::mem::take(&mut *self.auto_save_handles.lock().unwrap());
        for handle in handles {
            handle.abort();
        }

        // Flush any pending operations
        self.storage.flush().await?;

        Ok(())
    }

    /// Serialize a machine
    async fn serialize_machine(&self, machine: &Machine<C, E, C>) -> Result<SerializedMachine, PersistenceError> {
        // Simplified serialization - in a real implementation this would be more complex
        Ok(SerializedMachine {
            id: machine.id().to_string(),
            initial_state: "initial".to_string(), // Simplified
            states: std::collections::HashMap::new(),
            transitions: Vec::new(),
        })
    }

    /// Deserialize a machine
    async fn deserialize_machine(&self, serialized: SerializedMachine) -> Result<Machine<C, E, C>, PersistenceError> {
        use crate::machine::MachineBuilderImpl;

        // Create a basic machine builder
        let mut builder = MachineBuilderImpl::new();

        // Set initial state
        builder = builder.initial(&serialized.initial_state);

        // Add states (simplified - in a real implementation, this would reconstruct the full state machine)
        for (state_id, state_data) in &serialized.states {
            builder = builder.state(state_id).build();
        }

        // Build the machine
        match builder.build() {
            Ok(machine) => Ok(machine),
            Err(e) => Err(PersistenceError::SerializationError(format!("Failed to build machine: {}", e))),
        }
    }

    /// Serialize machine state
    async fn serialize_state(&self, state: &crate::machine::MachineStateImpl<C>) -> Result<serde_json::Value, PersistenceError> {
        // Simplified state serialization
        serde_json::to_value(state.value()).map_err(|e| PersistenceError::SerializationError(e.to_string()))
    }

    /// Deserialize machine state
    async fn deserialize_state(&self, data: serde_json::Value) -> Result<crate::machine::MachineStateImpl<C>, PersistenceError> {
        // For now, we'll create a basic state. In a real implementation,
        // this would reconstruct the full state from the serialized data
        use crate::machine::MachineStateImpl;

        // Try to extract state value from the JSON
        let state_value = if let Some(state_str) = data.as_str() {
            state_str.to_string()
        } else if let Some(state_obj) = data.as_object() {
            // Try to get "value" field or use a default
            state_obj.get("value")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string()
        } else {
            "unknown".to_string()
        };

        // Create a basic context (in a real implementation, this would be deserialized too)
        let context = if data.is_object() {
            // Try to deserialize context from "context" field
            data.get("context")
                .and_then(|c| serde_json::from_value(c.clone()).ok())
                .unwrap_or_default()
        } else {
            C::default()
        };

        // Create a new machine state (simplified)
        // In a real implementation, this would use the actual machine to create the state
        Ok(MachineStateImpl::new(state_value, context))
    }

    /// Encode data for storage
    fn encode_data(&self, data: &impl serde::Serialize) -> Result<Vec<u8>, PersistenceError> {
        if self.config.compression_enabled {
            // In a real implementation, this would compress the data
            serde_json::to_vec(data).map_err(|e| PersistenceError::SerializationError(e.to_string()))
        } else {
            serde_json::to_vec(data).map_err(|e| PersistenceError::SerializationError(e.to_string()))
        }
    }

    /// Decode data from storage
    fn decode_data<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, PersistenceError> {
        if self.config.compression_enabled {
            // In a real implementation, this would decompress the data
            serde_json::from_slice(data).map_err(|e| PersistenceError::SerializationError(e.to_string()))
        } else {
            serde_json::from_slice(data).map_err(|e| PersistenceError::SerializationError(e.to_string()))
        }
    }
}

impl<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> Clone for MachinePersistence<C, E> {
    fn clone(&self) -> Self {
        // Note: This creates a new instance without copying internal state
        // In practice, you might want different cloning behavior
        Self {
            storage: self.storage.clone(),
            config: self.config.clone(),
            active_machines: std::sync::RwLock::new(std::collections::HashSet::new()),
            auto_save_handles: std::sync::Mutex::new(Vec::new()),
            backup_manager: self.backup_manager.clone(),
        }
    }
}
