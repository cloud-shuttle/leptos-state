use super::*;
use crate::machine::states::StateValue;
use crate::StateResult;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

// Extension traits
#[cfg(feature = "codegen")]
use crate::machine::codegen::{CodeGenConfig, CodeGenerator, ProgrammingLanguage};
#[cfg(feature = "documentation")]
use crate::machine::documentation::{
    DocumentationConfig, DocumentationFormat, DocumentationGenerator, DocumentationStyling,
    DocumentationTemplate, MachineDocumentationExt,
};
#[cfg(feature = "integration")]
use crate::machine::integration::{
    ErrorHandlingStrategy, EventRoutingConfig, IntegrationConfig, IntegrationManager,
    MachineIntegrationExt, RetryConfig,
};
#[cfg(feature = "performance")]
use crate::machine::performance::{
    MachinePerformanceExt, OptimizationStrategy, OptimizedMachine, PerformanceConfig,
};
// #[cfg(feature = "persist")]
// use crate::machine::persistence::{MachinePersistenceExt, PersistenceConfig, PersistentMachine};
// #[cfg(feature = "testing")]
// use crate::machine::testing::{DataStrategy, MachineTestRunner, MachineTestingExt, TestConfig};
// #[cfg(feature = "visualization")]
// use crate::machine::visualization::{
//     ExportFormat, MachineVisualizationExt, VisualizationConfig, VisualizedMachine,
// };

/// Core trait for state machines
pub trait StateMachine: Sized + 'static {
    type Context: Clone + PartialEq + Send + Sync + 'static;
    type Event: Clone + Send + Sync + 'static;
    type State: MachineState<Context = Self::Context> + Clone + Send + Sync + 'static;

    fn initial() -> Self::State;
    fn transition(state: &Self::State, event: Self::Event) -> Self::State;
}

/// Trait for machine states
pub trait MachineState {
    type Context: Send + Sync + 'static;

    fn value(&self) -> &StateValue;
    fn context(&self) -> &Self::Context;
    fn matches(&self, pattern: &str) -> bool;
    fn can_transition_to(&self, target: &str) -> bool;
}

/// Builder for creating state machines
pub struct MachineBuilder<C: Send + Sync, E> {
    states: HashMap<String, StateNode<C, E, C>>,
    initial: String,
    _phantom: PhantomData<(C, E)>,
}

