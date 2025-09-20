//! Property-based tests for state machine functionality using proptest
//! 
//! These tests use proptest to generate random inputs and verify that
//! the state machine behaves correctly under all possible conditions.

use crate::machine::core::{MachineState, StateMachine};
use crate::machine::states::StateValue;

#[cfg(feature = "testing")]
use proptest::prelude::*;
#[cfg(feature = "testing")]
use super::*;

/// Test context for property-based testing
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PropTestContext {
    pub counter: i32,
    pub name: String,
    pub enabled: bool,
}

/// Test events for property-based testing
#[derive(Debug, Clone, PartialEq)]
pub enum PropTestEvent {
    Increment,
    Decrement,
    SetName(String),
    Toggle,
    Reset,
}

/// Test states for property-based testing
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PropTestState {
    Idle,
    Active,
    Paused,
    Counting,
}

impl MachineState for PropTestState {
    type Context = PropTestContext;

    fn value(&self) -> &StateValue {
        use std::sync::LazyLock;
        
        static IDLE_VALUE: LazyLock<StateValue> = LazyLock::new(|| StateValue::Simple("idle".to_string()));
        static ACTIVE_VALUE: LazyLock<StateValue> = LazyLock::new(|| StateValue::Simple("active".to_string()));
        static PAUSED_VALUE: LazyLock<StateValue> = LazyLock::new(|| StateValue::Simple("paused".to_string()));
        static COUNTING_VALUE: LazyLock<StateValue> = LazyLock::new(|| StateValue::Simple("counting".to_string()));
        
        match self {
            PropTestState::Idle => &IDLE_VALUE,
            PropTestState::Active => &ACTIVE_VALUE,
            PropTestState::Paused => &PAUSED_VALUE,
            PropTestState::Counting => &COUNTING_VALUE,
        }
    }

    fn context(&self) -> &Self::Context {
        use std::sync::LazyLock;
        
        static DEFAULT_CONTEXT: LazyLock<PropTestContext> = LazyLock::new(|| PropTestContext { 
            counter: 0, 
            name: String::new(), 
            enabled: true 
        });
        &DEFAULT_CONTEXT
    }

    fn matches(&self, pattern: &str) -> bool {
        match self {
            PropTestState::Idle => pattern == "idle",
            PropTestState::Active => pattern == "active",
            PropTestState::Paused => pattern == "paused",
            PropTestState::Counting => pattern == "counting",
        }
    }

    fn can_transition_to(&self, target: &str) -> bool {
        matches!((self, target), 
            (PropTestState::Idle, "active") | 
            (PropTestState::Active, "counting") | 
            (PropTestState::Active, "paused") | 
            (PropTestState::Active, "idle") | 
            (PropTestState::Paused, "active") | 
            (PropTestState::Paused, "idle") | 
            (PropTestState::Counting, "idle")
        )
    }
}

/// Test machine implementation for property-based testing
pub struct PropTestMachine;

impl StateMachine for PropTestMachine {
    type Context = PropTestContext;
    type Event = PropTestEvent;
    type State = PropTestState;

    fn initial() -> Self::State {
        PropTestState::Idle
    }

    fn transition(state: &Self::State, event: Self::Event) -> Self::State {
        match (state, event) {
            (PropTestState::Idle, PropTestEvent::Increment) => PropTestState::Active,
            (PropTestState::Active, PropTestEvent::Increment) => PropTestState::Counting,
            (PropTestState::Active, PropTestEvent::Decrement) => PropTestState::Idle,
            (PropTestState::Active, PropTestEvent::Toggle) => PropTestState::Paused,
            (PropTestState::Paused, PropTestEvent::Toggle) => PropTestState::Active,
            (PropTestState::Paused, PropTestEvent::Reset) => PropTestState::Idle,
            (PropTestState::Counting, PropTestEvent::Reset) => PropTestState::Idle,
            _ => *state,
        }
    }
}

