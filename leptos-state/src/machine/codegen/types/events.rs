//! Event information for code generation

/// Event information for code generation
#[derive(Debug, Clone)]
pub struct EventGenInfo {
    /// Event ID
    pub id: String,
    /// Event name
    pub name: String,
    /// Event data type
    pub data_type: Option<String>,
    /// Whether this is a built-in event
    pub built_in: bool,
    /// Event description
    pub description: Option<String>,
}

impl EventGenInfo {
    /// Create a new event info
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            data_type: None,
            built_in: false,
            description: None,
        }
    }

    /// Create a built-in event
    pub fn built_in(id: String, name: String) -> Self {
        Self {
            id,
            name,
            data_type: None,
            built_in: true,
            description: None,
        }
    }

    /// Set data type
    pub fn with_data_type(mut self, data_type: String) -> Self {
        self.data_type = Some(data_type);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Check if event has data
    pub fn has_data(&self) -> bool {
        self.data_type.is_some()
    }

    /// Get data type or default
    pub fn data_type(&self) -> &str {
        self.data_type.as_deref().unwrap_or("()")
    }

    /// Get description or default
    pub fn description(&self) -> &str {
        self.description.as_deref().unwrap_or("")
    }

    /// Check if event is valid
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty()
    }

    /// Get event signature for code generation
    pub fn signature(&self) -> String {
        format!("{}({})", self.name, self.data_type())
    }

    /// Get full event description
    pub fn full_description(&self) -> String {
        let mut desc = format!("Event '{}'", self.name);

        if let Some(ref data_type) = self.data_type {
            desc.push_str(&format!(" with data: {}", data_type));
        }

        if let Some(ref description) = self.description {
            desc.push_str(&format!(" - {}", description));
        }

        if self.built_in {
            desc.push_str(" (built-in)");
        }

        desc
    }
}

impl Default for EventGenInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            data_type: None,
            built_in: false,
            description: None,
        }
    }
}

impl std::fmt::Display for EventGenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_description())
    }
}