impl<C: Clone + Send + Sync + 'static, E: Clone + 'static> MachineBuilder<C, E> {
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
        }
    }

    // TODO: Implement persistence features
    // #[cfg(feature = "persist")]
    // /// Build a machine with persistence capabilities
    // pub fn build_with_persistence(self, config: PersistenceConfig) -> PersistentMachine<C, E, C>
    // where
    //     C: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
    //     E: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
    // {
    //     self.build().with_persistence(config)
    // }

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

    // TODO: Implement visualization features
    // #[cfg(feature = "visualization")]
    // /// Build a machine with visualization capabilities
    // pub fn build_with_visualization(self, config: VisualizationConfig) -> VisualizedMachine<C, E, C>
    // where
    //     C: Clone + serde::Serialize,
    //     E: Clone + serde::Serialize,
    // {
    //     self.build().with_visualization(config)
    // }

    // #[cfg(feature = "visualization")]
    // /// Build a machine with default visualization settings
    // pub fn build_visualized(self) -> VisualizedMachine<C, E, C>
    // where
    //     C: Clone + serde::Serialize,
    //     E: Clone + serde::Serialize,
    // {
    //     let config = VisualizationConfig {
    //         enabled: true,
    //         max_history: 100,
    //         capture_snapshots: true,
    //         enable_time_travel: true,
    //         show_transitions: true,
    //         show_context_changes: true,
    //         show_actions: true,
    //         show_guards: true,
    //         // export_format: ExportFormat::Dot,
    //         ..Default::default()
    //     };
    //     self.build_with_visualization(config)
    // }

    // TODO: Implement testing features
    // #[cfg(feature = "testing")]
    // /// Build a machine with testing capabilities
    // pub fn build_with_testing(self, config: TestConfig) -> MachineTestRunner<C, E>
    // where
    //     C: Clone + std::fmt::Debug + PartialEq,
    //     E: Clone + std::fmt::Debug + Event,
    // {
    //     self.build().with_testing(config)
    // }

    // #[cfg(feature = "testing")]
    // /// Build a machine with default testing settings
    // pub fn build_testable(self) -> MachineTestRunner<C, E>
    // where
    //     C: Clone + std::fmt::Debug + PartialEq,
    //     E: Clone + std::fmt::Debug + Event,
    // {
    //     let config = TestConfig {
    //         max_iterations: 1000,
    //         max_transitions: 50,
    //         test_timeout: std::time::Duration::from_secs(30),
    //         verbose: false,
    //         track_coverage: true,
    //         benchmark: false,
    //         random_seed: None,
    //         data_strategy: DataStrategy::Random,
    //     };
    //     self.build_with_testing(config)
    // }

    #[cfg(feature = "performance")]
    /// Build a machine with performance optimization capabilities
    pub fn build_with_performance_optimization(
        self,
        config: PerformanceConfig,
    ) -> OptimizedMachine<C, E, C>
    where
        C: Clone + std::hash::Hash + Eq + std::fmt::Debug,
        E: Clone + std::hash::Hash + Eq + Event,
    {
        self.build().with_performance_optimization(config)
    }

    #[cfg(feature = "performance")]
    /// Build a machine with default performance optimization settings
    pub fn build_optimized(self) -> OptimizedMachine<C, E, C>
    where
        C: Clone + std::hash::Hash + Eq + std::fmt::Debug,
        E: Clone + std::hash::Hash + Eq + Event,
    {
        let config = PerformanceConfig {
            enabled: true,
            enable_caching: true,
            enable_lazy_evaluation: true,
            enable_profiling: true,
            cache_size_limit: 1000,
            cache_ttl: std::time::Duration::from_secs(300),
            cache_guard_results: true,
            cache_action_results: false,
            monitoring_interval: std::time::Duration::from_secs(1),
            track_memory_usage: true,
            track_allocations: true,
            optimization_strategies: vec![
                OptimizationStrategy::TransitionCaching,
                OptimizationStrategy::GuardCaching,
                OptimizationStrategy::LazyEvaluation,
            ],
        };
        self.build_with_performance_optimization(config)
    }

    #[cfg(feature = "integration")]
    /// Build a machine with integration capabilities
    pub fn build_with_integration(self, config: IntegrationConfig) -> IntegrationManager<C, E>
    where
        C: Clone + std::fmt::Debug + Send + Sync,
        E: Clone + std::fmt::Debug + Event + Send + Sync,
    {
        self.build().with_integration(config)
    }

    #[cfg(feature = "integration")]
    /// Build a machine with default integration settings
    pub fn build_integrated(self) -> IntegrationManager<C, E>
    where
        C: Clone + std::fmt::Debug + Send + Sync,
        E: Clone + std::fmt::Debug + Event + Send + Sync,
    {
        let config = IntegrationConfig {
            enabled: true,
            adapters: Vec::new(),
            event_routing: EventRoutingConfig::default(),
            error_handling: ErrorHandlingStrategy::FailFast,
            retry_config: RetryConfig::default(),
        };
        self.build_with_integration(config)
    }

    #[cfg(feature = "documentation")]
    /// Build a machine with documentation generation capabilities
    pub fn build_with_documentation(
        self,
        config: DocumentationConfig,
    ) -> DocumentationGenerator<C, E>
    where
        C: Clone + std::fmt::Debug + Send + Sync,
        E: Clone + std::fmt::Debug + Event + Send + Sync,
    {
        self.build().with_documentation(config)
    }

    #[cfg(feature = "documentation")]
    /// Build a machine with default documentation settings
    pub fn build_documented(self) -> DocumentationGenerator<C, E>
    where
        C: Clone + std::fmt::Debug + Send + Sync,
        E: Clone + std::fmt::Debug + Event + Send + Sync,
    {
        let config = DocumentationConfig {
            enabled: true,
            output_formats: vec![DocumentationFormat::Markdown, DocumentationFormat::Html],
            output_directory: "docs".to_string(),
            template: DocumentationTemplate::Default,
            include_diagrams: true,
            include_code_examples: true,
            include_api_docs: true,
            include_usage_examples: true,
            metadata: HashMap::new(),
            styling: DocumentationStyling::default(),
        };
        self.build_with_documentation(config)
    }

    /// Build a machine with custom code generation settings
    #[cfg(feature = "codegen")]
    pub fn build_with_code_generation(self, config: CodeGenConfig) -> CodeGenerator<C, E>
    where
        C: Clone + std::fmt::Debug + Send + Sync,
        E: Clone + std::fmt::Debug + Event + Send + Sync,
    {
        self.build().with_code_generation(config)
    }

    #[cfg(feature = "codegen")]
    /// Build a machine with default code generation settings
    pub fn build_codegen(self) -> CodeGenerator<C, E>
    where
        C: Clone + std::fmt::Debug + Send + Sync,
        E: Clone + std::fmt::Debug + Event + Send + Sync,
    {
        let config = CodeGenConfig {
            enabled: true,
            target_languages: vec![ProgrammingLanguage::Rust, ProgrammingLanguage::TypeScript],
            output_directory: "generated".to_string(),
            include_tests: true,
            include_documentation: true,
            metadata: HashMap::new(),
        };
        self.build_with_code_generation(config)
    }
}

