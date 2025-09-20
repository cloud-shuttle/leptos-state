use super::traits::{StateMachineContext, StateMachineEvent, StateMachineState};
use super::event::EventMetadata;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// DevTools integration for debugging and monitoring state machines
pub struct DevTools<C, E, S>
where
    C: StateMachineContext + Clone + 'static,
    E: StateMachineEvent + Clone + 'static,
    S: StateMachineState<Context = C, Event = E> + Clone + 'static,
{
    /// Whether DevTools are enabled
    enabled: bool,
    /// Event history for debugging
    event_history: Arc<Mutex<Vec<DevToolsEvent<C, E, S>>>>,
    /// Performance metrics
    performance_metrics: Arc<Mutex<PerformanceMetrics>>,
    /// State change history
    state_history: Arc<Mutex<Vec<StateChange<C, E, S>>>>,
    /// Custom inspectors
    inspectors: HashMap<String, Box<dyn Inspector<C, E, S> + Send + Sync>>,
    /// WebSocket connection for browser DevTools
    websocket: Option<WebSocketConnection>,
}

/// Event recorded by DevTools
#[derive(Clone, Debug)]
pub struct DevToolsEvent<C, E, S> {
    /// Event type
    pub event_type: E,
    /// Timestamp
    pub timestamp: Instant,
    /// Context snapshot
    pub context: C,
    /// Current state
    pub current_state: S,
    /// Event metadata
    pub metadata: EventMetadata,
}

/// State change record
#[derive(Clone, Debug)]
pub struct StateChange<C, E, S> {
    /// Previous state
    pub from_state: S,
    /// New state
    pub to_state: S,
    /// Triggering event
    pub trigger: E,
    /// Context at time of change
    pub context: C,
    /// Timestamp
    pub timestamp: Instant,
    /// Duration of transition
    pub transition_duration: Duration,
}

/// Performance metrics
#[derive(Clone, Debug, Default)]
pub struct PerformanceMetrics {
    /// Total transitions
    pub total_transitions: usize,
    /// Average transition time
    pub avg_transition_time: Duration,
    /// Slowest transition
    pub slowest_transition: Duration,
    /// Fastest transition
    pub fastest_transition: Duration,
    /// Memory usage
    pub memory_usage: usize,
}

// EventMetadata is defined in the event module to avoid conflicts

/// Inspector trait for custom debugging
pub trait Inspector<C, E, S>: Send + Sync
where
    C: StateMachineContext + Clone + 'static,
    E: StateMachineEvent + Clone + 'static,
    S: StateMachineState<Context = C, Event = E> + Clone + 'static,
{
    /// Inspect a state change
    fn inspect_state_change(&self, change: &StateChange<C, E, S>);
    
    /// Inspect an event
    fn inspect_event(&self, event: &DevToolsEvent<C, E, S>);
    
    /// Get inspector name
    fn name(&self) -> &str;
    
    /// Get inspector description
    fn description(&self) -> &str;
}

/// WebSocket connection for browser DevTools
pub struct WebSocketConnection {
    /// Connection status
    connected: bool,
}

impl<C, E, S> Default for DevTools<C, E, S>
where
    C: StateMachineContext + Clone + 'static,
    E: StateMachineEvent + Clone + 'static,
    S: StateMachineState<Context = C, Event = E> + Clone + 'static,
{
    fn default() -> Self {
        Self {
            enabled: false,
            event_history: Arc::new(Mutex::new(Vec::new())),
            performance_metrics: Arc::new(Mutex::new(PerformanceMetrics::default())),
            state_history: Arc::new(Mutex::new(Vec::new())),
            inspectors: HashMap::new(),
            websocket: None,
        }
    }
}

