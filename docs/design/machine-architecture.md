# ⚙️ State Machine Architecture Design

## Overview
Design document for the state machine system - providing XState-like finite state machine capabilities with Leptos integration.

## Core State Machine Concepts

### 1. Finite State Machine
A system with a finite number of states and well-defined transitions between states.

### 2. Hierarchical States
States can contain nested sub-states for complex state modeling.

### 3. Guards and Actions
Conditional logic (guards) and side effects (actions) that control and respond to transitions.

## Machine Core Architecture

### State Machine Interface

```rust
pub trait StateMachine {
    type State: Clone + PartialEq + 'static;
    type Event: Clone + 'static;
    type Context: Clone + 'static;
    
    /// Get current state
    fn current_state(&self) -> &Self::State;
    
    /// Send event to machine
    fn send(&mut self, event: Self::Event) -> Result<(), TransitionError>;
    
    /// Check if transition is valid
    fn can_transition(&self, event: &Self::Event) -> bool;
    
    /// Get available transitions from current state
    fn available_transitions(&self) -> Vec<Self::Event>;
}
```

### Core Machine Implementation

```rust
pub struct Machine<S, E, C = ()> {
    /// Current state
    current_state: S,
    
    /// Machine context/extended state
    context: C,
    
    /// State definitions
    states: HashMap<S, StateNode<S, E, C>>,
    
    /// Initial state
    initial_state: S,
    
    /// Transition history for debugging
    history: Vec<Transition<S, E>>,
    
    /// Guards registry
    guards: HashMap<String, Arc<dyn Fn(&C, &E) -> bool + Send + Sync>>,
    
    /// Actions registry  
    actions: HashMap<String, Arc<dyn Fn(&mut C, &E) + Send + Sync>>,
}

impl<S, E, C> Machine<S, E, C> 
where 
    S: Clone + PartialEq + Hash + 'static,
    E: Clone + 'static,
    C: Clone + 'static,
{
    pub fn new(initial_state: S, context: C) -> Self {
        Self {
            current_state: initial_state.clone(),
            context,
            states: HashMap::new(),
            initial_state,
            history: Vec::new(),
            guards: HashMap::new(),
            actions: HashMap::new(),
        }
    }
}
```

### State Node Definition

```rust
pub struct StateNode<S, E, C> {
    /// State identifier
    id: S,
    
    /// Entry actions
    on_entry: Vec<String>,
    
    /// Exit actions
    on_exit: Vec<String>,
    
    /// Transitions from this state
    transitions: HashMap<E, TransitionConfig<S, C>>,
    
    /// Child states (for hierarchical machines)
    children: Option<HashMap<S, StateNode<S, E, C>>>,
    
    /// Initial child state
    initial_child: Option<S>,
    
    /// State type
    state_type: StateType,
}

#[derive(Debug, Clone)]
pub enum StateType {
    Simple,
    Compound,    // Has child states
    Parallel,    // Multiple active child states
    Final,       // Terminal state
    History,     // Remembers previous state
}
```

### Transition Configuration

```rust
pub struct TransitionConfig<S, C> {
    /// Target state
    target: S,
    
    /// Guard conditions
    guards: Vec<String>,
    
    /// Actions to execute
    actions: Vec<String>,
    
    /// Transition type
    transition_type: TransitionType,
}

#[derive(Debug, Clone)]
pub enum TransitionType {
    External,    // Exit and re-enter states
    Internal,    // Stay within current state
    Local,       // Minimal exit/entry
}
```

## Builder Pattern API

### Machine Builder

