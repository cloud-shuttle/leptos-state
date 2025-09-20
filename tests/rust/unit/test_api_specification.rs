//! Test suite for API specification generation and validation
//! This test defines the expected behavior for OpenAPI spec generation

use leptos_state::{
    machine::{Machine, MachineBuilder, StateValue},
    store::Store,
    api_spec::{ApiSpecGenerator, OpenApiSpec, ApiContract},
    schema::{JsonSchema, SchemaValidator},
};

#[cfg(feature = "serialization")]
use serde_json::Value;

// Test data structures for API specification testing
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize, serde::Deserialize))]
pub struct TestContext {
    pub counter: i32,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize, serde::Deserialize))]
pub enum TestEvent {
    Start,
    Stop,
    Increment,
    Decrement,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize, serde::Deserialize))]
pub enum TestState {
    Idle,
    Active,
    Paused,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize, serde::Deserialize))]
pub struct TestStoreState {
    pub count: i32,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_spec_generation() {
        // Test that we can generate an OpenAPI specification from a state machine
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
                .on(TestEvent::Start, "active")
            .state("active")
                .on(TestEvent::Stop, "idle")
                .on(TestEvent::Increment, "active")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let spec_generator = ApiSpecGenerator::new();
        let openapi_spec = spec_generator.generate_openapi_spec(&machine)
            .expect("Failed to generate OpenAPI spec");

        // Verify OpenAPI spec structure
        assert_eq!(openapi_spec.info.title, "leptos-state State Machine API");
        assert_eq!(openapi_spec.info.version, "1.0.0");
        assert!(openapi_spec.paths.len() > 0);
        
        // Verify state machine endpoints are present
        assert!(openapi_spec.paths.contains_key("/state"));
        assert!(openapi_spec.paths.contains_key("/transition"));
        assert!(openapi_spec.paths.contains_key("/states"));
    }

    #[test]
    fn test_json_schema_generation() {
        // Test that we can generate JSON schemas for our types
        let schema_generator = JsonSchema::new();
        
        let context_schema = schema_generator.generate_schema::<TestContext>()
            .expect("Failed to generate context schema");
        let event_schema = schema_generator.generate_schema::<TestEvent>()
            .expect("Failed to generate event schema");
        let state_schema = schema_generator.generate_schema::<TestState>()
            .expect("Failed to generate state schema");

        // Verify schema structure
        assert_eq!(context_schema.schema_type, "object");
        assert!(context_schema.properties.contains_key("counter"));
        assert!(context_schema.properties.contains_key("name"));
        
        assert_eq!(event_schema.schema_type, "string");
        assert!(event_schema.enum_values.is_some());
        
        assert_eq!(state_schema.schema_type, "string");
        assert!(state_schema.enum_values.is_some());
    }

    #[test]
    #[cfg(feature = "serialization")]
    fn test_schema_validation() {
        // Test that schema validation works correctly
        let validator = SchemaValidator::new();
        let schema = JsonSchema::new().generate_schema::<TestContext>()
            .expect("Failed to generate schema");

        // Valid data should pass validation
        let valid_data = serde_json::json!({
            "counter": 42,
            "name": "test"
        });
        assert!(validator.validate(&schema, &valid_data).is_ok());

        // Invalid data should fail validation
        let invalid_data = serde_json::json!({
            "counter": "not_a_number",
            "name": "test"
        });
        assert!(validator.validate(&schema, &invalid_data).is_err());
    }

    #[test]
    fn test_api_contract_creation() {
        // Test that we can create API contracts
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
                .on(TestEvent::Start, "active")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let contract = ApiContract::from_machine(&machine)
            .expect("Failed to create API contract");

        // Verify contract structure
        assert_eq!(contract.version, "1.0.0");
        assert!(contract.endpoints.len() > 0);
        assert!(contract.schemas.len() > 0);
        
        // Verify required endpoints are present
        let endpoint_paths: Vec<&str> = contract.endpoints.iter()
            .map(|e| e.path.as_str())
            .collect();
        assert!(endpoint_paths.contains(&"/state"));
        assert!(endpoint_paths.contains(&"/transition"));
    }