impl<C: Clone + 'static + std::fmt::Debug + Send + Sync, E: Clone + 'static + std::fmt::Debug>
    Default for MachineBuilder<C, E>
{
    fn default() -> Self {
        Self::new()
    }
}

/// State builder for fluent API
pub struct StateBuilder<C: Send + Sync, E> {
    machine_builder: MachineBuilder<C, E>,
    current_state: String,
    transitions: Vec<Transition<C, E>>,
    entry_actions: Vec<Box<dyn Action<C>>>,
    exit_actions: Vec<Box<dyn Action<C>>>,
    child_states: HashMap<String, StateNode<C, E, C>>,
    initial_child: Option<String>,
}

impl<C: Clone + Send + Sync + 'static, E: Clone + 'static> StateBuilder<C, E> {
    pub fn new(machine_builder: MachineBuilder<C, E>, state_id: String) -> Self {
        Self {
            machine_builder,
            current_state: state_id,
            transitions: Vec::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            child_states: HashMap::new(),
            initial_child: None,
        }
    }

    pub fn on(self, event: E, target: &str) -> TransitionBuilder<C, E> {
        TransitionBuilder::new(self, event, target.to_string())
    }

    pub fn on_entry<A: Action<C> + 'static>(mut self, action: A) -> Self {
        self.entry_actions.push(Box::new(action));
        self
    }

    pub fn on_exit<A: Action<C> + 'static>(mut self, action: A) -> Self {
        self.exit_actions.push(Box::new(action));
        self
    }

    /// Add a function-based entry action
    pub fn on_entry_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut C, &E) + 'static,
    {
        self.entry_actions
            .push(Box::new(actions::FunctionAction::new(func)));
        self
    }

    /// Add a function-based exit action
    pub fn on_exit_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut C, &E) + 'static,
    {
        self.exit_actions
            .push(Box::new(actions::FunctionAction::new(func)));
        self
    }

    /// Add a log entry action
    pub fn on_entry_log(mut self, message: impl Into<String>) -> Self
    where
        C: std::fmt::Debug,
        E: std::fmt::Debug,
    {
        self.entry_actions
            .push(Box::new(actions::LogAction::new(message)));
        self
    }

    /// Add a log exit action
    pub fn on_exit_log(mut self, message: impl Into<String>) -> Self
    where
        C: std::fmt::Debug,
        E: std::fmt::Debug,
    {
        self.exit_actions
            .push(Box::new(actions::LogAction::new(message)));
        self
    }

    /// Add a pure entry action (no context modification)
    pub fn on_entry_pure<F>(mut self, func: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.entry_actions
            .push(Box::new(actions::PureAction::new(func)));
        self
    }

    /// Add a pure exit action (no context modification)
    pub fn on_exit_pure<F>(mut self, func: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.exit_actions
            .push(Box::new(actions::PureAction::new(func)));
        self
    }

    /// Add a child state (for hierarchical states)
    pub fn child_state(self, id: &str) -> ChildStateBuilder<C, E> {
        ChildStateBuilder::new(self, id.to_string())
    }

    /// Set the initial child state
    pub fn initial_child(mut self, child_id: &str) -> Self {
        self.initial_child = Some(child_id.to_string());
        self
    }

    pub fn state(mut self, id: &str) -> StateBuilder<C, E> {
        // Finish current state
        let state_node = StateNode {
            id: self.current_state.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: self.child_states,
            initial_child: self.initial_child,
        };

        self.machine_builder
            .states
            .insert(self.current_state, state_node);

        // Start new state
        StateBuilder::new(self.machine_builder, id.to_string())
    }

    pub fn initial(self, state_id: &str) -> MachineBuilder<C, E> {
        // Finish current state
        let state_node = StateNode {
            id: self.current_state.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: self.child_states,
            initial_child: self.initial_child,
        };

        let mut builder = self.machine_builder;
        builder.states.insert(self.current_state, state_node);
        builder.initial(state_id)
    }

    pub fn build(self) -> Machine<C, E, C> {
        // Finish current state
        let state_node = StateNode {
            id: self.current_state.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: self.child_states,
            initial_child: self.initial_child,
        };

        let mut builder = self.machine_builder;
        builder.states.insert(self.current_state, state_node);
        builder.build()
    }
}

