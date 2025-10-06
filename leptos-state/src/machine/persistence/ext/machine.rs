//! Persistent machine implementation

use crate::machine::persistence_core::{PersistenceConfig, PersistenceError};
use crate::machine::{Machine, MachinePersistence};
use crate::StateResult;

/// A state machine with persistence capabilities
pub struct PersistentMachine<C: crate::machine::core::traits::CloneableStateMachineType, E: crate::machine::core::traits::CloneableStateMachineType + PartialEq> {
    /// The underlying machine
    machine: Machine<C, E, C>,
    /// Persistence manager
    persistence: MachinePersistence<C, E>,
    /// Auto-save enabled
    auto_save_enabled: bool,
}

impl<C: crate::machine::core::traits::CloneableStateMachineType, E: crate::machine::core::traits::CloneableStateMachineType + PartialEq> PersistentMachine<C, E> {
    /// Create a new persistent machine
    pub fn new(machine: Machine<C, E, C>, persistence: MachinePersistence<C, E>) -> Self {
        Self {
            machine,
            persistence,
            auto_save_enabled: false,
        }
    }

    /// Enable auto-save
    pub fn with_auto_save(mut self) -> Self {
        self.auto_save_enabled = true;
        self
    }

    /// Disable auto-save
    pub fn without_auto_save(mut self) -> Self {
        self.auto_save_enabled = false;
        self
    }

    /// Get the current state
    pub fn current_state(&self) -> crate::machine::MachineStateImpl<C> {
        self.machine.current_state()
    }

    /// Transition to a new state
    pub async fn transition(&mut self, event: E) -> StateResult<crate::machine::MachineStateImpl<C>>
    where
        C: Default + serde::Serialize + 'static,
        E: Eq + std::hash::Hash + serde::Serialize + 'static,
    {
        let result = self.machine.transition(&self.current_state(), event);

        if self.auto_save_enabled {
            if let Err(e) = self.persistence.save_machine_state(&self.machine, &result).await {
                // Log error but don't fail the transition
                eprintln!("Failed to save machine state: {}", e);
            }
        }

        Ok(result)
    }

    /// Save the current state manually
    pub async fn save(&self) -> Result<(), PersistenceError>
    where
        E: Eq + std::hash::Hash,
        C: serde::Serialize + 'static,
        E: serde::Serialize + 'static,
    {
        self.persistence.save_machine_state(&self.machine, &self.current_state()).await
    }

    /// Load a saved state
    pub async fn load(&mut self) -> Result<(), PersistenceError>
    where
        E: Eq + std::hash::Hash,
        C: for<'de> serde::Deserialize<'de> + Default + Clone + 'static,
        E: for<'de> serde::Deserialize<'de> + Clone + 'static,
    {
        let (machine, state) = self.persistence.load_machine_state(&self.machine.id()).await?;
        self.machine = machine;
        // Note: The loaded state would need to be integrated with the current state
        Ok(())
    }

    /// Get persistence information
    pub fn persistence_info(&self) -> super::info::PersistenceInfo {
        super::info::PersistenceInfo {
            storage_type: self.persistence.config().storage_type.clone(),
            auto_save_enabled: self.auto_save_enabled,
            last_save_time: None, // Would need to be tracked
            save_count: 0, // Would need to be tracked
        }
    }

    /// Get the underlying machine (read-only)
    pub fn machine(&self) -> &Machine<C, E, C> {
        &self.machine
    }

    /// Get the persistence manager (read-only)
    pub fn persistence(&self) -> &MachinePersistence<C, E> {
        &self.persistence
    }

    /// Check if auto-save is enabled
    pub fn is_auto_save_enabled(&self) -> bool {
        self.auto_save_enabled
    }

    /// Get machine statistics
    pub async fn statistics(&self) -> Result<super::super::manager::stats::PersistenceStats, PersistenceError> {
        self.persistence.get_statistics().await
    }
}

impl<C: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> Clone for PersistentMachine<C, E> {
    fn clone(&self) -> Self {
        // Note: This creates a new persistence manager, so statistics and state will not be shared
        Self {
            machine: self.machine.clone(),
            persistence: self.persistence.clone(),
            auto_save_enabled: self.auto_save_enabled,
        }
    }
}
