//! State Machine Integration Patterns
//!
//! This module provides integration capabilities for state machines
//! with external systems, APIs, databases, and message queues.

use super::*;
use crate::utils::types::{StateError, StateResult};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

/// Integration configuration for state machines
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    /// Whether integration is enabled
    pub enabled: bool,
    /// Integration adapters to use
    pub adapters: Vec<IntegrationAdapter>,
    /// Event routing configuration
    pub event_routing: EventRoutingConfig,
    /// Error handling strategy
    pub error_handling: ErrorHandlingStrategy,
    /// Retry configuration
    pub retry_config: RetryConfig,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            adapters: Vec::new(),
            event_routing: EventRoutingConfig::default(),
            error_handling: ErrorHandlingStrategy::FailFast,
            retry_config: RetryConfig::default(),
        }
    }
}

/// Integration adapter for external systems
#[derive(Debug, Clone)]
pub struct IntegrationAdapter {
    /// Adapter name
    pub name: String,
    /// Adapter type
    pub adapter_type: AdapterType,
    /// Configuration for the adapter
    pub config: HashMap<String, String>,
    /// Whether the adapter is enabled
    pub enabled: bool,
}

/// Types of integration adapters
#[derive(Debug, Clone, PartialEq)]
pub enum AdapterType {
    /// HTTP/REST API adapter
    HttpApi,
    /// WebSocket adapter
    WebSocket,
    /// Database adapter
    Database,
    /// Message queue adapter
    MessageQueue,
    /// Custom adapter
    Custom(String),
}

/// Event routing configuration
#[derive(Debug, Clone)]
pub struct EventRoutingConfig {
    /// Routing rules
    pub rules: Vec<RoutingRule>,
    /// Default route
    pub default_route: Option<String>,
}

impl Default for EventRoutingConfig {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            default_route: None,
        }
    }
}

/// Routing rule for events
#[derive(Debug, Clone)]
pub struct RoutingRule {
    /// Rule name
    pub name: String,
    /// Event pattern to match
    pub pattern: EventPattern,
    /// Target route
    pub target: String,
    /// Whether the rule is enabled
    pub enabled: bool,
}

/// Event pattern for routing
#[derive(Debug, Clone)]
pub struct EventPattern {
    /// Event type pattern
    pub event_type: String,
    /// Event source pattern
    pub source: Option<String>,
}

/// Error handling strategy
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorHandlingStrategy {
    /// Fail fast on error
    FailFast,
    /// Retry with exponential backoff
    RetryWithBackoff,
    /// Continue with fallback
    ContinueWithFallback,
    /// Log and continue
    LogAndContinue,
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: usize,
    /// Initial retry delay
    pub initial_delay: Duration,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

/// Integration event for external systems
#[derive(Debug, Clone)]
pub struct IntegrationEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: String,
    /// Event source
    pub source: String,
    /// Event timestamp
    pub timestamp: Instant,
    /// Event payload
    pub payload: String,
    /// Event metadata
    pub metadata: HashMap<String, String>,
    /// Event priority
    pub priority: EventPriority,
}

/// Event priority levels
#[derive(Debug, Clone, PartialEq)]
pub enum EventPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Integration manager for state machines
pub struct IntegrationManager<C: Send + Sync + Clone + 'static, E> {
    config: IntegrationConfig,
    adapters: Arc<RwLock<HashMap<String, Box<dyn IntegrationAdapterTrait + Send + Sync>>>>,
    _event_queue: Arc<Mutex<VecDeque<IntegrationEvent>>>,
    metrics: Arc<RwLock<IntegrationMetrics>>,
    #[allow(dead_code)]
    machine: Arc<Machine<C, E, C>>,
}

