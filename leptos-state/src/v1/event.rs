//! # Event Handling for State Machines
//! 
//! This module provides concrete implementations for events and event handling
//! that work with our trait hierarchy.

use std::collections::HashMap;
use std::fmt;

use super::traits::{StateMachineEvent, StateMachineContext};
use super::error::StateMachineError;

// =============================================================================
// Event Types
// =============================================================================

/// A concrete event that can trigger state transitions
#[derive(Debug, Clone, PartialEq)]
pub struct Event<E>
where
    E: StateMachineEvent,
{
    /// The event type
    pub event_type: E,
    
    /// Timestamp when the event was created
    pub timestamp: std::time::Instant,
    
    /// Unique identifier for this event instance
    #[cfg(feature = "persist")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "persist"))]
    pub id: String,
    
    /// Additional data associated with the event
    pub data: EventData,
    
    /// Metadata about the event
    pub metadata: EventMetadata,
}

impl<E> Event<E>
where
    E: StateMachineEvent,
{
    /// Creates a new event
    pub fn new(event_type: E) -> Self {
        Self {
            event_type,
            timestamp: std::time::Instant::now(),
            #[cfg(feature = "persist")]
            id: uuid::Uuid::new_v4(),
            #[cfg(not(feature = "persist"))]
            id: format!("event_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()),
            data: EventData::default(),
            metadata: EventMetadata::default(),
        }
    }
    
    /// Creates an event with data
    pub fn with_data(mut self, data: EventData) -> Self {
        self.data = data;
        self
    }
    
    /// Creates an event with metadata
    pub fn with_metadata(mut self, metadata: EventMetadata) -> Self {
        self.metadata = metadata;
        self
    }
    
    /// Returns the age of this event
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
    }
    
    /// Checks if this event is expired (older than the given duration)
    pub fn is_expired(&self, max_age: std::time::Duration) -> bool {
        self.age() > max_age
    }
}

impl<E> fmt::Display for Event<E>
where
    E: StateMachineEvent,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event({:?}, id={})", self.event_type, self.id)
    }
}

// =============================================================================
// Event Data
// =============================================================================

/// Data associated with an event
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventData {
    /// String key-value pairs
    pub strings: HashMap<String, String>,
    
    /// Numeric key-value pairs
    pub numbers: HashMap<String, f64>,
    
    /// Boolean key-value pairs
    pub booleans: HashMap<String, bool>,
    
    /// Raw bytes data
    pub bytes: Vec<u8>,
}

impl EventData {
    /// Creates new empty event data
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sets a string value
    pub fn with_string(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.strings.insert(key.into(), value.into());
        self
    }
    
    /// Sets a numeric value
    pub fn with_number(mut self, key: impl Into<String>, value: f64) -> Self {
        self.numbers.insert(key.into(), value);
        self
    }
    
    /// Sets a boolean value
    pub fn with_boolean(mut self, key: impl Into<String>, value: bool) -> Self {
        self.booleans.insert(key.into(), value);
        self
    }
    
    /// Sets raw bytes data
    pub fn with_bytes(mut self, bytes: Vec<u8>) -> Self {
        self.bytes = bytes;
        self
    }
    
    /// Gets a string value
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.strings.get(key)
    }
    
    /// Gets a numeric value
    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.numbers.get(key).copied()
    }
    
    /// Gets a boolean value
    pub fn get_boolean(&self, key: &str) -> Option<bool> {
        self.booleans.get(key).copied()
    }
    
    /// Gets raw bytes data
    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

// =============================================================================
// Event Metadata
// =============================================================================

/// Metadata about an event
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventMetadata {
    /// Source of the event
    pub source: Option<String>,
    
    /// Priority of the event (higher numbers = higher priority)
    pub priority: i32,
    
    /// Whether this event should be logged
    pub should_log: bool,
    
    /// Tags for categorizing the event
    pub tags: Vec<String>,
    
    /// Custom metadata
    pub custom: HashMap<String, String>,
}

impl EventMetadata {
    /// Creates new event metadata
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sets the source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
    
    /// Sets the priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    
    /// Sets whether to log this event
    pub fn with_logging(mut self, should_log: bool) -> Self {
        self.should_log = should_log;
        self
    }
    
