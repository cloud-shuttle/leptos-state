//! Core integration functionality

use super::integration_metrics::IntegrationMetrics;
use super::*;
use std::hash::Hash;

/// Integration manager for state machines
pub struct IntegrationManager<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
> {
    /// Configuration
    pub config: IntegrationConfig,
    /// Registered adapters
    pub adapters: std::collections::HashMap<String, Box<dyn IntegrationAdapterTrait + Send + Sync>>,
    /// Event queue
    pub event_queue: std::sync::Mutex<Vec<IntegrationEvent>>,
    /// Active integrations
    pub active_integrations: std::sync::Mutex<std::collections::HashSet<String>>,
    /// Metrics collector
    pub metrics: IntegrationMetrics,
    /// Event filters
    pub filters: Vec<EventFilter>,
    /// Background task handles
    pub task_handles: std::sync::Mutex<Vec<tokio::task::JoinHandle<()>>>,
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static, E: Clone + Send + Sync + Hash + Eq + std::fmt::Debug + 'static>
    IntegrationManager<C, E>
{
    /// Create a new integration manager
    pub fn new(config: IntegrationConfig) -> Self {
        Self {
            config,
            adapters: std::collections::HashMap::new(),
            event_queue: std::sync::Mutex::new(Vec::new()),
            active_integrations: std::sync::Mutex::new(std::collections::HashSet::new()),
            metrics: IntegrationMetrics::new(),
            filters: Vec::new(),
            task_handles: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Register an adapter
    pub fn register_adapter(
        &mut self,
        name: String,
        adapter: Box<dyn IntegrationAdapterTrait + Send + Sync>,
    ) {
        self.adapters.insert(name, adapter);
    }

    /// Unregister an adapter
    pub fn unregister_adapter(&mut self, name: &str) {
        self.adapters.remove(name);
    }

    /// Get an adapter by name
    pub fn get_adapter(
        &self,
        name: &str,
    ) -> Option<&Box<dyn IntegrationAdapterTrait + Send + Sync>> {
        self.adapters.get(name)
    }

    /// Send an event through integrations
    pub async fn send_event(&self, event: IntegrationEvent) -> Result<(), IntegrationError> {
        if !self.config.enabled {
            return Ok(());
        }

        // Apply filters
        if !self.should_process_event(&event) {
            self.metrics.record_filtered_event();
            return Ok(());
        }

        // Route the event
        let routes = self.route_event(&event);

        if routes.is_empty() {
            self.metrics.record_unrouted_event();
            return Ok(());
        }

        // Send to each route
        for route in routes {
            if let Some(adapter) = self.adapters.get(&route) {
                let event_clone = event.clone();
                let adapter_clone = adapter.clone_adapter();

                // Spawn background task for sending
                let handle = tokio::spawn(async move {
                    match adapter_clone.send_event(event_clone).await {
                        Ok(_) => {
                            // Record success
                        }
                        Err(error) => {
                            eprintln!("Integration error: {:?}", error);
                            // Handle error according to strategy
                        }
                    }
                });

                self.task_handles.lock().unwrap().push(handle);
            }
        }

        self.metrics.record_sent_event();
        Ok(())
    }

    /// Receive events from integrations
    pub async fn receive_events(&self) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        if !self.config.enabled {
            return Ok(Vec::new());
        }

        let mut all_events = Vec::new();

        for (name, adapter) in &self.adapters {
            match adapter.receive_events().await {
                Ok(mut events) => {
                    // Apply filters
                    events.retain(|event| self.should_process_event(event));
                    all_events.extend(events);
                }
                Err(error) => {
                    eprintln!("Error receiving from adapter {}: {:?}", name, error);
                }
            }
        }

        self.metrics.record_received_events(all_events.len());
        Ok(all_events)
    }

    /// Process an event batch
    pub async fn process_batch(&self, batch: EventBatch) -> Result<(), IntegrationError> {
        if !self.config.enabled {
            return Ok(());
        }

        // Apply filters to batch
        let filtered_events: Vec<_> = batch
            .events
            .into_iter()
            .filter(|event| self.should_process_event(event))
            .collect();

        if filtered_events.is_empty() {
            self.metrics.record_filtered_batch(batch.events.len());
            return Ok(());
        }

        // Route and send events
        for event in filtered_events {
            self.send_event(event).await?;
        }

        Ok(())
    }

    /// Check if an event should be processed based on filters
    fn should_process_event(&self, event: &IntegrationEvent) -> bool {
        // If no filters, allow all
        if self.filters.is_empty() {
            return true;
        }

        // Event must pass at least one filter
        self.filters.iter().any(|filter| filter.allows(event))
    }

    /// Route an event to appropriate adapters
    fn route_event(&self, event: &IntegrationEvent) -> Vec<String> {
        if !self.config.event_routing.enabled {
            // Send to all adapters
            return self.adapters.keys().cloned().collect();
        }

        let mut routes = Vec::new();

        // Check routing rules
        for rule in &self.config.event_routing.rules {
            if rule.enabled && rule.pattern.matches(event) {
                routes.push(rule.destination.clone());

                // Apply transformation if specified
                if let Some(ref transformation) = rule.transformation {
                    // Note: In a real implementation, we'd return transformed events
                    // For now, just route to destination
                }
            }
        }

        // If no routes found, use default
        if routes.is_empty() {
            if let Some(ref default) = self.config.event_routing.default_destination {
                routes.push(default.clone());
            } else {
                // Send to all adapters as fallback
                routes.extend(self.adapters.keys().cloned());
            }
        }

        routes
    }

    /// Add an event filter
    pub fn add_filter(&mut self, filter: EventFilter) {
        self.filters.push(filter);
    }

    /// Clear all filters
    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    /// Get current metrics
    pub fn metrics(&self) -> &IntegrationMetrics {
        &self.metrics
    }

    /// Start background processing
    pub fn start_background_processing(&self) {
        let manager = self.clone_manager();

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));

            loop {
                interval.tick().await;

                // Process queued events
                if let Ok(events) = manager.receive_events().await {
                    for event in events {
                        if let Err(e) = manager.send_event(event).await {
                            eprintln!("Background processing error: {:?}", e);
                        }
                    }
                }

                // Clean up completed tasks
                manager
                    .task_handles
                    .lock()
                    .unwrap()
                    .retain(|handle| !handle.is_finished());
            }
        });

        self.task_handles.lock().unwrap().push(handle);
    }

    /// Stop background processing
    pub async fn stop_background_processing(&self) {
        let mut handles = self.task_handles.lock().unwrap();
        for handle in handles.drain(..) {
            handle.abort();
        }
    }

    /// Clone the manager (without task handles)
    fn clone_manager(&self) -> IntegrationManager<C, E> {
        IntegrationManager {
            config: self.config.clone(),
            adapters: self.adapters.clone(),
            event_queue: std::sync::Mutex::new(Vec::new()),
            active_integrations: std::sync::Mutex::new(std::collections::HashSet::new()),
            metrics: self.metrics.clone(),
            filters: self.filters.clone(),
            task_handles: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Health check for integrations
    pub async fn health_check(&self) -> HealthStatus {
        if !self.config.enabled {
            return HealthStatus::Unknown;
        }

        let mut all_healthy = true;

        for (name, adapter) in &self.adapters {
            match adapter.health_check().await {
                Ok(true) => {
                    // Healthy
                }
                Ok(false) => {
                    eprintln!("Adapter {} is unhealthy", name);
                    all_healthy = false;
                }
                Err(error) => {
                    eprintln!("Health check error for adapter {}: {:?}", name, error);
                    all_healthy = false;
                }
            }
        }

        if all_healthy {
            HealthStatus::Healthy
        } else {
            HealthStatus::Warning
        }
    }
}

/// Integration adapter trait
#[async_trait::async_trait]
pub trait IntegrationAdapterTrait {
    /// Send an event
    async fn send_event(&self, event: IntegrationEvent) -> Result<(), IntegrationError>;

    /// Receive events
    async fn receive_events(&self) -> Result<Vec<IntegrationEvent>, IntegrationError>;

    /// Health check
    async fn health_check(&self) -> Result<bool, IntegrationError>;

    /// Clone the adapter
    fn clone_adapter(&self) -> Box<dyn IntegrationAdapterTrait + Send + Sync>;
}

/// Types of integration adapters
#[derive(Debug, Clone, PartialEq)]
pub enum AdapterType {
    /// HTTP API adapter
    HttpApi,
    /// Database adapter
    Database,
    /// Message queue adapter
    MessageQueue,
    /// File system adapter
    FileSystem,
    /// Custom adapter
    Custom(String),
}

impl AdapterType {
    /// Get adapter type as string
    pub fn as_str(&self) -> &str {
        match self {
            AdapterType::HttpApi => "http_api",
            AdapterType::Database => "database",
            AdapterType::MessageQueue => "message_queue",
            AdapterType::FileSystem => "filesystem",
            AdapterType::Custom(ref s) => s,
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
    /// Whether the adapter is enabled
    pub enabled: bool,
    /// Configuration
    pub config: ConnectionConfig,
}

impl IntegrationAdapter {
    /// Create a new integration adapter
    pub fn new(name: String, adapter_type: AdapterType, config: ConnectionConfig) -> Self {
        Self {
            name,
            adapter_type,
            enabled: true,
            config,
        }
    }

    /// Enable or disable the adapter
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Health status
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    /// System is healthy
    Healthy,
    /// System has warnings
    Warning,
    /// System has critical issues
    Critical,
    /// Health status unknown
    Unknown,
}

impl HealthStatus {
    /// Check if status is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }

    /// Check if status indicates problems
    pub fn has_issues(&self) -> bool {
        matches!(self, HealthStatus::Warning | HealthStatus::Critical)
    }
}

/// Integration pipeline for complex workflows
pub struct IntegrationPipeline<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
> {
    /// Pipeline steps
    pub steps: Vec<Box<dyn IntegrationStep<C, E>>>,
    /// Pipeline configuration
    pub config: PipelineConfig,
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static, E: Clone + Send + Sync + Hash + Eq + std::fmt::Debug + 'static>
    IntegrationPipeline<C, E>
{
    /// Create a new integration pipeline
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            config: PipelineConfig::default(),
        }
    }

    /// Add a step to the pipeline
    pub fn add_step(&mut self, step: Box<dyn IntegrationStep<C, E>>) {
        self.steps.push(step);
    }

    /// Process an event through the pipeline
    pub async fn process_event(
        &self,
        event: IntegrationEvent,
    ) -> Result<IntegrationEvent, IntegrationError> {
        let mut current_event = event;

        for step in &self.steps {
            current_event = step.process(current_event).await?;
        }

        Ok(current_event)
    }

    /// Process multiple events through the pipeline
    pub async fn process_events(
        &self,
        events: Vec<IntegrationEvent>,
    ) -> Vec<Result<IntegrationEvent, IntegrationError>> {
        let mut results = Vec::new();

        for event in events {
            let result = self.process_event(event).await;
            results.push(result);
        }

        results
    }
}

/// Integration step trait
#[async_trait::async_trait]
pub trait IntegrationStep<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
>
{
    /// Process an event
    async fn process(&self, event: IntegrationEvent) -> Result<IntegrationEvent, IntegrationError>;

    /// Get step name
    fn name(&self) -> &str;
}

/// Pipeline configuration
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Maximum processing time per step
    pub step_timeout: std::time::Duration,
    /// Whether to continue on step errors
    pub continue_on_error: bool,
    /// Maximum concurrent processing
    pub max_concurrent: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            step_timeout: std::time::Duration::from_secs(30),
            continue_on_error: false,
            max_concurrent: 10,
        }
    }
}
