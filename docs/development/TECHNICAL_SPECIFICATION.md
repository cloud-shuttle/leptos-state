# 🔧 Technical Specification v1.0.0
## **Architectural Redesign Implementation Details**

> **Status**: 🚧 In Planning  
> **Version**: 1.0.0  
> **Target**: September 2025 - Modern Rust Ecosystem

---

## 📋 **Overview**

This document provides the detailed technical specification for implementing the architectural redesign of `leptos-state`. It covers the complete implementation details, data structures, trait definitions, and integration patterns.

---

## 🏗️ **Core Architecture**

### **1.1 Trait Hierarchy**

```rust
// Base traits with proper bounds
pub trait StateMachineContext: 
    Clone + Debug + Default + Send + Sync + 'static {}

pub trait StateMachineEvent: 
    Clone + Debug + PartialEq + Send + Sync + 'static {}

pub trait StateMachineState: 
    Clone + Debug + Send + Sync + 'static {
    type Context: StateMachineContext;
    type Event: StateMachineEvent;
}

// Core machine trait
pub trait StateMachine: StateMachineState {
    fn initial_state(&self) -> Self::State;
    fn transition(&self, state: &Self::State, event: Self::Event) -> Self::State;
    fn can_transition(&self, state: &Self::State, event: Self::Event) -> bool;
}

// State machine implementation
pub struct Machine<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    states: HashMap<String, StateNode<C, E, S>>,
    initial: String,
    _phantom: PhantomData<(C, E, S)>,
}
```

### **1.2 State Node Structure**

```rust
#[derive(Clone, Debug)]
pub struct StateNode<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    id: String,
    transitions: Vec<Transition<C, E, S>>,
    entry_actions: Vec<Box<dyn Action<C, E, S>>>,
    exit_actions: Vec<Box<dyn Action<C, E, S>>,
    child_states: Vec<StateNode<C, E, S>>,
    initial_child: Option<String>,
    metadata: StateMetadata,
}

#[derive(Clone, Debug)]
pub struct StateMetadata {
    description: Option<String>,
    tags: Vec<String>,
    custom_data: HashMap<String, serde_json::Value>,
}
```

### **1.3 Transition System**

```rust
#[derive(Clone, Debug)]
pub struct Transition<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    event: E,
    target: String,
    guards: Vec<Box<dyn Guard<C, E, S>>>,
    actions: Vec<Box<dyn Action<C, E, S>>>,
    metadata: TransitionMetadata,
}

#[derive(Clone, Debug)]
pub struct TransitionMetadata {
    description: Option<String>,
    priority: u32,
    is_internal: bool,
    custom_data: HashMap<String, serde_json::Value>,
}
```

---

## 🔧 **Action and Guard System**

### **2.1 Action Traits**

```rust
pub trait Action<C, E, S>: Send + Sync
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    fn execute(&self, context: &mut C, event: &E, state: &S) -> Result<(), ActionError>;
    fn name(&self) -> &str;
    fn is_async(&self) -> bool;
}

// Built-in action types
pub struct FunctionAction<F> {
    name: String,
    func: F,
    is_async: bool,
}

impl<C, E, S, F> Action<C, E, S> for FunctionAction<F>
where
    F: Fn(&mut C, &E, &S) -> Result<(), ActionError> + Send + Sync,
{
    fn execute(&self, context: &mut C, event: &E, state: &S) -> Result<(), ActionError> {
        (self.func)(context, event, state)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_async(&self) -> bool {
        self.is_async
    }
}
```

### **2.2 Guard Traits**

```rust
pub trait Guard<C, E, S>: Send + Sync
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    fn evaluate(&self, context: &C, event: &E, state: &S) -> Result<bool, GuardError>;
    fn name(&self) -> &str;
}

// Built-in guard types
pub struct FunctionGuard<F> {
    name: String,
    func: F,
}

impl<C, E, S, F> Guard<C, E, S> for FunctionGuard<F>
where
    F: Fn(&C, &E, &S) -> Result<bool, GuardError> + Send + Sync,
{
    fn evaluate(&self, context: &C, event: &E, state: &S) -> Result<bool, GuardError> {
        (self.func)(context, event, state)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// Composite guards
pub struct AndGuard<G1, G2> {
    guard1: G1,
    guard2: G2,
}

pub struct OrGuard<G1, G2> {
    guard1: G1,
    guard2: G2,
}

pub struct NotGuard<G> {
    guard: G,
}
```

