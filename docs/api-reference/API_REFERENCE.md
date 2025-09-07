# 📚 API Reference - leptos-state v1.0.0

This document provides a comprehensive reference for all public APIs in leptos-state v1.0.0.

## 🏗️ **Core Architecture**

### **Module Structure**
```
leptos_state
├── machine         # State machine implementation
│   ├── builder     # MachineBuilder for creating state machines
│   ├── states      # StateValue and state management
│   ├── events      # Event handling and MachineEvent trait
│   ├── persistence # State persistence system
│   ├── visualization # State diagram generation
│   └── testing     # Testing utilities
├── store           # Reactive store system
│   ├── hooks       # Leptos integration hooks
│   └── persistence # Store persistence
├── v1              # Legacy v1 API (deprecated)
└── utils           # Utility types and helpers
```

## 🎯 **Core Traits**

### **MachineState**
```rust
pub trait MachineState: Clone + PartialEq + Debug + Default + Send + Sync {
    type Context: Clone + PartialEq + Debug + Default + Send + Sync;
    
    fn value(&self) -> &StateValue;
    fn context(&self) -> &Self::Context;
    fn matches(&self, other: &Self) -> bool;
    fn can_transition_to(&self, target: &Self) -> bool;
}
```

**Purpose**: Core trait for state machine states with associated context.

**Example**:
```rust
#[derive(Clone, PartialEq, Debug, Default)]
enum TrafficState {
    #[default]
    Red,
    Yellow,
    Green,
}

#[derive(Clone, PartialEq, Debug, Default)]
struct TrafficContext {
    timer: u32,
    emergency_mode: bool,
}

impl MachineState for TrafficState {
    type Context = TrafficContext;
    
    fn value(&self) -> &StateValue {
        match self {
            TrafficState::Red => &StateValue::Simple("red".to_string()),
            TrafficState::Yellow => &StateValue::Simple("yellow".to_string()),
            TrafficState::Green => &StateValue::Simple("green".to_string()),
        }
    }
    
    fn context(&self) -> &Self::Context {
        &TrafficContext::default()
    }
    
    fn matches(&self, other: &Self) -> bool {
        self == other
    }
    
    fn can_transition_to(&self, _target: &Self) -> bool {
        true
    }
}
```

### **MachineEvent**
```rust
pub trait MachineEvent: Clone + PartialEq + Debug + Default + Send + Sync {
    fn event_type(&self) -> &str;
}
```

**Purpose**: Core trait for state machine events.

**Example**:
```rust
#[derive(Clone, PartialEq, Debug, Default)]
enum TrafficEvent {
    #[default]
    Timer,
    EmergencyStop,
    EmergencyClear,
}

impl MachineEvent for TrafficEvent {
    fn event_type(&self) -> &str {
        match self {
            TrafficEvent::Timer => "timer",
            TrafficEvent::EmergencyStop => "emergency_stop",
            TrafficEvent::EmergencyClear => "emergency_clear",
        }
    }
}
```

### **StateMachine**
```rust
pub trait StateMachine {
    type Context: StateMachineContext;
    type Event: StateMachineEvent;
    type State: StateMachineState<Context = Self::Context, Event = Self::Event>;

    fn initial_state(&self) -> Self::State;
    fn transition(&self, state: &Self::State, event: Self::Event) -> Self::State;
    fn can_transition(&self, state: &Self::State, event: Self::Event) -> bool;
    fn try_transition(&self, state: &Self::State, event: Self::Event) -> Result<Self::State, TransitionError<Self::Event>>;
    fn state_count(&self) -> usize;
    fn is_valid_state(&self, state: &Self::State) -> bool;
    fn is_reachable(&self, state: &Self::State) -> bool;
}
```

**Purpose**: Core trait defining state machine behavior.

