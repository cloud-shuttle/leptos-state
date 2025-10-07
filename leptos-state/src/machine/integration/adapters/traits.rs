//! Common traits for integration adapters

use super::super::events::{IntegrationEvent, IntegrationError};
use async_trait::async_trait;

/// Common trait for all integration adapters
#[async_trait]
pub trait IntegrationAdapterTrait: Send + Sync {
    /// Get the adapter type
    fn adapter_type(&self) -> AdapterType;

    /// Get the adapter name
    fn name(&self) -> &str;

    /// Get adapter configuration as JSON
    fn config(&self) -> serde_json::Value;

    /// Check if the adapter is healthy/available
    async fn health_check(&self) -> Result<(), IntegrationError>;

    /// Send an event through this adapter
    async fn send_event(&self, event: &IntegrationEvent) -> Result<(), IntegrationError>;

    /// Receive events from this adapter (if supported)
    async fn receive_events(&self) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        Err(IntegrationError::NotSupported("Event receiving not supported by this adapter".into()))
    }

    /// Get adapter statistics
    fn stats(&self) -> AdapterStats;

    /// Check if adapter supports a specific feature
    fn supports_feature(&self, feature: AdapterFeature) -> bool {
        match feature {
            AdapterFeature::SendEvents => true,
            AdapterFeature::ReceiveEvents => false,
            AdapterFeature::HealthCheck => true,
            AdapterFeature::Batching => false,
            AdapterFeature::RetryLogic => false,
            AdapterFeature::Authentication => false,
        }
    }
}

/// Adapter types enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AdapterType {
    /// REST API adapter
    RestApi,
    /// Database adapter
    Database,
    /// Message queue adapter
    MessageQueue,
    /// File system adapter
    FileSystem,
    /// WebSocket adapter
    WebSocket,
    /// Custom adapter
    Custom(String),
}

impl std::fmt::Display for AdapterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterType::RestApi => write!(f, "REST API"),
            AdapterType::Database => write!(f, "Database"),
            AdapterType::MessageQueue => write!(f, "Message Queue"),
            AdapterType::FileSystem => write!(f, "File System"),
            AdapterType::WebSocket => write!(f, "WebSocket"),
            AdapterType::Custom(name) => write!(f, "Custom: {}", name),
        }
    }
}

/// Adapter features that may be supported
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdapterFeature {
    /// Can send events
    SendEvents,
    /// Can receive events
    ReceiveEvents,
    /// Supports health checks
    HealthCheck,
    /// Supports batching multiple events
    Batching,
    /// Has built-in retry logic
    RetryLogic,
    /// Supports authentication
    Authentication,
}

/// Adapter statistics
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdapterStats {
    /// Total events sent
    pub events_sent: u64,
    /// Total events received
    pub events_received: u64,
    /// Total errors encountered
    pub errors: u64,
    /// Last successful operation timestamp
    pub last_success: Option<std::time::SystemTime>,
    /// Last error timestamp
    pub last_error: Option<std::time::SystemTime>,
    /// Current health status
    pub healthy: bool,
}

impl Default for AdapterStats {
    fn default() -> Self {
        Self {
            events_sent: 0,
            events_received: 0,
            errors: 0,
            last_success: None,
            last_error: None,
            healthy: true,
        }
    }
}

impl AdapterStats {
    /// Record a successful operation
    pub fn record_success(&mut self) {
        self.events_sent += 1;
        self.last_success = Some(std::time::SystemTime::now());
        self.healthy = true;
    }

    /// Record an error
    pub fn record_error(&mut self) {
        self.errors += 1;
        self.last_error = Some(std::time::SystemTime::now());
        self.healthy = false;
    }

    /// Record a received event
    pub fn record_received(&mut self) {
        self.events_received += 1;
    }
}

/// Configuration for adapter connections
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ConnectionConfig {
    /// Connection URL or endpoint
    pub url: String,
    /// Connection timeout
    pub timeout: std::time::Duration,
    /// Maximum retries
    pub max_retries: u32,
    /// Additional configuration as JSON
    pub extra_config: serde_json::Value,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            timeout: std::time::Duration::from_secs(30),
            max_retries: 3,
            extra_config: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

/// Type alias for boxed adapter (useful for dynamic dispatch)
pub type BoxedAdapter = Box<dyn IntegrationAdapterTrait + Send + Sync>;

/// Create a boxed adapter from any IntegrationAdapterTrait implementor
pub fn boxed<A: IntegrationAdapterTrait + Send + Sync + 'static>(adapter: A) -> BoxedAdapter {
    Box::new(adapter)
}
