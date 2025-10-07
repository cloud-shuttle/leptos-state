//! Core event routing configuration structures

use super::rules::RoutingRule;

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

/// Builder for event routing configuration
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::integration::config::routing::rules::RoutingRule;

    #[test]
    fn test_event_routing_config_defaults() {
        let config = EventRoutingConfig::new();
        assert!(config.enabled);
        assert!(config.rules.is_empty());
        assert!(config.default_destination.is_none());
        assert!(!config.route_internal);
        assert_eq!(config.max_routing_depth, 10);
    }

    #[test]
    fn test_event_routing_config_builder() {
        let config = EventRoutingConfigBuilder::new()
            .enabled(true)
            .route_event_type("user.created", "user-service")
            .route_source("api", "gateway")
            .default_destination("default-queue")
            .route_internal(true)
            .max_routing_depth(5)
            .build();

        assert!(config.enabled);
        assert_eq!(config.rules.len(), 2);
        assert_eq!(config.default_destination, Some("default-queue".to_string()));
        assert!(config.route_internal);
        assert_eq!(config.max_routing_depth, 5);
    }

    #[test]
    fn test_config_validation() {
        let valid_config = EventRoutingConfig::new();
        assert!(valid_config.validate().is_ok());

        let invalid_config = EventRoutingConfig {
            max_routing_depth: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_config_merge() {
        let mut config1 = EventRoutingConfig::new();
        config1.enabled = false;

        let config2 = EventRoutingConfig {
            enabled: true,
            default_destination: Some("default-dest".to_string()),
            ..Default::default()
        };

        config1.merge(&config2);

        assert!(config1.enabled); // Should be enabled from config2
        assert_eq!(config1.default_destination, Some("default-dest".to_string()));
    }

    #[test]
    fn test_config_summary() {
        let config = EventRoutingConfig::new();
        let summary = config.summary();
        assert!(summary.contains("enabled: true"));
        assert!(summary.contains("rules: 0"));
    }
}
