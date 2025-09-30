//! Integration adapters for connecting to external systems

use crate::machine::integration::events::IntegrationEvent;
use super::health::HealthStatus;

/// Integration adapter trait for external systems
#[async_trait::async_trait]
pub trait IntegrationAdapterTrait: Send + Sync {
    /// Get the adapter type
    fn adapter_type(&self) -> AdapterType;

    /// Get the adapter name
    fn name(&self) -> &str;

    /// Send an event to the external system
    async fn send_event(&self, event: &IntegrationEvent) -> Result<(), String>;

    /// Receive events from the external system
    async fn receive_events(&self) -> Result<Vec<IntegrationEvent>, String>;

    /// Perform a health check
    async fn health_check(&self) -> HealthStatus;

    /// Get adapter configuration
    fn config(&self) -> serde_json::Value;

    /// Check if the adapter is connected
    fn is_connected(&self) -> bool {
        // Note: This is a synchronous check. For async health checks,
        // use health_check().await and check the result.
        // Default implementation assumes connected unless overridden.
        true
    }

    /// Get connection information
    fn connection_info(&self) -> String {
        format!("{} ({})", self.name(), self.adapter_type().as_str())
    }

    /// Clone the adapter
    fn clone_adapter(&self) -> Box<dyn IntegrationAdapterTrait + Send + Sync> {
        // This is a placeholder - real implementations should override this
        panic!("clone_adapter not implemented for this adapter type")
    }
}

/// Types of integration adapters
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdapterType {
    /// REST API adapter
    RestApi,
    /// WebSocket adapter
    WebSocket,
    /// Message queue (e.g., RabbitMQ, Kafka)
    MessageQueue,
    /// Database adapter
    Database,
    /// File system adapter
    FileSystem,
    /// Email adapter
    Email,
    /// SMS adapter
    Sms,
    /// Push notification adapter
    PushNotification,
    /// Custom adapter
    Custom(String),
}

impl AdapterType {
    /// Get string representation
    pub fn as_str(&self) -> &str {
        match self {
            Self::RestApi => "rest_api",
            Self::WebSocket => "websocket",
            Self::MessageQueue => "message_queue",
            Self::Database => "database",
            Self::FileSystem => "filesystem",
            Self::Email => "email",
            Self::Sms => "sms",
            Self::PushNotification => "push_notification",
            Self::Custom(ref s) => s,
        }
    }

    /// Check if adapter type requires persistent connection
    pub fn requires_persistent_connection(&self) -> bool {
        matches!(self, Self::WebSocket | Self::MessageQueue)
    }

    /// Check if adapter type supports bidirectional communication
    pub fn supports_bidirectional(&self) -> bool {
        matches!(self, Self::WebSocket | Self::MessageQueue)
    }

    /// Check if adapter type is for external services
    pub fn is_external_service(&self) -> bool {
        !matches!(self, Self::FileSystem | Self::Database)
    }

    /// Get all adapter types
    pub fn all() -> Vec<Self> {
        vec![
            Self::RestApi,
            Self::WebSocket,
            Self::MessageQueue,
            Self::Database,
            Self::FileSystem,
            Self::Email,
            Self::Sms,
            Self::PushNotification,
        ]
    }
}

impl std::fmt::Display for AdapterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for AdapterType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rest_api" | "rest" => Ok(Self::RestApi),
            "websocket" | "ws" => Ok(Self::WebSocket),
            "message_queue" | "mq" | "queue" => Ok(Self::MessageQueue),
            "database" | "db" => Ok(Self::Database),
            "filesystem" | "fs" => Ok(Self::FileSystem),
            "email" => Ok(Self::Email),
            "sms" => Ok(Self::Sms),
            "push_notification" | "push" => Ok(Self::PushNotification),
            custom => Ok(Self::Custom(custom.to_string())),
        }
    }
}

/// Integration adapter for external systems
pub struct IntegrationAdapter {
    /// Adapter name
    name: String,
    /// Adapter type
    adapter_type: AdapterType,
    /// Configuration
    config: serde_json::Value,
    /// Connection status
    connected: std::sync::atomic::AtomicBool,
    /// Last health check time
    last_health_check: std::sync::Mutex<Option<std::time::SystemTime>>,
}

impl IntegrationAdapter {
    /// Create a new integration adapter
    pub fn new(name: String, adapter_type: AdapterType, config: serde_json::Value) -> Self {
        Self {
            name,
            adapter_type,
            config,
            connected: std::sync::atomic::AtomicBool::new(false),
            last_health_check: std::sync::Mutex::new(None),
        }
    }

    /// Get adapter name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get adapter type
    pub fn adapter_type(&self) -> &AdapterType {
        &self.adapter_type
    }

    /// Get configuration
    pub fn config(&self) -> &serde_json::Value {
        &self.config
    }

