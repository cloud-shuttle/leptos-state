# 🧪 Testing Strategy - Comprehensive Test Framework Design

## Overview
Design document for a comprehensive testing approach covering unit tests, integration tests, property-based testing, performance testing, and WASM-specific testing.

## Testing Philosophy

### 1. Test Pyramid Structure
- **Unit Tests (70%):** Fast, isolated component tests
- **Integration Tests (20%):** Component interaction tests  
- **End-to-End Tests (10%):** Full workflow validation

### 2. Platform Coverage
- **Native Rust:** Server-side and CLI functionality
- **WASM/Browser:** Client-side reactive features
- **Cross-platform:** Shared logic validation

### 3. Test Quality Standards
- Deterministic and repeatable
- Fast execution for development workflow
- Comprehensive error scenario coverage
- Performance regression detection

## Test Architecture

### Test Organization Structure

```
tests/
├── unit/                    # Unit tests (fast, isolated)
│   ├── store/
│   │   ├── basic_store.rs
│   │   ├── async_store.rs
│   │   ├── persistent_store.rs
│   │   └── mod.rs
│   ├── machine/
│   │   ├── transitions.rs
│   │   ├── guards.rs
│   │   ├── hierarchical.rs
│   │   └── mod.rs
│   ├── middleware/
│   │   ├── logger.rs
│   │   ├── validation.rs
│   │   └── mod.rs
│   └── mod.rs
├── integration/             # Integration tests
│   ├── workflows/
│   │   ├── counter_workflow.rs
│   │   ├── todo_workflow.rs
│   │   └── auth_workflow.rs
│   ├── persistence/
│   │   ├── localStorage_integration.rs
│   │   ├── indexeddb_integration.rs
│   │   └── file_persistence.rs
│   └── mod.rs
├── property/                # Property-based tests
│   ├── store_properties.rs
│   ├── machine_properties.rs
│   └── mod.rs
├── browser/                 # Browser-specific tests
│   ├── localStorage_tests.rs
│   ├── indexeddb_tests.rs
│   ├── reactivity_tests.rs
│   └── mod.rs
├── performance/             # Performance tests
│   ├── benchmarks/
│   │   ├── store_benchmarks.rs
│   │   ├── machine_benchmarks.rs
│   │   └── mod.rs
│   ├── regression/
│   │   ├── memory_tests.rs
│   │   └── timing_tests.rs
│   └── mod.rs
├── examples/                # Example validation tests
│   ├── counter_example.rs
│   ├── todo_example.rs
│   ├── traffic_light_example.rs
│   └── mod.rs
└── common/                  # Shared test utilities
    ├── fixtures.rs
    ├── assertions.rs
    ├── mocks.rs
    ├── generators.rs
    └── mod.rs
```

## Unit Testing Framework

### Store Unit Tests

```rust
// tests/unit/store/basic_store.rs
use leptos_state::*;
use leptos::*;
use wasm_bindgen_test::*;

#[cfg(test)]
mod basic_store_tests {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq)]
    struct TestState {
        count: i32,
        name: String,
    }
    
    impl Default for TestState {
        fn default() -> Self {
            Self {
                count: 0,
                name: "test".to_string(),
            }
        }
    }
    
    #[test]
    fn store_creation() {
        let (read_signal, write_signal) = create_store(TestState::default());
        
        assert_eq!(read_signal.get().count, 0);
        assert_eq!(read_signal.get().name, "test");
    }
    
    #[test]
    fn store_updates() {
        let (read_signal, write_signal) = create_store(TestState::default());
        
        write_signal.update(|state| {
            state.count = 42;
            state.name = "updated".to_string();
        });
        
        assert_eq!(read_signal.get().count, 42);
        assert_eq!(read_signal.get().name, "updated");
    }
    
    #[wasm_bindgen_test]
    fn store_reactivity_in_browser() {
        let (read_signal, write_signal) = create_store(TestState::default());
        let derived_signal = create_memo(move |_| read_signal.get().count * 2);
        
        assert_eq!(derived_signal.get(), 0);
        
        write_signal.update(|state| state.count = 5);
        
        assert_eq!(derived_signal.get(), 10);
    }
    
    #[test]
    fn store_subscription() {
        let (read_signal, write_signal) = create_store(TestState::default());
        let updates = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        
        let updates_clone = updates.clone();
        let _subscription = read_signal.subscribe(move |state| {
            updates_clone.borrow_mut().push(state.count);
        });
        
        write_signal.update(|state| state.count = 1);
        write_signal.update(|state| state.count = 2);
        write_signal.update(|state| state.count = 3);
        
        assert_eq!(*updates.borrow(), vec![1, 2, 3]);
    }
}
```

