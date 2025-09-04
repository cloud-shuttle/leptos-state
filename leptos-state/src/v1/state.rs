//! # State Representation for State Machines
//! 
//! This module provides concrete implementations of state types that work
//! with our trait hierarchy. States can be simple, hierarchical, or parallel.

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use super::traits::{StateMachineContext, StateMachineEvent};
use super::error::StateMachineError;

// =============================================================================
// State Value Types
// =============================================================================

/// Represents the value of a state in a state machine
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateValue {
    /// A simple state with a string identifier
    Simple(String),
    
    /// A hierarchical state with parent and child
    Hierarchical {
        parent: String,
        child: Box<StateValue>,
    },
    
    /// Parallel states that can be active simultaneously
    Parallel {
        states: Vec<StateValue>,
    },
    
    /// A final state that cannot transition further
    Final(String),
}

impl StateValue {
    /// Creates a simple state
    pub fn simple(id: impl Into<String>) -> Self {
        StateValue::Simple(id.into())
    }
    
    /// Creates a hierarchical state
    pub fn hierarchical(parent: impl Into<String>, child: StateValue) -> Self {
        StateValue::Hierarchical {
            parent: parent.into(),
            child: Box::new(child),
        }
    }
    
    /// Creates parallel states
    pub fn parallel(states: Vec<StateValue>) -> Self {
        StateValue::Parallel { states }
    }
    
    /// Creates a final state
    pub fn final_state(id: impl Into<String>) -> Self {
        StateValue::Final(id.into())
    }
    
    /// Returns the string representation of this state
    pub fn as_str(&self) -> &str {
        match self {
            StateValue::Simple(id) => id,
            StateValue::Hierarchical { parent, .. } => parent,
            StateValue::Parallel { states } => {
                // For parallel states, return the first state's ID
                states.first().map(|s| s.as_str()).unwrap_or("parallel")
            }
            StateValue::Final(id) => id,
        }
    }
    
    /// Checks if this state matches a pattern
    pub fn matches(&self, pattern: &str) -> bool {
        match self {
            StateValue::Simple(id) => id == pattern,
            StateValue::Hierarchical { parent, child } => {
                parent == pattern || child.matches(pattern)
            }
            StateValue::Parallel { states } => {
                states.iter().any(|s| s.matches(pattern))
            }
            StateValue::Final(id) => id == pattern,
        }
    }
    
    /// Returns all state IDs contained in this state value
    pub fn all_ids(&self) -> Vec<String> {
        match self {
            StateValue::Simple(id) => vec![id.clone()],
            StateValue::Hierarchical { parent, child } => {
                let mut ids = vec![parent.clone()];
                ids.extend(child.all_ids());
                ids
            }
            StateValue::Parallel { states } => {
                states.iter().flat_map(|s| s.all_ids()).collect()
            }
            StateValue::Final(id) => vec![id.clone()],
        }
    }
    
    /// Returns the depth of this state (1 for simple, 2+ for hierarchical)
    pub fn depth(&self) -> usize {
        match self {
            StateValue::Simple(_) | StateValue::Final(_) => 1,
            StateValue::Hierarchical { child, .. } => 1 + child.depth(),
            StateValue::Parallel { states } => {
                states.iter().map(|s| s.depth()).max().unwrap_or(1)
            }
        }
    }
}

impl fmt::Display for StateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateValue::Simple(id) => write!(f, "{}", id),
            StateValue::Hierarchical { parent, child } => {
                write!(f, "{}.{}", parent, child)
            }
            StateValue::Parallel { states } => {
                let state_strings: Vec<String> = states.iter().map(|s| s.to_string()).collect();
                write!(f, "[{}]", state_strings.join("|"))
            }
            StateValue::Final(id) => write!(f, "{}*", id),
        }
    }
}

// =============================================================================
// State Node Structure
// =============================================================================

/// A node in the state machine representing a state and its transitions
#[derive(Debug, Clone)]
pub struct StateNode<C, E>
where
    C: StateMachineContext,
    E: StateMachineEvent,
{
    /// The state identifier
    pub id: String,
    
    /// The state value
    pub value: StateValue,
    
    /// Transitions from this state
    pub transitions: Vec<Transition<C, E>>,
    
    /// Actions to execute when entering this state
    pub entry_actions: Vec<Arc<dyn super::traits::Action<C>>>,
    
    /// Actions to execute when exiting this state
    pub exit_actions: Vec<Arc<dyn super::traits::Action<C>>>,
    
    /// Guards that control when this state can be entered
    pub entry_guards: Vec<Arc<dyn super::traits::Guard<C, E>>>,
    
    /// Guards that control when this state can be exited
    pub exit_guards: Vec<Arc<dyn super::traits::Guard<C, E>>>,
    
    /// Whether this is a final state
    pub is_final: bool,
    
    /// Metadata about this state
    pub metadata: StateMetadata,
}

