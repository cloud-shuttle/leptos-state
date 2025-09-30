//! Documentation data structures and output

use super::*;
use std::collections::HashMap;
use std::hash::Hash;

/// Generated document information
#[derive(Debug, Clone)]
pub struct GeneratedDocument {
    /// Format of the generated document
    pub format: DocumentationFormat,
    /// Content of the document
    pub content: String,
    /// Suggested filename
    pub filename: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl GeneratedDocument {
    /// Save the document to a file
    pub fn save_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        std::fs::write(path, &self.content)
    }

    /// Get the file extension for this document
    pub fn extension(&self) -> &str {
        self.format.extension()
    }

    /// Get the full filename with extension
    pub fn full_filename(&self) -> String {
        format!(
            "{}.{}",
            self.filename
                .trim_end_matches(&format!(".{}", self.extension())),
            self.extension()
        )
    }
}

/// Transition information for documentation
#[derive(Debug, Clone)]
pub struct TransitionInfo {
    /// Source state name
    pub from_state: String,
    /// Target state name
    pub to_state: String,
    /// Event that triggers this transition
    pub event: String,
    /// Guards that must pass for this transition
    pub guards: Vec<String>,
    /// Actions executed during this transition
    pub actions: Vec<String>,
    /// Priority of this transition
    pub priority: i32,
}

impl TransitionInfo {
    /// Create a new transition info
    pub fn new(from_state: String, to_state: String, event: String) -> Self {
        Self {
            from_state,
            to_state,
            event,
            guards: Vec::new(),
            actions: Vec::new(),
            priority: 0,
        }
    }

    /// Add a guard to this transition
    pub fn add_guard(&mut self, guard: String) {
        self.guards.push(guard);
    }

    /// Add an action to this transition
    pub fn add_action(&mut self, action: String) {
        self.actions.push(action);
    }

    /// Set the priority of this transition
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
}

/// State information for documentation
#[derive(Debug, Clone)]
pub struct StateInfo {
    /// State name
    pub name: String,
    /// State description
    pub description: Option<String>,
    /// Entry actions for this state
    pub entry_actions: Vec<String>,
    /// Exit actions for this state
    pub exit_actions: Vec<String>,
    /// Whether this is an initial state
    pub is_initial: bool,
    /// Whether this is a final state
    pub is_final: bool,
}

impl StateInfo {
    /// Create a new state info
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            is_initial: false,
            is_final: false,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add an entry action
    pub fn add_entry_action(&mut self, action: String) {
        self.entry_actions.push(action);
    }

    /// Add an exit action
    pub fn add_exit_action(&mut self, action: String) {
        self.exit_actions.push(action);
    }

    /// Mark as initial state
    pub fn mark_initial(&mut self) {
        self.is_initial = true;
    }

    /// Mark as final state
    pub fn mark_final(&mut self) {
        self.is_final = true;
    }
}

/// Action information for documentation
#[derive(Debug, Clone)]
pub struct ActionInfo {
    /// Action name
    pub name: String,
    /// Action description
    pub description: String,
    /// Action implementation details
    pub implementation: String,
    /// Whether this action has side effects
    pub has_side_effects: bool,
    /// Parameters this action accepts
    pub parameters: Vec<String>,
}

impl ActionInfo {
    /// Create a new action info
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            implementation: String::new(),
            has_side_effects: false,
            parameters: Vec::new(),
        }
    }

    /// Set the implementation details
    pub fn with_implementation(mut self, implementation: String) -> Self {
        self.implementation = implementation;
        self
    }

    /// Mark as having side effects
    pub fn with_side_effects(mut self, has_side_effects: bool) -> Self {
        self.has_side_effects = has_side_effects;
        self
    }

    /// Add a parameter
    pub fn add_parameter(&mut self, parameter: String) {
        self.parameters.push(parameter);
    }
}

/// Guard information for documentation
#[derive(Debug, Clone)]
pub struct GuardInfo {
    /// Guard name
    pub name: String,
    /// Guard description
    pub description: String,
    /// Guard implementation details
    pub implementation: String,
    /// Whether this guard is pure (no side effects)
    pub is_pure: bool,
    /// Parameters this guard accepts
    pub parameters: Vec<String>,
}

impl GuardInfo {
    /// Create a new guard info
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            implementation: String::new(),
            is_pure: true,
            parameters: Vec::new(),
        }
    }

    /// Set the implementation details
    pub fn with_implementation(mut self, implementation: String) -> Self {
        self.implementation = implementation;
        self
    }

    /// Mark as not pure (has side effects)
    pub fn with_side_effects(mut self) -> Self {
        self.is_pure = false;
        self
    }

    /// Add a parameter
    pub fn add_parameter(&mut self, parameter: String) {
        self.parameters.push(parameter);
    }
}