impl<C, E> IntegrationManager<C, E>
where
    C: Clone + std::fmt::Debug + Send + Sync,
    E: Clone + std::fmt::Debug + Event + Send + Sync,
{
    pub fn new(machine: Machine<C, E, C>, config: IntegrationConfig) -> Self {
        Self {
            config,
            adapters: Arc::new(RwLock::new(HashMap::new())),
            _event_queue: Arc::new(Mutex::new(VecDeque::new())),
            metrics: Arc::new(RwLock::new(IntegrationMetrics::new())),
            machine: Arc::new(machine),
        }
    }

    /// Register an integration adapter
    pub fn register_adapter(
        &self,
        name: String,
        adapter: Box<dyn IntegrationAdapterTrait + Send + Sync>,
    ) {
        if let Ok(mut adapters) = self.adapters.write() {
            adapters.insert(name, adapter);
        }
    }

    /// Process an incoming integration event
    pub fn process_incoming_event(&self, event: IntegrationEvent) -> StateResult<()> {
        let start_time = Instant::now();

        // Route the event
        let route = self.route_event(&event)?;

        // Send to appropriate adapter
        self.send_event(&event, &route)?;

        // Update metrics
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.incoming_events += 1;
            metrics.total_processing_time += start_time.elapsed();
        }

        Ok(())
    }

    /// Process an outgoing integration event
    pub fn process_outgoing_event(&self, event: IntegrationEvent) -> StateResult<()> {
        let start_time = Instant::now();

        // Route the event
        let route = self.route_event(&event)?;

        // Send to appropriate adapter
        self.send_event(&event, &route)?;

        // Update metrics
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.outgoing_events += 1;
            metrics.total_processing_time += start_time.elapsed();
        }

        Ok(())
    }

    /// Route an event based on configuration
    fn route_event(&self, event: &IntegrationEvent) -> StateResult<String> {
        let rules = &self.config.event_routing.rules;

        for rule in rules {
            if rule.enabled && self.matches_pattern(event, &rule.pattern) {
                return Ok(rule.target.clone());
            }
        }

        // Use default route if available
        if let Some(default_route) = &self.config.event_routing.default_route {
            return Ok(default_route.clone());
        }

        Err(StateError::custom("No route found for event".to_string()))
    }

    /// Check if an event matches a pattern
    fn matches_pattern(&self, event: &IntegrationEvent, pattern: &EventPattern) -> bool {
        // Check event type
        if !event.event_type.contains(&pattern.event_type) {
            return false;
        }

        // Check source if specified
        if let Some(source_pattern) = &pattern.source {
            if !event.source.contains(source_pattern) {
                return false;
            }
        }

        true
    }

    /// Send event to adapter
    fn send_event(&self, event: &IntegrationEvent, route: &str) -> StateResult<()> {
        if let Ok(adapters) = self.adapters.read() {
            if let Some(adapter) = adapters.get(route) {
                return adapter.send_event(event);
            }
        }

        Err(StateError::custom(format!(
            "No adapter found for route: {}",
            route
        )))
    }

    /// Get integration metrics
    pub fn get_metrics(&self) -> IntegrationMetrics {
        if let Ok(metrics) = self.metrics.read() {
            metrics.clone()
        } else {
            IntegrationMetrics::new()
        }
    }
}

/// Integration adapter trait
pub trait IntegrationAdapterTrait: Send + Sync {
    /// Send an event
    fn send_event(&self, event: &IntegrationEvent) -> StateResult<()>;

    /// Receive events
    fn receive_events(&self) -> StateResult<Vec<IntegrationEvent>>;

    /// Get adapter name
    fn name(&self) -> &str;

    /// Check if adapter is healthy
    fn is_healthy(&self) -> bool;
}

/// HTTP API adapter
pub struct HttpApiAdapter {
    name: String,
    base_url: String,
    headers: HashMap<String, String>,
    timeout: Duration,
}

