use leptos_state::machine::*;
use leptos_state::utils::types::StateResult;

#[derive(Debug, Clone, PartialEq, Default)]
struct TestContext {
    count: i32,
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
enum TestEvent {
    Start,
    Stop,
    Increment,
    Decrement,
}

impl Event for TestEvent {
    fn event_type(&self) -> &str {
        match self {
            TestEvent::Start => "start",
            TestEvent::Stop => "stop",
            TestEvent::Increment => "increment",
            TestEvent::Decrement => "decrement",
        }
    }
}

#[test]
fn test_basic_machine_creation() {
    // Test basic machine creation without complex builder API
    let machine = Machine {
        states: std::collections::HashMap::new(),
        initial: "idle".to_string(),
    };
    
    assert_eq!(machine.initial, "idle");
    assert!(machine.states.is_empty());
}

#[test]
fn test_machine_state_impl() {
    let state = MachineStateImpl {
        value: StateValue::Simple("idle".to_string()),
        context: TestContext {
            count: 0,
            name: "test".to_string(),
        },
    };
    
    assert_eq!(*state.value(), StateValue::Simple("idle".to_string()));
    assert_eq!(state.context().count, 0);
    assert_eq!(state.context().name, "test");
}

#[test]
fn test_state_value() {
    let simple_state = StateValue::Simple("idle".to_string());
    assert_eq!(simple_state.to_string(), "idle");
    
    let compound_state = StateValue::Compound(vec![
        "parent".to_string(),
        "child".to_string(),
    ]);
    assert_eq!(compound_state.to_string(), "parent.child");
}

#[test]
fn test_event_trait() {
    let start_event = TestEvent::Start;
    let stop_event = TestEvent::Stop;
    
    assert_eq!(start_event.event_type(), "start");
    assert_eq!(stop_event.event_type(), "stop");
}

#[test]
fn test_machine_initial_state() {
    let machine = Machine {
        states: std::collections::HashMap::new(),
        initial: "idle".to_string(),
    };
    
    let initial_state = machine.initial_state();
    assert_eq!(*initial_state.value(), StateValue::Simple("idle".to_string()));
    assert_eq!(initial_state.context().count, 0); // Default value
}

#[test]
fn test_machine_clone() {
    let machine = Machine {
        states: std::collections::HashMap::new(),
        initial: "idle".to_string(),
    };
    
    let cloned = machine.clone();
    assert_eq!(cloned.initial, machine.initial);
    assert_eq!(cloned.states.len(), machine.states.len());
}

#[test]
fn test_context_default() {
    let context = TestContext::default();
    assert_eq!(context.count, 0);
    assert_eq!(context.name, "");
}

#[test]
fn test_context_clone() {
    let context = TestContext {
        count: 42,
        name: "test".to_string(),
    };
    
    let cloned = context.clone();
    assert_eq!(cloned.count, context.count);
    assert_eq!(cloned.name, context.name);
}

#[test]
fn test_context_partial_eq() {
    let context1 = TestContext {
        count: 42,
        name: "test".to_string(),
    };
    
    let context2 = TestContext {
        count: 42,
        name: "test".to_string(),
    };
    
    let context3 = TestContext {
        count: 43,
        name: "test".to_string(),
    };
    
    assert_eq!(context1, context2);
    assert_ne!(context1, context3);
}

#[test]
fn test_event_clone() {
    let event = TestEvent::Increment;
    let cloned = event.clone();
    assert_eq!(event, cloned);
}

#[test]
fn test_event_partial_eq() {
    let event1 = TestEvent::Start;
    let event2 = TestEvent::Start;
    let event3 = TestEvent::Stop;
    
    assert_eq!(event1, event2);
    assert_ne!(event1, event3);
}

#[test]
fn test_state_value_clone() {
    let state_value = StateValue::Simple("idle".to_string());
    let cloned = state_value.clone();
    assert_eq!(state_value, cloned);
}

#[test]
fn test_state_value_partial_eq() {
    let state1 = StateValue::Simple("idle".to_string());
    let state2 = StateValue::Simple("idle".to_string());
    let state3 = StateValue::Simple("running".to_string());
    
    assert_eq!(state1, state2);
    assert_ne!(state1, state3);
}

#[test]
fn test_machine_state_impl_default() {
    let state = MachineStateImpl::default();
    assert_eq!(*state.value(), StateValue::Simple("idle".to_string()));
    assert_eq!(state.context().count, 0);
}

#[test]
fn test_machine_state_impl_clone() {
    let state = MachineStateImpl {
        value: StateValue::Simple("running".to_string()),
        context: TestContext {
            count: 42,
            name: "test".to_string(),
        },
    };
    
    let cloned = state.clone();
    assert_eq!(cloned.value(), state.value());
    assert_eq!(cloned.context().count, state.context().count);
    assert_eq!(cloned.context().name, state.context().name);
}

#[test]
fn test_machine_state_impl_partial_eq() {
    let state1 = MachineStateImpl {
        value: StateValue::Simple("idle".to_string()),
        context: TestContext {
            count: 0,
            name: "test".to_string(),
        },
    };
    
    let state2 = MachineStateImpl {
        value: StateValue::Simple("idle".to_string()),
        context: TestContext {
            count: 0,
            name: "test".to_string(),
        },
    };
    
    let state3 = MachineStateImpl {
        value: StateValue::Simple("running".to_string()),
        context: TestContext {
            count: 0,
            name: "test".to_string(),
        },
    };
    
    assert_eq!(state1, state2);
    assert_ne!(state1, state3);
}
