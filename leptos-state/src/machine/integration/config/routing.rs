//! Event routing configuration structures

/// Event routing configuration
#[derive(Debug, Clone, PartialEq)]
pub struct EventRoutingConfig {
    /// Whether routing is enabled
    pub enabled: bool,
    /// Routing rules
    pub rules: Vec<RoutingRule>,
    /// Default destination for unrouted events
    pub default_destination: Option<String>,
    /// Whether to route internal events
    pub route_internal: bool,
    /// Maximum routing depth to prevent loops
    pub max_routing_depth: usize,
}

impl Default for EventRoutingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            default_destination: None,
            route_internal: false,
            max_routing_depth: 10,
        }
    }
}

impl EventRoutingConfig {
    /// Create a new routing config
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable routing
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Add a routing rule
    pub fn add_rule(mut self, rule: RoutingRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Add multiple routing rules
    pub fn add_rules<I>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = RoutingRule>,
    {
        self.rules.extend(rules);
        self
    }

    /// Set default destination
    pub fn default_destination<S: Into<String>>(mut self, destination: S) -> Self {
        self.default_destination = Some(destination.into());
        self
    }

    /// Set whether to route internal events
    pub fn route_internal(mut self, route: bool) -> Self {
        self.route_internal = route;
        self
    }

    /// Set maximum routing depth
    pub fn max_routing_depth(mut self, depth: usize) -> Self {
        self.max_routing_depth = depth;
        self
    }

    /// Find routing rule for an event
    pub fn find_rule(&self, event_type: &str, source: &str, destination: Option<&str>) -> Option<&RoutingRule> {
        self.rules.iter().find(|rule| rule.matches(event_type, source, destination))
    }

    /// Get all rules for a specific event type
    pub fn rules_for_event_type(&self, event_type: &str) -> Vec<&RoutingRule> {
        self.rules.iter().filter(|rule| rule.pattern.matches_event_type(event_type)).collect()
    }

    /// Get all rules for a specific source
    pub fn rules_for_source(&self, source: &str) -> Vec<&RoutingRule> {
        self.rules.iter().filter(|rule| rule.pattern.matches_source(source)).collect()
    }

    /// Validate the routing configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.max_routing_depth == 0 {
            return Err("max_routing_depth must be greater than 0".to_string());
        }

        for (index, rule) in self.rules.iter().enumerate() {
            if let Err(err) = rule.validate() {
                return Err(format!("Rule {}: {}", index, err));
            }
        }

        Ok(())
    }

    /// Merge with another routing config (self takes precedence)
    pub fn merge(&mut self, other: &EventRoutingConfig) {
        if !self.enabled && other.enabled {
            self.enabled = true;
        }

        // Add rules that don't already exist
        for rule in &other.rules {
            if !self.rules.iter().any(|r| r.pattern == rule.pattern) {
                self.rules.push(rule.clone());
            }
        }

        if self.default_destination.is_none() {
            self.default_destination = other.default_destination.clone();
        }

        if !self.route_internal && other.route_internal {
            self.route_internal = true;
        }

        self.max_routing_depth = self.max_routing_depth.max(other.max_routing_depth);
    }

    /// Get routing summary
    pub fn summary(&self) -> String {
        format!(
            "EventRoutingConfig {{ enabled: {}, rules: {}, default_dest: {:?}, route_internal: {} }}",
            self.enabled,
            self.rules.len(),
            self.default_destination,
            self.route_internal
        )
    }

    /// Check if routing is configured
    pub fn is_configured(&self) -> bool {
        self.enabled && (!self.rules.is_empty() || self.default_destination.is_some())
    }
}

impl std::fmt::Display for EventRoutingConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Routing rule for events
#[derive(Debug, Clone, PartialEq)]
pub struct RoutingRule {
    /// Event pattern to match
    pub pattern: EventPattern,
    /// Destination for matched events
    pub destination: String,
    /// Priority (higher numbers = higher priority)
    pub priority: i32,
    /// Whether rule is enabled
    pub enabled: bool,
    /// Transformation to apply
    pub transformation: Option<EventTransformation>,
}

impl RoutingRule {
    /// Create a new routing rule
    pub fn new(pattern: EventPattern, destination: String) -> Self {
        Self {
            pattern,
            destination,
            priority: 0,
            enabled: true,
            transformation: None,
        }
    }

    /// Create a rule for specific event type
    pub fn for_event_type<S: Into<String>>(event_type: S, destination: String) -> Self {
        Self::new(EventPattern::event_type(event_type), destination)
    }

    /// Create a rule for specific source
    pub fn for_source<S: Into<String>>(source: S, destination: String) -> Self {
        Self::new(EventPattern::source(source), destination)
    }

    /// Set priority
    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Enable or disable rule
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set transformation
    pub fn transformation(mut self, transformation: EventTransformation) -> Self {
        self.transformation = Some(transformation);
        self
    }

