//! Data structures for visualization

use super::*;

/// State diagram representation for export
#[derive(Debug, Clone)]
pub struct StateDiagram<'a, C: Send + Sync, E> {
    /// Machine name or identifier
    pub name: String,
    /// Initial state
    pub initial_state: String,
    /// All states in the machine
    pub states: Vec<StateInfo<'a, C, E>>,
    /// All transitions
    pub transitions: Vec<TransitionInfo<'a, C, E>>,
    /// Configuration used for generation
    pub config: VisualizationConfig,
    /// Generation timestamp
    pub generated_at: std::time::Instant,
}

impl<'a, C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + 'static> StateDiagram<'a, C, E> {
    /// Create a new state diagram from a machine
    pub fn new(machine: &'a Machine<C, E, C>, config: &VisualizationConfig) -> Self {
        let name = "StateMachine".to_string(); // Could be made configurable
        let initial_state = machine.initial_state();
        let states = machine
            .get_states()
            .iter()
            .filter_map(|state_name| {
                machine
                    .states_map()
                    .get(state_name)
                    .map(|state_node| StateInfo::new(state_name, state_node, config))
            })
            .collect();

        let transitions = machine
            .states_map()
            .iter()
            .flat_map(|(from_state, state_node)| {
                state_node
                    .transitions
                    .iter()
                    .map(move |transition| TransitionInfo::new(from_state, transition, config))
            })
            .collect();

        Self {
            name,
            initial_state,
            states,
            transitions,
            config: config.clone(),
            generated_at: std::time::Instant::now(),
        }
    }

    /// Get all state names
    pub fn state_names(&self) -> Vec<&str> {
        self.states.iter().map(|s| s.name.as_str()).collect()
    }

    /// Get transitions from a specific state
    pub fn transitions_from(&self, state_name: &str) -> Vec<&TransitionInfo<'a, C, E>> {
        self.transitions
            .iter()
            .filter(|t| t.from_state == state_name)
            .collect()
    }

    /// Get transitions to a specific state
    pub fn transitions_to(&self, state_name: &str) -> Vec<&TransitionInfo<'a, C, E>> {
        self.transitions
            .iter()
            .filter(|t| t.to_state == state_name)
            .collect()
    }

    /// Check if the diagram is valid
    pub fn is_valid(&self) -> bool {
        // Check that initial state exists
        self.states.iter().any(|s| s.name == self.initial_state) &&
        // Check that all transition states exist
        self.transitions.iter().all(|t| {
            self.states.iter().any(|s| s.name == t.from_state) &&
            self.states.iter().any(|s| s.name == t.to_state)
        })
    }
}

