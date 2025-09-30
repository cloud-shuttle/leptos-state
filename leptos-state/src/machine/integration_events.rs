//! Integration event structures

use super::*;

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

    /// Mark as internal event
    pub fn internal(mut self, internal: bool) -> Self {
        self.internal = internal;
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get data as typed value
    pub fn get_data<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.data.clone())
    }

    /// Check if event is expired (based on metadata)
    pub fn is_expired(&self) -> bool {
        if let Some(ttl_str) = self.metadata.get("ttl_seconds") {
            if let Ok(ttl_secs) = ttl_str.parse::<u64>() {
                let ttl_duration = std::time::Duration::from_secs(ttl_secs);
                return self.timestamp.elapsed() > ttl_duration;
            }
        }
        false
    }

    /// Get event age
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
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

/// Event priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    /// Lowest priority
    Lowest = 0,
    /// Low priority
    Low = 1,
    /// Normal priority
    Normal = 2,
    /// High priority
    High = 3,
    /// Highest priority
    Highest = 4,
    /// Critical priority (processed immediately)
    Critical = 5,
}

impl EventPriority {
    /// Get priority as string
    pub fn as_str(&self) -> &'static str {
        match self {
            EventPriority::Lowest => "lowest",
            EventPriority::Low => "low",
            EventPriority::Normal => "normal",
            EventPriority::High => "high",
            EventPriority::Highest => "highest",
            EventPriority::Critical => "critical",
        }
    }

    /// Parse priority from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "lowest" => Some(EventPriority::Lowest),
            "low" => Some(EventPriority::Low),
            "normal" => Some(EventPriority::Normal),
            "high" => Some(EventPriority::High),
            "highest" => Some(EventPriority::Highest),
            "critical" => Some(EventPriority::Critical),
            _ => None,
        }
    }
}

/// Error handling strategy
#[derive(Debug, PartialEq)]
pub enum ErrorHandlingStrategy {
    /// Ignore errors and continue
    Ignore,
    /// Log errors but continue
    LogAndContinue,
    /// Retry on error
    Retry,
    /// Fail fast on error
    FailFast,
    /// Use dead letter queue
    DeadLetterQueue,
    /// Custom error handler
    Custom(Box<dyn Fn(&IntegrationError) -> ErrorAction + Send + Sync>),
}

impl Clone for ErrorHandlingStrategy {
    fn clone(&self) -> Self {
        match self {
            Self::Ignore => Self::Ignore,
            Self::LogAndContinue => Self::LogAndContinue,
            Self::Retry => Self::Retry,
            Self::FailFast => Self::FailFast,
            Self::DeadLetterQueue => Self::DeadLetterQueue,
            Self::Custom(_) => Self::Ignore, // Can't clone trait objects, fallback to Ignore
        }
    }
}

impl ErrorHandlingStrategy {
    /// Handle an error according to the strategy
    pub fn handle_error(&self, error: &IntegrationError) -> ErrorAction {
        match self {
            ErrorHandlingStrategy::Ignore => ErrorAction::Ignore,
            ErrorHandlingStrategy::LogAndContinue => {
                eprintln!("Integration error: {:?}", error);
                ErrorAction::Continue
            }
            ErrorHandlingStrategy::Retry => ErrorAction::Retry,
            ErrorHandlingStrategy::FailFast => ErrorAction::Fail,
            ErrorHandlingStrategy::DeadLetterQueue => ErrorAction::DeadLetter,
            ErrorHandlingStrategy::Custom(handler) => handler(error),
        }
    }
}

/// Error action to take
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorAction {
    /// Ignore the error
    Ignore,
    /// Continue processing
    Continue,
    /// Retry the operation
    Retry,
    /// Fail the operation
    Fail,
    /// Send to dead letter queue
    DeadLetter,
}

/// Integration error
#[derive(Debug, Clone)]
pub struct IntegrationError {
    /// Error type
    pub error_type: IntegrationErrorType,
    /// Error message
    pub message: String,
    /// Source event that caused the error
    pub source_event: Option<IntegrationEvent>,
    /// Timestamp
    pub timestamp: std::time::Instant,
    /// Retry count
    pub retry_count: usize,
    /// Context information
    pub context: std::collections::HashMap<String, String>,
}

impl IntegrationError {
    /// Create a new integration error
    pub fn new(error_type: IntegrationErrorType, message: String) -> Self {
        Self {
            error_type,
            message,
            source_event: None,
            timestamp: std::time::Instant::now(),
            retry_count: 0,
            context: std::collections::HashMap::new(),
        }
    }

    /// Set source event
    pub fn source_event(mut self, event: IntegrationEvent) -> Self {
        self.source_event = Some(event);
        self
    }

    /// Set retry count
    pub fn retry_count(mut self, count: usize) -> Self {
        self.retry_count = count;
        self
    }

    /// Add context
    pub fn context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }

    /// Check if error should be retried
    pub fn should_retry(&self, max_retries: usize) -> bool {
        self.retry_count < max_retries
    }

    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }
}

/// Integration error types
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationErrorType {
    /// Network error
    NetworkError,
    /// Authentication error
    AuthenticationError,
    /// Authorization error
    AuthorizationError,
    /// Configuration error
    ConfigurationError,
    /// Timeout error
    TimeoutError,
    /// Serialization error
    SerializationError,
    /// Deserialization error
    DeserializationError,
    /// Validation error
    ValidationError,
    /// Rate limit exceeded
    RateLimitError,
    /// External service error
    ExternalServiceError,
    /// Internal error
    InternalError,
    /// Unknown error
    Unknown,
}

/// Event batch for bulk operations
#[derive(Debug, Clone)]
pub struct EventBatch {
    /// Events in the batch
    pub events: Vec<IntegrationEvent>,
    /// Batch ID
    pub id: String,
    /// Batch creation timestamp
    pub created_at: std::time::Instant,
    /// Batch metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Processing priority
    pub priority: EventPriority,
}

