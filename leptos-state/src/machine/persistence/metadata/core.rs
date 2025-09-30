//! Core machine metadata structures and functionality

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
    pub schema: Option<super::schema::SchemaInfo>,
    /// Statistics
    pub stats: super::stats::MachineStats,
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
            stats: super::stats::MachineStats::new(),
        }
    }

    /// Create metadata with name
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Create metadata with description
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Create metadata with version
    pub fn with_version<S: Into<String>>(mut self, version: S) -> Self {
        self.version = version.into();
        self
    }

    /// Create metadata with author
    pub fn with_author<S: Into<String>>(mut self, author: S) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Add a tag
    pub fn with_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add multiple tags
    pub fn with_tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags.extend(tags.into_iter().map(|s| s.into()));
        self
    }

    /// Set a custom property
    pub fn with_property<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Set schema information
    pub fn with_schema(mut self, schema: super::schema::SchemaInfo) -> Self {
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

    /// Get a property value
    pub fn get_property(&self, key: &str) -> Option<&serde_json::Value> {
        self.properties.get(key)
    }

    /// Check if metadata has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Check if metadata has all specified tags
    pub fn has_tags(&self, tags: &[String]) -> bool {
        tags.iter().all(|tag| self.has_tag(tag))
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

    /// Check if metadata is stale (modified more than threshold ago)
    pub fn is_stale(&self, threshold_seconds: u64) -> bool {
        self.time_since_modified() > threshold_seconds
    }

    /// Get display name (name or "unnamed")
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("unnamed")
    }

    /// Validate metadata
    pub fn validate(&self) -> Result<(), String> {
        if let Some(name) = &self.name {
            if name.trim().is_empty() {
                return Err("Machine name cannot be empty".to_string());
            }
        }

        if self.version.trim().is_empty() {
            return Err("Version cannot be empty".to_string());
        }

        // Validate version format (basic semver check)
        if !self.version.contains('.') {
            return Err("Version must be in semver format (x.y.z)".to_string());
        }

        Ok(())
    }

    /// Merge with another metadata (self takes precedence)
    pub fn merge(&mut self, other: &MachineMetadata) {
        if self.name.is_none() {
            self.name = other.name.clone();
        }
        if self.description.is_none() {
            self.description = other.description.clone();
        }
        if self.author.is_none() {
            self.author = other.author.clone();
        }

        // Merge tags (avoid duplicates)
        for tag in &other.tags {
            if !self.has_tag(tag) {
                self.tags.push(tag.clone());
            }
        }

        // Merge properties (self takes precedence)
        for (key, value) in &other.properties {
            if !self.properties.contains_key(key) {
                self.properties.insert(key.clone(), value.clone());
            }
        }

        // Update timestamps
        self.touch();
    }

    /// Create a summary of the metadata
    pub fn summary(&self) -> String {
        let mut summary = format!("Machine '{}' (v{})", self.display_name(), self.version);

        if let Some(author) = &self.author {
            summary.push_str(&format!(" by {}", author));
        }

        if !self.tags.is_empty() {
            summary.push_str(&format!(" [{}]", self.tags.join(", ")));
        }

        summary.push_str(&format!(" - {} properties", self.properties.len()));
        summary
    }
}

impl Default for MachineMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MachineMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}