    #[test]
    fn test_contract_validation() {
        // Test that API contracts can be validated
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
                .on(TestEvent::Start, "active")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let contract = ApiContract::from_machine(&machine)
            .expect("Failed to create API contract");

        // Contract should be valid
        assert!(contract.validate().is_ok());
        
        // Test contract serialization/deserialization
        let serialized = serde_json::to_string(&contract)
            .expect("Failed to serialize contract");
        let deserialized: ApiContract = serde_json::from_str(&serialized)
            .expect("Failed to deserialize contract");
        
        assert_eq!(contract, deserialized);
    }

    #[test]
    fn test_api_versioning() {
        // Test that API versioning works correctly
        let machine_v1 = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
                .on(TestEvent::Start, "active")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let contract_v1 = ApiContract::from_machine(&machine_v1)
            .expect("Failed to create v1 contract");
        contract_v1.set_version("1.0.0");

        // Test version comparison
        assert!(contract_v1.is_compatible_with("1.0.0"));
        assert!(contract_v1.is_compatible_with("1.0.1"));
        assert!(!contract_v1.is_compatible_with("2.0.0"));
    }

    #[test]
    fn test_breaking_change_detection() {
        // Test that we can detect breaking changes between API versions
        let machine_v1 = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
                .on(TestEvent::Start, "active")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let contract_v1 = ApiContract::from_machine(&machine_v1)
            .expect("Failed to create v1 contract");

        // Create a modified machine (simulating a breaking change)
        let machine_v2 = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
                .on(TestEvent::Start, "active")
            .state("new_state")  // Added new state
                .on(TestEvent::Stop, "idle")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let contract_v2 = ApiContract::from_machine(&machine_v2)
            .expect("Failed to create v2 contract");

        // Detect breaking changes
        let breaking_changes = contract_v1.detect_breaking_changes(&contract_v2);
        assert!(breaking_changes.len() > 0);
        assert!(breaking_changes.iter().any(|change| change.is_breaking()));
    }

    #[test]
    fn test_store_api_specification() {
        // Test that we can generate API specs for stores
        let store_schema = JsonSchema::new().generate_schema::<TestStoreState>()
            .expect("Failed to generate store schema");

        // Verify store schema structure
        assert_eq!(store_schema.schema_type, "object");
        assert!(store_schema.properties.contains_key("count"));
        assert!(store_schema.properties.contains_key("name"));

        // Test store API contract
        let store_contract = ApiContract::for_store::<TestStoreState>()
            .expect("Failed to create store contract");

        assert!(store_contract.endpoints.iter().any(|e| e.path == "/store"));
        assert!(store_contract.endpoints.iter().any(|e| e.path == "/store/update"));
    }

    #[test]
    #[cfg(feature = "serialization")]
    fn test_openapi_spec_serialization() {
        // Test that OpenAPI specs can be serialized to JSON/YAML
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
                .on(TestEvent::Start, "active")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let spec_generator = ApiSpecGenerator::new();
        let openapi_spec = spec_generator.generate_openapi_spec(&machine)
            .expect("Failed to generate OpenAPI spec");

        // Test JSON serialization
        let json_spec = openapi_spec.to_json()
            .expect("Failed to serialize to JSON");
        assert!(json_spec.contains("\"openapi\""));
        assert!(json_spec.contains("\"info\""));
        assert!(json_spec.contains("\"paths\""));

        // Test YAML serialization
        let yaml_spec = openapi_spec.to_yaml()
            .expect("Failed to serialize to YAML");
        assert!(yaml_spec.contains("openapi:"));
        assert!(yaml_spec.contains("info:"));
        assert!(yaml_spec.contains("paths:"));
    }
}