impl HttpApiAdapter {
    pub fn new(name: String, base_url: String) -> Self {
        Self {
            name,
            base_url,
            headers: HashMap::new(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

impl IntegrationAdapterTrait for HttpApiAdapter {
    fn send_event(&self, event: &IntegrationEvent) -> StateResult<()> {
        // Simulate HTTP request
        println!("Sending event {} to HTTP API: {}", event.id, self.base_url);
        Ok(())
    }

    fn receive_events(&self) -> StateResult<Vec<IntegrationEvent>> {
        // Simulate HTTP response
        println!("Receiving events from HTTP API: {}", self.base_url);
        Ok(Vec::new())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_healthy(&self) -> bool {
        // Simulate health check
        true
    }
}

/// Database adapter
pub struct DatabaseAdapter {
    name: String,
    _connection_string: String,
    table_name: String,
}

impl DatabaseAdapter {
    pub fn new(name: String, connection_string: String, table_name: String) -> Self {
        Self {
            name,
            _connection_string: connection_string,
            table_name,
        }
    }
}

impl IntegrationAdapterTrait for DatabaseAdapter {
    fn send_event(&self, event: &IntegrationEvent) -> StateResult<()> {
        println!(
            "Storing event {} in database table: {}",
            event.id, self.table_name
        );
        Ok(())
    }

    fn receive_events(&self) -> StateResult<Vec<IntegrationEvent>> {
        println!("Reading events from database table: {}", self.table_name);
        Ok(Vec::new())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_healthy(&self) -> bool {
        // Simulate database health check
        true
    }
}

/// Message queue adapter
pub struct MessageQueueAdapter {
    name: String,
    _queue_url: String,
    queue_name: String,
}

impl MessageQueueAdapter {
    pub fn new(name: String, queue_url: String, queue_name: String) -> Self {
        Self {
            name,
            _queue_url: queue_url,
            queue_name,
        }
    }
}

impl IntegrationAdapterTrait for MessageQueueAdapter {
    fn send_event(&self, event: &IntegrationEvent) -> StateResult<()> {
        println!(
            "Publishing event {} to queue: {}",
            event.id, self.queue_name
        );
        Ok(())
    }

    fn receive_events(&self) -> StateResult<Vec<IntegrationEvent>> {
        println!("Consuming events from queue: {}", self.queue_name);
        Ok(Vec::new())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_healthy(&self) -> bool {
        // Simulate queue health check
        true
    }
}

/// Integration metrics
#[derive(Debug, Clone)]
pub struct IntegrationMetrics {
    /// Number of incoming events
    pub incoming_events: usize,
    /// Number of outgoing events
    pub outgoing_events: usize,
    /// Total processing time
    pub total_processing_time: Duration,
    /// Number of errors
    pub errors: usize,
    /// Number of retries
    pub retries: usize,
}

impl IntegrationMetrics {
    pub fn new() -> Self {
        Self {
            incoming_events: 0,
            outgoing_events: 0,
            total_processing_time: Duration::ZERO,
            errors: 0,
            retries: 0,
        }
    }
}

/// Extension trait for adding integration to machines
pub trait MachineIntegrationExt<C: Send + Sync + Clone + 'static, E> {
    /// Add integration capabilities to the machine
    fn with_integration(self, config: IntegrationConfig) -> IntegrationManager<C, E>;
}

impl<C, E> MachineIntegrationExt<C, E> for Machine<C, E, C>
where
    C: Clone + std::fmt::Debug + Send + Sync,
    E: Clone + std::fmt::Debug + Event + Send + Sync,
{
    fn with_integration(self, config: IntegrationConfig) -> IntegrationManager<C, E> {
        IntegrationManager::new(self, config)
    }
}

/// Integration builder for fluent configuration
pub struct IntegrationBuilder<C: Send + Sync + Clone + 'static, E> {
    machine: Machine<C, E, C>,
    config: IntegrationConfig,
}

impl<C, E> IntegrationBuilder<C, E>
where
    C: Clone + std::fmt::Debug + Send + Sync,
    E: Clone + std::fmt::Debug + Event + Send + Sync,
{
    pub fn new(machine: Machine<C, E, C>) -> Self {
        Self {
            machine,
            config: IntegrationConfig::default(),
        }
    }

    pub fn with_config(mut self, config: IntegrationConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_adapter(mut self, adapter: IntegrationAdapter) -> Self {
        self.config.adapters.push(adapter);
        self
    }

    pub fn with_error_handling(mut self, strategy: ErrorHandlingStrategy) -> Self {
        self.config.error_handling = strategy;
        self
    }

    pub fn with_retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.config.retry_config = retry_config;
        self
    }

    pub fn build(self) -> IntegrationManager<C, E> {
        IntegrationManager::new(self.machine, self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::machine::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestContext {
        count: i32,
        name: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum TestEvent {
        Increment,
        Decrement,
        SetName(String),
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &str {
            match self {
                TestEvent::Increment => "increment",
                TestEvent::Decrement => "decrement",
                TestEvent::SetName(_) => "set_name",
            }
        }
    }

    #[test]
    fn test_integration_config_default() {
        let config = IntegrationConfig::default();
        assert!(config.enabled);
        assert!(config.adapters.is_empty());
    }

    #[test]
    fn test_http_api_adapter() {
        let adapter = HttpApiAdapter::new(
            "test_api".to_string(),
            "https://api.example.com".to_string(),
        )
        .with_header("Authorization".to_string(), "Bearer token".to_string())
        .with_timeout(Duration::from_secs(60));

        assert_eq!(adapter.name(), "test_api");
        assert!(adapter.is_healthy());

        let event = IntegrationEvent {
            id: "test_event".to_string(),
            event_type: "test".to_string(),
            source: "test".to_string(),
            timestamp: Instant::now(),
            payload: "{}".to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Normal,
        };

        let result = adapter.send_event(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_database_adapter() {
        let adapter = DatabaseAdapter::new(
            "test_db".to_string(),
            "postgresql://localhost/test".to_string(),
            "events".to_string(),
        );

        assert_eq!(adapter.name(), "test_db");
        assert!(adapter.is_healthy());

        let event = IntegrationEvent {
            id: "test_event".to_string(),
            event_type: "test".to_string(),
            source: "test".to_string(),
            timestamp: Instant::now(),
            payload: "{}".to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Normal,
        };

        let result = adapter.send_event(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_message_queue_adapter() {
        let adapter = MessageQueueAdapter::new(
            "test_queue".to_string(),
            "amqp://localhost".to_string(),
            "events".to_string(),
        );

        assert_eq!(adapter.name(), "test_queue");
        assert!(adapter.is_healthy());

        let event = IntegrationEvent {
            id: "test_event".to_string(),
            event_type: "test".to_string(),
            source: "test".to_string(),
            timestamp: Instant::now(),
            payload: "{}".to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Normal,
        };

        let result = adapter.send_event(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_integration_builder() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let integration_manager = IntegrationBuilder::new(machine)
            .with_adapter(IntegrationAdapter {
                name: "http".to_string(),
                adapter_type: AdapterType::HttpApi,
                config: HashMap::new(),
                enabled: true,
            })
            .with_error_handling(ErrorHandlingStrategy::RetryWithBackoff)
            .with_retry_config(RetryConfig {
                max_retries: 5,
                initial_delay: Duration::from_secs(1),
                max_delay: Duration::from_secs(30),
                backoff_multiplier: 2.0,
            })
            .build();

        let metrics = integration_manager.get_metrics();
        assert_eq!(metrics.incoming_events, 0);
        assert_eq!(metrics.outgoing_events, 0);
    }
}
