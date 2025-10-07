//! Event patterns for routing rules

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

    /// Get pattern specificity (higher = more specific)
    pub fn specificity(&self) -> u32 {
        let mut specificity = 0u32;

        if self.event_type_pattern.is_some() {
            specificity += 10;
        }
        if self.source_pattern.is_some() {
            specificity += 5;
        }
        if self.destination_pattern.is_some() {
            specificity += 5;
        }
        if !self.conditions.is_empty() {
            specificity += self.conditions.len() as u32;
        }

        specificity
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_pattern_creation() {
        let pattern = EventPattern::new();
        assert!(pattern.event_type_pattern.is_none());
        assert!(pattern.source_pattern.is_none());
        assert!(pattern.destination_pattern.is_none());
        assert!(pattern.conditions.is_empty());
    }

    #[test]
    fn test_event_pattern_constructors() {
        let pattern1 = EventPattern::event_type("user.created");
        assert_eq!(pattern1.event_type_pattern, Some("user.created".to_string()));

        let pattern2 = EventPattern::source("web-api");
        assert_eq!(pattern2.source_pattern, Some("web-api".to_string()));
    }

    #[test]
    fn test_event_pattern_building() {
        let pattern = EventPattern::new()
            .with_event_type("order.*")
            .with_source("api-*")
            .with_destination("order-service")
            .with_condition("priority", "high");

        assert_eq!(pattern.event_type_pattern, Some("order.*".to_string()));
        assert_eq!(pattern.source_pattern, Some("api-*".to_string()));
        assert_eq!(pattern.destination_pattern, Some("order-service".to_string()));
        assert_eq!(pattern.conditions["priority"], "high");
    }

    #[test]
    fn test_pattern_matching() {
        let pattern = EventPattern::new()
            .with_event_type("user.*")
            .with_source("web-*");

        // Should match
        assert!(pattern.matches("user.created", "web-api", None));
        assert!(pattern.matches("user.updated", "web-mobile", None));

        // Should not match
        assert!(!pattern.matches("order.created", "web-api", None));
        assert!(!pattern.matches("user.created", "mobile-api", None));
    }

    #[test]
    fn test_wildcard_matching() {
        let pattern = EventPattern::event_type("user.*");

        assert!(pattern.matches_event_type("user.created"));
        assert!(pattern.matches_event_type("user.updated"));
        assert!(!pattern.matches_event_type("order.created"));

        // Test different wildcard patterns
        let contains_pattern = EventPattern::event_type("*test*");
        assert!(contains_pattern.matches_event_type("mytest"));
        assert!(contains_pattern.matches_event_type("testmine"));
        assert!(!contains_pattern.matches_event_type("nomatch"));

        let suffix_pattern = EventPattern::event_type("*ed");
        assert!(suffix_pattern.matches_event_type("created"));
        assert!(!suffix_pattern.matches_event_type("creating"));
    }

    #[test]
    fn test_pattern_validation() {
        // Valid pattern
        let valid_pattern = EventPattern::event_type("user.created");
        assert!(valid_pattern.validate().is_ok());

        // Invalid patterns - empty strings
        let invalid_pattern1 = EventPattern {
            event_type_pattern: Some("".to_string()),
            ..Default::default()
        };
        assert!(invalid_pattern1.validate().is_err());

        let invalid_pattern2 = EventPattern {
            source_pattern: Some("   ".to_string()),
            ..Default::default()
        };
        assert!(invalid_pattern2.validate().is_err());
    }

    #[test]
    fn test_pattern_specificity() {
        let basic_pattern = EventPattern::new();
        assert_eq!(basic_pattern.specificity(), 0);

        let event_pattern = EventPattern::event_type("test");
        assert_eq!(event_pattern.specificity(), 10);

        let complex_pattern = EventPattern::new()
            .with_event_type("test")
            .with_source("src")
            .with_destination("dst")
            .with_condition("key", "value");

        assert_eq!(complex_pattern.specificity(), 21); // 10 + 5 + 5 + 1
    }

    #[test]
    fn test_pattern_display() {
        let pattern = EventPattern::event_type("user.*").with_source("api");
        let display = format!("{}", pattern);
        assert!(display.contains("type:user.*"));
        assert!(display.contains("src:api"));
    }

    #[test]
    fn test_destination_matching() {
        let pattern = EventPattern::new().with_destination("order-*");

        // Should match with destination
        assert!(pattern.matches("event", "source", Some("order-service")));
        assert!(pattern.matches("event", "source", Some("order-queue")));

        // Should not match without destination
        assert!(!pattern.matches("event", "source", None));

        // Should not match wrong destination
        assert!(!pattern.matches("event", "source", Some("payment-service")));
    }

    #[test]
    fn test_pattern_combinations() {
        // Test multiple pattern combinations
        let pattern = EventPattern::new()
            .with_event_type("user.*")
            .with_source("web-*")
            .with_destination("user-service");

        // All match
        assert!(pattern.matches("user.created", "web-api", Some("user-service")));

        // Event type doesn't match
        assert!(!pattern.matches("order.created", "web-api", Some("user-service")));

        // Source doesn't match
        assert!(!pattern.matches("user.created", "mobile-api", Some("user-service")));

        // Destination doesn't match
        assert!(!pattern.matches("user.created", "web-api", Some("auth-service")));
    }
}
