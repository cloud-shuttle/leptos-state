//! Integration tests for state machine functionality

use leptos::prelude::*;
use leptos_state::*;
use wasm_bindgen_test::*;

use super::fixtures::*;
use leptos_state::machine::actions::FunctionAction;
use leptos_state::machine::guards::FunctionGuard;
use leptos_state::machine::states::StateValue;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn machine_creation_and_initial_state() {
    let machine = create_test_machine();
    let initial_state = machine.initial_state();

    assert_eq!(initial_state.value(), &StateValue::simple("idle"));
    assert_eq!(initial_state.context().value, 0);
    assert_eq!(initial_state.context().flag, false);
}

#[wasm_bindgen_test]
fn machine_basic_transitions() {
    let machine = create_test_machine();
    let initial_state = machine.initial_state();

    // Test valid transition
    let active_state = machine.transition(&initial_state, TestEvent::Increment);
    assert_eq!(active_state.value(), &StateValue::simple("active"));

    // Test transition back
    let idle_state = machine.transition(&active_state, TestEvent::Decrement);
    assert_eq!(idle_state.value(), &StateValue::simple("idle"));

    // Test reset from active
    let active_again = machine.transition(&idle_state, TestEvent::Increment);
    let reset_state = machine.transition(&active_again, TestEvent::Reset);
    assert_eq!(reset_state.value(), &StateValue::simple("idle"));
}

#[wasm_bindgen_test]
fn machine_invalid_transitions() {
    let machine = create_test_machine();
    let initial_state = machine.initial_state();

    // Invalid transition - decrement from idle should stay in idle
    let still_idle = machine.transition(&initial_state, TestEvent::Decrement);
    assert_eq!(still_idle.value(), &StateValue::simple("idle"));

    // Invalid transition - toggle event doesn't exist in our machine
    let still_idle2 = machine.transition(&initial_state, TestEvent::Toggle);
    assert_eq!(still_idle2.value(), &StateValue::simple("idle"));
}

#[wasm_bindgen_test]
fn machine_with_guards() {
    let guard = FunctionGuard::new(|ctx: &TestContext, _| ctx.value >= 5);

    let machine = MachineBuilder::<TestContext, TestEvent>::new()
        .state("locked")
        .on(TestEvent::Increment, "unlocked")
        .guard(guard)
        .state("unlocked")
        .on(TestEvent::Reset, "locked")
        .initial("locked")
        .build();

    let initial_context = TestContext {
        value: 3,
        flag: false,
    };
    let initial_state = machine.initial_with_context(initial_context);

    // Should not transition because guard fails (value < 5)
    let still_locked = machine.transition(&initial_state, TestEvent::Increment);
    assert_eq!(still_locked.value(), &StateValue::simple("locked"));

    // Update context to satisfy guard
    let context_with_high_value = TestContext {
        value: 10,
        flag: false,
    };
    let state_with_high_value = machine.initial_with_context(context_with_high_value);

    // Should transition because guard passes (value >= 5)
    let unlocked = machine.transition(&state_with_high_value, TestEvent::Increment);
    assert_eq!(unlocked.value(), &StateValue::simple("unlocked"));
}

#[wasm_bindgen_test]
fn machine_with_actions() {
    let increment_action = FunctionAction::new(|ctx: &mut TestContext, _| {
        ctx.value += 1;
    });

    let toggle_action = FunctionAction::new(|ctx: &mut TestContext, _| {
        ctx.flag = !ctx.flag;
    });

    let machine = MachineBuilder::<TestContext, TestEvent>::new()
        .state("start")
        .on_entry(increment_action)
        .on(TestEvent::Toggle, "end")
        .action(toggle_action)
        .state("end")
        .initial("start")
        .build();

    let initial_state = machine.initial_state();

    // Transition should execute the action
    let end_state = machine.transition(&initial_state, TestEvent::Toggle);

    assert_eq!(end_state.value(), &StateValue::simple("end"));
    assert_eq!(end_state.context().flag, true); // Should be toggled by action
}

#[wasm_bindgen_test]
fn state_value_matching() {
    let simple_state = StateValue::simple("idle");
    assert!(simple_state.matches("idle"));
    assert!(!simple_state.matches("active"));
    assert!(simple_state.matches("*")); // Wildcard should always match

    let compound_state = StateValue::compound("parent", StateValue::simple("child"));
    assert!(compound_state.matches("parent"));
    assert!(compound_state.matches("child"));
    assert!(compound_state.matches("parent.child"));
    assert!(!compound_state.matches("parent.other"));
}

#[test]
fn machine_builder_fluent_api() {
    // Test that the builder pattern works correctly
    let machine = MachineBuilder::<TestContext, TestEvent>::new()
        .state("state1")
        .on(TestEvent::Increment, "state2")
        .state("state2")
        .on(TestEvent::Reset, "state1")
        .state("state3")
        .on(TestEvent::Decrement, "state1")
        .initial("state1")
        .build();

    // Verify machine was built correctly
    assert_eq!(machine.initial_state_id(), "state1");
    assert!(machine.get_states().contains(&"state1".to_string()));
    assert!(machine.get_states().contains(&"state2".to_string()));
}