---

## 🏗️ **Builder Pattern**

### **3.1 Machine Builder**

```rust
pub struct MachineBuilder<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    states: HashMap<String, StateNode<C, E, S>>,
    initial: String,
    metadata: MachineMetadata,
    _phantom: PhantomData<(C, E, S)>,
}

impl<C, E, S> MachineBuilder<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            initial: String::new(),
            metadata: MachineMetadata::default(),
            _phantom: PhantomData,
        }
    }
    
    pub fn state(mut self, id: &str) -> StateBuilder<C, E, S> {
        StateBuilder::new(self, id.to_string())
    }
    
    pub fn initial(mut self, state_id: &str) -> Self {
        self.initial = state_id.to_string();
        self
    }
    
    pub fn metadata(mut self, metadata: MachineMetadata) -> Self {
        self.metadata = metadata;
        self
    }
    
    pub fn build(self) -> Result<Machine<C, E, S>, BuildError> {
        if self.initial.is_empty() {
            return Err(BuildError::NoInitialState);
        }
        
        if !self.states.contains_key(&self.initial) {
            return Err(BuildError::InvalidInitialState);
        }
        
        // Validate state machine
        self.validate()?;
        
        Ok(Machine {
            states: self.states,
            initial: self.initial,
            _phantom: PhantomData,
        })
    }
    
    fn validate(&self) -> Result<(), BuildError> {
        // Validate all states are reachable
        // Validate no dead ends
        // Validate proper hierarchy
        Ok(())
    }
}
```

### **3.2 State Builder**

```rust
pub struct StateBuilder<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    machine_builder: MachineBuilder<C, E, S>,
    state_id: String,
    transitions: Vec<Transition<C, E, S>>,
    entry_actions: Vec<Box<dyn Action<C, E, S>>>,
    exit_actions: Vec<Box<dyn Action<C, E, S>>>,
    child_states: Vec<StateNode<C, E, S>>,
    initial_child: Option<String>,
    metadata: StateMetadata,
}

impl<C, E, S> StateBuilder<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    pub fn on(mut self, event: E, target: &str) -> Self {
        let transition = Transition {
            event,
            target: target.to_string(),
            guards: Vec::new(),
            actions: Vec::new(),
            metadata: TransitionMetadata::default(),
        };
        self.transitions.push(transition);
        self
    }
    
    pub fn guard(mut self, guard: impl Guard<C, E, S> + 'static) -> Self {
        if let Some(last_transition) = self.transitions.last_mut() {
            last_transition.guards.push(Box::new(guard));
        }
        self
    }
    
    pub fn action(mut self, action: impl Action<C, E, S> + 'static) -> Self {
        if let Some(last_transition) = self.transitions.last_mut() {
            last_transition.actions.push(Box::new(action));
        }
        self
    }
    
    pub fn entry_action(mut self, action: impl Action<C, E, S> + 'static) -> Self {
        self.entry_actions.push(Box::new(action));
        self
    }
    
    pub fn exit_action(mut self, action: impl Action<C, E, S> + 'static) -> Self {
        self.exit_actions.push(Box::new(action));
        self
    }
    
    pub fn child(mut self, child_state: StateNode<C, E, S>) -> Self {
        self.child_states.push(child_state);
        self
    }
    
    pub fn initial_child(mut self, child_id: &str) -> Self {
        self.initial_child = Some(child_id.to_string());
        self
    }
    
    pub fn metadata(mut self, metadata: StateMetadata) -> Self {
        self.metadata = metadata;
        self
    }
    
    pub fn build(mut self) -> MachineBuilder<C, E, S> {
        let state_node = StateNode {
            id: self.state_id.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: self.child_states,
            initial_child: self.initial_child,
            metadata: self.metadata,
        };
        
        self.machine_builder.states.insert(self.state_id, state_node);
        self.machine_builder
    }
}
```

---

## 🏪 **Store System**

### **4.1 Store Traits**

