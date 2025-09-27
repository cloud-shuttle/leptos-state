use crate::machine::states::StateValue;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Core trait for state machines
pub trait StateMachine: Sized + 'static {
    type Context: Clone + PartialEq + Send + Sync + 'static;
    type Event: Clone + Send + Sync + 'static;
    type State: MachineState<Context = Self::Context> + Clone + Send + Sync + 'static;

    fn initial() -> Self::State;
    fn transition(state: &Self::State, event: Self::Event) -> Self::State;
}

/// Main builder trait for constructing state machines
pub trait MachineBuilder {
    type State;
    type Event;
    type Context;

    fn new() -> Self;
    fn state<Name: Into<String>>(self, name: Name) -> Self;
    fn initial<Name: Into<String>>(self, state: Name) -> Self;
    fn transition<E, S>(self, from: S, event: E, to: S) -> Self
    where
        S: Into<String> + Clone,
        E: Into<Self::Event>;
    fn build_with_context(self, context: Self::Context) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>;
    fn build(self) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>
    where
        Self::Context: Default,
    {
        self.build_with_context(Self::Context::default())
    }
}

/// Trait for machine states
pub trait MachineState {
    type Context: Send + Sync + 'static;

    fn value(&self) -> &StateValue;
    fn context(&self) -> &Self::Context;
    fn matches(&self, pattern: &str) -> bool;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any;
}

/// Core Machine implementation
#[derive(Debug)]
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
    pub guards: HashMap<String, Box<dyn Guard<C>>>,
    pub actions: HashMap<String, Box<dyn Action<C>>>,
    pub history: MachineHistory,
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
            .ok_or(MachineError::InvalidState)?;

        let next_state = current_state.get_transition(&event)?;

        self.execute_transition(&current_state.name, &next_state)?;
        self.current_state = next_state.clone();
        self.history.record_transition(&current_state.name, &next_state);

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

    fn execute_transition(&mut self, from: &str, to: &str) -> Result<(), MachineError> {
        // Execute exit actions
        let from_state = self.states.get(from).unwrap();
        for action_name in &from_state.exit_actions {
            let action = self.actions.get(action_name)
                .ok_or(MachineError::MissingAction(action_name.clone()))?;
            action.execute(&mut self.context)?;
        }

        // Execute transition actions
        for action_name in &from_state.transition_actions {
            let action = self.actions.get(action_name)
                .ok_or(MachineError::MissingAction(action_name.clone()))?;
            action.execute(&mut self.context)?;
        }

        // Execute entry actions
        let to_state = self.states.get(to).unwrap();
        for action_name in &to_state.entry_actions {
            let action = self.actions.get(action_name)
                .ok_or(MachineError::MissingAction(action_name.clone()))?;
            action.execute(&mut self.context)?;
        }

        Ok(())
    }
}

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
    pub fn new(name: String, state_type: StateType) -> Self {
        Self {
            name,
            state_type,
            transitions: HashMap::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            children: HashMap::new(),
            parent: None,
            data: None,
        }
    }

    pub fn add_transition(&mut self, event: E, target: String) -> &mut Self {
        self.transitions.insert(event, target);
        self
    }

    pub fn get_transition(&self, event: &E) -> Result<&String, MachineError> {
        self.transitions.get(event)
            .ok_or(MachineError::InvalidTransition)
    }

    pub fn is_compound(&self) -> bool {
        matches!(self.state_type, StateType::Compound)
    }

    pub fn is_atomic(&self) -> bool {
        matches!(self.state_type, StateType::Atomic)
    }
}

/// State types
#[derive(Debug, Clone, PartialEq)]
pub enum StateType {
    Atomic,
    Compound,
    Parallel,
    History,
    Final,
}

/// Machine configuration
#[derive(Clone, Debug)]
pub struct MachineConfig {
    pub strict_mode: bool,
    pub auto_cleanup: bool,
    pub max_history_size: usize,
}

/// Machine history tracking
#[derive(Clone, Debug, Default)]
pub struct MachineHistory {
    transitions: Vec<(String, String, std::time::SystemTime)>,
}

impl MachineHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_transition(&mut self, from: &str, to: &str) {
        self.transitions.push((
            from.to_string(),
            to.to_string(),
            std::time::SystemTime::now(),
        ));
    }

    pub fn get_transitions(&self) -> &[(String, String, std::time::SystemTime)] {
        &self.transitions
    }

    pub fn clear(&mut self) {
        self.transitions.clear();
    }
}

/// Guard trait for conditional transitions
pub trait Guard<C>: Send + Sync {
    fn evaluate(&self, context: &C) -> Result<bool, MachineError>;
    fn name(&self) -> &str;
}

/// Action trait for state changes
pub trait Action<C>: Send + Sync {
    fn execute(&self, context: &mut C) -> Result<(), MachineError>;
    fn name(&self) -> &str;
}

/// Machine errors
#[derive(Debug, Clone)]
pub enum MachineError {
    InvalidState(String),
    InvalidTransition,
    GuardFailed(String),
    MissingGuard(String),
    MissingAction(String),
    ContextError(String),
}

impl std::fmt::Display for MachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineError::InvalidState(s) => write!(f, "Invalid state: {}", s),
            MachineError::InvalidTransition => write!(f, "Invalid transition"),
            MachineError::GuardFailed(s) => write!(f, "Guard failed: {}", s),
            MachineError::MissingGuard(s) => write!(f, "Missing guard: {}", s),
            MachineError::MissingAction(s) => write!(f, "Missing action: {}", s),
            MachineError::ContextError(s) => write!(f, "Context error: {}", s),
        }
    }
}

impl std::error::Error for MachineError {}

pub type MachineResult<T> = Result<T, MachineError>;

/// Default implementations for common types
impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            auto_cleanup: true,
            max_history_size: 100,
        }
    }
}

/// Helper macros for machine creation
#[macro_export]
macro_rules! machine_state {
    ($name:expr, $state_type:expr) => {
        StateNode::new($name.to_string(), $state_type)
    };
}

#[macro_export]
macro_rules! machine_transition {
    ($state:expr, $event:expr, $target:expr) => {
        $state.add_transition($event, $target.to_string())
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::machine::MachineState;

    #[derive(Clone, Debug, PartialEq)]
    struct TestState {
        value: i32,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum TestEvent {
        Increment,
        Decrement,
    }

    #[derive(Clone, Debug, PartialEq)]
    struct TestContext {
        counter: i32,
    }

    impl Default for TestContext {
        fn default() -> Self {
            Self { counter: 0 }
        }
    }

    #[test]
    fn machine_creation_works() {
        let machine = Machine::<TestState, TestEvent, TestContext>::new("test".to_string());
        assert_eq!(machine.id, "test");
    }

    #[test]
    fn state_transitions_work() {
        let mut machine = Machine::<TestState, TestEvent, TestContext>::new("counter".to_string());
        let idle_state = StateNode::new("idle".to_string(), StateType::Atomic);
        machine.add_state(idle_state);

        assert!(machine.can_transition_to("idle"));
    }
}
