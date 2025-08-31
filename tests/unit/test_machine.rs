use leptos_state::{
    machine::{Machine, MachineBuilder, StateMachine, MachineState},
    utils::types::StateResult,
};
use tests::common::{TestContext, TestEvent, TestState};

#[test]
fn test_machine_creation() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .initial("idle")
        .build();

    assert_eq!(machine.initial, "idle");
    assert_eq!(machine.states.len(), 2);
    assert!(machine.states.contains_key("idle"));
    assert!(machine.states.contains_key("active"));
}

#[test]
fn test_machine_transition() {
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
fn test_machine_invalid_transition() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext::default());
    
    let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
    assert!(result.is_err());
    assert_eq!(state.current_state, "idle"); // Should remain in idle
}

#[test]
fn test_machine_context_update() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .transition("idle", "start", "active")
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext { counter: 0 });
    
    // Update context
    state.context.counter = 42;
    
    let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
    assert!(result.is_ok());
    assert_eq!(state.context.counter, 42); // Context should be preserved
}

#[test]
fn test_machine_clone() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .initial("idle")
        .build();

    let cloned = machine.clone();
    assert_eq!(machine.initial, cloned.initial);
    assert_eq!(machine.states.len(), cloned.states.len());
}

#[test]
fn test_machine_builder_fluent_api() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("finished", |_| TestState::Finished)
        .transition("idle", "start", "active")
        .transition("active", "complete", "finished")
        .transition("finished", "reset", "idle")
        .initial("idle")
        .build();

    assert_eq!(machine.initial, "idle");
    assert_eq!(machine.states.len(), 3);
    
    // Test all transitions
    let mut state = MachineState::new(&machine, TestContext::default());
    
    // idle -> active
    let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "active");
    
    // active -> finished
    let result = Machine::transition(&machine, &mut state, &TestEvent::Complete);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "finished");
    
    // finished -> idle
    let result = Machine::transition(&machine, &mut state, &TestEvent::Reset);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "idle");
}

#[test]
fn test_machine_state_validation() {
    // Test with invalid initial state
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .initial("nonexistent")
        .build();

    let mut state = MachineState::new(&machine, TestContext::default());
    let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
    assert!(result.is_err());
}

#[test]
fn test_machine_empty_states() {
    let machine = MachineBuilder::new()
        .initial("idle")
        .build();

    assert_eq!(machine.states.len(), 0);
    assert_eq!(machine.initial, "idle");
}

#[test]
fn test_machine_state_equality() {
    let machine1 = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .initial("idle")
        .build();

    let machine2 = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .initial("idle")
        .build();

    // Machines with same structure should be equal
    assert_eq!(machine1.initial, machine2.initial);
    assert_eq!(machine1.states.len(), machine2.states.len());
}

#[test]
fn test_machine_state_creation_with_context() {
    let machine = MachineBuilder::new()
        .state("counter", |ctx| {
            TestState::Counter(ctx.counter)
        })
        .initial("counter")
        .build();

    let context = TestContext { counter: 42 };
    let state = MachineState::new(&machine, context);
    
    // The state should be created with the context value
    assert_eq!(state.current_state, "counter");
    assert_eq!(state.context.counter, 42);
}

#[test]
fn test_machine_multiple_transitions() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("paused", |_| TestState::Paused)
        .transition("idle", "start", "active")
        .transition("active", "pause", "paused")
        .transition("paused", "resume", "active")
        .transition("active", "stop", "idle")
        .transition("paused", "stop", "idle")
        .initial("idle")
        .build();

    let mut state = MachineState::new(&machine, TestContext::default());
    
    // Test complex transition path
    assert_eq!(state.current_state, "idle");
    
    // idle -> active
    Machine::transition(&machine, &mut state, &TestEvent::Start).unwrap();
    assert_eq!(state.current_state, "active");
    
    // active -> paused
    Machine::transition(&machine, &mut state, &TestEvent::Pause).unwrap();
    assert_eq!(state.current_state, "paused");
    
    // paused -> active
    Machine::transition(&machine, &mut state, &TestEvent::Resume).unwrap();
    assert_eq!(state.current_state, "active");
    
    // active -> idle
    Machine::transition(&machine, &mut state, &TestEvent::Stop).unwrap();
    assert_eq!(state.current_state, "idle");
}
