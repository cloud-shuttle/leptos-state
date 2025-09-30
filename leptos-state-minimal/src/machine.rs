//! State machine implementation

use crate::{State, Event, MachineError, MachineResult};
use std::collections::HashMap;

/// A state machine that manages state transitions
///
/// The machine is generic over state and event types, requiring only
/// that they live as long as the application.
pub struct Machine<S: State, E: Event> {
    states: HashMap<String, StateNode<S, E>>,
    current_state: String,
    context: S,
}

impl<S: State, E: Event> Machine<S, E> {
    /// Create a new state machine
    pub fn new(initial_state: &str, context: S) -> Self {
        Self {
            states: HashMap::new(),
            current_state: initial_state.to_string(),
            context,
        }
    }

    /// Add a state to the machine
    pub fn add_state(&mut self, name: &str, state: StateNode<S, E>) {
        self.states.insert(name.to_string(), state);
    }

    /// Send an event to the machine
    ///
    /// This will attempt to transition to a new state based on the current
    /// state and the event. Returns an error if the transition is invalid.
    pub fn send(&mut self, event: E) -> MachineResult<()> {
        let current_state_name = self.current_state.clone();

        let current_state = self.states.get(&current_state_name)
            .ok_or_else(|| MachineError::InvalidState { state: current_state_name.clone() })?;

        // Find the transition for this event
        if let Some(transition) = current_state.transitions.get(event.event_type()) {
            let target_state = transition.target.clone();

            // Execute exit actions for current state
            if let Some(exit_actions) = &current_state.exit_actions {
                for action in exit_actions {
                    action(&mut self.context, &event);
                }
            }

            // Execute transition actions
            if let Some(transition_actions) = &transition.actions {
                for action in transition_actions {
                    action(&mut self.context, &event);
                }
            }

            // Update to new state
            self.current_state = target_state.clone();

            // Execute entry actions for new state
            if let Some(new_state) = self.states.get(&target_state) {
                if let Some(entry_actions) = &new_state.entry_actions {
                    for action in entry_actions {
                        action(&mut self.context, &event);
                    }
                }
            }

            Ok(())
        } else {
            Err(MachineError::InvalidTransition {
                from: current_state_name,
                to: format!("no transition for event {:?}", event.event_type()),
            })
        }
    }

    /// Get the current state name
    pub fn current_state(&self) -> &str {
        &self.current_state
    }

    /// Get a reference to the context
    pub fn context(&self) -> &S {
        &self.context
    }

    /// Get a mutable reference to the context
    pub fn context_mut(&mut self) -> &mut S {
        &mut self.context
    }

    /// Check if a transition is valid without executing it
    pub fn can_transition(&self, event: &E) -> bool {
        if let Some(current_state) = self.states.get(&self.current_state) {
            current_state.transitions.contains_key(event.event_type())
        } else {
            false
        }
    }

    /// Get all possible states
    pub fn states(&self) -> Vec<&str> {
        self.states.keys().map(|s| s.as_str()).collect()
    }

    /// Get all possible transitions from current state
    pub fn possible_transitions(&self) -> Vec<String> {
        if let Some(current_state) = self.states.get(&self.current_state) {
            current_state.transitions.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

/// A node in the state machine representing a state
pub struct StateNode<S: State, E: Event> {
    /// Actions to execute when entering this state
    pub entry_actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,

    /// Actions to execute when exiting this state
    pub exit_actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,

    /// Transitions from this state (keyed by event type)
    pub transitions: HashMap<String, Transition<S, E>>,
}

impl<S: State, E: Event> StateNode<S, E> {
    /// Create a new state node
    pub fn new() -> Self {
        Self {
            entry_actions: None,
            exit_actions: None,
            transitions: HashMap::new(),
        }
    }

    /// Add an entry action
    pub fn on_entry<F>(mut self, action: F) -> Self
    where
        F: Fn(&mut S, &E) + Send + Sync + 'static,
    {
        self.entry_actions.get_or_insert_with(Vec::new).push(Box::new(action));
        self
    }

    /// Add an exit action
    pub fn on_exit<F>(mut self, action: F) -> Self
    where
        F: Fn(&mut S, &E) + Send + Sync + 'static,
    {
        self.exit_actions.get_or_insert_with(Vec::new).push(Box::new(action));
        self
    }

    /// Add a transition
    pub fn on(mut self, event: E, target: &str) -> Self {
        let transition = Transition::new(target);
        self.transitions.insert(event.event_type().to_string(), transition);
        self
    }

    /// Add a transition with actions
    pub fn on_with_actions<F>(mut self, event: E, target: &str, actions: Vec<F>) -> Self
    where
        F: Fn(&mut S, &E) + Send + Sync + 'static,
    {
        let transition = Transition::new(target).with_actions(actions);
        self.transitions.insert(event.event_type().to_string(), transition);
        self
    }
}

impl<S: State, E: Event> Default for StateNode<S, E> {
    fn default() -> Self {
        Self::new()
    }
}

/// A transition between states
pub struct Transition<S: State, E: Event> {
    /// Target state name
    pub target: String,

    /// Actions to execute during transition
    pub actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,
}

impl<S: State, E: Event> Transition<S, E> {
    /// Create a new transition
    pub fn new(target: &str) -> Self {
        Self {
            target: target.to_string(),
            actions: None,
        }
    }

    /// Add actions to the transition
    pub fn with_actions<F>(mut self, actions: Vec<F>) -> Self
    where
        F: Fn(&mut S, &E) + Send + Sync + 'static,
    {
        self.actions = Some(actions.into_iter().map(|f| Box::new(f) as Box<dyn Fn(&mut S, &E) + Send + Sync>).collect());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug)]
    struct TestContext {
        value: i32,
    }

    #[derive(Clone, PartialEq, Debug)]
    enum TestEvent {
        Increment,
        Decrement,
        Reset,
    }

    impl TestEvent {
        fn event_type(&self) -> &'static str {
            match self {
                TestEvent::Increment => "increment",
                TestEvent::Decrement => "decrement",
                TestEvent::Reset => "reset",
            }
        }
    }

    #[test]
    fn machine_creation_works() {
        let context = TestContext { value: 0 };
        let machine = Machine::<TestContext, TestEvent>::new("idle", context);
        assert_eq!(machine.current_state(), "idle");
        assert_eq!(machine.context().value, 0);
    }

    #[test]
    fn state_node_creation_works() {
        let state_node = StateNode::<TestContext, TestEvent>::new();
        assert!(state_node.entry_actions.is_none());
        assert!(state_node.transitions.is_empty());
    }

    #[test]
    fn transition_creation_works() {
        let transition = Transition::<TestContext, TestEvent>::new("running");
        assert_eq!(transition.target, "running");
        assert!(transition.actions.is_none());
    }

    #[test]
    fn simple_machine_transition() {
        let context = TestContext { value: 0 };
        let mut machine = Machine::new("idle", context);

        // Add a simple state
        let idle_state = StateNode::new().on(TestEvent::Increment, "running");
        machine.add_state("idle", idle_state);

        // Add running state
        let running_state = StateNode::new();
        machine.add_state("running", running_state);

        // Test transition
        assert_eq!(machine.current_state(), "idle");
        machine.send(TestEvent::Increment).unwrap();
        assert_eq!(machine.current_state(), "running");
    }

    #[test]
    fn invalid_transition_returns_error() {
        let context = TestContext { value: 0 };
        let mut machine = Machine::new("idle", context);

        let idle_state = StateNode::new();
        machine.add_state("idle", idle_state);

        let result = machine.send(TestEvent::Increment);
        assert!(result.is_err());
    }
}
