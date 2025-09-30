//! Schema information and validation structures

/// Schema information for machine validation
#[derive(Debug, Clone)]
pub struct SchemaInfo {
    /// Schema version
    pub version: String,
    /// Schema format (e.g., "json-schema", "custom")
    pub format: String,
    /// Schema content (JSON string or custom format)
    pub content: String,
    /// Validation rules
    pub validation_rules: Vec<ValidationRule>,
    /// Whether strict validation is enabled
    pub strict_validation: bool,
}

impl SchemaInfo {
    /// Create new schema info
    pub fn new(version: String, format: String, content: String) -> Self {
        Self {
            version,
            format,
            content,
            validation_rules: Vec::new(),
            strict_validation: false,
        }
    }

    /// Create JSON schema
    pub fn json_schema(version: String, schema_content: String) -> Self {
        Self::new(version, "json-schema".to_string(), schema_content)
    }

    /// Create custom schema
    pub fn custom(version: String, format: String, content: String) -> Self {
        Self::new(version, format, content)
    }

    /// Add a validation rule
    pub fn with_rule(mut self, rule: ValidationRule) -> Self {
        self.validation_rules.push(rule);
        self
    }

    /// Add multiple validation rules
    pub fn with_rules<I>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = ValidationRule>,
    {
        self.validation_rules.extend(rules);
        self
    }

    /// Enable strict validation
    pub fn strict(mut self) -> Self {
        self.strict_validation = true;
        self
    }

    /// Get validation rules by type
    pub fn rules_by_type(&self, validation_type: ValidationType) -> Vec<&ValidationRule> {
        self.validation_rules
            .iter()
            .filter(|rule| rule.validation_type == validation_type)
            .collect()
    }

    /// Check if schema has any rules of given type
    pub fn has_rule_type(&self, validation_type: ValidationType) -> bool {
        self.validation_rules
            .iter()
            .any(|rule| rule.validation_type == validation_type)
    }