/// State information for visualization
#[derive(Debug, Clone)]
pub struct StateInfo<'a, C: Send + Sync, E> {
    /// State name
    pub name: String,
    /// State description
    pub description: Option<String>,
    /// Entry actions
    pub entry_actions: Vec<String>,
    /// Exit actions
    pub exit_actions: Vec<String>,
    /// Child states (for hierarchical machines)
    pub child_states: Vec<String>,
    /// Whether this is the initial state
    pub is_initial: bool,
    /// State metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl<'a, C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + 'static> StateInfo<'a, C, E> {
    /// Create state info from a state node
    pub fn new(
        state_name: &str,
        state_node: &'a StateNode<C, E, C>,
        config: &VisualizationConfig,
    ) -> Self {
        let entry_actions = if config.show_actions {
            state_node
                .entry_actions
                .iter()
                .map(|action| action.description())
                .collect()
        } else {
            Vec::new()
        };

        let exit_actions = if config.show_actions {
            state_node
                .exit_actions
                .iter()
                .map(|action| action.description())
                .collect()
        } else {
            Vec::new()
        };

        let child_states = state_node.child_states.keys().map(|s| s.clone()).collect();

        Self {
            name: state_name.to_string(),
            description: None, // Could be extracted from attributes
            entry_actions,
            exit_actions,
            child_states,
            is_initial: false, // Set by caller
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Check if this state has actions
    pub fn has_actions(&self) -> bool {
        !self.entry_actions.is_empty() || !self.exit_actions.is_empty()
    }

    /// Check if this state has child states
    pub fn has_children(&self) -> bool {
        !self.child_states.is_empty()
    }

    /// Get all actions (entry + exit)
    pub fn all_actions(&self) -> Vec<&str> {
        self.entry_actions
            .iter()
            .chain(self.exit_actions.iter())
            .map(|s| s.as_str())
            .collect()
    }
}

/// Transition information for visualization
#[derive(Debug, Clone)]
pub struct TransitionInfo<'a, C: Send + Sync, E> {
    /// Source state
    pub from_state: String,
    /// Target state
    pub to_state: String,
    /// Event that triggers the transition
    pub event: Option<String>,
    /// Guard conditions
    pub guards: Vec<String>,
    /// Actions executed during transition
    pub actions: Vec<String>,
    /// Transition metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl<'a, C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + 'static> TransitionInfo<'a, C, E> {
    /// Create transition info from a transition
    pub fn new(
        from_state: &str,
        transition: &'a Transition<C, E>,
        config: &VisualizationConfig,
    ) -> Self {
        let guards = if config.show_guards {
            transition
                .guards
                .iter()
                .map(|guard| guard.description())
                .collect()
        } else {
            Vec::new()
        };

        let actions = if config.show_actions {
            transition
                .actions
                .iter()
                .map(|action| action.description())
                .collect()
        } else {
            Vec::new()
        };

        Self {
            from_state: from_state.to_string(),
            to_state: transition.target.clone(),
            event: None, // Could be extracted from event type
            guards,
            actions,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Check if this transition has guards
    pub fn has_guards(&self) -> bool {
        !self.guards.is_empty()
    }

    /// Check if this transition has actions
    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Get transition label for diagrams
    pub fn get_label(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref event) = self.event {
            parts.push(event.clone());
        }

        if !self.guards.is_empty() {
            parts.push(format!("[{} guards]", self.guards.len()));
        }

        if !self.actions.is_empty() {
            parts.push(format!("/{} actions", self.actions.len()));
        }

        parts.join(" ")
    }
}

/// Machine snapshot for time travel
#[derive(Debug, Clone)]
pub struct MachineSnapshot<C: Send + Sync, E> {
    /// Machine state at snapshot time
    pub current_state: String,
    /// Context at snapshot time
    pub context: C,
    /// Event history leading to this state
    pub event_history: Vec<TransitionEvent<C, E>>,
    /// Timestamp when snapshot was taken
    pub timestamp: std::time::Instant,
    /// Snapshot metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + 'static> MachineSnapshot<C, E> {
    /// Create a new snapshot
    pub fn new(machine: Machine<C, E, C>) -> Self {
        // This is a simplified implementation
        // In a real implementation, we'd need to access the machine's internal state
        Self {
            current_state: "unknown".to_string(), // Would get from machine
            context: machine.initial_with_context(Default::default()),
            event_history: Vec::new(),
            timestamp: std::time::Instant::now(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add metadata to the snapshot
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get the age of this snapshot
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
    }

    /// Check if the snapshot is recent (within last minute)
    pub fn is_recent(&self) -> bool {
        self.age() < std::time::Duration::from_secs(60)
    }
}

// TODO: Re-enable serde support for MachineSnapshot when needed
// Currently removed to avoid compilation issues with generic type bounds
// impl<C: Send + Sync, E> serde::Serialize for MachineSnapshot<C, E>
// where
//     C: serde::Serialize,
//     E: serde::Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         use serde::ser::{SerializeStruct, Serializer};

//         let mut state = serializer.serialize_struct("MachineSnapshot", 5)?;
//         state.serialize_field("current_state", &self.current_state)?;
//         state.serialize_field("context", &self.context)?;
//         state.serialize_field("event_history", &self.event_history)?;
//         state.serialize_field("timestamp", &self.timestamp.elapsed().as_nanos())?;
//         state.serialize_field("metadata", &self.metadata)?;
//         state.end()
//     }
// }

// impl<'de, C: Send + Sync, E> serde::Deserialize<'de> for MachineSnapshot<C, E>
// where
//     C: serde::Deserialize<'de>,
//     E: serde::Deserialize<'de>,
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         use serde::de::{self, Deserializer, MapAccess, Visitor};
//         use std::fmt;

//         #[derive(serde::Deserialize)]
//         struct MachineSnapshotData<C2, E2> {
//             current_state: String,
//             context: C2,
//             event_history: Vec<TransitionEvent<C2, E2>>,
//             timestamp: u128,
//             metadata: std::collections::HashMap<String, String>,
//         }

//         let data = MachineSnapshotData::deserialize(deserializer)?;

//         Ok(MachineSnapshot {
//             current_state: data.current_state,
//             context: data.context,
//             event_history: data.event_history,
//             timestamp: std::time::Instant::now()
//                 - std::time::Duration::from_nanos(data.timestamp as u64),
//             metadata: data.metadata,
//         })
//     }
// }

impl<'a, C: Send + Sync, E> serde::Serialize for StateDiagram<'a, C, E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{SerializeStruct, Serializer};

        let mut state = serializer.serialize_struct("StateDiagram", 7)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("initial_state", &self.initial_state)?;
        state.serialize_field("states", &self.states)?;
        state.serialize_field("transitions", &self.transitions)?;
        state.serialize_field("config", &self.config)?;
        state.serialize_field("generated_at", &self.generated_at.elapsed().as_nanos())?;
        state.end()
    }
}

impl<'a, C: Send + Sync, E> serde::Serialize for StateInfo<'a, C, E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("StateInfo", 7)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("entry_actions", &self.entry_actions)?;
        state.serialize_field("exit_actions", &self.exit_actions)?;
        state.serialize_field("child_states", &self.child_states)?;
        state.serialize_field("is_initial", &self.is_initial)?;
        state.serialize_field("metadata", &self.metadata)?;
        state.end()
    }
}

impl<'a, C: Send + Sync, E> serde::Serialize for TransitionInfo<'a, C, E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("TransitionInfo", 6)?;
        state.serialize_field("from_state", &self.from_state)?;
        state.serialize_field("to_state", &self.to_state)?;
        state.serialize_field("event", &self.event)?;
        state.serialize_field("guards", &self.guards)?;
        state.serialize_field("actions", &self.actions)?;
        state.serialize_field("metadata", &self.metadata)?;
        state.end()
    }
}
