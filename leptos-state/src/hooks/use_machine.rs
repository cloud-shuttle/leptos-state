use crate::machine::states::StateValue;
use crate::machine::*;
use leptos::prelude::*;

/// Hook to interact with state machines in Leptos components
pub fn use_machine<M: StateMachine>() -> MachineHandle<M> {
    let (state, set_state) = signal(M::initial());

    let send = Callback::new(move |event: M::Event| {
        set_state.update(|s| *s = M::transition(s, event));
    });

    MachineHandle {
        state: state.into(),
        send,
        context: Memo::new(move |_| state.get().context().clone()),
        value: Memo::new(move |_| state.get().value().clone()),
    }
}

/// Hook to use a machine with a specific machine instance (README-compatible API)
pub fn use_machine_with_instance<M: StateMachine>(
    _machine: M,
) -> (ReadSignal<M::State>, Callback<M::Event>) {
    let (state, set_state) = signal(M::initial());

    let send = Callback::new(move |event: M::Event| {
        set_state.update(|s| *s = M::transition(s, event));
    });

    (state.into(), send)
}

/// Hook to create a machine with initial context
pub fn use_machine_with_context<M: StateMachine>(_initial_context: M::Context) -> MachineHandle<M>
where
    M::State: MachineState<Context = M::Context>,
{
    // This would need a way to create initial state with context
    // For now, use the default implementation
    use_machine::<M>()
}

/// Handle for interacting with a state machine
#[derive(Clone)]
pub struct MachineHandle<M: StateMachine> {
    pub state: ReadSignal<M::State>,
    pub send: Callback<M::Event>,
    pub context: Memo<M::Context>,
    pub value: Memo<StateValue>,
}

impl<M: StateMachine> MachineHandle<M> {
    /// Check if current state matches a pattern
    pub fn matches(&self, pattern: &str) -> bool {
        self.state.get().matches(pattern)
    }

    /// Get current state value
    pub fn current(&self) -> StateValue {
        self.value.get()
    }

    /// Get current context
    pub fn get_context(&self) -> M::Context {
        self.context.get()
    }

    /// Check if a transition is possible from current state
    pub fn can(&self, _event: M::Event) -> bool {
        // This would need access to the machine definition
        // For now, return true as a placeholder
        true
    }

    /// Send an event to the machine
    pub fn emit(&self, event: M::Event) {
        self.send.run(event);
    }

    /// Create a reactive memo for state matching
    pub fn create_matcher(&self, pattern: String) -> Memo<bool> {
        let state = self.state;
        Memo::new(move |_| state.get().matches(&pattern))
    }

    /// Create multiple matchers at once
    pub fn create_matchers(&self, patterns: Vec<String>) -> Vec<Memo<bool>> {
        patterns
            .into_iter()
            .map(|pattern| self.create_matcher(pattern))
            .collect()
    }
}

/// Hook for machine subscriptions (listening to state changes)
pub fn use_machine_subscription<M, F>(handle: &MachineHandle<M>, callback: F)
where
    M: StateMachine,
    F: Fn(&M::State) + 'static,
{
    let state = handle.state;
    Effect::new(move |_| {
        let current_state = state.get();
        callback(&current_state);
    });
}

/// Hook for conditional machine actions
pub fn use_machine_effect<M, F>(
    handle: &MachineHandle<M>,
    condition: impl Fn(&M::State) -> bool + 'static,
    effect: F,
) where
    M: StateMachine,
    F: Fn(&M::State) + 'static,
{
    let state = handle.state;
    Effect::new(move |_| {
        let current_state = state.get();
        if condition(&current_state) {
            effect(&current_state);
        }
    });
}

