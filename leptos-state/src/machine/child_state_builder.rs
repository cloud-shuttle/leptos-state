use super::*;
use std::collections::HashMap;

/// Builder for child states in hierarchical machines
pub struct ChildStateBuilder<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> {
    pub parent_builder: StateBuilder<C, E>,
    pub child_id: String,
    pub transitions: Vec<Transition<C, E>>,
    pub entry_actions: Vec<Box<dyn Action<C, E>>>,
    pub exit_actions: Vec<Box<dyn Action<C, E>>>,
}

impl<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> ChildStateBuilder<C, E> {
    pub fn new(parent_builder: StateBuilder<C, E>, child_id: String) -> Self {
        Self {
            parent_builder,
            child_id,
            transitions: Vec::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
        }
    }

    pub fn on(self, event: E, target: &str) -> ChildTransitionBuilder<C, E> {
        ChildTransitionBuilder::new(self, event, target.to_string())
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
        F: Fn(&mut C, &E) + Clone + Send + Sync + 'static,
    {
        self.entry_actions
            .push(Box::new(actions::FunctionAction::new(func)));
        self
    }

    /// Add a function-based exit action
    pub fn on_exit_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut C, &E) + Clone + Send + Sync + 'static,
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
        F: Fn() + Clone + Send + Sync + 'static,
    {
        self.entry_actions
            .push(Box::new(actions::PureAction::new(func)));
        self
    }

    /// Add a pure exit action (no context modification)
    pub fn on_exit_pure<F>(mut self, func: F) -> Self
    where
        F: Fn() + Clone + Send + Sync + 'static,
    {
        self.exit_actions
            .push(Box::new(actions::PureAction::new(func)));
        self
    }

    pub fn child_state(self, id: &str) -> ChildStateBuilder<C, E> {
        // Finish current child state
        let child_node = StateNode {
            id: self.child_id.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: HashMap::new(),
            initial_child: None,
            _phantom: std::marker::PhantomData,
        };

        let mut parent = self.parent_builder;
        parent.child_states.insert(self.child_id, child_node);

        // Start new child state
        parent.child_state(id)
    }

    pub fn parent(self) -> StateBuilder<C, E> {
        // Finish current child state and return to parent
        let child_node = StateNode {
            id: self.child_id.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: HashMap::new(),
            initial_child: None,
            _phantom: std::marker::PhantomData,
        };

        let mut parent = self.parent_builder;
        parent.child_states.insert(self.child_id, child_node);
        parent
    }
}
