//! Type aliases and identifiers for configuration

/// Type alias for store identifiers
pub type StoreId = String;

/// Type alias for machine identifiers
pub type MachineId = String;

/// Type alias for state identifiers
pub type StateId = String;

/// Type alias for event identifiers
pub type EventId = String;

/// Type alias for action identifiers
pub type ActionId = String;

/// Type alias for transition identifiers
pub type TransitionId = String;

/// Type alias for guard identifiers
pub type GuardId = String;

/// Configuration key type
pub type ConfigKey = String;

/// Configuration value type
pub type ConfigValue = serde_json::Value;

/// Configuration map type
pub type ConfigMap = std::collections::HashMap<ConfigKey, ConfigValue>;

/// Identifier validation
pub trait Identifier {
    /// Validate the identifier format
    fn validate(&self) -> Result<(), String>;

    /// Check if the identifier is empty
    fn is_empty(&self) -> bool;

    /// Get the identifier as a string
    fn as_str(&self) -> &str;
}

impl Identifier for StoreId {
    fn validate(&self) -> Result<(), String> {
        if self.trim().is_empty() {
            return Err("Store ID cannot be empty".to_string());
        }
        if self.len() > 255 {
            return Err("Store ID too long (max 255 characters)".to_string());
        }
        // Check for invalid characters
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\0'];
        for &ch in &invalid_chars {
            if self.contains(ch) {
                return Err(format!("Store ID contains invalid character: {}", ch));
            }
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.trim().is_empty()
    }

    fn as_str(&self) -> &str {
        self
    }
}

impl Identifier for MachineId {
    fn validate(&self) -> Result<(), String> {
        if self.trim().is_empty() {
            return Err("Machine ID cannot be empty".to_string());
        }
        if self.len() > 255 {
            return Err("Machine ID too long (max 255 characters)".to_string());
        }
        // Check for invalid characters
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\0'];
        for &ch in &invalid_chars {
            if self.contains(ch) {
                return Err(format!("Machine ID contains invalid character: {}", ch));
            }
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.trim().is_empty()
    }

    fn as_str(&self) -> &str {
        self
    }
}

impl Identifier for StateId {
    fn validate(&self) -> Result<(), String> {
        if self.trim().is_empty() {
            return Err("State ID cannot be empty".to_string());
        }
        if self.len() > 100 {
            return Err("State ID too long (max 100 characters)".to_string());
        }
        // Allow more characters for state IDs as they can be descriptive
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.trim().is_empty()
    }

    fn as_str(&self) -> &str {
        self
    }
}

impl Identifier for EventId {
    fn validate(&self) -> Result<(), String> {
        if self.trim().is_empty() {
            return Err("Event ID cannot be empty".to_string());
        }
        if self.len() > 100 {
            return Err("Event ID too long (max 100 characters)".to_string());
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.trim().is_empty()
    }

    fn as_str(&self) -> &str {
        self
    }
}

/// Identifier utilities
pub struct IdentifierUtils;

impl IdentifierUtils {
    /// Generate a unique identifier with prefix
    pub fn generate_unique(prefix: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let random_part = (rand::random::<u32>() % 10000).to_string();
        format!("{}_{}_{}", prefix, timestamp, random_part)
    }

    /// Sanitize an identifier for filesystem use
    pub fn sanitize_for_filesystem(id: &str) -> String {
        id.chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0'..='\x1f' => '_',
                c => c,
            })
            .collect()
    }

    /// Check if two identifiers are equal (case-insensitive for some types)
    pub fn equals_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }

    /// Validate a collection of identifiers
    pub fn validate_collection<T: Identifier>(ids: &[T]) -> Result<(), String> {
        for (index, id) in ids.iter().enumerate() {
            if let Err(error) = id.validate() {
                return Err(format!("ID at index {}: {}", index, error));
            }
        }

        // Check for duplicates
        let mut seen = std::collections::HashSet::new();
        for id in ids {
            if !seen.insert(id.as_str()) {
                return Err(format!("Duplicate ID: {}", id.as_str()));
            }
        }

        Ok(())
    }