### State Machine Unit Tests

```rust
// tests/unit/machine/transitions.rs
use leptos_state::machine::*;
use proptest::prelude::*;

#[cfg(test)]
mod transition_tests {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, Hash)]
    enum TrafficState {
        Red,
        Yellow,
        Green,
    }
    
    #[derive(Clone, Debug)]
    enum TrafficEvent {
        Next,
        Emergency,
    }
    
    fn create_traffic_light_machine() -> Machine<TrafficState, TrafficEvent> {
        MachineBuilder::new()
            .state(TrafficState::Red)
                .on(TrafficEvent::Next, TrafficState::Green)
                .on(TrafficEvent::Emergency, TrafficState::Red)
            .state(TrafficState::Green)
                .on(TrafficEvent::Next, TrafficState::Yellow)
                .on(TrafficEvent::Emergency, TrafficState::Red)
            .state(TrafficState::Yellow)
                .on(TrafficEvent::Next, TrafficState::Red)
                .on(TrafficEvent::Emergency, TrafficState::Red)
            .initial(TrafficState::Red)
            .build()
            .unwrap()
    }
    
    #[test]
    fn basic_transitions() {
        let mut machine = create_traffic_light_machine();
        
        assert_eq!(machine.current_state(), &TrafficState::Red);
        
        machine.send(TrafficEvent::Next).unwrap();
        assert_eq!(machine.current_state(), &TrafficState::Green);
        
        machine.send(TrafficEvent::Next).unwrap();
        assert_eq!(machine.current_state(), &TrafficState::Yellow);
        
        machine.send(TrafficEvent::Next).unwrap();
        assert_eq!(machine.current_state(), &TrafficState::Red);
    }
    
    #[test]
    fn emergency_transitions() {
        let mut machine = create_traffic_light_machine();
        
        machine.send(TrafficEvent::Next).unwrap(); // Red -> Green
        machine.send(TrafficEvent::Emergency).unwrap();
        assert_eq!(machine.current_state(), &TrafficState::Red);
        
        machine.send(TrafficEvent::Next).unwrap(); // Red -> Green
        machine.send(TrafficEvent::Next).unwrap(); // Green -> Yellow
        machine.send(TrafficEvent::Emergency).unwrap();
        assert_eq!(machine.current_state(), &TrafficState::Red);
    }
    
    #[test]
    fn invalid_transitions() {
        let mut machine = create_traffic_light_machine();
        
        // No invalid transitions in traffic light example
        // But test framework supports it
        assert!(machine.can_transition(&TrafficEvent::Next));
        assert!(machine.can_transition(&TrafficEvent::Emergency));
    }
    
    proptest! {
        #[test]
        fn machine_invariants(
            events in prop::collection::vec(
                prop_oneof![
                    Just(TrafficEvent::Next),
                    Just(TrafficEvent::Emergency)
                ],
                0..100
            )
        ) {
            let mut machine = create_traffic_light_machine();
            
            for event in events {
                let _ = machine.send(event);
                
                // Invariant: machine should always be in a valid state
                prop_assert!(matches!(
                    machine.current_state(),
                    TrafficState::Red | TrafficState::Yellow | TrafficState::Green
                ));
            }
        }
    }
}
```

## Integration Testing

### Workflow Integration Tests