```rust
pub trait Store: Clone + 'static {
    type State: Clone + PartialEq + Send + Sync + 'static;
    type Actions: StoreActions<Self::State>;
    
    fn create() -> Self::State;
    fn actions() -> Self::Actions;
}

pub trait StoreActions<S>: Clone + Send + Sync + 'static {
    fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + Sync;
        
    fn set(&self, new_state: S) -> Result<(), StoreError>;
    fn get(&self) -> S;
    fn subscribe<F>(&self, subscriber: F) -> SubscriptionHandle
    where
        F: Fn(&S) + Send + Sync + 'static;
}

// Reactive store with Leptos integration
pub struct ReactiveStore<S: Store> {
    state: RwSignal<S::State>,
    actions: S::Actions,
}

impl<S: Store> ReactiveStore<S> {
    pub fn new(initial: S::State) -> Self {
        let state = RwSignal::new(initial);
        let actions = S::actions();
        Self { state, actions }
    }
    
    pub fn state(&self) -> ReadSignal<S::State> {
        self.state.read_only()
    }
    
    pub fn actions(&self) -> &S::Actions {
        &self.actions
    }
}
```

### **4.2 Store Actions Implementation**

```rust
pub struct StoreActionsImpl<S> {
    state: RwSignal<S>,
}

impl<S: Clone + PartialEq + Send + Sync + 'static> StoreActions<S> for StoreActionsImpl<S> {
    fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + Sync,
    {
        self.state.update(updater);
        Ok(())
    }
    
    fn set(&self, new_state: S) -> Result<(), StoreError> {
        self.state.set(new_state);
        Ok(())
    }
    
    fn get(&self) -> S {
        self.state.get()
    }
    
    fn subscribe<F>(&self, subscriber: F) -> SubscriptionHandle
    where
        F: Fn(&S) + Send + Sync + 'static,
    {
        let effect = create_effect(move |_| {
            let current_state = self.state.get();
            subscriber(&current_state);
        });
        
        SubscriptionHandle::new(effect)
    }
}
```

---

## 🌐 **Leptos Integration**

### **5.1 Hooks System**

```rust
// Machine hook
pub fn use_machine<C, E, S>(
    machine: Machine<C, E, S>,
    initial_context: C,
) -> (ReadSignal<MachineState<C, E, S>>, MachineActions<C, E, S>)
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    let (state, set_state) = create_signal(MachineState::new(machine, initial_context));
    let actions = MachineActions::new(state, set_state);
    (state.read_only(), actions)
}

// Store hook
pub fn use_store<S: Store>() -> (ReadSignal<S::State>, StoreActions<S::State>) {
    let store = use_context::<ReactiveStore<S>>()
        .expect("Store not provided - use provide_store");
    (store.state(), store.actions().clone())
}

// Computed store values
pub fn use_store_slice<S: Store, T>(
    selector: impl Fn(&S::State) -> T + Send + Sync + 'static,
) -> Memo<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let (state, _) = use_store::<S>();
    create_memo(move |_| selector(&state.get()))
}
```

### **5.2 Machine State Management**

```rust
pub struct MachineState<C, E, S> {
    machine: Machine<C, E, S>,
    current_state: S,
    context: C,
    history: Vec<StateTransition<C, E, S>>,
}

impl<C, E, S> MachineState<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    pub fn new(machine: Machine<C, E, S>, initial_context: C) -> Self {
        let current_state = machine.initial_state();
        Self {
            machine,
            current_state,
            context: initial_context,
            history: Vec::new(),
        }
    }
    
    pub fn transition(&mut self, event: E) -> Result<(), TransitionError> {
        let old_state = self.current_state.clone();
        let new_state = self.machine.transition(&self.current_state, event.clone());
        
        // Execute exit actions
        self.execute_exit_actions(&old_state)?;
        
        // Update state
        self.current_state = new_state;
        
        // Execute entry actions
        self.execute_entry_actions(&self.current_state)?;
        
        // Record transition
        let transition = StateTransition {
            from: old_state,
            to: self.current_state.clone(),
            event,
            timestamp: std::time::Instant::now(),
        };
        self.history.push(transition);
        
        Ok(())
    }
    
    fn execute_exit_actions(&self, state: &S) -> Result<(), ActionError> {
        // Implementation
        Ok(())
    }
    
    fn execute_entry_actions(&self, state: &S) -> Result<(), ActionError> {
        // Implementation
        Ok(())
    }
}
```

---

## 🚀 **Feature System**

### **6.1 Persistence**

