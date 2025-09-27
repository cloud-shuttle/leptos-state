//! State machine implementation inspired by XState
//!
//! Provides finite state machines with hierarchical states, guards, and actions.

pub mod actions;
pub mod action_core;
pub mod action_composite;
pub mod action_control;
pub mod action_builder;
pub mod action_executor;
pub mod builder;
pub mod child_state_builder;
pub mod child_transition_builder;
pub mod codegen;
pub mod core;
pub mod core_types;
pub mod documentation;
pub mod doc_config;
pub mod doc_styling;
pub mod doc_generator;
pub mod doc_data;
pub mod doc_builder;
pub mod events;
pub mod guards;
pub mod history;
pub mod integration;
pub mod machine;
pub mod machine_builder;
pub mod machine_state_impl;
pub mod performance;
pub mod performance_config;
pub mod performance_metrics;
pub mod performance_profiler;
pub mod cache_system;
pub mod lazy_evaluation;
pub mod optimized_machine;
pub mod performance_builder;
// #[cfg(feature = "serialization")]
// pub mod persistence;
pub mod state_builder;
pub mod states;
pub mod testing;
pub mod test_types;
pub mod test_runner;
pub mod test_cases;
pub mod property_testing;
pub mod integration_testing;
pub mod test_data_generation;
pub mod coverage_tracking;
pub mod performance_tracking;
pub mod test_builder;
pub mod test_macros;
pub mod traits;
pub mod transition_builder;
pub mod types;
// #[cfg(feature = "serialization")]
// pub mod visualization;

// Re-export core types from new modular structure
pub use traits::{StateMachine, MachineState};
pub use machine_builder::MachineBuilder;
pub use state_builder::StateBuilder;
pub use child_state_builder::ChildStateBuilder;
pub use child_transition_builder::ChildTransitionBuilder;
pub use transition_builder::TransitionBuilder;
pub use core_types::{StateNode, Transition, Machine};
pub use machine_state_impl::MachineStateImpl;
pub use core::{MachineError, MachineResult, Guard, Action, MachineConfig, MachineHistory};
pub use builder::{MachineBuilderImpl, create_machine_builder};
pub use types::{ContextValue, HistoryEntry, EventRoutingConfig, StateValidationConfig, PerformanceConfig, IntegrationConfig, CompleteMachineConfig};

// Legacy compatibility - re-export from old modules for now
// pub use machine::*;
// Core machine types
pub use actions::{ActionBuilder, ActionExecution, ActionExecutor};
pub use action_core::{Action, FunctionAction, AssignAction, LogAction, LogLevel, PureAction};
pub use action_composite::{ConditionalAction, SequentialAction, ParallelAction, CompositeAction, CompositeLogic};
pub use action_control::{RetryAction, RetryBackoff, TimerAction, MetricsAction, TimeoutAction, CircuitBreakerAction};
pub use action_builder::{ActionBuilder as ActionBuilderCore, ConditionalActionBuilder, ActionExecution as ActionExecutionCore, actions};
pub use action_executor::{EnhancedActionExecutor, ErrorHandlingStrategy, ActionExecutionStats, ExecutionResult, BatchActionExecutor, ActionScheduler, PrioritizedAction};
pub use codegen::{CodeGenConfig, CodeGenerator, GeneratedFile, MachineCodeGenExt};
pub use documentation::{DocumentationConfig, DocumentationGenerator, GeneratedDocument};
pub use doc_config::{DocumentationConfig as DocConfig, DocumentationFormat, DocumentationTemplate, DocumentationStyling, ColorScheme, DocumentationOptions};
pub use doc_styling::{TemplateData, BuiltInTemplates, HtmlStyling, MarkdownStyling};
pub use doc_generator::{DocumentationGenerator as DocGenerator};
pub use doc_data::{GeneratedDocument as DocOutput, TransitionInfo, StateInfo, ActionInfo, GuardInfo, DocumentationData as DocData};
pub use doc_builder::{DocumentationBuilder as DocBuilder, MachineDocumentationExt as DocExt, DocumentationBatch, DocumentationPresets};
pub use events::Event;
pub use guards::{GuardBuilder, GuardEvaluation, GuardEvaluator};
pub use history::{HistoryMachine, HistoryTracker, HistoryType};
pub use integration::{IntegrationAdapter, IntegrationManager};
pub use performance::{OptimizedMachine, PerformanceProfiler};
pub use performance_config::{PerformanceConfig, OptimizationStrategy, OptimizationParameters};
pub use performance_metrics::{PerformanceMetrics, PerformanceBottleneck, BottleneckType, OptimizationSuggestion, PerformanceAnalysis};
pub use performance_profiler::{PerformanceProfiler as PerfProfiler, PerformanceReport};
pub use cache_system::{CacheStats, MemoryTracker, TransitionCache, CacheKey, CachedTransition};
pub use lazy_evaluation::{LazyEvaluator, Lazy, LazyResult, LazyWithMetadata, PerformanceLazy};
pub use optimized_machine::{OptimizedMachine as PerfOptimizedMachine, OptimizationLevel, MachinePerformanceExt};
pub use performance_builder::{PerformanceBuilder, PerformanceOptimizationExt, PerformancePresets};
// #[cfg(feature = "serialization")]
// pub use persistence::{MachinePersistence, PersistenceConfig, PersistentMachine};
pub use testing::{MachineTestRunner, TestCase, TestConfig, TestResult};
pub use test_types::{TestConfig as TestConfigTypes, DataStrategy, TestResult as TestResultTypes, TestCoverage, PerformanceMetrics, TestStep};
pub use test_runner::{MachineTestRunner as TestRunner};
pub use test_cases::{TestCase as TestCaseTypes, TestCaseStep, TestCaseExecutor};
pub use property_testing::{Property, PropertyResult, PropertyTestResult, PropertyTestRunner};
pub use integration_testing::{IntegrationScenario, IntegrationTestResult, IntegrationTestRunner};
pub use test_data_generation::{TestDataGenerator, DefaultTestDataGenerator, MachineTestDataGenerator, DataGenerationStrategy, DataGenerationConfig, TestDataGenerationManager};
pub use coverage_tracking::{CoverageTracker, CoverageStats, CoverageReport};
pub use performance_tracking::{PerformanceTracker, PerformanceReport};
pub use test_builder::{TestBuilder, TestSuiteResult, MachineTestingExt};
// #[cfg(feature = "serialization")]
// pub use visualization::{MachineVisualizer, VisualizationConfig, VisualizedMachine};
