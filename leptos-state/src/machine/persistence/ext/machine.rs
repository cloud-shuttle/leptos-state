//! Persistent machine implementation

use crate::machine::persistence_core::{PersistenceConfig, PersistenceError};
use crate::machine::{Machine, MachinePersistence};
use crate::StateResult;

/// A state machine with persistence capabilities
pub struct PersistentMachine<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> {
    /// The underlying machine
    machine: Machine<C, E, C>,
    /// Persistence manager
    persistence: MachinePersistence<C, E>,
    /// Auto-save enabled
    auto_save_enabled: bool,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> PersistentMachine<C, E> {
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
    pub async fn transition(&mut self, event: E) -> StateResult<crate::machine::MachineStateImpl<C>> {
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
    pub async fn save(&self) -> Result<(), PersistenceError> {
        self.persistence.save_machine_state(&self.machine, &self.current_state()).await
    }

    /// Load a saved state
    pub async fn load(&mut self) -> Result<(), PersistenceError> {
        if let Some((machine, state)) = self.persistence.load_machine_state().await? {
            self.machine = machine;
            // Note: The loaded state would need to be integrated with the current state
        }
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
    pub async fn statistics(&self) -> Result<super::monitoring::PersistenceStats, PersistenceError> {
        self.persistence.get_statistics().await
    }
}

impl<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> Clone for PersistentMachine<C, E> {
    fn clone(&self) -> Self {
        // Note: This creates a new persistence manager, so statistics and state will not be shared
        Self {
            machine: self.machine.clone(),
            persistence: self.persistence.clone(),
            auto_save_enabled: self.auto_save_enabled,
        }
    }
}