```rust
#[cfg(feature = "persist")]
pub trait StorageBackend: Send + Sync {
    type Error: std::error::Error + Send + Sync;
    
    async fn save<K, V>(&self, key: K, value: &V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize;
        
    async fn load<K, V>(&self, key: K) -> Result<Option<V>, Self::Error>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>;
        
    async fn delete<K>(&self, key: K) -> Result<(), Self::Error>
    where
        K: AsRef<str>;
        
    async fn clear(&self) -> Result<(), Self::Error>;
}

#[cfg(feature = "persist")]
pub struct LocalStorageBackend;

#[cfg(feature = "persist")]
impl StorageBackend for LocalStorageBackend {
    type Error = LocalStorageError;
    
    async fn save<K, V>(&self, key: K, value: &V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        // Implementation using web-sys
        Ok(())
    }
    
    async fn load<K, V>(&self, key: K) -> Result<Option<V>, Self::Error>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>,
    {
        // Implementation using web-sys
        Ok(None)
    }
    
    async fn delete<K>(&self, key: K) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
    {
        // Implementation
        Ok(())
    }
    
    async fn clear(&self) -> Result<(), Self::Error> {
        // Implementation
        Ok(())
    }
}
```

### **6.2 Visualization**

```rust
#[cfg(feature = "visualization")]
pub trait StateMachineVisualizer<C, E, S> {
    fn generate_dot(&self) -> String;
    fn generate_mermaid(&self) -> String;
    fn generate_plantuml(&self) -> String;
    fn export_svg(&self) -> Result<String, VisualizationError>;
    fn export_png(&self) -> Result<Vec<u8>, VisualizationError>;
}

#[cfg(feature = "visualization")]
impl<C, E, S> StateMachineVisualizer<C, E, S> for Machine<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    fn generate_mermaid(&self) -> String {
        let mut mermaid = String::from("graph TD\n");
        
        for (state_id, state) in &self.states {
            mermaid.push_str(&format!("    {}[{}]\n", state_id, state_id));
            
            for transition in &state.transitions {
                mermaid.push_str(&format!(
                    "    {} -->|{:?}| {}\n",
                    state_id, transition.event, transition.target
                ));
            }
        }
        
        mermaid
    }
    
    fn generate_dot(&self) -> String {
        // DOT format implementation
        String::new()
    }
    
    fn generate_plantuml(&self) -> String {
        // PlantUML format implementation
        String::new()
    }
    
    fn export_svg(&self) -> Result<String, VisualizationError> {
        // SVG export implementation
        Ok(String::new())
    }
    
    fn export_png(&self) -> Result<Vec<u8>, VisualizationError> {
        // PNG export implementation
        Ok(Vec::new())
    }
}
```

---

## 🧪 **Testing Framework**

```rust
#[cfg(feature = "testing")]
pub trait StateMachineTester<C, E, S> {
    fn property_test<F>(&self, property: F) -> TestResult
    where
        F: Fn(&Machine<C, E, S>, &[E]) -> bool;
        
    fn generate_test_cases(&self, count: usize) -> Vec<TestCase<C, E, S>>;
    fn run_test_suite(&self, suite: TestSuite<C, E, S>) -> TestReport;
}

#[cfg(feature = "testing")]
pub struct ProptestRunner<C, E, S> {
    machine: Machine<C, E, S>,
    config: ProptestConfig,
}

#[cfg(feature = "testing")]
impl<C, E, S> StateMachineTester<C, E, S> for ProptestRunner<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    fn property_test<F>(&self, property: F) -> TestResult
    where
        F: Fn(&Machine<C, E, S>, &[E]) -> bool,
    {
        proptest!(|(events: Vec<E>)| {
            prop_assert!(property(&self.machine, &events));
        });
        
        TestResult::Passed
    }
    
    fn generate_test_cases(&self, count: usize) -> Vec<TestCase<C, E, S>> {
        // Generate test cases using proptest
        Vec::new()
    }
    
    fn run_test_suite(&self, suite: TestSuite<C, E, S>) -> TestReport {
        // Run test suite
        TestReport::default()
    }
}
```

---

## 🔄 **Migration System**

