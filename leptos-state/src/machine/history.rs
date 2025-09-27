//! State machine history support

use super::*;
use crate::machine::states::StateValue;
use crate::machine::machine::MachineState;
use std::collections::HashMap;
use std::hash::Hash;

/// Type of history state
#[derive(Debug, Clone, PartialEq)]
pub enum HistoryType {
    /// Shallow history - remember only direct child state
    Shallow,
    /// Deep history - remember the entire state configuration
    Deep,
}

/// History state configuration
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryState {
    pub history_type: HistoryType,
    pub parent_state: String,
    pub default_target: Option<String>,
}

impl HistoryState {
    pub fn shallow(parent_state: impl Into<String>) -> Self {
        Self {
            history_type: HistoryType::Shallow,
            parent_state: parent_state.into(),
            default_target: None,
        }
    }

    pub fn deep(parent_state: impl Into<String>) -> Self {
        Self {
            history_type: HistoryType::Deep,
            parent_state: parent_state.into(),
            default_target: None,
        }
    }

    pub fn with_default(mut self, target: impl Into<String>) -> Self {
        self.default_target = Some(target.into());
        self
    }
}

/// Machine with history tracking capabilities
pub struct HistoryMachine<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    pub base_machine: Machine<C, E, C>,
    pub history_states: HashMap<String, HistoryState>,
    pub history_tracker: HistoryTracker<C>,
}

impl<
        C: Clone
            + PartialEq
            + Default
            + 'static
            + std::fmt::Debug
            + std::marker::Send
            + std::marker::Sync,
        E: Clone + 'static + std::fmt::Debug + Send + Sync + Hash + Eq,
    > HistoryMachine<C, E>
{
    pub fn new(base_machine: Machine<C, E, C>) -> Self {
        Self {
            base_machine,
            history_states: HashMap::new(),
            history_tracker: HistoryTracker::new(),
        }
    }

    pub fn add_history_state(mut self, id: String, history_state: HistoryState) -> Self {
        self.history_states.insert(id, history_state);
        self
    }

    pub fn initial_state(&self) -> MachineStateImpl<C>
    where
        C: Default,
    {
        let state_name = self.base_machine.initial_state();
        let state_value = StateValue::Simple(state_name.to_string());
        let state = MachineStateImpl::new(state_value, C::default());
        self.history_tracker.record_state(&state);
        state
    }

    pub fn initial_with_context(&self, context: C) -> MachineStateImpl<C> {
        let state_name = self.base_machine.initial_state();
        let state_value = StateValue::Simple(state_name.to_string());
        let state = MachineStateImpl::new(state_value, context);
        self.history_tracker.record_state(&state);
        state
    }

    pub fn transition(&self, state: &MachineStateImpl<C>, event: E) -> MachineStateImpl<C>
    where
        E: PartialEq,
    {
        // For now, return the same state since core Machine doesn't have transition method
        // In a real implementation, this would need to be properly implemented
        state.clone()
    }

    /// Transition to a history state
    pub fn transition_to_history(&self, history_id: &str) -> Option<MachineStateImpl<C>> {
        if let Some(history_state) = self.history_states.get(history_id) {
            match history_state.history_type {
                HistoryType::Shallow => self
                    .history_tracker
                    .get_shallow_history(&history_state.parent_state)
                    .or_else(|| self.get_default_state(&history_state.default_target)),
                HistoryType::Deep => self
                    .history_tracker
                    .get_deep_history(&history_state.parent_state)
                    .or_else(|| self.get_default_state(&history_state.default_target)),
            }
        } else {
            None
        }
    }

    fn get_default_state(&self, default_target: &Option<String>) -> Option<MachineStateImpl<C>>
    where
        C: Default,
    {
        if let Some(target) = default_target {
            let state_value = StateValue::Simple(target.clone());
            Some(MachineStateImpl::new(state_value, C::default()))
        } else {
            None
        }
    }

    /// Clear all history for a specific parent state
    pub fn clear_history(&self, parent_state: &str) {
        self.history_tracker.clear_history(parent_state);
    }

    /// Clear all history
    pub fn clear_all_history(&self) {
        self.history_tracker.clear_all();
    }
}

