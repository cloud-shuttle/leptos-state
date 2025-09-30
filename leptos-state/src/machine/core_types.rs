use super::*;
use crate::machine::states::StateValue;
use crate::StateResult;
use std::collections::HashMap;

/// State node in the machine definition
#[derive(Debug)]
pub struct StateNode<
    C: Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + 'static,
    S: Clone + std::fmt::Debug,
> {
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
pub struct Transition<
    C: Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + 'static,
> {
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

    /// Export a diagram of the machine
    pub fn export_diagram(
        &self,
        _format: (), // crate::machine::visualization::ExportFormat,
    ) -> StateResult<String> {
        // Note: Machine doesn't implement Clone, so we can't use visualization in this context
        // This would need to be addressed in a future iteration
        Err(crate::utils::types::StateError::new(
            "Visualization not available - Machine doesn't implement Clone",
        ))
    }

    pub fn initial_state(&self) -> MachineStateImpl<C>
    where
        C: Default,
    {
        MachineStateImpl {
            value: StateValue::Simple(self.initial.clone()),
            context: Default::default(),
        }
    }

    pub fn initial_with_context(&self, context: C) -> MachineStateImpl<C> {
        MachineStateImpl {
            value: StateValue::Simple(self.initial.clone()),
            context,
        }
    }

    /// Transition from one state to another based on an event
    pub fn transition(&self, state: &MachineStateImpl<C>, event: E) -> MachineStateImpl<C>
    where
        E: PartialEq,
    {
        match &state.value() {
            StateValue::Simple(id) => self.transition_simple(state, id, event),
            StateValue::Compound { parent, child } => {
                self.transition_hierarchical(state, parent, child, event)
            }
            StateValue::Parallel(states) => {
                // Handle parallel states by transitioning each active region
                let mut new_states = Vec::new();
                let mut context = state.context().clone();

                for parallel_state in states {
                    let temp_state = MachineStateImpl {
                        value: parallel_state.clone(),
                        context: context.clone(),
                    };
                    let transitioned = self.transition(&temp_state, event.clone());
                    new_states.push(transitioned.value().clone());
                    context = transitioned.context().clone();
                }

                MachineStateImpl {
                    value: StateValue::Parallel(new_states),
                    context,
                }
            }
        }
    }

    fn transition_simple(
        &self,
        state: &MachineStateImpl<C>,
        state_id: &str,
        event: E,
    ) -> MachineStateImpl<C>
    where
        E: PartialEq,
    {
        if let Some(state_node) = self.states.get(state_id) {
            // Look for a matching transition
            for transition in &state_node.transitions {
                if transition.event == event {
                    // Check all guards
                    let guards_pass = transition
                        .guards
                        .iter()
                        .all(|guard| guard.check(state.context(), &event));

                    if guards_pass {
                        let mut new_context = state.context().clone();

                        // Execute transition actions
                        for action in &transition.actions {
                            action.execute(&mut new_context, &event);
                        }

                        // Execute exit actions for current state
                        for action in &state_node.exit_actions {
                            action.execute(&mut new_context, &event);
                        }

                        // Determine target state value (simple or compound)
                        let new_value = self.resolve_target_state(&transition.target);

                        let new_state = MachineStateImpl {
                            value: new_value,
                            context: new_context,
                        };

                        // Execute entry actions for target state
                        return self.execute_entry_actions(new_state, &transition.target, &event);
                    }
                }
            }
        }

        // No valid transition found, return current state
        state.clone()
    }

    fn transition_hierarchical(
        &self,
        state: &MachineStateImpl<C>,
        parent_id: &str,
        child: &StateValue,
        event: E,
    ) -> MachineStateImpl<C>
    where
        E: PartialEq,
    {
        // First try child state transitions
        let child_state = MachineStateImpl {
            value: (*child).clone(),
            context: state.context().clone(),
        };

        let child_transitioned = self.transition(&child_state, event.clone());

        // If child transitioned, update the compound state
        if child_transitioned.value() != child {
            return MachineStateImpl {
                value: StateValue::Compound {
                    parent: parent_id.to_string(),
                    child: Box::new(child_transitioned.value().clone()),
                },
                context: child_transitioned.context().clone(),
            };
        }

        // If child didn't transition, try parent transitions
        self.transition_simple(state, parent_id, event)
    }

    fn resolve_target_state(&self, target: &str) -> StateValue {
        if let Some(state_node) = self.states.get(target) {
            if !state_node.child_states.is_empty() {
                // This is a compound state, resolve initial child
                if let Some(initial_child) = &state_node.initial_child {
                    return StateValue::Compound {
                        parent: target.to_string(),
                        child: Box::new(self.resolve_target_state(initial_child)),
                    };
                }
            }
        }

        StateValue::Simple(target.to_string())
    }

    fn execute_entry_actions(
        &self,
        mut state: MachineStateImpl<C>,
        target_id: &str,
        event: &E,
    ) -> MachineStateImpl<C> {
        if let Some(target_node) = self.states.get(target_id) {
            for action in &target_node.entry_actions {
                action.execute(&mut state.context, event);
            }
        }

        state
    }
}
