# 🤖 Machine Core Design

## Overview
Finite state machine implementation with hierarchical states, guards, actions, and transitions.

## Architecture

### Core Components
```
machine/
├── core.rs        # Machine struct and state management (200 lines)
├── states.rs      # State definitions and hierarchy (150 lines)
├── transitions.rs # Transition logic and validation (120 lines)
├── guards.rs      # Guard evaluation system (100 lines)
├── actions.rs     # Action execution framework (120 lines)
└── types.rs       # Type definitions (80 lines)
```

## Core Machine Struct

```rust
pub struct Machine {
    id: String,
    states: HashMap<String, StateNode>,
    current_state: String,
    context: Context,
    guards: HashMap<String, Box<dyn Guard>>,
    actions: HashMap<String, Box<dyn Action>>,
    history: MachineHistory,
    config: MachineConfig,
}

impl Machine {
    pub fn new(id: String) -> Self {
        Self {
            id,
            states: HashMap::new(),
            current_state: "initial".to_string(),
            context: Context::new(),
            guards: HashMap::new(),
            actions: HashMap::new(),
            history: MachineHistory::new(),
            config: MachineConfig::default(),
        }
    }

    pub fn add_state(&mut self, state: StateNode) -> &mut Self {
        let state_name = state.name.clone();
        self.states.insert(state_name.clone(), state);
        if self.current_state == "initial" {
            self.current_state = state_name;
        }
        self
    }

    pub fn transition(&mut self, event: &str) -> Result<(), MachineError> {
        let current_state = self.states.get(&self.current_state)
            .ok_or(MachineError::InvalidState)?;

        let next_state = current_state.get_transition(event)?;

        self.execute_transition(&current_state.name, &next_state)?;
        self.current_state = next_state.clone();
        self.history.record_transition(&current_state.name, &next_state);

        Ok(())
    }

    pub fn can_transition_to(&self, state: &str) -> bool {
        self.states.contains_key(state)
    }
}
```

## State Management

```rust
#[derive(Clone, Debug)]
pub struct StateNode {
    pub name: String,
    pub state_type: StateType,
    pub transitions: HashMap<String, String>,
    pub entry_actions: Vec<String>,
    pub exit_actions: Vec<String>,
    pub children: HashMap<String, StateNode>,
    pub parent: Option<String>,
}

impl StateNode {
    pub fn new(name: String, state_type: StateType) -> Self {
        Self {
            name,
            state_type,
            transitions: HashMap::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            children: HashMap::new(),
            parent: None,
        }
    }

    pub fn add_transition(&mut self, event: String, target: String) -> &mut Self {
        self.transitions.insert(event, target);
        self
    }

    pub fn get_transition(&self, event: &str) -> Result<&String, MachineError> {
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
```

## Transition System

```rust
pub struct TransitionEngine {
    machine: Arc<RwLock<Machine>>,
}

impl TransitionEngine {
    pub fn new(machine: Arc<RwLock<Machine>>) -> Self {
        Self { machine }
    }

    pub fn validate_transition(&self, from: &str, to: &str, event: &str) -> Result<(), MachineError> {
        let machine = self.machine.read().unwrap();

        // Check if target state exists
        if !machine.states.contains_key(to) {
            return Err(MachineError::InvalidState(to.to_string()));
        }

        // Check if transition is allowed from current state
        let current_state = machine.states.get(from)
            .ok_or(MachineError::InvalidState(from.to_string()))?;

        if !current_state.transitions.contains_key(event) {
            return Err(MachineError::InvalidTransition);
        }

        // Evaluate guards
        let target_state = machine.states.get(to).unwrap();
        for guard_name in &target_state.entry_guards {
            let guard = machine.guards.get(guard_name)
                .ok_or(MachineError::MissingGuard(guard_name.clone()))?;

            if !guard.evaluate(&machine.context)? {
                return Err(MachineError::GuardFailed(guard_name.clone()));
            }
        }

        Ok(())
    }

    pub fn execute_transition(&self, from: &str, to: &str) -> Result<(), MachineError> {
        let mut machine = self.machine.write().unwrap();

        // Execute exit actions
        let from_state = machine.states.get(from).unwrap();
        for action_name in &from_state.exit_actions {
            let action = machine.actions.get(action_name)
                .ok_or(MachineError::MissingAction(action_name.clone()))?;
            action.execute(&mut machine.context)?;
        }

        // Execute transition actions
        for action_name in &from_state.transition_actions {
            let action = machine.actions.get(action_name)
                .ok_or(MachineError::MissingAction(action_name.clone()))?;
            action.execute(&mut machine.context)?;
        }

        // Execute entry actions
        let to_state = machine.states.get(to).unwrap();
        for action_name in &to_state.entry_actions {
            let action = machine.actions.get(action_name)
                .ok_or(MachineError::MissingAction(action_name.clone()))?;
            action.execute(&mut machine.context)?;
        }

        Ok(())
    }
}
```