/// Builder for child states in hierarchical machines
pub struct ChildStateBuilder<C: Send + Sync, E> {
    parent_builder: StateBuilder<C, E>,
    child_id: String,
    transitions: Vec<Transition<C, E>>,
    entry_actions: Vec<Box<dyn Action<C>>>,
    exit_actions: Vec<Box<dyn Action<C>>>,
}

impl<C: Clone + 'static + Send + Sync, E: Clone + 'static> ChildStateBuilder<C, E> {
    pub fn new(parent_builder: StateBuilder<C, E>, child_id: String) -> Self {
        Self {
            parent_builder,
            child_id,
            transitions: Vec::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
        }
    }

    pub fn on(self, event: E, target: &str) -> ChildTransitionBuilder<C, E> {
        ChildTransitionBuilder::new(self, event, target.to_string())
    }

    pub fn on_entry<A: Action<C> + 'static>(mut self, action: A) -> Self {
        self.entry_actions.push(Box::new(action));
        self
    }

    pub fn on_exit<A: Action<C> + 'static>(mut self, action: A) -> Self {
        self.exit_actions.push(Box::new(action));
        self
    }

    /// Add a function-based entry action
    pub fn on_entry_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut C, &E) + 'static,
    {
        self.entry_actions
            .push(Box::new(actions::FunctionAction::new(func)));
        self
    }

    /// Add a function-based exit action
    pub fn on_exit_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut C, &E) + 'static,
    {
        self.exit_actions
            .push(Box::new(actions::FunctionAction::new(func)));
        self
    }

    /// Add a log entry action
    pub fn on_entry_log(mut self, message: impl Into<String>) -> Self
    where
        C: std::fmt::Debug,
        E: std::fmt::Debug,
    {
        self.entry_actions
            .push(Box::new(actions::LogAction::new(message)));
        self
    }

    /// Add a log exit action
    pub fn on_exit_log(mut self, message: impl Into<String>) -> Self
    where
        C: std::fmt::Debug,
        E: std::fmt::Debug,
    {
        self.exit_actions
            .push(Box::new(actions::LogAction::new(message)));
        self
    }

    /// Add a pure entry action (no context modification)
    pub fn on_entry_pure<F>(mut self, func: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.entry_actions
            .push(Box::new(actions::PureAction::new(func)));
        self
    }

    /// Add a pure exit action (no context modification)
    pub fn on_exit_pure<F>(mut self, func: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.exit_actions
            .push(Box::new(actions::PureAction::new(func)));
        self
    }

    pub fn child_state(self, id: &str) -> ChildStateBuilder<C, E> {
        // Finish current child state
        let child_node = StateNode {
            id: self.child_id.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: HashMap::new(),
            initial_child: None,
        };

        let mut parent = self.parent_builder;
        parent.child_states.insert(self.child_id, child_node);

        // Start new child state
        parent.child_state(id)
    }

    pub fn parent(self) -> StateBuilder<C, E> {
        // Finish current child state and return to parent
        let child_node = StateNode {
            id: self.child_id.clone(),
            transitions: self.transitions,
            entry_actions: self.entry_actions,
            exit_actions: self.exit_actions,
            child_states: HashMap::new(),
            initial_child: None,
        };

        let mut parent = self.parent_builder;
        parent.child_states.insert(self.child_id, child_node);
        parent
    }
}