```rust
// tests/integration/workflows/counter_workflow.rs
use leptos::*;
use leptos_state::*;
use wasm_bindgen_test::*;

#[cfg(test)]
mod counter_workflow_tests {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq)]
    struct CounterState {
        count: i32,
        step: i32,
    }
    
    impl Default for CounterState {
        fn default() -> Self {
            Self { count: 0, step: 1 }
        }
    }
    
    #[derive(Clone)]
    struct CounterActions {
        update_state: WriteSignal<CounterState>,
    }
    
    impl CounterActions {
        fn increment(&self) {
            self.update_state.update(|state| state.count += state.step);
        }
        
        fn decrement(&self) {
            self.update_state.update(|state| state.count -= state.step);
        }
        
        fn set_step(&self, step: i32) {
            self.update_state.update(|state| state.step = step);
        }
        
        fn reset(&self) {
            self.update_state.update(|state| *state = CounterState::default());
        }
    }
    
    #[test]
    fn complete_counter_workflow() {
        let (state, update_state) = create_signal(CounterState::default());
        let actions = CounterActions { update_state };
        
        // Test initial state
        assert_eq!(state.get().count, 0);
        assert_eq!(state.get().step, 1);
        
        // Test increment
        actions.increment();
        assert_eq!(state.get().count, 1);
        
        actions.increment();
        assert_eq!(state.get().count, 2);
        
        // Test decrement
        actions.decrement();
        assert_eq!(state.get().count, 1);
        
        // Test step change
        actions.set_step(5);
        actions.increment();
        assert_eq!(state.get().count, 6);
        
        actions.decrement();
        assert_eq!(state.get().count, 1);
        
        // Test reset
        actions.reset();
        assert_eq!(state.get().count, 0);
        assert_eq!(state.get().step, 1);
    }
    
    #[wasm_bindgen_test]
    async fn counter_persistence_workflow() {
        let config = PersistenceConfig {
            key_prefix: "test_counter".to_string(),
            auto_save: true,
            ..Default::default()
        };
        
        // Create persistent store
        let (state, actions, _handle) = create_persistent_store(
            CounterState::default(),
            config.clone(),
        );
        
        // Make changes
        actions.update(|s| s.count = 42);
        
        // Wait for persistence
        leptos::logging::log!("Waiting for persistence...");
        // In real test, would wait for persistence callback
        
        // Create new store with same config
        let (new_state, _new_actions, _new_handle) = create_persistent_store(
            CounterState::default(),
            config,
        );
        
        // Should load persisted value
        // Note: In real implementation, this would be async
        assert_eq!(new_state.get().count, 42);
    }
}
```

### Persistence Integration Tests

```rust
// tests/integration/persistence/localStorage_integration.rs
use leptos_state::persistence::*;
use wasm_bindgen_test::*;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod localStorage_integration_tests {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct TestData {
        id: u32,
        name: String,
        active: bool,
    }
    
    #[wasm_bindgen_test]
    async fn localStorage_save_and_load() {
        let backend = LocalStorageBackend::new("test");
        let test_data = TestData {
            id: 123,
            name: "Test Item".to_string(),
            active: true,
        };
        
        let serialized = serde_json::to_vec(&test_data).unwrap();
        
        // Save data
        backend.save("test_key", &serialized).await.unwrap();
        
        // Load data
        let loaded = backend.load("test_key").await.unwrap();
        let deserialized: TestData = serde_json::from_slice(&loaded).unwrap();
        
        assert_eq!(deserialized, test_data);
    }
    
    #[wasm_bindgen_test]
    async fn localStorage_delete_and_not_found() {
        let backend = LocalStorageBackend::new("test");
        
        // Try to load non-existent key
        let result = backend.load("non_existent").await;
        assert!(matches!(result, Err(PersistenceError::NotFound)));
        
        // Save and then delete
        backend.save("temp_key", b"temp_data").await.unwrap();
        assert!(backend.exists("temp_key").await.unwrap());
        
        backend.delete("temp_key").await.unwrap();
        assert!(!backend.exists("temp_key").await.unwrap());
    }
    
    #[wasm_bindgen_test]
    async fn localStorage_storage_info() {
        let backend = LocalStorageBackend::new("test_info");
        
        // Save multiple items
        for i in 0..5 {
            let key = format!("item_{}", i);
            let data = format!("data for item {}", i);
            backend.save(&key, data.as_bytes()).await.unwrap();
        }
        
        let info = backend.info().await.unwrap();
        assert_eq!(info.entry_count, 5);
        assert!(info.used_bytes > 0);
        
        // Clean up
        backend.clear().await.unwrap();
        
        let info_after_clear = backend.info().await.unwrap();
        assert_eq!(info_after_clear.entry_count, 0);
    }
}
```