    /// Check if rule matches an event
    pub fn matches(&self, event_type: &str, source: &str, destination: Option<&str>) -> bool {
        self.enabled && self.pattern.matches(event_type, source, destination)
    }

    /// Validate the rule
    pub fn validate(&self) -> Result<(), String> {
        if self.destination.trim().is_empty() {
            return Err("destination cannot be empty".to_string());
        }

        self.pattern.validate()?;

        if let Some(transformation) = &self.transformation {
            transformation.validate()?;
        }

        Ok(())
    }

    /// Get rule summary
    pub fn summary(&self) -> String {
        format!(
            "RoutingRule {{ pattern: {}, dest: '{}', priority: {}, enabled: {} }}",
            self.pattern, self.destination, self.priority, self.enabled
        )
    }
}

impl std::fmt::Display for RoutingRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Event pattern for routing
#[derive(Debug, Clone, PartialEq)]
pub struct EventPattern {
    /// Event type pattern (supports wildcards)
    pub event_type_pattern: Option<String>,
    /// Source pattern (supports wildcards)
    pub source_pattern: Option<String>,
    /// Destination pattern (supports wildcards)
    pub destination_pattern: Option<String>,
    /// Additional conditions
    pub conditions: std::collections::HashMap<String, serde_json::Value>,
}

impl EventPattern {
    /// Create a new event pattern
    pub fn new() -> Self {
        Self {
            event_type_pattern: None,
            source_pattern: None,
            destination_pattern: None,
            conditions: std::collections::HashMap::new(),
        }
    }

    /// Create pattern for specific event type
    pub fn event_type<S: Into<String>>(event_type: S) -> Self {
        Self::new().with_event_type(event_type)
    }

    /// Create pattern for specific source
    pub fn source<S: Into<String>>(source: S) -> Self {
        Self::new().with_source(source)
    }

    /// Set event type pattern
    pub fn with_event_type<S: Into<String>>(mut self, pattern: S) -> Self {
        self.event_type_pattern = Some(pattern.into());
        self
    }

    /// Set source pattern
    pub fn with_source<S: Into<String>>(mut self, pattern: S) -> Self {
        self.source_pattern = Some(pattern.into());
        self
    }

    /// Set destination pattern
    pub fn with_destination<S: Into<String>>(mut self, pattern: S) -> Self {
        self.destination_pattern = Some(pattern.into());
        self
    }

    /// Add a condition
    pub fn with_condition<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.conditions.insert(key.into(), value.into());
        self
    }

    /// Check if pattern matches an event
    pub fn matches(&self, event_type: &str, source: &str, destination: Option<&str>) -> bool {
        // Check event type pattern
        if let Some(pattern) = &self.event_type_pattern {
            if !self.matches_pattern(event_type, pattern) {
                return false;
            }
        }

        // Check source pattern
        if let Some(pattern) = &self.source_pattern {
            if !self.matches_pattern(source, pattern) {
                return false;
            }
        }

        // Check destination pattern
        if let Some(pattern) = &self.destination_pattern {
            if let Some(dest) = destination {
                if !self.matches_pattern(dest, pattern) {
                    return false;
                }
            } else {
                return false; // Pattern requires destination but none provided
            }
        }

        // Additional conditions would be checked here
        // For now, assume they pass if no specific validation is needed

        true
    }

    /// Check if pattern matches event type
    pub fn matches_event_type(&self, event_type: &str) -> bool {
        if let Some(pattern) = &self.event_type_pattern {
            self.matches_pattern(event_type, pattern)
        } else {
            true
        }
    }

    /// Check if pattern matches source
    pub fn matches_source(&self, source: &str) -> bool {
        if let Some(pattern) = &self.source_pattern {
            self.matches_pattern(source, pattern)
        } else {
            true
        }
    }

    /// Simple pattern matching (supports * wildcards)
    fn matches_pattern(&self, value: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if pattern.starts_with('*') && pattern.ends_with('*') {
            let inner = &pattern[1..pattern.len() - 1];
            return value.contains(inner);
        }

        if pattern.starts_with('*') {
            let suffix = &pattern[1..];
            return value.ends_with(suffix);
        }

        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            return value.starts_with(prefix);
        }

        value == pattern
    }

    /// Validate the pattern
    pub fn validate(&self) -> Result<(), String> {
        // Basic validation - patterns shouldn't be empty
        if let Some(pattern) = &self.event_type_pattern {
            if pattern.trim().is_empty() {
                return Err("event_type_pattern cannot be empty".to_string());
            }
        }

        if let Some(pattern) = &self.source_pattern {
            if pattern.trim().is_empty() {
                return Err("source_pattern cannot be empty".to_string());
            }
        }

        if let Some(pattern) = &self.destination_pattern {
            if pattern.trim().is_empty() {
                return Err("destination_pattern cannot be empty".to_string());
            }
        }

        Ok(())
    }

    /// Get pattern summary
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        if let Some(pattern) = &self.event_type_pattern {
            parts.push(format!("type:{}", pattern));
        }

        if let Some(pattern) = &self.source_pattern {
            parts.push(format!("src:{}", pattern));
        }

        if let Some(pattern) = &self.destination_pattern {
            parts.push(format!("dst:{}", pattern));
        }

        if !self.conditions.is_empty() {
            parts.push(format!("conditions:{}", self.conditions.len()));
        }

        if parts.is_empty() {
            "any".to_string()
        } else {
            parts.join(", ")
        }
    }
}

