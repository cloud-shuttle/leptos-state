//! Extension traits for integration

use super::*;
use std::hash::Hash;

/// Extension trait for adding integration to machines
pub trait MachineIntegrationExt<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Get an integration manager for this machine
    fn integration_manager(&self, config: IntegrationConfig) -> IntegrationManager<C, E>;

    /// Send an event through integrations
    fn send_integration_event(&self, manager: &IntegrationManager<C, E>, event: IntegrationEvent) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), IntegrationError>> + Send + '_>>;

    /// Create a WebSocket integration
    fn with_websocket_integration(&self, manager: &mut IntegrationManager<C, E>, config: ConnectionConfig) -> Result<(), String>;

    /// Create an HTTP API integration
    fn with_http_integration(&self, manager: &mut IntegrationManager<C, E>, config: ConnectionConfig) -> Result<(), String>;

    /// Create a database integration
    fn with_database_integration(&self, manager: &mut IntegrationManager<C, E>, config: ConnectionConfig) -> Result<(), String>;
}

impl<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> MachineIntegrationExt<C, E> for Machine<C, E, C> {
    fn integration_manager(&self, config: IntegrationConfig) -> IntegrationManager<C, E> {
        IntegrationManager::new(config)
    }

    fn send_integration_event(&self, manager: &IntegrationManager<C, E>, event: IntegrationEvent) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), IntegrationError>> + Send + '_>> {
        Box::pin(manager.send_event(event))
    }

    fn with_websocket_integration(&self, manager: &mut IntegrationManager<C, E>, config: ConnectionConfig) -> Result<(), String> {
        let adapter = WebSocketAdapter::new(config);
        manager.register_adapter("websocket".to_string(), Box::new(adapter));
        Ok(())
    }

    fn with_http_integration(&self, manager: &mut IntegrationManager<C, E>, config: ConnectionConfig) -> Result<(), String> {
        let adapter = HttpApiAdapter::new(config);
        manager.register_adapter("http".to_string(), Box::new(adapter));
        Ok(())
    }

    fn with_database_integration(&self, manager: &mut IntegrationManager<C, E>, config: ConnectionConfig) -> Result<(), String> {
        let adapter = DatabaseAdapter::new(config);
        manager.register_adapter("database".to_string(), Box::new(adapter));
        Ok(())
    }
}

/// Integration builder for fluent configuration
pub struct IntegrationBuilder<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Integration manager being built
    pub manager: IntegrationManager<C, E>,
    /// Builder configuration
    pub config: IntegrationConfig,
}

impl<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> IntegrationBuilder<C, E> {
    /// Create a new integration builder
    pub fn new(machine: &Machine<C, E, C>) -> Self {
        let config = IntegrationConfig::default();
        let manager = machine.integration_manager(config.clone());

        Self { manager, config }
    }

    /// Set integration configuration
    pub fn with_config(mut self, config: IntegrationConfig) -> Self {
        self.config = config;
        self.manager.config = config.clone();
        self
    }

    /// Enable integration
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self.manager.config.enabled = enabled;
        self
    }

    /// Set maximum concurrent operations
    pub fn max_concurrent(mut self, max: usize) -> Self {
        self.config.max_concurrent = max;
        self.manager.config.max_concurrent = max;
        self
    }

    /// Add a WebSocket adapter
    pub fn with_websocket(mut self, config: ConnectionConfig) -> Self {
        let adapter = WebSocketAdapter::new(config);
        self.manager.register_adapter("websocket".to_string(), Box::new(adapter));
        self
    }

    /// Add an HTTP API adapter
    pub fn with_http_api(mut self, config: ConnectionConfig) -> Self {
        let adapter = HttpApiAdapter::new(config);
        self.manager.register_adapter("http_api".to_string(), Box::new(adapter));
        self
    }

    /// Add a database adapter
    pub fn with_database(mut self, config: ConnectionConfig) -> Self {
        let adapter = DatabaseAdapter::new(config);
        self.manager.register_adapter("database".to_string(), Box::new(adapter));
        self
    }

    /// Add a message queue adapter
    pub fn with_message_queue(mut self, config: ConnectionConfig) -> Self {
        let adapter = MessageQueueAdapter::new(config);
        self.manager.register_adapter("message_queue".to_string(), Box::new(adapter));
        self
    }

    /// Add a file system adapter
    pub fn with_file_system(mut self, config: ConnectionConfig, output_dir: std::path::PathBuf) -> Self {
        let adapter = FileSystemAdapter::new(config, output_dir);
        self.manager.register_adapter("filesystem".to_string(), Box::new(adapter));
        self
    }

    /// Add event routing rules
    pub fn with_routing(mut self, routing_config: EventRoutingConfig) -> Self {
        self.config.event_routing = routing_config;
        self.manager.config.event_routing = routing_config;
        self
    }

    /// Add event filters
    pub fn with_filters(mut self, filters: Vec<EventFilter>) -> Self {
        for filter in filters {
            self.manager.add_filter(filter);
        }
        self
    }

    /// Enable metrics collection
    pub fn with_metrics(mut self, enable: bool) -> Self {
        self.config.collect_metrics = enable;
        self.manager.config.collect_metrics = enable;
        self
    }

    /// Build the integration manager
    pub fn build(self) -> IntegrationManager<C, E> {
        self.manager
    }

    /// Build and start background processing
    pub fn build_and_start(self) -> IntegrationManager<C, E> {
        let manager = self.manager;
        manager.start_background_processing();
        manager
    }
}

