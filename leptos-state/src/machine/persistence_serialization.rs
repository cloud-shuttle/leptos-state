//! Serialization and deserialization for state machines

use super::*;
use super::persistence_core::PersistenceError;

/// Serialized state machine data
#[derive(Debug, Clone)]
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
    pub fn add_state(mut self, state: SerializedState<C>) -> Self {
        self.states.push(state);
        self
    }

    /// Set initial state
    pub fn initial_state(mut self, initial: String) -> Self {
        self.initial_state = initial;
        self
    }

    /// Add a transition
    pub fn add_transition(mut self, transition: SerializedTransition<E>) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Set context
    pub fn context(mut self, context: C) -> Self {
        self.context = Some(context);
        self
    }

    /// Set current state
    pub fn current_state(mut self, current: String) -> Self {
        self.current_state = current;
        self
    }

    /// Set metadata
    pub fn metadata(mut self, metadata: MachineMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Validate the serialized machine
    pub fn validate(&self) -> Result<(), PersistenceError> {
        if self.id.is_empty() {
            return Err(PersistenceError::ValidationError("Machine ID cannot be empty".to_string()));
        }

        if self.states.is_empty() {
            return Err(PersistenceError::ValidationError("Machine must have at least one state".to_string()));
        }

        if self.initial_state.is_empty() {
            return Err(PersistenceError::ValidationError("Initial state cannot be empty".to_string()));
        }

        // Check if initial state exists
        if !self.states.iter().any(|s| s.id == self.initial_state) {
            return Err(PersistenceError::ValidationError(
                format!("Initial state '{}' not found in states", self.initial_state)
            ));
        }

        // Check if current state exists
        if !self.states.iter().any(|s| s.id == self.current_state) {
            return Err(PersistenceError::ValidationError(
                format!("Current state '{}' not found in states", self.current_state)
            ));
        }

        // Validate states
        for state in &self.states {
            state.validate()?;
        }

        // Validate transitions
        for transition in &self.transitions {
            transition.validate(&self.states)?;
        }

        Ok(())
    }

    /// Get state by ID
    pub fn get_state(&self, id: &str) -> Option<&SerializedState<C>> {
        self.states.iter().find(|s| s.id == id)
    }

    /// Get transitions from a state
    pub fn get_transitions_from(&self, state_id: &str) -> Vec<&SerializedTransition<E>> {
        self.transitions.iter()
            .filter(|t| t.from_state == state_id)
            .collect()
    }

    /// Calculate machine complexity metrics
    pub fn complexity_metrics(&self) -> ComplexityMetrics {
        ComplexityMetrics {
            state_count: self.states.len(),
            transition_count: self.transitions.len(),
            avg_transitions_per_state: if self.states.is_empty() {
                0.0
            } else {
                self.transitions.len() as f64 / self.states.len() as f64
            },
            has_parallel_states: self.states.iter().any(|s| !s.child_states.is_empty()),
            has_guarded_transitions: self.transitions.iter().any(|t| !t.guards.is_empty()),
        }
    }
}

