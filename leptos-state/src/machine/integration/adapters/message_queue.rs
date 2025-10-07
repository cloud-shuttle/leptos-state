//! Message queue integration adapter

use super::traits::{IntegrationAdapterTrait, AdapterType, AdapterStats, ConnectionConfig, AdapterFeature};
use super::super::events::{IntegrationEvent, IntegrationError};
use async_trait::async_trait;

/// Message queue adapter for pub/sub messaging
#[derive(Debug)]
pub struct MessageQueueAdapter {
    /// Configuration
    pub config: ConnectionConfig,
    /// Queue/topic name
    pub queue_name: String,
    /// Message routing keys/topics
    pub routing_keys: std::collections::HashMap<String, String>,
    /// Statistics
    stats: std::sync::Mutex<AdapterStats>,
}

impl MessageQueueAdapter {
    /// Create a new message queue adapter
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            queue_name: "integration_events".to_string(),
            config,
            routing_keys: std::collections::HashMap::new(),
            stats: std::sync::Mutex::new(AdapterStats::default()),
        }
    }

    /// Set the queue name
    pub fn with_queue_name(mut self, queue_name: impl Into<String>) -> Self {
        self.queue_name = queue_name.into();
        self
    }

    /// Add a routing key for a specific event type
    pub fn add_routing_key(&mut self, event_type: impl Into<String>, routing_key: impl Into<String>) {
        self.routing_keys.insert(event_type.into(), routing_key.into());
    }

    /// Remove a routing key
    pub fn remove_routing_key(&mut self, event_type: &str) {
        self.routing_keys.remove(event_type);
    }

    /// Get routing key for event type, falling back to event type itself
    pub fn get_routing_key(&self, event_type: &str) -> String {
        self.routing_keys
            .get(event_type)
            .cloned()
            .unwrap_or_else(|| event_type.to_string())
    }

    /// Get the message queue type from connection string
    pub fn queue_type(&self) -> &str {
        if self.config.url.contains("amqp://") || self.config.url.contains("rabbitmq://") {
            "rabbitmq"
        } else if self.config.url.contains("kafka://") {
            "kafka"
        } else if self.config.url.contains("redis://") {
            "redis"
        } else if self.config.url.contains("sqs://") {
            "sqs"
        } else {
            "unknown"
        }
    }

    /// Publish a message to the queue (placeholder implementation)
    async fn publish_message(&self, routing_key: &str, message: &str) -> Result<(), IntegrationError> {
        // This is a placeholder. In a real implementation, you would:
        // 1. Connect to the message queue system
        // 2. Publish the message with the routing key
        // 3. Handle acknowledgments

        match self.queue_type() {
            "rabbitmq" => {
                Err(IntegrationError::NotImplemented(format!(
                    "RabbitMQ publishing not implemented. Key: {}, Message: {}", routing_key, message
                )))
            }
            "kafka" => {
                Err(IntegrationError::NotImplemented(format!(
                    "Kafka publishing not implemented. Key: {}, Message: {}", routing_key, message
                )))
            }
            "redis" => {
                Err(IntegrationError::NotImplemented(format!(
                    "Redis publishing not implemented. Key: {}, Message: {}", routing_key, message
                )))
            }
            "sqs" => {
                Err(IntegrationError::NotImplemented(format!(
                    "SQS publishing not implemented. Key: {}, Message: {}", routing_key, message
                )))
            }
            _ => {
                Err(IntegrationError::ConnectionError(format!(
                    "Unsupported queue type: {}. Key: {}, Message: {}", self.queue_type(), routing_key, message
                )))
            }
        }
    }

    /// Consume messages from the queue (placeholder implementation)
    async fn consume_messages(&self, max_messages: usize) -> Result<Vec<(String, String)>, IntegrationError> {
        // This is a placeholder. In a real implementation, you would:
        // 1. Connect to the message queue system
        // 2. Consume messages from the queue
        // 3. Return message data

        match self.queue_type() {
            "rabbitmq" => {
                Err(IntegrationError::NotImplemented("RabbitMQ consuming not implemented".into()))
            }
            "kafka" => {
                Err(IntegrationError::NotImplemented("Kafka consuming not implemented".into()))
            }
            "redis" => {
                Err(IntegrationError::NotImplemented("Redis consuming not implemented".into()))
            }
            "sqs" => {
                Err(IntegrationError::NotImplemented("SQS consuming not implemented".into()))
            }
            _ => {
                Err(IntegrationError::ConnectionError(format!(
                    "Unsupported queue type: {}", self.queue_type()
                )))
            }
        }
    }
}

#[async_trait]
impl IntegrationAdapterTrait for MessageQueueAdapter {
    fn adapter_type(&self) -> AdapterType {
        AdapterType::MessageQueue
    }

    fn name(&self) -> &str {
        "Message Queue Adapter"
    }

    fn config(&self) -> serde_json::Value {
        serde_json::json!({
            "connection_url": self.config.url,
            "timeout_seconds": self.config.timeout.as_secs(),
            "max_retries": self.config.max_retries,
            "queue_name": self.queue_name,
            "routing_keys": self.routing_keys,
            "queue_type": self.queue_type(),
            "extra_config": self.config.extra_config
        })
    }

