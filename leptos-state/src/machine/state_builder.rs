use super::*;
use std::collections::HashMap;

/// State builder for fluent API
pub struct StateBuilder<C: Send + Sync, E: Send + Sync> {
    machine_builder: MachineBuilder<C, E>,
    current_state: String,
    transitions: Vec<Transition<C, E>>,
    entry_actions: Vec<Box<dyn Action<C, E>>>,
    exit_actions: Vec<Box<dyn Action<C, E>>>,
    child_states: HashMap<String, StateNode<C, E, C>>,
    initial_child: Option<String>,
}

impl<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> StateBuilder<C, E> {
    pub fn new(machine_builder: MachineBuilder<C, E>, state_id: String) -> Self {
        Self {
            machine_builder,
            current_state: state_id,
            transitions: Vec::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            child_states: HashMap::new(),
            initial_child: None,
        }
    }

    pub fn on(self, event: E, target: &str) -> TransitionBuilder<C, E> {
        TransitionBuilder::new(self, event, target.to_string())
    }

    pub fn on_entry<A: Action<C, E> + 'static>(mut self, action: A) -> Self {
        self.entry_actions.push(Box::new(action));
        self
    }

    pub fn on_exit<A: Action<C, E> + 'static>(mut self, action: A) -> Self {
        self.exit_actions.push(Box::new(action));
        self
    }

    /// Add a function-based entry action
    pub fn on_entry_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut C, &E) + Send + Sync + 'static,
    {
        self.entry_actions
            .push(Box::new(actions::FunctionAction::new(func)));
        self
    }

    /// Add a function-based exit action
    pub fn on_exit_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut C, &E) + Send + Sync + 'static,
    {
        self.exit_actions
            .push(Box::new(actions::FunctionAction::new(func)));
        self
    }

    /// Add a log entry action
    pub fn on_entry_log(mut self, message: impl Into<String>) -> Self
    where
        C: std::fmt::Debug,
        E: std::fmt::Debug,
    {
        self.entry_actions
            .push(Box::new(actions::LogAction::new(message)));
        self
    }

    /// Add a log exit action
    pub fn on_exit_log(mut self, message: impl Into<String>) -> Self
    where
        C: std::fmt::Debug,
        E: std::fmt::Debug,
    {
        self.exit_actions
            .push(Box::new(actions::LogAction::new(message)));
        self
    }

    /// Add a pure entry action (no context modification)
    pub fn on_entry_pure<F>(mut self, func: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.entry_actions
            .push(Box::new(actions::PureAction::new(func)));
        self
    }

    /// Add a pure exit action (no context modification)
    pub fn on_exit_pure<F>(mut self, func: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.exit_actions
            .push(Box::new(actions::PureAction::new(func)));
        self
    }

    /// Add a child state (for hierarchical states)
    pub fn child_state(self, id: &str) -> ChildStateBuilder<C, E> {
        ChildStateBuilder::new(self, id.to_string())
    }

    /// Set the initial child state
    pub fn initial_child(mut self, child_id: &str) -> Self {
        self.initial_child = Some(child_id.to_string());
        self
    }

    pub fn state(mut self, id: &str) -> StateBuilder<C, E> {
        // Finish current state
        let state_node = StateNode {
            id: self.current_state.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: self.child_states,
            initial_child: self.initial_child,
            _phantom: std::marker::PhantomData,
        };

        self.machine_builder
            .states
            .insert(self.current_state, state_node);

        // Start new state
        StateBuilder::new(self.machine_builder, id.to_string())
    }

    pub fn initial(self, state_id: &str) -> MachineBuilder<C, E> {
        // Finish current state
        let state_node = StateNode {
            id: self.current_state.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: self.child_states,
            initial_child: self.initial_child,
            _phantom: std::marker::PhantomData,
        };

        let mut builder = self.machine_builder;
        builder.states.insert(self.current_state, state_node);
        builder.initial(state_id)
    }

    pub fn build(self) -> Machine<C, E, C> {
        // Finish current state
        let state_node = StateNode {
            id: self.current_state.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: self.child_states,
            initial_child: self.initial_child,
            _phantom: std::marker::PhantomData,
        };

        let mut builder = self.machine_builder;
        builder.states.insert(self.current_state, state_node);
        builder.build()
    }
}
