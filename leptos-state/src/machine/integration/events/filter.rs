//! Event filter for selective processing

use super::super::EventPriority;

/// Event filter for selective processing
#[derive(Debug, Clone)]
pub struct EventFilter {
    /// Filter name
    pub name: String,
    /// Event type patterns to include (empty means all)
    pub event_types: Vec<String>,
    /// Source patterns to include (empty means all)
    pub sources: Vec<String>,
    /// Priority filter (None means all)
    pub priority_filter: Option<PriorityFilter>,
    /// Age filter (None means no age limit)
    pub age_filter: Option<std::time::Duration>,
    /// Custom filter function
    pub custom_filter: Option<Box<dyn Fn(&super::core::IntegrationEvent) -> bool + Send + Sync>>,
    /// Whether to invert the filter (exclude matching events)
    pub invert: bool,
}

impl EventFilter {
    /// Create a new filter with a name
    pub fn new(name: String) -> Self {
        Self {
            name,
            event_types: Vec::new(),
            sources: Vec::new(),
            priority_filter: None,
            age_filter: None,
            custom_filter: None,
            invert: false,
        }
    }

    /// Create a filter that matches specific event types
    pub fn by_event_types(name: String, event_types: Vec<String>) -> Self {
        Self::new(name).with_event_types(event_types)
    }

    /// Create a filter that matches specific sources
    pub fn by_sources(name: String, sources: Vec<String>) -> Self {
        Self::new(name).with_sources(sources)
    }

    /// Create a filter that matches specific priority
    pub fn by_priority(name: String, priority: EventPriority) -> Self {
        Self::new(name).with_priority(priority)
    }

    /// Add event type patterns
    pub fn with_event_types(mut self, event_types: Vec<String>) -> Self {
        self.event_types = event_types;
        self
    }

    /// Add source patterns
    pub fn with_sources(mut self, sources: Vec<String>) -> Self {
        self.sources = sources;
        self
    }

    /// Set priority filter
    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority_filter = Some(PriorityFilter::Exact(priority));
        self
    }

    /// Set priority range filter
    pub fn with_priority_range(mut self, min: EventPriority, max: EventPriority) -> Self {
        self.priority_filter = Some(PriorityFilter::Range(min, max));
        self
    }

    /// Set minimum priority filter
    pub fn with_min_priority(mut self, min: EventPriority) -> Self {
        self.priority_filter = Some(PriorityFilter::Min(min));
        self
    }

    /// Set maximum priority filter
    pub fn with_max_priority(mut self, max: EventPriority) -> Self {
        self.priority_filter = Some(PriorityFilter::Max(max));
        self
    }

    /// Set age filter
    pub fn with_max_age(mut self, max_age: std::time::Duration) -> Self {
        self.age_filter = Some(max_age);
        self
    }

    /// Add custom filter function
    pub fn with_custom_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&super::core::IntegrationEvent) -> bool + Send + Sync + 'static,
    {
        self.custom_filter = Some(Box::new(filter));
        self
    }

    /// Invert the filter (exclude matching events instead of including)
    pub fn invert(mut self) -> Self {
        self.invert = true;
        self
    }

    /// Check if an event matches this filter
    pub fn matches(&self, event: &super::core::IntegrationEvent) -> bool {
        let mut matches = true;

        // Check event types
        if !self.event_types.is_empty() {
            matches &= self.event_types.iter().any(|pattern| event.event_type.contains(pattern));
        }

        // Check sources
        if !self.sources.is_empty() {
            matches &= self.sources.iter().any(|pattern| event.source.contains(pattern));
        }

        // Check priority
        if let Some(priority_filter) = &self.priority_filter {
            matches &= priority_filter.matches(event.priority);
        }

        // Check age
        if let Some(max_age) = self.age_filter {
            matches &= !event.is_expired(max_age);
        }

        // Check custom filter
        if let Some(custom_filter) = &self.custom_filter {
            matches &= custom_filter(event);
        }

        // Apply inversion
        if self.invert {
            matches = !matches;
        }

        matches
    }

    /// Filter a collection of events
    pub fn filter_events(&self, events: &[super::core::IntegrationEvent]) -> Vec<super::core::IntegrationEvent> {
        events.iter().filter(|e| self.matches(e)).cloned().collect()
    }

    /// Filter an event batch
    pub fn filter_batch(&self, batch: &super::batch::EventBatch) -> super::batch::EventBatch {
        batch.filter(|event| self.matches(event))
    }

    /// Get filter description
    pub fn description(&self) -> String {
        let mut desc = format!("EventFilter '{}'", self.name);

        if !self.event_types.is_empty() {
            desc.push_str(&format!(" [types: {}]", self.event_types.join(", ")));
        }

        if !self.sources.is_empty() {
            desc.push_str(&format!(" [sources: {}]", self.sources.join(", ")));
        }

        if let Some(priority_filter) = &self.priority_filter {
            desc.push_str(&format!(" [priority: {}]", priority_filter.description()));
        }

        if let Some(max_age) = self.age_filter {
            desc.push_str(&format!(" [max_age: {:.2}s]", max_age.as_secs_f64()));
        }

        if self.custom_filter.is_some() {
            desc.push_str(" [custom]");
        }

        if self.invert {
            desc.push_str(" (inverted)");
        }

        desc
    }

    /// Check if filter is empty (matches everything)
    pub fn is_empty(&self) -> bool {
        self.event_types.is_empty() &&
        self.sources.is_empty() &&
        self.priority_filter.is_none() &&
        self.age_filter.is_none() &&
        self.custom_filter.is_none()
    }

    /// Create a combined filter (AND logic)
    pub fn and(self, other: Self) -> CombinedEventFilter {
        CombinedEventFilter::new(vec![self, other], CombineLogic::And)
    }

    /// Create a combined filter (OR logic)
    pub fn or(self, other: Self) -> CombinedEventFilter {
        CombinedEventFilter::new(vec![self, other], CombineLogic::Or)
    }
}

