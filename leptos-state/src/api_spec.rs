//! API specification generation and validation
//! This module provides tools for generating OpenAPI specifications and API contracts

use crate::machine::Machine;
use std::collections::HashMap;

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

/// OpenAPI 3.0 specification generator
pub struct ApiSpecGenerator;

impl Default for ApiSpecGenerator {
    fn default() -> Self {
        Self
    }
}

impl ApiSpecGenerator {
    /// Create a new API spec generator
    pub fn new() -> Self {
        Self
    }

    /// Generate OpenAPI specification from a state machine
    pub fn generate_openapi_spec<C, E>(
        &self,
        _machine: &Machine<C, E>,
    ) -> Result<OpenApiSpec, ApiSpecError>
    where
        C: Clone + PartialEq + std::fmt::Debug + Default + Send + Sync,
        E: Clone + PartialEq + std::fmt::Debug + Default + Send + Sync + crate::machine::events::Event,
    {
        // Generate paths for state machine operations
        let mut paths = HashMap::new();
        
        // Add state machine endpoints
        paths.insert("/machine/state".to_string(), PathItem::default());
        paths.insert("/machine/transition".to_string(), PathItem::default());
        paths.insert("/machine/events".to_string(), PathItem::default());
        paths.insert("/machine/states".to_string(), PathItem::default());
        
        Ok(OpenApiSpec {
            openapi: "3.0.0".to_string(),
            info: OpenApiInfo {
                title: "leptos-state State Machine API".to_string(),
                version: "1.0.0".to_string(),
                description: Some("API for leptos-state state machines".to_string()),
            },
            paths,
            components: Some(OpenApiComponents {}),
        })
    }
}

/// OpenAPI 3.0 specification structure
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: OpenApiInfo,
    pub paths: HashMap<String, PathItem>,
    pub components: Option<OpenApiComponents>,
}

/// OpenAPI info section
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct OpenApiInfo {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
}

/// OpenAPI path item
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct PathItem {
    // TODO: Add actual path operations
}

/// OpenAPI components section
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct OpenApiComponents {
    // TODO: Add schemas, responses, etc.
}

impl OpenApiSpec {
    /// Serialize to JSON
    #[cfg(feature = "serialization")]
    pub fn to_json(&self) -> Result<String, ApiSpecError> {
        serde_json::to_string_pretty(self).map_err(ApiSpecError::SerializationError)
    }

    /// Serialize to YAML
    #[cfg(feature = "serialization")]
    pub fn to_yaml(&self) -> Result<String, ApiSpecError> {
        serde_yaml::to_string(self).map_err(ApiSpecError::SerializationError)
    }

    /// Serialize to JSON (fallback when serialization feature is disabled)
    #[cfg(not(feature = "serialization"))]
    pub fn to_json(&self) -> Result<String, ApiSpecError> {
        Err(ApiSpecError::SerializationError("Serialization feature not enabled".to_string()))
    }

    /// Serialize to YAML (fallback when serialization feature is disabled)
    #[cfg(not(feature = "serialization"))]
    pub fn to_yaml(&self) -> Result<String, ApiSpecError> {
        Err(ApiSpecError::SerializationError("Serialization feature not enabled".to_string()))
    }
}

/// API contract for state machines
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct ApiContract {
    pub version: String,
    pub endpoints: Vec<ApiEndpoint>,
    pub schemas: Vec<ApiSchema>,
}

/// API endpoint definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct ApiEndpoint {
    pub path: String,
    pub method: String,
    pub description: Option<String>,
}

/// API schema definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct ApiSchema {
    pub name: String,
    pub schema_type: String,
    pub properties: HashMap<String, String>,
}

impl ApiContract {
    /// Create API contract from a state machine
    pub fn from_machine<C, E>(_machine: &Machine<C, E>) -> Result<Self, ApiSpecError>
    where
        C: Clone + PartialEq + std::fmt::Debug + Default + Send + Sync,
        E: Clone + PartialEq + std::fmt::Debug + Default + Send + Sync + crate::machine::events::Event,
    {
        let endpoints = vec![
            ApiEndpoint {
                path: "/machine/state".to_string(),
                method: "GET".to_string(),
                description: Some("Get current state".to_string()),
            },
            ApiEndpoint {
                path: "/machine/transition".to_string(),
                method: "POST".to_string(),
                description: Some("Trigger state transition".to_string()),
            },
            ApiEndpoint {
                path: "/machine/events".to_string(),
                method: "GET".to_string(),
                description: Some("Get available events".to_string()),
            },
            ApiEndpoint {
                path: "/machine/states".to_string(),
                method: "GET".to_string(),
                description: Some("Get all states".to_string()),
            },
        ];

        let schemas = vec![
            ApiSchema {
                name: "State".to_string(),
                schema_type: "string".to_string(),
                properties: HashMap::new(),
            },
            ApiSchema {
                name: "Event".to_string(),
                schema_type: "string".to_string(),
                properties: HashMap::new(),
            },
            ApiSchema {
                name: "Context".to_string(),
                schema_type: "object".to_string(),
                properties: HashMap::new(),
            },
        ];

        Ok(ApiContract {
            version: "1.0.0".to_string(),
            endpoints,
            schemas,
        })
    }