/// Documentation data structure
#[derive(Debug, Clone)]
pub struct DocumentationData {
    /// Machine name
    pub machine_name: String,
    /// Machine description
    pub machine_description: Option<String>,
    /// States in the machine
    pub states: Vec<StateInfo>,
    /// Transitions in the machine
    pub transitions: Vec<TransitionInfo>,
    /// Actions defined in the machine
    pub actions: Vec<ActionInfo>,
    /// Guards defined in the machine
    pub guards: Vec<GuardInfo>,
    /// Events that can trigger transitions
    pub events: Vec<String>,
    /// Metadata about the machine
    pub metadata: HashMap<String, String>,
    /// Generation timestamp
    pub generated_at: std::time::SystemTime,
}

impl DocumentationData {
    /// Create new documentation data from a machine
    pub fn new<
        C: Send + Sync + Clone + PartialEq + 'static,
        E: Clone + Send + Sync + Hash + Eq + 'static,
    >(
        machine: Machine<C, E, C>,
    ) -> Self {
        let mut data = Self {
            machine_name: "StateMachine".to_string(),
            machine_description: None,
            states: Vec::new(),
            transitions: Vec::new(),
            actions: Vec::new(),
            guards: Vec::new(),
            events: Vec::new(),
            metadata: HashMap::new(),
            generated_at: std::time::SystemTime::now(),
        };

        data.populate_from_machine(machine);
        data
    }

    /// Populate data from a machine
    fn populate_from_machine<
        C: Send + Sync + Clone + PartialEq + 'static,
        E: Clone + Send + Sync + Hash + Eq + 'static,
    >(
        &mut self,
        machine: Machine<C, E, C>,
    ) {
        // Set initial state
        self.metadata
            .insert("initial_state".to_string(), machine.initial.clone());

        // Populate states
        for state_name in machine.get_states() {
            let mut state_info = StateInfo::new(state_name.clone());
            if state_name == machine.initial {
                state_info.mark_initial();
            }
            self.states.push(state_info);
        }

        // Note: In a real implementation, we would need to extract transitions,
        // actions, and guards from the machine structure. This is simplified
        // for the modular split.
    }

    /// Add a state
    pub fn add_state(&mut self, state: StateInfo) {
        self.states.push(state);
    }

    /// Add a transition
    pub fn add_transition(&mut self, transition: TransitionInfo) {
        self.transitions.push(transition);
    }

    /// Add an action
    pub fn add_action(&mut self, action: ActionInfo) {
        self.actions.push(action);
    }

    /// Add a guard
    pub fn add_guard(&mut self, guard: GuardInfo) {
        self.guards.push(guard);
    }

    /// Add an event
    pub fn add_event(&mut self, event: String) {
        if !self.events.contains(&event) {
            self.events.push(event);
        }
    }

    /// Set machine name
    pub fn set_machine_name(&mut self, name: String) {
        self.machine_name = name;
    }

    /// Set machine description
    pub fn set_machine_description(&mut self, description: String) {
        self.machine_description = Some(description);
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get state by name
    pub fn get_state(&self, name: &str) -> Option<&StateInfo> {
        self.states.iter().find(|s| s.name == name)
    }

    /// Get state by name (mutable)
    pub fn get_state_mut(&mut self, name: &str) -> Option<&mut StateInfo> {
        self.states.iter_mut().find(|s| s.name == name)
    }

    /// Get transitions from a state
    pub fn get_transitions_from(&self, state_name: &str) -> Vec<&TransitionInfo> {
        self.transitions
            .iter()
            .filter(|t| t.from_state == state_name)
            .collect()
    }

    /// Get transitions to a state
    pub fn get_transitions_to(&self, state_name: &str) -> Vec<&TransitionInfo> {
        self.transitions
            .iter()
            .filter(|t| t.to_state == state_name)
            .collect()
    }

    /// Validate the documentation data
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check that all transition states exist
        for transition in &self.transitions {
            if !self.states.iter().any(|s| s.name == transition.from_state) {
                errors.push(format!(
                    "Transition references unknown from_state: {}",
                    transition.from_state
                ));
            }
            if !self.states.iter().any(|s| s.name == transition.to_state) {
                errors.push(format!(
                    "Transition references unknown to_state: {}",
                    transition.to_state
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// Serialization support
impl serde::Serialize for DocumentationData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{SerializeMap, SerializeStruct};

        let mut state = serializer.serialize_struct("DocumentationData", 9)?;
        state.serialize_field("machine_name", &self.machine_name)?;
        state.serialize_field("machine_description", &self.machine_description)?;
        state.serialize_field("states", &self.states)?;
        state.serialize_field("transitions", &self.transitions)?;
        state.serialize_field("actions", &self.actions)?;
        state.serialize_field("guards", &self.guards)?;
        state.serialize_field("events", &self.events)?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field(
            "generated_at",
            &self
                .generated_at
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for DocumentationData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Simplified deserialization - would need full implementation
        Err(serde::de::Error::custom(
            "DocumentationData deserialization not implemented",
        ))
    }
}