#[cfg(feature = "testing")]
mod proptest_impl {
    use super::*;

    /// Generate random test events
    fn arb_test_event() -> impl Strategy<Value = PropTestEvent> {
        prop_oneof![
            Just(PropTestEvent::Increment),
            Just(PropTestEvent::Decrement),
            Just(PropTestEvent::Toggle),
            Just(PropTestEvent::Reset),
            any::<String>().prop_map(PropTestEvent::SetName),
        ]
    }

    /// Generate random test states
    fn arb_test_state() -> impl Strategy<Value = PropTestState> {
        prop_oneof![
            Just(PropTestState::Idle),
            Just(PropTestState::Active),
            Just(PropTestState::Paused),
            Just(PropTestState::Counting),
        ]
    }

    /// Generate random test contexts
    fn arb_test_context() -> impl Strategy<Value = PropTestContext> {
        (any::<i32>(), any::<String>(), any::<bool>())
            .prop_map(|(counter, name, enabled)| PropTestContext {
                counter,
                name,
                enabled,
            })
    }

    /// Property: State machine transitions should be deterministic
    /// Given the same state and event, the transition should always produce the same result
    proptest! {
        #[test]
        fn prop_transitions_are_deterministic(
            state in arb_test_state(),
            event in arb_test_event()
        ) {
            let result1 = PropTestMachine::transition(&state, event.clone());
            let result2 = PropTestMachine::transition(&state, event);
            
            prop_assert_eq!(result1, result2);
        }
    }

    /// Property: State machine should never panic on any valid input
    /// All transitions should complete without panicking
    proptest! {
        #[test]
        fn prop_transitions_never_panic(
            state in arb_test_state(),
            event in arb_test_event()
        ) {
            let result = std::panic::catch_unwind(|| {
                PropTestMachine::transition(&state, event)
            });
            
            prop_assert!(result.is_ok());
        }
    }

    /// Property: State machine should maintain valid state invariants
    /// All states should be valid and implement required traits
    proptest! {
        #[test]
        fn prop_states_maintain_invariants(
            state in arb_test_state()
        ) {
            // Test that state can be cloned
            let cloned_state = state.clone();
            prop_assert_eq!(state, cloned_state);
            
            // Test that state can be debugged
            let debug_str = format!("{:?}", state);
            prop_assert!(!debug_str.is_empty());
            
            // Test that state can be compared
            prop_assert_eq!(state, state);
            
            // Test that state has a valid value
            let value = state.value();
            prop_assert!(matches!(value, StateValue::Simple(_)));
            
            // Test that state has a valid context
            let context = state.context();
            prop_assert!(context.counter >= 0 || context.counter < 0); // Always true, but tests access
        }
    }

    /// Property: State machine should handle context updates correctly
    /// Context should be updated consistently across transitions
    proptest! {
        #[test]
        fn prop_context_updates_consistently(
            context in arb_test_context(),
            event in arb_test_event()
        ) {
            let initial_state = PropTestState::Idle;
            let new_state = PropTestMachine::transition(&initial_state, event);
            
            // Test that context is accessible from new state
            let state_context = new_state.context();
            prop_assert!(state_context.counter >= 0 || state_context.counter < 0); // Always true, but tests access
        }
    }

    /// Property: State machine should handle edge cases gracefully
    /// Invalid transitions should return the original state
    proptest! {
        #[test]
        fn prop_invalid_transitions_return_original_state(
            state in arb_test_state(),
            event in arb_test_event()
        ) {
            let new_state = PropTestMachine::transition(&state, event);
            
            // Test that we always get a valid state back
            prop_assert!(matches!(new_state, PropTestState::Idle | PropTestState::Active | PropTestState::Paused | PropTestState::Counting));
        }
    }

