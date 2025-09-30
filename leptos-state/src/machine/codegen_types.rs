//! Generated file types and transition information

use super::*;

/// Generated file information
#[derive(Debug, Clone)]
pub struct GeneratedFile {
    /// File name
    pub file_name: String,
    /// Generated content
    pub content: String,
    /// Target programming language
    pub language: ProgrammingLanguage,
    /// Time taken to generate
    pub generation_time: std::time::Duration,
    /// Number of lines in generated file
    pub line_count: usize,
}

impl GeneratedFile {
    /// Create a new generated file
    pub fn new(file_name: String, content: String, language: ProgrammingLanguage) -> Self {
        Self {
            file_name,
            line_count: content.lines().count(),
            content,
            language,
            generation_time: std::time::Duration::from_nanos(0),
        }
    }

    /// Save the file to disk
    pub fn save_to_file(&self, base_path: &std::path::Path) -> Result<std::path::PathBuf, String> {
        let full_path = base_path.join(&self.file_name);

        // Create directory if it doesn't exist
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        std::fs::write(&full_path, &self.content)
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(full_path)
    }

    /// Get file extension
    pub fn extension(&self) -> &str {
        self.language.extension()
    }

    /// Check if the file is empty
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }

    /// Get file size in bytes
    pub fn size_bytes(&self) -> usize {
        self.content.len()
    }

    /// Get content hash for change detection
    pub fn content_hash(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

/// Transition information for code generation
#[derive(Debug, Clone)]
pub struct TransitionInfo {
    /// Source state
    pub from_state: String,
    /// Target state
    pub to_state: String,
    /// Event that triggers the transition
    pub event_type: Option<String>,
    /// Guard conditions
    pub guards: Vec<String>,
    /// Actions to execute
    pub actions: Vec<String>,
    /// Transition priority
    pub priority: i32,
    /// Whether this is an internal transition
    pub internal: bool,
}

impl TransitionInfo {
    /// Create a new transition info
    pub fn new(from_state: String, to_state: String) -> Self {
        Self {
            from_state,
            to_state,
            event_type: None,
            guards: Vec::new(),
            actions: Vec::new(),
            priority: 0,
            internal: false,
        }
    }

    /// Set event type
    pub fn with_event(mut self, event_type: String) -> Self {
        self.event_type = Some(event_type);
        self
    }

    /// Add a guard
    pub fn add_guard(mut self, guard: String) -> Self {
        self.guards.push(guard);
        self
    }

    /// Add an action
    pub fn add_action(mut self, action: String) -> Self {
        self.actions.push(action);
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Mark as internal transition
    pub fn internal(mut self, internal: bool) -> Self {
        self.internal = internal;
        self
    }

    /// Check if this transition has guards
    pub fn has_guards(&self) -> bool {
        !self.guards.is_empty()
    }

    /// Check if this transition has actions
    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Get transition signature for code generation
    pub fn get_signature(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref event) = self.event_type {
            parts.push(event.clone());
        }

        if !self.guards.is_empty() {
            parts.push(format!("[{} guards]", self.guards.len()));
        }

        if !self.actions.is_empty() {
            parts.push(format!("/{} actions", self.actions.len()));
        }

        if parts.is_empty() {
            "automatic".to_string()
        } else {
            parts.join(" ")
        }
    }
}

/// State information for code generation
#[derive(Debug, Clone)]
pub struct StateGenInfo {
    /// State name
    pub name: String,
    /// State description
    pub description: Option<String>,
    /// Entry actions
    pub entry_actions: Vec<String>,
    /// Exit actions
    pub exit_actions: Vec<String>,
    /// Child states
    pub child_states: Vec<String>,
    /// State metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Whether this is the initial state
    pub is_initial: bool,
    /// Whether this is a final state
    pub is_final: bool,
}

impl StateGenInfo {
    /// Create a new state info
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            child_states: Vec::new(),
            metadata: std::collections::HashMap::new(),
            is_initial: false,
            is_final: false,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
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

    /// Add child state
    pub fn add_child_state(mut self, child: String) -> Self {
        self.child_states.push(child);
        self
    }

    /// Add metadata
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Mark as initial state
    pub fn initial(mut self, initial: bool) -> Self {
        self.is_initial = initial;
        self
    }

    /// Mark as final state
    pub fn final_state(mut self, final_state: bool) -> Self {
        self.is_final = final_state;
        self
    }

    /// Check if this state has actions
    pub fn has_actions(&self) -> bool {
        !self.entry_actions.is_empty() || !self.exit_actions.is_empty()
    }

    /// Check if this state has child states
    pub fn has_children(&self) -> bool {
        !self.child_states.is_empty()
    }

    /// Get all actions
    pub fn all_actions(&self) -> Vec<&str> {
        self.entry_actions
            .iter()
            .chain(self.exit_actions.iter())
            .map(|s| s.as_str())
            .collect()
    }
}

/// Event information for code generation
#[derive(Debug, Clone)]
pub struct EventGenInfo {
    /// Event name
    pub name: String,
    /// Event description
    pub description: Option<String>,
    /// Event payload type
    pub payload_type: Option<String>,
    /// Whether this event is external
    pub external: bool,
    /// Event metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl EventGenInfo {
    /// Create a new event info
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            payload_type: None,
            external: true,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set payload type
    pub fn with_payload_type(mut self, payload_type: String) -> Self {
        self.payload_type = Some(payload_type);
        self
    }

    /// Mark as internal event
    pub fn internal(mut self, internal: bool) -> Self {
        self.external = !internal;
        self
    }

    /// Add metadata
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Guard information for code generation
#[derive(Debug, Clone)]
pub struct GuardGenInfo {
    /// Guard name
    pub name: String,
    /// Guard description
    pub description: Option<String>,
    /// Guard implementation
    pub implementation: String,
    /// Guard type
    pub guard_type: GuardType,
    /// Guard metadata
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuardType {
    /// Function guard
    Function,
    /// State guard
    State,
    /// Time guard
    Time,
    /// Counter guard
    Counter,
    /// Composite guard
    Composite,
}

impl GuardGenInfo {
    /// Create a new guard info
    pub fn new(name: String, guard_type: GuardType) -> Self {
        Self {
            name,
            description: None,
            implementation: String::new(),
            guard_type,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set implementation
    pub fn with_implementation(mut self, implementation: String) -> Self {
        self.implementation = implementation;
        self
    }

    /// Add metadata
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Action information for code generation
#[derive(Debug, Clone)]
pub struct ActionGenInfo {
    /// Action name
    pub name: String,
    /// Action description
    pub description: Option<String>,
    /// Action implementation
    pub implementation: String,
    /// Action type
    pub action_type: ActionType,
    /// Whether this is an entry action
    pub is_entry: bool,
    /// Whether this is an exit action
    pub is_exit: bool,
    /// Action metadata
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    /// Function action
    Function,
    /// Assign action
    Assign,
    /// Log action
    Log,
    /// Timer action
    Timer,
    /// Composite action
    Composite,
}

impl ActionGenInfo {
    /// Create a new action info
    pub fn new(name: String, action_type: ActionType) -> Self {
        Self {
            name,
            description: None,
            implementation: String::new(),
            action_type,
            is_entry: false,
            is_exit: false,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set implementation
    pub fn with_implementation(mut self, implementation: String) -> Self {
        self.implementation = implementation;
        self
    }

    /// Mark as entry action
    pub fn entry(mut self, entry: bool) -> Self {
        self.is_entry = entry;
        self
    }

    /// Mark as exit action
    pub fn exit(mut self, exit: bool) -> Self {
        self.is_exit = exit;
        self
    }

    /// Add metadata
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Code generation context
#[derive(Debug, Clone)]
pub struct CodeGenContext {
    /// Target machine information
    pub machine_info: MachineGenInfo,
    /// Available states
    pub states: Vec<StateGenInfo>,
    /// Available events
    pub events: Vec<EventGenInfo>,
    /// Available guards
    pub guards: Vec<GuardGenInfo>,
    /// Available actions
    pub actions: Vec<ActionGenInfo>,
    /// Generation configuration
    pub config: CodeGenConfig,
    /// Custom variables
    pub variables: std::collections::HashMap<String, String>,
}

impl CodeGenContext {
    /// Create a new code generation context
    pub fn new(machine_info: MachineGenInfo, config: CodeGenConfig) -> Self {
        Self {
            machine_info,
            states: Vec::new(),
            events: Vec::new(),
            guards: Vec::new(),
            actions: Vec::new(),
            config,
            variables: std::collections::HashMap::new(),
        }
    }

    /// Add a state
    pub fn add_state(&mut self, state: StateGenInfo) {
        self.states.push(state);
    }

    /// Add an event
    pub fn add_event(&mut self, event: EventGenInfo) {
        self.events.push(event);
    }

    /// Add a guard
    pub fn add_guard(&mut self, guard: GuardGenInfo) {
        self.guards.push(guard);
    }

    /// Add an action
    pub fn add_action(&mut self, action: ActionGenInfo) {
        self.actions.push(action);
    }

    /// Set a variable
    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }

    /// Get a variable
    pub fn get_variable(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }

    /// Get initial state
    pub fn get_initial_state(&self) -> Option<&StateGenInfo> {
        self.states.iter().find(|s| s.is_initial)
    }

    /// Get final states
    pub fn get_final_states(&self) -> Vec<&StateGenInfo> {
        self.states.iter().filter(|s| s.is_final).collect()
    }
}

/// Machine information for code generation
#[derive(Debug, Clone)]
pub struct MachineGenInfo {
    /// Machine name
    pub name: String,
    /// Machine description
    pub description: Option<String>,
    /// Initial state
    pub initial_state: String,
    /// Machine type
    pub machine_type: MachineType,
    /// Machine metadata
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MachineType {
    /// Simple state machine
    Simple,
    /// Hierarchical state machine
    Hierarchical,
    /// Parallel state machine
    Parallel,
}

impl MachineGenInfo {
    /// Create a new machine info
    pub fn new(name: String, initial_state: String) -> Self {
        Self {
            name,
            description: None,
            initial_state,
            machine_type: MachineType::Simple,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set machine type
    pub fn with_type(mut self, machine_type: MachineType) -> Self {
        self.machine_type = machine_type;
        self
    }

    /// Add metadata
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}
