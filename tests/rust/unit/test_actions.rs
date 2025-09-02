use leptos_state::{
    machine::actions::*,
    utils::types::StateResult,
};
use tests::common::{TestContext, TestEvent};

#[test]
fn test_function_action() {
    let action = FunctionAction::new(|ctx: &mut TestContext, event: &TestEvent| {
        ctx.counter += 1;
        Ok(())
    });

    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = action.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 1);
}

#[test]
fn test_function_action_with_error() {
    let action = FunctionAction::new(|_ctx: &mut TestContext, _event: &TestEvent| {
        Err("Action failed".into())
    });

    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = action.execute(&mut context, &event);
    assert!(result.is_err());
}

#[test]
fn test_assign_action() {
    let action = AssignAction::new("counter", 42);
    
    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = action.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 42);
}

#[test]
fn test_log_action() {
    let action = LogAction::new(LogLevel::Info, "Test message");
    
    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = action.execute(&mut context, &event);
    assert!(result.is_ok());
}

#[test]
fn test_pure_action() {
    let action = PureAction::new(|| {
        println!("Pure action executed");
        Ok(())
    });
    
    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = action.execute(&mut context, &event);
    assert!(result.is_ok());
}

#[test]
fn test_conditional_action() {
    let condition = |ctx: &TestContext, _event: &TestEvent| ctx.counter > 5;
    let action = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter *= 2;
        Ok(())
    });
    
    let conditional = ConditionalAction::new(condition, action);
    
    let mut context = TestContext { counter: 10 };
    let event = TestEvent::Start;
    
    let result = conditional.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 20);
}

#[test]
fn test_conditional_action_false() {
    let condition = |ctx: &TestContext, _event: &TestEvent| ctx.counter > 5;
    let action = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter *= 2;
        Ok(())
    });
    
    let conditional = ConditionalAction::new(condition, action);
    
    let mut context = TestContext { counter: 3 };
    let event = TestEvent::Start;
    
    let result = conditional.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 3); // Should not change
}

#[test]
fn test_sequential_action() {
    let action1 = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter += 1;
        Ok(())
    });
    let action2 = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter *= 2;
        Ok(())
    });
    
    let sequential = SequentialAction::new(vec![action1, action2]);
    
    let mut context = TestContext { counter: 5 };
    let event = TestEvent::Start;
    
    let result = sequential.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 12); // (5 + 1) * 2
}

#[test]
fn test_sequential_action_with_error() {
    let action1 = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter += 1;
        Ok(())
    });
    let action2 = FunctionAction::new(|_ctx: &mut TestContext, _event: &TestEvent| {
        Err("Action failed".into())
    });
    
    let sequential = SequentialAction::new(vec![action1, action2]);
    
    let mut context = TestContext { counter: 5 };
    let event = TestEvent::Start;
    
    let result = sequential.execute(&mut context, &event);
    assert!(result.is_err());
    assert_eq!(context.counter, 6); // First action should still execute
}

#[test]
fn test_parallel_action() {
    let action1 = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter += 1;
        Ok(())
    });
    let action2 = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter += 2;
        Ok(())
    });
    
    let parallel = ParallelAction::new(vec![action1, action2]);
    
    let mut context = TestContext { counter: 5 };
    let event = TestEvent::Start;
    
    let result = parallel.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 8); // 5 + 1 + 2
}

#[test]
fn test_retry_action() {
    let mut attempts = 0;
    let action = FunctionAction::new(move |_ctx: &mut TestContext, _event: &TestEvent| {
        attempts += 1;
        if attempts < 3 {
            Err("Temporary failure".into())
        } else {
            Ok(())
        }
    });
    
    let retry = RetryAction::new(action, 3, std::time::Duration::from_millis(1));
    
    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = retry.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(attempts, 3);
}

#[test]
fn test_retry_action_max_attempts() {
    let action = FunctionAction::new(|_ctx: &mut TestContext, _event: &TestEvent| {
        Err("Always fails".into())
    });
    
    let retry = RetryAction::new(action, 2, std::time::Duration::from_millis(1));
    
    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = retry.execute(&mut context, &event);
    assert!(result.is_err());
}

#[test]
fn test_timer_action() {
    let action = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter += 1;
        Ok(())
    });
    
    let timer = TimerAction::new(action, std::time::Duration::from_millis(10));
    
    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = timer.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 1);
}

#[test]
fn test_metrics_action() {
    let action = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter += 1;
        Ok(())
    });
    
    let metrics = MetricsAction::new("test_action", action);
    
    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = metrics.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 1);
}

#[test]
fn test_composite_action() {
    let actions = vec![
        Box::new(FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
            ctx.counter += 1;
            Ok(())
        })),
        Box::new(FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
            ctx.counter *= 2;
            Ok(())
        })),
    ];
    
    let composite = CompositeAction::new(CompositeLogic::All, actions);
    
    let mut context = TestContext { counter: 5 };
    let event = TestEvent::Start;
    
    let result = composite.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 12); // (5 + 1) * 2
}

#[test]
fn test_action_builder() {
    let builder = ActionBuilder::new();
    let action = builder
        .function(|ctx: &mut TestContext, _event: &TestEvent| {
            ctx.counter += 1;
            Ok(())
        })
        .build();
    
    let mut context = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    let result = action.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 1);
}

#[test]
fn test_action_builder_sequential() {
    let action = ActionBuilder::new()
        .sequential()
        .function(|ctx: &mut TestContext, _event: &TestEvent| {
            ctx.counter += 1;
            Ok(())
        })
        .function(|ctx: &mut TestContext, _event: &TestEvent| {
            ctx.counter *= 2;
            Ok(())
        })
        .build();
    
    let mut context = TestContext { counter: 5 };
    let event = TestEvent::Start;
    
    let result = action.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 12);
}

#[test]
fn test_action_builder_conditional() {
    let action = ActionBuilder::new()
        .conditional(|ctx: &TestContext, _event: &TestEvent| ctx.counter > 5)
        .function(|ctx: &mut TestContext, _event: &TestEvent| {
            ctx.counter *= 2;
            Ok(())
        })
        .build();
    
    let mut context = TestContext { counter: 10 };
    let event = TestEvent::Start;
    
    let result = action.execute(&mut context, &event);
    assert!(result.is_ok());
    assert_eq!(context.counter, 20);
}

#[test]
fn test_action_clone() {
    let action = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
        ctx.counter += 1;
        Ok(())
    });
    
    let cloned = action.clone();
    
    let mut context1 = TestContext { counter: 0 };
    let mut context2 = TestContext { counter: 0 };
    let event = TestEvent::Start;
    
    action.execute(&mut context1, &event).unwrap();
    cloned.execute(&mut context2, &event).unwrap();
    
    assert_eq!(context1.counter, context2.counter);
}