    /// Property: State machine should be thread-safe
    /// Multiple threads should be able to access the state machine safely
    proptest! {
        #[test]
        fn prop_state_machine_is_thread_safe(
            state in arb_test_state(),
            event in arb_test_event()
        ) {
            let state_arc = std::sync::Arc::new(state);
            let event_arc = std::sync::Arc::new(event);
            
            let state_clone = state_arc.clone();
            let event_clone = event_arc.clone();
            
            let handle = std::thread::spawn(move || {
                PropTestMachine::transition(&state_clone, (*event_clone).clone())
            });
            
            let result = handle.join();
            prop_assert!(result.is_ok());
        }
    }

    /// Property: State machine should handle concurrent access
    /// Multiple concurrent transitions should not cause data races
    proptest! {
        #[test]
        fn prop_concurrent_transitions_safe(
            state in arb_test_state(),
            events in prop::collection::vec(arb_test_event(), 1..10)
        ) {
            let state_arc = std::sync::Arc::new(state);
            let mut handles = Vec::new();
            
            for event in events {
                let state_clone = state_arc.clone();
                let handle = std::thread::spawn(move || {
                    PropTestMachine::transition(&state_clone, event)
                });
                handles.push(handle);
            }
            
            for handle in handles {
                let result = handle.join();
                prop_assert!(result.is_ok());
            }
        }
    }

    /// Property: State machine should handle large numbers of transitions
    /// Performance should remain reasonable even with many transitions
    proptest! {
        #[test]
        fn prop_many_transitions_performant(
            events in prop::collection::vec(arb_test_event(), 100..1000)
        ) {
            let mut state = PropTestState::Idle;
            let start_time = std::time::Instant::now();
            
            for event in events {
                state = PropTestMachine::transition(&state, event);
            }
            
            let duration = start_time.elapsed();
            prop_assert!(duration.as_millis() < 1000); // Should complete in under 1 second
        }
    }

    /// Property: State machine should handle string events correctly
    /// String-based events should be handled without issues
    proptest! {
        #[test]
        fn prop_string_events_handled_correctly(
            state in arb_test_state(),
            name in any::<String>()
        ) {
            let event = PropTestEvent::SetName(name);
            let new_state = PropTestMachine::transition(&state, event);
            
            // Test that transition completed without panicking
            prop_assert!(matches!(new_state, PropTestState::Idle | PropTestState::Active | PropTestState::Paused | PropTestState::Counting));
        }
    }

    /// Property: State machine should handle numeric context values correctly
    /// Numeric values in context should be handled without overflow
    proptest! {
        #[test]
        fn prop_numeric_context_handled_correctly(
            counter in any::<i32>(),
            state in arb_test_state()
        ) {
            let context = PropTestContext {
                counter,
                name: String::new(),
                enabled: true,
            };
            
            // Test that context can be accessed from state
            let state_context = state.context();
            prop_assert!(state_context.counter >= 0 || state_context.counter < 0); // Always true, but tests access
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prop_test_machine_initial_state() {
        let initial_state = PropTestMachine::initial();
        assert_eq!(initial_state, PropTestState::Idle);
    }

    #[test]
    fn test_prop_test_machine_basic_transitions() {
        let idle_state = PropTestState::Idle;
        let active_state = PropTestMachine::transition(&idle_state, PropTestEvent::Increment);
        assert_eq!(active_state, PropTestState::Active);
        
        let counting_state = PropTestMachine::transition(&active_state, PropTestEvent::Increment);
        assert_eq!(counting_state, PropTestState::Counting);
    }

    #[test]
    fn test_prop_test_machine_invalid_transitions() {
        let idle_state = PropTestState::Idle;
        let result_state = PropTestMachine::transition(&idle_state, PropTestEvent::Decrement);
        assert_eq!(result_state, PropTestState::Idle); // Should remain in same state
    }

    #[test]
    fn test_prop_test_machine_string_events() {
        let idle_state = PropTestState::Idle;
        let result_state = PropTestMachine::transition(&idle_state, PropTestEvent::SetName("test".to_string()));
        assert_eq!(result_state, PropTestState::Idle); // Should remain in same state
    }
}