```rust
pub mod migration {
    use super::*;
    
    // Migration from v0.2.x to v1.0.0
    pub fn migrate_v0_2_machine<C, E>(
        old_machine: v0_2::Machine<C, E>,
    ) -> Result<Machine<C, E>, MigrationError>
    where
        C: StateMachineContext,
        E: StateMachineEvent,
    {
        let mut builder = MachineBuilder::new();
        
        // Migrate states
        for (state_id, old_state) in old_machine.states_map() {
            let mut state_builder = builder.state(state_id);
            
            // Migrate transitions
            for transition in old_state.transitions() {
                state_builder = state_builder.on(transition.event().clone(), transition.target());
            }
            
            builder = state_builder.build();
        }
        
        // Set initial state
        builder = builder.initial(old_machine.initial_state_id());
        
        // Build and return
        builder.build().map_err(MigrationError::BuildError)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Build error: {0}")]
    BuildError(#[from] BuildError),
    
    #[error("Incompatible types: {0}")]
    IncompatibleTypes(String),
    
    #[error("Missing required trait implementation: {0}")]
    MissingTrait(String),
}
```

---

## 📊 **Performance Considerations**

### **8.1 Zero-Cost Abstractions**

- **Trait Objects**: Minimize use of trait objects where possible
- **Generic Constraints**: Use const generics for compile-time optimizations
- **Memory Layout**: Optimize struct layouts for cache performance
- **Allocation Strategy**: Use arena allocation for short-lived objects

### **8.2 WASM Optimizations**

- **Size Optimization**: Minimize WASM binary size
- **Runtime Performance**: Optimize for web runtime characteristics
- **Memory Management**: Efficient memory usage in browser environment
- **Serialization**: Fast serialization for persistence

### **8.3 Benchmarking**

```rust
#[cfg(feature = "performance")]
pub mod benchmarks {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    use super::*;
    
    pub fn benchmark_transitions(c: &mut Criterion) {
        let machine = create_test_machine();
        let mut state = machine.initial_state();
        
        c.bench_function("state_transition", |b| {
            b.iter(|| {
                let event = TestEvent::Next;
                black_box(machine.transition(&state, event));
            });
        });
    }
    
    criterion_group!(benches, benchmark_transitions);
    criterion_main!(benches);
}
```

---

## 🔒 **Error Handling**

```rust
#[derive(Debug, thiserror::Error)]
pub enum StateMachineError {
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Invalid transition: {0}")]
    InvalidTransition(String),
    
    #[error("Action execution failed: {0}")]
    ActionError(#[from] ActionError),
    
    #[error("Guard evaluation failed: {0}")]
    GuardError(#[from] GuardError),
    
    #[error("Build error: {0}")]
    BuildError(#[from] BuildError),
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("State update failed: {0}")]
    UpdateFailed(String),
    
    #[error("Serialization failed: {0}")]
    SerializationFailed(#[from] serde_json::Error),
    
    #[error("Storage operation failed: {0}")]
    StorageError(String),
}
```

---

## 📚 **Documentation Generation**

```rust
#[cfg(feature = "documentation")]
pub trait StateMachineDocumenter<C, E, S> {
    fn generate_markdown(&self) -> String;
    fn generate_html(&self) -> String;
    fn generate_examples(&self) -> Vec<CodeExample>;
}

#[cfg(feature = "documentation")]
impl<C, E, S> StateMachineDocumenter<C, E, S> for Machine<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    fn generate_markdown(&self) -> String {
        let mut markdown = String::new();
        
        markdown.push_str("# State Machine Documentation\n\n");
        markdown.push_str(&format!("**Initial State**: {}\n\n", self.initial));
        
        markdown.push_str("## States\n\n");
        for (state_id, state) in &self.states {
            markdown.push_str(&format!("### {}\n", state_id));
            if let Some(desc) = &state.metadata.description {
                markdown.push_str(&format!("{}\n\n", desc));
            }
            
            markdown.push_str("**Transitions:**\n");
            for transition in &state.transitions {
                markdown.push_str(&format!("- `{:?}` → {}\n", transition.event, transition.target));
            }
            markdown.push_str("\n");
        }
        
        markdown
    }
    
    fn generate_html(&self) -> String {
        // HTML generation implementation
        String::new()
    }
    
    fn generate_examples(&self) -> Vec<CodeExample> {
        // Example generation implementation
        Vec::new()
    }
}
```

---

*This technical specification is a living document and will be updated as implementation progresses. Last updated: September 4, 2025*
