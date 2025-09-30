//! Metadata builder for fluent construction

use super::core::MachineMetadata;

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

    /// Set the machine name
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.metadata.name = Some(name.into());
        self
    }

    /// Set the machine description
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.metadata.description = Some(description.into());
        self
    }

    /// Set the version
    pub fn version<S: Into<String>>(mut self, version: S) -> Self {
        self.metadata.version = version.into();
        self
    }

    /// Set the author
    pub fn author<S: Into<String>>(mut self, author: S) -> Self {
        self.metadata.author = Some(author.into());
        self
    }

    /// Add a tag
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.metadata.tags.push(tag.into());
        self
    }

    /// Add multiple tags
    pub fn tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.metadata.tags.extend(tags.into_iter().map(|s| s.into()));
        self
    }

    /// Add a custom property
    pub fn property<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.metadata.properties.insert(key.into(), value.into());
        self
    }

    /// Add multiple properties
    pub fn properties<I, K, V>(mut self, properties: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<serde_json::Value>,
    {
        for (key, value) in properties {
            self.metadata.properties.insert(key.into(), value.into());
        }
        self
    }

    /// Set schema information
    pub fn schema(mut self, schema: super::schema::SchemaInfo) -> Self {
        self.metadata.schema = Some(schema);
        self
    }

    /// Set statistics
    pub fn stats(mut self, stats: super::stats::MachineStats) -> Self {
        self.metadata.stats = stats;
        self
    }

    /// Set creation timestamp
    pub fn created_at(mut self, timestamp: u64) -> Self {
        self.metadata.created_at = timestamp;
        self
    }

    /// Set modification timestamp
    pub fn modified_at(mut self, timestamp: u64) -> Self {
        self.metadata.modified_at = timestamp;
        self
    }

    /// Mark as modified now
    pub fn touch(mut self) -> Self {
        self.metadata.touch();
        self
    }

    /// Build the metadata
    pub fn build(self) -> MachineMetadata {
        self.metadata
    }

    /// Build and validate the metadata
    pub fn build_validated(self) -> Result<MachineMetadata, String> {
        let metadata = self.build();
        metadata.validate()?;
        Ok(metadata)
    }

    /// Create a builder from existing metadata
    pub fn from_metadata(metadata: MachineMetadata) -> Self {
        Self { metadata }
    }

    /// Create a builder with default values for a specific machine type
    pub fn for_machine_type(machine_type: &str) -> Self {
        let mut builder = Self::new();

        match machine_type {
            "traffic_light" => {
                builder = builder
                    .name("Traffic Light State Machine")
                    .description("A simple traffic light state machine with red, yellow, and green states")
                    .tag("demo")
                    .tag("traffic")
                    .tag("finite-state");
            }
            "counter" => {
                builder = builder
                    .name("Counter State Machine")
                    .description("A simple counter with increment and decrement operations")
                    .tag("demo")
                    .tag("counter")
                    .tag("basic");
            }
            "todo" => {
                builder = builder
                    .name("Todo App State Machine")
                    .description("A todo application state machine for managing tasks")
                    .tag("demo")
                    .tag("todo")
                    .tag("application");
            }
            _ => {
                builder = builder
                    .name(format!("{} State Machine", machine_type))
                    .tag("custom");
            }
        }

        builder
    }
}

impl Default for MetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MachineMetadata> for MetadataBuilder {
    fn from(metadata: MachineMetadata) -> Self {
        Self::from_metadata(metadata)
    }
}

/// Convenience functions for creating metadata
pub mod factories {
    use super::*;

    /// Create metadata for a traffic light machine
    pub fn traffic_light() -> MachineMetadata {
        MetadataBuilder::for_machine_type("traffic_light").build()
    }

    /// Create metadata for a counter machine
    pub fn counter() -> MachineMetadata {
        MetadataBuilder::for_machine_type("counter").build()
    }

    /// Create metadata for a todo machine
    pub fn todo_app() -> MachineMetadata {
        MetadataBuilder::for_machine_type("todo").build()
    }

    /// Create minimal metadata with just a name
    pub fn minimal<S: Into<String>>(name: S) -> MachineMetadata {
        MetadataBuilder::new().name(name).build()
    }

    /// Create comprehensive metadata
    pub fn comprehensive(
        name: String,
        description: String,
        version: String,
        author: String,
        tags: Vec<String>,
    ) -> MachineMetadata {
        MetadataBuilder::new()
            .name(name)
            .description(description)
            .version(version)
            .author(author)
            .tags(tags)
            .build()
    }
}
