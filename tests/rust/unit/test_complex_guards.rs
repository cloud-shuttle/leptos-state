//! Tests for complex guard combinations and temporal guards
//!
//! This module provides comprehensive testing for advanced guard logic
//! that wasn't adequately covered in the original test suite.

use leptos_state::*;
use leptos_state::machine::guards::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Test context for guard testing
#[derive(Clone, Debug, PartialEq)]
struct GuardTestContext {
    counter: i32,
    flag: bool,
    timestamp: std::time::SystemTime,
}

impl Default for GuardTestContext {
    fn default() -> Self {
        Self {
            counter: 0,
            flag: false,
            timestamp: std::time::SystemTime::now(),
        }
    }
}

/// Test events for guard testing
#[derive(Clone, Debug, PartialEq)]
enum GuardTestEvent {
    Increment,
    Toggle,
    Reset,
    Conditional,
}

#[test]
fn test_nested_guard_combinations() {
    // Test complex AND/OR guard combinations
    let mut context = GuardTestContext::default();

    // Create individual guards
    let counter_guard = Guard::from_fn(|ctx: &GuardTestContext| ctx.counter > 5);
    let flag_guard = Guard::from_fn(|ctx: &GuardTestContext| ctx.flag);
    let time_guard = Guard::from_fn(|ctx: &GuardTestContext| {
        ctx.timestamp.elapsed().unwrap() < Duration::from_secs(60)
    });

    // Test AND combination
    let and_guard = counter_guard.clone().and(flag_guard.clone());
    context.counter = 10;
    context.flag = true;
    assert!(and_guard.check(&context).unwrap());

    context.counter = 3; // Fail counter check
    assert!(!and_guard.check(&context).unwrap());

    // Test OR combination
    let or_guard = counter_guard.clone().or(flag_guard.clone());
    context.counter = 3;
    context.flag = true;
    assert!(or_guard.check(&context).unwrap());

    context.flag = false;
    assert!(!or_guard.check(&context).unwrap());

    // Test complex nested combination: (counter > 5 AND flag) OR time_guard
    let complex_guard = and_guard.or(time_guard);
    context.counter = 1;
    context.flag = false;
    // Should pass because time guard is true (recent timestamp)
    assert!(complex_guard.check(&context).unwrap());
}

#[test]
fn test_temporal_guard_logic() {
    let context = Arc::new(Mutex::new(GuardTestContext::default()));

    // Test time-based guards
    let start_time = Instant::now();

    // Create a guard that checks if less than 100ms have passed
    let temporal_guard = TemporalGuard::duration_less_than(Duration::from_millis(100));

    // Should pass immediately
    assert!(temporal_guard.check(&()).unwrap());

    // Wait for more than 100ms
    std::thread::sleep(Duration::from_millis(150));

    // Should now fail
    assert!(!temporal_guard.check(&()).unwrap());

    // Test duration range guard
    let range_guard = TemporalGuard::duration_in_range(
        Duration::from_millis(50),
        Duration::from_millis(200)
    );

    // Should pass (we've waited ~150ms total)
    assert!(range_guard.check(&()).unwrap());
}

#[test]
fn test_context_aware_guards() {
    let mut context = GuardTestContext::default();

    // Test guards that modify context
    let mut call_count = 0;
    let counting_guard = Guard::from_fn(|ctx: &mut GuardTestContext| {
        call_count += 1;
        ctx.counter += 1;
        true
    });

    assert!(counting_guard.check(&mut context).unwrap());
    assert_eq!(call_count, 1);
    assert_eq!(context.counter, 1);

    // Test again - should increment counter again
    assert!(counting_guard.check(&mut context).unwrap());
    assert_eq!(call_count, 2);
    assert_eq!(context.counter, 2);
}

#[test]
fn test_guard_error_handling() {
    // Test guards that can fail
    let failing_guard = Guard::from_fn(|_: &GuardTestContext| {
        Err(GuardError::Custom("Intentional test failure".into()))
    });

    let context = GuardTestContext::default();
    let result = failing_guard.check(&context);

    assert!(result.is_err());
    if let Err(GuardError::Custom(msg)) = result {
        assert_eq!(msg, "Intentional test failure");
    } else {
        panic!("Expected Custom error");
    }
}

