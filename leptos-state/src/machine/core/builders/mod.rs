//! Fluent builders for creating state machines

use super::*;
use crate::machine::action_core::FunctionAction;
use crate::machine::guard_core::FunctionGuard;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Builder for creating state machines
#[derive(Debug)]
pub struct MachineBuilder<C: Send + Sync, E: Send + Sync> {
    states: HashMap<String, StateNode<C, E, C>>,
    initial: String,
    _phantom: PhantomData<(C, E)>,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + Default + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> MachineBuilder<C, E> {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            initial: String::new(),
            _phantom: PhantomData,
        }
    }

    pub fn state(self, id: &str) -> StateBuilder<C, E> {
        StateBuilder::new(self, id.to_string())
    }

    pub fn initial(mut self, state_id: &str) -> Self {
        self.initial = state_id.to_string();
        self
    }

    pub fn build(self) -> Machine<C, E, C> {
        Machine {
            states: self.states,
            initial: self.initial,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Build a machine with persistence capabilities
    #[cfg(feature = "persist")]
    pub fn build_with_persistence(self, config: crate::machine::persistence_core::PersistenceConfig) -> crate::machine::persistence_ext::PersistentMachine<C, E, C>
    where
        C: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
        E: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        use crate::machine::persistence_ext::MachinePersistenceExt;
        self.build().with_persistence(config)
    }

    // #[cfg(feature = "persist")]
    // /// Build a machine with default persistence settings
    // pub fn build_persistent(self) -> PersistentMachine<C, E, C>
    // where
    //     C: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
    //     E: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
    // {
    //     let config = PersistenceConfig {
    //         enabled: true,
    //         storage_key: "leptos_state_machine".to_string(),
    //         auto_save: true,
    //         auto_restore: true,
    //         ..Default::default()
    //     };
    //     self.build_with_persistence(config)
    // }

    /// Build a machine with visualization capabilities
    #[cfg(feature = "visualization")]
    pub fn build_with_visualization(self, config: crate::machine::visualization_core::VisualizationConfig) -> crate::machine::visualization_ext::VisualizedMachine<C, E, C>
    where
        C: Clone + serde::Serialize,
        E: Clone + serde::Serialize,
    {
        use crate::machine::visualization_ext::MachineVisualizationExt;
        self.build().with_visualization(config)
    }

    // #[cfg(feature = "visualization")]
    // /// Build a machine with default visualization settings
    // pub fn build_visualized(self) -> VisualizedMachine<C, E, C>
    // where
    //     C: Clone + serde::Serialize,
    //     E: Clone + serde::Serialize,
    // {
    //     let config = VisualizationConfig {
    //         enabled: true,
    //         format: ExportFormat::Mermaid,
    //         theme: VisualizationTheme::default(),
    //         layout: LayoutConfig::default(),
    //         ..Default::default()
    //     };
    //     self.build_with_visualization(config)
    // }

    /// Build a machine with testing capabilities
    #[cfg(feature = "testing")]
    pub fn build_with_testing(self, config: crate::machine::testing::TestConfig) -> crate::machine::testing::TestMachine<C, E, C> {
        use crate::machine::testing::MachineTestingExt;
        self.build().with_testing(config)
    }

    // #[cfg(feature = "testing")]
    // /// Build a machine with default testing settings
    // pub fn build_testable(self) -> TestMachine<C, E, C> {
    //     let config = TestConfig {
    //         enable_property_testing: true,
    //         enable_coverage: true,
    //         ..Default::default()
    //     };
    //     self.build_with_testing(config)
    // }
}