```rust
pub struct MachineBuilder<S, E, C = ()> {
    states: HashMap<S, StateBuilder<S, E, C>>,
    initial_state: Option<S>,
    context: Option<C>,
    guards: HashMap<String, Arc<dyn Fn(&C, &E) -> bool + Send + Sync>>,
    actions: HashMap<String, Arc<dyn Fn(&mut C, &E) + Send + Sync>>,
}

impl<S, E, C> MachineBuilder<S, E, C> 
where 
    S: Clone + PartialEq + Hash + 'static,
    E: Clone + 'static,
    C: Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            initial_state: None,
            context: None,
            guards: HashMap::new(),
            actions: HashMap::new(),
        }
    }
    
    pub fn state(mut self, id: S) -> StateBuilder<S, E, C> {
        StateBuilder::new(id, self)
    }
    
    pub fn initial(mut self, state: S) -> Self {
        self.initial_state = Some(state);
        self
    }
    
    pub fn context(mut self, context: C) -> Self {
        self.context = Some(context);
        self
    }
    
    pub fn guard<F>(mut self, name: impl Into<String>, guard: F) -> Self 
    where F: Fn(&C, &E) -> bool + Send + Sync + 'static
    {
        self.guards.insert(name.into(), Arc::new(guard));
        self
    }
    
    pub fn action<F>(mut self, name: impl Into<String>, action: F) -> Self 
    where F: Fn(&mut C, &E) + Send + Sync + 'static
    {
        self.actions.insert(name.into(), Arc::new(action));
        self
    }
    
    pub fn build(self) -> Result<Machine<S, E, C>, BuildError> {
        let initial_state = self.initial_state.ok_or(BuildError::NoInitialState)?;
        let context = self.context.unwrap_or_default();
        
        let mut machine = Machine::new(initial_state, context);
        machine.guards = self.guards;
        machine.actions = self.actions;
        
        // Build states
        for (id, state_builder) in self.states {
            machine.states.insert(id, state_builder.build()?);
        }
        
        Ok(machine)
    }
}
```

### State Builder

```rust
pub struct StateBuilder<S, E, C> {
    id: S,
    on_entry: Vec<String>,
    on_exit: Vec<String>,
    transitions: HashMap<E, TransitionBuilder<S, C>>,
    children: HashMap<S, StateBuilder<S, E, C>>,
    initial_child: Option<S>,
    state_type: StateType,
    parent_builder: MachineBuilder<S, E, C>,
}

impl<S, E, C> StateBuilder<S, E, C> 
where 
    S: Clone + PartialEq + Hash + 'static,
    E: Clone + 'static,
    C: Clone + 'static,
{
    pub fn on(mut self, event: E, target: S) -> TransitionBuilder<S, C> {
        TransitionBuilder::new(event, target, self)
    }
    
    pub fn on_entry(mut self, action: impl Into<String>) -> Self {
        self.on_entry.push(action.into());
        self
    }
    
    pub fn on_exit(mut self, action: impl Into<String>) -> Self {
        self.on_exit.push(action.into());
        self
    }
    
    pub fn child_state(mut self, id: S) -> StateBuilder<S, E, C> {
        self.state_type = StateType::Compound;
        StateBuilder::new_child(id, self)
    }
    
    pub fn initial_child(mut self, state: S) -> Self {
        self.initial_child = Some(state);
        self
    }
    
    pub fn end_state(mut self) -> MachineBuilder<S, E, C> {
        self.parent_builder.states.insert(self.id.clone(), self);
        self.parent_builder
    }
}
```

## Leptos Integration

### Machine Hook

```rust
pub fn use_machine<S, E, C>(
    machine: Machine<S, E, C>
) -> (ReadSignal<MachineState<S, C>>, impl Fn(E))
where 
    S: Clone + PartialEq + 'static,
    E: Clone + 'static,
    C: Clone + 'static,
{
    let (state, set_state) = create_signal(MachineState {
        current: machine.current_state().clone(),
        context: machine.context().clone(),
        can_transition: machine.available_transitions(),
    });
    
    let machine_ref = create_rw_signal(machine);
    
    let send = move |event: E| {
        machine_ref.update(|machine| {
            if let Ok(()) = machine.send(event) {
                set_state.set(MachineState {
                    current: machine.current_state().clone(),
                    context: machine.context().clone(),
                    can_transition: machine.available_transitions(),
                });
            }
        });
    };
    
    (state, send)
}

#[derive(Clone, Debug)]
pub struct MachineState<S, C> {
    pub current: S,
    pub context: C,
    pub can_transition: Vec<String>,
}
```