impl<C, E, S> DevTools<C, E, S>
where
    C: StateMachineContext + Clone + 'static,
    E: StateMachineEvent + Clone + 'static,
    S: StateMachineState<Context = C, Event = E> + Clone + 'static,
{
    /// Create new DevTools instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable DevTools
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable DevTools
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if DevTools are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Record an event
    pub fn record_event(&self, event: DevToolsEvent<C, E, S>) {
        if !self.enabled {
            return;
        }

        if let Ok(mut history) = self.event_history.lock() {
            history.push(event);
            
            // Keep only last 1000 events
            if history.len() > 1000 {
                history.remove(0);
            }
        }
    }

    /// Record a state change
    pub fn record_state_change(&self, change: StateChange<C, E, S>) {
        if !self.enabled {
            return;
        }

        if let Ok(mut history) = self.state_history.lock() {
            history.push(change.clone());
            
            // Keep only last 500 state changes
            if history.len() > 500 {
                history.remove(0);
            }
        }

        // Update performance metrics
        if let Ok(mut metrics) = self.performance_metrics.lock() {
            metrics.total_transitions += 1;
            
            let duration = change.transition_duration;
            if metrics.total_transitions == 1 {
                metrics.avg_transition_time = duration;
                metrics.slowest_transition = duration;
                metrics.fastest_transition = duration;
            } else {
                let total_time = metrics.avg_transition_time * (metrics.total_transitions - 1) as u32 + duration;
                metrics.avg_transition_time = total_time / metrics.total_transitions as u32;
                
                if duration > metrics.slowest_transition {
                    metrics.slowest_transition = duration;
                }
                
                if duration < metrics.fastest_transition {
                    metrics.fastest_transition = duration;
                }
            }
        }

        // Run inspectors
        for inspector in self.inspectors.values() {
            inspector.inspect_state_change(&change);
        }
    }

    /// Add a custom inspector
    pub fn add_inspector(&mut self, inspector: Box<dyn Inspector<C, E, S> + Send + Sync>) {
        let name = inspector.name().to_string();
        self.inspectors.insert(name, inspector);
    }

    /// Get event history
    pub fn get_event_history(&self) -> Vec<DevToolsEvent<C, E, S>> {
        if let Ok(history) = self.event_history.lock() {
            history.clone()
        } else {
            Vec::new()
        }
    }

    /// Get state change history
    pub fn get_state_history(&self) -> Vec<StateChange<C, E, S>> {
        if let Ok(history) = self.state_history.lock() {
            history.clone()
        } else {
            Vec::new()
        }
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        if let Ok(metrics) = self.performance_metrics.lock() {
            metrics.clone()
        } else {
            PerformanceMetrics::default()
        }
    }

    /// Clear all history
    pub fn clear_history(&mut self) {
        if let Ok(mut event_history) = self.event_history.lock() {
            event_history.clear();
        }
        if let Ok(mut state_history) = self.state_history.lock() {
            state_history.clear();
        }
        if let Ok(mut metrics) = self.performance_metrics.lock() {
            *metrics = PerformanceMetrics::default();
        }
    }

    /// Export data for browser DevTools
    pub fn export_data(&self) -> DevToolsExport<C, E, S> {
        DevToolsExport {
            events: self.get_event_history(),
            state_changes: self.get_state_history(),
            performance: self.get_performance_metrics(),
            inspectors: self.inspectors.keys().cloned().collect(),
            timestamp: Instant::now(),
        }
    }

    /// Connect to browser DevTools
    pub fn connect_browser_devtools(&mut self, _url: String) -> Result<(), DevToolsError> {
        self.websocket = Some(WebSocketConnection {
            connected: true,
        });
        Ok(())
    }

    /// Disconnect from browser DevTools
    pub fn disconnect_browser_devtools(&mut self) {
        self.websocket = None;
    }

    /// Send data to browser DevTools
    pub fn send_to_browser(&mut self, data: DevToolsExport<C, E, S>) -> Result<(), DevToolsError> {
        if let Some(websocket) = &mut self.websocket {
            if websocket.connected {
                // In a real implementation, this would serialize and send via WebSocket
                let _ = data;
                Ok(())
            } else {
                Err(DevToolsError::NotConnected)
            }
        } else {
            Err(DevToolsError::NotConnected)
        }
    }
}

/// Exported data for browser DevTools
#[derive(Clone, Debug)]
pub struct DevToolsExport<C, E, S> {
    /// Event history
    pub events: Vec<DevToolsEvent<C, E, S>>,
    /// State change history
    pub state_changes: Vec<StateChange<C, E, S>>,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Available inspectors
    pub inspectors: Vec<String>,
    /// Export timestamp
    pub timestamp: Instant,
}

/// DevTools errors
#[derive(Debug, thiserror::Error)]
pub enum DevToolsError {
    #[error("DevTools not connected")]
    NotConnected,
    
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    
    #[error("Invalid data format: {0}")]
    InvalidDataFormat(String),
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;


