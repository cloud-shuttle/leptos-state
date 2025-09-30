//! Core serialization structures and data models

use crate::machine::persistence_metadata::MachineMetadata;

/// Serialized state machine data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializedMachine<C, E, S> {
    /// Machine format version
    pub version: u32,
    /// Machine ID
    pub id: String,
    /// Machine states
    pub states: Vec<SerializedState<C>>,
    /// Machine initial state
    pub initial_state: String,
    /// Machine transitions
    pub transitions: Vec<SerializedTransition<E>>,
    /// Machine context
    pub context: Option<C>,
    /// Machine current state
    pub current_state: String,
    /// Machine metadata
    pub metadata: MachineMetadata,
    /// Serialization timestamp
    pub timestamp: u64,
}

impl<C, E, S> SerializedMachine<C, E, S> {
    /// Create a new serialized machine
    pub fn new() -> Self {
        Self {
            version: 1,
            id: String::new(),
            states: Vec::new(),
            initial_state: String::new(),
            transitions: Vec::new(),
            context: None,
            current_state: String::new(),
            metadata: MachineMetadata::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Set machine ID
    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    /// Add a state
    pub fn with_state(mut self, state: SerializedState<C>) -> Self {
        self.states.push(state);
        self
    }

    /// Set initial state
    pub fn with_initial_state(mut self, initial_state: String) -> Self {
        self.initial_state = initial_state;
        self
    }

    /// Add a transition
    pub fn with_transition(mut self, transition: SerializedTransition<E>) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Set context
    pub fn with_context(mut self, context: C) -> Self {
        self.context = Some(context);
        self
    }

    /// Set current state
    pub fn with_current_state(mut self, current_state: String) -> Self {
        self.current_state = current_state;
        self
    }

    /// Set metadata
    pub fn with_metadata(mut self, metadata: MachineMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Set version
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Get state by ID
    pub fn get_state(&self, id: &str) -> Option<&SerializedState<C>> {
        self.states.iter().find(|s| s.id == id)
    }

    /// Get transition by index
    pub fn get_transition(&self, index: usize) -> Option<&SerializedTransition<E>> {
        self.transitions.get(index)
    }

    /// Get all state IDs
    pub fn state_ids(&self) -> Vec<String> {
        self.states.iter().map(|s| s.id.clone()).collect()
    }

    /// Check if machine has states
    pub fn has_states(&self) -> bool {
        !self.states.is_empty()
    }

    /// Check if machine has transitions
    pub fn has_transitions(&self) -> bool {
        !self.transitions.is_empty()
    }

    /// Get state count
    pub fn state_count(&self) -> usize {
        self.states.len()
    }

    /// Get transition count
    pub fn transition_count(&self) -> usize {
        self.transitions.len()
    }
}

impl<C, E, S> Default for SerializedMachine<C, E, S> {
    fn default() -> Self {
        Self::new()
    }
}

/// Serialized state data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializedState<C> {
    /// State ID
    pub id: String,
    /// State type
    pub state_type: StateType,
    /// State context (if any)
    pub context: Option<C>,
    /// Child states (for compound states)
    pub child_states: Vec<String>,
    /// Initial child state (for compound states)
    pub initial_child: Option<String>,
    /// Entry actions
    pub entry_actions: Vec<String>,
    /// Exit actions
    pub exit_actions: Vec<String>,
}

impl<C> SerializedState<C> {
    /// Create a new serialized state
    pub fn new(id: String, state_type: StateType) -> Self {
        Self {
            id,
            state_type,
            context: None,
            child_states: Vec::new(),
            initial_child: None,
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
        }
    }

    /// Create a simple state
    pub fn simple(id: String) -> Self {
        Self::new(id, StateType::Simple)
    }

    /// Create a compound state
    pub fn compound(id: String) -> Self {
        Self::new(id, StateType::Compound)
    }

    /// Create a parallel state
    pub fn parallel(id: String) -> Self {
        Self::new(id, StateType::Parallel)
    }

    /// Set context
    pub fn with_context(mut self, context: C) -> Self {
        self.context = Some(context);
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

    /// Check if this is a compound state
    pub fn is_compound(&self) -> bool {
        matches!(self.state_type, StateType::Compound)
    }

    /// Check if this is a parallel state
    pub fn is_parallel(&self) -> bool {
        matches!(self.state_type, StateType::Parallel)
    }

    /// Check if this state has children
    pub fn has_children(&self) -> bool {
        !self.child_states.is_empty()
    }

    /// Get child count
    pub fn child_count(&self) -> usize {
        self.child_states.len()
    }
}

/// Serialized transition data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializedTransition<E> {
    /// Transition event
    pub event: E,
    /// Source state ID
    pub source: String,
    /// Target state ID
    pub target: String,
    /// Guard conditions
    pub guards: Vec<String>,
    /// Transition actions
    pub actions: Vec<String>,
}

impl<E> SerializedTransition<E> {
    /// Create a new serialized transition
    pub fn new(event: E, source: String, target: String) -> Self {
        Self {
            event,
            source,
            target,
            guards: Vec::new(),
            actions: Vec::new(),
        }
    }

    /// Add a guard
    pub fn with_guard(mut self, guard: String) -> Self {
        self.guards.push(guard);
        self
    }

    /// Add an action
    pub fn with_action(mut self, action: String) -> Self {
        self.actions.push(action);
        self
    }

    /// Check if transition has guards
    pub fn has_guards(&self) -> bool {
        !self.guards.is_empty()
    }

    /// Check if transition has actions
    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Get guard count
    pub fn guard_count(&self) -> usize {
        self.guards.len()
    }

    /// Get action count
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }
}

/// State types
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StateType {
    /// Simple state with no children
    Simple,
    /// Compound state with child states
    Compound,
    /// Parallel state with concurrent regions
    Parallel,
}

impl StateType {
    /// Check if this state type can have children
    pub fn can_have_children(&self) -> bool {
        matches!(self, StateType::Compound | StateType::Parallel)
    }

    /// Get a string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            StateType::Simple => "simple",
            StateType::Compound => "compound",
            StateType::Parallel => "parallel",
        }
    }
}

impl std::fmt::Display for StateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
