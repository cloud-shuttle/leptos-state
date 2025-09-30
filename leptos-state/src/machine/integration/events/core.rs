//! Core integration event structures and functionality

use super::super::EventPriority;

/// Integration event for external systems
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegrationEvent {
    /// Unique event ID
    pub id: String,
    /// Event type
    pub event_type: String,
    /// Event source
    pub source: String,
    /// Event priority
    pub priority: EventPriority,
    /// Event timestamp
    pub timestamp: std::time::Instant,
    /// Event data
    pub data: serde_json::Value,
    /// Event metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Correlation ID for tracking
    pub correlation_id: Option<String>,
    /// Whether this event is internal
    pub internal: bool,
}

impl IntegrationEvent {
    /// Create a new integration event
    pub fn new(event_type: String, source: String, data: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type,
            source,
            priority: EventPriority::Normal,
            timestamp: std::time::Instant::now(),
            data,
            metadata: std::collections::HashMap::new(),
            correlation_id: None,
            internal: false,
        }
    }

    /// Create an internal event
    pub fn internal(event_type: String, source: String, data: serde_json::Value) -> Self {
        Self::new(event_type, source, data).internal(true)
    }

    /// Set priority
    pub fn priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set correlation ID
    pub fn correlation_id(mut self, id: String) -> Self {
        self.correlation_id = Some(id);
        self
    }

    /// Set as internal event
    pub fn internal(mut self, internal: bool) -> Self {
        self.internal = internal;
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get age of the event
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
    }

    /// Check if event is expired based on TTL
    pub fn is_expired(&self, ttl: std::time::Duration) -> bool {
        self.age() > ttl
    }

    /// Check if event has correlation ID
    pub fn has_correlation_id(&self) -> bool {
        self.correlation_id.is_some()
    }

    /// Get correlation ID or default
    pub fn correlation_id(&self) -> &str {
        self.correlation_id.as_deref().unwrap_or("")
    }

    /// Check if event has metadata
    pub fn has_metadata(&self) -> bool {
        !self.metadata.is_empty()
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if event is high priority
    pub fn is_high_priority(&self) -> bool {
        matches!(self.priority, EventPriority::High | EventPriority::Critical)
    }

    /// Check if event is low priority
    pub fn is_low_priority(&self) -> bool {
        matches!(self.priority, EventPriority::Low)
    }

    /// Get event size estimate
    pub fn size_estimate(&self) -> usize {
        self.event_type.len() +
        self.source.len() +
        self.data.to_string().len() +
        self.metadata.values().map(|v| v.len()).sum::<usize>() +
        self.correlation_id.as_ref().map_or(0, |s| s.len())
    }

    /// Create a copy with new ID
    pub fn copy_with_new_id(&self) -> Self {
        let mut copy = self.clone();
        copy.id = uuid::Uuid::new_v4().to_string();
        copy.timestamp = std::time::Instant::now();
        copy
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Create from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Get event signature for logging
    pub fn signature(&self) -> String {
        format!("{}@{}[{}]", self.event_type, self.source, self.id)
    }

    /// Check if event matches a pattern
    pub fn matches(&self, pattern: &str) -> bool {
        self.event_type.contains(pattern) ||
        self.source.contains(pattern) ||
        self.id.contains(pattern)
    }

    /// Create a summary of the event
    pub fn summary(&self) -> String {
        format!(
            "IntegrationEvent {{ type: {}, source: {}, priority: {}, size: {} }}",
            self.event_type,
            self.source,
            self.priority.as_str(),
            self.size_estimate()
        )
    }
}

impl Default for IntegrationEvent {
    fn default() -> Self {
        Self::new(
            "unknown".to_string(),
            "unknown".to_string(),
            serde_json::Value::Null,
        )
    }
}

impl std::fmt::Display for IntegrationEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl PartialEq for IntegrationEvent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for IntegrationEvent {}

impl std::hash::Hash for IntegrationEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for IntegrationEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntegrationEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