impl Clone for EventFilter {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            event_types: self.event_types.clone(),
            sources: self.sources.clone(),
            priority_filter: self.priority_filter.clone(),
            age_filter: self.age_filter,
            custom_filter: None, // Cannot clone trait objects
            invert: self.invert,
        }
    }
}

impl std::fmt::Debug for EventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new("default".to_string())
    }
}

impl std::fmt::Display for EventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Priority filter options
#[derive(Debug, Clone)]
pub enum PriorityFilter {
    /// Exact priority match
    Exact(EventPriority),
    /// Priority range (inclusive)
    Range(EventPriority, EventPriority),
    /// Minimum priority
    Min(EventPriority),
    /// Maximum priority
    Max(EventPriority),
}

impl PriorityFilter {
    /// Check if priority matches the filter
    pub fn matches(&self, priority: EventPriority) -> bool {
        match self {
            Self::Exact(p) => priority == *p,
            Self::Range(min, max) => priority >= *min && priority <= *max,
            Self::Min(min) => priority >= *min,
            Self::Max(max) => priority <= *max,
        }
    }

    /// Get filter description
    pub fn description(&self) -> String {
        match self {
            Self::Exact(p) => format!("{}", p),
            Self::Range(min, max) => format!("{} to {}", min, max),
            Self::Min(min) => format!("≥ {}", min),
            Self::Max(max) => format!("≤ {}", max),
        }
    }
}

/// Combined event filter for complex filtering logic
#[derive(Debug, Clone)]
pub struct CombinedEventFilter {
    /// Individual filters
    pub filters: Vec<EventFilter>,
    /// Combination logic
    pub logic: CombineLogic,
}

impl CombinedEventFilter {
    /// Create a new combined filter
    pub fn new(filters: Vec<EventFilter>, logic: CombineLogic) -> Self {
        Self { filters, logic }
    }

    /// Check if an event matches the combined filter
    pub fn matches(&self, event: &super::core::IntegrationEvent) -> bool {
        match self.logic {
            CombineLogic::And => self.filters.iter().all(|f| f.matches(event)),
            CombineLogic::Or => self.filters.iter().any(|f| f.matches(event)),
        }
    }

    /// Filter events using the combined filter
    pub fn filter_events(&self, events: &[super::core::IntegrationEvent]) -> Vec<super::core::IntegrationEvent> {
        events.iter().filter(|e| self.matches(e)).cloned().collect()
    }
}

/// Logic for combining filters
#[derive(Debug, Clone)]
pub enum CombineLogic {
    /// All filters must match (AND)
    And,
    /// At least one filter must match (OR)
    Or,
}