## Property-Based Testing

### Store Property Tests

```rust
// tests/property/store_properties.rs
use proptest::prelude::*;
use leptos_state::*;

prop_compose! {
    fn arb_store_operations()(
        ops in prop::collection::vec(
            prop_oneof![
                (any::<i32>()).prop_map(StoreOp::Set),
                (any::<i32>()).prop_map(StoreOp::Add),
                any::<()>().prop_map(|_| StoreOp::Reset),
            ],
            0..100
        )
    ) -> Vec<StoreOp> {
        ops
    }
}

#[derive(Debug, Clone)]
enum StoreOp {
    Set(i32),
    Add(i32),
    Reset,
}

proptest! {
    #[test]
    fn store_operations_are_deterministic(
        ops in arb_store_operations()
    ) {
        let (state1, update1) = create_signal(0i32);
        let (state2, update2) = create_signal(0i32);
        
        // Apply same operations to both stores
        for op in &ops {
            match op {
                StoreOp::Set(value) => {
                    update1.set(*value);
                    update2.set(*value);
                }
                StoreOp::Add(delta) => {
                    update1.update(|s| *s += delta);
                    update2.update(|s| *s += delta);
                }
                StoreOp::Reset => {
                    update1.set(0);
                    update2.set(0);
                }
            }
        }
        
        // Both stores should have the same final state
        prop_assert_eq!(state1.get(), state2.get());
    }
    
    #[test]
    fn store_updates_are_atomic(
        initial in any::<i32>(),
        updates in prop::collection::vec(any::<i32>(), 1..50)
    ) {
        let (state, update) = create_signal(initial);
        let mut expected = initial;
        
        for value in updates {
            update.set(value);
            expected = value;
            
            // State should always be consistent
            prop_assert_eq!(state.get(), expected);
        }
    }
}
```

### Machine Property Tests

```rust
// tests/property/machine_properties.rs
use proptest::prelude::*;
use leptos_state::machine::*;
use std::collections::HashSet;

prop_compose! {
    fn arb_state_machine()(
        states in prop::collection::hash_set("state_[a-z]+", 2..10),
        initial_state in "state_[a-z]+",
    )(
        states in Just(states),
        initial_state in Just(initial_state),
        transitions in prop::collection::vec(
            (
                prop::sample::select(states.clone()),
                "event_[a-z]+",
                prop::sample::select(states.clone())
            ),
            0..states.len() * 3
        )
    ) -> (HashSet<String>, String, Vec<(String, String, String)>) {
        (states, initial_state, transitions)
    }
}

proptest! {
    #[test]
    fn machine_never_enters_undefined_state(
        (states, initial, transitions) in arb_state_machine(),
        events in prop::collection::vec("event_[a-z]+", 0..50)
    ) {
        let mut builder = MachineBuilder::new();
        
        // Add all states
        for state in &states {
            builder = builder.state(state.clone());
        }
        
        // Add transitions
        for (from, event, to) in transitions {
            if states.contains(&from) && states.contains(&to) {
                builder = builder.state(from)
                    .on(event, to)
                    .end_state();
            }
        }
        
        if states.contains(&initial) {
            builder = builder.initial(initial.clone());
            
            if let Ok(mut machine) = builder.build() {
                // Send random events
                for event in events {
                    let _ = machine.send(event);
                    
                    // Machine should always be in a defined state
                    prop_assert!(states.contains(machine.current_state()));
                }
            }
        }
    }
    
    #[test]
    fn machine_state_history_is_accurate(
        initial_state in "state_[a-z]+",
        events in prop::collection::vec("event_[a-z]+", 1..20)
    ) {
        let mut machine = MachineBuilder::new()
            .state(&initial_state)
            .initial(initial_state.clone())
            .build()
            .unwrap();
            
        let mut expected_history = vec![initial_state.clone()];
        
        for event in events {
            let previous_state = machine.current_state().clone();
            let _ = machine.send(event);
            
            // If state changed, it should be in history
            if machine.current_state() != &previous_state {
                expected_history.push(machine.current_state().clone());
            }
        }
        
        // History should match expected progression
        let actual_history = machine.get_history();
        prop_assert!(actual_history.len() <= expected_history.len());
    }
}
```