/// Fluent API for creating integrations
pub mod integrations {
    use super::*;

    /// Create a WebSocket integration
    pub fn websocket<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        config: ConnectionConfig
    ) -> Box<dyn IntegrationAdapterTrait + Send + Sync> {
        Box::new(WebSocketAdapter::new(config))
    }

    /// Create an HTTP API integration
    pub fn http_api<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        config: ConnectionConfig
    ) -> Box<dyn IntegrationAdapterTrait + Send + Sync> {
        Box::new(HttpApiAdapter::new(config))
    }

    /// Create a database integration
    pub fn database<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        config: ConnectionConfig
    ) -> Box<dyn IntegrationAdapterTrait + Send + Sync> {
        Box::new(DatabaseAdapter::new(config))
    }

    /// Create a message queue integration
    pub fn message_queue<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        config: ConnectionConfig
    ) -> Box<dyn IntegrationAdapterTrait + Send + Sync> {
        Box::new(MessageQueueAdapter::new(config))
    }

    /// Create a file system integration
    pub fn filesystem<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        config: ConnectionConfig,
        output_dir: std::path::PathBuf
    ) -> Box<dyn IntegrationAdapterTrait + Send + Sync> {
        Box::new(FileSystemAdapter::new(config, output_dir))
    }

    /// Create an event filter that allows all events
    pub fn allow_all_events() -> EventFilter {
        EventFilter::allow_all()
    }

    /// Create an event filter that blocks all events
    pub fn block_all_events() -> EventFilter {
        EventFilter::block_all()
    }

    /// Create an event filter for specific event types
    pub fn filter_event_types(event_types: Vec<String>) -> EventFilter {
        EventFilter::allow_all().include_event_types(event_types)
    }

    /// Create an event filter for specific sources
    pub fn filter_sources(sources: Vec<String>) -> EventFilter {
        EventFilter::allow_all().include_sources(sources)
    }

    /// Create a routing rule
    pub fn route_to(event_pattern: EventPattern, destination: String) -> RoutingRule {
        RoutingRule::new("auto_generated".to_string(), event_pattern, destination)
    }

    /// Create a basic HTTP connection config
    pub fn http_connection(url: String) -> ConnectionConfig {
        ConnectionConfig::new(url)
    }

    /// Create a database connection config
    pub fn database_connection(url: String) -> ConnectionConfig {
        ConnectionConfig::new(url)
    }

    /// Create a message queue connection config
    pub fn message_queue_connection(url: String) -> ConnectionConfig {
        ConnectionConfig::new(url)
    }
}

/// Integration presets for common use cases
pub mod presets {
    use super::*;

    /// Create a web application integration setup
    pub fn web_application<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        machine: &Machine<C, E, C>,
        api_url: String,
        websocket_url: String
    ) -> IntegrationBuilder<C, E> {
        IntegrationBuilder::new(machine)
            .enabled(true)
            .max_concurrent(20)
            .with_http_api(ConnectionConfig::new(api_url))
            .with_websocket(ConnectionConfig::new(websocket_url))
            .with_metrics(true)
    }

    /// Create a microservice integration setup
    pub fn microservice<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        machine: &Machine<C, E, C>,
        message_queue_url: String,
        database_url: String
    ) -> IntegrationBuilder<C, E> {
        IntegrationBuilder::new(machine)
            .enabled(true)
            .max_concurrent(50)
            .with_message_queue(ConnectionConfig::new(message_queue_url))
            .with_database(ConnectionConfig::new(database_url))
            .with_routing(EventRoutingConfig::default())
            .with_metrics(true)
    }

    /// Create a data processing integration setup
    pub fn data_processing<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        machine: &Machine<C, E, C>,
        output_dir: std::path::PathBuf,
        database_url: String
    ) -> IntegrationBuilder<C, E> {
        IntegrationBuilder::new(machine)
            .enabled(true)
            .max_concurrent(10)
            .with_file_system(ConnectionConfig::new("file://localhost".to_string()), output_dir)
            .with_database(ConnectionConfig::new(database_url))
            .with_metrics(true)
    }

    /// Create a monitoring integration setup
    pub fn monitoring<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        machine: &Machine<C, E, C>,
        monitoring_endpoint: String
    ) -> IntegrationBuilder<C, E> {
        IntegrationBuilder::new(machine)
            .enabled(true)
            .max_concurrent(5)
            .with_http_api(ConnectionConfig::new(monitoring_endpoint))
            .with_metrics(true)
    }
}
