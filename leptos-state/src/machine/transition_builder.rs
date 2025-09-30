use super::*;

/// Transition builder for fluent API
pub struct TransitionBuilder<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> {
    state_builder: StateBuilder<C, E>,
    event: E,
    target: String,
    guards: Vec<Box<dyn Guard<C, E>>>,
    actions: Vec<Box<dyn Action<C, E>>>,
}

impl<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> TransitionBuilder<C, E> {
    pub fn new(state_builder: StateBuilder<C, E>, event: E, target: String) -> Self {
        Self {
            state_builder,
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
        F: Fn(&C) -> T + Send + Sync + 'static,
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

    pub fn on(self, event: E, target: &str) -> TransitionBuilder<C, E> {
        let transition = Transition {
            event: self.event,
            target: self.target,
            guards: self.guards,
            actions: self.actions,
        };

        let mut state_builder = self.state_builder;
        state_builder.transitions.push(transition);

        TransitionBuilder::new(state_builder, event, target.to_string())
    }

    pub fn state(self, id: &str) -> StateBuilder<C, E> {
        let transition = Transition {
            event: self.event,
            target: self.target,
            guards: self.guards,
            actions: self.actions,
        };

        let mut state_builder = self.state_builder;
        state_builder.transitions.push(transition);
        state_builder.state(id)
    }

    /// Finish the current transition and set the initial state on the underlying builder
    pub fn initial(self, state_id: &str) -> MachineBuilder<C, E> {
        let transition = Transition {
            event: self.event,
            target: self.target,
            guards: self.guards,
            actions: self.actions,
        };

        let mut state_builder = self.state_builder;
        state_builder.transitions.push(transition);
        state_builder.initial(state_id)
    }

    /// Finish the current transition and add an exit function to the current state
    pub fn on_exit_fn<F>(self, func: F) -> StateBuilder<C, E>
    where
        F: Fn(&mut C, &E) + Clone + Send + Sync + 'static,
    {
        let transition = Transition {
            event: self.event,
            target: self.target,
            guards: self.guards,
            actions: self.actions,
        };

        let mut state_builder = self.state_builder;
        state_builder.transitions.push(transition);
        state_builder.on_exit_fn(func)
    }

    pub fn build(self) -> Machine<C, E, C> {
        let transition = Transition {
            event: self.event,
            target: self.target,
            guards: self.guards,
            actions: self.actions,
        };

        let mut state_builder = self.state_builder;
        state_builder.transitions.push(transition);
        state_builder.build()
    }
}
