use std::hash::Hash;

use super::core_machine::Machine;

/// Core trait for state machines
pub trait StateMachine: Sized + 'static {
    type Context: Clone + PartialEq + Send + Sync + 'static;
    type Event: Clone + Send + Sync + 'static;
    type State: MachineState<Context = Self::Context> + Clone + Send + Sync + 'static;

    fn initial() -> Self::State;
    fn transition(state: &Self::State, event: Self::Event) -> Self::State;
}

/// Main builder trait for constructing state machines
pub trait MachineBuilder<C, E, S> {
    fn new() -> Self;
    fn state<Name: Into<String>>(self, name: Name) -> Self;
    fn initial<Name: Into<String>>(self, state: Name) -> Self;
    fn transition<E2, S2>(self, from: S2, event: E2, to: S2) -> Self
    where
        S2: Into<String> + Clone,
        E2: Into<E>;
    fn build_with_context(self, context: C) -> crate::StateResult<Machine<C, E, S>>;
    fn build(self) -> crate::StateResult<Machine<C, E, S>>
    where
        C: Default,
        Self: Sized,
    {
        self.build_with_context(C::default())
    }
}

/// Trait for machine states
pub trait MachineState {
    type Context: Send + Sync + 'static;

    fn value(&self) -> &crate::machine::types_basic::StateValue;
    fn context(&self) -> &Self::Context;
    fn matches(&self, pattern: &str) -> bool;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any;
}
