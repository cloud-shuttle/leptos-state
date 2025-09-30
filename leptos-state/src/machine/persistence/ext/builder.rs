//! Fluent API for creating persistent machines

use crate::machine::persistence::storage::{StorageConfig, StorageFactory};
use crate::machine::persistence_core::{PersistenceConfig, PersistenceError, StorageType};
use crate::machine::{Machine, MachinePersistence};

/// Fluent API for creating persistent machines
pub struct PersistenceBuilder<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> {
    machine: Option<Machine<C, E, C>>,
    config: PersistenceConfig,
    custom_config: StorageConfig,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> PersistenceBuilder<C, E> {
    /// Create a new persistence builder
    pub fn new() -> Self {
        Self {
            machine: None,
            config: PersistenceConfig::default(),
            custom_config: StorageConfig::new(),
        }
    }

    /// Set the machine to make persistent
    pub fn machine(mut self, machine: Machine<C, E, C>) -> Self {
        self.machine = Some(machine);
        self
    }

    /// Set storage type
    pub fn storage_type(mut self, storage_type: StorageType) -> Self {
        self.config.storage_type = storage_type;
        self
    }

    /// Use memory storage
    pub fn memory(mut self) -> Self {
        self.config.storage_type = StorageType::Memory;
        self
    }

    /// Use local storage
    pub fn local(mut self) -> Self {
        self.config.storage_type = StorageType::LocalStorage;
        self
    }

    /// Use filesystem storage with path
    pub fn filesystem<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.config.storage_type = StorageType::FileSystem(path.into());
        self
    }

    /// Set storage key
    pub fn key(mut self, key: String) -> Self {
        self.config.storage_key = key;
        self
    }

    /// Enable auto-save
    pub fn auto_save(mut self, enabled: bool) -> Self {
        self.config.auto_save = enabled;
        self
    }

    /// Enable auto-restore
    pub fn auto_restore(mut self, enabled: bool) -> Self {
        self.config.auto_restore = enabled;
        self
    }

    /// Set compression level
    pub fn compression(mut self, level: u32) -> Self {
        self.config.compression_level = level;
        self
    }

    /// Enable encryption
    pub fn encryption(mut self, enabled: bool) -> Self {
        self.config.encryption_enabled = enabled;
        self
    }

    /// Set custom storage capacity
    pub fn capacity(mut self, capacity: u64) -> Self {
        self.custom_config = self.custom_config.with_capacity(capacity);
        self
    }

    /// Set custom storage configuration
    pub fn custom_config(mut self, config: StorageConfig) -> Self {
        self.custom_config = config;
        self
    }

    /// Build the persistent machine
    pub fn build(self) -> Result<crate::machine::persistence::ext::machine::PersistentMachine<C, E>, PersistenceError> {
        let machine = self.machine.ok_or_else(|| PersistenceError::ConfigError("No machine provided".to_string()))?;

        let storage = StorageFactory::new().create_storage(&self.config.storage_type, self.custom_config)?;
        let persistence = MachinePersistence::new(storage, self.config);

        Ok(crate::machine::persistence::ext::machine::PersistentMachine::new(machine, persistence))
    }

    /// Build with auto-save enabled
    pub fn build_with_auto_save(self) -> Result<crate::machine::persistence::ext::machine::PersistentMachine<C, E>, PersistenceError> {
        self.build().map(|pm| pm.with_auto_save())
    }
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> Default for PersistenceBuilder<C, E> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for creating persistent machines
pub mod factories {
    use super::*;

    /// Create a memory-based persistent machine
    pub fn memory<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(
        machine: crate::machine::Machine<C, E, C>
    ) -> PersistenceBuilder<C, E> {
        PersistenceBuilder::new()
            .machine(machine)
            .memory()
    }

    /// Create a local storage persistent machine
    pub fn local<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(
        machine: Machine<C, E, C>
    ) -> Result<crate::machine::persistence::ext::machine::PersistentMachine<C, E>, PersistenceError> {
        PersistenceBuilder::new()
            .machine(machine)
            .local()
            .build()
    }

    /// Create a filesystem-based persistent machine
    pub fn filesystem<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static, P: Into<std::path::PathBuf>>(
        machine: Machine<C, E, C>,
        path: P
    ) -> PersistenceBuilder<C, E> {
        PersistenceBuilder::new()
            .machine(machine)
            .filesystem(path)
    }

    /// Create a persistent machine with auto-save enabled
    pub fn with_auto_save<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(
        builder: PersistenceBuilder<C, E>
    ) -> PersistenceBuilder<C, E> {
        builder.auto_save(true)
    }
}
