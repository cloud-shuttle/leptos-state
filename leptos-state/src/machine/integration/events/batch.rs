//! Event batch functionality for bulk operations

use super::super::EventPriority;

/// Event batch for bulk operations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventBatch {
    /// Batch ID
    pub id: String,
    /// Events in the batch
    pub events: Vec<super::core::IntegrationEvent>,
    /// Batch creation timestamp
    pub created_at: std::time::Instant,
    /// Batch priority (highest priority of contained events)
    pub priority: EventPriority,
    /// Batch metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Whether batch is sealed (no more events can be added)
    pub sealed: bool,
}

impl EventBatch {
    /// Create a new empty batch
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            events: Vec::new(),
            created_at: std::time::Instant::now(),
            priority: EventPriority::Normal,
            metadata: std::collections::HashMap::new(),
            sealed: false,
        }
    }

    /// Create a batch with initial events
    pub fn with_events(events: Vec<super::core::IntegrationEvent>) -> Self {
        let mut batch = Self::new();
        for event in events {
            batch.add_event(event);
        }
        batch
    }

    /// Add an event to the batch
    pub fn add_event(&mut self, event: super::core::IntegrationEvent) -> Result<(), String> {
        if self.sealed {
            return Err("Cannot add events to a sealed batch".to_string());
        }

        // Update batch priority if new event has higher priority
        if event.priority > self.priority {
            self.priority = event.priority;
        }

        self.events.push(event);
        Ok(())
    }

    /// Seal the batch (prevent further additions)
    pub fn seal(&mut self) {
        self.sealed = true;
    }

    /// Check if batch is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Get number of events in batch
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Get batch age
    pub fn age(&self) -> std::time::Duration {
        self.created_at.elapsed()
    }

    /// Get events by priority
    pub fn events_by_priority(&self, priority: EventPriority) -> Vec<&super::core::IntegrationEvent> {
        self.events.iter().filter(|e| e.priority == priority).collect()
    }

    /// Get high priority events
    pub fn high_priority_events(&self) -> Vec<&super::core::IntegrationEvent> {
        self.events.iter().filter(|e| e.is_high_priority()).collect()
    }

    /// Get low priority events
    pub fn low_priority_events(&self) -> Vec<&super::core::IntegrationEvent> {
        self.events.iter().filter(|e| e.is_low_priority()).collect()
    }

    /// Get events from a specific source
    pub fn events_from_source(&self, source: &str) -> Vec<&super::core::IntegrationEvent> {
        self.events.iter().filter(|e| e.source == source).collect()
    }

    /// Get events of a specific type
    pub fn events_of_type(&self, event_type: &str) -> Vec<&super::core::IntegrationEvent> {
        self.events.iter().filter(|e| e.event_type == event_type).collect()
    }

    /// Get events with correlation ID
    pub fn events_with_correlation(&self, correlation_id: &str) -> Vec<&super::core::IntegrationEvent> {
        self.events.iter().filter(|e| e.correlation_id() == correlation_id).collect()
    }

    /// Split batch by priority
    pub fn split_by_priority(self) -> std::collections::HashMap<EventPriority, EventBatch> {
        let mut batches = std::collections::HashMap::new();

        for event in self.events {
            batches.entry(event.priority).or_insert_with(|| {
                let mut batch = EventBatch::new();
                batch.priority = event.priority;
                batch
            }).events.push(event);
        }

        // Seal all batches
        for batch in batches.values_mut() {
            batch.seal();
        }

        batches
    }

    /// Split batch by source
    pub fn split_by_source(self) -> std::collections::HashMap<String, EventBatch> {
        let mut batches = std::collections::HashMap::new();

        for event in self.events {
            batches.entry(event.source.clone()).or_insert_with(|| {
                let mut batch = EventBatch::new();
                batch.metadata.insert("source".to_string(), event.source.clone());
                batch
            }).events.push(event);
        }

        // Seal all batches
        for batch in batches.values_mut() {
            batch.seal();
        }

        batches
    }

    /// Filter events using a predicate
    pub fn filter<F>(&self, predicate: F) -> EventBatch
    where
        F: Fn(&super::core::IntegrationEvent) -> bool,
    {
        let filtered_events = self.events.iter().filter(|e| predicate(e)).cloned().collect();
        let mut batch = EventBatch::with_events(filtered_events);
        batch.metadata.clone_from(&self.metadata);
        batch.seal();
        batch
    }

    /// Take first N events
    pub fn take(&self, n: usize) -> EventBatch {
        let taken_events = self.events.iter().take(n).cloned().collect();
        let mut batch = EventBatch::with_events(taken_events);
        batch.metadata.clone_from(&self.metadata);
        batch.metadata.insert("original_size".to_string(), self.len().to_string());
        batch.seal();
        batch
    }

    /// Skip first N events
    pub fn skip(&self, n: usize) -> EventBatch {
        let skipped_events = self.events.iter().skip(n).cloned().collect();
        let mut batch = EventBatch::with_events(skipped_events);
        batch.metadata.clone_from(&self.metadata);
        batch.metadata.insert("skipped".to_string(), n.to_string());
        batch.seal();
        batch
    }

    /// Get batch statistics
    pub fn statistics(&self) -> EventBatchStats {
        let mut stats = EventBatchStats::default();
        let mut event_types = std::collections::HashMap::new();
        let mut sources = std::collections::HashMap::new();
        let mut priorities = std::collections::HashMap::new();

        for event in &self.events {
            *event_types.entry(event.event_type.clone()).or_insert(0) += 1;
            *sources.entry(event.source.clone()).or_insert(0) += 1;
            *priorities.entry(event.priority).or_insert(0) += 1;

            stats.total_size += event.size_estimate();
        }

        stats.event_count = self.events.len();
        stats.unique_event_types = event_types.len();
        stats.unique_sources = sources.len();
        stats.priority_distribution = priorities;
        stats.average_event_size = if stats.event_count > 0 {
            stats.total_size / stats.event_count
        } else {
            0
        };

        stats
    }

    /// Get batch summary
    pub fn summary(&self) -> String {
        let stats = self.statistics();
        format!(
            "EventBatch {{ id: {}, events: {}, types: {}, sources: {}, size: {}, sealed: {} }}",
            self.id, stats.event_count, stats.unique_event_types, stats.unique_sources, stats.total_size, self.sealed
        )
    }
}

impl Default for EventBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl IntoIterator for EventBatch {
    type Item = super::core::IntegrationEvent;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.events.into_iter()
    }
}

/// Statistics for an event batch
#[derive(Debug, Clone, Default)]
pub struct EventBatchStats {
    /// Total number of events
    pub event_count: usize,
    /// Number of unique event types
    pub unique_event_types: usize,
    /// Number of unique sources
    pub unique_sources: usize,
    /// Total size in bytes
    pub total_size: usize,
    /// Average event size in bytes
    pub average_event_size: usize,
    /// Priority distribution
    pub priority_distribution: std::collections::HashMap<EventPriority, usize>,
}

impl EventBatchStats {
    /// Get most common priority
    pub fn most_common_priority(&self) -> Option<EventPriority> {
        self.priority_distribution.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(priority, _)| *priority)
    }

    /// Get priority distribution as string
    pub fn priority_summary(&self) -> String {
        let mut priorities: Vec<_> = self.priority_distribution.iter().collect();
        priorities.sort_by_key(|(p, _)| *p);

        priorities.iter()
            .map(|(priority, count)| format!("{}: {}", priority, count))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