impl EventBatch {
    /// Create a new event batch
    pub fn new(events: Vec<IntegrationEvent>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            events,
            created_at: std::time::Instant::now(),
            metadata: std::collections::HashMap::new(),
            priority: EventPriority::Normal,
        }
    }

    /// Add an event to the batch
    pub fn add_event(&mut self, event: IntegrationEvent) {
        self.events.push(event);
    }

    /// Set priority
    pub fn priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get batch size
    pub fn size(&self) -> usize {
        self.events.len()
    }

    /// Check if batch is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Get batch age
    pub fn age(&self) -> std::time::Duration {
        self.created_at.elapsed()
    }

    /// Split batch into smaller batches
    pub fn split(self, max_size: usize) -> Vec<EventBatch> {
        if self.events.len() <= max_size {
            return vec![self];
        }

        let mut batches = Vec::new();
        let mut current_batch_events = Vec::new();

        for event in self.events {
            current_batch_events.push(event);

            if current_batch_events.len() >= max_size {
                let batch = EventBatch {
                    id: uuid::Uuid::new_v4().to_string(),
                    events: std::mem::take(&mut current_batch_events),
                    created_at: self.created_at,
                    metadata: self.metadata.clone(),
                    priority: self.priority.clone(),
                };
                batches.push(batch);
            }
        }

        // Add remaining events
        if !current_batch_events.is_empty() {
            let batch = EventBatch {
                id: uuid::Uuid::new_v4().to_string(),
                events: current_batch_events,
                created_at: self.created_at,
                metadata: self.metadata,
                priority: self.priority.clone(),
            };
            batches.push(batch);
        }

        batches
    }
}

/// Event filter for selective processing
#[derive(Debug)]
pub struct EventFilter {
    /// Event types to include (None means all)
    pub include_event_types: Option<Vec<String>>,
    /// Event types to exclude
    pub exclude_event_types: Vec<String>,
    /// Sources to include (None means all)
    pub include_sources: Option<Vec<String>>,
    /// Sources to exclude
    pub exclude_sources: Vec<String>,
    /// Minimum priority
    pub min_priority: Option<EventPriority>,
    /// Maximum priority
    pub max_priority: Option<EventPriority>,
    /// Custom filter function
    pub custom_filter: Option<Box<dyn Fn(&IntegrationEvent) -> bool + Send + Sync>>,
}

impl Clone for EventFilter {
    fn clone(&self) -> Self {
        Self {
            include_event_types: self.include_event_types.clone(),
            exclude_event_types: self.exclude_event_types.clone(),
            include_sources: self.include_sources.clone(),
            exclude_sources: self.exclude_sources.clone(),
            min_priority: self.min_priority,
            max_priority: self.max_priority,
            custom_filter: None, // Can't clone trait objects
        }
    }
}

impl EventFilter {
    /// Create a new event filter that allows all events
    pub fn allow_all() -> Self {
        Self {
            include_event_types: None,
            exclude_event_types: Vec::new(),
            include_sources: None,
            exclude_sources: Vec::new(),
            min_priority: None,
            max_priority: None,
            custom_filter: None,
        }
    }

    /// Create a new event filter that blocks all events
    pub fn block_all() -> Self {
        Self {
            include_event_types: Some(Vec::new()),
            exclude_event_types: Vec::new(),
            include_sources: Some(Vec::new()),
            exclude_sources: Vec::new(),
            min_priority: None,
            max_priority: None,
            custom_filter: None,
        }
    }

    /// Include specific event types
    pub fn include_event_types(mut self, types: Vec<String>) -> Self {
        self.include_event_types = Some(types);
        self
    }

    /// Exclude specific event types
    pub fn exclude_event_types(mut self, types: Vec<String>) -> Self {
        self.exclude_event_types = types;
        self
    }

    /// Include specific sources
    pub fn include_sources(mut self, sources: Vec<String>) -> Self {
        self.include_sources = Some(sources);
        self
    }

    /// Exclude specific sources
    pub fn exclude_sources(mut self, sources: Vec<String>) -> Self {
        self.exclude_sources = sources;
        self
    }

    /// Set minimum priority
    pub fn min_priority(mut self, priority: EventPriority) -> Self {
        self.min_priority = Some(priority);
        self
    }

    /// Set maximum priority
    pub fn max_priority(mut self, priority: EventPriority) -> Self {
        self.max_priority = Some(priority);
        self
    }

    /// Add custom filter
    pub fn custom_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&IntegrationEvent) -> bool + Send + Sync + 'static,
    {
        self.custom_filter = Some(Box::new(filter));
        self
    }

    /// Check if event passes the filter
    pub fn allows(&self, event: &IntegrationEvent) -> bool {
        // Check event type inclusion
        if let Some(ref include_types) = self.include_event_types {
            if !include_types.contains(&event.event_type) {
                return false;
            }
        }

        // Check event type exclusion
        if self.exclude_event_types.contains(&event.event_type) {
            return false;
        }

        // Check source inclusion
        if let Some(ref include_sources) = self.include_sources {
            if !include_sources.contains(&event.source) {
                return false;
            }
        }

        // Check source exclusion
        if self.exclude_sources.contains(&event.source) {
            return false;
        }

        // Check priority range
        if let Some(min_pri) = &self.min_priority {
            if event.priority < min_pri {
                return false;
            }
        }

        if let Some(max_pri) = &self.max_priority {
            if event.priority > max_pri {
                return false;
            }
        }

        // Check custom filter
        if let Some(ref filter) = self.custom_filter {
            if !filter(event) {
                return false;
            }
        }

        true
    }
}