    /// Find duplicate identifiers in a collection
    pub fn find_duplicates<T: Identifier>(ids: &[T]) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();

        for id in ids {
            if !seen.insert(id.as_str()) {
                duplicates.push(id.as_str().to_string());
            }
        }

        duplicates
    }

    /// Group identifiers by prefix
    pub fn group_by_prefix<T: Identifier>(ids: &[T], separator: char) -> std::collections::HashMap<String, Vec<String>> {
        let mut groups = std::collections::HashMap::new();

        for id in ids {
            let parts: Vec<&str> = id.as_str().split(separator).collect();
            if let Some(prefix) = parts.first() {
                groups.entry(prefix.to_string())
                    .or_insert_with(Vec::new)
                    .push(id.as_str().to_string());
            }
        }

        groups
    }
}

/// Identifier formatting utilities
pub mod formatting {
    use super::*;

    /// Format an identifier as a valid Rust identifier
    pub fn as_rust_identifier(id: &str) -> String {
        let mut result = String::new();

        for (i, ch) in id.chars().enumerate() {
            if i == 0 {
                // First character must be a letter or underscore
                if ch.is_alphabetic() || ch == '_' {
                    result.push(ch);
                } else {
                    result.push('_');
                }
            } else {
                // Subsequent characters can be letters, digits, or underscores
                if ch.is_alphanumeric() || ch == '_' {
                    result.push(ch);
                } else {
                    result.push('_');
                }
            }
        }

        if result.is_empty() {
            "identifier".to_string()
        } else {
            result
        }
    }

    /// Format an identifier as a valid TypeScript identifier
    pub fn as_typescript_identifier(id: &str) -> String {
        as_rust_identifier(id) // Similar rules apply
    }

    /// Format an identifier as a valid Python identifier
    pub fn as_python_identifier(id: &str) -> String {
        let mut result = String::new();

        for (i, ch) in id.chars().enumerate() {
            if i == 0 {
                if ch.is_alphabetic() || ch == '_' {
                    result.push(ch.to_ascii_lowercase());
                } else {
                    result.push('_');
                }
            } else {
                if ch.is_alphanumeric() || ch == '_' {
                    result.push(ch.to_ascii_lowercase());
                } else {
                    result.push('_');
                }
            }
        }

        if result.is_empty() {
            "identifier".to_string()
        } else {
            result
        }
    }

    /// Format an identifier as a valid Java identifier
    pub fn as_java_identifier(id: &str) -> String {
        let rust_identifier = as_rust_identifier(id);
        // Java has the same rules as Rust for identifiers
        rust_identifier
    }

    /// Format an identifier as uppercase constant
    pub fn as_constant(id: &str) -> String {
        id.chars()
            .map(|c| if c.is_alphanumeric() { c.to_ascii_uppercase() } else { '_' })
            .collect()
    }

    /// Format an identifier as camelCase
    pub fn as_camel_case(id: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;

        for ch in id.chars() {
            if ch == '_' || ch == '-' || ch == ' ' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(ch.to_ascii_uppercase());
                capitalize_next = false;
            } else if result.is_empty() {
                result.push(ch.to_ascii_lowercase());
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Format an identifier as PascalCase
    pub fn as_pascal_case(id: &str) -> String {
        let camel = as_camel_case(id);
        if let Some(first) = camel.chars().next() {
            first.to_ascii_uppercase().to_string() + &camel[1..]
        } else {
            camel
        }
    }

    /// Format an identifier as snake_case
    pub fn as_snake_case(id: &str) -> String {
        id.chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_ascii_lowercase())
                } else {
                    c.to_ascii_lowercase().to_string()
                }
            })
            .collect()
    }
}