/// State builder for fluent API
#[derive(Debug)]
pub struct StateBuilder<C: Send + Sync, E: Send + Sync> {
    machine_builder: MachineBuilder<C, E>,
    state_id: String,
    state_node: StateNode<C, E, C>,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + Default + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> StateBuilder<C, E> {
    pub fn new(machine_builder: MachineBuilder<C, E>, state_id: String) -> Self {
        Self {
            machine_builder,
            state_id: state_id.clone(),
            state_node: StateNode {
                id: state_id,
                transitions: Vec::new(),
                entry_actions: Vec::new(),
                exit_actions: Vec::new(),
                child_states: HashMap::new(),
                initial_child: None,
                _phantom: PhantomData,
            },
        }
    }

    pub fn on<E2>(mut self, event: E2, target: &str) -> TransitionBuilder<C, E>
    where
        E2: Into<E>,
    {
        let transition = Transition {
            event: event.into(),
            target: target.to_string(),
            guards: Vec::new(),
            actions: Vec::new(),
        };
        TransitionBuilder::new(self, transition)
    }

    pub fn on_entry_fn<F>(mut self, action_fn: F) -> Self
    where
        F: Fn(&mut C, &E) + Send + Sync + 'static,
    {
        let action = FunctionAction::new(action_fn);
        self.state_node.entry_actions.push(Box::new(action));
        self
    }

    pub fn on_exit_fn<F>(mut self, action_fn: F) -> Self
    where
        F: Fn(&mut C, &E) + Send + Sync + 'static,
    {
        let action = FunctionAction::new(action_fn);
        self.state_node.exit_actions.push(Box::new(action));
        self
    }

    pub fn child(self, child_id: &str) -> ChildStateBuilder<C, E> {
        ChildStateBuilder::new(self, child_id.to_string())
    }

    pub fn build(mut self) -> MachineBuilder<C, E> {
        self.machine_builder.states.insert(self.state_id, self.state_node);
        self.machine_builder
    }
}

/// Builder for child states in hierarchical machines
#[derive(Debug)]
pub struct ChildStateBuilder<C: Send + Sync, E: Send + Sync> {
    state_builder: StateBuilder<C, E>,
    child_id: String,
    child_node: StateNode<C, E, C>,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + Default + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> ChildStateBuilder<C, E> {
    pub fn new(state_builder: StateBuilder<C, E>, child_id: String) -> Self {
        Self {
            state_builder,
            child_id: child_id.clone(),
            child_node: StateNode {
                id: child_id,
                transitions: Vec::new(),
                entry_actions: Vec::new(),
                exit_actions: Vec::new(),
                child_states: HashMap::new(),
                initial_child: None,
                _phantom: PhantomData,
            },
        }
    }

    pub fn on<E2>(mut self, event: E2, target: &str) -> ChildTransitionBuilder<C, E>
    where
        E2: Into<E>,
    {
        let transition = Transition {
            event: event.into(),
            target: target.to_string(),
            guards: Vec::new(),
            actions: Vec::new(),
        };
        ChildTransitionBuilder::new(self, transition)
    }

    pub fn initial(mut self) -> Self {
        self.state_builder.state_node.initial_child = Some(self.child_id.clone());
        self
    }

    pub fn on_entry_fn<F>(mut self, action_fn: F) -> Self
    where
        F: Fn(&mut C, &E) + Send + Sync + 'static,
    {
        let action = FunctionAction::new(action_fn);
        self.child_node.entry_actions.push(Box::new(action));
        self
    }

    pub fn on_exit_fn<F>(mut self, action_fn: F) -> Self
    where
        F: Fn(&mut C, &E) + Send + Sync + 'static,
    {
        let action = FunctionAction::new(action_fn);
        self.child_node.exit_actions.push(Box::new(action));
        self
    }

    pub fn build(mut self) -> StateBuilder<C, E> {
        self.state_builder.state_node.child_states.insert(self.child_id, self.child_node);
        self.state_builder
    }
}

/// Transition builder for child states
#[derive(Debug)]
pub struct ChildTransitionBuilder<C: Send + Sync, E: Send + Sync> {
    child_builder: ChildStateBuilder<C, E>,
    transition: Transition<C, E>,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + Default + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> ChildTransitionBuilder<C, E> {
    pub fn new(child_builder: ChildStateBuilder<C, E>, transition: Transition<C, E>) -> Self {
        Self {
            child_builder,
            transition,
        }
    }

    pub fn guard<F>(mut self, guard_fn: F) -> Self
    where
        F: Fn(&C, &E) -> bool + Send + Sync + 'static,
    {
        let guard = FunctionGuard::new(guard_fn);
        self.transition.guards.push(Box::new(guard));
        self
    }

    pub fn action<F>(mut self, action_fn: F) -> Self
    where
        F: Fn(&mut C, &E) + Send + Sync + 'static,
    {
        let action = FunctionAction::new(action_fn);
        self.transition.actions.push(Box::new(action));
        self
    }

    pub fn build(mut self) -> ChildStateBuilder<C, E> {
        self.child_builder.child_node.transitions.push(self.transition);
        self.child_builder
    }
}

/// Transition builder for fluent API
#[derive(Debug)]
pub struct TransitionBuilder<C: Send + Sync, E: Send + Sync> {
    state_builder: StateBuilder<C, E>,
    transition: Transition<C, E>,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + Default + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> TransitionBuilder<C, E> {
    pub fn new(state_builder: StateBuilder<C, E>, transition: Transition<C, E>) -> Self {
        Self {
            state_builder,
            transition,
        }
    }

    pub fn guard<F>(mut self, guard_fn: F) -> Self
    where
        F: Fn(&C, &E) -> bool + Send + Sync + 'static,
    {
        let guard = FunctionGuard::new(guard_fn);
        self.transition.guards.push(Box::new(guard));
        self
    }

    pub fn action<F>(mut self, action_fn: F) -> Self
    where
        F: Fn(&mut C, &E) + Send + Sync + 'static,
    {
        let action = FunctionAction::new(action_fn);
        self.transition.actions.push(Box::new(action));
        self
    }

    pub fn build(mut self) -> StateBuilder<C, E> {
        self.state_builder.state_node.transitions.push(self.transition);
        self.state_builder
    }
}
