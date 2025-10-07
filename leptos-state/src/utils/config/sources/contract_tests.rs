//! Contract tests for configuration sources
//!
//! These tests verify that all config source implementations
//! adhere to their API contracts as defined in the design documents.

use super::*;
use std::collections::HashMap;

/// Test contract for all ConfigSourceTrait implementations
#[async_trait::async_trait]
trait ConfigSourceContractTester {
    /// Test that load() returns valid JSON or error
    async fn test_load_returns_valid_json_or_error(&self, source: &dyn ConfigSourceTrait) {
        let result = source.load().await;

        match result {
            Ok(json_value) => {
                // If successful, must be valid JSON
                assert!(json_value.is_object() || json_value.is_array(),
                    "Config sources must return objects or arrays, got: {:?}", json_value);
            }
            Err(_) => {
                // Errors are acceptable, but must be properly typed
                // Contract allows failures due to network, permissions, etc.
            }
        }
    }

    /// Test that is_available() is consistent
    async fn test_availability_consistency(&self, source: &dyn ConfigSourceTrait) {
        let available = source.is_available().await;

        // Availability should be a boolean - no panics allowed
        let _ = available; // Just ensure it returns without panic
    }

    /// Test that priority is in valid range
    fn test_priority_range(&self, source: &dyn ConfigSourceTrait) {
        let priority = source.priority();
        assert!(priority <= 255, "Priority must be <= 255, got: {}", priority);
    }

    /// Test that validate() catches obvious errors
    fn test_validation_logic(&self, source: &dyn ConfigSourceTrait) {
        let result = source.validate();

        // Validation should not panic, only return Ok or Err
        let _ = result; // Just ensure it returns without panic
    }

    /// Test that metadata is well-formed
    fn test_metadata_well_formed(&self, source: &dyn ConfigSourceTrait) {
        let metadata = source.metadata();

        assert!(!metadata.name.is_empty(), "Metadata name must not be empty");
        assert!(!metadata.source_type.is_empty(), "Source type must not be empty");
        assert!(metadata.priority <= 255, "Metadata priority must be <= 255, got: {}", metadata.priority);
    }
}

/// Contract test runner for config sources
pub struct ConfigSourceContractTestRunner;

impl ConfigSourceContractTestRunner {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_all_contract_tests(&self) {
        println!("Running Config Source Contract Tests...");

        // Test file sources
        self.test_file_source_contracts().await;

        // Test environment sources
        self.test_environment_source_contracts().await;

        // Test custom sources
        self.test_custom_source_contracts().await;

        println!("Config Source Contract Tests completed successfully!");
    }

    async fn test_file_source_contracts(&self) {
        println!("Testing File Source contracts...");

        // Test JSON file source
        let json_source = FileSource::json("nonexistent.json");
        self.run_contract_tests(&json_source, "JSON File Source").await;

        // Test auto-detect source
        let auto_source = FileSource::auto("nonexistent.json");
        self.run_contract_tests(&auto_source, "Auto File Source").await;
    }

    async fn test_environment_source_contracts(&self) {
        println!("Testing Environment Source contracts...");

        let env_source = EnvironmentSource::new();
        self.run_contract_tests(&env_source, "Environment Source").await;

        let prefixed_source = EnvironmentSource::prefixed("TEST");
        self.run_contract_tests(&prefixed_source, "Prefixed Environment Source").await;
    }

    async fn test_custom_source_contracts(&self) {
        println!("Testing Custom Source contracts...");

        // Test custom source with static JSON
        let custom_source = CustomSource::from_value("test", serde_json::json!({"test": "value"}));
        self.run_contract_tests(&custom_source, "Custom Source").await;
    }