### Machine Context Provider

```rust
#[component]
pub fn MachineProvider<S, E, C>(
    machine: Machine<S, E, C>,
    children: Children,
) -> impl IntoView 
where 
    S: Clone + PartialEq + 'static,
    E: Clone + 'static,
    C: Clone + 'static,
{
    let (state, send) = use_machine(machine);
    
    provide_context((state, send));
    
    children()
}

pub fn use_machine_context<S, E, C>() -> (ReadSignal<MachineState<S, C>>, impl Fn(E))
where 
    S: Clone + PartialEq + 'static,
    E: Clone + 'static,
    C: Clone + 'static,
{
    expect_context()
}
```

## Advanced Features

### 1. Hierarchical State Machines

```rust
// Example: Media player with nested states
let media_machine = MachineBuilder::new()
    .state("loading")
        .on(MediaEvent::Loaded, "ready")
    .end_state()
    
    .state("ready")
        .child_state("stopped")
            .on(MediaEvent::Play, "playing")
        .end_state()
        
        .child_state("playing")
            .on(MediaEvent::Pause, "paused")
            .on(MediaEvent::Stop, "stopped")
        .end_state()
        
        .child_state("paused")
            .on(MediaEvent::Play, "playing")
            .on(MediaEvent::Stop, "stopped")
        .end_state()
        
        .initial_child("stopped")
    .end_state()
    
    .initial("loading")
    .build()?;
```

### 2. Guards and Actions

```rust
#[derive(Clone)]
struct AuthContext {
    user: Option<User>,
    login_attempts: u32,
}

let auth_machine = MachineBuilder::new()
    .context(AuthContext { user: None, login_attempts: 0 })
    
    .guard("has_user", |ctx: &AuthContext, _| ctx.user.is_some())
    .guard("under_attempt_limit", |ctx: &AuthContext, _| ctx.login_attempts < 3)
    
    .action("increment_attempts", |ctx: &mut AuthContext, _| {
        ctx.login_attempts += 1;
    })
    .action("reset_attempts", |ctx: &mut AuthContext, _| {
        ctx.login_attempts = 0;
    })
    .action("set_user", |ctx: &mut AuthContext, event| {
        if let AuthEvent::LoginSuccess(user) = event {
            ctx.user = Some(user.clone());
        }
    })
    
    .state("logged_out")
        .on(AuthEvent::Login, "logging_in")
            .guard("under_attempt_limit")
    .end_state()
    
    .state("logging_in")
        .on(AuthEvent::LoginSuccess, "logged_in")
            .action("set_user")
            .action("reset_attempts")
        .on(AuthEvent::LoginFailure, "logged_out")
            .action("increment_attempts")
    .end_state()
    
    .state("logged_in")
        .on(AuthEvent::Logout, "logged_out")
    .end_state()
    
    .initial("logged_out")
    .build()?;
```

### 3. Parallel States

```rust
let app_machine = MachineBuilder::new()
    .state("app")
        .state_type(StateType::Parallel)
        
        .child_state("navigation")
            .child_state("home")
            .child_state("settings")
            .child_state("profile")
            .initial_child("home")
        .end_state()
        
        .child_state("notifications")
            .child_state("enabled")
            .child_state("disabled")
            .initial_child("enabled")
        .end_state()
        
        .child_state("theme")
            .child_state("light")
            .child_state("dark")
            .initial_child("light")
        .end_state()
    .end_state()
    
    .initial("app")
    .build()?;
```

## Transition Semantics

### Transition Types

```rust
pub enum TransitionType {
    /// Exit source state, enter target state
    External,
    
    /// Stay within current state, don't exit/enter
    Internal,
    
    /// Minimal exit/entry (for hierarchical states)
    Local,
}
```