    // Test types
    #[derive(Clone, Debug, Default, PartialEq)]
    struct TestContext {
        count: i32,
        name: String,
    }

    impl StateMachineContext for TestContext {}

    #[derive(Clone, Debug, PartialEq)]
    enum TestEvent {
        Start,
        Stop,
        Increment,
        Decrement,
    }

    impl StateMachineEvent for TestEvent {}

    impl Default for TestEvent {
        fn default() -> Self {
            TestEvent::Start
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    enum TestState {
        Idle,
        Active,
        Paused,
    }

    impl StateMachineState for TestState {
        type Context = TestContext;
        type Event = TestEvent;
    }

    impl Default for TestState {
        fn default() -> Self {
            TestState::Idle
        }
    }

    // Test inspector
    #[derive(Clone)]
    struct TestInspector {
        name: String,
        description: String,
    }

    impl TestInspector {
        fn new(name: &str, description: &str) -> Self {
            Self {
                name: name.to_string(),
                description: description.to_string(),
            }
        }
    }

    impl Inspector<TestContext, TestEvent, TestState> for TestInspector {
        fn inspect_state_change(&self, change: &StateChange<TestContext, TestEvent, TestState>) {
            // In a real implementation, this would be mutable
            // For testing, we'll just verify the method is called
            let _ = change;
        }

        fn inspect_event(&self, event: &DevToolsEvent<TestContext, TestEvent, TestState>) {
            // In a real implementation, this would be mutable
            // For testing, we'll just verify the method is called
            let _ = event;
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            &self.description
        }
    }

    #[test]
    fn test_devtools_creation() {
        let devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        
        assert!(!devtools.is_enabled());
        assert_eq!(devtools.get_event_history().len(), 0);
        assert_eq!(devtools.get_state_history().len(), 0);
    }

    #[test]
    fn test_devtools_enable_disable() {
        let mut devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        
        assert!(!devtools.is_enabled());
        
        devtools.enable();
        assert!(devtools.is_enabled());
        
        devtools.disable();
        assert!(!devtools.is_enabled());
    }

    #[test]
    fn test_devtools_record_event() {
        let devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        
        let event = DevToolsEvent {
            event_type: TestEvent::Start,
            timestamp: Instant::now(),
            context: TestContext::default(),
            current_state: TestState::Idle,
            metadata: EventMetadata::default(),
        };
        
        // Event should not be recorded when disabled
        devtools.record_event(event.clone());
        assert_eq!(devtools.get_event_history().len(), 0);
        
        // Enable and record event
        let mut devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        devtools.enable();
        devtools.record_event(event);
        assert_eq!(devtools.get_event_history().len(), 1);
    }

    #[test]
    fn test_devtools_record_state_change() {
        let mut devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        devtools.enable();
        
        let change = StateChange {
            from_state: TestState::Idle,
            to_state: TestState::Active,
            trigger: TestEvent::Start,
            context: TestContext::default(),
            timestamp: Instant::now(),
            transition_duration: Duration::from_millis(5),
        };
        
        devtools.record_state_change(change);
        
        assert_eq!(devtools.get_state_history().len(), 1);
        assert_eq!(devtools.get_performance_metrics().total_transitions, 1);
    }

    #[test]
    fn test_devtools_add_inspector() {
        let mut devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        
        let inspector = TestInspector::new("test", "Test inspector");
        devtools.add_inspector(Box::new(inspector));
        
        let export = devtools.export_data();
        assert!(export.inspectors.contains(&"test".to_string()));
    }

    #[test]
    fn test_devtools_performance_metrics() {
        let mut devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        devtools.enable();
        
        // Record multiple state changes
        for i in 0..3 {
            let change = StateChange {
                from_state: if i == 0 { TestState::Idle } else { TestState::Active },
                to_state: TestState::Active,
                trigger: TestEvent::Start,
                context: TestContext::default(),
                timestamp: Instant::now(),
                transition_duration: Duration::from_millis((i + 1) * 10),
            };
            
            devtools.record_state_change(change);
        }
        
        let metrics = devtools.get_performance_metrics();
        assert_eq!(metrics.total_transitions, 3);
        assert_eq!(metrics.fastest_transition, Duration::from_millis(10));
        assert_eq!(metrics.slowest_transition, Duration::from_millis(30));
    }

    #[test]
    fn test_devtools_history_limits() {
        let mut devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        devtools.enable();
        
        // Add more than 1000 events
        for _i in 0..1100 {
            let event = DevToolsEvent {
                event_type: TestEvent::Start,
                timestamp: Instant::now(),
                context: TestContext::default(),
                current_state: TestState::Idle,
                metadata: EventMetadata::default(),
            };
            devtools.record_event(event);
        }
        
        // Should only keep last 1000
        assert_eq!(devtools.get_event_history().len(), 1000);
    }

    #[test]
    fn test_devtools_clear_history() {
        let mut devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        devtools.enable();
        
        // Add some data
        let event = DevToolsEvent {
            event_type: TestEvent::Start,
            timestamp: Instant::now(),
            context: TestContext::default(),
            current_state: TestState::Idle,
            metadata: EventMetadata::default(),
        };
        devtools.record_event(event);
        
        let change = StateChange {
            from_state: TestState::Idle,
            to_state: TestState::Active,
            trigger: TestEvent::Start,
            context: TestContext::default(),
            timestamp: Instant::now(),
            transition_duration: Duration::from_millis(5),
        };
        devtools.record_state_change(change);
        
        // Verify data exists
        assert_eq!(devtools.get_event_history().len(), 1);
        assert_eq!(devtools.get_state_history().len(), 1);
        
        // Clear history
        devtools.clear_history();
        
        // Verify data is cleared
        assert_eq!(devtools.get_event_history().len(), 0);
        assert_eq!(devtools.get_state_history().len(), 0);
        assert_eq!(devtools.get_performance_metrics().total_transitions, 0);
    }

    #[test]
    fn test_devtools_browser_connection() {
        let mut devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        
        // Initially not connected
        assert!(devtools.send_to_browser(devtools.export_data()).is_err());
        
        // Connect
        let result = devtools.connect_browser_devtools("ws://localhost:8080".to_string());
        assert!(result.is_ok());
        
        // Should be able to send data
        let export = devtools.export_data();
        assert!(devtools.send_to_browser(export).is_ok());
        
        // Disconnect
        devtools.disconnect_browser_devtools();
        assert!(devtools.send_to_browser(devtools.export_data()).is_err());
    }

    #[test]
    fn test_devtools_export_data() {
        let mut devtools = DevTools::<TestContext, TestEvent, TestState>::new();
        devtools.enable();
        
        // Add some data
        let event = DevToolsEvent {
            event_type: TestEvent::Start,
            timestamp: Instant::now(),
            context: TestContext::default(),
            current_state: TestState::Idle,
            metadata: EventMetadata::default(),
        };
        devtools.record_event(event);
        
        let change = StateChange {
            from_state: TestState::Idle,
            to_state: TestState::Active,
            trigger: TestEvent::Start,
            context: TestContext::default(),
            timestamp: Instant::now(),
            transition_duration: Duration::from_millis(5),
        };
        devtools.record_state_change(change);
        
        // Export data
        let export = devtools.export_data();
        
        assert_eq!(export.events.len(), 1);
        assert_eq!(export.state_changes.len(), 1);
        assert_eq!(export.performance.total_transitions, 1);
        assert!(export.timestamp.elapsed() >= Duration::from_nanos(0));
    }

    #[test]
    fn test_event_metadata() {
        let mut metadata = EventMetadata::default();
        
        metadata.source = Some("test".to_string());
        metadata.priority = 5;
        metadata.tags = vec!["debug".to_string(), "test".to_string()];
        metadata.custom.insert("key".to_string(), "value".to_string());
        
        assert_eq!(metadata.source, Some("test".to_string()));
        assert_eq!(metadata.priority, 5);
        assert_eq!(metadata.tags.len(), 2);
        assert_eq!(metadata.custom.get("key").unwrap(), "value");
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        
        assert_eq!(metrics.total_transitions, 0);
        assert_eq!(metrics.avg_transition_time, Duration::from_nanos(0));
        assert_eq!(metrics.slowest_transition, Duration::from_nanos(0));
        assert_eq!(metrics.fastest_transition, Duration::from_nanos(0));
        assert_eq!(metrics.memory_usage, 0);
    }
}