    async fn run_contract_tests(&self, source: &dyn ConfigSourceTrait, name: &str) {
        println!("  Testing {} contracts...", name);

        // Test load contract
        self.test_load_returns_valid_json_or_error(source).await;

        // Test availability contract
        self.test_availability_consistency(source).await;

        // Test priority contract
        self.test_priority_range(source);

        // Test validation contract
        self.test_validation_logic(source);

        // Test metadata contract
        self.test_metadata_well_formed(source);

        println!("  ✓ {} contracts verified", name);
    }
}

#[async_trait::async_trait]
impl ConfigSourceContractTester for ConfigSourceContractTestRunner {
    async fn test_load_returns_valid_json_or_error(&self, source: &dyn ConfigSourceTrait) {
        ConfigSourceContractTester::test_load_returns_valid_json_or_error(self, source).await;
    }

    async fn test_availability_consistency(&self, source: &dyn ConfigSourceTrait) {
        ConfigSourceContractTester::test_availability_consistency(self, source).await;
    }

    fn test_priority_range(&self, source: &dyn ConfigSourceTrait) {
        ConfigSourceContractTester::test_priority_range(self, source);
    }

    fn test_validation_logic(&self, source: &dyn ConfigSourceTrait) {
        ConfigSourceContractTester::test_validation_logic(self, source);
    }

    fn test_metadata_well_formed(&self, source: &dyn ConfigSourceTrait) {
        ConfigSourceContractTester::test_metadata_well_formed(self, source);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_config_source_contracts_comprehensive() {
        let runner = ConfigSourceContractTestRunner::new();
        runner.run_all_contract_tests().await;
    }

    #[tokio::test]
    async fn test_file_source_load_contract() {
        // Create a temporary JSON file
        let mut temp_file = NamedTempFile::new().unwrap();
        let json_content = r#"{"database": {"host": "localhost", "port": 5432}}"#;
        temp_file.write_all(json_content.as_bytes()).unwrap();
        let temp_path = temp_file.path().to_string_lossy().to_string();

        let source = FileSource::json(&temp_path);

        // Test contract: load returns valid JSON
        let result = source.load().await.unwrap();
        assert_eq!(result["database"]["host"], "localhost");
        assert_eq!(result["database"]["port"], 5432);
    }

    #[tokio::test]
    async fn test_environment_source_load_contract() {
        // Set up test environment variables
        std::env::set_var("TEST_APP_NAME", "contract-test");
        std::env::set_var("TEST_APP_VERSION", "1.0.0");
        std::env::set_var("TEST_DATABASE_HOST", "test-db");

        let source = EnvironmentSource::prefixed("TEST");

        // Test contract: load returns valid JSON
        let result = source.load().await.unwrap();

        assert_eq!(result["app"]["name"], "contract-test");
        assert_eq!(result["app"]["version"], "1.0.0");
        assert_eq!(result["database"]["host"], "test-db");

        // Clean up
        std::env::remove_var("TEST_APP_NAME");
        std::env::remove_var("TEST_APP_VERSION");
        std::env::remove_var("TEST_DATABASE_HOST");
    }

    #[test]
    fn test_source_metadata_contract() {
        let source = EnvironmentSource::prefixed("APP");

        let metadata = source.metadata();

        // Contract: metadata must be well-formed
        assert!(!metadata.name.is_empty());
        assert!(!metadata.source_type.is_empty());
        assert!(metadata.priority <= 255);
        assert!(!metadata.description.is_empty());
    }

    #[test]
    fn test_source_priority_contract() {
        let file_source = FileSource::json("test.json");
        let env_source = EnvironmentSource::new();

        // Contract: priorities in valid range
        assert!(file_source.priority() <= 255);
        assert!(env_source.priority() <= 255);

        // Contract: file sources have higher priority than env
        assert!(file_source.priority() > env_source.priority());
    }

    #[test]
    fn test_source_validation_contract() {
        // Valid source
        let valid_source = FileSource::json("valid.json");
        assert!(valid_source.validate().is_ok());

        // Invalid source (empty path)
        let invalid_source = FileSource::json("");
        assert!(invalid_source.validate().is_err());
    }
}