### Transition Processing

```rust
impl<S, E, C> Machine<S, E, C> {
    fn process_transition(&mut self, event: E) -> Result<(), TransitionError> {
        // 1. Find valid transition
        let transition = self.find_transition(&event)?;
        
        // 2. Check guards
        self.check_guards(&transition, &event)?;
        
        // 3. Execute exit actions
        self.execute_exit_actions(&transition);
        
        // 4. Execute transition actions
        self.execute_transition_actions(&transition, &event);
        
        // 5. Change state
        self.current_state = transition.target.clone();
        
        // 6. Execute entry actions
        self.execute_entry_actions(&transition);
        
        // 7. Record history
        self.history.push(Transition {
            from: self.current_state.clone(),
            to: transition.target.clone(),
            event,
            timestamp: std::time::Instant::now(),
        });
        
        Ok(())
    }
}
```

## Error Handling

### Machine Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum MachineError {
    #[error("Invalid transition from {from:?} on event {event:?}")]
    InvalidTransition { from: String, event: String },
    
    #[error("Guard '{guard}' failed")]
    GuardFailed { guard: String },
    
    #[error("Action '{action}' failed: {error}")]
    ActionFailed { action: String, error: String },
    
    #[error("State '{state}' not found")]
    StateNotFound { state: String },
    
    #[error("Build error: {0}")]
    Build(#[from] BuildError),
}

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("No initial state specified")]
    NoInitialState,
    
    #[error("Invalid state hierarchy")]
    InvalidHierarchy,
    
    #[error("Circular transition detected")]
    CircularTransition,
}
```

## Performance Considerations

### 1. Transition Caching
```rust
struct TransitionCache<S, E> {
    cache: HashMap<(S, E), Option<S>>,
}

impl<S, E> TransitionCache<S, E> 
where S: Clone + Hash + Eq, E: Clone + Hash + Eq
{
    fn get_or_compute<F>(&mut self, from: S, event: E, compute: F) -> Option<S> 
    where F: FnOnce() -> Option<S>
    {
        self.cache.entry((from, event)).or_insert_with(compute).clone()
    }
}
```

### 2. Lazy State Evaluation
```rust
// Compute derived state lazily
impl<S, E, C> Machine<S, E, C> {
    fn derived_state(&self) -> impl Fn() -> DerivedState {
        let current = self.current_state.clone();
        let context = self.context.clone();
        
        move || DerivedState::compute(&current, &context)
    }
}
```

## Testing Framework

### Machine Testing Utilities

```rust
pub mod testing {
    pub struct MachineTester<S, E, C> {
        machine: Machine<S, E, C>,
        expected_transitions: Vec<(S, E, S)>,
    }
    
    impl<S, E, C> MachineTester<S, E, C> 
    where S: Clone + PartialEq + Debug,
          E: Clone + Debug,
          C: Clone + Debug,
    {
        pub fn new(machine: Machine<S, E, C>) -> Self {
            Self { machine, expected_transitions: Vec::new() }
        }
        
        pub fn expect_transition(mut self, from: S, event: E, to: S) -> Self {
            self.expected_transitions.push((from, event, to));
            self
        }
        
        pub fn run_test(mut self) -> Result<(), TestError> {
            for (from, event, expected_to) in self.expected_transitions {
                // Set machine to from state
                self.machine.force_state(from.clone());
                
                // Send event
                self.machine.send(event.clone())?;
                
                // Check result
                let actual_to = self.machine.current_state();
                if actual_to != &expected_to {
                    return Err(TestError::UnexpectedTransition {
                        from,
                        event,
                        expected: expected_to,
                        actual: actual_to.clone(),
                    });
                }
            }
            Ok(())
        }
    }
}
```

This state machine architecture provides a robust foundation for complex state management while maintaining clean integration with Leptos reactive system.