/// Tracks state history for history states
pub struct HistoryTracker<C: Send + Sync + Clone + 'static> {
    shallow_history: std::cell::RefCell<HashMap<String, StateValue>>,
    deep_history: std::cell::RefCell<HashMap<String, MachineStateImpl<C>>>,
}

impl<C: Clone + PartialEq + Default + Send + Sync + 'static> HistoryTracker<C> {
    pub fn new() -> Self {
        Self {
            shallow_history: std::cell::RefCell::new(HashMap::new()),
            deep_history: std::cell::RefCell::new(HashMap::new()),
        }
    }

    /// Record a state for history tracking
    pub fn record_state(&self, state: &MachineStateImpl<C>) {
        match &state.value() {
            StateValue::Simple(id) => {
                // For simple states, record as shallow history
                self.shallow_history
                    .borrow_mut()
                    .insert(id.clone(), state.value().clone());
                self.deep_history
                    .borrow_mut()
                    .insert(id.clone(), state.clone());
            }
            StateValue::Compound { parent, child } => {
                // For compound states, record both parent and full state
                self.shallow_history
                    .borrow_mut()
                    .insert(parent.clone(), child.as_ref().clone());
                self.deep_history
                    .borrow_mut()
                    .insert(parent.clone(), state.clone());

                // Also record child state recursively
                let child_state =
                    MachineStateImpl::new(child.as_ref().clone(), state.context().clone());
                self.record_state(&child_state);
            }
            StateValue::Parallel(states) => {
                // For parallel states, record each region
                for (i, parallel_state) in states.iter().enumerate() {
                    let region_id = format!("parallel_{}", i);
                    let region_state =
                        MachineStateImpl::new(parallel_state.clone(), state.context().clone());

                    self.shallow_history
                        .borrow_mut()
                        .insert(region_id.clone(), parallel_state.clone());
                    self.deep_history
                        .borrow_mut()
                        .insert(region_id, region_state);
                }
            }
        }
    }

    /// Get shallow history for a parent state
    pub fn get_shallow_history(&self, parent_state: &str) -> Option<MachineStateImpl<C>> {
        let shallow_history = self.shallow_history.borrow();
        let deep_history = self.deep_history.borrow();

        if let Some(child_state) = shallow_history.get(parent_state) {
            if let Some(full_state) = deep_history.get(parent_state) {
                return Some(MachineStateImpl::new(
                    StateValue::Compound {
                        parent: parent_state.to_string(),
                        child: Box::new(child_state.clone()),
                    },
                    full_state.context().clone(),
                ));
            }
        }

        None
    }

    /// Get deep history for a parent state
    pub fn get_deep_history(&self, parent_state: &str) -> Option<MachineStateImpl<C>> {
        self.deep_history.borrow().get(parent_state).cloned()
    }

    /// Clear history for a specific parent state
    pub fn clear_history(&self, parent_state: &str) {
        self.shallow_history.borrow_mut().remove(parent_state);
        self.deep_history.borrow_mut().remove(parent_state);
    }

    /// Clear all history
    pub fn clear_all(&self) {
        self.shallow_history.borrow_mut().clear();
        self.deep_history.borrow_mut().clear();
    }

    /// Get all recorded history states
    pub fn get_all_history(&self) -> Vec<String> {
        self.deep_history.borrow().keys().cloned().collect()
    }
}

impl<C: Clone + PartialEq + Default + Send + Sync + 'static> Default for HistoryTracker<C> {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder extension for adding history states
pub trait HistoryMachineBuilder<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    fn with_history_state(self, id: &str, history_state: HistoryState) -> HistoryMachine<C, E>;
}

