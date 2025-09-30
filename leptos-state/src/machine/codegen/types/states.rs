//! State information for code generation

/// State information for code generation
#[derive(Debug, Clone)]
pub struct StateGenInfo {
    /// State ID
    pub id: String,
    /// State name
    pub name: String,
    /// State type
    pub state_type: String,
    /// Parent state ID (if hierarchical)
    pub parent_id: Option<String>,
    /// Child state IDs
    pub child_states: Vec<String>,
    /// Initial child state ID
    pub initial_child: Option<String>,
    /// Entry actions
    pub entry_actions: Vec<String>,
    /// Exit actions
    pub exit_actions: Vec<String>,
    /// State metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl StateGenInfo {
    /// Create a new state info
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            state_type: "simple".to_string(),
            parent_id: None,
            child_states: Vec::new(),
            initial_child: None,
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a compound state
    pub fn compound(id: String, name: String) -> Self {
        Self {
            id,
            name,
            state_type: "compound".to_string(),
            parent_id: None,
            child_states: Vec::new(),
            initial_child: None,
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a parallel state
    pub fn parallel(id: String, name: String) -> Self {
        Self {
            id,
            name,
            state_type: "parallel".to_string(),
            parent_id: None,
            child_states: Vec::new(),
            initial_child: None,
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set parent state
    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Add a child state
    pub fn with_child(mut self, child_id: String) -> Self {
        self.child_states.push(child_id);
        self
    }

    /// Set initial child
    pub fn with_initial_child(mut self, initial_child: String) -> Self {
        self.initial_child = Some(initial_child);
        self
    }

    /// Add entry action
    pub fn with_entry_action(mut self, action: String) -> Self {
        self.entry_actions.push(action);
        self
    }

    /// Add exit action
    pub fn with_exit_action(mut self, action: String) -> Self {
        self.exit_actions.push(action);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if this is a compound state
    pub fn is_compound(&self) -> bool {
        self.state_type == "compound" || !self.child_states.is_empty()
    }

    /// Check if this is a parallel state
    pub fn is_parallel(&self) -> bool {
        self.state_type == "parallel"
    }

    /// Check if this state has children
    pub fn has_children(&self) -> bool {
        !self.child_states.is_empty()
    }

    /// Check if this state has a parent
    pub fn has_parent(&self) -> bool {
        self.parent_id.is_some()
    }

    /// Check if this state has entry actions
    pub fn has_entry_actions(&self) -> bool {
        !self.entry_actions.is_empty()
    }

    /// Check if this state has exit actions
    pub fn has_exit_actions(&self) -> bool {
        !self.exit_actions.is_empty()
    }

    /// Get child count
    pub fn child_count(&self) -> usize {
        self.child_states.len()
    }

    /// Get entry action count
    pub fn entry_action_count(&self) -> usize {
        self.entry_actions.len()
    }

    /// Get exit action count
    pub fn exit_action_count(&self) -> usize {
        self.exit_actions.len()
    }

    /// Get full state path (including parent hierarchy)
    pub fn full_path(&self) -> String {
        if let Some(ref parent) = self.parent_id {
            format!("{}.{}", parent, self.id)
        } else {
            self.id.clone()
        }
    }

    /// Get state depth in hierarchy
    pub fn depth(&self) -> usize {
        self.parent_id.as_ref().map_or(0, |_| 1)
    }

    /// Check if state is valid
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty()
    }

    /// Get state description
    pub fn description(&self) -> String {
        let mut desc = format!("State '{}' ({})", self.name, self.state_type);

        if self.has_children() {
            desc.push_str(&format!(" with {} children", self.child_count()));
        }

        if self.has_entry_actions() {
            desc.push_str(&format!(" + {} entry actions", self.entry_action_count()));
        }

        if self.has_exit_actions() {
            desc.push_str(&format!(" + {} exit actions", self.exit_action_count()));
        }

        desc
    }
}

impl Default for StateGenInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            state_type: "simple".to_string(),
            parent_id: None,
            child_states: Vec::new(),
            initial_child: None,
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl std::fmt::Display for StateGenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