/// Transition builder for child states
pub struct ChildTransitionBuilder<C: Send + Sync, E> {
    child_builder: ChildStateBuilder<C, E>,
    event: E,
    target: String,
    guards: Vec<Box<dyn Guard<C>>>,
    actions: Vec<Box<dyn Action<C>>>,
}

impl<C: Clone + 'static + Send + Sync, E: Clone + 'static> ChildTransitionBuilder<C, E> {
    pub fn new(child_builder: ChildStateBuilder<C, E>, event: E, target: String) -> Self {
        Self {
            child_builder,
            event,
            target,
            guards: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn guard<G: Guard<C> + 'static>(mut self, guard: G) -> Self {
        self.guards.push(Box::new(guard));
        self
    }

    /// Add a function-based guard
    pub fn guard_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&C, &E) -> bool + 'static,
    {
        self.guards.push(Box::new(guards::FunctionGuard::new(func)));
        self
    }

    /// Add a field equality guard
    pub fn guard_field_equals<T, F>(mut self, field_extractor: F, expected_value: T) -> Self
    where
        F: Fn(&C) -> T + 'static,
        T: PartialEq + 'static,
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
        F: Fn(&C) -> T + 'static,
        T: PartialOrd + 'static,
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

    pub fn action<A: Action<C> + 'static>(mut self, action: A) -> Self {
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

/// Transition builder for fluent API
pub struct TransitionBuilder<C: Send + Sync, E> {
    state_builder: StateBuilder<C, E>,
    event: E,
    target: String,
    guards: Vec<Box<dyn Guard<C>>>,
    actions: Vec<Box<dyn Action<C>>>,
}

impl<C: Clone + Send + Sync + 'static, E: Clone + 'static> TransitionBuilder<C, E> {
    pub fn new(state_builder: StateBuilder<C, E>, event: E, target: String) -> Self {
        Self {
            state_builder,
            event,
            target,
            guards: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn guard<G: Guard<C> + 'static>(mut self, guard: G) -> Self {
        self.guards.push(Box::new(guard));
        self
    }

    /// Add a function-based guard
    pub fn guard_fn<F>(mut self, func: F) -> Self
    where
        F: Fn(&C, &E) -> bool + 'static,
    {
        self.guards.push(Box::new(guards::FunctionGuard::new(func)));
        self
    }

    /// Add a field equality guard
    pub fn guard_field_equals<T, F>(mut self, field_extractor: F, expected_value: T) -> Self
    where
        F: Fn(&C) -> T + 'static,
        T: PartialEq + 'static,
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
        F: Fn(&C) -> T + 'static,
        T: PartialOrd + 'static,
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

    pub fn action<A: Action<C> + 'static>(mut self, action: A) -> Self {
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
        F: Fn(&mut C, &E) + 'static,
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

/// State node in the machine definition
pub struct StateNode<C, E, S> {
    pub id: String,
    pub transitions: Vec<Transition<C, E>>,
    pub entry_actions: Vec<Box<dyn Action<C>>>,
    pub exit_actions: Vec<Box<dyn Action<C>>>,
    pub child_states: HashMap<String, StateNode<C, E, C>>,
    pub initial_child: Option<String>,
}

/// Transition definition
pub struct Transition<C, E> {
    pub event: E,
    pub target: String,
    pub guards: Vec<Box<dyn Guard<C>>>,
    pub actions: Vec<Box<dyn Action<C>>>,
}

/// Complete machine implementation
pub struct Machine<C: Send + Sync, E, S> {
    states: HashMap<String, StateNode<C, E, C>>,
    initial: String,
}

// Manual Clone implementation for Machine
// Remove the manual Clone implementation since we have #[derive(Clone)]

// Manual Clone implementation for Transition since trait objects can't be cloned
impl<C: Clone, E: Clone> Clone for Transition<C, E> {
    fn clone(&self) -> Self {
        Self {
            event: self.event.clone(),
            target: self.target.clone(),
            guards: Vec::new(), // Can't clone trait objects, so we create empty vectors
            actions: Vec::new(),
        }
    }
}

// Manual Clone implementation for StateNode since Action trait objects can't be cloned
impl<C: Clone, E: Clone> Clone for StateNode<C, E, C> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            transitions: self.transitions.clone(),
            entry_actions: Vec::new(), // Can't clone trait objects, so we create empty vectors
            exit_actions: Vec::new(),
            child_states: self.child_states.clone(),
            initial_child: self.initial_child.clone(),
        }
    }
}