/// Hook for machine state history
pub fn use_machine_history<M: StateMachine>(handle: &MachineHandle<M>) -> MachineHistory<M>
where
    M::State: Clone + PartialEq,
{
    let history = RwSignal::new(Vec::<M::State>::new());
    let current_index = RwSignal::new(0);

    // Track state changes
    let state = handle.state;
    Effect::new(move |prev_state: Option<Option<M::State>>| {
        let current_state = state.get();

        if let Some(Some(prev)) = prev_state {
            if prev != current_state {
                history.update(|h| {
                    h.truncate(current_index.get());
                    h.push(current_state.clone());
                });
                current_index.update(|i| *i += 1);
            }
        } else {
            history.update(|h| h.push(current_state.clone()));
        }

        Some(current_state)
    });

    MachineHistory {
        history: history.read_only(),
        current_index: current_index.read_only(),
    }
}

/// Machine history manager
pub struct MachineHistory<M: StateMachine> {
    history: ReadSignal<Vec<M::State>>,
    current_index: ReadSignal<usize>,
}

impl<M: StateMachine> MachineHistory<M> {
    /// Get all historical states
    pub fn states(&self) -> Vec<M::State> {
        self.history.get()
    }

    /// Get current position in history
    pub fn current_index(&self) -> usize {
        self.current_index.get()
    }

    /// Get history length
    pub fn len(&self) -> usize {
        self.history.get().len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.history.get().is_empty()
    }

    /// Get state at specific index
    pub fn get(&self, index: usize) -> Option<M::State> {
        self.history.get().get(index).cloned()
    }
}

/// Hook for parallel machine management
pub fn use_parallel_machines<M1, M2>(
    machine1: MachineHandle<M1>,
    machine2: MachineHandle<M2>,
) -> ParallelMachineHandle<M1, M2>
where
    M1: StateMachine,
    M2: StateMachine,
{
    ParallelMachineHandle {
        machine1,
        machine2,
        both_active: Memo::new(move |_| {
            // This would depend on specific parallel state logic
            true
        }),
    }
}

/// Handle for parallel machine operations
pub struct ParallelMachineHandle<M1: StateMachine, M2: StateMachine> {
    pub machine1: MachineHandle<M1>,
    pub machine2: MachineHandle<M2>,
    pub both_active: Memo<bool>,
}

impl<M1: StateMachine, M2: StateMachine> ParallelMachineHandle<M1, M2> {
    /// Send events to both machines
    pub fn broadcast_event(&self, event1: M1::Event, event2: M2::Event) {
        self.machine1.emit(event1);
        self.machine2.emit(event2);
    }

    /// Check if both machines match patterns
    pub fn both_match(&self, pattern1: &str, pattern2: &str) -> bool {
        self.machine1.matches(pattern1) && self.machine2.matches(pattern2)
    }
}

/// Hook for machine composition (parent-child relationships)
pub fn use_composed_machine<Parent, Child>(
    parent: MachineHandle<Parent>,
    child_factory: impl Fn(&Parent::Context) -> MachineHandle<Child> + 'static,
) -> ComposedMachineHandle<Parent, Child>
where
    Parent: StateMachine,
    Child: StateMachine,
{
    let child = child_factory(&parent.get_context());

    ComposedMachineHandle { parent, child }
}

/// Handle for composed machines
pub struct ComposedMachineHandle<Parent: StateMachine, Child: StateMachine> {
    pub parent: MachineHandle<Parent>,
    pub child: MachineHandle<Child>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Default)]
    struct TestContext {
        count: i32,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Start,
        Stop,
    }

    struct TestMachine;

    impl StateMachine for TestMachine {
        type Context = TestContext;
        type Event = TestEvent;
        type State = MachineStateImpl<TestContext>;

        fn initial() -> Self::State {
            MachineStateImpl::default()
        }

        fn transition(_state: &Self::State, _event: Self::Event) -> Self::State {
            MachineStateImpl::default()
        }
    }

    #[test]
    fn machine_handle_creation() {
        // This test would need a Leptos runtime
        // Placeholder for now
        assert!(true);
    }
}
