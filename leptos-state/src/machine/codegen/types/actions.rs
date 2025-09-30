//! Action information for code generation

/// Action information for code generation
#[derive(Debug, Clone)]
pub struct ActionGenInfo {
    /// Action ID
    pub id: String,
    /// Action name
    pub name: String,
    /// Action type
    pub action_type: ActionType,
    /// Parameters
    pub parameters: Vec<String>,
    /// Description
    pub description: Option<String>,
}

impl ActionGenInfo {
    /// Create a new action info
    pub fn new(id: String, name: String, action_type: ActionType) -> Self {
        Self {
            id,
            name,
            action_type,
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

    /// Check if action has parameters
    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }

    /// Get parameter count
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Get action signature
    pub fn signature(&self) -> String {
        let params = if self.parameters.is_empty() {
            "()".to_string()
        } else {
            format!("({})", self.parameters.join(", "))
        };
        format!("{}::{} {}", self.action_type.as_str(), self.name, params)
    }

    /// Check if action is valid
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty()
    }

    /// Get description or default
    pub fn description(&self) -> &str {
        self.description.as_deref().unwrap_or("")
    }

    /// Get full action description
    pub fn full_description(&self) -> String {
        let mut desc = format!("Action '{}' ({})", self.name, self.action_type.as_str());

        if self.has_parameters() {
            desc.push_str(&format!(" with {} parameters", self.parameter_count()));
        }

        if let Some(ref description) = self.description {
            desc.push_str(&format!(" - {}", description));
        }

        desc
    }
}

impl Default for ActionGenInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            action_type: ActionType::Function,
            parameters: Vec::new(),
            description: None,
        }
    }
}

/// Action types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    /// Function action
    Function,
    /// Assignment action
    Assign,
    /// Log action
    Log,
    /// Send action
    Send,
    /// Raise action
    Raise,
    /// Cancel action
    Cancel,
    /// Start action
    Start,
    /// Stop action
    Stop,
    /// Custom action
    Custom,
}

impl ActionType {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionType::Function => "function",
            ActionType::Assign => "assign",
            ActionType::Log => "log",
            ActionType::Send => "send",
            ActionType::Raise => "raise",
            ActionType::Cancel => "cancel",
            ActionType::Start => "start",
            ActionType::Stop => "stop",
            ActionType::Custom => "custom",
        }
    }

    /// Check if action is built-in
    pub fn is_builtin(&self) -> bool {
        !matches!(self, ActionType::Function | ActionType::Custom)
    }

    /// Check if action modifies state
    pub fn modifies_state(&self) -> bool {
        matches!(self, ActionType::Assign | ActionType::Send | ActionType::Raise)
    }

    /// Check if action is a control action
    pub fn is_control(&self) -> bool {
        matches!(self, ActionType::Start | ActionType::Stop | ActionType::Cancel)
    }
}

impl std::fmt::Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Display for ActionGenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_description())
    }
}
