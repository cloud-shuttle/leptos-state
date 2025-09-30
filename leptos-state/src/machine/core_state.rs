use super::core_errors::MachineError;
use super::types_basic::StateType;
use std::collections::HashMap;

/// State node definition
#[derive(Debug)]
pub struct StateNode<S, E, C>
where
    S: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static + std::hash::Hash + Eq,
    C: Clone + Send + Sync + 'static,
{
    pub name: String,
    pub state_type: StateType,
    pub transitions: HashMap<E, String>,
    pub entry_actions: Vec<String>,
    pub exit_actions: Vec<String>,
    pub children: HashMap<String, StateNode<S, E, C>>,
    pub context: C,
    pub parent: Option<String>,
    pub data: Option<S>,
}

impl<S, E, C> StateNode<S, E, C>
where
    S: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static + std::hash::Hash + Eq,
    C: Clone + PartialEq + Send + Sync + 'static,
{
    pub fn new(name: String, state_type: StateType, context: C) -> Self {
        Self {
            name,
            state_type,
            transitions: HashMap::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            children: HashMap::new(),
            context,
            parent: None,
            data: None,
        }
    }

    pub fn add_transition(&mut self, event: E, target: String) -> &mut Self {
        self.transitions.insert(event, target);
        self
    }

    pub fn get_transition(&self, event: &E) -> Result<&String, MachineError> {
        self.transitions
            .get(event)
            .ok_or(MachineError::InvalidTransition)
    }

    pub fn is_compound(&self) -> bool {
        matches!(self.state_type, StateType::Compound)
    }

    pub fn is_atomic(&self) -> bool {
        matches!(self.state_type, StateType::Atomic)
    }
}