**Example**:
```rust
impl StateMachine for TrafficState {
    fn initial_state(&self) -> Self {
        TrafficState::Red
    }
    
    fn transition(&self, state: &Self, event: Self::Event) -> Self {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => TrafficState::Green,
            (TrafficState::Green, TrafficEvent::Timer) => TrafficState::Yellow,
            (TrafficState::Yellow, TrafficEvent::Timer) => TrafficState::Red,
            (_, TrafficEvent::EmergencyStop) => TrafficState::Emergency,
            (TrafficState::Emergency, TrafficEvent::EmergencyClear) => TrafficState::Red,
            _ => state.clone(),
        }
    }
    
    fn can_transition(&self, state: &Self, event: Self::Event) -> bool {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => true,
            (TrafficState::Green, TrafficEvent::Timer) => true,
            (TrafficState::Yellow, TrafficEvent::Timer) => true,
            (_, TrafficEvent::EmergencyStop) => true,
            (TrafficState::Emergency, TrafficEvent::EmergencyClear) => true,
            _ => false,
        }
    }
    
    fn try_transition(&self, state: &Self, event: Self::Event) -> Result<Self, TransitionError<Self::Event>> {
        if self.can_transition(state, event.clone()) {
            Ok(self.transition(state, event))
        } else {
            Err(TransitionError::InvalidTransition(event))
        }
    }
    
    fn state_count(&self) -> usize { 4 }
    fn is_valid_state(&self, _state: &Self) -> bool { true }
    fn is_reachable(&self, _state: &Self) -> bool { true }
}
```

### **StoreState**
```rust
pub trait StoreState: Clone + PartialEq + Debug + Default + Send + Sync {
    // Marker trait for store state types
}
```

**Purpose**: Marker trait for types that can serve as store state.

**Example**:
```rust
#[derive(Clone, PartialEq, Debug, Default)]
struct CounterStore {
    count: i32,
    name: String,
}

impl StoreState for CounterStore {}
```

### **Store**
```rust
pub trait Store: StoreState {
    fn create() -> Self;
    fn create_with_state(state: Self) -> Self;
    fn update<F>(&mut self, f: F) where F: FnOnce(&mut Self);
    fn get(&self) -> &Self;
    fn get_mut(&mut self) -> &mut Self;
}
```

**Purpose**: Core trait defining store behavior.

**Example**:
```rust
impl Store for CounterStore {
    fn create() -> Self {
        Self { count: 0, name: "Counter".to_string() }
    }
    
    fn create_with_state(state: Self) -> Self {
        state
    }
    
    fn update<F>(&mut self, f: F) 
    where 
        F: FnOnce(&mut Self) {
        f(self);
    }
    
    fn get(&self) -> &Self {
        self
    }
    
    fn get_mut(&mut self) -> &mut Self {
        self
    }
}
```

## 🏭 **State Machine Implementation**

### **MachineBuilder**
```rust
pub struct MachineBuilder<C, E> {
    // Private fields
}

impl<C, E> MachineBuilder<C, E>
where
    C: Clone + PartialEq + Debug + Default + Send + Sync,
    E: Clone + PartialEq + Debug + Default + Send + Sync,
{
    pub fn new() -> Self;
    pub fn state(mut self, name: &'static str) -> StateBuilder<C, E>;
    pub fn initial(mut self, state_name: &'static str) -> Self;
    pub fn build(self) -> Machine<C, E>;
}
```

**Purpose**: Builder pattern for creating state machines.

**Example**:
```rust
let machine = MachineBuilder::<TrafficContext, TrafficEvent>::new()
    .state("red")
        .on(TrafficEvent::Timer, "green")
    .state("yellow")
        .on(TrafficEvent::Timer, "red")
    .state("green")
        .on(TrafficEvent::Timer, "yellow")
    .initial("red")
    .build();
```

### **Machine**
```rust
pub struct Machine<C, E> {
    // Private fields
}

impl<C, E> Machine<C, E>
where
    C: Clone + PartialEq + Debug + Default + Send + Sync,
    E: Clone + PartialEq + Debug + Default + Send + Sync,
{
    pub fn initial_state(&self) -> MachineStateImpl<C, E>;
    pub fn transition(&self, state: &MachineStateImpl<C, E>, event: E) -> MachineStateImpl<C, E>;
    pub fn can_transition(&self, state: &MachineStateImpl<C, E>, event: &E) -> bool;
    pub fn is_valid_state(&self, state: &MachineStateImpl<C, E>) -> bool;
    pub fn state_count(&self) -> usize;
    pub fn transition_count(&self) -> usize;
}
```

**Purpose**: Concrete implementation of state machines.

