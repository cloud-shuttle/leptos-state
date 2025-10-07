//! Event transformations for routing rules

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

    /// Apply transformation to an event (placeholder for future implementation)
    pub fn apply(&self, event: &mut serde_json::Value) -> Result<(), String> {
        // Remove fields
        if let Some(obj) = event.as_object_mut() {
            for field in &self.remove_fields {
                obj.remove(field);
            }

            // Rename fields
            for (from, to) in &self.rename_fields {
                if let Some(value) = obj.remove(from) {
                    obj.insert(to.clone(), value);
                }
            }

            // Add fields
            for (key, value) in &self.add_fields {
                obj.insert(key.clone(), value.clone());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_transformation_creation() {
        let transform = EventTransformation::new();
        assert!(transform.add_fields.is_empty());
        assert!(transform.remove_fields.is_empty());
        assert!(transform.rename_fields.is_empty());
        assert!(!transform.preserve_original);
    }

    #[test]
    fn test_event_transformation_building() {
        let transform = EventTransformation::new()
            .add_field("processed_at", "2024-01-01T00:00:00Z")
            .add_field("version", "1.0")
            .remove_field("internal_id")
            .rename_field("user_id", "customer_id")
            .preserve_original();

        assert_eq!(transform.add_fields.len(), 2);
        assert_eq!(transform.remove_fields, vec!["internal_id"]);
        assert_eq!(transform.rename_fields["user_id"], "customer_id");
        assert!(transform.preserve_original);
    }

    #[test]
    fn test_transformation_validation() {
        // Valid transformation
        let valid_transform = EventTransformation::new()
            .add_field("new_field", "value")
            .remove_field("old_field")
            .rename_field("rename_me", "renamed");
        assert!(valid_transform.validate().is_ok());

        // Invalid: add and remove same field
        let invalid_transform1 = EventTransformation::new()
            .add_field("conflict", "value")
            .remove_field("conflict");
        assert!(invalid_transform1.validate().is_err());

        // Invalid: remove and rename same field
        let invalid_transform2 = EventTransformation::new()
            .remove_field("conflict")
            .rename_field("conflict", "new_name");
        assert!(invalid_transform2.validate().is_err());

        // Invalid: add and rename from same field
        let invalid_transform3 = EventTransformation::new()
            .add_field("conflict", "value")
            .rename_field("conflict", "new_name");
        assert!(invalid_transform3.validate().is_err());

        // Invalid: rename to field that's being added
        let invalid_transform4 = EventTransformation::new()
            .add_field("new_name", "value")
            .rename_field("old_name", "new_name");
        assert!(invalid_transform4.validate().is_err());
    }

    #[test]
    fn test_transformation_application() {
        let mut event = serde_json::json!({
            "user_id": 123,
            "action": "login",
            "internal_id": "secret123",
            "timestamp": "2024-01-01"
        });

        let transform = EventTransformation::new()
            .add_field("processed", true)
            .add_field("version", "2.0")
            .remove_field("internal_id")
            .rename_field("user_id", "customer_id");

        let result = transform.apply(&mut event);
        assert!(result.is_ok());

        let obj = event.as_object().unwrap();
        assert_eq!(obj["customer_id"], 123); // Renamed
        assert!(!obj.contains_key("user_id")); // Old field removed
        assert!(!obj.contains_key("internal_id")); // Removed
        assert_eq!(obj["processed"], true); // Added
        assert_eq!(obj["version"], "2.0"); // Added
        assert_eq!(obj["action"], "login"); // Unchanged
        assert_eq!(obj["timestamp"], "2024-01-01"); // Unchanged
    }

    #[test]
    fn test_transformation_summary() {
        let transform = EventTransformation::new()
            .add_field("field1", "value1")
            .add_field("field2", "value2")
            .remove_field("old_field")
            .rename_field("old_name", "new_name");

        let summary = transform.summary();
        assert!(summary.contains("add:2"));
        assert!(summary.contains("remove:1"));
        assert!(summary.contains("rename:1"));
    }

    #[test]
    fn test_empty_transformation_summary() {
        let transform = EventTransformation::new();
        assert_eq!(transform.summary(), "no-op");
    }

    #[test]
    fn test_transformation_display() {
        let transform = EventTransformation::new().add_field("test", "value");
        let display = format!("{}", transform);
        assert!(display.starts_with("EventTransformation("));
        assert!(display.contains("add:1"));
    }

    #[test]
    fn test_transformation_preserves_non_objects() {
        // Test with non-object JSON (should not modify)
        let mut event = serde_json::json!("just a string");
        let transform = EventTransformation::new().add_field("test", "value");

        let result = transform.apply(&mut event);
        assert!(result.is_ok());
        assert_eq!(event, serde_json::json!("just a string"));
    }

    #[test]
    fn test_complex_transformation_chain() {
        let mut event = serde_json::json!({
            "id": "123",
            "type": "user_event",
            "user": {
                "name": "John",
                "email": "john@example.com"
            },
            "metadata": {
                "source": "web",
                "version": "1.0"
            }
        });

        let transform = EventTransformation::new()
            .add_field("processed_at", "2024-01-01T12:00:00Z")
            .add_field("environment", "production")
            .remove_field("metadata")
            .rename_field("user", "customer")
            .rename_field("type", "event_type");

        let result = transform.apply(&mut event);
        assert!(result.is_ok());

        let obj = event.as_object().unwrap();
        assert!(obj.contains_key("processed_at"));
        assert!(obj.contains_key("environment"));
        assert!(obj.contains_key("customer"));
        assert!(obj.contains_key("event_type"));
        assert!(!obj.contains_key("metadata"));
        assert!(!obj.contains_key("user"));
        assert!(!obj.contains_key("type"));
        assert_eq!(obj["id"], "123"); // Unchanged
    }
}
