//! Routing rules for event routing

use super::patterns::EventPattern;
use super::transformations::EventTransformation;

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

impl PartialOrd for RoutingRule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoutingRule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then by pattern specificity
        match other.priority.cmp(&self.priority) {
            std::cmp::Ordering::Equal => {
                // Compare pattern specificity (more specific patterns first)
                let self_specificity = self.pattern.specificity();
                let other_specificity = other.pattern.specificity();
                self_specificity.cmp(&other_specificity).reverse()
            }
            ordering => ordering,
        }
    }
}

impl Eq for RoutingRule {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::integration::config::routing::patterns::EventPattern;

    #[test]
    fn test_routing_rule_creation() {
        let pattern = EventPattern::event_type("user.created");
        let rule = RoutingRule::new(pattern, "user-service".to_string());

        assert_eq!(rule.destination, "user-service");
        assert_eq!(rule.priority, 0);
        assert!(rule.enabled);
        assert!(rule.transformation.is_none());
    }

    #[test]
    fn test_routing_rule_constructors() {
        let rule1 = RoutingRule::for_event_type("order.placed", "order-service".to_string());
        assert_eq!(rule1.pattern.event_type_pattern, Some("order.placed".to_string()));

        let rule2 = RoutingRule::for_source("api-gateway", "auth-service".to_string());
        assert_eq!(rule2.pattern.source_pattern, Some("api-gateway".to_string()));
    }

    #[test]
    fn test_routing_rule_configuration() {
        let rule = RoutingRule::for_event_type("payment.processed", "payment-service".to_string())
            .priority(10)
            .enabled(true);

        assert_eq!(rule.priority, 10);
        assert!(rule.enabled);
    }

    #[test]
    fn test_routing_rule_matching() {
        let rule = RoutingRule::for_event_type("user.login", "auth-service".to_string());

        assert!(rule.matches("user.login", "web-app", None));
        assert!(!rule.matches("user.logout", "web-app", None));
        assert!(!rule.matches("user.login", "web-app", Some("different")));
    }

    #[test]
    fn test_routing_rule_validation() {
        // Valid rule
        let valid_rule = RoutingRule::for_event_type("test.event", "test-dest".to_string());
        assert!(valid_rule.validate().is_ok());

        // Invalid rule - empty destination
        let invalid_rule = RoutingRule::new(EventPattern::event_type("test.event"), "".to_string());
        assert!(invalid_rule.validate().is_err());
    }

    #[test]
    fn test_routing_rule_ordering() {
        let rule1 = RoutingRule::for_event_type("test", "dest1".to_string()).priority(5);
        let rule2 = RoutingRule::for_event_type("test", "dest2".to_string()).priority(10);
        let rule3 = RoutingRule::for_event_type("specific.test", "dest3".to_string()).priority(5);

        assert!(rule2 > rule1); // Higher priority first
        assert!(rule3 > rule1); // Same priority, more specific pattern first
    }

    #[test]
    fn test_routing_rule_display() {
        let rule = RoutingRule::for_event_type("user.created", "user-service".to_string());
        let display = format!("{}", rule);
        assert!(display.contains("user-service"));
        assert!(display.contains("enabled: true"));
    }
}
