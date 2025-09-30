//! Core state machine traits and data structures

use super::*;
use crate::machine::states::StateValue;
use crate::StateResult;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

// Re-export builders for convenience
pub use builders::*;

// Import required traits
use crate::machine::core_actions::Action;
use crate::machine::core_guards::Guard;

/// Core trait for state machines
pub trait StateMachine: Sized + 'static {
    type Context: Clone + PartialEq + Send + Sync + 'static;
    type Event: Clone + Send + Sync + 'static;
    type State: MachineState<Context = Self::Context> + Clone + Send + Sync + 'static;

    fn initial() -> Self::State;
    fn transition(state: &Self::State, event: Self::Event) -> Self::State;
}

/// Trait for machine states
pub trait MachineState {
    type Context: Send + Sync + 'static;

    fn value(&self) -> &StateValue;
    fn context(&self) -> &Self::Context;
    fn matches(&self, pattern: &str) -> bool;
    fn can_transition_to(&self, target: &str) -> bool;
}

/// State node in the machine definition
#[derive(Debug)]
pub struct StateNode<C, E, S>
where
    C: Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + 'static,
    S: Clone + std::fmt::Debug,
{
    pub id: String,
    pub transitions: Vec<Transition<C, E>>,
    pub entry_actions: Vec<Box<dyn Action<C, E>>>,
    pub exit_actions: Vec<Box<dyn Action<C, E>>>,
    pub child_states: HashMap<String, StateNode<C, E, C>>,
    pub initial_child: Option<String>,
    pub _phantom: std::marker::PhantomData<S>,
}

/// Transition definition
#[derive(Debug)]
pub struct Transition<C, E>
where
    C: Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + 'static,
{
    pub event: E,
    pub target: String,
    pub guards: Vec<Box<dyn Guard<C, E>>>,
    pub actions: Vec<Box<dyn Action<C, E>>>,
}

/// Complete machine implementation
#[derive(Debug)]
pub struct Machine<
    C: Send + Sync + Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static,
    S: Clone + std::fmt::Debug,
> {
    pub states: HashMap<String, StateNode<C, E, C>>,
    pub initial: String,
    pub _phantom: std::marker::PhantomData<S>,
}

// Manual Clone implementation for Transition since trait objects can't be cloned
impl<C: Clone + Default, E: Clone + Send> Clone for Transition<C, E> {
    fn clone(&self) -> Self {
        Self {
            event: self.event.clone(),
            target: self.target.clone(),
            guards: Vec::new(), // Can't clone trait objects, so we create empty vectors
            actions: Vec::new(),
        }
    }
}

// Manual Clone implementation for StateNode since Action trait objects can't be cloned
impl<C: Clone + Default, E: Clone + Send> Clone for StateNode<C, E, C> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            transitions: self.transitions.clone(),
            entry_actions: Vec::new(), // Can't clone trait objects, so we create empty vectors
            exit_actions: Vec::new(),
            child_states: self.child_states.clone(),
            initial_child: self.initial_child.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

// Manual Clone implementation for Machine since trait objects can't be cloned
impl<C: Clone + Send + Sync + std::fmt::Debug + Default + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> Clone for Machine<C, E, C> {
    fn clone(&self) -> Self {
        Self {
            states: self.states.clone(),
            initial: self.initial.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C: Send + Sync + Clone + std::fmt::Debug + Default + 'static, E: Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash> Machine<C, E, C> {
    /// Get all state IDs in the machine
    pub fn get_states(&self) -> Vec<String> {
        self.states.keys().cloned().collect()
    }

    /// Get the initial state ID
    pub fn initial_state_id(&self) -> &str {
        &self.initial
    }

    /// Get a reference to the states map
    pub fn states_map(&self) -> &HashMap<String, StateNode<C, E, C>> {
        &self.states
    }

    /// Get initial state for this machine
    pub fn initial_state(&self) -> MachineStateImpl<C> {
        transitions::initial_state(self)
    }

    /// Create initial state with custom context
    pub fn initial_with_context(&self, context: C) -> MachineStateImpl<C> {
        transitions::initial_with_context(self, context)
    }

    /// Transition from one state to another based on an event
    pub fn transition(&self, state: &MachineStateImpl<C>, event: E) -> MachineStateImpl<C> {
        transitions::transition(self, state, event)
    }
}

/// Concrete implementation of machine state
#[derive(Debug, Clone, PartialEq)]
pub struct MachineStateImpl<C: Send + Sync> {
    pub value: StateValue,
    pub context: C,
}

impl<C: Send + Sync + 'static> MachineState for MachineStateImpl<C> {
    type Context = C;

    fn value(&self) -> &StateValue {
        &self.value
    }

    fn context(&self) -> &Self::Context {
        &self.context
    }

    fn matches(&self, pattern: &str) -> bool {
        self.value.matches(pattern)
    }

    fn can_transition_to(&self, target: &str) -> bool {
        // Check if the target state exists in the machine
        // This is a simplified implementation - in a full implementation,
        // you would need access to the machine definition to check transitions
        // For now, we'll assume any state can transition to any other state
        // In a real implementation, this would check the machine's transition table
        !target.is_empty()
    }
}

impl<C: Send + Sync> MachineStateImpl<C> {
    /// Create a new machine state with the given value and context
    pub fn new(value: StateValue, context: C) -> Self {
        Self { value, context }
    }

    /// Create a new machine state with the given value and default context
    pub fn with_value(value: StateValue) -> Self
    where
        C: Default,
    {
        Self {
            value,
            context: C::default(),
        }
    }

    /// Create a new machine state with the given context and default value
    pub fn with_context(context: C) -> Self {
        Self {
            value: StateValue::Simple("idle".to_string()),
            context,
        }
    }
}

impl<C: Send + Sync> Default for MachineStateImpl<C>
where
    C: Default,
{
    fn default() -> Self {
        Self {
            value: StateValue::Simple("idle".to_string()),
            context: C::default(),
        }
    }
}
