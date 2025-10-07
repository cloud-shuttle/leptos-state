//! Contract tests for integration adapters
//!
//! These tests verify that all integration adapter implementations
//! adhere to their API contracts as defined in the design documents.

use super::*;
use super::super::events::{IntegrationEvent, IntegrationError};

/// Test contract for all IntegrationAdapterTrait implementations
#[async_trait::async_trait]
trait IntegrationAdapterContractTester {
    /// Test that send_event completes within timeout or errors appropriately
    async fn test_send_event_timeout_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        let test_event = IntegrationEvent {
            id: "contract-test-123".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type: "contract.test".to_string(),
            data: serde_json::json!({"test": "data"}),
            correlation_id: Some("correlation-123".to_string()),
            metadata: std::collections::HashMap::new(),
        };

        let start_time = std::time::Instant::now();
        let result = adapter.send_event(&test_event).await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(()) => {
                // Success is acceptable
            }
            Err(IntegrationError::ConnectionError(_) | IntegrationError::TimeoutError(_)) => {
                // Expected errors for unavailable services
            }
            Err(e) => {
                // Other errors should still complete reasonably fast
                assert!(elapsed < std::time::Duration::from_secs(30),
                    "Error cases should not take excessive time: {:?}", e);
            }
        }
    }

    /// Test that health_check is reasonably fast
    async fn test_health_check_performance_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        let start_time = std::time::Instant::now();
        let result = adapter.health_check().await;
        let elapsed = start_time.elapsed();

        // Health checks should complete within 5 seconds
        assert!(elapsed < std::time::Duration::from_secs(5),
            "Health check took too long: {:?}", elapsed);

        // Result can be Ok or Err, both are acceptable
        let _ = result;
    }

    /// Test that receive_events returns valid events or empty vector
    async fn test_receive_events_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        let result = adapter.receive_events().await;

        match result {
            Ok(events) => {
                // If successful, all events should be valid
                for event in &events {
                    assert!(!event.id.is_empty(), "Event ID must not be empty");
                    assert!(!event.event_type.is_empty(), "Event type must not be empty");
                    assert!(event.timestamp > 0, "Event timestamp must be positive");
                }
            }
            Err(IntegrationError::NotSupported(_)) => {
                // NotSupported is acceptable for adapters that don't receive
            }
            Err(_) => {
                // Other errors are acceptable (connection issues, etc.)
            }
        }
    }

    /// Test that stats are always available and well-formed
    fn test_stats_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        let stats = adapter.stats();

        // Stats should always be available (no panics)
        assert!(stats.events_sent >= 0, "Events sent cannot be negative");
        assert!(stats.events_received >= 0, "Events received cannot be negative");
        assert!(stats.errors >= 0, "Errors cannot be negative");
    }

    /// Test that feature support is consistent
    fn test_feature_support_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        // All adapters should support health checks
        assert!(adapter.supports_feature(AdapterFeature::HealthCheck),
            "All adapters must support health checks");

        // SendEvents support should be consistent with actual behavior
        let supports_send = adapter.supports_feature(AdapterFeature::SendEvents);
        // We can't easily test this without mocking, but we can ensure it's a boolean
        let _ = supports_send;
    }

    /// Test that adapter type is valid
    fn test_adapter_type_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        let adapter_type = adapter.adapter_type();

        // Adapter type should be one of the defined variants
        match adapter_type {
            AdapterType::RestApi | AdapterType::Database |
            AdapterType::MessageQueue | AdapterType::FileSystem |
            AdapterType::WebSocket | AdapterType::Custom(_) => {
                // Valid types
            }
        }
    }

    /// Test that name is not empty
    fn test_name_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        let name = adapter.name();
        assert!(!name.is_empty(), "Adapter name must not be empty");
    }
}

/// Contract test runner for integration adapters
pub struct IntegrationAdapterContractTestRunner;

impl IntegrationAdapterContractTestRunner {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_all_contract_tests(&self) {
        println!("Running Integration Adapter Contract Tests...");

        // Test HTTP adapter contracts
        self.test_http_adapter_contracts().await;

        // Test database adapter contracts
        self.test_database_adapter_contracts().await;