impl<C, E> StateNode<C, E>
where
    C: StateMachineContext,
    E: StateMachineEvent,
{
    /// Creates a new state node
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            id: id.clone(),
            value: StateValue::simple(id),
            transitions: Vec::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            entry_guards: Vec::new(),
            exit_guards: Vec::new(),
            is_final: false,
            metadata: StateMetadata::default(),
        }
    }
    
    /// Sets the state value
    pub fn with_value(mut self, value: StateValue) -> Self {
        self.value = value;
        self
    }
    
    /// Adds a transition from this state
    pub fn with_transition(mut self, transition: Transition<C, E>) -> Self {
        self.transitions.push(transition);
        self
    }
    
    /// Adds an entry action
    pub fn with_entry_action(mut self, action: Arc<dyn super::traits::Action<C>>) -> Self {
        self.entry_actions.push(action);
        self
    }
    
    /// Adds an exit action
    pub fn with_exit_action(mut self, action: Arc<dyn super::traits::Action<C>>) -> Self {
        self.exit_actions.push(action);
        self
    }
    
    /// Adds an entry guard
    pub fn with_entry_guard(mut self, guard: Arc<dyn super::traits::Guard<C, E>>) -> Self {
        self.entry_guards.push(guard);
        self
    }
    
    /// Adds an exit guard
    pub fn with_exit_guard(mut self, guard: Arc<dyn super::traits::Guard<C, E>>) -> Self {
        self.exit_guards.push(guard);
        self
    }
    
    /// Marks this as a final state
    pub fn as_final(mut self) -> Self {
        self.is_final = true;
        self
    }
    
    /// Sets metadata for this state
    pub fn with_metadata(mut self, metadata: StateMetadata) -> Self {
        self.metadata = metadata;
        self
    }
    
    /// Checks if this state can transition on the given event
    pub fn can_transition(&self, event: &E) -> bool {
        self.transitions.iter().any(|t| t.event == *event)
    }
    
    /// Gets the target state for a transition on the given event
    pub fn get_transition_target(&self, event: &E) -> Option<&StateValue> {
        self.transitions
            .iter()
            .find(|t| t.event == *event)
            .map(|t| &t.target)
    }
    
    /// Executes entry actions for this state
    pub fn execute_entry_actions(&self, context: &mut C) -> Result<(), StateMachineError<C, E, StateValue>> {
        for action in &self.entry_actions {
            action.execute(context)
                .map_err(|e| StateMachineError::Action(e))?;
        }
        Ok(())
    }
    
    /// Executes exit actions for this state
    pub fn execute_exit_actions(&self, context: &mut C) -> Result<(), StateMachineError<C, E, StateValue>> {
        for action in &self.exit_actions {
            action.execute(context)
                .map_err(|e| StateMachineError::Action(e))?;
        }
        Ok(())
    }
    
    /// Checks if this state can be entered with the given context and event
    pub fn can_enter(&self, context: &C, event: &E) -> bool {
        self.entry_guards.iter().all(|g| g.check(context, event))
    }
    
    /// Checks if this state can be exited with the given context and event
    pub fn can_exit(&self, context: &C, event: &E) -> bool {
        self.exit_guards.iter().all(|g| g.check(context, event))
    }
}

// =============================================================================
// Transition Structure
// =============================================================================

/// A transition between states in the state machine
#[derive(Debug, Clone)]
pub struct Transition<C, E>
where
    C: StateMachineContext,
    E: StateMachineEvent,
{
    /// The event that triggers this transition
    pub event: E,
    
    /// The target state for this transition
    pub target: StateValue,
    
    /// Guards that must be satisfied for this transition
    pub guards: Vec<Arc<dyn super::traits::Guard<C, E>>>,
    
    /// Actions to execute during this transition
    pub actions: Vec<Arc<dyn super::traits::Action<C>>>,
    
    /// Whether this transition is internal (doesn't change state)
    pub is_internal: bool,
    
    /// Metadata about this transition
    pub metadata: TransitionMetadata,
}