/// Serialized state data
#[derive(Debug, Clone)]
pub struct SerializedState<C> {
    /// State ID
    pub id: String,
    /// State type
    pub state_type: StateType,
    /// State context (optional)
    pub context: Option<C>,
    /// Child states
    pub child_states: Vec<SerializedState<C>>,
    /// Initial child state
    pub initial_child: Option<String>,
    /// Entry actions
    pub entry_actions: Vec<String>,
    /// Exit actions
    pub exit_actions: Vec<String>,
    /// State metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl<C> SerializedState<C> {
    /// Create a new serialized state
    pub fn new(id: String) -> Self {
        Self {
            id,
            state_type: StateType::Atomic,
            context: None,
            child_states: Vec::new(),
            initial_child: None,
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set state type
    pub fn state_type(mut self, state_type: StateType) -> Self {
        self.state_type = state_type;
        self
    }

    /// Set context
    pub fn context(mut self, context: C) -> Self {
        self.context = Some(context);
        self
    }

    /// Add a child state
    pub fn add_child(mut self, child: SerializedState<C>) -> Self {
        self.child_states.push(child);
        self
    }

    /// Set initial child
    pub fn initial_child(mut self, initial: String) -> Self {
        self.initial_child = Some(initial);
        self
    }

    /// Add entry action
    pub fn add_entry_action(mut self, action: String) -> Self {
        self.entry_actions.push(action);
        self
    }

    /// Add exit action
    pub fn add_exit_action(mut self, action: String) -> Self {
        self.exit_actions.push(action);
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Validate the state
    pub fn validate(&self) -> Result<(), PersistenceError> {
        if self.id.is_empty() {
            return Err(PersistenceError::ValidationError("State ID cannot be empty".to_string()));
        }

        // Validate child states
        if !self.child_states.is_empty() {
            if self.state_type == StateType::Atomic {
                return Err(PersistenceError::ValidationError(
                    format!("Atomic state '{}' cannot have child states", self.id)
                ));
            }

            // Check initial child exists
            if let Some(ref initial) = self.initial_child {
                if !self.child_states.iter().any(|s| s.id == *initial) {
                    return Err(PersistenceError::ValidationError(
                        format!("Initial child state '{}' not found in children of '{}'", initial, self.id)
                    ));
                }
            }

            // Validate child states recursively
            for child in &self.child_states {
                child.validate()?;
            }
        }

        Ok(())
    }

    /// Get child state by ID
    pub fn get_child(&self, id: &str) -> Option<&SerializedState<C>> {
        self.child_states.iter().find(|s| s.id == id)
    }

    /// Check if state is compound
    pub fn is_compound(&self) -> bool {
        !self.child_states.is_empty()
    }

    /// Check if state is parallel
    pub fn is_parallel(&self) -> bool {
        self.state_type == StateType::Parallel
    }
}

/// State types
#[derive(Debug, Clone, PartialEq)]
pub enum StateType {
    /// Atomic state (no substates)
    Atomic,
    /// Compound state (has substates)
    Compound,
    /// Parallel state (substates execute in parallel)
    Parallel,
    /// Final state
    Final,
    /// History state
    History,
}

impl StateType {
    /// Check if state type can have children
    pub fn can_have_children(&self) -> bool {
        matches!(self, StateType::Compound | StateType::Parallel)
    }

    /// Check if state is terminal
    pub fn is_terminal(&self) -> bool {
        matches!(self, StateType::Final)
    }
}

/// Serialized transition data
#[derive(Debug, Clone)]
pub struct SerializedTransition<E> {
    /// From state ID
    pub from_state: String,
    /// To state ID
    pub to_state: String,
    /// Event that triggers the transition
    pub event: Option<String>,
    /// Guard conditions
    pub guards: Vec<String>,
    /// Actions to execute
    pub actions: Vec<String>,
    /// Transition metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl<E> SerializedTransition<E> {
    /// Create a new serialized transition
    pub fn new(from_state: String, to_state: String) -> Self {
        Self {
            from_state,
            to_state,
            event: None,
            guards: Vec::new(),
            actions: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set event
    pub fn event(mut self, event: String) -> Self {
        self.event = Some(event);
        self
    }

    /// Add guard
    pub fn add_guard(mut self, guard: String) -> Self {
        self.guards.push(guard);
        self
    }

    /// Add action
    pub fn add_action(mut self, action: String) -> Self {
        self.actions.push(action);
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Validate the transition
    pub fn validate<C>(&self, states: &[SerializedState<C>]) -> Result<(), PersistenceError> {
        // Check from state exists
        if !states.iter().any(|s| s.id == self.from_state) {
            return Err(PersistenceError::ValidationError(
                format!("Transition from state '{}' not found", self.from_state)
            ));
        }

        // Check to state exists
        if !states.iter().any(|s| s.id == self.to_state) {
            return Err(PersistenceError::ValidationError(
                format!("Transition to state '{}' not found", self.to_state)
            ));
        }

        Ok(())
    }

    /// Check if transition is guarded
    pub fn is_guarded(&self) -> bool {
        !self.guards.is_empty()
    }

    /// Check if transition has actions
    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }
}

/// Complexity metrics for analysis
#[derive(Debug, Clone)]
pub struct ComplexityMetrics {
    /// Number of states
    pub state_count: usize,
    /// Number of transitions
    pub transition_count: usize,
    /// Average transitions per state
    pub avg_transitions_per_state: f64,
    /// Whether machine has parallel states
    pub has_parallel_states: bool,
    /// Whether machine has guarded transitions
    pub has_guarded_transitions: bool,
}

impl ComplexityMetrics {
    /// Calculate complexity score (0-100)
    pub fn complexity_score(&self) -> f64 {
        let mut score = 0.0;

        // Base score from counts
        score += (self.state_count as f64 * 2.0).min(30.0);
        score += (self.transition_count as f64 * 1.5).min(30.0);

        // Parallel states increase complexity
        if self.has_parallel_states {
            score += 15.0;
        }

        // Guarded transitions increase complexity
        if self.has_guarded_transitions {
            score += 15.0;
        }

        // High connectivity increases complexity
        if self.avg_transitions_per_state > 3.0 {
            score += 10.0;
        }

        score.min(100.0)
    }

    /// Get complexity level
    pub fn complexity_level(&self) -> &'static str {
        let score = self.complexity_score();
        match score {
            0.0..=25.0 => "Simple",
            26.0..=50.0 => "Moderate",
            51.0..=75.0 => "Complex",
            _ => "Very Complex",
        }
    }
}

/// Serialization format information
#[derive(Debug, Clone)]
pub struct SerializationFormat {
    /// Format name
    pub name: String,
    /// Format version
    pub version: u32,
    /// Supported compression
    pub supports_compression: bool,
    /// Human readable
    pub human_readable: bool,
    /// Binary format
    pub binary: bool,
}

impl SerializationFormat {
    /// JSON format
    pub fn json() -> Self {
        Self {
            name: "json".to_string(),
            version: 1,
            supports_compression: true,
            human_readable: true,
            binary: false,
        }
    }

    /// Binary format
    pub fn binary() -> Self {
        Self {
            name: "binary".to_string(),
            version: 1,
            supports_compression: true,
            human_readable: false,
            binary: true,
        }
    }

    /// MessagePack format
    pub fn messagepack() -> Self {
        Self {
            name: "messagepack".to_string(),
            version: 1,
            supports_compression: true,
            human_readable: false,
            binary: true,
        }
    }
}

// Implement serde traits for SerializedMachine if serde is available
#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Deserialize, Serialize};

    impl<C, E, S> Serialize for SerializedMachine<C, E, S>
    where
        C: Serialize,
        E: Serialize,
    {
        fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
        where
            Ser: serde::Serializer,
        {
            // Custom serialization logic here
            // This is a simplified implementation
            use serde::ser::SerializeStruct;

            let mut state = serializer.serialize_struct("SerializedMachine", 9)?;
            state.serialize_field("version", &self.version)?;
            state.serialize_field("id", &self.id)?;
            state.serialize_field("states", &self.states)?;
            state.serialize_field("initial_state", &self.initial_state)?;
            state.serialize_field("transitions", &self.transitions)?;
            state.serialize_field("context", &self.context)?;
            state.serialize_field("current_state", &self.current_state)?;
            state.serialize_field("metadata", &self.metadata)?;
            state.serialize_field("timestamp", &self.timestamp)?;
            state.end()
        }
    }

    impl<'de, C, E, S> Deserialize<'de> for SerializedMachine<C, E, S>
    where
        C: Deserialize<'de>,
        E: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            // Custom deserialization logic here
            // This is a simplified implementation
            #[derive(Deserialize)]
            struct SerializedMachineData<C, E> {
                version: u32,
                id: String,
                states: Vec<SerializedState<C>>,
                initial_state: String,
                transitions: Vec<SerializedTransition<E>>,
                context: Option<C>,
                current_state: String,
                metadata: MachineMetadata,
                timestamp: u64,
            }

            let data = SerializedMachineData::deserialize(deserializer)?;

            Ok(SerializedMachine {
                version: data.version,
                id: data.id,
                states: data.states,
                initial_state: data.initial_state,
                transitions: data.transitions,
                context: data.context,
                current_state: data.current_state,
                metadata: data.metadata,
                timestamp: data.timestamp,
            })
        }
    }
}
