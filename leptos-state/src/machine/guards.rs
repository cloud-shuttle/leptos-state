//! Advanced guard system for state machine transitions
//!
//! This module provides a comprehensive guard system that allows conditional
//! transitions based on context, events, and state conditions.

use crate::machine::events::Event;
use std::marker::PhantomData;
use std::sync::Arc;

/// Trait for transition guards
pub trait Guard<C, E> {
    /// Check if the transition should be allowed
    fn check(&self, context: &C, event: &E) -> bool;

    /// Get a description of what this guard checks
    fn description(&self) -> &str {
        "Unknown guard"
    }
}

/// Function-based guard implementation
#[derive(Clone)]
pub struct FunctionGuard<C, E, F> {
    func: F,
    description: String,
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, F> FunctionGuard<C, E, F>
where
    F: Fn(&C, &E) -> bool,
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            description: "Function guard".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E, F> Guard<C, E> for FunctionGuard<C, E, F>
where
    F: Fn(&C, &E) -> bool,
{
    fn check(&self, context: &C, event: &E) -> bool {
        (self.func)(context, event)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Always true guard (allow all transitions)
pub struct AlwaysGuard;

impl<C, E> Guard<C, E> for AlwaysGuard {
    fn check(&self, _context: &C, _event: &E) -> bool {
        true
    }

    fn description(&self) -> &str {
        "Always allow"
    }
}

/// Never true guard (block all transitions)
pub struct NeverGuard;

impl<C, E> Guard<C, E> for NeverGuard {
    fn check(&self, _context: &C, _event: &E) -> bool {
        false
    }

    fn description(&self) -> &str {
        "Always block"
    }
}

/// And guard - all child guards must pass
pub struct AndGuard<C, E> {
    guards: Vec<Box<dyn Guard<C, E>>>,
}

impl<C, E> AndGuard<C, E> {
    pub fn new(guards: Vec<Box<dyn Guard<C, E>>>) -> Self {
        Self { guards }
    }

    pub fn add_guard(mut self, guard: Box<dyn Guard<C, E>>) -> Self {
        self.guards.push(guard);
        self
    }
}

impl<C, E> Guard<C, E> for AndGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        self.guards.iter().all(|guard| guard.check(context, event))
    }

    fn description(&self) -> &str {
        "All conditions must be true"
    }
}

/// Or guard - any child guard must pass
pub struct OrGuard<C, E> {
    guards: Vec<Box<dyn Guard<C, E>>>,
}

impl<C, E> OrGuard<C, E> {
    pub fn new(guards: Vec<Box<dyn Guard<C, E>>>) -> Self {
        Self { guards }
    }

    pub fn add_guard(mut self, guard: Box<dyn Guard<C, E>>) -> Self {
        self.guards.push(guard);
        self
    }
}

impl<C, E> Guard<C, E> for OrGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        self.guards.iter().any(|guard| guard.check(context, event))
    }

    fn description(&self) -> &str {
        "Any condition must be true"
    }
}

/// Not guard - inverts the result of the child guard
pub struct NotGuard<C, E> {
    guard: Box<dyn Guard<C, E>>,
}

impl<C, E> NotGuard<C, E> {
    pub fn new(guard: Box<dyn Guard<C, E>>) -> Self {
        Self { guard }
    }
}

impl<C, E> Guard<C, E> for NotGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        !self.guard.check(context, event)
    }

    fn description(&self) -> &str {
        "Inverted condition"
    }
}