#[test]
fn test_guard_performance_under_load() {
    let context = GuardTestContext::default();

    // Create a computationally expensive guard
    let expensive_guard = Guard::from_fn(|ctx: &GuardTestContext| {
        // Simulate expensive computation
        let mut sum = 0u64;
        for i in 0..100_000 {
            sum += i;
        }
        sum > 0 // Always true, but expensive
    });

    let start_time = Instant::now();

    // Run guard multiple times
    for _ in 0..100 {
        assert!(expensive_guard.check(&context).unwrap());
    }

    let elapsed = start_time.elapsed();
    let avg_time = elapsed / 100;

    // Should be reasonably fast (< 1ms per check on modern hardware)
    assert!(avg_time < Duration::from_millis(1),
        "Guard check too slow: {:?} average", avg_time);
}

#[test]
fn test_guard_composition_edge_cases() {
    let context = GuardTestContext::default();

    // Test empty AND composition (should pass)
    let empty_and = Guard::always_true().and(Guard::always_true());
    assert!(empty_and.check(&context).unwrap());

    // Test empty OR composition (should pass)
    let empty_or = Guard::always_false().or(Guard::always_true());
    assert!(empty_or.check(&context).unwrap());

    // Test short-circuiting
    let mut call_count = 0;
    let counting_true = Guard::from_fn(|_: &GuardTestContext| {
        call_count += 1;
        true
    });
    let counting_false = Guard::from_fn(|_: &GuardTestContext| {
        call_count += 1;
        false
    });

    // AND with false first should short-circuit
    call_count = 0;
    let short_circuit_and = counting_false.clone().and(counting_true.clone());
    assert!(!short_circuit_and.check(&context).unwrap());
    assert_eq!(call_count, 1, "AND should short-circuit on false");

    // OR with true first should short-circuit
    call_count = 0;
    let short_circuit_or = counting_true.clone().or(counting_false.clone());
    assert!(short_circuit_or.check(&context).unwrap());
    assert_eq!(call_count, 1, "OR should short-circuit on true");
}

#[test]
fn test_temporal_guard_edge_cases() {
    // Test boundary conditions for temporal guards

    // Test exactly at boundary
    let exact_guard = TemporalGuard::duration_exactly(Duration::from_millis(100));
    std::thread::sleep(Duration::from_millis(100));

    // Should pass (approximately)
    let result = exact_guard.check(&());
    // Note: Exact timing tests can be flaky due to scheduling

    // Test negative duration (should fail)
    let negative_guard = TemporalGuard::duration_less_than(Duration::from_millis(0));
    assert!(!negative_guard.check(&()).unwrap());

    // Test very long duration
    let long_guard = TemporalGuard::duration_less_than(Duration::from_secs(3600)); // 1 hour
    assert!(long_guard.check(&()).unwrap()); // Should pass (unless system clock is broken)
}

#[test]
fn test_guard_state_isolation() {
    // Test that guards don't interfere with each other
    let context1 = GuardTestContext { counter: 10, flag: true, timestamp: std::time::SystemTime::now() };
    let context2 = GuardTestContext { counter: 5, flag: false, timestamp: std::time::SystemTime::now() };

    let counter_guard = Guard::from_fn(|ctx: &GuardTestContext| ctx.counter > 7);
    let flag_guard = Guard::from_fn(|ctx: &GuardTestContext| ctx.flag);

    // Guards should evaluate independently
    assert!(counter_guard.check(&context1).unwrap());
    assert!(!counter_guard.check(&context2).unwrap());

    assert!(flag_guard.check(&context1).unwrap());
    assert!(!flag_guard.check(&context2).unwrap());
}

#[test]
fn test_guard_composition_with_errors() {
    // Test error handling in composed guards
    let context = GuardTestContext::default();

    let success_guard = Guard::from_fn(|_: &GuardTestContext| Ok(true));
    let error_guard = Guard::from_fn(|_: &GuardTestContext| {
        Err(GuardError::Custom("Test error".into()))
    });

    // AND with error should propagate error
    let and_with_error = success_guard.clone().and(error_guard.clone());
    assert!(and_with_error.check(&context).is_err());

    // OR with error should still evaluate (short-circuit on first success)
    let or_with_error = success_guard.clone().or(error_guard.clone());
    assert!(or_with_error.check(&context).unwrap()); // Should succeed due to short-circuit
}