impl Default for EventPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Event transformation for routing
#[derive(Debug, Clone, PartialEq)]
pub struct EventTransformation {
    /// Fields to add
    pub add_fields: std::collections::HashMap<String, serde_json::Value>,
    /// Fields to remove
    pub remove_fields: Vec<String>,
    /// Fields to rename (old_name -> new_name)
    pub rename_fields: std::collections::HashMap<String, String>,
    /// Whether to preserve original event
    pub preserve_original: bool,
}

impl EventTransformation {
    /// Create a new transformation
    pub fn new() -> Self {
        Self {
            add_fields: std::collections::HashMap::new(),
            remove_fields: Vec::new(),
            rename_fields: std::collections::HashMap::new(),
            preserve_original: false,
        }
    }

    /// Add a field
    pub fn add_field<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.add_fields.insert(key.into(), value.into());
        self
    }

    /// Remove a field
    pub fn remove_field<S: Into<String>>(mut self, field: S) -> Self {
        self.remove_fields.push(field.into());
        self
    }

    /// Rename a field
    pub fn rename_field<K: Into<String>, V: Into<String>>(mut self, from: K, to: V) -> Self {
        self.rename_fields.insert(from.into(), to.into());
        self
    }

    /// Preserve original event
    pub fn preserve_original(mut self) -> Self {
        self.preserve_original = true;
        self
    }

    /// Validate the transformation
    pub fn validate(&self) -> Result<(), String> {
        // Check for conflicting operations
        for remove_field in &self.remove_fields {
            if self.add_fields.contains_key(remove_field) {
                return Err(format!("Cannot add and remove field '{}' in same transformation", remove_field));
            }

            if self.rename_fields.contains_key(remove_field) {
                return Err(format!("Cannot remove and rename field '{}' in same transformation", remove_field));
            }
        }

        for (from, to) in &self.rename_fields {
            if self.add_fields.contains_key(from) {
                return Err(format!("Cannot add and rename field '{}' in same transformation", from));
            }

            if self.add_fields.contains_key(to) {
                return Err(format!("Cannot rename to field '{}' that is being added", to));
            }
        }

        Ok(())
    }

    /// Get transformation summary
    pub fn summary(&self) -> String {
        let mut ops = Vec::new();

        if !self.add_fields.is_empty() {
            ops.push(format!("add:{}", self.add_fields.len()));
        }

        if !self.remove_fields.is_empty() {
            ops.push(format!("remove:{}", self.remove_fields.len()));
        }

        if !self.rename_fields.is_empty() {
            ops.push(format!("rename:{}", self.rename_fields.len()));
        }

        if ops.is_empty() {
            "no-op".to_string()
        } else {
            ops.join(", ")
        }
    }
}

impl Default for EventTransformation {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventTransformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventTransformation({})", self.summary())
    }
}

/// Builder for event routing configuration
pub struct EventRoutingConfigBuilder {
    config: EventRoutingConfig,
}

impl EventRoutingConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: EventRoutingConfig::new(),
        }
    }

    /// Enable routing
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self
    }

    /// Add a routing rule
    pub fn add_rule(mut self, rule: RoutingRule) -> Self {
        self.config.rules.push(rule);
        self
    }

    /// Add a rule for event type
    pub fn route_event_type<S: Into<String>>(mut self, event_type: S, destination: S) -> Self {
        let rule = RoutingRule::for_event_type(event_type, destination.into());
        self.config.rules.push(rule);
        self
    }

    /// Add a rule for source
    pub fn route_source<S: Into<String>>(mut self, source: S, destination: S) -> Self {
        let rule = RoutingRule::for_source(source, destination.into());
        self.config.rules.push(rule);
        self
    }

    /// Set default destination
    pub fn default_destination<S: Into<String>>(mut self, destination: S) -> Self {
        self.config.default_destination = Some(destination.into());
        self
    }

    /// Set route internal
    pub fn route_internal(mut self, route: bool) -> Self {
        self.config.route_internal = route;
        self
    }

    /// Set max routing depth
    pub fn max_routing_depth(mut self, depth: usize) -> Self {
        self.config.max_routing_depth = depth;
        self
    }

    /// Build the configuration
    pub fn build(self) -> EventRoutingConfig {
        self.config
    }
}

impl Default for EventRoutingConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
