//! State machine implementation inspired by XState
//!
//! Provides finite state machines with hierarchical states, guards, and actions.

pub mod action_builder;
pub mod action_composite;
pub mod action_control;
pub mod action_core;
pub mod action_executor;
pub mod actions;
pub mod builder;
pub mod cache_system;
pub mod child_state_builder;
pub mod child_transition_builder;
pub mod codegen;
pub mod codegen_builder;
// Code generation config is now in codegen::config
// Code generation core is now in codegen::core
pub mod codegen_ext;
// Code generation types are now in codegen::types
pub mod core;
pub mod core_types;
pub mod doc_builder;
pub mod doc_config;
pub mod doc_data;
pub mod doc_generator;
pub mod doc_styling;
pub mod documentation;
pub mod events;
pub mod guard_builder;
pub mod guard_composite;
pub mod guard_context;
pub mod guard_core;
pub mod guard_logical;
pub mod guard_state;
pub mod guard_temporal;
pub mod guards;
pub mod history;
pub mod history_builder;
pub mod history_core;
pub mod history_machine;
pub mod history_tracker;
pub mod integration;
pub mod integration_adapters;
// Integration config is now in integration::config
// Integration core is now in integration::core
// Integration events are now in integration::events
pub mod integration_ext;
// Integration metrics are now in integration::metrics
pub mod lazy_evaluation;
pub mod machine_builder;
pub mod machine_state_impl;
pub mod optimized_machine;
pub mod performance;
pub mod performance_builder;
pub mod performance_config;
pub mod performance_metrics;
pub mod performance_profiler;
// #[cfg(feature = "serialization")]
pub mod persistence;
pub mod core_actions;
pub mod core_errors;
pub mod core_guards;
pub mod core_machine;
pub mod core_macros;
pub mod core_state;
pub mod core_traits;
pub mod coverage_tracking;
pub mod integration_testing;
pub mod performance_tracking;
pub mod persistence_core;
// Persistence extensions are now in persistence::ext
// Persistence manager is now in persistence::manager
// Persistence metadata is now in persistence::metadata
// Serialization is now in persistence::serialization
// Storage backends are now in persistence::storage
pub mod property_testing;
pub mod state_builder;
pub mod states;
pub mod test_builder;
pub mod test_cases;
pub mod test_data_generation;
pub mod test_macros;
pub mod test_runner;
pub mod test_types;
pub mod testing;
pub mod traits;
pub mod transition_builder;
pub mod types;
pub mod types_basic;
pub mod types_config;
pub mod types_context;
pub mod types_history;
// #[cfg(feature = "serialization")]
// pub mod visualization;
pub mod visualization_config;
pub mod visualization_core;
pub mod visualization_data;
pub mod visualization_debug;
pub mod visualization_events;
pub mod visualization_ext;
pub mod visualization_monitor;

// Re-export core types from new modular structure
pub use builder::{create_machine_builder, MachineBuilderImpl};
pub use child_state_builder::ChildStateBuilder;
pub use child_transition_builder::ChildTransitionBuilder;
pub use core_actions::Action;
pub use core_errors::{MachineError, MachineResult};
pub use core_guards::Guard;
// Use new core module structure
pub use core::{Machine, StateNode, Transition, MachineBuilder, MachineStateImpl};
pub use state_builder::StateBuilder;
pub use traits::{MachineState, StateMachine};
pub use transition_builder::TransitionBuilder;
pub use types::{
    CompleteMachineConfig, ContextValue, EventRoutingConfig, HistoryEntry, IntegrationConfig,
    PerformanceConfig, StateValidationConfig,
};
pub use types_config::MachineConfig;
pub use types_history::MachineHistory;