**Example**:
```rust
let machine = MachineBuilder::<TrafficContext, TrafficEvent>::new()
    .state("red")
        .on(TrafficEvent::Timer, "green")
    .state("green")
        .on(TrafficEvent::Timer, "yellow")
    .state("yellow")
        .on(TrafficEvent::Timer, "red")
    .initial("red")
    .build();

let initial_state = machine.initial_state();
let new_state = machine.transition(&initial_state, TrafficEvent::Timer);
```

### **StateNode**
```rust
pub struct StateNode<C, E, S> {
    // Private fields
}

impl<C, E, S> StateNode<C, E, S> {
    pub fn new(name: &'static str) -> Self;
    pub fn with_value(mut self, value: StateValue<S>) -> Self;
    pub fn with_transition(mut self, transition: Transition<E, S>) -> Self;
    pub fn with_guard(mut self, guard: Arc<dyn Guard<C, E>>) -> Self;
    pub fn with_action(mut self, action: Arc<dyn Action<C>>) -> Self;
}
```

**Purpose**: Represents a state in the state machine with transitions, guards, and actions.

### **Transition**
```rust
pub struct Transition<E, S> {
    // Private fields
}

impl<E, S> Transition<E, S> {
    pub fn new(event: E, target: StateValue<S>) -> Self;
    pub fn with_guard(mut self, guard: Arc<dyn Guard<C, E>>) -> Self;
    pub fn with_action(mut self, action: Arc<dyn Action<C>>) -> Self;
}
```

**Purpose**: Represents a transition from one state to another.

### **StateValue**
```rust
pub enum StateValue {
    Simple(String),
    Compound(Vec<String>),
    Parallel(Vec<String>),
    History(String),
    Final,
}
```

**Purpose**: Represents different types of state values.

**Example**:
```rust
use leptos_state::machine::states::StateValue;

// Simple state value
let red_state = StateValue::Simple("red".to_string());

// Compound state value
let active_state = StateValue::Compound(vec!["active".to_string(), "processing".to_string()]);

// Parallel state value
let parallel_state = StateValue::Parallel(vec!["ui".to_string(), "data".to_string()]);

// History state value
let history_state = StateValue::History("previous".to_string());

// Final state value
let final_state = StateValue::Final;
```

## 🏪 **Store System**

### **StoreState**
```rust
pub trait StoreState: Clone + PartialEq + Debug + Default + Send + Sync {
    fn create() -> Self;
    fn update<F>(&mut self, f: F) where F: FnOnce(&mut Self);
}
```

**Purpose**: Core trait for store state types.

**Example**:
```rust
#[derive(Clone, PartialEq, Debug, Default)]
struct CounterStore {
    count: i32,
    name: String,
}

impl StoreState for CounterStore {
    fn create() -> Self {
        Self { 
            count: 0, 
            name: "Counter".to_string() 
        }
    }
    
    fn update<F>(&mut self, f: F) 
    where 
        F: FnOnce(&mut Self) {
        f(self);
    }
}
```

### **Store Hooks**
```rust
// Leptos integration hooks
pub fn use_store<T>() -> (ReadSignal<T>, WriteSignal<T>)
where
    T: StoreState + 'static;

pub fn use_store_with_persistence<T>(key: &'static str) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: StoreState + 'static;
```

**Purpose**: Leptos hooks for using stores in components.

**Example**:
```rust
use leptos::*;
use leptos_state::store::*;

fn Counter() -> impl IntoView {
    let (store, set_store) = use_store::<CounterStore>();
    
    let increment = move |_| {
        set_store.update(|state| state.count += 1);
    };
    
    let decrement = move |_| {
        set_store.update(|state| state.count -= 1);
    };
    
    view! {
        <div>
            <h2>"Counter: " {move || store.get().count}</h2>
            <button on:click=increment>"Increment"</button>
            <button on:click=decrement>"Decrement"</button>
        </div>
    }
}
```

## 🛠️ **Guards and Actions**

### **Guard**
```rust
pub trait Guard<C, E>: Send + Sync {
    fn check(&self, context: &C, event: &E) -> bool;
    fn description(&self) -> &'static str;
}
```

**Purpose**: Defines conditions that must be met for a transition to occur.

