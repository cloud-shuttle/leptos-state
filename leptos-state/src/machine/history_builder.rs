//! History builder extensions

use super::*;

/// Builder extension for adding history states
pub trait HistoryMachineBuilder<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Add a history state to the machine being built
    fn history_state(&mut self, state: HistoryState) -> &mut Self;

    /// Configure history settings
    fn with_history_config(&mut self, config: HistoryConfig) -> &mut Self;

    /// Build with history support
    fn build_with_history(self) -> HistoryMachine<C, E>;
}

/// History-enabled machine builder
pub struct HistoryMachineBuilderImpl<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// The base machine builder
    pub base_builder: crate::machine::MachineBuilder<C, E, C>,
    /// History configuration
    pub history_config: HistoryConfig,
    /// History states to add
    pub history_states: Vec<HistoryState>,
}

impl<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> HistoryMachineBuilderImpl<C, E> {
    /// Create a new history-enabled machine builder
    pub fn new(base_builder: crate::machine::MachineBuilder<C, E, C>) -> Self {
        Self {
            base_builder,
            history_config: HistoryConfig::default(),
            history_states: Vec::new(),
        }
    }

    /// Add a history state
    pub fn add_history_state(&mut self, state: HistoryState) {
        self.history_states.push(state);
    }

    /// Set history configuration
    pub fn set_history_config(&mut self, config: HistoryConfig) {
        self.history_config = config;
    }

    /// Build the history machine
    pub fn build(self) -> HistoryMachine<C, E> {
        let base_machine = self.base_builder.build();
        let mut history_machine = HistoryMachine::new(base_machine, self.history_config);

        // Add all history states
        for state in self.history_states {
            history_machine.add_history_state(state);
        }

        history_machine
    }
}

impl<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> HistoryMachineBuilder<C, E> for HistoryMachineBuilderImpl<C, E> {
    fn history_state(&mut self, state: HistoryState) -> &mut Self {
        self.add_history_state(state);
        self
    }

    fn with_history_config(&mut self, config: HistoryConfig) -> &mut Self {
        self.set_history_config(config);
        self
    }

    fn build_with_history(self) -> HistoryMachine<C, E> {
        self.build()
    }
}

/// Fluent API for creating history machines
pub mod history_builder {
    use super::*;

    /// Create a new history machine builder
    pub fn create_history_machine<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>() -> HistoryMachineBuilderImpl<C, E> {
        let base_builder = crate::machine::create_machine_builder();
        HistoryMachineBuilderImpl::new(base_builder)
    }

    /// Create a shallow history state
    pub fn shallow_history(id: &str) -> HistoryState {
        HistoryState::shallow(id.to_string())
    }

    /// Create a deep history state
    pub fn deep_history(id: &str) -> HistoryState {
        HistoryState::deep(id.to_string())
    }

    /// Create history configuration
    pub fn history_config() -> HistoryConfig {
        HistoryConfig::new()
    }

    /// Create a history machine with common defaults
    pub fn default_history_machine<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        initial_state: &str,
    ) -> HistoryMachineBuilderImpl<C, E> {
        let mut builder = create_history_machine();

        // Set up default history configuration
        let config = HistoryConfig {
            enabled: true,
            max_total_history: 500,
            persist_history: false,
            persistence_key: None,
            compress_history: false,
        };

        builder.with_history_config(config);

        // Add a default shallow history state
        let history_state = HistoryState::shallow("H".to_string())
            .default_target(initial_state.to_string());

        builder.history_state(history_state);

        builder
    }
}

/// History machine factory functions
pub mod factory {
    use super::*;

    /// Create a simple history machine
    pub fn simple_history_machine<C: Send + Sync + Clone + Default + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        states: Vec<&str>,
        initial_state: &str,
    ) -> Result<HistoryMachine<C, E>, String> {
        if !states.contains(&initial_state) {
            return Err(format!("Initial state '{}' not in states list", initial_state));
        }

        let mut builder = history_builder::create_history_machine();

        // Configure basic machine structure
        // Note: This is a simplified example - in practice you'd need more configuration

        let history_machine = builder.build_with_history();

        Ok(history_machine)
    }

    /// Create a history machine with persistence
    pub fn persistent_history_machine<C: Send + Sync + Clone + Default + serde::Serialize + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        states: Vec<&str>,
        initial_state: &str,
        persistence_key: &str,
    ) -> Result<HistoryMachine<C, E>, String> {
        let mut builder = history_builder::create_history_machine();

        // Configure persistence
        let config = HistoryConfig::new().persist(persistence_key.to_string());
        builder.with_history_config(config);

        // Add history state
        let history_state = HistoryState::deep("H".to_string())
            .default_target(initial_state.to_string());
        builder.history_state(history_state);

        Ok(builder.build_with_history())
    }

    /// Create a memory-efficient history machine
    pub fn memory_efficient_history_machine<C: Send + Sync + Clone + Default + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        max_history_per_state: usize,
        max_total_history: usize,
    ) -> HistoryMachineBuilderImpl<C, E> {
        let mut builder = history_builder::create_history_machine();

        let config = HistoryConfig {
            enabled: true,
            max_total_history,
            persist_history: false,
            persistence_key: None,
            compress_history: true,
        };

        builder.with_history_config(config);

        // Create a history tracker with limited capacity
        // Note: In practice, this would be configured on the tracker itself

        builder
    }
}

