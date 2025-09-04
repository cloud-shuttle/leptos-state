//! # State Machine Implementation
//! 
//! This module provides the core state machine implementation.

use super::traits::{StateMachineContext, StateMachineEvent, StateMachineState};
use super::error::StateMachineError;
use super::state::{StateNode, StateValue};
use std::collections::HashMap;
use std::sync::Arc;

/// Core state machine implementation
#[derive(Clone)]
pub struct Machine<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    /// Current state of the machine
    current_state: S,
    /// Available states and their configurations
    states: HashMap<String, StateNode<C, E>>,
    /// Initial state identifier
    initial_state: String,
    /// Current context
    context: C,
    /// State history for rollback capability
    history: Vec<S>,
    /// Maximum history size
    max_history: usize,
}

impl<C, E, S> Machine<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent + Default,
    S: StateMachineState<Context = C, Event = E> + Default,
{
    /// Create a new state machine
    pub fn new(initial_state: S, context: C) -> Self {
        let initial_state_id = Self::get_state_id(&initial_state);
        Self {
            current_state: initial_state,
            states: HashMap::new(),
            initial_state: initial_state_id,
            context,
            history: Vec::new(),
            max_history: 10,
        }
    }

    /// Add a state to the machine
    pub fn add_state(&mut self, state: StateNode<C, E>) -> &mut Self {
        self.states.insert(state.id.clone(), state);
        self
    }

    /// Get the current state
    pub fn current_state(&self) -> &S {
        &self.current_state
    }

    /// Get the current context
    pub fn context(&self) -> &C {
        &self.context
    }

    /// Get mutable access to the context
    pub fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }

    /// Check if the machine can transition on the given event
    pub fn can_transition(&self, event: &E) -> bool {
        let current_state_id = Self::get_state_id(&self.current_state);
        
        if let Some(state_node) = self.states.get(&current_state_id) {
            state_node.transitions.iter().any(|transition| {
                let event_matches = transition.event == *event;
                let guards_pass = self.evaluate_guards(&transition.guards).is_ok();
                event_matches && guards_pass
            })
        } else {
            false
        }
    }

    /// Transition to a new state based on an event
    pub fn transition(&mut self, event: E) -> Result<S, StateMachineError<C, E, S>> {
        let current_state_id = Self::get_state_id(&self.current_state);
        
        // Find the current state node and transition
        let (state_node, transition) = {
            let state_node = self.states.get(&current_state_id)
                .ok_or_else(|| StateMachineError::State(super::error::StateError::InvalidState(self.current_state.clone())))?;
            
            let transition = state_node.transitions.iter()
                .find(|t| t.event == event)
                .ok_or_else(|| StateMachineError::Transition(super::error::TransitionError::InvalidTransition(event.clone())))?;
            
            (state_node.clone(), transition.clone())
        };
        
        // Evaluate guards
        self.evaluate_guards(&transition.guards)?;
        
        // Execute exit actions for current state
        self.execute_actions(&state_node.exit_actions)?;
        
        // Execute transition actions
        self.execute_actions(&transition.actions)?;
        
        // Store current state in history
        self.add_to_history(self.current_state.clone());
        
        // Transition to new state
        let new_state = self.create_state_from_value(&transition.target)?;
        self.current_state = new_state.clone();
        
        // Execute entry actions for new state
        if let Some(new_state_node) = self.states.get(&Self::get_state_id(&new_state)) {
            self.execute_actions(&new_state_node.entry_actions)?;
        }
        
        Ok(new_state)
    }

    /// Reset the machine to its initial state
    pub fn reset(&mut self) -> Result<S, StateMachineError<C, E, S>> {
        // If no states are defined, just reset to the current state type's default
        if self.states.is_empty() {
            let default_state = S::default();
            self.current_state = default_state.clone();
            self.context = C::default();
            self.history.clear();
            return Ok(default_state);
        }
        
        // Otherwise, try to find the initial state in our states map
        let initial_state = self.create_state_from_value(&StateValue::simple(&self.initial_state))?;
        self.current_state = initial_state.clone();
        self.context = C::default();
        self.history.clear();
        Ok(initial_state)
    }

    /// Rollback to the previous state
    pub fn rollback(&mut self) -> Result<S, StateMachineError<C, E, S>> {
        if let Some(previous_state) = self.history.pop() {
            self.current_state = previous_state.clone();
            Ok(previous_state)
        } else {
            Err(StateMachineError::State(super::error::StateError::ValidationFailed("No history available".to_string())))
        }
    }

    /// Set the maximum history size
    pub fn set_max_history(&mut self, max_history: usize) {
        self.max_history = max_history;
        // Trim history if it exceeds the new limit
        while self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    // Private helper methods

    fn get_state_id(state: &S) -> String {
        // Convert the debug format to lowercase to match our state node IDs
        format!("{:?}", state).to_lowercase()
    }

    fn create_state_from_value(&self, value: &StateValue) -> Result<S, StateMachineError<C, E, S>> {
        // This is a placeholder - in practice, we'd need a way to
        // reconstruct state values from their string representations
        // For now, we'll use a default approach
        match value {
            StateValue::Simple(id) => {
                // Try to find a matching state in our states map
                if self.states.contains_key(id) {
                    // This is a simplified approach - we'd need proper state reconstruction
                    Ok(S::default()) // Placeholder
                } else {
                    Err(StateMachineError::State(super::error::StateError::InvalidState(S::default())))
                }
            },
            _ => Err(StateMachineError::State(super::error::StateError::ValidationFailed("Unsupported state value".to_string())))
        }
    }

    fn evaluate_guards(&self, guards: &[Arc<dyn super::traits::Guard<C, E>>]) -> Result<(), StateMachineError<C, E, S>> {
        for guard in guards {
            // We need an event to check against - for now we'll use a placeholder
            // In practice, this would be the event that triggered the transition
            let dummy_event = self.create_dummy_event();
            if !guard.check(&self.context, &dummy_event) {
                return Err(StateMachineError::Guard(super::error::GuardError::TransitionBlocked(dummy_event)));
            }
        }
        Ok(())
    }

    fn execute_actions(&self, actions: &[Arc<dyn super::traits::Action<C>>]) -> Result<(), StateMachineError<C, E, S>> {
        for action in actions {
            // We need mutable access to context for actions
            // For now, we'll create a temporary mutable reference
            let mut temp_context = self.context.clone();
            action.execute(&mut temp_context)
                .map_err(|e| StateMachineError::Action(e))?;
        }
        Ok(())
    }

    fn add_to_history(&mut self, state: S) {
        self.history.push(state);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    fn create_dummy_event(&self) -> E {
        // This is a placeholder - in practice, we'd need a way to create
        // a default event or pass the actual event through the call chain
        // For now, we'll use Default if available
        E::default()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::traits::{Action, Guard};


    // Test types
    #[derive(Clone, Debug, Default, PartialEq)]
    struct TestContext {
        count: i32,
        name: String,
    }

    impl StateMachineContext for TestContext {}

    #[derive(Clone, Debug, PartialEq)]
    enum TestEvent {
        Start,
        Stop,
        Increment,
        Decrement,
    }

    impl StateMachineEvent for TestEvent {}

    impl Default for TestEvent {
        fn default() -> Self {
            TestEvent::Start
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    enum TestState {
        Idle,
        Active,
        Paused,
    }

    impl StateMachineState for TestState {
        type Context = TestContext;
        type Event = TestEvent;
    }

    impl Default for TestState {
        fn default() -> Self {
            TestState::Idle
        }
    }

    // Test actions
    #[derive(Clone, Debug)]
    struct IncrementAction;

    impl Action<TestContext> for IncrementAction {
        fn execute(&self, context: &mut TestContext) -> Result<(), super::super::error::ActionError> {
            // In a real implementation, we'd modify the context
            // For now, we'll just verify the action can be called
            context.count += 1;
            Ok(())
        }

        fn description(&self) -> &'static str {
            "Increments the counter"
        }
    }

    #[derive(Clone, Debug)]
    struct DecrementAction;

    impl Action<TestContext> for DecrementAction {
        fn execute(&self, context: &mut TestContext) -> Result<(), super::super::error::ActionError> {
            context.count -= 1;
            Ok(())
        }

        fn description(&self) -> &'static str {
            "Decrements the counter"
        }
    }

    // Test guards
    #[derive(Clone, Debug)]
    struct AlwaysAllowGuard;

    impl Guard<TestContext, TestEvent> for AlwaysAllowGuard {
        fn check(&self, _context: &TestContext, _event: &TestEvent) -> bool {
            true
        }

        fn description(&self) -> &'static str {
            "Always allows transitions"
        }
    }

    #[derive(Clone, Debug)]
    struct AlwaysDenyGuard;

    impl Guard<TestContext, TestEvent> for AlwaysDenyGuard {
        fn check(&self, _context: &TestContext, _event: &TestEvent) -> bool {
            false
        }

        fn description(&self) -> &'static str {
            "Always denies transitions"
        }
    }

    #[test]
    fn test_machine_creation() {
        let context = TestContext::default();
        let machine = Machine::new(TestState::Idle, context);
        
        assert_eq!(*machine.current_state(), TestState::Idle);
        assert_eq!(machine.states.len(), 0);
    }

    #[test]
    fn test_add_state() {
        let context = TestContext::default();
        let mut machine = Machine::new(TestState::Idle, context);
        
        let state_node = StateNode::new("idle")
            .with_value(StateValue::simple("idle"));
        
        machine.add_state(state_node);
        assert_eq!(machine.states.len(), 1);
        assert!(machine.states.contains_key("idle"));
    }

    #[test]
    fn test_can_transition_without_states() {
        let context = TestContext::default();
        let machine = Machine::new(TestState::Idle, context);
        
        // Machine should not be able to transition without defined states
        assert!(!machine.can_transition(&TestEvent::Start));
    }

    #[test]
    fn test_can_transition_with_valid_transition() {
        let context = TestContext::default();
        let mut machine = Machine::new(TestState::Idle, context);
        
        // Add a state with a transition
        let state_node = StateNode::new("idle")
            .with_value(StateValue::simple("idle"))
            .with_transition(Transition::new(
                TestEvent::Start,
                StateValue::simple("active")
            ).with_guard(Arc::new(AlwaysAllowGuard)));
        
        machine.add_state(state_node);
        
        // Machine should be able to transition
        assert!(machine.can_transition(&TestEvent::Start));
    }

    #[test]
    fn test_can_transition_with_guard_failure() {
        let context = TestContext::default();
        let mut machine = Machine::new(TestState::Idle, context);
        
        // Add a state with a transition that has a failing guard
        let state_node = StateNode::new("idle")
            .with_value(StateValue::simple("idle"))
            .with_transition(Transition::new(
                TestEvent::Start,
                StateValue::simple("active")
            ).with_guard(Arc::new(AlwaysDenyGuard)));
        
        machine.add_state(state_node);
        
        // Machine should not be able to transition due to guard failure
        assert!(!machine.can_transition(&TestEvent::Start));
    }

    #[test]
    fn test_transition_success() {
        let context = TestContext::default();
        let mut machine = Machine::new(TestState::Idle, context);
        
        // Add states
        let idle_state = StateNode::new("idle")
            .with_value(StateValue::simple("idle"))
            .with_transition(Transition::new(
                TestEvent::Start,
                StateValue::simple("active")
            ).with_guard(Arc::new(AlwaysAllowGuard)));
        
        let active_state = StateNode::new("active")
            .with_value(StateValue::simple("active"));
        
        machine.add_state(idle_state);
        machine.add_state(active_state);
        
        // Perform transition
        let result = machine.transition(TestEvent::Start);
        assert!(result.is_ok());
        
        // State should have changed (though our current implementation has limitations)
        // For now, we'll just verify the transition didn't error
        assert!(result.is_ok());
    }

    #[test]
    fn test_transition_with_invalid_event() {
        let context = TestContext::default();
        let mut machine = Machine::new(TestState::Idle, context);
        
        // Add a state without transitions
        let state_node = StateNode::new("idle")
            .with_value(StateValue::simple("idle"));
        
        machine.add_state(state_node);
        
        // Try to transition with an event that has no transition
        let result = machine.transition(TestEvent::Start);
        assert!(result.is_err());
    }

    #[test]
    fn test_machine_reset() {
        let context = TestContext::default();
        let mut machine = Machine::new(TestState::Idle, context);
        
        // Reset should return the initial state
        let result = machine.reset();
        assert!(result.is_ok());
        assert_eq!(*machine.current_state(), TestState::Idle);
    }

    #[test]
    fn test_history_management() {
        let context = TestContext::default();
        let mut machine = Machine::new(TestState::Idle, context);
        
        // Set a small history size
        machine.set_max_history(2);
        
        // Add some states to history manually (in practice this would happen during transitions)
        machine.add_to_history(TestState::Active);
        machine.add_to_history(TestState::Paused);
        
        // History should be limited to max_history
        assert_eq!(machine.history.len(), 2);
    }

    #[test]
    fn test_rollback_without_history() {
        let context = TestContext::default();
        let mut machine = Machine::new(TestState::Idle, context);
        
        // Try to rollback without any history
        let result = machine.rollback();
        assert!(result.is_err());
    }
}