impl<
        C: Clone
            + PartialEq
            + Default
            + 'static
            + std::fmt::Debug
            + std::marker::Send
            + std::marker::Sync,
        E: Clone + 'static + std::fmt::Debug + Send + Sync + Hash + Eq,
    > HistoryMachineBuilder<C, E> for Machine<C, E, C>
{
    fn with_history_state(self, id: &str, history_state: HistoryState) -> HistoryMachine<C, E> {
        HistoryMachine::new(self).add_history_state(id.to_string(), history_state)
    }
}

/// History transition event wrapper
#[derive(Debug, Clone)]
pub enum HistoryEvent<E> {
    /// Regular event
    Regular(E),
    /// Transition to history state
    ToHistory(String),
}

impl<E> HistoryEvent<E> {
    pub fn regular(event: E) -> Self {
        Self::Regular(event)
    }

    pub fn to_history(history_id: impl Into<String>) -> Self {
        Self::ToHistory(history_id.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::events::*;

    #[derive(Debug, Clone, PartialEq, Default)]
    struct TestContext {
        count: i32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum TestEvent {
        Start,
        Stop,
        Pause,
        Resume,
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &str {
            match self {
                TestEvent::Start => "start",
                TestEvent::Stop => "stop",
                TestEvent::Pause => "pause",
                TestEvent::Resume => "resume",
            }
        }
    }

    fn create_test_machine_with_history() -> HistoryMachine<TestContext, TestEvent> {
        let base_machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
            .on(TestEvent::Start, "active")
            .state("active")
            .child_state("running")
            .on(TestEvent::Pause, "paused")
            .parent()
            .child_state("paused")
            .on(TestEvent::Resume, "running")
            .parent()
            .initial_child("running")
            .on(TestEvent::Stop, "idle")
            .build();

        let history_state = HistoryState::shallow("active").with_default("running");

        HistoryMachine::new(base_machine)
            .add_history_state("active_history".to_string(), history_state)
    }

    #[test]
    fn history_tracker_records_states() {
        let tracker = HistoryTracker::<TestContext>::new();

        let state = MachineStateImpl::new(
            StateValue::Simple("test".to_string()),
            TestContext::default(),
        );

        tracker.record_state(&state);

        let history = tracker.get_shallow_history("test");
        assert!(history.is_some());
    }

    #[test]
    fn shallow_history_restoration() {
        let machine = create_test_machine_with_history();

        // Start the machine and transition to running state
        let initial = machine.initial_state();
        let active = machine.transition(&initial, TestEvent::Start);
        let paused = machine.transition(&active, TestEvent::Pause);

        // Now transition back to idle
        let idle = machine.transition(&paused, TestEvent::Stop);
        assert_eq!(*idle.value(), StateValue::Simple("idle".to_string()));

        // Transition to history - should restore to running state (default)
        if let Some(restored) = machine.transition_to_history("active_history") {
            if let StateValue::Compound { parent, child } = &restored.value() {
                assert_eq!(parent, "active");
                assert_eq!(**child, StateValue::Simple("running".to_string()));
            } else {
                panic!("Expected compound state");
            }
        } else {
            panic!("History restoration failed");
        }
    }

    #[test]
    fn deep_history_with_context() {
        let tracker = HistoryTracker::new();

        let state = MachineStateImpl::new(
            StateValue::Simple("test".to_string()),
            TestContext { count: 42 },
        );

        tracker.record_state(&state);

        let restored = tracker.get_deep_history("test");
        assert!(restored.is_some());
        assert_eq!(restored.unwrap().context().count, 42);
    }

    #[test]
    fn history_clearing() {
        let tracker = HistoryTracker::<TestContext>::new();

        let state = MachineStateImpl::new(
            StateValue::Simple("test".to_string()),
            TestContext::default(),
        );

        tracker.record_state(&state);
        assert!(tracker.get_shallow_history("test").is_some());

        tracker.clear_history("test");
        assert!(tracker.get_shallow_history("test").is_none());
    }
}