**Example**:
```rust
struct CanIncrementGuard;

impl Guard<TrafficContext, TrafficEvent> for CanIncrementGuard {
    fn check(&self, context: &TrafficContext, _event: &TrafficEvent) -> bool {
        !context.emergency_mode
    }
    
    fn description(&self) -> &'static str {
        "Check if increment is allowed (not in emergency mode)"
    }
}
```

### **Action**
```rust
pub trait Action<C>: Send + Sync {
    fn execute(&self, context: &mut C) -> Result<(), ActionError>;
    fn description(&self) -> &'static str;
}
```

**Purpose**: Defines side effects that occur during state transitions.

**Example**:
```rust
struct LogTransitionAction;

impl Action<TrafficContext> for LogTransitionAction {
    fn execute(&self, context: &mut TrafficContext) -> Result<(), ActionError> {
        println!("State transition logged at timer: {}", context.timer);
        context.timer += 1;
        Ok(())
    }
    
    fn description(&self) -> &'static str {
        "Log the state transition and increment timer"
    }
}
```

## 💾 **Persistence System**

### **StorageBackend**
```rust
pub trait StorageBackend: Send + Sync + Debug {
    type Error: std::error::Error + Send + Sync + 'static;
    
    fn save<K, V>(&self, key: K, value: &V) -> Result<(), Self::Error>
    where
        K: AsRef<str>,
        V: Serialize;
        
    fn load<K, V>(&self, key: K) -> Result<Option<V>, Self::Error>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>;
        
    fn remove<K>(&self, key: K) -> Result<(), Self::Error>
    where
        K: AsRef<str>;
        
    fn exists<K>(&self, key: K) -> Result<bool, Self::Error>
    where
        K: AsRef<str>;
        
    fn list_keys(&self) -> Result<Vec<String>, Self::Error>;
}
```

**Purpose**: Defines the interface for storage backends.

### **PersistenceManager**
```rust
pub struct PersistenceManager {
    // Private fields
}

impl PersistenceManager {
    pub fn with_memory_backend() -> Self;
    pub fn with_local_storage_backend() -> Self;
    
    pub fn save_store<S>(&self, key: &str, store: &S) -> Result<(), PersistenceError>
    where
        S: StoreState + Serialize;
        
    pub fn load_store<S>(&self, key: &str) -> Result<Option<S>, PersistenceError>
    where
        S: StoreState + for<'de> Deserialize<'de>;
        
    pub fn save_state_machine<C, E, S>(&self, key: &str, machine: &Machine<C, E, S>) -> Result<(), PersistenceError>
    where
        C: StateMachineContext + Serialize,
        E: StateMachineEvent + Serialize,
        S: StateMachineState<Context = C, Event = E> + Serialize;
        
    pub fn load_state_machine<C, E, S>(&self, key: &str) -> Result<Option<Machine<C, E, S>>, PersistenceError>
    where
        C: StateMachineContext + for<'de> Deserialize<'de>,
        E: StateMachineEvent + for<'de> Deserialize<'de>,
        S: StateMachineState<Context = C, Event = E> + for<'de> Deserialize<'de>;
}
```

**Purpose**: Manages persistence operations for stores and state machines.

## 🧪 **Testing Framework**

### **PropertyTestGenerator**
```rust
pub struct PropertyTestGenerator<C, E, S> {
    // Private fields
}

impl<C, E, S> PropertyTestGenerator<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    pub fn new() -> Self;
    pub fn with_config(mut self, config: PropertyTestConfig) -> Self;
    pub fn generate_test_cases(&self, count: usize) -> Vec<TestCase<C, E, S>>;
}
```

**Purpose**: Generates property-based test cases for state machines.

### **StateMachineTester**
```rust
pub struct StateMachineTester<C, E, S> {
    // Private fields
}

impl<C, E, S> StateMachineTester<C, E, S>
where
    C: StateMachineContext + Default + PartialEq,
    E: StateMachineEvent + Default + PartialEq,
    S: StateMachineState<Context = C, Event = E> + Default + PartialEq,
{
    pub fn new() -> Self;
    pub fn run_test_case(&self, test_case: &TestCase<C, E, S>) -> TestResult;
    pub fn run_test_suite(&self, test_suite: &TestSuite<C, E, S>) -> TestSuiteResult;
}
```

**Purpose**: Executes test cases against state machines.