    /// Create API contract for a store
    pub fn for_store<T>() -> Result<Self, ApiSpecError>
    where
        T: Clone + PartialEq + std::fmt::Debug + Default + Send + Sync,
    {
        let endpoints = vec![
            ApiEndpoint {
                path: "/store".to_string(),
                method: "GET".to_string(),
                description: Some("Get store state".to_string()),
            },
            ApiEndpoint {
                path: "/store/update".to_string(),
                method: "POST".to_string(),
                description: Some("Update store state".to_string()),
            },
            ApiEndpoint {
                path: "/store/reset".to_string(),
                method: "POST".to_string(),
                description: Some("Reset store to default state".to_string()),
            },
        ];

        let schemas = vec![
            ApiSchema {
                name: "StoreState".to_string(),
                schema_type: "object".to_string(),
                properties: HashMap::new(),
            },
        ];

        Ok(ApiContract {
            version: "1.0.0".to_string(),
            endpoints,
            schemas,
        })
    }

    /// Set the version of the contract
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Check if this contract is compatible with a given version
    pub fn is_compatible_with(&self, version: &str) -> bool {
        // Simple semantic versioning compatibility check
        let self_parts: Vec<u32> = self.version
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        let other_parts: Vec<u32> = version
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();

        if self_parts.len() >= 2 && other_parts.len() >= 2 {
            // Major version must match, minor version can be higher
            self_parts[0] == other_parts[0] && other_parts[1] >= self_parts[1]
        } else {
            false
        }
    }

    /// Validate the contract
    pub fn validate(&self) -> Result<(), ApiSpecError> {
        if self.version.is_empty() {
            return Err(ApiSpecError::ValidationError("Version cannot be empty".to_string()));
        }
        if self.endpoints.is_empty() {
            return Err(ApiSpecError::ValidationError("At least one endpoint is required".to_string()));
        }
        Ok(())
    }

    /// Detect breaking changes between two contracts
    pub fn detect_breaking_changes(&self, other: &ApiContract) -> Vec<BreakingChange> {
        let mut changes = Vec::new();

        // Check for removed endpoints
        for endpoint in &self.endpoints {
            if !other.endpoints.iter().any(|e| e.path == endpoint.path && e.method == endpoint.method) {
                changes.push(BreakingChange {
                    change_type: BreakingChangeType::RemovedEndpoint,
                    description: format!("Endpoint {} {} was removed", endpoint.method, endpoint.path),
                    is_breaking: true,
                });
            }
        }

        // Check for removed schemas
        for schema in &self.schemas {
            if !other.schemas.iter().any(|s| s.name == schema.name) {
                changes.push(BreakingChange {
                    change_type: BreakingChangeType::RemovedSchema,
                    description: format!("Schema {} was removed", schema.name),
                    is_breaking: true,
                });
            }
        }

        changes
    }
}

/// Breaking change information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct BreakingChange {
    pub change_type: BreakingChangeType,
    pub description: String,
    pub is_breaking: bool,
}

impl BreakingChange {
    /// Check if this is a breaking change
    pub fn is_breaking(&self) -> bool {
        self.is_breaking
    }
}

/// Types of breaking changes
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum BreakingChangeType {
    RemovedEndpoint,
    RemovedSchema,
    ChangedSchema,
    ChangedEndpoint,
}

/// API specification errors
#[derive(Debug, thiserror::Error)]
pub enum ApiSpecError {
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Generation error: {0}")]
    GenerationError(String),
}

#[cfg(feature = "serialization")]
impl From<serde_json::Error> for ApiSpecError {
    fn from(err: serde_json::Error) -> Self {
        ApiSpecError::SerializationError(err.to_string())
    }
}

#[cfg(feature = "serialization")]
impl From<serde_yaml::Error> for ApiSpecError {
    fn from(err: serde_yaml::Error) -> Self {
        ApiSpecError::SerializationError(err.to_string())
    }
}