    /// Adds a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    /// Sets custom metadata
    pub fn with_custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }
}

// =============================================================================
// Event Queue
// =============================================================================

/// A queue for managing events in a state machine
#[derive(Debug, Clone)]
pub struct EventQueue<E>
where
    E: StateMachineEvent,
{
    /// Events waiting to be processed
    events: Vec<Event<E>>,
    
    /// Maximum number of events in the queue
    max_size: usize,
    
    /// Whether to drop expired events
    drop_expired: bool,
    
    /// Maximum age for events before they're considered expired
    max_age: Option<std::time::Duration>,
}

impl<E> EventQueue<E>
where
    E: StateMachineEvent,
{
    /// Creates a new event queue
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_size: 1000,
            drop_expired: true,
            max_age: Some(std::time::Duration::from_secs(60)),
        }
    }
    
    /// Creates a queue with custom configuration
    pub fn with_config(max_size: usize, drop_expired: bool, max_age: Option<std::time::Duration>) -> Self {
        Self {
            events: Vec::new(),
            max_size,
            drop_expired,
            max_age,
        }
    }
    
    /// Adds an event to the queue
    pub fn enqueue(&mut self, event: Event<E>) -> Result<(), EventQueueError> {
        // Check if queue is full
        if self.events.len() >= self.max_size {
            return Err(EventQueueError::QueueFull);
        }
        
        // Check if event is expired
        if self.drop_expired {
            if let Some(max_age) = self.max_age {
                if event.is_expired(max_age) {
                    return Err(EventQueueError::EventExpired);
                }
            }
        }
        
        self.events.push(event);
        Ok(())
    }
    
    /// Removes and returns the next event from the queue
    pub fn dequeue(&mut self) -> Option<Event<E>> {
        self.events.pop()
    }
    
    /// Peeks at the next event without removing it
    pub fn peek(&self) -> Option<&Event<E>> {
        self.events.last()
    }
    
    /// Returns the number of events in the queue
    pub fn len(&self) -> usize {
        self.events.len()
    }
    
    /// Checks if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    
    /// Clears all events from the queue
    pub fn clear(&mut self) {
        self.events.clear();
    }
    
    /// Removes expired events from the queue
    pub fn remove_expired(&mut self) -> usize {
        if let Some(max_age) = self.max_age {
            let initial_len = self.events.len();
            self.events.retain(|event| !event.is_expired(max_age));
            initial_len - self.events.len()
        } else {
            0
        }
    }
    
    /// Returns an iterator over all events in the queue
    pub fn iter(&self) -> std::slice::Iter<'_, Event<E>> {
        self.events.iter()
    }
}

// =============================================================================
// Event Handler
// =============================================================================

/// Handles events for a state machine
pub struct EventHandler<C, E>
where
    C: StateMachineContext,
    E: StateMachineEvent,
{
    /// Event queue
    queue: EventQueue<E>,
    
    /// Event handlers for specific event types
    handlers: HashMap<std::any::TypeId, Box<dyn EventHandlerFn<C, E>>>,
    
    /// Global event handlers
    global_handlers: Vec<Box<dyn EventHandlerFn<C, E>>>,
}

impl<C, E> EventHandler<C, E>
where
    C: StateMachineContext,
    E: StateMachineEvent,
{
    /// Creates a new event handler
    pub fn new() -> Self {
        Self {
            queue: EventQueue::new(),
            handlers: HashMap::new(),
            global_handlers: Vec::new(),
        }
    }
    
    /// Registers a handler for a specific event type
    pub fn register_handler<F>(&mut self, handler: F)
    where
        F: EventHandlerFn<C, E> + 'static,
    {
        let type_id = std::any::TypeId::of::<E>();
        self.handlers.insert(type_id, Box::new(handler));
    }
    
    /// Registers a global event handler
    pub fn register_global_handler<F>(&mut self, handler: F)
    where
        F: EventHandlerFn<C, E> + 'static,
    {
        self.global_handlers.push(Box::new(handler));
    }
    
    /// Processes all events in the queue
    pub fn process_events(&mut self, context: &mut C) -> Result<usize, StateMachineError<C, E, super::state::StateValue>> {
        let mut processed = 0;
        
        while let Some(event) = self.queue.dequeue() {
            // Call global handlers
            for handler in &self.global_handlers {
                handler.handle(context, &event)?;
            }
            
            // Call specific handlers
            if let Some(handler) = self.handlers.get(&std::any::TypeId::of::<E>()) {
                handler.handle(context, &event)?;
            }
            
            processed += 1;
        }
        
        Ok(processed)
    }
    
    /// Adds an event to the queue
    pub fn enqueue_event(&mut self, event: Event<E>) -> Result<(), EventQueueError> {
        self.queue.enqueue(event)
    }
    
    /// Returns the number of events in the queue
    pub fn queue_length(&self) -> usize {
        self.queue.len()
    }
}

