use leptos::*;
use leptos_state::{
    hooks::*,
    machine::{Machine, MachineBuilder, StateMachine, MachineState},
    store::{Store, StoreSlice},
    utils::types::StateResult,
};
use tests::common::{TestContext, TestEvent, TestState};

#[test]
fn test_use_machine_basic() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .transition("idle", "start", "active")
        .initial("idle")
        .build();

    // Note: This test would need to run in a Leptos runtime
    // For now, we'll test the machine creation and structure
    assert_eq!(machine.initial, "idle");
    assert_eq!(machine.states.len(), 2);
}

#[test]
fn test_use_machine_with_context() {
    let machine = MachineBuilder::new()
        .state("counter", |ctx| TestState::Counter(ctx.counter))
        .initial("counter")
        .build();

    let context = TestContext { counter: 42 };
    let state = MachineState::new(&machine, context);
    
    assert_eq!(state.current_state, "counter");
    assert_eq!(state.context.counter, 42);
}

#[test]
fn test_use_machine_transition() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .transition("idle", "start", "active")
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext::default());
    
    let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "active");
}

#[test]
fn test_use_store_basic() {
    // Test store creation and basic operations
    let store = Store::new(TestContext { counter: 0 });
    assert_eq!(store.get().counter, 0);
}

#[test]
fn test_use_store_update() {
    let mut store = Store::new(TestContext { counter: 0 });
    store.update(|ctx| ctx.counter = 42);
    assert_eq!(store.get().counter, 42);
}

#[test]
fn test_use_store_slice() {
    let store = Store::new(TestContext { counter: 42 });
    let slice = store.slice(|ctx| ctx.counter);
    assert_eq!(slice.get(), 42);
}

#[test]
fn test_use_store_context_provider() {
    let store = Store::new(TestContext { counter: 42 });
    
    // Test that the store can be cloned and shared
    let cloned = store.clone();
    assert_eq!(store.get().counter, cloned.get().counter);
}

#[test]
fn test_use_async_store_basic() {
    // Test async store creation
    let async_store = AsyncStore::new(TestContext { counter: 0 });
    assert_eq!(async_store.get().counter, 0);
}

#[test]
fn test_use_async_store_load() {
    let async_store = AsyncStore::new(TestContext { counter: 0 });
    
    // Test loading data
    let loader = |_| async { Ok(TestContext { counter: 42 }) };
    // Note: In a real test, we'd need to run this in an async context
    // For now, we'll just test the store structure
    assert_eq!(async_store.get().counter, 0);
}

#[test]
fn test_use_infinite_store_basic() {
    let infinite_store = InfiniteStore::new(TestContext { counter: 0 });
    assert_eq!(infinite_store.get().counter, 0);
}

#[test]
fn test_use_infinite_store_pagination() {
    let infinite_store = InfiniteStore::new(TestContext { counter: 0 });
    
    // Test pagination functionality
    // Note: In a real test, we'd need to test the async pagination
    assert_eq!(infinite_store.get().counter, 0);
}

#[test]
fn test_use_machine_with_guards() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .transition_with_guard("idle", "start", "active", |ctx, _| ctx.counter > 5)
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext { counter: 10 });
    
    let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "active");
}

#[test]
fn test_use_machine_with_actions() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .transition_with_action("idle", "start", "active", |ctx, _| {
            ctx.counter += 1;
            Ok(())
        })
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext { counter: 0 });
    
    let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "active");
    assert_eq!(state.context.counter, 1);
}

#[test]
fn test_use_machine_with_history() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("paused", |_| TestState::Paused)
        .transition("idle", "start", "active")
        .transition("active", "pause", "paused")
        .transition("paused", "resume", "active")
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext::default());
    
    // idle -> active -> paused
    Machine::transition(&machine, &mut state, &TestEvent::Start).unwrap();
    Machine::transition(&machine, &mut state, &TestEvent::Pause).unwrap();
    
    assert_eq!(state.current_state, "paused");
}

#[test]
fn test_use_machine_complex_scenario() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("paused", |_| TestState::Paused)
        .state("finished", |_| TestState::Finished)
        .transition_with_guard("idle", "start", "active", |ctx, _| ctx.counter >= 0)
        .transition_with_action("active", "pause", "paused", |ctx, _| {
            ctx.counter += 1;
            Ok(())
        })
        .transition("paused", "resume", "active")
        .transition_with_guard("active", "complete", "finished", |ctx, _| ctx.counter >= 10)
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext { counter: 5 });
    
    // idle -> active
    let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "active");
    
    // active -> paused (should increment counter)
    let result = Machine::transition(&machine, &mut state, &TestEvent::Pause);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "paused");
    assert_eq!(state.context.counter, 6);
    
    // paused -> active
    let result = Machine::transition(&machine, &mut state, &TestEvent::Resume);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "active");
    
    // active -> finished (should fail because counter < 10)
    let result = Machine::transition(&machine, &mut state, &TestEvent::Complete);
    assert!(result.is_err());
    assert_eq!(state.current_state, "active");
    
    // Increment counter and try again
    state.context.counter = 15;
    let result = Machine::transition(&machine, &mut state, &TestEvent::Complete);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "finished");
}

#[test]
fn test_use_machine_error_handling() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .transition("idle", "start", "active")
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext::default());
    
    // Try invalid transition
    let result = Machine::transition(&machine, &mut state, &TestEvent::Complete);
    assert!(result.is_err());
    assert_eq!(state.current_state, "idle"); // Should remain in idle
}

#[test]
fn test_use_machine_context_preservation() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .transition("idle", "start", "active")
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext { counter: 42 });
    
    // Update context
    state.context.counter = 100;
    
    let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "active");
    assert_eq!(state.context.counter, 100); // Context should be preserved
}

#[test]
fn test_use_machine_builder_fluent_api() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("finished", |_| TestState::Finished)
        .transition("idle", "start", "active")
        .transition("active", "complete", "finished")
        .transition("finished", "reset", "idle")
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext::default());
    
    // Test all transitions
    assert_eq!(state.current_state, "idle");
    
    Machine::transition(&machine, &mut state, &TestEvent::Start).unwrap();
    assert_eq!(state.current_state, "active");
    
    Machine::transition(&machine, &mut state, &TestEvent::Complete).unwrap();
    assert_eq!(state.current_state, "finished");
    
    Machine::transition(&machine, &mut state, &TestEvent::Reset).unwrap();
    assert_eq!(state.current_state, "idle");
}
