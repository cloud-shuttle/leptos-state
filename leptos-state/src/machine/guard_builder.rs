//! Guard builder for complex guard combinations

use super::*;

/// Builder for complex guard combinations
pub struct GuardBuilder<C, E> {
    /// Built guards
    pub guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    /// Description of the builder
    pub description: String,
}

impl<C, E> GuardBuilder<C, E> {
    /// Create a new guard builder
    pub fn new() -> Self {
        Self {
            guards: Vec::new(),
            description: "Guard Builder".to_string(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add a function guard
    pub fn function<F>(mut self, func: F) -> Self
    where
        F: Fn(&C, &E) -> bool + Clone + 'static,
    {
        self.guards.push(Box::new(FunctionGuard::new(func)));
        self
    }

    /// Add an always guard
    pub fn always(mut self) -> Self {
        self.guards.push(Box::new(AlwaysGuard::new()));
        self
    }

    /// Add a never guard
    pub fn never(mut self) -> Self {
        self.guards.push(Box::new(NeverGuard::new()));
        self
    }

    /// Add a field equality guard
    pub fn field_equals<T, F>(mut self, field_extractor: F, expected_value: T) -> Self
    where
        F: Fn(&C) -> &T + Clone + 'static,
        T: PartialEq + Clone + 'static,
    {
        self.guards.push(Box::new(FieldEqualityGuard::new(
            field_extractor,
            expected_value,
        )));
        self
    }

    /// Add a range guard
    pub fn in_range<T, F>(mut self, field_extractor: F) -> RangeGuardBuilder<C, E, T, F>
    where
        F: Fn(&C) -> &T + Clone + 'static,
        T: PartialOrd + Clone + 'static,
    {
        RangeGuardBuilder::new(self, field_extractor)
    }

    /// Add a comparison guard
    pub fn compare<T, F1, F2>(
        mut self,
        field1_extractor: F1,
        field2_extractor: F2,
        comparison: ComparisonOp,
    ) -> Self
    where
        F1: Fn(&C) -> &T + Clone + 'static,
        F2: Fn(&C) -> &T + Clone + 'static,
        T: PartialOrd + PartialEq + Clone + 'static,
    {
        self.guards.push(Box::new(ComparisonGuard::new(
            field1_extractor,
            field2_extractor,
            comparison,
        )));
        self
    }

    /// Add a null check guard
    pub fn is_null<F, T>(mut self, field_extractor: F) -> Self
    where
        F: Fn(&C) -> Option<&T> + Clone + 'static,
    {
        self.guards
            .push(Box::new(NullCheckGuard::is_null(field_extractor)));
        self
    }

    /// Add a not-null check guard
    pub fn is_not_null<F, T>(mut self, field_extractor: F) -> Self
    where
        F: Fn(&C) -> Option<&T> + Clone + 'static,
    {
        self.guards
            .push(Box::new(NullCheckGuard::is_not_null(field_extractor)));
        self
    }

    /// Add an event type guard
    pub fn event_type(mut self, expected_type: String) -> Self {
        self.guards
            .push(Box::new(EventTypeGuard::new(expected_type)));
        self
    }

    /// Add a state guard
    pub fn in_state<F>(mut self, expected_state: String, state_getter: F) -> Self
    where
        F: Fn(&C) -> String + Send + Sync + 'static,
    {
        self.guards
            .push(Box::new(StateGuard::new(expected_state, state_getter)));
        self
    }

    /// Add a time guard
    pub fn after_time(mut self, min_time_ms: u64) -> Self {
        self.guards.push(Box::new(TimeGuard::new(min_time_ms)));
        self
    }

    /// Add a counter guard
    pub fn max_count(mut self, max_count: usize) -> Self {
        self.guards.push(Box::new(CounterGuard::new(max_count)));
        self
    }

    /// Add a rate limit guard
    pub fn rate_limit(mut self, max_per_window: usize, window_ms: u64) -> Self {
        self.guards
            .push(Box::new(RateLimitGuard::new(max_per_window, window_ms)));
        self
    }

    /// Add a cooldown guard
    pub fn cooldown(mut self, cooldown_ms: u64) -> Self {
        self.guards.push(Box::new(CooldownGuard::new(cooldown_ms)));
        self
    }

    /// Add a composite guard
    pub fn composite(
        mut self,
        guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
        logic: CompositeLogic,
    ) -> Self {
        self.guards
            .push(Box::new(CompositeGuard::new(guards, logic)));
        self
    }

    /// Create an AND guard from all added guards
    pub fn and(mut self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(AndGuard::new(std::mem::take(&mut self.guards)).with_description(self.description))
    }

    /// Create an OR guard from all added guards
    pub fn or(mut self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(OrGuard::new(std::mem::take(&mut self.guards)).with_description(self.description))
    }

    /// Create a composite guard from all added guards
    pub fn composite_guard(mut self, logic: CompositeLogic) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(
            CompositeGuard::new(std::mem::take(&mut self.guards), logic)
                .with_description(self.description),
        )
    }

    /// Get all built guards
    pub fn get_guards(mut self) -> Vec<Box<dyn GuardEvaluator<C, E>>> {
        std::mem::take(&mut self.guards)
    }
}

/// Range guard builder
pub struct RangeGuardBuilder<C, E, T, F> {
    parent_builder: GuardBuilder<C, E>,
    field_extractor: F,
    min_value: Option<T>,
    max_value: Option<T>,
}

impl<C, E, T, F> RangeGuardBuilder<C, E, T, F>
where
    F: Fn(&C) -> &T + Clone + 'static,
    T: PartialOrd + Clone + 'static,
{
    /// Create a new range guard builder
    pub fn new(parent_builder: GuardBuilder<C, E>, field_extractor: F) -> Self {
        Self {
            parent_builder,
            field_extractor,
            min_value: None,
            max_value: None,
        }
    }

    /// Set minimum value
    pub fn min(mut self, value: T) -> Self {
        self.min_value = Some(value);
        self
    }

    /// Set maximum value
    pub fn max(mut self, value: T) -> Self {
        self.max_value = Some(value);
        self
    }

    /// Build the range guard and add it to the parent builder
    pub fn build(mut self) -> GuardBuilder<C, E> {
        let mut guard = RangeGuard::new(self.field_extractor);

        if let Some(min) = self.min_value {
            guard = guard.min(min);
        }

        if let Some(max) = self.max_value {
            guard = guard.max(max);
        }

        self.parent_builder.guards.push(Box::new(guard));
        self.parent_builder
    }
}

/// Guard evaluation result with detailed information
#[derive(Debug, Clone)]
pub struct GuardEvaluation {
    /// Guard description
    pub guard_description: String,
    /// Evaluation result
    pub result: bool,
    /// Evaluation duration
    pub duration: std::time::Duration,
    /// Error message if evaluation failed
    pub error_message: Option<String>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl GuardEvaluation {
    /// Create a new guard evaluation result
    pub fn new(guard_description: String, result: bool, duration: std::time::Duration) -> Self {
        Self {
            guard_description,
            result,
            duration,
            error_message: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a successful evaluation
    pub fn success(guard_description: String, duration: std::time::Duration) -> Self {
        Self::new(guard_description, true, duration)
    }

    /// Create a failed evaluation
    pub fn failure(guard_description: String, duration: std::time::Duration) -> Self {
        Self::new(guard_description, false, duration)
    }

    /// Add error message
    pub fn with_error(mut self, error: String) -> Self {
        self.error_message = Some(error);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Extension trait for evaluating guards with detailed results
pub trait GuardBatchEvaluator<C, E> {
    /// Evaluate a single guard and return detailed result
    fn evaluate_guard(
        &self,
        guard: &dyn GuardEvaluator<C, E>,
        context: &C,
        event: &E,
    ) -> GuardEvaluation;

    /// Evaluate multiple guards and return their results
    fn evaluate_batch(
        &self,
        guards: &[Box<dyn GuardEvaluator<C, E>>],
        context: &C,
        event: &E,
    ) -> Vec<GuardEvaluation>;
}

impl<C, E> GuardBatchEvaluator<C, E> for Vec<Box<dyn GuardEvaluator<C, E>>> {
    fn evaluate_guard(
        &self,
        guard: &dyn GuardEvaluator<C, E>,
        context: &C,
        event: &E,
    ) -> GuardEvaluation {
        let start = std::time::Instant::now();
        let result = guard.check(context, event);
        let duration = start.elapsed();

        if result {
            GuardEvaluation::success(guard.description(), duration)
        } else {
            GuardEvaluation::failure(guard.description(), duration)
        }
    }

    fn evaluate_batch(
        &self,
        guards: &[Box<dyn GuardEvaluator<C, E>>],
        context: &C,
        event: &E,
    ) -> Vec<GuardEvaluation> {
        guards
            .iter()
            .map(|guard| self.evaluate_guard(guard.as_ref(), context, event))
            .collect()
    }
}

/// Fluent guard creation utilities
pub mod guards {
    use super::*;

    /// Create a function guard
    pub fn function<C, E, F>(func: F) -> Box<dyn GuardEvaluator<C, E>>
    where
        F: Fn(&C, &E) -> bool + Clone + 'static,
    {
        Box::new(FunctionGuard::new(func))
    }

    /// Create an always guard
    pub fn always<C, E>() -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(AlwaysGuard::new())
    }

    /// Create a never guard
    pub fn never<C, E>() -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(NeverGuard::new())
    }

    /// Create an AND guard
    pub fn and<C: 'static, E: 'static>(
        guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    ) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(AndGuard::new(guards))
    }

    /// Create an OR guard
    pub fn or<C: 'static, E: 'static>(
        guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    ) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(OrGuard::new(guards))
    }

    /// Create a NOT guard
    pub fn not<C: 'static, E: 'static>(
        guard: Box<dyn GuardEvaluator<C, E>>,
    ) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(NotGuard::new(guard))
    }

    /// Create a time guard
    pub fn after_time<C, E>(min_time_ms: u64) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(TimeGuard::new(min_time_ms))
    }

    /// Create a counter guard
    pub fn max_count<C, E>(max_count: usize) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(CounterGuard::new(max_count))
    }

    /// Create a composite guard
    pub fn composite<C, E>(
        guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
        logic: CompositeLogic,
    ) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(CompositeGuard::new(guards, logic))
    }
}