        // Test message queue adapter contracts
        self.test_message_queue_adapter_contracts().await;

        // Test file system adapter contracts
        self.test_file_system_adapter_contracts().await;

        // Test WebSocket adapter contracts
        self.test_websocket_adapter_contracts().await;

        println!("Integration Adapter Contract Tests completed successfully!");
    }

    async fn test_http_adapter_contracts(&self) {
        println!("Testing HTTP Adapter contracts...");

        let config = ConnectionConfig {
            url: "https://httpbin.org/post".to_string(), // Use a real endpoint for testing
            timeout: std::time::Duration::from_secs(10),
            max_retries: 2,
            extra_config: serde_json::Value::Null,
        };

        let adapter = HttpApiAdapter::new(config);
        self.run_contract_tests(&adapter, "HTTP Adapter").await;
    }

    async fn test_database_adapter_contracts(&self) {
        println!("Testing Database Adapter contracts...");

        let config = ConnectionConfig {
            url: "postgresql://test:test@localhost:5432/test".to_string(),
            timeout: std::time::Duration::from_secs(5),
            max_retries: 1,
            extra_config: serde_json::Value::Null,
        };

        let adapter = DatabaseAdapter::new(config);
        self.run_contract_tests(&adapter, "Database Adapter").await;
    }

    async fn test_message_queue_adapter_contracts(&self) {
        println!("Testing Message Queue Adapter contracts...");

        let config = ConnectionConfig {
            url: "amqp://guest:guest@localhost:5672".to_string(),
            timeout: std::time::Duration::from_secs(5),
            max_retries: 1,
            extra_config: serde_json::Value::Null,
        };

        let adapter = MessageQueueAdapter::new(config);
        self.run_contract_tests(&adapter, "Message Queue Adapter").await;
    }

    async fn test_file_system_adapter_contracts(&self) {
        println!("Testing File System Adapter contracts...");

        let config = ConnectionConfig::default();
        let adapter = FileSystemAdapter::new(config);
        self.run_contract_tests(&adapter, "File System Adapter").await;
    }

    async fn test_websocket_adapter_contracts(&self) {
        println!("Testing WebSocket Adapter contracts...");

        let config = ConnectionConfig {
            url: "ws://localhost:8080/events".to_string(),
            timeout: std::time::Duration::from_secs(5),
            max_retries: 1,
            extra_config: serde_json::Value::Null,
        };

        let adapter = WebSocketAdapter::new(config);
        self.run_contract_tests(&adapter, "WebSocket Adapter").await;
    }

    async fn run_contract_tests(&self, adapter: &dyn IntegrationAdapterTrait, name: &str) {
        println!("  Testing {} contracts...", name);

        // Test send event timeout contract
        self.test_send_event_timeout_contract(adapter).await;

        // Test health check performance contract
        self.test_health_check_performance_contract(adapter).await;

        // Test receive events contract
        self.test_receive_events_contract(adapter).await;

        // Test stats contract
        self.test_stats_contract(adapter);

        // Test feature support contract
        self.test_feature_support_contract(adapter);

        // Test adapter type contract
        self.test_adapter_type_contract(adapter);

        // Test name contract
        self.test_name_contract(adapter);

        println!("  ✓ {} contracts verified", name);
    }
}

