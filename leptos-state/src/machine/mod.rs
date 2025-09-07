//! State machine implementation inspired by XState
//!
//! Provides finite state machines with hierarchical states, guards, and actions.

pub mod actions;
pub mod bundle_optimization;
pub mod codegen;
pub mod documentation;
pub mod events;
pub mod guards;
pub mod history;
pub mod integration;
pub mod machine;
pub mod optimization;
pub mod performance;
pub mod persistence;
pub mod proptest_tests;
pub mod states;
pub mod testing;
pub mod visualization;

pub use machine::*;
// Core machine types
pub use machine::{MachineState, StateMachine};
pub use actions::{Action, ActionBuilder, ActionExecution, ActionExecutor};
pub use bundle_optimization::{BundleOptimization, BundleOptimizationConfig, BundleInfo, BundleAnalysis, BundleComparison, OptimizedBundle, WasmInfo, LoadingStrategy};
pub use codegen::{CodeGenConfig, CodeGenerator, GeneratedFile, MachineCodeGenExt};
pub use documentation::{DocumentationConfig, DocumentationGenerator, GeneratedDocument};
pub use events::Event;
pub use guards::{Guard, GuardBuilder, GuardEvaluation, GuardEvaluator};
pub use history::{HistoryMachine, HistoryTracker, HistoryType};
pub use integration::{IntegrationAdapter, IntegrationConfig, IntegrationManager};
pub use optimization::{MachineOptimization, OptimizedMachine as OptimizationMachine, OptimizationCache, BatchUpdateManager, LazyEvaluationManager, PerformanceMonitor};
pub use performance::{OptimizedMachine, PerformanceConfig, PerformanceProfiler};
pub use persistence::{MachinePersistence, PersistenceConfig, PersistentMachine};
pub use states::{StateValue, HistoryState, HistoryType as StatesHistoryType};
pub use testing::{MachineTestRunner, TestCase, TestConfig, TestResult};
pub use visualization::{MachineVisualizer, VisualizationConfig, VisualizedMachine};
