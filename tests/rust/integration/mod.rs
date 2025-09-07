//! Integration test utilities and fixtures

use leptos::*;
use leptos_state::*;

#[derive(Clone, PartialEq, Debug)]
pub struct TestState {
    pub count: i32,
    pub name: String,
    pub enabled: bool,
}

impl Default for TestState {
    fn default() -> Self {
        Self {
            count: 0,
            name: "test".to_string(),
            enabled: true,
        }
    }
}

create_store!(TestStore, TestState, TestState::default());

/// Create a test runtime for integration tests
pub fn create_test_runtime() -> RuntimeHandle {
    leptos::create_runtime()
}

/// Helper to track effect executions
pub fn track_effect_count() -> (impl Fn(), ReadSignal<usize>) {
    let (count, set_count) = create_signal(0usize);
    let trigger = move || set_count.update(|c| *c += 1);
    (trigger, count)
}

/// Mock state machine for testing
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TestContext {
    pub value: i32,
    pub flag: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestEvent {
    Increment,
    Decrement,
    Toggle,
    Reset,
}

impl Event for TestEvent {
    fn event_type(&self) -> &str {
        match self {
            TestEvent::Increment => "increment",
            TestEvent::Decrement => "decrement", 
            TestEvent::Toggle => "toggle",
            TestEvent::Reset => "reset",
        }
    }
}

pub fn create_test_machine() -> Machine<TestContext, TestEvent> {
    MachineBuilder::<TestContext, TestEvent>::new()
        .state("idle")
            .on(TestEvent::Increment, "active")
        .state("active")
            .on(TestEvent::Decrement, "idle")
            .on(TestEvent::Reset, "idle")
        .initial("idle")
        .build()
}
