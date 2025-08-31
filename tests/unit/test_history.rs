use leptos_state::{
    machine::history::*,
    machine::{Machine, MachineBuilder, StateMachine, MachineState},
    utils::types::StateResult,
};
use tests::common::{TestContext, TestEvent, TestState};

#[test]
fn test_history_tracker_creation() {
    let tracker = HistoryTracker::new();
    assert_eq!(tracker.history.len(), 0);
    assert_eq!(tracker.max_depth, 100);
}

#[test]
fn test_history_tracker_with_custom_depth() {
    let tracker = HistoryTracker::with_max_depth(50);
    assert_eq!(tracker.max_depth, 50);
}

#[test]
fn test_history_tracker_push() {
    let mut tracker = HistoryTracker::new();
    
    tracker.push("state1".to_string());
    assert_eq!(tracker.history.len(), 1);
    assert_eq!(tracker.history[0], "state1");
}

#[test]
fn test_history_tracker_push_max_depth() {
    let mut tracker = HistoryTracker::with_max_depth(2);
    
    tracker.push("state1".to_string());
    tracker.push("state2".to_string());
    tracker.push("state3".to_string());
    
    assert_eq!(tracker.history.len(), 2);
    assert_eq!(tracker.history[0], "state2");
    assert_eq!(tracker.history[1], "state3");
}

#[test]
fn test_history_tracker_clear() {
    let mut tracker = HistoryTracker::new();
    
    tracker.push("state1".to_string());
    tracker.push("state2".to_string());
    assert_eq!(tracker.history.len(), 2);
    
    tracker.clear();
    assert_eq!(tracker.history.len(), 0);
}

#[test]
fn test_history_tracker_get_last() {
    let mut tracker = HistoryTracker::new();
    
    assert_eq!(tracker.get_last(), None);
    
    tracker.push("state1".to_string());
    assert_eq!(tracker.get_last(), Some("state1"));
    
    tracker.push("state2".to_string());
    assert_eq!(tracker.get_last(), Some("state2"));
}

#[test]
fn test_history_tracker_clone() {
    let mut tracker = HistoryTracker::new();
    tracker.push("state1".to_string());
    tracker.push("state2".to_string());
    
    let cloned = tracker.clone();
    assert_eq!(tracker.history, cloned.history);
    assert_eq!(tracker.max_depth, cloned.max_depth);
}

#[test]
fn test_history_machine_creation() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("paused", |_| TestState::Paused)
        .transition("idle", "start", "active")
        .transition("active", "pause", "paused")
        .transition("paused", "resume", "active")
        .initial("idle")
        .build();

    let history_machine = HistoryMachine::new(machine);
    assert_eq!(history_machine.machine.initial, "idle");
    assert_eq!(history_machine.history_tracker.history.len(), 0);
}

#[test]
fn test_history_machine_transition() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("paused", |_| TestState::Paused)
        .transition("idle", "start", "active")
        .transition("active", "pause", "paused")
        .transition("paused", "resume", "active")
        .initial("idle")
        .build();

    let mut history_machine = HistoryMachine::new(machine);
    let mut state = MachineState::new(&history_machine.machine, TestContext::default());
    
    // idle -> active
    let result = HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Start);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "active");
    assert_eq!(history_machine.history_tracker.history.len(), 1);
    assert_eq!(history_machine.history_tracker.history[0], "idle");
}

#[test]
fn test_history_machine_multiple_transitions() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("paused", |_| TestState::Paused)
        .transition("idle", "start", "active")
        .transition("active", "pause", "paused")
        .transition("paused", "resume", "active")
        .initial("idle")
        .build();

    let mut history_machine = HistoryMachine::new(machine);
    let mut state = MachineState::new(&history_machine.machine, TestContext::default());
    
    // idle -> active -> paused
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Start).unwrap();
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Pause).unwrap();
    
    assert_eq!(state.current_state, "paused");
    assert_eq!(history_machine.history_tracker.history.len(), 2);
    assert_eq!(history_machine.history_tracker.history[0], "idle");
    assert_eq!(history_machine.history_tracker.history[1], "active");
}

#[test]
fn test_history_machine_restore_last() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("paused", |_| TestState::Paused)
        .transition("idle", "start", "active")
        .transition("active", "pause", "paused")
        .transition("paused", "resume", "active")
        .initial("idle")
        .build();

    let mut history_machine = HistoryMachine::new(machine);
    let mut state = MachineState::new(&history_machine.machine, TestContext::default());
    
    // idle -> active -> paused
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Start).unwrap();
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Pause).unwrap();
    
    // Restore to last state (active)
    let result = history_machine.restore_last(&mut state);
    assert!(result.is_ok());
    assert_eq!(state.current_state, "active");
    assert_eq!(history_machine.history_tracker.history.len(), 1);
}

#[test]
fn test_history_machine_restore_empty_history() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .initial("idle")
        .build();

    let mut history_machine = HistoryMachine::new(machine);
    let mut state = MachineState::new(&history_machine.machine, TestContext::default());
    
    // Try to restore with empty history
    let result = history_machine.restore_last(&mut state);
    assert!(result.is_err());
    assert_eq!(state.current_state, "idle"); // Should remain unchanged
}

#[test]
fn test_history_machine_clear_history() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .transition("idle", "start", "active")
        .initial("idle")
        .build();

    let mut history_machine = HistoryMachine::new(machine);
    let mut state = MachineState::new(&history_machine.machine, TestContext::default());
    
    // idle -> active
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Start).unwrap();
    assert_eq!(history_machine.history_tracker.history.len(), 1);
    
    history_machine.clear_history();
    assert_eq!(history_machine.history_tracker.history.len(), 0);
}

#[test]
fn test_history_machine_get_history() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("paused", |_| TestState::Paused)
        .transition("idle", "start", "active")
        .transition("active", "pause", "paused")
        .initial("idle")
        .build();

    let mut history_machine = HistoryMachine::new(machine);
    let mut state = MachineState::new(&history_machine.machine, TestContext::default());
    
    // idle -> active -> paused
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Start).unwrap();
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Pause).unwrap();
    
    let history = history_machine.get_history();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], "idle");
    assert_eq!(history[1], "active");
}

#[test]
fn test_history_machine_clone() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .transition("idle", "start", "active")
        .initial("idle")
        .build();

    let mut history_machine = HistoryMachine::new(machine);
    let mut state = MachineState::new(&history_machine.machine, TestContext::default());
    
    // idle -> active
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Start).unwrap();
    
    let cloned = history_machine.clone();
    assert_eq!(history_machine.machine.initial, cloned.machine.initial);
    assert_eq!(history_machine.history_tracker.history, cloned.history_tracker.history);
}

#[test]
fn test_history_machine_max_depth() {
    let machine = MachineBuilder::new()
        .state("idle", |_| TestState::Idle)
        .state("active", |_| TestState::Active)
        .state("paused", |_| TestState::Paused)
        .transition("idle", "start", "active")
        .transition("active", "pause", "paused")
        .transition("paused", "resume", "active")
        .initial("idle")
        .build();

    let mut history_machine = HistoryMachine::with_max_depth(machine, 2);
    let mut state = MachineState::new(&history_machine.machine, TestContext::default());
    
    // idle -> active -> paused -> active
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Start).unwrap();
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Pause).unwrap();
    HistoryMachine::transition(&mut history_machine, &mut state, &TestEvent::Resume).unwrap();
    
    // Should only keep last 2 states
    assert_eq!(history_machine.history_tracker.history.len(), 2);
    assert_eq!(history_machine.history_tracker.history[0], "active");
    assert_eq!(history_machine.history_tracker.history[1], "paused");
}