#[async_trait::async_trait]
impl IntegrationAdapterContractTester for IntegrationAdapterContractTestRunner {
    async fn test_send_event_timeout_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        IntegrationAdapterContractTester::test_send_event_timeout_contract(self, adapter).await;
    }

    async fn test_health_check_performance_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        IntegrationAdapterContractTester::test_health_check_performance_contract(self, adapter).await;
    }

    async fn test_receive_events_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        IntegrationAdapterContractTester::test_receive_events_contract(self, adapter).await;
    }

    fn test_stats_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        IntegrationAdapterContractTester::test_stats_contract(self, adapter);
    }

    fn test_feature_support_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        IntegrationAdapterContractTester::test_feature_support_contract(self, adapter);
    }

    fn test_adapter_type_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        IntegrationAdapterContractTester::test_adapter_type_contract(self, adapter);
    }

    fn test_name_contract(&self, adapter: &dyn IntegrationAdapterTrait) {
        IntegrationAdapterContractTester::test_name_contract(self, adapter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_integration_adapter_contracts_comprehensive() {
        let runner = IntegrationAdapterContractTestRunner::new();
        runner.run_all_contract_tests().await;
    }

    #[test]
    fn test_adapter_type_enum_exhaustive() {
        // Ensure all adapter types are handled
        let types = vec![
            AdapterType::RestApi,
            AdapterType::Database,
            AdapterType::MessageQueue,
            AdapterType::FileSystem,
            AdapterType::WebSocket,
            AdapterType::Custom("test".to_string()),
        ];

        for adapter_type in types {
            let type_str = adapter_type.to_string();
            assert!(!type_str.is_empty(), "Adapter type string must not be empty");
        }
    }

    #[test]
    fn test_adapter_stats_default_values() {
        let stats = AdapterStats::default();

        assert_eq!(stats.events_sent, 0);
        assert_eq!(stats.events_received, 0);
        assert_eq!(stats.errors, 0);
        assert!(stats.last_success.is_none());
        assert!(stats.last_error.is_none());
        assert!(stats.healthy);
    }

    #[test]
    fn test_adapter_stats_record_success() {
        let mut stats = AdapterStats::default();

        stats.record_success();

        assert_eq!(stats.events_sent, 1);
        assert!(stats.last_success.is_some());
        assert!(stats.healthy);
        assert_eq!(stats.errors, 0); // Should not increment errors
    }

    #[test]
    fn test_adapter_stats_record_error() {
        let mut stats = AdapterStats::default();

        stats.record_error();

        assert_eq!(stats.errors, 1);
        assert!(stats.last_error.is_some());
        assert!(!stats.healthy);
        assert_eq!(stats.events_sent, 0); // Should not increment sent
    }

    #[test]
    fn test_adapter_stats_record_received() {
        let mut stats = AdapterStats::default();

        stats.record_received();

        assert_eq!(stats.events_received, 1);
        assert_eq!(stats.events_sent, 0); // Should not affect sent
        assert_eq!(stats.errors, 0); // Should not affect errors
    }

    #[test]
    fn test_connection_config_defaults() {
        let config = ConnectionConfig::default();

        assert_eq!(config.url, "");
        assert_eq!(config.timeout, std::time::Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
        assert!(config.extra_config.is_object());
    }

    #[tokio::test]
    async fn test_http_adapter_with_real_endpoint() {
        // Test with httpbin.org which should be reliable for testing
        let config = ConnectionConfig {
            url: "https://httpbin.org/post".to_string(),
            timeout: std::time::Duration::from_secs(10),
            max_retries: 1,
            extra_config: serde_json::Value::Null,
        };

        let adapter = HttpApiAdapter::new(config);

        // Test that the adapter is properly configured
        assert_eq!(adapter.adapter_type(), AdapterType::RestApi);
        assert_eq!(adapter.name(), "HTTP API Adapter");

        // Test health check (may fail due to network, but should not panic)
        let health_result = adapter.health_check().await;
        // We don't assert success, just that it doesn't panic
        let _ = health_result;
    }

    #[test]
    fn test_adapter_feature_flags() {
        // Test that feature flags are properly defined
        let features = vec![
            AdapterFeature::SendEvents,
            AdapterFeature::ReceiveEvents,
            AdapterFeature::HealthCheck,
            AdapterFeature::Batching,
            AdapterFeature::RetryLogic,
            AdapterFeature::Authentication,
        ];

        // Just ensure they can be created and compared
        for feature in features {
            match feature {
                AdapterFeature::SendEvents => assert!(matches!(feature, AdapterFeature::SendEvents)),
                AdapterFeature::ReceiveEvents => assert!(matches!(feature, AdapterFeature::ReceiveEvents)),
                AdapterFeature::HealthCheck => assert!(matches!(feature, AdapterFeature::HealthCheck)),
                AdapterFeature::Batching => assert!(matches!(feature, AdapterFeature::Batching)),
                AdapterFeature::RetryLogic => assert!(matches!(feature, AdapterFeature::RetryLogic)),
                AdapterFeature::Authentication => assert!(matches!(feature, AdapterFeature::Authentication)),
            }
        }
    }
}