    /// Validate data against schema
    pub fn validate(&self, data: &serde_json::Value) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for rule in &self.validation_rules {
            if let Err(error) = rule.validate(data) {
                errors.push(error);
                if self.strict_validation {
                    break; // Stop on first error in strict mode
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get schema summary
    pub fn summary(&self) -> String {
        format!(
            "Schema v{} ({}) with {} rules{}",
            self.version,
            self.format,
            self.validation_rules.len(),
            if self.strict_validation { " (strict)" } else { "" }
        )
    }
}

impl std::fmt::Display for SchemaInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Validation rule for schema
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Validation type
    pub validation_type: ValidationType,
    /// Rule parameters (JSON value)
    pub parameters: serde_json::Value,
    /// Whether the rule is enabled
    pub enabled: bool,
}

impl ValidationRule {
    /// Create a new validation rule
    pub fn new(name: String, validation_type: ValidationType, parameters: serde_json::Value) -> Self {
        Self {
            name,
            description: format!("{} validation rule", name),
            validation_type,
            parameters,
            enabled: true,
        }
    }

    /// Create a required field rule
    pub fn required_field(field: &str) -> Self {
        Self::new(
            format!("required_{}", field),
            ValidationType::Required,
            serde_json::json!({ "field": field }),
        )
    }

    /// Create a type validation rule
    pub fn type_check(field: &str, expected_type: &str) -> Self {
        Self::new(
            format!("type_{}", field),
            ValidationType::Type,
            serde_json::json!({ "field": field, "type": expected_type }),
        )
    }

    /// Create a range validation rule
    pub fn range(field: &str, min: Option<f64>, max: Option<f64>) -> Self {
        let mut params = serde_json::json!({ "field": field });
        if let Some(min_val) = min {
            params["min"] = serde_json::json!(min_val);
        }
        if let Some(max_val) = max {
            params["max"] = serde_json::json!(max_val);
        }

        Self::new(
            format!("range_{}", field),
            ValidationType::Range,
            params,
        )
    }

    /// Create a pattern validation rule
    pub fn pattern(field: &str, pattern: &str) -> Self {
        Self::new(
            format!("pattern_{}", field),
            ValidationType::Pattern,
            serde_json::json!({ "field": field, "pattern": pattern }),
        )
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Enable or disable the rule
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Validate data against this rule
    pub fn validate(&self, data: &serde_json::Value) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        match self.validation_type {
            ValidationType::Required => self.validate_required(data),
            ValidationType::Type => self.validate_type(data),
            ValidationType::Range => self.validate_range(data),
            ValidationType::Pattern => self.validate_pattern(data),
            ValidationType::Custom => self.validate_custom(data),
        }
    }

    /// Validate required field
    fn validate_required(&self, data: &serde_json::Value) -> Result<(), String> {
        if let Some(field) = self.parameters.get("field").and_then(|v| v.as_str()) {
            if data.get(field).is_none() {
                return Err(format!("Required field '{}' is missing", field));
            }
        }
        Ok(())
    }

    /// Validate type
    fn validate_type(&self, data: &serde_json::Value) -> Result<(), String> {
        if let (Some(field), Some(expected_type)) = (
            self.parameters.get("field").and_then(|v| v.as_str()),
            self.parameters.get("type").and_then(|v| v.as_str()),
        ) {
            if let Some(value) = data.get(field) {
                let actual_type = match value {
                    serde_json::Value::Null => "null",
                    serde_json::Value::Bool(_) => "boolean",
                    serde_json::Value::Number(_) => "number",
                    serde_json::Value::String(_) => "string",
                    serde_json::Value::Array(_) => "array",
                    serde_json::Value::Object(_) => "object",
                };

                if actual_type != expected_type {
                    return Err(format!(
                        "Field '{}' has type '{}', expected '{}'",
                        field, actual_type, expected_type
                    ));
                }
            }
        }
        Ok(())
    }

    /// Validate range
    fn validate_range(&self, data: &serde_json::Value) -> Result<(), String> {
        if let Some(field) = self.parameters.get("field").and_then(|v| v.as_str()) {
            if let Some(value) = data.get(field) {
                if let Some(num) = value.as_f64() {
                    if let Some(min) = self.parameters.get("min").and_then(|v| v.as_f64()) {
                        if num < min {
                            return Err(format!("Field '{}' value {} is below minimum {}", field, num, min));
                        }
                    }
                    if let Some(max) = self.parameters.get("max").and_then(|v| v.as_f64()) {
                        if num > max {
                            return Err(format!("Field '{}' value {} is above maximum {}", field, num, max));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate pattern
    fn validate_pattern(&self, data: &serde_json::Value) -> Result<(), String> {
        if let (Some(field), Some(pattern)) = (
            self.parameters.get("field").and_then(|v| v.as_str()),
            self.parameters.get("pattern").and_then(|v| v.as_str()),
        ) {
            if let Some(value) = data.get(field) {
                if let Some(str_val) = value.as_str() {
                    // Simple pattern matching (could be enhanced with regex)
                    if !str_val.contains(pattern) {
                        return Err(format!("Field '{}' value '{}' does not match pattern '{}'", field, str_val, pattern));
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate custom rule (placeholder)
    fn validate_custom(&self, _data: &serde_json::Value) -> Result<(), String> {
        // Custom validation logic would be implemented here
        Ok(())
    }

    /// Get rule summary
    pub fn summary(&self) -> String {
        format!(
            "ValidationRule '{}' ({}) - {}{}",
            self.name,
            self.validation_type.as_str(),
            self.description,
            if self.enabled { "" } else { " (disabled)" }
        )
    }
}

impl std::fmt::Display for ValidationRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Validation types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationType {
    /// Required field validation
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

impl ValidationType {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Type => "type",
            Self::Range => "range",
            Self::Pattern => "pattern",
            Self::Custom => "custom",
        }
    }
}

impl std::fmt::Display for ValidationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