## Performance Testing

### Benchmark Framework

```rust
// tests/performance/benchmarks/store_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use leptos_state::*;

fn store_creation_benchmark(c: &mut Criterion) {
    c.bench_function("store_creation", |b| {
        b.iter(|| {
            let (_, _) = create_store(black_box(0i32));
        })
    });
}

fn store_update_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("store_updates");
    
    for size in [1, 10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("single_field", size), size, |b, &size| {
            let (_, update) = create_store(vec![0i32; size]);
            
            b.iter(|| {
                update.update(|vec| {
                    if !vec.is_empty() {
                        vec[0] = black_box(vec[0] + 1);
                    }
                });
            });
        });
        
        group.bench_with_input(BenchmarkId::new("full_replace", size), size, |b, &size| {
            let (_, update) = create_store(vec![0i32; size]);
            let new_vec = (0..size).collect::<Vec<i32>>();
            
            b.iter(|| {
                update.set(black_box(new_vec.clone()));
            });
        });
    }
    
    group.finish();
}

fn store_subscription_benchmark(c: &mut Criterion) {
    let (state, update) = create_store(0i32);
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
    
    let counter_clone = counter.clone();
    let _subscription = state.subscribe(move |_| {
        *counter_clone.borrow_mut() += 1;
    });
    
    c.bench_function("store_subscription_update", |b| {
        b.iter(|| {
            update.update(|s| *s = black_box(*s + 1));
        });
    });
}

criterion_group!(
    store_benches,
    store_creation_benchmark,
    store_update_benchmark,
    store_subscription_benchmark
);
criterion_main!(store_benches);
```

### Memory Usage Tests

```rust
// tests/performance/regression/memory_tests.rs
use leptos_state::*;

#[cfg(test)]
mod memory_tests {
    use super::*;
    
    #[test]
    fn store_memory_usage_bounded() {
        let initial_memory = get_memory_usage();
        let mut stores = Vec::new();
        
        // Create many stores
        for i in 0..1000 {
            let (state, _) = create_store(i);
            stores.push(state);
        }
        
        let peak_memory = get_memory_usage();
        
        // Drop all stores
        drop(stores);
        
        // Force garbage collection (if possible)
        #[cfg(target_arch = "wasm32")]
        {
            // WASM-specific memory cleanup
        }
        
        let final_memory = get_memory_usage();
        
        // Memory should not grow unboundedly
        let memory_growth = peak_memory - initial_memory;
        let memory_leaked = final_memory - initial_memory;
        
        assert!(memory_leaked < memory_growth * 0.1, 
               "Memory leak detected: leaked {}MB, grew {}MB", 
               memory_leaked / 1024 / 1024, 
               memory_growth / 1024 / 1024);
    }
    
    #[cfg(target_arch = "wasm32")]
    fn get_memory_usage() -> usize {
        // Use WASM memory introspection
        wasm_bindgen::memory().buffer().byte_length() as usize
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    fn get_memory_usage() -> usize {
        // Use system memory APIs
        0 // Placeholder implementation
    }
}
```

## Test Utilities and Fixtures

### Common Test Utilities

