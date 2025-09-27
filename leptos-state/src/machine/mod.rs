//! State machine implementation inspired by XState
//!
//! Provides finite state machines with hierarchical states, guards, and actions.

pub mod actions;
pub mod builder;
pub mod codegen;
pub mod core;
pub mod documentation;
pub mod events;
pub mod guards;
pub mod history;
pub mod integration;
pub mod machine;
pub mod performance;
// #[cfg(feature = "serialization")]
// pub mod persistence;
pub mod states;
pub mod testing;
pub mod types;
// #[cfg(feature = "serialization")]
// pub mod visualization;

// Re-export core types from new modular structure
pub use core::{Machine, StateMachine, MachineState, StateNode, StateType, MachineError, MachineResult, Guard, Action, Context, MachineConfig, MachineHistory};
pub use builder::{MachineBuilder, MachineBuilderImpl, create_machine_builder};
pub use types::{ContextValue, HistoryEntry, EventRoutingConfig, StateValidationConfig, PerformanceConfig, IntegrationConfig, CompleteMachineConfig};

// Legacy compatibility - re-export from old modules for now
pub use machine::*;
// Core machine types
pub use actions::{Action, ActionBuilder, ActionExecution, ActionExecutor};
pub use codegen::{CodeGenConfig, CodeGenerator, GeneratedFile, MachineCodeGenExt};
pub use documentation::{DocumentationConfig, DocumentationGenerator, GeneratedDocument};
pub use events::Event;
pub use guards::{Guard, GuardBuilder, GuardEvaluation, GuardEvaluator};
pub use history::{HistoryMachine, HistoryTracker, HistoryType};
pub use integration::{IntegrationAdapter, IntegrationConfig, IntegrationManager};
pub use performance::{OptimizedMachine, PerformanceConfig, PerformanceProfiler};
// #[cfg(feature = "serialization")]
// pub use persistence::{MachinePersistence, PersistenceConfig, PersistentMachine};
pub use testing::{MachineTestRunner, TestCase, TestConfig, TestResult};
// #[cfg(feature = "serialization")]
// pub use visualization::{MachineVisualizer, VisualizationConfig, VisualizedMachine};
