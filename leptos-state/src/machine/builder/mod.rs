use crate::machine::core_errors::{MachineError, MachineResult};
use crate::machine::core_machine::Machine;
use crate::machine::core_state::StateNode;
use crate::machine::core_traits::MachineBuilder;
use crate::machine::types_basic::StateType;
use std::collections::HashMap;

// MachineBuilder trait is now defined in core.rs to avoid conflicts
// This module implements the trait for MachineBuilderImpl

/// Fluent builder for creating state machines
pub struct MachineBuilderImpl<S, E, C>
where
    S: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static + std::hash::Hash + Eq,
    C: Clone + PartialEq + Send + Sync + std::fmt::Debug + 'static,
{
    states: HashMap<String, StateNode<S, E, C>>,
    initial_state: Option<String>,
    current_state: Option<String>,
    _phantom: std::marker::PhantomData<C>,
}

impl<S, E, C> MachineBuilder<C, E, S> for crate::machine::MachineBuilderImpl<S, E, C>
where
    S: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + std::fmt::Debug + 'static + std::hash::Hash + Eq,
    C: Clone + PartialEq + Send + Sync + std::fmt::Debug + Default + 'static,
{
    fn new() -> Self {
        Self {
            states: HashMap::new(),
            initial_state: None,
            current_state: None,
            _phantom: std::marker::PhantomData,
        }
    }

    fn state<Name: Into<String>>(mut self, name: Name) -> Self {
        let state_name = name.into();
        let state = StateNode::new(state_name.clone(), StateType::Atomic, C::default());
        self.states.insert(state_name.clone(), state);
        self.current_state = Some(state_name);
        self
    }

    fn initial<Name: Into<String>>(mut self, state: Name) -> Self {
        self.initial_state = Some(state.into());
        self
    }

    fn transition<E2, S2>(mut self, from: S2, event: E2, to: S2) -> Self
    where
        S2: Into<String> + Clone,
        E2: Into<E>,
    {
        let from_state = from.into();
        let to_state = to.into();

        if let Some(state) = self.states.get_mut(&from_state) {
            state.add_transition(event.into(), to_state);
        }
        self
    }

    fn build_with_context(self, context: C) -> crate::StateResult<Machine<C, E, S>> {
        let initial_state = self.initial_state.ok_or(crate::StateError::StateNotFound(
            "No initial state set".to_string(),
        ))?;

        if !self.states.contains_key(&initial_state) {
            return Err(crate::StateError::StateNotFound(format!(
                "Initial state '{}' not found",
                initial_state
            )));
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
    E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static + std::hash::Hash + Eq,
    C: Clone + PartialEq + Send + Sync + std::fmt::Debug + Default + 'static,
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
    use crate::machine::core::{StateNode, StateType};
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
