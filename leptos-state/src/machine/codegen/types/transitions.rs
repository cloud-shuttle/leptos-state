//! Transition information for code generation

/// Transition information for code generation
#[derive(Debug, Clone)]
pub struct TransitionInfo {
    /// Transition ID
    pub id: String,
    /// Source state ID
    pub source_state: String,
    /// Target state ID
    pub target_state: String,
    /// Event that triggers this transition
    pub event: Option<String>,
    /// Guard conditions
    pub guards: Vec<String>,
    /// Actions to execute
    pub actions: Vec<String>,
    /// Whether this is an internal transition
    pub internal: bool,
    /// Transition priority
    pub priority: i32,
}

impl TransitionInfo {
    /// Create a new transition info
    pub fn new(id: String, source_state: String, target_state: String) -> Self {
        Self {
            id,
            source_state,
            target_state,
            event: None,
            guards: Vec::new(),
            actions: Vec::new(),
            internal: false,
            priority: 0,
        }
    }

    /// Set the triggering event
    pub fn with_event(mut self, event: String) -> Self {
        self.event = Some(event);
        self
    }

    /// Add a guard condition
    pub fn with_guard(mut self, guard: String) -> Self {
        self.guards.push(guard);
        self
    }

    /// Add an action
    pub fn with_action(mut self, action: String) -> Self {
        self.actions.push(action);
        self
    }

    /// Mark as internal transition
    pub fn internal(mut self) -> Self {
        self.internal = true;
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
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

    /// Get transition signature (source -> target)
    pub fn signature(&self) -> String {
        format!("{} -> {}", self.source_state, self.target_state)
    }

    /// Get full transition description
    pub fn description(&self) -> String {
        let mut desc = format!("Transition '{}': {}", self.id, self.signature());

        if let Some(ref event) = self.event {
            desc.push_str(&format!(" on event '{}'", event));
        }

        if self.has_guards() {
            desc.push_str(&format!(" with {} guard(s)", self.guards.len()));
        }

        if self.has_actions() {
            desc.push_str(&format!(" executing {} action(s)", self.actions.len()));
        }

        if self.internal {
            desc.push_str(" (internal)");
        }

        desc
    }

    /// Get the event name or "auto" if none
    pub fn event_name(&self) -> &str {
        self.event.as_deref().unwrap_or("auto")
    }

    /// Check if this transition conflicts with another
    pub fn conflicts_with(&self, other: &TransitionInfo) -> bool {
        self.source_state == other.source_state &&
        self.event == other.event &&
        self.priority == other.priority
    }

    /// Get guard count
    pub fn guard_count(&self) -> usize {
        self.guards.len()
    }

    /// Get action count
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Check if transition is valid
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() &&
        !self.source_state.is_empty() &&
        !self.target_state.is_empty()
    }
}

impl Default for TransitionInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            source_state: String::new(),
            target_state: String::new(),
            event: None,
            guards: Vec::new(),
            actions: Vec::new(),
            internal: false,
            priority: 0,
        }
    }
}

impl std::fmt::Display for TransitionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
