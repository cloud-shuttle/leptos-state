//! Core persistence extension traits and basic functionality

use crate::machine::persistence::storage::StorageFactory;
use crate::machine::persistence_core::{PersistenceConfig, PersistenceError};
use crate::machine::Machine;

/// Extension trait for adding persistence to machines
pub trait MachinePersistenceExt<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> {
    /// Create a persistent machine
    fn with_persistence(
        self,
        config: PersistenceConfig,
    ) -> Result<crate::machine::persistence::ext::machine::PersistentMachine<C, E>, PersistenceError>;

    /// Enable persistence with default configuration
    fn persistent(self) -> Result<crate::machine::persistence::ext::machine::PersistentMachine<C, E>, PersistenceError>;
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> MachinePersistenceExt<C, E> for Machine<C, E, C> {
    fn with_persistence(
        self,
        config: PersistenceConfig,
    ) -> Result<crate::machine::persistence::ext::machine::PersistentMachine<C, E>, PersistenceError> {
        let storage = StorageFactory::new().create_storage(&config.storage_type, crate::machine::persistence::storage::StorageConfig::new())?;
        let persistence_manager = crate::machine::MachinePersistence::new(storage, config);
        Ok(crate::machine::persistence::ext::machine::PersistentMachine::new(self, persistence_manager))
    }

    fn persistent(self) -> Result<crate::machine::persistence::ext::machine::PersistentMachine<C, E>, PersistenceError> {
        let config = PersistenceConfig::default();
        self.with_persistence(config)
    }
}
