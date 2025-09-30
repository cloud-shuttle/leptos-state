//! Machine information for code generation

/// Machine information for code generation
#[derive(Debug, Clone)]
pub struct MachineGenInfo {
    /// Machine ID
    pub id: String,
    /// Machine name
    pub name: String,
    /// Machine type
    pub machine_type: MachineType,
    /// Initial state ID
    pub initial_state: String,
    /// State information
    pub states: Vec<super::states::StateGenInfo>,
    /// Event information
    pub events: Vec<super::events::EventGenInfo>,
    /// Transition information
    pub transitions: Vec<super::transitions::TransitionInfo>,
    /// Context type
    pub context_type: Option<String>,
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl MachineGenInfo {
    /// Create a new machine info
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            machine_type: MachineType::Simple,
            initial_state: String::new(),
            states: Vec::new(),
            events: Vec::new(),
            transitions: Vec::new(),
            context_type: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set machine type
    pub fn with_type(mut self, machine_type: MachineType) -> Self {
        self.machine_type = machine_type;
        self
    }

    /// Set initial state
    pub fn with_initial_state(mut self, initial_state: String) -> Self {
        self.initial_state = initial_state;
        self
    }

    /// Add a state
    pub fn with_state(mut self, state: super::states::StateGenInfo) -> Self {
        self.states.push(state);
        self
    }

    /// Add an event
    pub fn with_event(mut self, event: super::events::EventGenInfo) -> Self {
        self.events.push(event);
        self
    }

    /// Add a transition
    pub fn with_transition(mut self, transition: super::transitions::TransitionInfo) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Set context type
    pub fn with_context_type(mut self, context_type: String) -> Self {
        self.context_type = Some(context_type);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get state by ID
    pub fn get_state(&self, id: &str) -> Option<&super::states::StateGenInfo> {
        self.states.iter().find(|s| s.id == id)
    }

    /// Get event by ID
    pub fn get_event(&self, id: &str) -> Option<&super::events::EventGenInfo> {
        self.events.iter().find(|e| e.id == id)
    }

    /// Get transitions from a state
    pub fn get_transitions_from(&self, state_id: &str) -> Vec<&super::transitions::TransitionInfo> {
        self.transitions.iter()
            .filter(|t| t.source_state == state_id)
            .collect()
    }

    /// Get all state IDs
    pub fn state_ids(&self) -> Vec<String> {
        self.states.iter().map(|s| s.id.clone()).collect()
    }

    /// Get all event IDs
    pub fn event_ids(&self) -> Vec<String> {
        self.events.iter().map(|e| e.id.clone()).collect()
    }

    /// Check if machine has states
    pub fn has_states(&self) -> bool {
        !self.states.is_empty()
    }

    /// Check if machine has events
    pub fn has_events(&self) -> bool {
        !self.events.is_empty()
    }

    /// Check if machine has transitions
    pub fn has_transitions(&self) -> bool {
        !self.transitions.is_empty()
    }

    /// Get state count
    pub fn state_count(&self) -> usize {
        self.states.len()
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Get transition count
    pub fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    /// Check if machine is hierarchical
    pub fn is_hierarchical(&self) -> bool {
        self.states.iter().any(|s| s.has_children())
    }

    /// Check if machine has parallel states
    pub fn has_parallel_states(&self) -> bool {
        self.states.iter().any(|s| s.is_parallel())
    }

    /// Get maximum state depth
    pub fn max_depth(&self) -> usize {
        self.states.iter()
            .map(|s| s.depth())
            .max()
            .unwrap_or(0)
    }

    /// Check if machine is valid
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() &&
        !self.name.is_empty() &&
        !self.initial_state.is_empty() &&
        self.has_states()
    }

    /// Get machine description
    pub fn description(&self) -> String {
        let mut desc = format!("Machine '{}' ({})", self.name, self.machine_type.as_str());

        desc.push_str(&format!(" with {} states", self.state_count()));

        if self.has_events() {
            desc.push_str(&format!(", {} events", self.event_count()));
        }

        if self.has_transitions() {
            desc.push_str(&format!(", {} transitions", self.transition_count()));
        }

        if self.is_hierarchical() {
            desc.push_str(" (hierarchical)");
        }

        if self.has_parallel_states() {
            desc.push_str(" (parallel)");
        }

        desc
    }
}

/// Machine types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineType {
    /// Simple state machine
    Simple,
    /// Hierarchical state machine
    Hierarchical,
    /// Parallel state machine
    Parallel,
}

impl MachineType {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            MachineType::Simple => "simple",
            MachineType::Hierarchical => "hierarchical",
            MachineType::Parallel => "parallel",
        }
    }

    /// Check if machine type supports hierarchy
    pub fn supports_hierarchy(&self) -> bool {
        matches!(self, MachineType::Hierarchical)
    }

    /// Check if machine type supports parallelism
    pub fn supports_parallelism(&self) -> bool {
        matches!(self, MachineType::Parallel)
    }
}

impl std::fmt::Display for MachineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for MachineGenInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            machine_type: MachineType::Simple,
            initial_state: String::new(),
            states: Vec::new(),
            events: Vec::new(),
            transitions: Vec::new(),
            context_type: None,
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl std::fmt::Display for MachineGenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