## Guard System

```rust
pub trait Guard: Send + Sync {
    fn evaluate(&self, context: &Context) -> Result<bool, MachineError>;
    fn name(&self) -> &str;
}

pub struct ConditionGuard {
    name: String,
    condition: Box<dyn Fn(&Context) -> bool + Send + Sync>,
}

impl ConditionGuard {
    pub fn new<F>(name: String, condition: F) -> Self
    where
        F: Fn(&Context) -> bool + Send + Sync + 'static,
    {
        Self {
            name,
            condition: Box::new(condition),
        }
    }
}

impl Guard for ConditionGuard {
    fn evaluate(&self, context: &Context) -> Result<bool, MachineError> {
        Ok((self.condition)(context))
    }

    fn name(&self) -> &str {
        &self.name
    }
}
```

## Action System

```rust
pub trait Action: Send + Sync {
    fn execute(&self, context: &mut Context) -> Result<(), MachineError>;
    fn name(&self) -> &str;
}

pub struct FunctionAction {
    name: String,
    action: Box<dyn Fn(&mut Context) -> Result<(), MachineError> + Send + Sync>,
}

impl FunctionAction {
    pub fn new<F>(name: String, action: F) -> Self
    where
        F: Fn(&mut Context) -> Result<(), MachineError> + Send + Sync + 'static,
    {
        Self {
            name,
            action: Box::new(action),
        }
    }
}

impl Action for FunctionAction {
    fn execute(&self, context: &mut Context) -> Result<(), MachineError> {
        (self.action)(context)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
```

## Type Definitions

```rust
#[derive(Debug, Clone)]
pub enum StateType {
    Atomic,
    Compound,
    Parallel,
    History,
    Final,
}

#[derive(Debug, Clone)]
pub enum MachineError {
    InvalidState(String),
    InvalidTransition,
    GuardFailed(String),
    MissingGuard(String),
    MissingAction(String),
    ContextError(String),
}

pub type MachineResult<T> = Result<T, MachineError>;

#[derive(Clone, Debug, Default)]
pub struct Context {
    data: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}
```

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_creation_works() {
        let machine = Machine::new("traffic_light".to_string());
        assert_eq!(machine.id, "traffic_light");
    }

    #[test]
    fn state_transitions_work() {
        let mut machine = Machine::new("counter".to_string());
        let idle_state = StateNode::new("idle".to_string(), StateType::Atomic);
        machine.add_state(idle_state);

        assert!(machine.can_transition_to("idle"));
    }

    #[test]
    fn guards_prevent_invalid_transitions() {
        let machine = Machine::new("guarded".to_string());
        let guard = ConditionGuard::new("test_guard".to_string(), |_| false);

        // Should fail when guard returns false
        assert!(machine.transition("invalid_event").is_err());
    }
}
```

## Performance Considerations

- **State Lookup:** HashMap for O(1) state access
- **Transition Caching:** Cache validated transitions
- **Action Batching:** Execute multiple actions efficiently
- **Memory Management:** Clean up old history entries

## Future Extensions

- [ ] Parallel states
- [ ] History states (shallow/deep)
- [ ] State machine composition
- [ ] Visual debugging
- [ ] Performance profiling