/// Context field equality guard
pub struct FieldEqualityGuard<C, E, T, F> {
    field_extractor: F,
    expected_value: T,
    field_name: String,
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, T, F> FieldEqualityGuard<C, E, T, F>
where
    F: Fn(&C) -> T,
    T: PartialEq,
{
    pub fn new(field_extractor: F, expected_value: T) -> Self {
        Self {
            field_extractor,
            expected_value,
            field_name: "field".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_field_name(mut self, field_name: impl Into<String>) -> Self {
        self.field_name = field_name.into();
        self
    }
}

impl<C, E, T, F> Guard<C, E> for FieldEqualityGuard<C, E, T, F>
where
    F: Fn(&C) -> T,
    T: PartialEq,
{
    fn check(&self, context: &C, _event: &E) -> bool {
        (self.field_extractor)(context) == self.expected_value
    }

    fn description(&self) -> &str {
        &self.field_name
    }
}

/// Range guard - checks if a numeric field is within a range
pub struct RangeGuard<C, E, T, F> {
    field_extractor: F,
    min: T,
    max: T,
    field_name: String,
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, T, F> RangeGuard<C, E, T, F>
where
    F: Fn(&C) -> T,
    T: PartialOrd,
{
    pub fn new(field_extractor: F, min: T, max: T) -> Self {
        Self {
            field_extractor,
            min,
            max,
            field_name: "field".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_field_name(mut self, field_name: impl Into<String>) -> Self {
        self.field_name = field_name.into();
        self
    }
}

impl<C, E, T, F> Guard<C, E> for RangeGuard<C, E, T, F>
where
    F: Fn(&C) -> T,
    T: PartialOrd,
{
    fn check(&self, context: &C, _event: &E) -> bool {
        let value = (self.field_extractor)(context);
        value >= self.min && value <= self.max
    }

    fn description(&self) -> &str {
        &self.field_name
    }
}

/// Event type guard - checks the event type
pub struct EventTypeGuard<C, E> {
    expected_type: String,
    _phantom: PhantomData<(C, E)>,
}

impl<C, E> EventTypeGuard<C, E> {
    pub fn new(expected_type: impl Into<String>) -> Self {
        Self {
            expected_type: expected_type.into(),
            _phantom: PhantomData,
        }
    }
}

impl<C, E> Guard<C, E> for EventTypeGuard<C, E>
where
    E: Event,
{
    fn check(&self, _context: &C, event: &E) -> bool {
        event.event_type() == self.expected_type
    }

    fn description(&self) -> &str {
        &self.expected_type
    }
}

/// State-based guard - checks if we're in a specific state
pub struct StateGuard<C, E> {
    expected_state: String,
    _phantom: PhantomData<(C, E)>,
}

impl<C, E> StateGuard<C, E> {
    pub fn new(expected_state: impl Into<String>) -> Self {
        Self {
            expected_state: expected_state.into(),
            _phantom: PhantomData,
        }
    }
}

impl<C, E> Guard<C, E> for StateGuard<C, E> {
    fn check(&self, _context: &C, _event: &E) -> bool {
        // This would need access to current state, which requires additional context
        // For now, we'll use a placeholder implementation
        true
    }

    fn description(&self) -> &str {
        &self.expected_state
    }
}

/// Time-based guard - checks if enough time has passed
pub struct TimeGuard<C, E> {
    min_duration: std::time::Duration,
    last_transition: Arc<std::sync::Mutex<Option<std::time::Instant>>>,
    _phantom: PhantomData<(C, E)>,
}

impl<C, E> TimeGuard<C, E> {
    pub fn new(min_duration: std::time::Duration) -> Self {
        Self {
            min_duration,
            last_transition: Arc::new(std::sync::Mutex::new(None)),
            _phantom: PhantomData,
        }
    }

    pub fn from_seconds(seconds: u64) -> Self {
        Self::new(std::time::Duration::from_secs(seconds))
    }

    pub fn from_millis(millis: u64) -> Self {
        Self::new(std::time::Duration::from_millis(millis))
    }
}

impl<C, E> Guard<C, E> for TimeGuard<C, E> {
    fn check(&self, _context: &C, _event: &E) -> bool {
        let now = std::time::Instant::now();
        if let Ok(mut last) = self.last_transition.lock() {
            if let Some(last_time) = *last {
                if now.duration_since(last_time) >= self.min_duration {
                    *last = Some(now);
                    true
                } else {
                    false
                }
            } else {
                *last = Some(now);
                true
            }
        } else {
            false
        }
    }

    fn description(&self) -> &str {
        "Time-based guard"
    }
}

/// Counter guard - limits the number of transitions
pub struct CounterGuard<C, E> {
    max_count: usize,
    current_count: Arc<std::sync::Mutex<usize>>,
    _phantom: PhantomData<(C, E)>,
}

impl<C, E> CounterGuard<C, E> {
    pub fn new(max_count: usize) -> Self {
        Self {
            max_count,
            current_count: Arc::new(std::sync::Mutex::new(0)),
            _phantom: PhantomData,
        }
    }
}

impl<C, E> Guard<C, E> for CounterGuard<C, E> {
    fn check(&self, _context: &C, _event: &E) -> bool {
        if let Ok(mut count) = self.current_count.lock() {
            if *count < self.max_count {
                *count += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn description(&self) -> &str {
        "Counter guard"
    }
}

/// Composite guard that combines multiple guards with custom logic
pub struct CompositeGuard<C, E> {
    guards: Vec<Box<dyn Guard<C, E>>>,
    logic: CompositeLogic,
}

#[derive(Debug, Clone, Copy)]
pub enum CompositeLogic {
    And,
    Or,
    Xor,
    AtLeast(usize),
    AtMost(usize),
}

impl<C, E> CompositeGuard<C, E> {
    pub fn new(logic: CompositeLogic) -> Self {
        Self {
            guards: Vec::new(),
            logic,
        }
    }

    pub fn add_guard(mut self, guard: Box<dyn Guard<C, E>>) -> Self {
        self.guards.push(guard);
        self
    }

    pub fn with_guards(mut self, guards: Vec<Box<dyn Guard<C, E>>>) -> Self {
        self.guards.extend(guards);
        self
    }
}

impl<C, E> Guard<C, E> for CompositeGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        let results: Vec<bool> = self
            .guards
            .iter()
            .map(|guard| guard.check(context, event))
            .collect();

        match self.logic {
            CompositeLogic::And => results.iter().all(|&r| r),
            CompositeLogic::Or => results.iter().any(|&r| r),
            CompositeLogic::Xor => results.iter().filter(|&&r| r).count() == 1,
            CompositeLogic::AtLeast(n) => results.iter().filter(|&&r| r).count() >= n,
            CompositeLogic::AtMost(n) => results.iter().filter(|&&r| r).count() <= n,
        }
    }

    fn description(&self) -> &str {
        match self.logic {
            CompositeLogic::And => "All conditions",
            CompositeLogic::Or => "Any condition",
            CompositeLogic::Xor => "Exactly one condition",
            CompositeLogic::AtLeast(_) => "At least N conditions",
            CompositeLogic::AtMost(_) => "At most N conditions",
        }
    }
}

/// Builder for complex guard combinations
pub struct GuardBuilder<C, E> {
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C: 'static + std::fmt::Debug, E: 'static + Event + std::fmt::Debug> GuardBuilder<C, E> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn always() -> Box<dyn Guard<C, E>> {
        Box::new(AlwaysGuard)
    }

    pub fn never() -> Box<dyn Guard<C, E>> {
        Box::new(NeverGuard)
    }

    pub fn and(guards: Vec<Box<dyn Guard<C, E>>>) -> Box<dyn Guard<C, E>> {
        Box::new(AndGuard::new(guards))
    }

    pub fn or(guards: Vec<Box<dyn Guard<C, E>>>) -> Box<dyn Guard<C, E>> {
        Box::new(OrGuard::new(guards))
    }

    pub fn not(guard: Box<dyn Guard<C, E>>) -> Box<dyn Guard<C, E>> {
        Box::new(NotGuard::new(guard))
    }

    pub fn function<F>(func: F) -> Box<dyn Guard<C, E>>
    where
        F: Fn(&C, &E) -> bool + 'static,
    {
        Box::new(FunctionGuard::new(func))
    }

    pub fn function_with_description<F>(
        func: F,
        description: impl Into<String>,
    ) -> Box<dyn Guard<C, E>>
    where
        F: Fn(&C, &E) -> bool + 'static,
    {
        Box::new(FunctionGuard::new(func).with_description(description))
    }

    pub fn field_equals<T, F>(field_extractor: F, expected_value: T) -> Box<dyn Guard<C, E>>
    where
        F: Fn(&C) -> T + 'static,
        T: PartialEq + 'static,
    {
        Box::new(FieldEqualityGuard::new(field_extractor, expected_value))
    }

    pub fn field_in_range<T, F>(field_extractor: F, min: T, max: T) -> Box<dyn Guard<C, E>>
    where
        F: Fn(&C) -> T + 'static,
        T: PartialOrd + 'static,
    {
        Box::new(RangeGuard::new(field_extractor, min, max))
    }

    pub fn event_type(expected_type: impl Into<String>) -> Box<dyn Guard<C, E>> {
        Box::new(EventTypeGuard::<C, E>::new(expected_type))
    }

    pub fn state(expected_state: impl Into<String>) -> Box<dyn Guard<C, E>> {
        Box::new(StateGuard::<C, E>::new(expected_state))
    }

    pub fn time_limit(duration: std::time::Duration) -> Box<dyn Guard<C, E>> {
        Box::new(TimeGuard::<C, E>::new(duration))
    }

    pub fn time_limit_seconds(seconds: u64) -> Box<dyn Guard<C, E>> {
        Box::new(TimeGuard::<C, E>::from_seconds(seconds))
    }

    pub fn time_limit_millis(millis: u64) -> Box<dyn Guard<C, E>> {
        Box::new(TimeGuard::<C, E>::from_millis(millis))
    }

    pub fn max_transitions(max_count: usize) -> Box<dyn Guard<C, E>> {
        Box::new(CounterGuard::<C, E>::new(max_count))
    }

    pub fn composite(logic: CompositeLogic) -> CompositeGuard<C, E> {
        CompositeGuard::new(logic)
    }
}

impl<C: 'static + std::fmt::Debug, E: 'static + Event + std::fmt::Debug> Default
    for GuardBuilder<C, E>
{
    fn default() -> Self {
        Self::new()
    }
}

/// Guard evaluation result with detailed information
#[derive(Debug, Clone)]
pub struct GuardEvaluation {
    pub passed: bool,
    pub guard_descriptions: Vec<String>,
    pub failed_guards: Vec<String>,
}

impl GuardEvaluation {
    pub fn new() -> Self {
        Self {
            passed: true,
            guard_descriptions: Vec::new(),
            failed_guards: Vec::new(),
        }
    }

    pub fn add_result(&mut self, guard_description: &str, passed: bool) {
        self.guard_descriptions.push(guard_description.to_string());
        if !passed {
            self.failed_guards.push(guard_description.to_string());
            self.passed = false;
        }
    }
}

/// Extension trait for evaluating guards with detailed results
pub trait GuardEvaluator<C, E> {
    fn evaluate_guards(&self, context: &C, event: &E) -> GuardEvaluation;
}

impl<C, E> GuardEvaluator<C, E> for Vec<Box<dyn Guard<C, E>>> {
    fn evaluate_guards(&self, context: &C, event: &E) -> GuardEvaluation {
        let mut evaluation = GuardEvaluation::new();

        for guard in self {
            let passed = guard.check(context, event);
            evaluation.add_result(guard.description(), passed);
        }

        evaluation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::events::Event;

    #[derive(Debug, Clone, PartialEq)]
    struct TestContext {
        count: i32,
        enabled: bool,
        name: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Increment,
        Decrement,
        Enable,
        Disable,
        Custom(String),
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &str {
            match self {
                TestEvent::Increment => "increment",
                TestEvent::Decrement => "decrement",
                TestEvent::Enable => "enable",
                TestEvent::Disable => "disable",
                TestEvent::Custom(name) => name,
            }
        }
    }

    #[test]
    fn function_guard_works() {
        let guard = FunctionGuard::new(|ctx: &TestContext, _: &TestEvent| ctx.count > 0)
            .with_description("Count must be positive");

        let context_pass = TestContext {
            count: 5,
            enabled: true,
            name: "test".to_string(),
        };
        let context_fail = TestContext {
            count: -1,
            enabled: true,
            name: "test".to_string(),
        };

        assert!(guard.check(&context_pass, &TestEvent::Increment));
        assert!(!guard.check(&context_fail, &TestEvent::Increment));
        assert_eq!(guard.description(), "Count must be positive");
    }

    #[test]
    fn always_and_never_guards() {
        let always = AlwaysGuard;
        let never = NeverGuard;
        let context = TestContext {
            count: 0,
            enabled: true,
            name: "test".to_string(),
        };
        let event = TestEvent::Increment;

        assert!(always.check(&context, &event));
        assert!(!never.check(&context, &event));
        assert_eq!(
            <AlwaysGuard as Guard<TestContext, TestEvent>>::description(&always),
            "Always allow"
        );
        assert_eq!(
            <NeverGuard as Guard<TestContext, TestEvent>>::description(&never),
            "Always block"
        );
    }

    #[test]
    fn and_guard_requires_all() {
        let guard1 = Box::new(FunctionGuard::new(|ctx: &TestContext, _| ctx.count > 0));
        let guard2 = Box::new(FunctionGuard::new(|ctx: &TestContext, _| ctx.enabled));

        let and_guard = AndGuard::new(vec![guard1, guard2]);

        let context_both_true = TestContext {
            count: 5,
            enabled: true,
            name: "test".to_string(),
        };
        let context_one_false = TestContext {
            count: 5,
            enabled: false,
            name: "test".to_string(),
        };

        assert!(and_guard.check(&context_both_true, &TestEvent::Increment));
        assert!(!and_guard.check(&context_one_false, &TestEvent::Increment));
    }

    #[test]
    fn or_guard_requires_any() {
        let guard1 = Box::new(FunctionGuard::new(|ctx: &TestContext, _| ctx.count > 10));
        let guard2 = Box::new(FunctionGuard::new(|ctx: &TestContext, _| ctx.enabled));

        let or_guard = OrGuard::new(vec![guard1, guard2]);

        let context_first_true = TestContext {
            count: 15,
            enabled: false,
            name: "test".to_string(),
        };
        let context_second_true = TestContext {
            count: 5,
            enabled: true,
            name: "test".to_string(),
        };
        let context_both_false = TestContext {
            count: 5,
            enabled: false,
            name: "test".to_string(),
        };

        assert!(or_guard.check(&context_first_true, &TestEvent::Increment));
        assert!(or_guard.check(&context_second_true, &TestEvent::Increment));
        assert!(!or_guard.check(&context_both_false, &TestEvent::Increment));
    }

    #[test]
    fn not_guard_inverts_result() {
        let base_guard = Box::new(FunctionGuard::new(|ctx: &TestContext, _| ctx.enabled));
        let not_guard = NotGuard::new(base_guard);

        let context_enabled = TestContext {
            count: 0,
            enabled: true,
            name: "test".to_string(),
        };
        let context_disabled = TestContext {
            count: 0,
            enabled: false,
            name: "test".to_string(),
        };

        assert!(!not_guard.check(&context_enabled, &TestEvent::Increment));
        assert!(not_guard.check(&context_disabled, &TestEvent::Increment));
    }

    #[test]
    fn field_equality_guard_works() {
        let guard =
            FieldEqualityGuard::new(|ctx: &TestContext| ctx.count, 42).with_field_name("count");

        let context_match = TestContext {
            count: 42,
            enabled: true,
            name: "test".to_string(),
        };
        let context_no_match = TestContext {
            count: 0,
            enabled: true,
            name: "test".to_string(),
        };

        assert!(guard.check(&context_match, &TestEvent::Increment));
        assert!(!guard.check(&context_no_match, &TestEvent::Increment));
        assert_eq!(guard.description(), "count");
    }

    #[test]
    fn range_guard_works() {
        let guard =
            RangeGuard::new(|ctx: &TestContext| ctx.count, 0, 10).with_field_name("count_range");

        let context_in_range = TestContext {
            count: 5,
            enabled: true,
            name: "test".to_string(),
        };
        let context_below_range = TestContext {
            count: -1,
            enabled: true,
            name: "test".to_string(),
        };
        let context_above_range = TestContext {
            count: 15,
            enabled: true,
            name: "test".to_string(),
        };

        assert!(guard.check(&context_in_range, &TestEvent::Increment));
        assert!(!guard.check(&context_below_range, &TestEvent::Increment));
        assert!(!guard.check(&context_above_range, &TestEvent::Increment));
        assert_eq!(guard.description(), "count_range");
    }

    #[test]
    fn event_type_guard_works() {
        let guard = EventTypeGuard::new("increment");
        let context = TestContext {
            count: 0,
            enabled: true,
            name: "test".to_string(),
        };

        assert!(guard.check(&context, &TestEvent::Increment));
        assert!(!guard.check(&context, &TestEvent::Decrement));
        assert_eq!(guard.description(), "increment");
    }

    #[test]
    fn time_guard_works() {
        let guard = TimeGuard::from_millis(100);
        let context = TestContext {
            count: 0,
            enabled: true,
            name: "test".to_string(),
        };
        let event = TestEvent::Increment;

        // First call should pass
        assert!(guard.check(&context, &event));

        // Immediate second call should fail
        assert!(!guard.check(&context, &event));

        // Wait and try again
        std::thread::sleep(std::time::Duration::from_millis(150));
        assert!(guard.check(&context, &event));
    }

    #[test]
    fn counter_guard_works() {
        let guard = CounterGuard::new(2);
        let context = TestContext {
            count: 0,
            enabled: true,
            name: "test".to_string(),
        };
        let event = TestEvent::Increment;

        // First two calls should pass
        assert!(guard.check(&context, &event));
        assert!(guard.check(&context, &event));

        // Third call should fail
        assert!(!guard.check(&context, &event));
    }

    #[test]
    fn composite_guard_works() {
        let guard1 = Box::new(FunctionGuard::new(|ctx: &TestContext, _| ctx.count > 0));
        let guard2 = Box::new(FunctionGuard::new(|ctx: &TestContext, _| ctx.enabled));
        let guard3 = Box::new(FunctionGuard::new(|ctx: &TestContext, _| {
            ctx.name == "test"
        }));

        let context = TestContext {
            count: 5,
            enabled: true,
            name: "test".to_string(),
        };
        let event = TestEvent::Increment;

        // Test XOR logic
        let xor_guard = CompositeGuard::new(CompositeLogic::Xor)
            .with_guards(vec![guard1.clone(), guard2.clone()]);

        // Both true, so XOR should fail
        assert!(!xor_guard.check(&context, &event));

        // Test AtLeast logic
        let at_least_guard = CompositeGuard::new(CompositeLogic::AtLeast(2))
            .with_guards(vec![guard1, guard2, guard3]);

        // All three true, so AtLeast(2) should pass
        assert!(at_least_guard.check(&context, &event));
    }

    #[test]
    fn guard_builder_creates_guards() {
        let _always = GuardBuilder::<TestContext, TestEvent>::always();
        let _never = GuardBuilder::<TestContext, TestEvent>::never();
        let _function = GuardBuilder::function(|ctx: &TestContext, _: &TestEvent| ctx.enabled);
        let _field_equals: Box<dyn Guard<TestContext, TestEvent>> =
            GuardBuilder::field_equals(|ctx: &TestContext| ctx.count, 42);
        let _field_range: Box<dyn Guard<TestContext, TestEvent>> =
            GuardBuilder::field_in_range(|ctx: &TestContext| ctx.count, 0, 10);
        let _event_type: Box<dyn Guard<TestContext, TestEvent>> =
            GuardBuilder::event_type("increment");
        let _time_limit: Box<dyn Guard<TestContext, TestEvent>> =
            GuardBuilder::time_limit_seconds(5);
        let _max_transitions: Box<dyn Guard<TestContext, TestEvent>> =
            GuardBuilder::max_transitions(10);
    }

    #[test]
    fn guard_evaluation_works() {
        let guards: Vec<Box<dyn Guard<TestContext, TestEvent>>> = vec![
            Box::new(FunctionGuard::new(|ctx: &TestContext, _| ctx.count > 0)),
            Box::new(FunctionGuard::new(|ctx: &TestContext, _| ctx.enabled)),
        ];

        let context_pass = TestContext {
            count: 5,
            enabled: true,
            name: "test".to_string(),
        };
        let context_fail = TestContext {
            count: 5,
            enabled: false,
            name: "test".to_string(),
        };

        let evaluation_pass = guards.evaluate_guards(&context_pass, &TestEvent::Increment);
        let evaluation_fail = guards.evaluate_guards(&context_fail, &TestEvent::Increment);

        assert!(evaluation_pass.passed);
        assert_eq!(evaluation_pass.failed_guards.len(), 0);

        assert!(!evaluation_fail.passed);
        assert_eq!(evaluation_fail.failed_guards.len(), 1);
    }
}