// =============================================================================
// Event Handler Function Trait
// =============================================================================

/// Function that handles events
pub trait EventHandlerFn<C, E>
where
    C: StateMachineContext,
    E: StateMachineEvent,
{
    /// Handles an event
    fn handle(&self, context: &mut C, event: &Event<E>) -> Result<(), StateMachineError<C, E, super::state::StateValue>>;
}

impl<C, E, F> EventHandlerFn<C, E> for F
where
    C: StateMachineContext,
    E: StateMachineEvent,
    F: Fn(&mut C, &Event<E>) -> Result<(), StateMachineError<C, E, super::state::StateValue>>,
{
    fn handle(&self, context: &mut C, event: &Event<E>) -> Result<(), StateMachineError<C, E, super::state::StateValue>> {
        self(context, event)
    }
}

// =============================================================================
// Event Queue Error
// =============================================================================

/// Errors that can occur in the event queue
#[derive(Debug, thiserror::Error)]
pub enum EventQueueError {
    #[error("Event queue is full")]
    QueueFull,
    
    #[error("Event has expired")]
    EventExpired,
    
    #[error("Invalid event data")]
    InvalidEventData,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Start,
        Stop,
        Pause,
    }

    impl StateMachineEvent for TestEvent {}

    #[derive(Debug, Clone, Default)]
    struct TestContext {
        count: i32,
    }

    impl StateMachineContext for TestContext {}

    #[test]
    fn test_event_creation() {
        let event = Event::new(TestEvent::Start)
            .with_data(EventData::new().with_string("key", "value"))
            .with_metadata(EventMetadata::new().with_priority(10));
        
        assert_eq!(event.event_type, TestEvent::Start);
        assert_eq!(event.data.get_string("key"), Some(&"value".to_string()));
        assert_eq!(event.metadata.priority, 10);
    }

    #[test]
    fn test_event_data() {
        let data = EventData::new()
            .with_string("name", "test")
            .with_number("count", 42.0)
            .with_boolean("active", true)
            .with_bytes(vec![1, 2, 3]);
        
        assert_eq!(data.get_string("name"), Some(&"test".to_string()));
        assert_eq!(data.get_number("count"), Some(42.0));
        assert_eq!(data.get_boolean("active"), Some(true));
        assert_eq!(data.get_bytes(), &[1, 2, 3]);
    }

    #[test]
    fn test_event_queue() {
        let mut queue = EventQueue::new();
        
        let event1 = Event::new(TestEvent::Start);
        let event2 = Event::new(TestEvent::Stop);
        
        assert!(queue.enqueue(event1).is_ok());
        assert!(queue.enqueue(event2).is_ok());
        assert_eq!(queue.len(), 2);
        
        assert_eq!(queue.dequeue().unwrap().event_type, TestEvent::Stop);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_event_handler() {
        let mut handler = EventHandler::new();
        let mut context = TestContext::default();
        
        // Register a global handler
        handler.register_global_handler(|ctx: &mut TestContext, _event: &Event<TestEvent>| {
            ctx.count += 1;
            Ok(())
        });
        
        // Add events
        let event = Event::new(TestEvent::Start);
        handler.enqueue_event(event).unwrap();
        
        // Process events
        let processed = handler.process_events(&mut context).unwrap();
        assert_eq!(processed, 1);
        assert_eq!(context.count, 1);
    }
}
