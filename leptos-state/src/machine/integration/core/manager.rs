//! Integration manager for coordinating external system integrations

use crate::machine::integration::config::IntegrationConfig;
use crate::machine::integration::events::IntegrationEvent;
use crate::machine::integration::metrics::IntegrationMetrics;
use super::adapters::IntegrationAdapterTrait;
use super::health::HealthStatus;

/// Integration manager for state machines
pub struct IntegrationManager<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + std::hash::Hash + Eq + 'static,
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
    pub filters: Vec<crate::machine::integration::events::EventFilter>,
    /// Background task handles
    pub task_handles: std::sync::Mutex<Vec<tokio::task::JoinHandle<()>>>,
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + std::fmt::Debug + 'static>
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
    pub fn unregister_adapter(&mut self, name: &str) -> Option<Box<dyn IntegrationAdapterTrait + Send + Sync>> {
        self.adapters.remove(name)
    }

    /// Get an adapter by name
    pub fn get_adapter(&self, name: &str) -> Option<&Box<dyn IntegrationAdapterTrait + Send + Sync>> {
        self.adapters.get(name)
    }

    /// Get an adapter by name (mutable)
    pub fn get_adapter_mut(&mut self, name: &str) -> Option<&mut Box<dyn IntegrationAdapterTrait + Send + Sync>> {
        self.adapters.get_mut(name)
    }

    /// List all registered adapters
    pub fn list_adapters(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }

    /// Check if an adapter is registered
    pub fn has_adapter(&self, name: &str) -> bool {
        self.adapters.contains_key(name)
    }

    /// Start an integration
    pub async fn start_integration(&self, integration_name: String) -> Result<(), String> {
        if self.active_integrations.lock().unwrap().contains(&integration_name) {
            return Err(format!("Integration '{}' is already active", integration_name));
        }

        // Start background tasks for this integration
        let handle = tokio::spawn(async move {
            // Placeholder for integration logic
            // In a real implementation, this would handle events, polling, etc.
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        });

        self.active_integrations.lock().unwrap().insert(integration_name);
        self.task_handles.lock().unwrap().push(handle);

        Ok(())
    }

    /// Stop an integration
    pub async fn stop_integration(&self, integration_name: &str) -> Result<(), String> {
        let was_active = self.active_integrations.lock().unwrap().remove(integration_name);

        if !was_active {
            return Err(format!("Integration '{}' is not active", integration_name));
        }

        // Stop associated tasks (simplified)
        // In practice, you'd want to gracefully shut down specific tasks

        Ok(())
    }

    /// Check if an integration is active
    pub fn is_integration_active(&self, integration_name: &str) -> bool {
        self.active_integrations.lock().unwrap().contains(integration_name)
    }

    /// List active integrations
    pub fn list_active_integrations(&self) -> Vec<String> {
        self.active_integrations.lock().unwrap().iter().cloned().collect()
    }

    /// Send an event to integrations
    pub async fn send_event(&self, event: IntegrationEvent) -> Result<(), String> {
        // Apply filters
        for filter in &self.filters {
            if !filter.matches(&event) {
                return Ok(()); // Filtered out
            }
        }

        // Queue the event
        self.event_queue.lock().unwrap().push(event.clone());

        // Send to all adapters
        for (name, adapter) in &self.adapters {
            match adapter.send_event(&event).await {
                Ok(_) => {
                    self.metrics.record_success(name.clone());
                }
                Err(e) => {
                    self.metrics.record_error(name.clone(), e);
                }
            }
        }

        Ok(())
    }

    /// Receive events from integrations
    pub async fn receive_events(&self) -> Vec<IntegrationEvent> {
        let mut events = Vec::new();

        for (name, adapter) in &self.adapters {
            match adapter.receive_events().await {
                Ok(mut adapter_events) => {
                    self.metrics.record_success(name.clone());
                    events.append(&mut adapter_events);
                }
                Err(e) => {
                    self.metrics.record_error(name.clone(), e);
                }
            }
        }

        events
    }

    /// Process queued events
    pub async fn process_event_queue(&self) -> Result<(), String> {
        let events = std::mem::take(&mut *self.event_queue.lock().unwrap());

        for event in events {
            // Process each event
            // This is a placeholder - real implementation would route events
            // to appropriate handlers based on event type and configuration
            self.metrics.record_event_processed();
        }

        Ok(())
    }

    /// Get health status of all integrations
    pub async fn get_health_status(&self) -> std::collections::HashMap<String, HealthStatus> {
        let mut status = std::collections::HashMap::new();

        for (name, adapter) in &self.adapters {
            let adapter_status = adapter.health_check().await;
            status.insert(name.clone(), adapter_status);
        }

        status
    }

    /// Get overall health status
    pub async fn get_overall_health(&self) -> HealthStatus {
        let statuses = self.get_health_status().await;

        if statuses.values().all(|s| matches!(s, HealthStatus::Healthy)) {
            HealthStatus::Healthy
        } else if statuses.values().any(|s| matches!(s, HealthStatus::Unhealthy)) {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Degraded
        }
    }

    /// Get metrics
    pub fn get_metrics(&self) -> &IntegrationMetrics {
        &self.metrics
    }

    /// Reset metrics
    pub fn reset_metrics(&mut self) {
        self.metrics = IntegrationMetrics::new();
    }

    /// Add an event filter
    pub fn add_filter(&mut self, filter: crate::machine::integration::events::EventFilter) {
        self.filters.push(filter);
    }

    /// Remove all filters
    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    /// Get configuration
    pub fn config(&self) -> &IntegrationConfig {
        &self.config
    }

    /// Shutdown the integration manager
    pub async fn shutdown(&self) -> Result<(), String> {
        // Stop all active integrations
        let active = self.list_active_integrations();
        for integration in active {
            let _ = self.stop_integration(&integration).await;
        }

        // Wait for all tasks to complete
        let handles = std::mem::take(&mut *self.task_handles.lock().unwrap());
        for handle in handles {
            let _ = handle.await;
        }

        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> IntegrationStatistics {
        IntegrationStatistics {
            registered_adapters: self.adapters.len(),
            active_integrations: self.active_integrations.lock().unwrap().len(),
            queued_events: self.event_queue.lock().unwrap().len(),
            total_events_processed: self.metrics.total_events_processed(),
            total_errors: self.metrics.total_errors(),
        }
    }
}

/// Integration statistics
#[derive(Debug, Clone)]
pub struct IntegrationStatistics {
    /// Number of registered adapters
    pub registered_adapters: usize,
    /// Number of active integrations
    pub active_integrations: usize,
    /// Number of queued events
    pub queued_events: usize,
    /// Total events processed
    pub total_events_processed: u64,
    /// Total errors
    pub total_errors: u64,
}

impl std::fmt::Display for IntegrationStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IntegrationStatistics {{ adapters: {}, active: {}, queued: {}, processed: {}, errors: {} }}",
            self.registered_adapters,
            self.active_integrations,
            self.queued_events,
            self.total_events_processed,
            self.total_errors
        )
    }
}