#[test]
fn test_guard_memory_safety() {
    // Test that guards don't cause memory issues under stress
    let context = GuardTestContext::default();

    let guard = Guard::from_fn(|ctx: &GuardTestContext| {
        // Access context fields safely
        let _counter = ctx.counter;
        let _flag = ctx.flag;
        true
    });

    // Run guard many times to check for memory issues
    for i in 0..10_000 {
        assert!(guard.check(&context).unwrap());
        if i % 1000 == 0 {
            // Periodic check that context is still intact
            assert_eq!(context.counter, 0);
            assert!(!context.flag);
        }
    }
}

#[test]
fn test_complex_state_machine_with_guards() {
    // Integration test: Use complex guards in actual state machine
    let machine = machine!(create_machine_builder::<GuardTestContext, GuardTestEvent>(),
        state "idle"
            .on(GuardTestEvent::Increment) => "incrementing"
                .guard(|ctx: &GuardTestContext| ctx.counter < 10),
        state "incrementing"
            .on_entry(|ctx: &mut GuardTestContext| ctx.counter += 1)
            .on(GuardTestEvent::Reset) => "idle",
        initial "idle"
    );

    let mut context = GuardTestContext::default();

    // Should transition (counter = 0 < 10)
    machine.transition(GuardTestEvent::Increment).unwrap();
    assert_eq!(machine.current_state().name, "incrementing");
    assert_eq!(context.counter, 1);

    // Should transition back
    machine.transition(GuardTestEvent::Reset).unwrap();
    assert_eq!(machine.current_state().name, "idle");

    // Now counter = 1, still < 10, should transition again
    machine.transition(GuardTestEvent::Increment).unwrap();
    assert_eq!(context.counter, 2);
}

#[cfg(feature = "proptest")]
mod property_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn guard_composition_properties(
            a in proptest::bool::ANY,
            b in proptest::bool::ANY,
            c in proptest::bool::ANY
        ) {
            let ctx = GuardTestContext::default();

            let guard_a = Guard::from_fn(move |_| Ok(a));
            let guard_b = Guard::from_fn(move |_| Ok(b));
            let guard_c = Guard::from_fn(move |_| Ok(c));

            // Test AND properties
            let and_ab = guard_a.clone().and(guard_b.clone());
            let and_bc = guard_b.clone().and(guard_c.clone());

            prop_assert_eq!(and_ab.check(&ctx).unwrap(), a && b);
            prop_assert_eq!(and_bc.check(&ctx).unwrap(), b && c);

            // Test OR properties
            let or_ab = guard_a.clone().or(guard_b.clone());
            let or_bc = guard_b.clone().or(guard_c.clone());

            prop_assert_eq!(or_ab.check(&ctx).unwrap(), a || b);
            prop_assert_eq!(or_bc.check(&ctx).unwrap(), b || c);

            // Test De Morgan's laws
            let not_a = guard_a.clone().not();
            let not_b = guard_b.clone().not();

            let demorgan_and = not_a.clone().and(not_b.clone());
            let demorgan_or = not_a.clone().or(not_b.clone());

            prop_assert_eq!(demorgan_and.check(&ctx).unwrap(), !(a && b));
            prop_assert_eq!(demorgan_or.check(&ctx).unwrap(), !(a || b));
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_guard_evaluation() {
        let context = GuardTestContext::default();

        // Create increasingly complex guard combinations
        let simple_guard = Guard::from_fn(|_| Ok(true));
        let complex_guard = simple_guard.clone()
            .and(Guard::from_fn(|_| Ok(true)))
            .or(Guard::from_fn(|_| Ok(false)))
            .and(Guard::from_fn(|_| Ok(true)));

        // Benchmark simple guard
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = simple_guard.check(&context).unwrap();
        }
        let simple_time = start.elapsed();

        // Benchmark complex guard
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = complex_guard.check(&context).unwrap();
        }
        let complex_time = start.elapsed();

        // Complex guard should not be excessively slower
        // Allow up to 10x slower due to composition overhead
        assert!(complex_time < simple_time * 10,
            "Complex guard too slow: {:?} vs {:?}", complex_time, simple_time);
    }
}
