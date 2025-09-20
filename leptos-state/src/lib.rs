//! # leptos-state v1.0.0 - Redesigned Architecture
//!
//! This module contains the completely redesigned architecture that fixes
//! the fundamental type system issues present in v0.2.x.
//!
//! ## Design Philosophy
//!
//! 1. **Trait-first design** with proper bounds
//! 2. **Feature flags that actually work** independently and together
//! 3. **Zero-cost abstractions** where possible
//! 4. **WASM-first but native-compatible**
//! 5. **Leptos v0.8+ integration** from day one

pub mod v1;
pub mod hooks;
pub mod machine;
pub mod store;
pub mod utils;
pub mod api_spec;
pub mod schema;
pub mod version;

// Re-export main types for easy access
pub use v1::*;
pub use hooks::*;
pub use version::*;
// Re-export specific machine types to avoid conflicts
pub use machine::{
    MachineState, StateMachine, Action, ActionBuilder, ActionExecution, ActionExecutor,
    BundleOptimization, BundleOptimizationConfig, BundleInfo, BundleAnalysis, BundleComparison, 
    OptimizedBundle, WasmInfo, LoadingStrategy, CodeGenConfig, CodeGenerator, GeneratedFile, 
    MachineCodeGenExt, DocumentationConfig, DocumentationGenerator, GeneratedDocument,
    Event, Guard, GuardBuilder, GuardEvaluation, GuardEvaluator, HistoryMachine, 
    HistoryTracker, HistoryType, IntegrationAdapter, IntegrationConfig, IntegrationManager,
    MachineOptimization, OptimizationMachine, OptimizationCache, BatchUpdateManager, 
    LazyEvaluationManager, PerformanceMonitor, OptimizedMachine, PerformanceConfig, 
    PerformanceProfiler, MachinePersistence, PersistenceConfig, PersistentMachine,
    StateValue, HistoryState, StatesHistoryType, MachineTestRunner, TestCase, 
    TestConfig, TestResult, MachineVisualizer, VisualizedMachine
};

#[cfg(feature = "visualization")]
pub use machine::VisualizationConfig;
