use leptos_state::{
    machine::guards::*,
    utils::types::StateResult,
};
use tests::common::{TestContext, TestEvent};

#[test]
fn test_function_guard() {
    let guard = FunctionGuard::new(|ctx: &TestContext, event: &TestEvent| {
        ctx.counter > 10
    });

    let context = TestContext { counter: 15 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_function_guard_false() {
    let guard = FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
        ctx.counter > 10
    });

    let context = TestContext { counter: 5 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_always_guard() {
    let guard = AlwaysGuard;
    
    let context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_never_guard() {
    let guard = NeverGuard;
    
    let context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_and_guard() {
    let guard1 = FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
        ctx.counter > 5
    });
    let guard2 = FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
        ctx.counter < 15
    });
    
    let and_guard = AndGuard::new(guard1, guard2);
    
    let context = TestContext { counter: 10 };
    let event = TestEvent::Start;
    
    let result = and_guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_and_guard_false() {
    let guard1 = FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
        ctx.counter > 5
    });
    let guard2 = FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
        ctx.counter < 15
    });
    
    let and_guard = AndGuard::new(guard1, guard2);
    
    let context = TestContext { counter: 20 };
    let event = TestEvent::Start;
    
    let result = and_guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_or_guard() {
    let guard1 = FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
        ctx.counter > 15
    });
    let guard2 = FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
        ctx.counter < 5
    });
    
    let or_guard = OrGuard::new(guard1, guard2);
    
    let context = TestContext { counter: 3 };
    let event = TestEvent::Start;
    
    let result = or_guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_not_guard() {
    let guard = FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
        ctx.counter > 10
    });
    
    let not_guard = NotGuard::new(guard);
    
    let context = TestContext { counter: 5 };
    let event = TestEvent::Start;
    
    let result = not_guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap()); // Should be true because counter is NOT > 10
}

#[test]
fn test_field_equality_guard() {
    let guard = FieldEqualityGuard::new("counter", 42);
    
    let context = TestContext { counter: 42 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_field_equality_guard_false() {
    let guard = FieldEqualityGuard::new("counter", 42);
    
    let context = TestContext { counter: 10 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_range_guard() {
    let guard = RangeGuard::new("counter", 5..=15);
    
    let context = TestContext { counter: 10 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_range_guard_out_of_bounds() {
    let guard = RangeGuard::new("counter", 5..=15);
    
    let context = TestContext { counter: 20 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_event_type_guard() {
    let guard = EventTypeGuard::new::<TestEvent>();
    
    let context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_composite_guard() {
    let guards = vec![
        Box::new(FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
            ctx.counter > 5
        })),
        Box::new(FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
            ctx.counter < 15
        })),
    ];
    
    let composite = CompositeGuard::new(CompositeLogic::And, guards);
    
    let context = TestContext { counter: 10 };
    let event = TestEvent::Start;
    
    let result = composite.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_guard_builder() {
    let builder = GuardBuilder::new();
    let guard = builder
        .function(|ctx: &TestContext, _event: &TestEvent| ctx.counter > 10)
        .build();
    
    let context = TestContext { counter: 15 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_guard_builder_composite() {
    let guard = GuardBuilder::new()
        .and()
        .function(|ctx: &TestContext, _event: &TestEvent| ctx.counter > 5)
        .function(|ctx: &TestContext, _event: &TestEvent| ctx.counter < 15)
        .build();
    
    let context = TestContext { counter: 10 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_guard_evaluation_error() {
    let guard = FieldEqualityGuard::new("nonexistent_field", 42);
    
    let context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = guard.evaluate(&context, &event);
    assert!(result.is_err());
}

#[test]
fn test_guard_clone() {
    let guard = FunctionGuard::new(|ctx: &TestContext, _event: &TestEvent| {
        ctx.counter > 10
    });
    
    let cloned = guard.clone();
    
    let context = TestContext { counter: 15 };
    let event = TestEvent::Start;
    
    let result1 = guard.evaluate(&context, &event);
    let result2 = cloned.evaluate(&context, &event);
    
    assert_eq!(result1.unwrap(), result2.unwrap());
}
