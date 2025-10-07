//! WebSocket integration adapter for real-time communication

use super::traits::{IntegrationAdapterTrait, AdapterType, AdapterStats, ConnectionConfig, AdapterFeature};
use super::super::events::{IntegrationEvent, IntegrationError};
use async_trait::async_trait;

/// WebSocket adapter for real-time event streaming
#[derive(Debug)]
pub struct WebSocketAdapter {
    /// Configuration
    pub config: ConnectionConfig,
    /// WebSocket URL
    pub ws_url: String,
    /// Subprotocol to use
    pub subprotocol: Option<String>,
    /// Connection state
    connection_state: std::sync::Mutex<WebSocketConnectionState>,
    /// Statistics
    stats: std::sync::Mutex<AdapterStats>,
}

#[derive(Debug, Clone)]
enum WebSocketConnectionState {
    Disconnected,
    Connected {
        // In a real implementation, you'd store the WebSocket connection here
    },
}

impl WebSocketAdapter {
    /// Create a new WebSocket adapter
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            ws_url: config.url.clone(),
            subprotocol: None,
            config,
            connection_state: std::sync::Mutex::new(WebSocketConnectionState::Disconnected),
            stats: std::sync::Mutex::new(AdapterStats::default()),
        }
    }

    /// Set the WebSocket URL explicitly
    pub fn with_ws_url(mut self, url: impl Into<String>) -> Self {
        self.ws_url = url.into();
        self
    }

    /// Set the WebSocket subprotocol
    pub fn with_subprotocol(mut self, protocol: impl Into<String>) -> Self {
        self.subprotocol = Some(protocol.into());
        self
    }

    /// Check if the WebSocket URL is valid
    pub fn validate_ws_url(&self) -> Result<(), IntegrationError> {
        if !self.ws_url.starts_with("ws://") && !self.ws_url.starts_with("wss://") {
            return Err(IntegrationError::ConnectionError(format!(
                "Invalid WebSocket URL: {}. Must start with ws:// or wss://",
                self.ws_url
            )));
        }

        // Additional URL validation could be added here
        Ok(())
    }

    /// Connect to the WebSocket server (placeholder implementation)
    async fn connect(&self) -> Result<(), IntegrationError> {
        // This is a placeholder. In a real implementation, you would:
        // 1. Establish a WebSocket connection using a WebSocket library
        // 2. Handle authentication if required
        // 3. Set up message handling

        self.validate_ws_url()?;

        // For now, just simulate connection
        let mut state = self.connection_state.lock().unwrap();
        *state = WebSocketConnectionState::Connected {};

        Err(IntegrationError::NotImplemented(format!(
            "WebSocket connection to {} not implemented", self.ws_url
        )))
    }

    /// Disconnect from the WebSocket server
    async fn disconnect(&self) -> Result<(), IntegrationError> {
        let mut state = self.connection_state.lock().unwrap();
        *state = WebSocketConnectionState::Disconnected;
        Ok(())
    }

    /// Send a message over WebSocket (placeholder implementation)
    async fn send_ws_message(&self, message: &str) -> Result<(), IntegrationError> {
        let state = self.connection_state.lock().unwrap();
        match *state {
            WebSocketConnectionState::Connected { .. } => {
                // In a real implementation, you would send the message over the WebSocket
                Err(IntegrationError::NotImplemented(format!(
                    "WebSocket message sending not implemented. Message: {}", message
                )))
            }
            WebSocketConnectionState::Disconnected => {
                Err(IntegrationError::ConnectionError("WebSocket is not connected".into()))
            }
        }
    }

    /// Receive messages from WebSocket (placeholder implementation)
    async fn receive_ws_messages(&self) -> Result<Vec<String>, IntegrationError> {
        let state = self.connection_state.lock().unwrap();
        match *state {
            WebSocketConnectionState::Connected { .. } => {
                // In a real implementation, you would receive messages from the WebSocket
                Err(IntegrationError::NotImplemented("WebSocket message receiving not implemented".into()))
            }
            WebSocketConnectionState::Disconnected => {
                Err(IntegrationError::ConnectionError("WebSocket is not connected".into()))
            }
        }
    }

    /// Broadcast an event to all connected clients (placeholder)
    pub async fn broadcast_event(&self, event: &IntegrationEvent) -> Result<(), IntegrationError> {
        let message = serde_json::to_string(event)
            .map_err(|e| IntegrationError::SerializationError(format!("Failed to serialize event: {}", e)))?;

        self.send_ws_message(&message).await
    }

    /// Subscribe to events of a specific type (placeholder)
    pub async fn subscribe_to_events(&self, event_types: &[&str]) -> Result<(), IntegrationError> {
        let subscription_message = serde_json::json!({
            "action": "subscribe",
            "event_types": event_types
        });

        let message = serde_json::to_string(&subscription_message)
            .map_err(|e| IntegrationError::SerializationError(format!("Failed to serialize subscription: {}", e)))?;

        self.send_ws_message(&message).await
    }

    /// Unsubscribe from events (placeholder)
    pub async fn unsubscribe_from_events(&self, event_types: &[&str]) -> Result<(), IntegrationError> {
        let unsubscription_message = serde_json::json!({
            "action": "unsubscribe",
            "event_types": event_types
        });

        let message = serde_json::to_string(&unsubscription_message)
            .map_err(|e| IntegrationError::SerializationError(format!("Failed to serialize unsubscription: {}", e)))?;

        self.send_ws_message(&message).await
    }
}

