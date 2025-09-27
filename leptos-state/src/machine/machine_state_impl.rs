use super::*;
use crate::machine::states::StateValue;

/// Concrete implementation of machine state
#[derive(Debug, Clone, PartialEq)]
pub struct MachineStateImpl<C: Send + Sync> {
    pub value: StateValue,
    pub context: C,
}

impl<C: Send + Sync + 'static> MachineState for MachineStateImpl<C> {
    type Context = C;

    fn value(&self) -> &StateValue {
        &self.value
    }

    fn context(&self) -> &Self::Context {
        &self.context
    }

    fn matches(&self, pattern: &str) -> bool {
        self.value.matches(pattern)
    }

    fn can_transition_to(&self, target: &str) -> bool {
        // Check if the target state exists in the machine
        // This is a simplified implementation - in a full implementation,
        // you would need access to the machine definition to check transitions
        // For now, we'll assume any state can transition to any other state
        // In a real implementation, this would check the machine's transition table
        !target.is_empty()
    }
}

impl<C: Send + Sync> MachineStateImpl<C> {
    /// Create a new machine state with the given value and context
    pub fn new(value: StateValue, context: C) -> Self {
        Self { value, context }
    }

    /// Create a new machine state with the given value and default context
    pub fn with_value(value: StateValue) -> Self
    where
        C: Default,
    {
        Self {
            value,
            context: C::default(),
        }
    }

    /// Create a new machine state with the given context and default value
    pub fn with_context(context: C) -> Self {
        Self {
            value: StateValue::Simple("idle".to_string()),
            context,
        }
    }
}

impl<C: Send + Sync> Default for MachineStateImpl<C>
where
    C: Default,
{
    fn default() -> Self {
        Self {
            value: StateValue::Simple("idle".to_string()),
            context: C::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::events::FunctionAction;
    use crate::machine::guards::FunctionGuard;

    #[derive(Debug, Clone, PartialEq, Default)]
    struct TestContext {
        count: i32,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Start,
        Stop,
        Increment,
    }

    #[test]
    fn machine_builder_creates_simple_machine() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
            .on(TestEvent::Start, "running")
            .state("running")
            .on(TestEvent::Stop, "idle")
            .build();

        assert!(!machine.states.is_empty());
        assert_eq!(machine.initial, "idle");
    }

    #[test]
    fn machine_with_guards() {
        let guard = FunctionGuard::new(|ctx: &TestContext, _| ctx.count > 0);

        let _machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("locked")
            .state("locked")
            .on(TestEvent::Start, "unlocked")
            .guard(guard)
            .state("unlocked")
            .build();
    }

    #[test]
    fn machine_transitions() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
            .on(TestEvent::Start, "running")
            .state("running")
            .on(TestEvent::Stop, "idle")
            .build();

        let initial_state = machine.initial_state();
        assert_eq!(
            initial_state.value(),
            &StateValue::Simple("idle".to_string())
        );

        let running_state = machine.transition(&initial_state, TestEvent::Start);
        assert_eq!(
            running_state.value(),
            &StateValue::Simple("running".to_string())
        );

        let back_to_idle = machine.transition(&running_state, TestEvent::Stop);
        assert_eq!(
            back_to_idle.value(),
            &StateValue::Simple("idle".to_string())
        );
    }

    #[test]
    fn machine_with_actions() {
        let _action = FunctionAction::new(|ctx: &mut TestContext, _: &TestEvent| ctx.count += 1);

        let _machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
            .on_entry_fn(|ctx: &mut TestContext, _| ctx.count += 1)
            .build();
    }

    #[test]
    fn machine_clone_works() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
            .on(TestEvent::Start, "running")
            .state("running")
            .on(TestEvent::Stop, "idle")
            .build();

        // Test that machine can be cloned
        let cloned_machine = machine.clone();
        assert_eq!(machine.initial, cloned_machine.initial);
        assert_eq!(machine.states.len(), cloned_machine.states.len());
    }

    #[test]
    fn machine_state_validation() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
            .on(TestEvent::Start, "running")
            .state("running")
            .on(TestEvent::Stop, "idle")
            .build();

        let initial_state = machine.initial_state();
        assert!(initial_state.matches("idle"));
        assert!(initial_state.can_transition_to("running"));
    }
}