// Legacy compatibility - re-export from old modules for now
// pub use machine::*;
// Core machine types
pub use action_builder::{
    actions as action_utils, ActionBuilder as ActionBuilderCore,
    ActionExecution as ActionExecutionCore, ConditionalActionBuilder,
};
pub use action_composite::{
    CompositeAction, CompositeLogic, ConditionalAction, ParallelAction, SequentialAction,
};
pub use action_control::{
    CircuitBreakerAction, MetricsAction, RetryAction, RetryBackoff, TimeoutAction, TimerAction,
};
pub use action_core::{AssignAction, FunctionAction, LogAction, LogLevel, PureAction};
pub use action_executor::{
    ActionExecutionStats, ActionScheduler, BatchActionExecutor, EnhancedActionExecutor,
    ErrorHandlingStrategy, ExecutionResult, PrioritizedAction,
};
pub use actions::{ActionBuilder, ActionExecution, ActionExecutor};
pub use cache_system::{CacheKey, CacheStats, CachedTransition, MemoryTracker, TransitionCache};
pub use codegen::config::CodeGenConfig;
pub use codegen::core::CodeGenerator;
pub use codegen_ext::MachineCodeGenExt;
pub use codegen::types::GeneratedFile;
pub use codegen_builder::{presets as codegen_presets, CodeGenBuilder as BuilderCodeGenBuilder};
pub use codegen::config::{CodeGenOptions, CodeTemplates, IndentationStyle, ProgrammingLanguage};
pub use codegen::core::{CodeGenerator as CoreCodeGenerator, GenerationStats};
pub use codegen_ext::{
    CodeGenPipeline, CodeGenStep, MachineBuilderCodeGenExt,
    MachineCodeGenExt as ExtMachineCodeGenExt, PipelineConfig, PipelineResult,
};
pub use codegen::types::{
    ActionGenInfo, ActionType, CodeGenContext, EventGenInfo, GeneratedFile as TypesGeneratedFile,
    GuardGenInfo, GuardType, MachineGenInfo, MachineType, StateGenInfo,
    TransitionInfo as TypesTransitionInfo,
};
pub use doc_builder::{
    DocumentationBatch, DocumentationBuilder as DocBuilder, DocumentationPresets,
    MachineDocumentationExt as DocExt,
};
pub use doc_config::{
    ColorScheme, DocumentationConfig as DocConfig, DocumentationFormat, DocumentationOptions,
    DocumentationStyling, DocumentationTemplate,
};
pub use doc_data::{
    ActionInfo, DocumentationData as DocData, GeneratedDocument as DocOutput, GuardInfo, StateInfo,
    TransitionInfo,
};
pub use doc_generator::DocumentationGenerator as DocGenerator;
pub use doc_styling::{BuiltInTemplates, HtmlStyling, MarkdownStyling, TemplateData};
pub use documentation::{DocumentationConfig, DocumentationGenerator, GeneratedDocument};
pub use events::Event;
pub use guard_builder::{
    guards as guard_utils, GuardBuilder as GuardBuilderCore,
    GuardEvaluation as GuardEvaluationCore, RangeGuardBuilder,
};
pub use guard_composite::{
    CompositeGuard, CompositeLogic as GuardCompositeLogic, ConditionalCompositeGuard,
    SequentialGuard, WeightedCompositeGuard,
};
pub use guard_context::{
    ComparisonGuard, ComparisonOp, FieldEqualityGuard, NullCheckGuard, RangeGuard,
};
pub use guard_core::{
    AlwaysGuard, FunctionGuard, GuardBatchEvaluator as GuardBatchEvaluatorCore,
    GuardEvaluator as GuardEvaluatorCore, NeverGuard,
};
pub use guard_logical::{AndGuard, MajorityGuard, NotGuard, OrGuard, XorGuard};
pub use guard_state::{
    ContextStateGuard, EventDataGuard, EventTypeGuard, StateGuard, StateTransitionGuard,
};
pub use guard_temporal::{CooldownGuard, CounterGuard, RateLimitGuard, TimeGuard};
pub use guards::{GuardBuilder, GuardEvaluation, GuardEvaluator};
pub use history::{HistoryMachine, HistoryTracker, HistoryType};
pub use history_builder::{factory, handlers, history_builder as history_builder_utils, utils};
pub use history_core::{
    HistoryConfig, HistoryEntry as HistoryEntryCore, HistoryEvent, HistoryState, HistoryStats,
};
pub use history_machine::{HistoryMachineBuilder, MachineHistoryExt};
pub use history_tracker::{HistoryIterator, HistorySnapshot};
pub use integration::core::{IntegrationAdapter, IntegrationManager};
pub use integration_adapters::{
    DatabaseAdapter, FileFormat, FileSystemAdapter, HttpApiAdapter, MessageQueueAdapter,
    WebSocketAdapter,
};
pub use integration::config::{
    ConnectionConfig, Credentials, EventPattern,
    EventRoutingConfig as IntegrationEventRoutingConfig, EventTransformation,
    IntegrationConfig as IntegrationConfigCore, PoolConfig, RetryConfig, RoutingRule,
};
pub use integration::core::{AdapterType, HealthStatus, IntegrationAdapterTrait};
pub use integration::events::{
    ErrorAction, ErrorHandlingStrategy as IntegrationErrorHandlingStrategy, EventBatch,
    EventFilter, EventPriority, IntegrationError, IntegrationErrorType, IntegrationEvent,
};
pub use integration_ext::{integrations, presets};
pub use integration::metrics::{
    AdapterMetrics, MetricsSummary, Percentiles, PerformanceReport,
    ResourceUsage, ThroughputMetrics,
};
pub use lazy_evaluation::{Lazy, LazyEvaluator, LazyResult, LazyWithMetadata, PerformanceLazy};
pub use optimized_machine::{
    MachinePerformanceExt, OptimizationLevel, OptimizedMachine as PerfOptimizedMachine,
};
pub use performance::{OptimizedMachine, PerformanceProfiler};
pub use performance_builder::{PerformanceBuilder, PerformanceOptimizationExt, PerformancePresets};
pub use performance_config::{
    OptimizationParameters, OptimizationStrategy, PerformanceConfig as PerformanceConfigCore,
};
pub use performance_metrics::{
    BottleneckType, OptimizationSuggestion, PerformanceAnalysis, PerformanceBottleneck,
    PerformanceMetrics,
};
pub use performance_profiler::{
    PerformanceProfiler as PerfProfiler, PerformanceReport as PerformanceReportCore,
};
// #[cfg(feature = "serialization")]
// pub use persistence::{MachinePersistence, PersistenceConfig, PersistentMachine};
pub use coverage_tracking::{CoverageReport, CoverageStats, CoverageTracker};
pub use integration_testing::{IntegrationScenario, IntegrationTestResult, IntegrationTestRunner};
pub use performance_tracking::{
    PerformanceReport as TrackingPerformanceReport, PerformanceTracker,
};
pub use persistence_core::{
    BackupConfig, MachineDeserialize, MachineSerialize, PersistenceConfig,
    PersistenceError as CorePersistenceError, PersistenceStrategy, StorageType,
};
pub use persistence::ext::{
    migration as migrations, monitoring, factories as persistence_builder, MachinePersistenceExt, PersistenceInfo,
    PersistentMachine,
};
pub use persistence::manager::{
    BackupEntry, BackupManager, MachineInfo, MachinePersistence, PersistenceStats,
};
pub use persistence::metadata::{
    MachineMetadata, MachineStats, MetadataBuilder, SchemaInfo, ValidationRule, ValidationType,
};
pub use persistence::serialization::{
    ComplexityMetrics, SerializedMachine, SerializedState, SerializedTransition, StateType,
};
pub use persistence::storage::{
    FileSystemStorage, LocalStorage, MachineStorage, MemoryStorage, StorageFactory, StorageInfo,
};
pub use property_testing::{Property, PropertyResult, PropertyTestResult, PropertyTestRunner};
pub use test_builder::{MachineTestingExt, TestBuilder, TestSuiteResult};
pub use test_cases::{TestCase as TestCaseTypes, TestCaseExecutor, TestCaseStep};
pub use test_data_generation::{
    DataGenerationConfig, DataGenerationStrategy, DefaultTestDataGenerator,
    MachineTestDataGenerator, TestDataGenerationManager, TestDataGenerator,
};
pub use test_runner::MachineTestRunner as TestRunner;
pub use test_types::{
    DataStrategy, PerformanceMetrics as TestPerformanceMetrics, TestConfig as TestConfigTypes,
    TestCoverage, TestResult as TestResultTypes, TestStep,
};
pub use testing::{MachineTestRunner, TestCase, TestConfig, TestResult};
pub use visualization_config::{
    ExportFormat, LayoutDirection, RenderingOptions, VisualizationConfig, VisualizationTheme,
};
pub use visualization_core::{
    ErrorSummary, MachineVisualizer, PerformanceReport as CorePerformanceReport,
};
pub use visualization_data::{
    StateDiagram, StateInfo as DataStateInfo, TransitionInfo as VisualizationTransitionInfo,
};
pub use visualization_debug::{
    Breakpoint, BreakpointType, TimeTravelDebugger, TimeTravelPosition, VisualizationStats,
};
pub use visualization_events::{
    ActionResult, ErrorEvent, ErrorEventType, GuardResult, PerformanceEvent, PerformanceEventType,
    StateChangeEvent, StateChangeType, TransitionEvent,
};
pub use visualization_ext::{
    AutoExportSettings, AutoVisualizer, MachineVisualizationExt, VisualizedMachine,
};
pub use visualization_monitor::{
    HealthCheckResult, HealthChecker, HealthStatus as VisualizationHealthStatus, MonitoringStats,
    StateInfo as MonitorStateInfo, StateMonitor, StateStatus,
};
// #[cfg(feature = "serialization")]
// pub use visualization::{MachineVisualizer, VisualizationConfig, VisualizedMachine};