## 🚀 **Leptos Integration**

### **Hooks**

#### **use_machine_with_context**
```rust
pub fn use_machine_with_context<S, C>(
    initial_state: S,
    initial_context: C,
) -> MachineHandle<S, C>
where
    S: StateMachineState + 'static,
    C: StateMachineContext + 'static,
{
    // Implementation
}
```

**Purpose**: Leptos hook for using state machines in components.

#### **use_store**
```rust
pub fn use_store<T>() -> (ReadSignal<T>, WriteSignal<T>)
where
    T: StoreState + 'static,
{
    // Implementation
}
```

**Purpose**: Leptos hook for using stores in components.

#### **use_store_with_persistence**
```rust
pub fn use_store_with_persistence<T>(key: &'static str) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: StoreState + 'static,
{
    // Implementation
}
```

**Purpose**: Leptos hook for using persistent stores in components.

## 🔧 **Error Types**

### **StateMachineError**
```rust
#[derive(Debug, thiserror::Error)]
pub enum StateMachineError {
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Invalid event: {0}")]
    InvalidEvent(String),
    
    #[error("Transition not allowed: {0} -> {1}")]
    TransitionNotAllowed(String, String),
    
    #[error("State not found: {0}")]
    StateNotFound(String),
    
    #[error("Context error: {0}")]
    ContextError(String),
    
    #[error("Guard failed: {0}")]
    GuardFailed(String),
    
    #[error("Action failed: {0}")]
    ActionFailed(String),
}
```

### **StoreError**
```rust
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("State update failed: {0}")]
    StateUpdateFailed(String),
    
    #[error("Subscription error: {0}")]
    SubscriptionError(String),
    
    #[error("History error: {0}")]
    HistoryError(String),
    
    #[error("Persistence error: {0}")]
    PersistenceError(String),
}
```

### **PersistenceError**
```rust
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Storage backend error: {0}")]
    StorageBackendError(String),
    
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),
    
    #[error("Migration failed: {0}")]
    MigrationFailed(String),
}
```

## 📊 **Performance Monitoring**

### **PerformanceBenchmark**
```rust
pub struct PerformanceBenchmark {
    // Private fields
}

impl PerformanceBenchmark {
    pub fn new() -> Self;
    pub fn with_thresholds(mut self, thresholds: PerformanceThresholds) -> Self;
    pub fn benchmark_operation<F>(&mut self, operation_name: &str, operation: F, iterations: usize) -> BenchmarkResult
    where
        F: Fn() -> usize;
    pub fn benchmark_memory_usage(&mut self, operation: &str, f: impl Fn() -> usize, iterations: usize) -> BenchmarkResult;
    pub fn run_benchmark_suite(&mut self, operations: Vec<(&str, Box<dyn Fn() -> usize>)>, iterations: usize) -> BenchmarkSuite;
    pub fn get_results(&self) -> &HashMap<String, BenchmarkResult>;
    pub fn get_suggestions(&self) -> &[OptimizationSuggestion];
    pub fn meets_thresholds(&self, result: &BenchmarkResult) -> bool;
}
```

**Purpose**: Provides comprehensive performance benchmarking and optimization suggestions.

## 🔄 **Migration Tools**

### **MigrationAnalyzer**
```rust
pub struct MigrationAnalyzer {
    // Private fields
}

impl MigrationAnalyzer {
    pub fn new() -> Self;
    pub fn analyze_code(&self, code: &str) -> Vec<MigrationIssue>;
    pub fn generate_suggestions(&self, issues: &[MigrationIssue]) -> Vec<MigrationSuggestion>;
}
```

**Purpose**: Analyzes code for migration issues and generates suggestions.

### **CodeTransformer**
```rust
pub struct CodeTransformer {
    // Private fields
}

impl CodeTransformer {
    pub fn new() -> Self;
    pub fn with_rules(mut self, rules: Vec<TransformationRule>) -> Self;
    pub fn transform(&self, code: &str) -> String;
}
```

**Purpose**: Transforms code from v0.2.x to v1.0.0.

---

**This API reference covers all public interfaces in leptos-state v1.0.0. For detailed examples and usage patterns, see the [Quickstart Guide](QUICKSTART.md) and [Performance Guide](PERFORMANCE.md).**
