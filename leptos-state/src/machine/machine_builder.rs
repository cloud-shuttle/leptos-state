use super::*;
use std::collections::HashMap;
use std::marker::PhantomData;

// Extension traits
#[cfg(feature = "codegen")]
use crate::machine::codegen::config::{CodeGenConfig, ProgrammingLanguage};
use crate::machine::codegen::core::CodeGenerator;
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

/// Builder for creating state machines
pub struct MachineBuilderImpl<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> {
    pub states: HashMap<String, StateNode<C, E, C>>,
    pub initial: String,
    _phantom: PhantomData<(C, E)>,
}

impl<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> MachineBuilderImpl<C, E> {
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

    /// Build a machine with testing capabilities
    #[cfg(feature = "testing")]
    pub fn build_with_testing(self, config: crate::machine::testing::TestConfig) -> crate::machine::testing::MachineTestRunner<C, E>
    where
        C: Clone + std::fmt::Debug + PartialEq,
        E: Clone + std::fmt::Debug + crate::machine::Event,
    {
        use crate::machine::testing::MachineTestingExt;
        self.build().with_testing(config)
    }

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

impl<
        C: Clone + 'static + std::fmt::Debug + Send + Sync,
        E: Clone + 'static + std::fmt::Debug + Send + Sync,
    > Default for MachineBuilderImpl<C, E>
{
    fn default() -> Self {
        Self::new()
    }
}

/// Alias for backward compatibility
pub type MachineBuilder<C, E> = MachineBuilderImpl<C, E>;