impl<C, E> Transition<C, E>
where
    C: StateMachineContext,
    E: StateMachineEvent,
{
    /// Creates a new transition
    pub fn new(event: E, target: StateValue) -> Self {
        Self {
            event,
            target,
            guards: Vec::new(),
            actions: Vec::new(),
            is_internal: false,
            metadata: TransitionMetadata::default(),
        }
    }
    
    /// Adds a guard to this transition
    pub fn with_guard(mut self, guard: Arc<dyn super::traits::Guard<C, E>>) -> Self {
        self.guards.push(guard);
        self
    }
    
    /// Adds an action to this transition
    pub fn with_action(mut self, action: Arc<dyn super::traits::Action<C>>) -> Self {
        self.actions.push(action);
        self
    }
    
    /// Marks this as an internal transition
    pub fn as_internal(mut self) -> Self {
        self.is_internal = true;
        self
    }
    
    /// Sets metadata for this transition
    pub fn with_metadata(mut self, metadata: TransitionMetadata) -> Self {
        self.metadata = metadata;
        self
    }
    
    /// Checks if this transition is allowed with the given context and event
    pub fn is_allowed(&self, context: &C, event: &E) -> bool {
        self.guards.iter().all(|g| g.check(context, event))
    }
    
    /// Executes actions for this transition
    pub fn execute_actions(&self, context: &mut C) -> Result<(), StateMachineError<C, E, StateValue>> {
        for action in &self.actions {
            action.execute(context)
                .map_err(|e| StateMachineError::Action(e))?;
        }
        Ok(())
    }
}

// =============================================================================
// Metadata Structures
// =============================================================================

/// Metadata about a state
#[derive(Debug, Clone, Default)]
pub struct StateMetadata {
    /// Human-readable description of the state
    pub description: Option<String>,
    
    /// Tags for categorizing the state
    pub tags: Vec<String>,
    
    /// Custom data associated with the state
    pub custom_data: HashMap<String, String>,
}

impl StateMetadata {
    /// Creates new state metadata
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sets the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// Adds a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    /// Sets custom data
    pub fn with_custom_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_data.insert(key.into(), value.into());
        self
    }
}

/// Metadata about a transition
#[derive(Debug, Clone, Default)]
pub struct TransitionMetadata {
    /// Human-readable description of the transition
    pub description: Option<String>,
    
    /// Priority of this transition (higher numbers = higher priority)
    pub priority: i32,
    
    /// Whether this transition should be logged
    pub should_log: bool,
    
    /// Custom data associated with the transition
    pub custom_data: HashMap<String, String>,
}

impl TransitionMetadata {
    /// Creates new transition metadata
    pub fn new() -> Self {
        Self {
            description: None,
            priority: 0,
            should_log: true,
            custom_data: HashMap::new(),
        }
    }
    
    /// Sets the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// Sets the priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    
    /// Sets whether to log this transition
    pub fn with_logging(mut self, should_log: bool) -> Self {
        self.should_log = should_log;
        self
    }
    
    /// Sets custom data
    pub fn with_custom_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_data.insert(key.into(), value.into());
        self
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default, PartialEq)]
    struct TestContext {
        count: i32,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Start,
        Stop,
        Pause,
    }

    impl StateMachineContext for TestContext {}
    impl StateMachineEvent for TestEvent {}

    #[test]
    fn test_state_value_creation() {
        let simple = StateValue::simple("idle");
        assert_eq!(simple.as_str(), "idle");
        assert_eq!(simple.depth(), 1);

        let hierarchical = StateValue::hierarchical("power", StateValue::simple("on"));
        assert_eq!(hierarchical.as_str(), "power");
        assert_eq!(hierarchical.depth(), 2);

        let parallel = StateValue::parallel(vec![
            StateValue::simple("left"),
            StateValue::simple("right"),
        ]);
        assert_eq!(parallel.depth(), 1);
    }

    #[test]
    fn test_state_value_matching() {
        let state = StateValue::hierarchical("power", StateValue::simple("on"));
        
        assert!(state.matches("power"));
        assert!(state.matches("on"));
        assert!(!state.matches("off"));
    }

    #[test]
    fn test_state_value_all_ids() {
        let state = StateValue::hierarchical("power", StateValue::simple("on"));
        let ids = state.all_ids();
        
        assert!(ids.contains(&"power".to_string()));
        assert!(ids.contains(&"on".to_string()));
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_state_node_creation() {
        let node: StateNode<TestContext, TestEvent> = StateNode::new("idle")
            .with_value(StateValue::simple("idle"))
            .as_final();
        
        assert_eq!(node.id, "idle");
        assert!(node.is_final);
        assert_eq!(node.transitions.len(), 0);
    }

    #[test]
    fn test_transition_creation() {
        let transition: Transition<TestContext, TestEvent> = Transition::new(
            TestEvent::Start,
            StateValue::simple("active")
        ).with_metadata(TransitionMetadata::new().with_priority(10));
        
        assert_eq!(transition.event, TestEvent::Start);
        assert_eq!(transition.metadata.priority, 10);
        assert!(!transition.is_internal);
    }

    #[test]
    fn test_metadata_creation() {
        let state_meta = StateMetadata::new()
            .with_description("A test state")
            .with_tag("test")
            .with_tag("example");
        
        assert_eq!(state_meta.description, Some("A test state".to_string()));
        assert_eq!(state_meta.tags.len(), 2);
        assert!(state_meta.tags.contains(&"test".to_string()));
    }
}
