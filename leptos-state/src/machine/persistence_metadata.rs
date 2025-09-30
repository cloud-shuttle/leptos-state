//! Machine metadata for persistence

use super::persistence_core::PersistenceError;
use super::*;

/// Machine metadata for persistence
#[derive(Debug, Clone)]
pub struct MachineMetadata {
    /// Machine name
    pub name: Option<String>,
    /// Machine description
    pub description: Option<String>,
    /// Machine version
    pub version: String,
    /// Creation timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub modified_at: u64,
    /// Author/creator
    pub author: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Custom properties
    pub properties: std::collections::HashMap<String, serde_json::Value>,
    /// Schema information
    pub schema: Option<SchemaInfo>,
    /// Statistics
    pub stats: MachineStats,
}

impl MachineMetadata {
    /// Create new metadata
    pub fn new() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            name: None,
            description: None,
            version: "1.0.0".to_string(),
            created_at: now,
            modified_at: now,
            author: None,
            tags: Vec::new(),
            properties: std::collections::HashMap::new(),
            schema: None,
            stats: MachineStats::new(),
        }
    }

    /// Set name
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set description
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set version
    pub fn version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    /// Set author
    pub fn author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Add tag
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Set tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Set custom property
    pub fn property(mut self, key: String, value: serde_json::Value) -> Self {
        self.properties.insert(key, value);
        self
    }

    /// Set schema information
    pub fn schema(mut self, schema: SchemaInfo) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Update modification timestamp
    pub fn touch(&mut self) {
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Get display name (name or "Unnamed Machine")
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unnamed Machine")
    }

    /// Check if machine has been modified since creation
    pub fn is_modified(&self) -> bool {
        self.modified_at > self.created_at
    }

    /// Get age in seconds
    pub fn age_seconds(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.created_at)
    }

    /// Get time since last modification in seconds
    pub fn time_since_modified(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.modified_at)
    }

    /// Check if metadata has required fields
    pub fn is_complete(&self) -> bool {
        self.name.is_some() && self.description.is_some() && self.author.is_some()
    }

    /// Validate metadata
    pub fn validate(&self) -> Result<(), PersistenceError> {
        if self.version.is_empty() {
            return Err(PersistenceError::ValidationError(
                "Version cannot be empty".to_string(),
            ));
        }

        // Validate version format (basic semver check)
        if !self.version.contains('.') {
            return Err(PersistenceError::ValidationError(
                "Version must be in semver format".to_string(),
            ));
        }

        if self.created_at == 0 {
            return Err(PersistenceError::ValidationError(
                "Created timestamp cannot be zero".to_string(),
            ));
        }

        if self.modified_at < self.created_at {
            return Err(PersistenceError::ValidationError(
                "Modified timestamp cannot be before created timestamp".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for MachineMetadata {
    fn default() -> Self {
        Self::new()
    }
}

/// Schema information for machine validation
#[derive(Debug, Clone)]
pub struct SchemaInfo {
    /// Schema version
    pub version: String,
    /// JSON schema for validation
    pub json_schema: Option<serde_json::Value>,
    /// TypeScript definitions
    pub typescript_defs: Option<String>,
    /// Validation rules
    pub validation_rules: Vec<ValidationRule>,
}

impl SchemaInfo {
    /// Create new schema info
    pub fn new(version: String) -> Self {
        Self {
            version,
            json_schema: None,
            typescript_defs: None,
            validation_rules: Vec::new(),
        }
    }

    /// Set JSON schema
    pub fn json_schema(mut self, schema: serde_json::Value) -> Self {
        self.json_schema = Some(schema);
        self
    }

    /// Set TypeScript definitions
    pub fn typescript_defs(mut self, defs: String) -> Self {
        self.typescript_defs = Some(defs);
        self
    }

    /// Add validation rule
    pub fn add_rule(mut self, rule: ValidationRule) -> Self {
        self.validation_rules.push(rule);
        self
    }

    /// Validate data against schema
    pub fn validate_data(&self, data: &serde_json::Value) -> Result<(), PersistenceError> {
        // Basic validation - in a real implementation, this would use JSON Schema validation
        for rule in &self.validation_rules {
            rule.validate(data)?;
        }
        Ok(())
    }
}

/// Validation rule for schema
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// JSON path to validate
    pub path: String,
    /// Validation type
    pub rule_type: ValidationType,
    /// Rule parameters
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

impl ValidationRule {
    /// Create a new validation rule
    pub fn new(name: String, path: String, rule_type: ValidationType) -> Self {
        Self {
            name,
            description: String::new(),
            path,
            rule_type,
            parameters: std::collections::HashMap::new(),
        }
    }

    /// Set description
    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add parameter
    pub fn parameter(mut self, key: String, value: serde_json::Value) -> Self {
        self.parameters.insert(key, value);
        self
    }

    /// Validate data against this rule
    pub fn validate(&self, data: &serde_json::Value) -> Result<(), PersistenceError> {
        // Simplified validation - in practice, this would use JSON path evaluation
        match self.rule_type {
            ValidationType::Required => {
                if data.get(&self.path).is_none() {
                    return Err(PersistenceError::ValidationError(format!(
                        "Required field '{}' is missing",
                        self.path
                    )));
                }
            }
            ValidationType::Type => {
                // Type checking would be implemented here
            }
            ValidationType::Range => {
                // Range checking would be implemented here
            }
            ValidationType::Pattern => {
                // Pattern matching would be implemented here
            }
            ValidationType::Custom => {
                // Custom validation would be implemented here
            }
        }
        Ok(())
    }
}

/// Validation types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationType {
    /// Field is required
    Required,
    /// Type validation
    Type,
    /// Range validation
    Range,
    /// Pattern validation
    Pattern,
    /// Custom validation
    Custom,
}

/// Machine statistics
#[derive(Debug, Clone)]
pub struct MachineStats {
    /// Total transitions executed
    pub transitions_executed: u64,
    /// Total time spent in transitions (nanoseconds)
    pub total_transition_time: u64,
    /// Average transition time (nanoseconds)
    pub avg_transition_time: u64,
    /// Peak memory usage (bytes)
    pub peak_memory_usage: u64,
    /// Current memory usage (bytes)
    pub current_memory_usage: u64,
    /// Total errors encountered
    pub total_errors: u64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Last activity timestamp
    pub last_activity: u64,
}

impl MachineStats {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
            transitions_executed: 0,
            total_transition_time: 0,
            avg_transition_time: 0,
            peak_memory_usage: 0,
            current_memory_usage: 0,
            total_errors: 0,
            uptime_seconds: 0,
            last_activity: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Record a transition execution
    pub fn record_transition(&mut self, duration: std::time::Duration) {
        self.transitions_executed += 1;
        let duration_ns = duration.as_nanos() as u64;
        self.total_transition_time += duration_ns;
        self.avg_transition_time = self.total_transition_time / self.transitions_executed;
        self.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Record an error
    pub fn record_error(&mut self) {
        self.total_errors += 1;
    }

    /// Update memory usage
    pub fn update_memory(&mut self, current: u64) {
        self.current_memory_usage = current;
        if current > self.peak_memory_usage {
            self.peak_memory_usage = current;
        }
    }

    /// Update uptime
    pub fn update_uptime(&mut self, uptime: std::time::Duration) {
        self.uptime_seconds = uptime.as_secs();
    }

    /// Get transitions per second
    pub fn transitions_per_second(&self) -> f64 {
        if self.uptime_seconds == 0 {
            0.0
        } else {
            self.transitions_executed as f64 / self.uptime_seconds as f64
        }
    }

    /// Get error rate (errors per transition)
    pub fn error_rate(&self) -> f64 {
        if self.transitions_executed == 0 {
            0.0
        } else {
            self.total_errors as f64 / self.transitions_executed as f64
        }
    }

    /// Get memory efficiency (lower is better)
    pub fn memory_efficiency(&self) -> f64 {
        if self.transitions_executed == 0 {
            0.0
        } else {
            self.peak_memory_usage as f64 / self.transitions_executed as f64
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for MachineStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata builder for fluent construction
pub struct MetadataBuilder {
    metadata: MachineMetadata,
}

impl MetadataBuilder {
    /// Create a new metadata builder
    pub fn new() -> Self {
        Self {
            metadata: MachineMetadata::new(),
        }
    }

    /// Set name
    pub fn name(mut self, name: String) -> Self {
        self.metadata.name = Some(name);
        self
    }

    /// Set description
    pub fn description(mut self, description: String) -> Self {
        self.metadata.description = Some(description);
        self
    }

    /// Set version
    pub fn version(mut self, version: String) -> Self {
        self.metadata.version = version;
        self
    }

    /// Set author
    pub fn author(mut self, author: String) -> Self {
        self.metadata.author = Some(author);
        self
    }

    /// Add tag
    pub fn tag(mut self, tag: String) -> Self {
        self.metadata.tags.push(tag);
        self
    }

    /// Add multiple tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.metadata.tags = tags;
        self
    }

    /// Add property
    pub fn property(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.properties.insert(key, value);
        self
    }

    /// Build the metadata
    pub fn build(self) -> MachineMetadata {
        self.metadata
    }
}

/// Metadata utilities
pub mod utils {
    use super::*;

    /// Create basic metadata
    pub fn basic_metadata(name: &str, version: &str) -> MachineMetadata {
        MetadataBuilder::new()
            .name(name.to_string())
            .version(version.to_string())
            .build()
    }

    /// Create metadata with tags
    pub fn tagged_metadata(name: &str, version: &str, tags: Vec<String>) -> MachineMetadata {
        MetadataBuilder::new()
            .name(name.to_string())
            .version(version.to_string())
            .tags(tags)
            .build()
    }

    /// Merge metadata (other takes precedence)
    pub fn merge_metadata(base: MachineMetadata, other: MachineMetadata) -> MachineMetadata {
        MachineMetadata {
            name: other.name.or(base.name),
            description: other.description.or(base.description),
            version: other.version,
            created_at: base.created_at, // Keep original creation time
            modified_at: std::cmp::max(base.modified_at, other.modified_at),
            author: other.author.or(base.author),
            tags: if other.tags.is_empty() {
                base.tags
            } else {
                other.tags
            },
            properties: {
                let mut merged = base.properties;
                merged.extend(other.properties);
                merged
            },
            schema: other.schema.or(base.schema),
            stats: base.stats, // Keep base stats
        }
    }

    /// Validate metadata completeness
    pub fn validate_metadata_completeness(metadata: &MachineMetadata) -> Vec<String> {
        let mut missing = Vec::new();

        if metadata.name.is_none() {
            missing.push("name".to_string());
        }
        if metadata.description.is_none() {
            missing.push("description".to_string());
        }
        if metadata.author.is_none() {
            missing.push("author".to_string());
        }
        if metadata.tags.is_empty() {
            missing.push("tags".to_string());
        }

        missing
    }

    /// Export metadata to JSON
    pub fn export_metadata_to_json(metadata: &MachineMetadata) -> Result<String, PersistenceError> {
        serde_json::to_string_pretty(metadata)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))
    }

    /// Import metadata from JSON
    pub fn import_metadata_from_json(json: &str) -> Result<MachineMetadata, PersistenceError> {
        serde_json::from_str(json)
            .map_err(|e| PersistenceError::DeserializationError(e.to_string()))
    }

    /// Create version comparison
    pub fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
        // Simple version comparison - in practice, you'd use a proper semver crate
        a.cmp(b)
    }
}
