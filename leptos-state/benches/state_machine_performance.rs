use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use leptos_state::machine::*;
use leptos_state::hooks::*;
use std::time::Duration;

// Test context and events for benchmarking
#[derive(Clone, Debug, PartialEq)]
struct TestContext {
    counter: u32,
    data: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
enum TestEvent {
    Increment,
    Decrement,
    Reset,
    BulkUpdate(Vec<u32>),
}

#[derive(Clone, Debug, PartialEq)]
enum TestState {
    Idle,
    Active,
    Processing,
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            counter: 0,
            data: vec![1, 2, 3, 4, 5],
        }
    }
}

impl MachineState for TestState {
    fn initial() -> Self {
        TestState::Idle
    }
}

// Benchmark state machine creation
fn benchmark_machine_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("machine_creation");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("simple_machine", |b| {
        b.iter(|| {
            let _machine = MachineBuilder::<TestContext, TestEvent, TestState>::new()
                .initial(TestState::Idle)
                .state(TestState::Idle)
                    .on(TestEvent::Increment, TestState::Active)
                .state(TestState::Active)
                    .on(TestEvent::Decrement, TestState::Idle)
                    .on(TestEvent::Reset, TestState::Idle)
                .build();
            black_box(_machine);
        });
    });
    
    group.bench_function("complex_machine", |b| {
        b.iter(|| {
            let _machine = MachineBuilder::<TestContext, TestEvent, TestState>::new()
                .initial(TestState::Idle)
                .state(TestState::Idle)
                    .on(TestEvent::Increment, TestState::Active)
                    .on(TestEvent::BulkUpdate(vec![]), TestState::Processing)
                .state(TestState::Active)
                    .on(TestEvent::Decrement, TestState::Idle)
                    .on(TestEvent::Reset, TestState::Idle)
                    .on(TestEvent::BulkUpdate(vec![]), TestState::Processing)
                .state(TestState::Processing)
                    .on(TestEvent::Reset, TestState::Idle)
                .build();
            black_box(_machine);
        });
    });
    
    group.finish();
}

// Benchmark state transitions
fn benchmark_state_transitions(c: &mut Criterion) {
    let mut group = c.benchmark_group("state_transitions");
    group.measurement_time(Duration::from_secs(10));
    
    let machine = MachineBuilder::<TestContext, TestEvent, TestState>::new()
        .initial(TestState::Idle)
        .state(TestState::Idle)
            .on(TestEvent::Increment, TestState::Active)
        .state(TestState::Active)
            .on(TestEvent::Decrement, TestState::Idle)
            .on(TestEvent::Reset, TestState::Idle)
        .build();
    
    group.bench_function("simple_transition", |b| {
        b.iter(|| {
            let mut context = TestContext::default();
            let _result = machine.transition(black_box(&TestEvent::Increment), &mut context);
            black_box(_result);
        });
    });
    
    group.bench_function("complex_transition", |b| {
        b.iter(|| {
            let mut context = TestContext::default();
            let _result = machine.transition(black_box(&TestEvent::BulkUpdate(vec![1, 2, 3, 4, 5])), &mut context);
            black_box(_result);
        });
    });
    
    group.finish();
}

// Benchmark bulk operations
fn benchmark_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_operations");
    group.measurement_time(Duration::from_secs(10));
    
    let machine = MachineBuilder::<TestContext, TestEvent, TestState>::new()
        .initial(TestState::Idle)
        .state(TestState::Idle)
            .on(TestEvent::Increment, TestState::Active)
        .state(TestState::Active)
            .on(TestEvent::Decrement, TestState::Idle)
            .on(TestEvent::Reset, TestState::Idle)
        .build();
    
    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("bulk_transitions", size), size, |b, &size| {
            b.iter(|| {
                let mut context = TestContext::default();
                for _ in 0..size {
                    let _result = machine.transition(black_box(&TestEvent::Increment), &mut context);
                    black_box(_result);
                }
            });
        });
    }
    
    group.finish();
}

// Benchmark memory usage
fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("machine_memory", |b| {
        b.iter(|| {
            let machines: Vec<_> = (0..1000).map(|_| {
                MachineBuilder::<TestContext, TestEvent, TestState>::new()
                    .initial(TestState::Idle)
                    .state(TestState::Idle)
                        .on(TestEvent::Increment, TestState::Active)
                    .state(TestState::Active)
                        .on(TestEvent::Decrement, TestState::Idle)
                    .build()
            }).collect();
            black_box(machines);
        });
    });
    
    group.bench_function("context_memory", |b| {
        b.iter(|| {
            let contexts: Vec<_> = (0..1000).map(|_| {
                TestContext {
                    counter: 0,
                    data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                }
            }).collect();
            black_box(contexts);
        });
    });
    
    group.finish();
}

// Benchmark store operations
fn benchmark_store_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("store_operations");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("store_creation", |b| {
        b.iter(|| {
            let _store = create_store(black_box(TestContext::default()));
            black_box(_store);
        });
    });
    
    group.bench_function("store_update", |b| {
        let (store, set_store) = create_store(TestContext::default());
        b.iter(|| {
            set_store.update(|ctx| {
                ctx.counter += 1;
            });
            black_box(store.get());
        });
    });
    
    group.bench_function("store_bulk_update", |b| {
        let (store, set_store) = create_store(TestContext::default());
        b.iter(|| {
            for i in 0..100 {
                set_store.update(|ctx| {
                    ctx.counter = i;
                });
            }
            black_box(store.get());
        });
    });
    
    group.finish();
}

// Benchmark persistence operations
fn benchmark_persistence_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("persistence_operations");
    group.measurement_time(Duration::from_secs(10));
    
    let machine = MachineBuilder::<TestContext, TestEvent, TestState>::new()
        .initial(TestState::Idle)
        .state(TestState::Idle)
            .on(TestEvent::Increment, TestState::Active)
        .state(TestState::Active)
            .on(TestEvent::Decrement, TestState::Idle)
        .build();
    
    group.bench_function("serialization", |b| {
        b.iter(|| {
            let context = TestContext::default();
            let state = TestState::Idle;
            let _serialized = serde_json::to_string(&(machine.clone(), state, context)).unwrap();
            black_box(_serialized);
        });
    });
    
    group.bench_function("deserialization", |b| {
        let context = TestContext::default();
        let state = TestState::Idle;
        let serialized = serde_json::to_string(&(machine.clone(), state, context)).unwrap();
        b.iter(|| {
            let _deserialized: (Machine<TestContext, TestEvent, TestState>, TestState, TestContext) = 
                serde_json::from_str(&serialized).unwrap();
            black_box(_deserialized);
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_machine_creation,
    benchmark_state_transitions,
    benchmark_bulk_operations,
    benchmark_memory_usage,
    benchmark_store_operations,
    benchmark_persistence_operations
);

criterion_main!(benches);