/// History utilities and helpers
pub mod utils {
    use super::*;

    /// Calculate optimal history size based on available memory
    pub fn calculate_optimal_history_size(available_memory_mb: usize) -> usize {
        // Rough estimate: 1KB per history entry
        const BYTES_PER_ENTRY: usize = 1024;
        const BYTES_PER_MB: usize = 1024 * 1024;

        let available_bytes = available_memory_mb * BYTES_PER_MB;
        available_bytes / BYTES_PER_ENTRY
    }

    /// Compress history data (simplified implementation)
    pub fn compress_history<C: Send + Sync + Clone + 'static>(
        history: &mut std::collections::HashMap<String, Vec<HistoryEntry<C>>>,
        max_entries_per_state: usize,
    ) {
        for entries in history.values_mut() {
            if entries.len() > max_entries_per_state {
                // Keep only the most recent entries
                let to_remove = entries.len() - max_entries_per_state;
                entries.drain(0..to_remove);
            }

            // Remove duplicate consecutive entries
            let mut i = 1;
            while i < entries.len() {
                if entries[i].state == entries[i - 1].state {
                    entries.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    /// Validate history configuration
    pub fn validate_history_config(config: &HistoryConfig) -> Result<(), String> {
        if config.persist_history && config.persistence_key.is_none() {
            return Err("Persistence enabled but no persistence key provided".to_string());
        }

        if config.max_total_history == 0 {
            return Err("Max total history cannot be zero".to_string());
        }

        Ok(())
    }

    /// Merge multiple history snapshots
    pub fn merge_history_snapshots<C: Send + Sync + Clone + 'static>(
        snapshots: Vec<HistorySnapshot<C>>,
    ) -> HistorySnapshot<C> {
        let mut merged_history: std::collections::HashMap<String, Vec<HistoryEntry<C>>> = std::collections::HashMap::new();

        for snapshot in snapshots {
            for (state_id, entries) in snapshot.history {
                let existing_entries = merged_history.entry(state_id).or_insert_with(Vec::new);
                existing_entries.extend(entries);
            }
        }

        // Sort and deduplicate entries
        for entries in merged_history.values_mut() {
            entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            entries.dedup_by(|a, b| a.timestamp == b.timestamp && a.state == b.state);
        }

        HistorySnapshot {
            history: merged_history,
            timestamp: std::time::Instant::now(),
        }
    }

    /// Export history to JSON
    pub fn export_history_to_json<C: Send + Sync + Clone + serde::Serialize + 'static>(
        history: &std::collections::HashMap<String, Vec<HistoryEntry<C>>>,
    ) -> Result<String, String> {
        serde_json::to_string_pretty(history)
            .map_err(|e| format!("Failed to export history: {}", e))
    }

    /// Import history from JSON
    pub fn import_history_from_json<C: Send + Sync + Clone + for<'de> serde::Deserialize<'de> + 'static>(
        json: &str,
    ) -> Result<std::collections::HashMap<String, Vec<HistoryEntry<C>>>, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("Failed to import history: {}", e))
    }
}

/// History event handlers and middleware
pub mod handlers {
    use super::*;

    /// History event handler trait
    pub trait HistoryEventHandler<C: Send + Sync + Clone + 'static> {
        /// Handle a history event
        fn handle_event(&self, event: &HistoryEntry<C>);

        /// Handle history cleanup
        fn handle_cleanup(&self, removed_count: usize);
    }

    /// Logging history handler
    pub struct LoggingHistoryHandler;

    impl<C: Send + Sync + Clone + 'static> HistoryEventHandler<C> for LoggingHistoryHandler {
        fn handle_event(&self, event: &HistoryEntry<C>) {
            eprintln!("History event: {} at {:?}", event.state, event.timestamp);
        }

        fn handle_cleanup(&self, removed_count: usize) {
            eprintln!("Cleaned up {} history entries", removed_count);
        }
    }

    /// Metrics history handler
    pub struct MetricsHistoryHandler {
        pub events_recorded: std::sync::atomic::AtomicU64,
        pub cleanups_performed: std::sync::atomic::AtomicU64,
    }

    impl MetricsHistoryHandler {
        pub fn new() -> Self {
            Self {
                events_recorded: std::sync::atomic::AtomicU64::new(0),
                cleanups_performed: std::sync::atomic::AtomicU64::new(0),
            }
        }

        pub fn get_events_recorded(&self) -> u64 {
            self.events_recorded.load(std::sync::atomic::Ordering::Relaxed)
        }

        pub fn get_cleanups_performed(&self) -> u64 {
            self.cleanups_performed.load(std::sync::atomic::Ordering::Relaxed)
        }
    }

    impl<C: Send + Sync + Clone + 'static> HistoryEventHandler<C> for MetricsHistoryHandler {
        fn handle_event(&self, _event: &HistoryEntry<C>) {
            self.events_recorded.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        fn handle_cleanup(&self, _removed_count: usize) {
            self.cleanups_performed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}