#[async_trait]
impl IntegrationAdapterTrait for WebSocketAdapter {
    fn adapter_type(&self) -> AdapterType {
        AdapterType::WebSocket
    }

    fn name(&self) -> &str {
        "WebSocket Adapter"
    }

    fn config(&self) -> serde_json::Value {
        serde_json::json!({
            "ws_url": self.ws_url,
            "subprotocol": self.subprotocol,
            "timeout_seconds": self.config.timeout.as_secs(),
            "max_retries": self.config.max_retries,
            "extra_config": self.config.extra_config
        })
    }

    async fn health_check(&self) -> Result<(), IntegrationError> {
        // For WebSocket health check, we could try to establish a connection
        // and immediately close it, or check if the server responds to HTTP requests

        self.validate_ws_url()?;

        // Placeholder: In a real implementation, you might try a quick connection
        // or check an HTTP health endpoint
        Err(IntegrationError::NotImplemented(format!(
            "WebSocket health check for {} not implemented", self.ws_url
        )))
    }

    async fn send_event(&self, event: &IntegrationEvent) -> Result<(), IntegrationError> {
        let result = self.broadcast_event(event).await;

        let mut stats = self.stats.lock().unwrap();
        match result {
            Ok(()) => {
                stats.record_success();
                Ok(())
            }
            Err(e) => {
                stats.record_error();
                Err(e)
            }
        }
    }

    async fn receive_events(&self) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        let messages = self.receive_ws_messages().await?;

        let mut events = Vec::new();
        let mut stats = self.stats.lock().unwrap();

        for message in messages {
            match serde_json::from_str::<IntegrationEvent>(&message) {
                Ok(event) => {
                    stats.record_received();
                    events.push(event);
                }
                Err(e) => {
                    stats.record_error();
                    // Log error but continue processing other messages
                    eprintln!("Failed to deserialize WebSocket message: {}", e);
                }
            }
        }

        Ok(events)
    }

    fn stats(&self) -> AdapterStats {
        self.stats.lock().unwrap().clone()
    }

    fn supports_feature(&self, feature: AdapterFeature) -> bool {
        match feature {
            AdapterFeature::SendEvents => true,
            AdapterFeature::ReceiveEvents => true,
            AdapterFeature::HealthCheck => true,
            AdapterFeature::Batching => true, // WebSocket can batch messages
            AdapterFeature::RetryLogic => true, // Connection management includes retries
            AdapterFeature::Authentication => true,
        }
    }
}

