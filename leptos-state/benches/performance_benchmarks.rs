use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos_state::v1::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Test data structures for benchmarking
#[derive(Clone, Debug, PartialEq, Default)]
struct BenchmarkContext {
    counter: i32,
    data: Vec<String>,
}

impl StateMachineContext for BenchmarkContext {}

#[derive(Clone, Debug, PartialEq, Default)]
enum BenchmarkEvent {
    #[default]
    Increment,
    Decrement,
    AddData(String),
    RemoveData(usize),
}

impl StateMachineEvent for BenchmarkEvent {}

#[derive(Clone, Debug, PartialEq, Default)]
enum BenchmarkState {
    #[default]
    Idle,
    Active,
    Processing,
}

impl StateMachineState for BenchmarkState {
    type Context = BenchmarkContext;
    type Event = BenchmarkEvent;
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
struct BenchmarkStore {
    count: i32,
    items: Vec<String>,
    metadata: HashMap<String, String>,
}

impl StoreState for BenchmarkStore {}

impl Store for BenchmarkStore {
    fn create() -> Self {
        Self::default()
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

// Benchmark state machine operations
fn benchmark_state_machine_creation(c: &mut Criterion) {
    c.bench_function("state_machine_creation", |b| {
        b.iter(|| {
            let context = BenchmarkContext::default();
            let _machine = Machine::new(BenchmarkState::Idle, context);
        });
    });
}

fn benchmark_state_machine_transition(c: &mut Criterion) {
    let context = BenchmarkContext::default();
    let mut machine = Machine::new(BenchmarkState::Idle, context);
    
    // Add states and transitions
    let idle_state = StateNode::new("idle")
        .with_value(StateValue::simple("idle"))
        .with_transition(Transition::new(
            BenchmarkEvent::Increment,
            StateValue::simple("active")
        ));
    
    let active_state = StateNode::new("active")
        .with_value(StateValue::simple("active"));
    
    machine.add_state(idle_state);
    machine.add_state(active_state);
    
    c.bench_function("state_machine_transition", |b| {
        b.iter(|| {
            let _result = machine.transition(BenchmarkEvent::Increment);
        });
    });
}

fn benchmark_store_operations(c: &mut Criterion) {
    c.bench_function("store_creation", |b| {
        b.iter(|| {
            let _store = BenchmarkStore::create();
        });
    });
    
    c.bench_function("store_update", |b| {
        let mut store = BenchmarkStore::create();
        b.iter(|| {
            store.update(|state| {
                state.count += 1;
                state.items.push("test".to_string());
            });
        });
    });
    
    c.bench_function("store_clone", |b| {
        let store = BenchmarkStore::create();
        b.iter(|| {
            let _cloned = store.clone();
        });
    });
}

fn benchmark_persistence_operations(c: &mut Criterion) {
    let manager = PersistenceManager::with_memory_backend();
    let store = BenchmarkStore::create();
    
    c.bench_function("persistence_save", |b| {
        b.iter(|| {
            let _result = manager.save_store("benchmark", &store);
        });
    });
    
    c.bench_function("persistence_load", |b| {
        // Ensure data exists first
        let _ = manager.save_store("benchmark", &store);
        
        b.iter(|| {
            let _result = manager.load_store::<BenchmarkStore>("benchmark");
        });
    });
}

fn benchmark_context_operations(c: &mut Criterion) {
    c.bench_function("context_creation", |b| {
        b.iter(|| {
            let _context: Context<BenchmarkContext> = Context::new();
        });
    });
    
    c.bench_function("context_update", |b| {
        let mut context: Context<BenchmarkContext> = Context::new();
        b.iter(|| {
            let _ = context.update(|ctx: &mut BenchmarkContext| {
                ctx.counter += 1;
                ctx.data.push("test".to_string());
                Ok(())
            });
        });
    });
    
    c.bench_function("context_clone", |b| {
        let context: Context<BenchmarkContext> = Context::new();
        b.iter(|| {
            let _cloned = context.clone();
        });
    });
}

fn benchmark_event_handling(c: &mut Criterion) {
    c.bench_function("event_creation", |b| {
        b.iter(|| {
            let _event = Event::new(BenchmarkEvent::Increment);
        });
    });
    
    c.bench_function("event_queue_operations", |b| {
        let mut queue = EventQueue::new();
        b.iter(|| {
            let _ = queue.enqueue(Event::new(BenchmarkEvent::Increment));
            let _event = queue.dequeue();
        });
    });
}

fn benchmark_memory_usage(c: &mut Criterion) {
    c.bench_function("large_store_memory", |b| {
        b.iter(|| {
            let mut store = BenchmarkStore::default();
            // Simulate large data
            for i in 0..1000 {
                store.items.push(format!("item_{}", i));
                store.metadata.insert(format!("key_{}", i), format!("value_{}", i));
            }
            black_box(store);
        });
    });
    
    c.bench_function("state_machine_memory", |b| {
        b.iter(|| {
            let context = BenchmarkContext {
                counter: 0,
                data: (0..1000).map(|i| format!("data_{}", i)).collect(),
            };
            let _machine = Machine::new(BenchmarkState::Idle, context);
        });
    });
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    c.bench_function("concurrent_store_updates", |b| {
        let store = Arc::new(Mutex::new(BenchmarkStore::create()));
        b.iter(|| {
            let store_clone = Arc::clone(&store);
            std::thread::spawn(move || {
                if let Ok(mut store) = store_clone.lock() {
                    store.update(|state| state.count += 1);
                }
            });
        });
    });
}

// Performance regression tests
fn benchmark_performance_regression(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_regression");
    
    group.bench_function("state_transition_latency", |b| {
        let context = BenchmarkContext::default();
        let mut machine = Machine::new(BenchmarkState::Idle, context);
        
        // Setup machine with states
        let idle_state = StateNode::new("idle")
            .with_value(StateValue::simple("idle"))
            .with_transition(Transition::new(
                BenchmarkEvent::Increment,
                StateValue::simple("active")
            ));
        
        let active_state = StateNode::new("active")
            .with_value(StateValue::simple("active"));
        
        machine.add_state(idle_state);
        machine.add_state(active_state);
        
        b.iter(|| {
            let _result = machine.transition(BenchmarkEvent::Increment);
            // Reset for next iteration
            let _ = machine.reset();
        });
    });
    
    group.bench_function("store_reactive_updates", |b| {
        let store = BenchmarkStore::create();
        b.iter(|| {
            let mut store_clone = store.clone();
            store_clone.update(|state| {
                state.count += 1;
                state.items.push("update".to_string());
            });
            black_box(store_clone);
        });
    });
    
    group.finish();
}

// Custom benchmark configuration
criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.01);
    targets = 
        benchmark_state_machine_creation,
        benchmark_state_machine_transition,
        benchmark_store_operations,
        benchmark_persistence_operations,
        benchmark_context_operations,
        benchmark_event_handling,
        benchmark_memory_usage,
        benchmark_concurrent_operations,
        benchmark_performance_regression
}

criterion_main!(benches);
