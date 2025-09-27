use crate::machine::core::{Machine, StateNode, StateType, MachineError, MachineResult};
use std::collections::HashMap;

/// Main builder trait for constructing state machines
pub trait MachineBuilder {
    type State;
    type Event;
    type Context;

    fn new() -> Self;
    fn state<Name: Into<String>>(self, name: Name) -> Self;
    fn initial<Name: Into<String>>(self, state: Name) -> Self;
    fn transition<E, S>(self, from: S, event: E, to: S) -> Self
    where
        S: Into<String> + Clone,
        E: Into<Self::Event>;
    fn build_with_context(self, context: Self::Context) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>;
    fn build(self) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>
    where
        Self::Context: Default,
    {
        self.build_with_context(Self::Context::default())
    }
}

/// Fluent builder for creating state machines
pub struct MachineBuilderImpl<S, E, C>
where
    S: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static + std::hash::Hash + Eq,
    C: Clone + PartialEq + Send + Sync + 'static,
{
    states: HashMap<String, StateNode<S, E, C>>,
    initial_state: Option<String>,
    current_state: Option<String>,
}

impl<S, E, C> MachineBuilder for MachineBuilderImpl<S, E, C>
where
    S: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static + std::hash::Hash + Eq,
    C: Clone + PartialEq + Send + Sync + 'static,
{
    type State = S;
    type Event = E;
    type Context = C;

    fn new() -> Self {
        Self {
            states: HashMap::new(),
            initial_state: None,
            current_state: None,
        }
    }

    fn state<Name: Into<String>>(mut self, name: Name) -> Self {
        let state_name = name.into();
        let state = StateNode::new(state_name.clone(), StateType::Atomic);
        self.states.insert(state_name.clone(), state);
        self.current_state = Some(state_name);
        self
    }

    fn initial<Name: Into<String>>(mut self, state: Name) -> Self {
        self.initial_state = Some(state.into());
        self
    }

    fn transition<S2>(mut self, from: S2, event: E, to: S2) -> Self
    where
        S2: Into<String> + Clone,
    {
        let from_state = from.into();
        let to_state = to.into();

        if let Some(state) = self.states.get_mut(&from_state) {
            state.add_transition(event, to_state);
        }
        self
    }

    fn build_with_context(self, context: C) -> MachineResult<Machine<S, E, C>> {
        let initial_state = self.initial_state
            .ok_or(MachineError::InvalidState("No initial state set".to_string()))?;

        if !self.states.contains_key(&initial_state) {
            return Err(MachineError::InvalidState(format!("Initial state '{}' not found", initial_state)));
        }

        let mut machine = Machine::new("built_machine".to_string(), context);

        for (_, state) in self.states {
            machine.add_state(state);
        }

        machine.current_state = initial_state;

        Ok(machine)
    }
}

/// Convenience function to create a new machine builder
pub fn create_machine_builder<S, E, C>() -> MachineBuilderImpl<S, E, C>
where
    S: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static + std::hash::Hash + Eq,
    C: Clone + PartialEq + Send + Sync + 'static,
{
    MachineBuilderImpl::new()
}

/// Helper macro for building machines with fluent syntax
#[macro_export]
macro_rules! machine {
    ($builder:expr => {
        $($body:tt)*
    }) => {
        {
            let mut builder = $builder;
            machine!(@internal builder, $($body)*);
            builder.build_with_context(Default::default()).unwrap()
        }
    };

    // Version with explicit context
    ($builder:expr, $context:expr => {
        $($body:tt)*
    }) => {
        {
            let mut builder = $builder;
            machine!(@internal builder, $($body)*);
            builder.build_with_context($context).unwrap()
        }
    };

    (@internal $builder:expr, state $name:literal $($rest:tt)*) => {
        {
            let builder = $builder.state($name);
            machine!(@internal builder, $($rest)*);
        }
    };

    (@internal $builder:expr, initial $name:literal $($rest:tt)*) => {
        {
            let builder = $builder.initial($name);
            machine!(@internal builder, $($rest)*);
        }
    };

    (@internal $builder:expr, transition $from:literal, $event:expr => $to:literal $($rest:tt)*) => {
        {
            let builder = $builder.transition($from, $event, $to);
            machine!(@internal builder, $($rest)*);
        }
    };

    (@internal $builder:expr,) => {
        $builder
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::core::{StateType, StateNode};
    use crate::machine::machine::MachineState;

    #[derive(Clone, Debug, PartialEq)]
    struct TestState {
        value: i32,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum TestEvent {
        Next,
        Previous,
    }

    #[derive(Clone, Debug, PartialEq)]
    struct TestContext {
        counter: i32,
    }

    impl Default for TestContext {
        fn default() -> Self {
            Self { counter: 0 }
        }
    }

    #[test]
    fn builder_creates_machine() {
        let machine = create_machine_builder::<TestState, TestEvent, TestContext>()
            .state("red")
            .state("green")
            .initial("red")
            .transition("red", TestEvent::Next, "green")
            .build_with_context(TestContext::default())
            .unwrap();

        assert_eq!(machine.get_current_state(), "red");
        assert!(machine.can_transition_to("red"));
        assert!(machine.can_transition_to("green"));
    }

    #[test]
    fn builder_fails_without_initial_state() {
        let result = create_machine_builder::<TestState, TestEvent, TestContext>()
            .state("red")
            .build_with_context(TestContext::default());

        assert!(result.is_err());
    }

    #[test]
    fn builder_fails_with_invalid_initial_state() {
        let result = create_machine_builder::<TestState, TestEvent, TestContext>()
            .state("red")
            .initial("blue")
            .build_with_context(TestContext::default());

        assert!(result.is_err());
    }

    #[test]
    fn macro_creates_machine() {
        let machine = machine!(create_machine_builder::<TestState, TestEvent, TestContext>() => {
            state "red"
            state "green"
            initial "red"
            transition "red", TestEvent::Next => "green"
        });

        assert_eq!(machine.get_current_state(), "red");
    }
}
