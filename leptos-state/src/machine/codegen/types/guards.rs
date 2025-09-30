//! Guard information for code generation

/// Guard information for code generation
#[derive(Debug, Clone)]
pub struct GuardGenInfo {
    /// Guard ID
    pub id: String,
    /// Guard name
    pub name: String,
    /// Guard type
    pub guard_type: GuardType,
    /// Parameters
    pub parameters: Vec<String>,
    /// Description
    pub description: Option<String>,
}

impl GuardGenInfo {
    /// Create a new guard info
    pub fn new(id: String, name: String, guard_type: GuardType) -> Self {
        Self {
            id,
            name,
            guard_type,
            parameters: Vec::new(),
            description: None,
        }
    }

    /// Add a parameter
    pub fn with_parameter(mut self, param: String) -> Self {
        self.parameters.push(param);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Check if guard has parameters
    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }

    /// Get parameter count
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Get guard signature
    pub fn signature(&self) -> String {
        let params = if self.parameters.is_empty() {
            "()".to_string()
        } else {
            format!("({})", self.parameters.join(", "))
        };
        format!("{}::{} {}", self.guard_type.as_str(), self.name, params)
    }

    /// Check if guard is valid
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty()
    }

    /// Get description or default
    pub fn description(&self) -> &str {
        self.description.as_deref().unwrap_or("")
    }

    /// Get full guard description
    pub fn full_description(&self) -> String {
        let mut desc = format!("Guard '{}' ({})", self.name, self.guard_type.as_str());

        if self.has_parameters() {
            desc.push_str(&format!(" with {} parameters", self.parameter_count()));
        }

        if let Some(ref description) = self.description {
            desc.push_str(&format!(" - {}", description));
        }

        desc
    }
}

impl Default for GuardGenInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            guard_type: GuardType::Function,
            parameters: Vec::new(),
            description: None,
        }
    }
}

/// Guard types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardType {
    /// Function guard
    Function,
    /// Logical AND guard
    And,
    /// Logical OR guard
    Or,
    /// Logical NOT guard
    Not,
    /// Time-based guard
    Temporal,
    /// State-based guard
    State,
    /// Context-based guard
    Context,
    /// Custom guard
    Custom,
}

impl GuardType {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            GuardType::Function => "function",
            GuardType::And => "and",
            GuardType::Or => "or",
            GuardType::Not => "not",
            GuardType::Temporal => "temporal",
            GuardType::State => "state",
            GuardType::Context => "context",
            GuardType::Custom => "custom",
        }
    }

    /// Check if guard type is logical
    pub fn is_logical(&self) -> bool {
        matches!(self, GuardType::And | GuardType::Or | GuardType::Not)
    }

    /// Check if guard type is conditional
    pub fn is_conditional(&self) -> bool {
        matches!(self, GuardType::Function | GuardType::Custom)
    }

    /// Check if guard type requires context
    pub fn requires_context(&self) -> bool {
        matches!(self, GuardType::Context | GuardType::State)
    }
}

impl std::fmt::Display for GuardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Display for GuardGenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_description())
    }
}