    /// Set connection status
    pub fn set_connected(&self, connected: bool) {
        self.connected.store(connected, std::sync::atomic::Ordering::Relaxed);
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.connected.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Update last health check time
    pub fn update_health_check_time(&self) {
        *self.last_health_check.lock().unwrap() = Some(std::time::SystemTime::now());
    }

    /// Get last health check time
    pub fn last_health_check_time(&self) -> Option<std::time::SystemTime> {
        *self.last_health_check.lock().unwrap()
    }

    /// Get time since last health check
    pub fn time_since_health_check(&self) -> Option<std::time::Duration> {
        self.last_health_check_time()
            .and_then(|time| std::time::SystemTime::now().duration_since(time).ok())
    }

    /// Create a REST API adapter
    pub fn rest_api(name: String, base_url: String, api_key: Option<String>) -> Self {
        let mut config = serde_json::json!({
            "base_url": base_url,
        });

        if let Some(key) = api_key {
            config["api_key"] = serde_json::Value::String(key);
        }

        Self::new(name, AdapterType::RestApi, config)
    }

    /// Create a WebSocket adapter
    pub fn websocket(name: String, url: String, reconnect_interval: Option<u64>) -> Self {
        let mut config = serde_json::json!({
            "url": url,
        });

        if let Some(interval) = reconnect_interval {
            config["reconnect_interval"] = serde_json::Value::Number(interval.into());
        }

        Self::new(name, AdapterType::WebSocket, config)
    }

    /// Create a message queue adapter
    pub fn message_queue(name: String, queue_url: String, queue_name: String) -> Self {
        let config = serde_json::json!({
            "queue_url": queue_url,
            "queue_name": queue_name,
        });

        Self::new(name, AdapterType::MessageQueue, config)
    }

    /// Create a database adapter
    pub fn database(name: String, connection_string: String, table_name: String) -> Self {
        let config = serde_json::json!({
            "connection_string": connection_string,
            "table_name": table_name,
        });

        Self::new(name, AdapterType::Database, config)
    }

    /// Create a filesystem adapter
    pub fn filesystem(name: String, directory: String, file_pattern: Option<String>) -> Self {
        let mut config = serde_json::json!({
            "directory": directory,
        });

        if let Some(pattern) = file_pattern {
            config["file_pattern"] = serde_json::Value::String(pattern);
        }

        Self::new(name, AdapterType::FileSystem, config)
    }

    /// Create an email adapter
    pub fn email(name: String, smtp_server: String, smtp_port: u16, credentials: Option<(String, String)>) -> Self {
        let mut config = serde_json::json!({
            "smtp_server": smtp_server,
            "smtp_port": smtp_port,
        });

        if let Some((username, password)) = credentials {
            config["username"] = serde_json::Value::String(username);
            config["password"] = serde_json::Value::String(password);
        }

        Self::new(name, AdapterType::Email, config)
    }
}

#[async_trait::async_trait]
impl IntegrationAdapterTrait for IntegrationAdapter {
    fn adapter_type(&self) -> AdapterType {
        self.adapter_type.clone()
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn send_event(&self, _event: &IntegrationEvent) -> Result<(), String> {
        // Placeholder implementation
        // Real implementation would send the event based on adapter type
        if self.is_connected() {
            Ok(())
        } else {
            Err("Adapter not connected".to_string())
        }
    }

    async fn receive_events(&self) -> Result<Vec<IntegrationEvent>, String> {
        // Placeholder implementation
        // Real implementation would receive events based on adapter type
        if self.is_connected() {
            Ok(Vec::new())
        } else {
            Err("Adapter not connected".to_string())
        }
    }

    async fn health_check(&self) -> HealthStatus {
        self.update_health_check_time();

        // Placeholder health check
        // Real implementation would perform actual health checks
        if self.is_connected() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        }
    }

    fn config(&self) -> serde_json::Value {
        self.config.clone()
    }

    fn clone_adapter(&self) -> Box<dyn IntegrationAdapterTrait + Send + Sync> {
        Box::new(Self {
            name: self.name.clone(),
            adapter_type: self.adapter_type.clone(),
            config: self.config.clone(),
            connected: std::sync::atomic::AtomicBool::new(self.is_connected()),
            last_health_check: std::sync::Mutex::new(*self.last_health_check.lock().unwrap()),
        })
    }
}

impl std::fmt::Debug for IntegrationAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntegrationAdapter")
            .field("name", &self.name)
            .field("adapter_type", &self.adapter_type)
            .field("connected", &self.is_connected())
            .finish()
    }
}

impl std::fmt::Display for IntegrationAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.adapter_type)
    }
}

/// Adapter factory for creating adapters from configuration
pub struct AdapterFactory;

impl AdapterFactory {
    /// Create an adapter from configuration
    pub fn create_from_config(name: String, adapter_type: AdapterType, config: serde_json::Value) -> IntegrationAdapter {
        IntegrationAdapter::new(name, adapter_type, config)
    }

    /// Create a REST API adapter from config
    pub fn create_rest_api(name: String, config: serde_json::Value) -> Result<IntegrationAdapter, String> {
        if let Some(base_url) = config.get("base_url").and_then(|v| v.as_str()) {
            let api_key = config.get("api_key").and_then(|v| v.as_str()).map(|s| s.to_string());
            Ok(IntegrationAdapter::rest_api(name, base_url.to_string(), api_key))
        } else {
            Err("Missing base_url in REST API config".to_string())
        }
    }

    /// Create a WebSocket adapter from config
    pub fn create_websocket(name: String, config: serde_json::Value) -> Result<IntegrationAdapter, String> {
        if let Some(url) = config.get("url").and_then(|v| v.as_str()) {
            let reconnect_interval = config.get("reconnect_interval").and_then(|v| v.as_u64());
            Ok(IntegrationAdapter::websocket(name, url.to_string(), reconnect_interval))
        } else {
            Err("Missing url in WebSocket config".to_string())
        }
    }
}