impl Drop for WebSocketAdapter {
    fn drop(&mut self) {
        // In a real implementation, you would cleanly close the WebSocket connection
        // For now, we just mark as disconnected
        let mut state = self.connection_state.lock().unwrap();
        *state = WebSocketConnectionState::Disconnected;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_websocket_adapter_creation() {
        let config = ConnectionConfig {
            url: "ws://localhost:8080/events".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            extra_config: serde_json::Value::Null,
        };

        let adapter = WebSocketAdapter::new(config);
        assert_eq!(adapter.adapter_type(), AdapterType::WebSocket);
        assert_eq!(adapter.name(), "WebSocket Adapter");
        assert_eq!(adapter.ws_url, "ws://localhost:8080/events");
        assert!(adapter.subprotocol.is_none());
    }

    #[test]
    fn test_websocket_url_validation() {
        let valid_config = ConnectionConfig {
            url: "wss://secure.example.com/events".to_string(),
            ..Default::default()
        };
        let adapter = WebSocketAdapter::new(valid_config);
        assert!(adapter.validate_ws_url().is_ok());

        let invalid_config = ConnectionConfig {
            url: "http://example.com/events".to_string(),
            ..Default::default()
        };
        let adapter = WebSocketAdapter::new(invalid_config);
        assert!(matches!(adapter.validate_ws_url(), Err(IntegrationError::ConnectionError(_))));
    }

    #[test]
    fn test_configuration_methods() {
        let config = ConnectionConfig::default();
        let adapter = WebSocketAdapter::new(config)
            .with_ws_url("wss://custom.example.com/ws")
            .with_subprotocol("integration-protocol");

        assert_eq!(adapter.ws_url, "wss://custom.example.com/ws");
        assert_eq!(adapter.subprotocol, Some("integration-protocol".to_string()));
    }

    #[test]
    fn test_feature_support() {
        let adapter = WebSocketAdapter::new(ConnectionConfig::default());

        assert!(adapter.supports_feature(AdapterFeature::SendEvents));
        assert!(adapter.supports_feature(AdapterFeature::ReceiveEvents));
        assert!(adapter.supports_feature(AdapterFeature::HealthCheck));
        assert!(adapter.supports_feature(AdapterFeature::Batching));
        assert!(adapter.supports_feature(AdapterFeature::RetryLogic));
        assert!(adapter.supports_feature(AdapterFeature::Authentication));
    }

    #[test]
    fn test_config_serialization() {
        let config = ConnectionConfig {
            url: "ws://localhost:8080".to_string(),
            timeout: Duration::from_secs(60),
            max_retries: 5,
            extra_config: serde_json::json!({"heartbeat_interval": 30}),
        };

        let adapter = WebSocketAdapter::new(config)
            .with_subprotocol("json-rpc");

        let config_json = adapter.config();
        assert_eq!(config_json["ws_url"], "ws://localhost:8080");
        assert_eq!(config_json["subprotocol"], "json-rpc");
        assert_eq!(config_json["timeout_seconds"], 60);
        assert_eq!(config_json["max_retries"], 5);
        assert_eq!(config_json["extra_config"]["heartbeat_interval"], 30);
    }

    #[test]
    fn test_connection_state() {
        let config = ConnectionConfig::default();
        let adapter = WebSocketAdapter::new(config);

        // Initially disconnected
        let state = adapter.connection_state.lock().unwrap();
        assert!(matches!(*state, WebSocketConnectionState::Disconnected));
    }

    #[tokio::test]
    async fn test_broadcast_event_message_format() {
        let config = ConnectionConfig::default();
        let adapter = WebSocketAdapter::new(config);

        let event = IntegrationEvent {
            id: "test-123".to_string(),
            timestamp: 1640995200,
            event_type: "user.created".to_string(),
            data: serde_json::json!({"user_id": 123}),
            correlation_id: None,
            metadata: std::collections::HashMap::new(),
        };

        // Test that the event can be serialized (even though sending will fail)
        let result = serde_json::to_string(&event);
        assert!(result.is_ok());

        // The actual broadcast will fail with NotImplemented
        let broadcast_result = adapter.broadcast_event(&event).await;
        assert!(matches!(broadcast_result, Err(IntegrationError::NotImplemented(_))));
    }
}
