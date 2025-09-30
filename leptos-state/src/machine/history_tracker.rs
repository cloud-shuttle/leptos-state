//! History tracker implementation

use super::*;

/// Tracks state history for history states
pub struct HistoryTracker<C: Send + Sync + Clone + 'static> {
    /// History storage (state_id -> history entries)
    pub history: std::collections::HashMap<String, Vec<HistoryEntry<C>>>,
    /// Maximum entries per state (0 = unlimited)
    pub max_per_state: usize,
    /// Whether tracking is enabled
    pub enabled: bool,
    /// Statistics
    pub stats: HistoryStats,
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static> HistoryTracker<C> {
    /// Create a new history tracker
    pub fn new() -> Self {
        Self {
            history: std::collections::HashMap::new(),
            max_per_state: 100,
            enabled: true,
            stats: HistoryStats::default(),
        }
    }

    /// Create a history tracker with custom settings
    pub fn with_capacity(max_per_state: usize) -> Self {
        Self {
            max_per_state,
            ..Self::new()
        }
    }

    /// Record a state transition
    pub fn record_state(&mut self, state_id: &str, context: Option<C>, event: Option<String>) {
        if !self.enabled {
            return;
        }

        let entry =
            HistoryEntry::new(state_id.to_string(), context).with_event(event.unwrap_or_default());

        let state_history = self
            .history
            .entry(state_id.to_string())
            .or_insert_with(Vec::new);

        // Add the entry
        state_history.push(entry);

        // Enforce maximum entries per state
        if self.max_per_state > 0 && state_history.len() > self.max_per_state {
            let to_remove = state_history.len() - self.max_per_state;
            state_history.drain(0..to_remove);
        }

        // Update statistics
        self.update_stats();
    }

    /// Get the last recorded state for a given state ID
    pub fn get_last_state(&self, state_id: &str) -> Option<&HistoryEntry<C>> {
        self.history
            .get(state_id)
            .and_then(|entries| entries.last())
    }

    /// Get the last N states for a given state ID
    pub fn get_last_n_states(&self, state_id: &str, n: usize) -> Vec<&HistoryEntry<C>> {
        self.history
            .get(state_id)
            .map(|entries| entries.iter().rev().take(n).collect())
            .unwrap_or_default()
    }

    /// Get all history for a specific state
    pub fn get_history(&self, state_id: &str) -> Vec<&HistoryEntry<C>> {
        self.history
            .get(state_id)
            .map(|entries| entries.iter().collect())
            .unwrap_or_default()
    }

    /// Get all history
    pub fn get_all_history(&self) -> &std::collections::HashMap<String, Vec<HistoryEntry<C>>> {
        &self.history
    }

    /// Get mutable access to all history (for persistence)
    pub fn get_all_history_mut(
        &mut self,
    ) -> &mut std::collections::HashMap<String, Vec<HistoryEntry<C>>> {
        &mut self.history
    }

    /// Clear history for a specific state
    pub fn clear_history(&mut self, state_id: &str) {
        if let Some(entries) = self.history.get_mut(state_id) {
            let removed_count = entries.len();
            entries.clear();
            self.stats.record_cleanup(removed_count);
        }
    }

    /// Clear all history
    pub fn clear_all_history(&mut self) {
        let total_removed: usize = self.history.values().map(|entries| entries.len()).sum();
        self.history.clear();
        self.stats.record_cleanup(total_removed);
        self.update_stats();
    }

    /// Remove old entries based on maximum age
    pub fn cleanup(&mut self, max_age: std::time::Duration) {
        let mut total_removed = 0;

        for entries in self.history.values_mut() {
            let initial_len = entries.len();
            entries.retain(|entry| !entry.is_expired(max_age));
            total_removed += initial_len - entries.len();
        }

        // Remove empty state entries
        self.history.retain(|_, entries| !entries.is_empty());

        if total_removed > 0 {
            self.stats.record_cleanup(total_removed);
        }

        self.update_stats();
    }

    /// Remove old entries based on maximum total entries
    pub fn cleanup_by_count(&mut self, max_total: usize) {
        let mut all_entries: Vec<(String, HistoryEntry<C>)> = Vec::new();

        // Collect all entries with their state IDs
        for (state_id, entries) in &self.history {
            for entry in entries {
                all_entries.push((state_id.clone(), entry.clone()));
            }
        }

        // Sort by timestamp (newest first)
        all_entries.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));

        // Keep only the most recent entries
        if all_entries.len() > max_total {
            let to_remove = all_entries.len() - max_total;
            all_entries.truncate(max_total);

            // Rebuild history map with only kept entries
            let mut new_history: std::collections::HashMap<String, Vec<HistoryEntry<C>>> =
                std::collections::HashMap::new();

            for (state_id, entry) in all_entries {
                new_history
                    .entry(state_id)
                    .or_insert_with(Vec::new)
                    .push(entry);
            }

            // Sort entries by timestamp within each state
            for entries in new_history.values_mut() {
                entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            }

            self.history = new_history;
            self.stats.record_cleanup(to_remove);
            self.update_stats();
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> HistoryStats {
        let mut stats = self.stats.clone();
        let state_count = self.history.len();
        let total_entries: usize = self.history.values().map(|entries| entries.len()).sum();

        stats.update(&[], state_count); // Empty vec since we don't have all entries easily accessible
        stats.total_entries = total_entries;

        if state_count > 0 {
            stats.avg_entries_per_state = total_entries as f64 / state_count as f64;
        }

        stats
    }

    /// Update internal statistics
    fn update_stats(&mut self) {
        let state_count = self.history.len();
        let total_entries: usize = self.history.values().map(|entries| entries.len()).sum();

        self.stats.update(&[], state_count);
        self.stats.total_entries = total_entries;
    }

    /// Enable or disable tracking
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if tracking is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set maximum entries per state
    pub fn set_max_per_state(&mut self, max: usize) {
        self.max_per_state = max;
    }

    /// Get the number of states being tracked
    pub fn state_count(&self) -> usize {
        self.history.len()
    }

    /// Get the total number of entries
    pub fn total_entries(&self) -> usize {
        self.history.values().map(|entries| entries.len()).sum()
    }

    /// Check if a state has any history
    pub fn has_history(&self, state_id: &str) -> bool {
        self.history
            .get(state_id)
            .map(|entries| !entries.is_empty())
            .unwrap_or(false)
    }

    /// Get the oldest entry for a state
    pub fn get_oldest_entry(&self, state_id: &str) -> Option<&HistoryEntry<C>> {
        self.history
            .get(state_id)
            .and_then(|entries| entries.first())
    }

    /// Get the newest entry for a state
    pub fn get_newest_entry(&self, state_id: &str) -> Option<&HistoryEntry<C>> {
        self.history
            .get(state_id)
            .and_then(|entries| entries.last())
    }

    /// Get entries for a state within a time range
    pub fn get_entries_in_range(
        &self,
        state_id: &str,
        start: std::time::Instant,
        end: std::time::Instant,
    ) -> Vec<&HistoryEntry<C>> {
        self.history
            .get(state_id)
            .map(|entries| {
                entries
                    .iter()
                    .filter(|entry| entry.timestamp >= start && entry.timestamp <= end)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Merge another history tracker into this one
    pub fn merge(&mut self, other: &HistoryTracker<C>) {
        for (state_id, other_entries) in &other.history {
            let entries = self
                .history
                .entry(state_id.clone())
                .or_insert_with(Vec::new);
            entries.extend(other_entries.iter().cloned());
        }

        // Sort and deduplicate entries
        for entries in self.history.values_mut() {
            entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            // Remove duplicates (same timestamp and state)
            entries.dedup_by(|a, b| a.timestamp == b.timestamp && a.state == b.state);
        }

        self.update_stats();
    }

    /// Create a snapshot of the current history state
    pub fn snapshot(&self) -> HistorySnapshot<C> {
        HistorySnapshot {
            history: self.history.clone(),
            timestamp: std::time::Instant::now(),
        }
    }

    /// Restore from a snapshot
    pub fn restore_from_snapshot(&mut self, snapshot: HistorySnapshot<C>) {
        self.history = snapshot.history;
        self.update_stats();
    }
}

/// History snapshot for backup/restore operations
#[derive(Debug, Clone)]
pub struct HistorySnapshot<C: Send + Sync + Clone + 'static> {
    /// The history data
    pub history: std::collections::HashMap<String, Vec<HistoryEntry<C>>>,
    /// When the snapshot was taken
    pub timestamp: std::time::Instant,
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static> HistorySnapshot<C> {
    /// Get the age of this snapshot
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
    }

    /// Check if snapshot is expired
    pub fn is_expired(&self, max_age: std::time::Duration) -> bool {
        self.age() > max_age
    }

    /// Get total entries in this snapshot
    pub fn total_entries(&self) -> usize {
        self.history.values().map(|entries| entries.len()).sum()
    }
}

/// Default implementation for HistoryTracker
impl<C: Clone + PartialEq + Default + Send + Sync + std::fmt::Debug + 'static> Default for HistoryTracker<C> {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over history entries
pub struct HistoryIterator<'a, C: Send + Sync + Clone + 'static> {
    entries: std::collections::hash_map::Values<'a, String, Vec<HistoryEntry<C>>>,
    current_state: Option<std::slice::Iter<'a, HistoryEntry<C>>>,
}

impl<'a, C: Send + Sync + Clone + 'static> HistoryIterator<'a, C> {
    /// Create a new history iterator
    pub fn new(history: &'a std::collections::HashMap<String, Vec<HistoryEntry<C>>>) -> Self {
        let mut entries = history.values();
        let current_state = entries.next().map(|vec| vec.iter());

        Self {
            entries,
            current_state,
        }
    }
}

impl<'a, C: Send + Sync + Clone + 'static> Iterator for HistoryIterator<'a, C> {
    type Item = &'a HistoryEntry<C>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut state_iter) = self.current_state {
                if let Some(entry) = state_iter.next() {
                    return Some(entry);
                }
            }

            self.current_state = self.entries.next().map(|vec| vec.iter());

            if self.current_state.is_none() {
                return None;
            }
        }
    }
}