    async fn health_check(&self) -> Result<(), IntegrationError> {
        // Placeholder health check
        // In a real implementation, you would attempt to connect to the message queue

        if self.config.url.trim().is_empty() {
            return Err(IntegrationError::ConnectionError("Queue URL is empty".into()));
        }

        match self.queue_type() {
            "rabbitmq" | "kafka" | "redis" | "sqs" => {
                // In a real implementation, we would try to establish a connection
                // For now, just validate the connection string format
                Err(IntegrationError::NotImplemented(format!(
                    "{} health check not implemented", self.queue_type()
                )))
            }
            _ => {
                Err(IntegrationError::ConnectionError(format!(
                    "Unsupported queue type: {}", self.queue_type()
                )))
            }
        }
    }

    async fn send_event(&self, event: &IntegrationEvent) -> Result<(), IntegrationError> {
        let routing_key = self.get_routing_key(&event.event_type);

        let message = serde_json::to_string(event)
            .map_err(|e| IntegrationError::SerializationError(format!("Failed to serialize event: {}", e)))?;

        let result = self.publish_message(&routing_key, &message).await;

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
        let messages = self.consume_messages(100).await?;

        let mut events = Vec::new();
        let mut stats = self.stats.lock().unwrap();

        for (routing_key, message) in messages {
            match serde_json::from_str::<IntegrationEvent>(&message) {
                Ok(event) => {
                    stats.record_received();
                    events.push(event);
                }
                Err(e) => {
                    stats.record_error();
                    // Log error but continue processing other messages
                    eprintln!("Failed to deserialize message from {}: {}", routing_key, e);
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
            AdapterFeature::Batching => true, // Message queues naturally support batching
            AdapterFeature::RetryLogic => true, // Most MQ systems have built-in retry
            AdapterFeature::Authentication => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_message_queue_adapter_creation() {
        let config = ConnectionConfig {
            url: "amqp://localhost:5672".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            extra_config: serde_json::Value::Null,
        };

        let adapter = MessageQueueAdapter::new(config);
        assert_eq!(adapter.adapter_type(), AdapterType::MessageQueue);
        assert_eq!(adapter.name(), "Message Queue Adapter");
        assert_eq!(adapter.queue_type(), "rabbitmq");
        assert_eq!(adapter.queue_name, "integration_events");
    }

    #[test]
    fn test_queue_type_detection() {
        let rabbitmq_config = ConnectionConfig {
            url: "amqp://localhost:5672".to_string(),
            ..Default::default()
        };
        let adapter = MessageQueueAdapter::new(rabbitmq_config);
        assert_eq!(adapter.queue_type(), "rabbitmq");

        let kafka_config = ConnectionConfig {
            url: "kafka://localhost:9092".to_string(),
            ..Default::default()
        };
        let adapter = MessageQueueAdapter::new(kafka_config);
        assert_eq!(adapter.queue_type(), "kafka");

        let redis_config = ConnectionConfig {
            url: "redis://localhost:6379".to_string(),
            ..Default::default()
        };
        let adapter = MessageQueueAdapter::new(redis_config);
        assert_eq!(adapter.queue_type(), "redis");

        let sqs_config = ConnectionConfig {
            url: "sqs://us-east-1".to_string(),
            ..Default::default()
        };
        let adapter = MessageQueueAdapter::new(sqs_config);
        assert_eq!(adapter.queue_type(), "sqs");

        let unknown_config = ConnectionConfig {
            url: "unknown://localhost".to_string(),
            ..Default::default()
        };
        let adapter = MessageQueueAdapter::new(unknown_config);
        assert_eq!(adapter.queue_type(), "unknown");
    }

    #[test]
    fn test_routing_key_configuration() {
        let config = ConnectionConfig::default();
        let mut adapter = MessageQueueAdapter::new(config);

        adapter.add_routing_key("user.created", "user.events");
        adapter.add_routing_key("order.placed", "order.events");

        assert_eq!(adapter.get_routing_key("user.created"), "user.events");
        assert_eq!(adapter.get_routing_key("order.placed"), "order.events");
        assert_eq!(adapter.get_routing_key("unknown.event"), "unknown.event"); // fallback
    }

    #[test]
    fn test_queue_name_configuration() {
        let config = ConnectionConfig::default();
        let adapter = MessageQueueAdapter::new(config).with_queue_name("custom_queue");
        assert_eq!(adapter.queue_name, "custom_queue");
    }

    #[test]
    fn test_feature_support() {
        let adapter = MessageQueueAdapter::new(ConnectionConfig::default());

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
            url: "amqp://guest:guest@localhost:5672/%2f".to_string(),
            timeout: Duration::from_secs(45),
            max_retries: 5,
            extra_config: serde_json::json!({"vhost": "/"}),
        };

        let mut adapter = MessageQueueAdapter::new(config).with_queue_name("events");
        adapter.add_routing_key("test", "test.routing");

        let config_json = adapter.config();
        assert_eq!(config_json["connection_url"], "amqp://guest:guest@localhost:5672/%2f");
        assert_eq!(config_json["timeout_seconds"], 45);
        assert_eq!(config_json["max_retries"], 5);
        assert_eq!(config_json["queue_name"], "events");
        assert_eq!(config_json["routing_keys"]["test"], "test.routing");
        assert_eq!(config_json["queue_type"], "rabbitmq");
        assert_eq!(config_json["extra_config"]["vhost"], "/");
    }
}
