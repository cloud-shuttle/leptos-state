//! Core history types and configurations

use super::*;

/// Type of history state
#[derive(Debug, Clone, PartialEq)]
pub enum HistoryType {
    /// Shallow history - remembers only the immediate substate
    Shallow,
    /// Deep history - remembers the entire substate path
    Deep,
}

impl HistoryType {
    /// Check if this is shallow history
    pub fn is_shallow(&self) -> bool {
        matches!(self, HistoryType::Shallow)
    }

    /// Check if this is deep history
    pub fn is_deep(&self) -> bool {
        matches!(self, HistoryType::Deep)
    }
}

/// History state configuration
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryState {
    /// The history state identifier
    pub id: String,
    /// Type of history
    pub history_type: HistoryType,
    /// Default target state if no history exists
    pub default_target: Option<String>,
    /// Whether to restore context along with state
    pub restore_context: bool,
    /// Maximum history depth (0 = unlimited)
    pub max_depth: usize,
    /// Whether history is enabled
    pub enabled: bool,
}

impl HistoryState {
    /// Create a new history state configuration
    pub fn new(id: String, history_type: HistoryType) -> Self {
        Self {
            id,
            history_type,
            default_target: None,
            restore_context: true,
            max_depth: 0, // unlimited
            enabled: true,
        }
    }

    /// Create a shallow history state
    pub fn shallow(id: String) -> Self {
        Self::new(id, HistoryType::Shallow)
    }

    /// Create a deep history state
    pub fn deep(id: String) -> Self {
        Self::new(id, HistoryType::Deep)
    }

    /// Set the default target state
    pub fn default_target(mut self, target: String) -> Self {
        self.default_target = Some(target);
        self
    }

    /// Enable or disable context restoration
    pub fn restore_context(mut self, restore: bool) -> Self {
        self.restore_context = restore;
        self
    }

    /// Set maximum history depth
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Enable or disable the history state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get the default target, panicking if none is set
    pub fn default_target_required(&self) -> &str {
        self.default_target.as_ref()
            .expect("History state must have a default target")
    }

    /// Check if history is limited
    pub fn has_depth_limit(&self) -> bool {
        self.max_depth > 0
    }
}

impl Default for HistoryState {
    fn default() -> Self {
        Self::shallow("history".to_string())
    }
}

/// History transition event wrapper
#[derive(Debug, Clone, PartialEq)]
pub enum HistoryEvent<E> {
    /// Regular event
    Event(E),
    /// Restore from history
    Restore,
    /// Clear history
    Clear,
}

impl<E> HistoryEvent<E> {
    /// Check if this is a regular event
    pub fn is_event(&self) -> bool {
        matches!(self, HistoryEvent::Event(_))
    }

    /// Check if this is a restore event
    pub fn is_restore(&self) -> bool {
        matches!(self, HistoryEvent::Restore)
    }

    /// Check if this is a clear event
    pub fn is_clear(&self) -> bool {
        matches!(self, HistoryEvent::Clear)
    }

    /// Extract the inner event if it's an Event variant
    pub fn into_event(self) -> Option<E> {
        match self {
            HistoryEvent::Event(e) => Some(e),
            _ => None,
        }
    }

    /// Get a reference to the inner event if it's an Event variant
    pub fn as_event(&self) -> Option<&E> {
        match self {
            HistoryEvent::Event(ref e) => Some(e),
            _ => None,
        }
    }
}

/// History configuration
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryConfig {
    /// Whether history is enabled globally
    pub enabled: bool,
    /// Maximum total history size across all states
    pub max_total_history: usize,
    /// Whether to persist history across sessions
    pub persist_history: bool,
    /// History persistence key
    pub persistence_key: Option<String>,
    /// Whether to compress history for memory efficiency
    pub compress_history: bool,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_total_history: 1000,
            persist_history: false,
            persistence_key: None,
            compress_history: false,
        }
    }
}

impl HistoryConfig {
    /// Create a new history config
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable history
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set maximum total history size
    pub fn max_total_history(mut self, max: usize) -> Self {
        self.max_total_history = max;
        self
    }

    /// Enable persistence with a key
    pub fn persist(mut self, key: String) -> Self {
        self.persist_history = true;
        self.persistence_key = Some(key);
        self
    }

    /// Enable or disable history compression
    pub fn compress(mut self, compress: bool) -> Self {
        self.compress_history = compress;
        self
    }

    /// Check if persistence is enabled
    pub fn should_persist(&self) -> bool {
        self.enabled && self.persist_history
    }

    /// Get persistence key, panicking if persistence is not enabled
    pub fn persistence_key_required(&self) -> &str {
        self.persistence_key.as_ref()
            .expect("Persistence key must be set when persistence is enabled")
    }
}

/// History entry representing a state transition
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryEntry<C: Clone + PartialEq + 'static> {
    /// The state that was active
    pub state: String,
    /// The context at the time
    pub context: Option<C>,
    /// Timestamp of the entry
    pub timestamp: std::time::Instant,
    /// Event that triggered the transition (if any)
    pub event: Option<String>,
    /// Whether this entry was restored from persistence
    pub restored: bool,
}

impl<C: Clone + PartialEq + 'static> HistoryEntry<C> {
    /// Create a new history entry
    pub fn new(state: String, context: Option<C>) -> Self {
        Self {
            state,
            context,
            timestamp: std::time::Instant::now(),
            event: None,
            restored: false,
        }
    }

    /// Create a history entry with an event
    pub fn with_event(mut self, event: String) -> Self {
        self.event = Some(event);
        self
    }

    /// Mark as restored from persistence
    pub fn restored(mut self, restored: bool) -> Self {
        self.restored = restored;
        self
    }

    /// Get the age of this entry
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
    }

    /// Check if entry is expired based on max age
    pub fn is_expired(&self, max_age: std::time::Duration) -> bool {
        self.age() > max_age
    }
}

/// History statistics
#[derive(Debug, Clone, Default)]
pub struct HistoryStats {
    /// Total number of entries
    pub total_entries: usize,
    /// Number of restored entries
    pub restored_entries: usize,
    /// Number of transitions recorded
    pub transitions_recorded: usize,
    /// Average entries per state
    pub avg_entries_per_state: f64,
    /// Memory usage estimate (bytes)
    pub memory_usage: usize,
    /// Last cleanup timestamp
    pub last_cleanup: Option<std::time::Instant>,
}

impl HistoryStats {
    /// Update statistics with current history data
    pub fn update<C: Clone + PartialEq + 'static>(&mut self, history: &[HistoryEntry<C>], state_count: usize) {
        self.total_entries = history.len();
        self.restored_entries = history.iter().filter(|e| e.restored).count();
        self.transitions_recorded = history.iter().filter(|e| e.event.is_some()).count();

        if state_count > 0 {
            self.avg_entries_per_state = self.total_entries as f64 / state_count as f64;
        }

        // Rough memory estimate
        self.memory_usage = history.iter()
            .map(|entry| {
                std::mem::size_of::<HistoryEntry<C>>() +
                entry.state.len() +
                entry.event.as_ref().map(|e| e.len()).unwrap_or(0)
            })
            .sum();
    }

    /// Record a cleanup operation
    pub fn record_cleanup(&mut self, removed_count: usize) {
        self.last_cleanup = Some(std::time::Instant::now());
        // Note: This doesn't update total_entries - that should be done by the caller
    }

    /// Get memory usage in human readable format
    pub fn memory_usage_human(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];

        let mut size = self.memory_usage as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
