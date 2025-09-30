//! History machine implementation

use super::*;
use std::hash::Hash;

/// Machine with history tracking capabilities
pub struct HistoryMachine<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
> {
    /// The underlying base machine
    pub base_machine: Machine<C, E, C>,
    /// History tracker
    pub history_tracker: HistoryTracker<C>,
    /// History configuration
    pub config: HistoryConfig,
    /// History states mapping (state_id -> history_state)
    pub history_states: std::collections::HashMap<String, HistoryState>,
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static, E: Clone + Send + Sync + Hash + Eq + std::fmt::Debug + 'static>
    HistoryMachine<C, E>
{
    /// Create a new history machine
    pub fn new(base_machine: Machine<C, E, C>, config: HistoryConfig) -> Self {
        Self {
            history_tracker: HistoryTracker::new(),
            config,
            history_states: std::collections::HashMap::new(),
            base_machine,
        }
    }

    /// Add a history state
    pub fn add_history_state(&mut self, state: HistoryState) {
        self.history_states.insert(state.id.clone(), state);
    }

    /// Remove a history state
    pub fn remove_history_state(&mut self, state_id: &str) {
        self.history_states.remove(state_id);
    }

    /// Get a history state by ID
    pub fn get_history_state(&self, state_id: &str) -> Option<&HistoryState> {
        self.history_states.get(state_id)
    }

    /// Check if a state is a history state
    pub fn is_history_state(&self, state_id: &str) -> bool {
        self.history_states.contains_key(state_id)
    }

    /// Get the current state
    pub fn current_state(&self) -> &MachineStateImpl<C> {
        self.base_machine.current_state()
    }

    /// Get the current state name
    pub fn current_state_name(&self) -> &str {
        self.base_machine.current_state_name()
    }

    /// Get the initial state
    pub fn initial_state(&self) -> MachineStateImpl<C> {
        let state_name = self.base_machine.initial_state();
        let context = self.base_machine.get_context().clone();
        MachineStateImpl::new(state_name, context)
    }

    /// Get available states
    pub fn get_states(&self) -> Vec<String> {
        self.base_machine.get_states()
    }

    /// Check if a transition is possible
    pub fn can_transition(&self, target: &str) -> bool {
        self.base_machine.can_transition(target)
    }

    /// Transition to a new state with history tracking
    pub fn transition(&mut self, event: E) -> Result<(), MachineError> {
        if !self.config.enabled {
            return self.base_machine.transition(event);
        }

        let current_state_name = self.current_state_name().to_string();

        // Record the current state before transition
        if let Some(history_state) = self.history_states.get(&current_state_name) {
            if history_state.enabled {
                let context = if history_state.restore_context {
                    Some(self.base_machine.get_context().clone())
                } else {
                    None
                };

                self.history_tracker.record_state(
                    &current_state_name,
                    context,
                    Some(format!("{:?}", event)),
                );
            }
        }

        // Perform the transition
        let result = self.base_machine.transition(event);

        // If transition succeeded and we're entering a history state, restore history
        if result.is_ok() {
            let new_state_name = self.current_state_name();
            if let Some(history_state) = self.history_states.get(new_state_name) {
                if history_state.enabled {
                    self.restore_history(history_state);
                }
            }
        }

        result
    }

    /// Restore history for a history state
    fn restore_history(&mut self, history_state: &HistoryState) {
        if let Some(last_state) = self.history_tracker.get_last_state(&history_state.id) {
            // For now, we just log the restoration
            // In a full implementation, this would transition to the historical state
            eprintln!(
                "Restoring history for state {}: {}",
                history_state.id, last_state.state
            );
        } else if let Some(default_target) = &history_state.default_target {
            // Transition to default target if no history exists
            eprintln!(
                "No history found for state {}, using default: {}",
                history_state.id, default_target
            );
        }
    }

    /// Clear history for a specific state
    pub fn clear_history(&mut self, state_id: &str) {
        self.history_tracker.clear_history(state_id);
    }

    /// Clear all history
    pub fn clear_all_history(&mut self) {
        self.history_tracker.clear_all_history();
    }

    /// Get history for a specific state
    pub fn get_history(&self, state_id: &str) -> Vec<&HistoryEntry<C>> {
        self.history_tracker.get_history(state_id)
    }

    /// Get all history
    pub fn get_all_history(&self) -> &std::collections::HashMap<String, Vec<HistoryEntry<C>>> {
        self.history_tracker.get_all_history()
    }

    /// Get history statistics
    pub fn get_history_stats(&self) -> HistoryStats {
        self.history_tracker.get_stats()
    }

    /// Cleanup old history entries
    pub fn cleanup_history(&mut self, max_age: std::time::Duration) {
        self.history_tracker.cleanup(max_age);
    }

    /// Persist history to storage
    pub fn persist_history(&self) -> Result<(), String> {
        if !self.config.should_persist() {
            return Ok(());
        }

        let key = self.config.persistence_key_required();
        let history_data = serde_json::to_string(&self.history_tracker.get_all_history())
            .map_err(|e| format!("Failed to serialize history: {}", e))?;

        // In a real implementation, this would save to localStorage or other persistence
        eprintln!("Persisting history to key: {}", key);
        Ok(())
    }

    /// Load history from storage
    pub fn load_history(&mut self) -> Result<(), String> {
        if !self.config.should_persist() {
            return Ok(());
        }

        let key = self.config.persistence_key_required();

        // In a real implementation, this would load from localStorage or other persistence
        eprintln!("Loading history from key: {}", key);
        Ok(())
    }

    /// Get the underlying machine
    pub fn machine(&self) -> &Machine<C, E, C> {
        &self.base_machine
    }

    /// Get mutable access to the underlying machine
    pub fn machine_mut(&mut self) -> &mut Machine<C, E, C> {
        &mut self.base_machine
    }
}

/// Extension trait for machines to add history support
pub trait MachineHistoryExt<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
>
{
    /// Add history support to this machine
    fn with_history(self, config: HistoryConfig) -> HistoryMachine<C, E>;

    /// Add a history state to the machine builder
    fn history_state(self, state: HistoryState) -> Self;
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static, E: Clone + Send + Sync + Hash + Eq + std::fmt::Debug + 'static>
    MachineHistoryExt<C, E> for Machine<C, E, C>
{
    fn with_history(self, config: HistoryConfig) -> HistoryMachine<C, E> {
        HistoryMachine::new(self, config)
    }

    fn history_state(self, _state: HistoryState) -> Self {
        // This would be implemented in the machine builder
        // For now, just return self
        self
    }
}

/// Builder extension for adding history states
pub trait HistoryMachineBuilder<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
>
{
    /// Add a history state to the machine being built
    fn history_state(&mut self, state: HistoryState) -> &mut Self;

    /// Configure history settings
    fn with_history_config(&mut self, config: HistoryConfig) -> &mut Self;

    /// Build with history support
    fn build_with_history(self) -> HistoryMachine<C, E>;
}
