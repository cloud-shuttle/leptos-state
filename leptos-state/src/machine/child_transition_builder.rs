use super::*;

/// Transition builder for child states
pub struct ChildTransitionBuilder<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> {
    child_builder: ChildStateBuilder<C, E>,
    event: E,
    target: String,
    guards: Vec<Box<dyn Guard<C, E>>>,
    actions: Vec<Box<dyn Action<C, E>>>,
}

impl<C: Clone + 'static + Send + Sync, E: Clone + Send + Sync + 'static>
    ChildTransitionBuilder<C, E>
{
    pub fn new(child_builder: ChildStateBuilder<C, E>, event: E, target: String) -> Self {
        Self {
            child_builder,
            event,
            target,
            guards: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn guard<G: Guard<C, E> + 'static>(mut self, guard: G) -> Self {
        self.guards.push(Box::new(guard));
        self
    }

    /// Add a function-based guard
    pub fn guard_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&C, &E) -> bool + Send + Sync + 'static,
    {
        self.guards.push(Box::new(guards::FunctionGuard::new(func)));
        self
    }

    /// Add a field equality guard
    pub fn guard_field_equals<T, F>(mut self, field_extractor: F, expected_value: T) -> Self
    where
        F: Fn(&C) -> T + Clone + Send + Sync + 'static,
        T: PartialEq + Clone + Send + Sync + 'static,
    {
        self.guards.push(Box::new(guards::FieldEqualityGuard::new(
            field_extractor,
            expected_value,
        )));
        self
    }

    /// Add a range guard
    pub fn guard_field_range<T, F>(mut self, field_extractor: F, min: T, max: T) -> Self
    where
        F: Fn(&C) -> T + Clone + Send + Sync + 'static,
        T: PartialOrd + Send + Sync + 'static,
    {
        self.guards
            .push(Box::new(guards::RangeGuard::new(field_extractor, min, max)));
        self
    }

    /// Add a time limit guard
    pub fn guard_time_limit(mut self, duration: std::time::Duration) -> Self {
        self.guards.push(Box::new(guards::TimeGuard::new(duration)));
        self
    }

    /// Add a counter guard
    pub fn guard_max_transitions(mut self, max_count: usize) -> Self {
        self.guards
            .push(Box::new(guards::CounterGuard::new(max_count)));
        self
    }

    pub fn action<A: Action<C, E> + 'static>(mut self, action: A) -> Self {
        self.actions.push(Box::new(action));
        self
    }

    pub fn on(self, event: E, target: &str) -> ChildTransitionBuilder<C, E> {
        let transition = Transition {
            event: self.event,
            target: self.target,
            guards: self.guards,
            actions: self.actions,
        };

        let mut child_builder = self.child_builder;
        child_builder.transitions.push(transition);

        ChildTransitionBuilder::new(child_builder, event, target.to_string())
    }

    pub fn parent(self) -> StateBuilder<C, E> {
        let transition = Transition {
            event: self.event,
            target: self.target,
            guards: self.guards,
            actions: self.actions,
        };

        let mut child_builder = self.child_builder;
        child_builder.transitions.push(transition);
        child_builder.parent()
    }
}
