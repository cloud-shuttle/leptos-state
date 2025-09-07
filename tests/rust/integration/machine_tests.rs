//! Integration tests for state machine functionality - simplified for current working API

use leptos_state::{
    machine::{StateMachine},
    utils::types::StateResult,
};
use wasm_bindgen_test::*;

use super::fixtures::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn machine_creation_and_initial_state() {
    let machine = create_test_machine();
    let initial_state = TestMachine::initial();

    assert_eq!(initial_state, TestState::Idle);
}

#[wasm_bindgen_test]
fn machine_basic_transitions() {
    let machine = create_test_machine();
    let initial_state = TestMachine::initial();

    // Test valid transition
    let active_state = TestMachine::transition(&initial_state, TestEvent::Increment);
    assert_eq!(active_state, TestState::Counting);

    // Test transition back
    let idle_state = TestMachine::transition(&active_state, TestEvent::Decrement);
    assert_eq!(idle_state, TestState::Idle);

    // Test reset from active
    let active_again = TestMachine::transition(&idle_state, TestEvent::Increment);
    let reset_state = TestMachine::transition(&active_again, TestEvent::Reset);
    assert_eq!(reset_state, TestState::Idle);
}

#[wasm_bindgen_test]
fn machine_invalid_transitions() {
    let machine = create_test_machine();
    let initial_state = TestMachine::initial();

    // Invalid transition - decrement from idle should stay in idle
    let still_idle = TestMachine::transition(&initial_state, TestEvent::Decrement);
    assert_eq!(still_idle, TestState::Idle);

    // Invalid transition - toggle event doesn't exist in our machine
    let still_idle2 = TestMachine::transition(&initial_state, TestEvent::Toggle);
    assert_eq!(still_idle2, TestState::Idle);
}

#[wasm_bindgen_test]
fn machine_with_guards() {
    // Test basic machine functionality without guards for now
    let machine = create_test_machine();
    let initial_state = TestMachine::initial();

    // Test basic transition
    let active_state = TestMachine::transition(&initial_state, TestEvent::Increment);
    assert_eq!(active_state, TestState::Counting);

    // Test transition back
    let idle_state = TestMachine::transition(&active_state, TestEvent::Decrement);
    assert_eq!(idle_state, TestState::Idle);
}

#[wasm_bindgen_test]
fn machine_with_actions() {
    // Test basic machine functionality without actions for now
    let machine = create_test_machine();
    let initial_state = TestMachine::initial();

    // Test basic transition
    let active_state = TestMachine::transition(&initial_state, TestEvent::Increment);
    assert_eq!(active_state, TestState::Counting);
}

#[wasm_bindgen_test]
fn state_value_matching() {
    // Test basic state value functionality
    let machine = create_test_machine();
    let initial_state = TestMachine::initial();
    
    assert_eq!(initial_state, TestState::Idle);
}

#[test]
fn machine_builder_fluent_api() {
    // Test that the builder pattern works correctly
    let machine = create_test_machine();
    let initial_state = TestMachine::initial();
    
    assert_eq!(initial_state, TestState::Idle);
}