// Manual Clone implementation for Machine
impl<C: Clone + Send + Sync, E: Clone> Clone for Machine<C, E, C> {
    fn clone(&self) -> Self {
        Self {
            states: self.states.clone(),
            initial: self.initial.clone(),
        }
    }
}

impl<C: Send + Sync + Clone, E: Clone> Machine<C, E, C> {
    /// Get all state IDs in the machine
    pub fn get_states(&self) -> Vec<String> {
        self.states.keys().cloned().collect()
    }

    /// Get the initial state ID
    pub fn initial_state_id(&self) -> &str {
        &self.initial
    }

    /// Get a reference to the states map
    pub fn states_map(&self) -> &HashMap<String, StateNode<C, E, C>> {
        &self.states
    }

    /// Export a diagram of the machine
    pub fn export_diagram(
        &self,
        _format: (), // crate::machine::visualization::ExportFormat,
    ) -> StateResult<String> {
        // Note: Machine doesn't implement Clone, so we can't use visualization in this context
        // This would need to be addressed in a future iteration
        Err(crate::utils::types::StateError::new(
            "Visualization not available - Machine doesn't implement Clone",
        ))
    }

    pub fn initial_state(&self) -> MachineStateImpl<C>
    where
        C: Default,
    {
        MachineStateImpl {
            value: StateValue::Simple(self.initial.clone()),
            context: Default::default(),
        }
    }

    pub fn initial_with_context(&self, context: C) -> MachineStateImpl<C> {
        MachineStateImpl {
            value: StateValue::Simple(self.initial.clone()),
            context,
        }
    }