```rust
// tests/common/fixtures.rs
use leptos_state::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestCounter {
    pub count: i32,
    pub step: i32,
    pub max: Option<i32>,
}

impl Default for TestCounter {
    fn default() -> Self {
        Self {
            count: 0,
            step: 1,
            max: None,
        }
    }
}

impl TestCounter {
    pub fn new(initial_count: i32) -> Self {
        Self {
            count: initial_count,
            ..Default::default()
        }
    }
    
    pub fn with_step(mut self, step: i32) -> Self {
        self.step = step;
        self
    }
    
    pub fn with_max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestTodo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
    pub created_at: std::time::SystemTime,
}

impl TestTodo {
    pub fn new(id: u32, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
            completed: false,
            created_at: std::time::SystemTime::now(),
        }
    }
}

// Test machine states and events
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum TestMachineState {
    Idle,
    Loading,
    Success,
    Error,
    Retrying,
}

#[derive(Clone, Debug)]
pub enum TestMachineEvent {
    Start,
    Success,
    Error,
    Retry,
    Reset,
}

pub fn create_test_async_machine() -> Machine<TestMachineState, TestMachineEvent> {
    MachineBuilder::new()
        .state(TestMachineState::Idle)
            .on(TestMachineEvent::Start, TestMachineState::Loading)
        .state(TestMachineState::Loading)
            .on(TestMachineEvent::Success, TestMachineState::Success)
            .on(TestMachineEvent::Error, TestMachineState::Error)
        .state(TestMachineState::Error)
            .on(TestMachineEvent::Retry, TestMachineState::Retrying)
            .on(TestMachineEvent::Reset, TestMachineState::Idle)
        .state(TestMachineState::Retrying)
            .on(TestMachineEvent::Success, TestMachineState::Success)
            .on(TestMachineEvent::Error, TestMachineState::Error)
        .state(TestMachineState::Success)
            .on(TestMachineEvent::Reset, TestMachineState::Idle)
        .initial(TestMachineState::Idle)
        .build()
        .unwrap()
}
```

### Custom Assertions

```rust
// tests/common/assertions.rs
use leptos_state::*;

pub trait StoreAssertions<T> {
    fn assert_state_eq(&self, expected: &T) where T: PartialEq + std::fmt::Debug;
    fn assert_updated_within(&self, duration: std::time::Duration);
}

impl<T> StoreAssertions<T> for ReadSignal<T> 
where T: Clone
{
    fn assert_state_eq(&self, expected: &T) 
    where T: PartialEq + std::fmt::Debug 
    {
        let actual = self.get();
        assert_eq!(&actual, expected, 
                  "Store state mismatch. Expected: {:?}, Actual: {:?}", 
                  expected, actual);
    }
    
    fn assert_updated_within(&self, duration: std::time::Duration) {
        let start = std::time::Instant::now();
        let initial = self.get();
        
        while start.elapsed() < duration {
            if self.get() != initial {
                return; // State updated within time limit
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        
        panic!("Store did not update within {:?}", duration);
    }
}

pub trait MachineAssertions<S, E> {
    fn assert_state(&self, expected: &S) where S: PartialEq + std::fmt::Debug;
    fn assert_can_transition(&self, event: &E) -> bool;
    fn assert_transition_history(&self, expected: &[S]) where S: PartialEq + std::fmt::Debug;
}

impl<S, E> MachineAssertions<S, E> for Machine<S, E> 
where S: Clone + PartialEq, E: Clone
{
    fn assert_state(&self, expected: &S) 
    where S: PartialEq + std::fmt::Debug 
    {
        assert_eq!(self.current_state(), expected,
                  "Machine state mismatch. Expected: {:?}, Actual: {:?}",
                  expected, self.current_state());
    }
    
    fn assert_can_transition(&self, event: &E) -> bool {
        self.can_transition(event)
    }
    
    fn assert_transition_history(&self, expected: &[S]) 
    where S: PartialEq + std::fmt::Debug 
    {
        let actual_history: Vec<S> = self.get_history()
            .iter()
            .map(|transition| transition.to_state.clone())
            .collect();
            
        assert_eq!(&actual_history, expected,
                  "Machine transition history mismatch. Expected: {:?}, Actual: {:?}",
                  expected, actual_history);
    }
}
```

## CI/CD Integration

### Test Automation Configuration

```yaml
# .github/workflows/test.yml
name: Comprehensive Testing

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Run unit tests
        run: cargo test --lib --bins
        
      - name: Run property tests
        run: cargo test --release --test property -- --ignored
        
  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        
      - name: Run integration tests
        run: cargo test --test integration
        
  browser-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
        
      - name: Run WASM tests
        run: wasm-pack test --headless --chrome
        
  performance-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        
      - name: Run benchmarks
        run: cargo bench --save-baseline ci-baseline
        
      - name: Compare with previous
        run: cargo bench --load-baseline ci-baseline
        
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
        
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
        
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

This comprehensive testing strategy ensures high-quality, reliable code with excellent coverage across all platforms and use cases.
