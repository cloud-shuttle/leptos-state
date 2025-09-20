//! Integration test fixtures - simplified for current working API

use leptos::prelude::*;
use leptos_state::{
    machine::{StateMachine, MachineState},
    machine::states::StateValue,
};

/// Helper to track effect executions
pub fn track_effect_count() -> (impl Fn(), ReadSignal<usize>) {
    let (count, set_count) = signal(0usize);
    let trigger = move || set_count.update(|c| *c += 1);
    (trigger, count)
}

/// Test machine implementation for integration tests
pub struct TestMachine;

impl StateMachine for TestMachine {
    type Context = TestContext;
    type Event = TestEvent;
    type State = TestState;

    fn initial() -> Self::State {
        TestState::Idle
    }

    fn transition(state: &Self::State, event: Self::Event) -> Self::State {
        match (state, event) {
            (TestState::Idle, TestEvent::Increment) => TestState::Counting,
            (TestState::Counting, TestEvent::Decrement) => TestState::Idle,
            (TestState::Counting, TestEvent::Increment) => TestState::Counting,
            (TestState::Idle, TestEvent::Decrement) => TestState::Idle,
            (_, TestEvent::Reset) => TestState::Idle,
            _ => state.clone(),
        }
    }
}

/// Create a test machine using the working API
pub fn create_test_machine() -> TestMachine {
    TestMachine
}

/// Test context for integration tests
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TestContext {
    pub counter: i32,
    pub name: String,
    pub enabled: bool,
}

/// Test events for integration tests
#[derive(Debug, Clone, PartialEq)]
pub enum TestEvent {
    Increment,
    Decrement,
    Toggle,
    Reset,
}

/// Test states for integration tests
#[derive(Debug, Clone, PartialEq)]
pub enum TestState {
    Idle,
    Active,
    Paused,
    Counting,
}

impl MachineState for TestState {
    type Context = TestContext;

    fn value(&self) -> &StateValue {
        use std::sync::LazyLock;
        
        static IDLE_VALUE: LazyLock<StateValue> = LazyLock::new(|| StateValue::Simple("idle".to_string()));
        static ACTIVE_VALUE: LazyLock<StateValue> = LazyLock::new(|| StateValue::Simple("active".to_string()));
        static PAUSED_VALUE: LazyLock<StateValue> = LazyLock::new(|| StateValue::Simple("paused".to_string()));
        static COUNTING_VALUE: LazyLock<StateValue> = LazyLock::new(|| StateValue::Simple("counting".to_string()));
        
        match self {
            TestState::Idle => &IDLE_VALUE,
            TestState::Active => &ACTIVE_VALUE,
            TestState::Paused => &PAUSED_VALUE,
            TestState::Counting => &COUNTING_VALUE,
        }
    }

    fn context(&self) -> &Self::Context {
        use std::sync::LazyLock;
        
        static DEFAULT_CONTEXT: LazyLock<TestContext> = LazyLock::new(|| TestContext { 
            counter: 0, 
            name: String::new(), 
            enabled: true 
        });
        &DEFAULT_CONTEXT
    }

    fn matches(&self, pattern: &str) -> bool {
        match self {
            TestState::Idle => pattern == "idle",
            TestState::Active => pattern == "active",
            TestState::Paused => pattern == "paused",
            TestState::Counting => pattern == "counting",
        }
    }

    fn can_transition_to(&self, target: &str) -> bool {
        match (self, target) {
            (TestState::Idle, "active") => true,
            (TestState::Active, "counting") => true,
            (TestState::Active, "paused") => true,
            (TestState::Active, "idle") => true,
            (TestState::Paused, "active") => true,
            (TestState::Paused, "idle") => true,
            (TestState::Counting, "idle") => true,
            _ => false,
        }
    }
}