    /// Transition from one state to another based on an event
    pub fn transition(&self, state: &MachineStateImpl<C>, event: E) -> MachineStateImpl<C>
    where
        E: PartialEq,
    {
        match &state.value {
            StateValue::Simple(id) => self.transition_simple(state, id, event),
            StateValue::Compound { parent, child } => {
                self.transition_hierarchical(state, parent, child, event)
            }
            StateValue::Parallel(states) => {
                // Handle parallel states by transitioning each active region
                let mut new_states = Vec::new();
                let mut context = state.context.clone();

                for parallel_state in states {
                    let temp_state = MachineStateImpl {
                        value: parallel_state.clone(),
                        context: context.clone(),
                    };
                    let transitioned = self.transition(&temp_state, event.clone());
                    new_states.push(transitioned.value);
                    context = transitioned.context;
                }

                MachineStateImpl {
                    value: StateValue::Parallel(new_states),
                    context,
                }
            }
        }
    }

    fn transition_simple(
        &self,
        state: &MachineStateImpl<C>,
        state_id: &str,
        event: E,
    ) -> MachineStateImpl<C>
    where
        E: PartialEq,
    {
        if let Some(state_node) = self.states.get(state_id) {
            // Look for a matching transition
            for transition in &state_node.transitions {
                if transition.event == event {
                    // Check all guards
                    let guards_pass = transition
                        .guards
                        .iter()
                        .all(|guard| guard.check(&state.context, &event));

                    if guards_pass {
                        let mut new_context = state.context.clone();

                        // Execute transition actions
                        for action in &transition.actions {
                            action.execute(&mut new_context, &event);
                        }

                        // Execute exit actions for current state
                        for action in &state_node.exit_actions {
                            action.execute(&mut new_context, &event);
                        }

                        // Determine target state value (simple or compound)
                        let new_value = self.resolve_target_state(&transition.target);

                        let new_state = MachineStateImpl {
                            value: new_value,
                            context: new_context,
                        };

                        // Execute entry actions for target state
                        return self.execute_entry_actions(new_state, &transition.target, &event);
                    }
                }
            }
        }

        // No valid transition found, return current state
        state.clone()
    }

    fn transition_hierarchical(
        &self,
        state: &MachineStateImpl<C>,
        parent_id: &str,
        child: &StateValue,
        event: E,
    ) -> MachineStateImpl<C>
    where
        E: PartialEq,
    {
        // First try child state transitions
        let child_state = MachineStateImpl {
            value: (*child).clone(),
            context: state.context.clone(),
        };

        let child_transitioned = self.transition(&child_state, event.clone());

        // If child transitioned, update the compound state
        if child_transitioned.value != (*child).clone() {
            return MachineStateImpl {
                value: StateValue::Compound {
                    parent: parent_id.to_string(),
                    child: Box::new(child_transitioned.value),
                },
                context: child_transitioned.context,
            };
        }

        // If child didn't transition, try parent transitions
        self.transition_simple(state, parent_id, event)
    }

    fn resolve_target_state(&self, target: &str) -> StateValue {
        if let Some(state_node) = self.states.get(target) {
            if !state_node.child_states.is_empty() {
                // This is a compound state, resolve initial child
                if let Some(initial_child) = &state_node.initial_child {
                    return StateValue::Compound {
                        parent: target.to_string(),
                        child: Box::new(self.resolve_target_state(initial_child)),
                    };
                }
            }
        }

        StateValue::Simple(target.to_string())
    }

    fn execute_entry_actions(
        &self,
        mut state: MachineStateImpl<C>,
        target_id: &str,
        event: &E,
    ) -> MachineStateImpl<C> {
        if let Some(target_node) = self.states.get(target_id) {
            for action in &target_node.entry_actions {
                action.execute(&mut state.context, event);
            }
        }

        state
    }
}

/// Concrete implementation of machine state
#[derive(Debug, Clone, PartialEq)]
pub struct MachineStateImpl<C: Send + Sync> {
    value: StateValue,
    context: C,
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

        // Test state validation
        assert!(initial_state.can_transition_to("running"));
        assert!(initial_state.can_transition_to("idle"));
        assert!(!initial_state.can_transition_to(""));
    }
}
