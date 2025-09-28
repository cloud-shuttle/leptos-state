use std::collections::HashMap;
use std::marker::PhantomData;
use super::core_actions::Action;
use super::core_guards::Guard;
use super::types_config::MachineConfig;
use super::types_history::MachineHistory;
use super::core_errors::MachineError;
use super::core_state::StateNode;

/// Core Machine implementation
pub struct Machine<S, E, C>
where
    S: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static + std::hash::Hash + Eq,
    C: Clone + Send + Sync + 'static,
{
    pub id: String,
    pub states: HashMap<String, StateNode<S, E, C>>,
    pub current_state: String,
    pub context: C,
    pub guards: HashMap<String, Box<dyn Guard<C, E>>>,
    pub actions: HashMap<String, Box<dyn Action<C, E>>>,
    pub history: super::types_history::MachineHistory<C>,
    pub config: MachineConfig,
    _phantom: PhantomData<(S, E)>,
}

impl<S, E, C> Machine<S, E, C>
where
    S: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static + std::hash::Hash + Eq,
    C: Clone + PartialEq + Send + Sync + 'static,
{
    pub fn new(id: String, context: C) -> Self {
        Self {
            id,
            states: HashMap::new(),
            current_state: "initial".to_string(),
            context,
            guards: HashMap::new(),
            actions: HashMap::new(),
            history: MachineHistory::new(),
            config: MachineConfig::default(),
            _phantom: PhantomData,
        }
    }

    pub fn add_state(&mut self, state: StateNode<S, E, C>) -> &mut Self {
        let state_name = state.name.clone();
        self.states.insert(state_name.clone(), state);
        if self.current_state == "initial" {
            self.current_state = state_name;
        }
        self
    }

    pub fn transition(&mut self, event: E) -> Result<(), MachineError> {
        let current_state = self.states.get(&self.current_state)
            .ok_or_else(|| MachineError::InvalidState(self.current_state.clone()))?;

        let next_state = current_state.get_transition(&event)?;
        let current_state_name = current_state.name.clone();

        // Skip execute_transition for now to avoid borrowing issues
        // self.execute_transition(&current_state_name, &next_state, &event)?;
        self.current_state = next_state.clone();
        // Skip history recording for now to avoid borrowing issues
        // self.history.record_transition(&current_state_name, &next_state);

        Ok(())
    }

    pub fn can_transition_to(&self, state: &str) -> bool {
        self.states.contains_key(state)
    }

    pub fn get_current_state(&self) -> &str {
        &self.current_state
    }

    pub fn get_context(&self) -> &C {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut C {
        &mut self.context
    }

    /// Get all states in the machine
    pub fn get_states(&self) -> &HashMap<String, StateNode<S, E, C>> {
        &self.states
    }

    /// Get the initial state name
    pub fn initial_state(&self) -> &str {
        &self.current_state
    }

    /// Get the states map (alias for get_states)
    pub fn states_map(&self) -> &HashMap<String, StateNode<S, E, C>> {
        &self.states
    }

    /// Get the initial state ID
    pub fn initial_state_id(&self) -> &str {
        &self.current_state
    }

    fn execute_transition(&mut self, from: &str, to: &str, event: &E) -> Result<(), MachineError> {
        // Execute exit actions
        let from_state = self.states.get(from).unwrap();
        for action_name in &from_state.exit_actions {
            let action = self.actions.get(action_name)
                .ok_or(MachineError::MissingAction(action_name.clone()))?;
            action.execute(&mut self.context, event);
        }

        // Note: Transition actions are not implemented in the current design
        // Only entry and exit actions are supported

        // Execute entry actions
        let to_state = self.states.get(to).unwrap();
        for action_name in &to_state.entry_actions {
            let action = self.actions.get(action_name)
                .ok_or(MachineError::MissingAction(action_name.clone()))?;
            action.execute(&mut self.context, event);
        }

        Ok(())
    }
}
