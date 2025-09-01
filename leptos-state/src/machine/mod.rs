//! State machine implementation inspired by XState
//!
//! Provides finite state machines with hierarchical states, guards, and actions.

pub mod machine;
pub mod states;
pub mod events;
pub mod guards;
pub mod history;
pub mod actions;
pub mod persistence;
pub mod visualization;
pub mod testing;
pub mod performance;
pub mod integration;
pub mod documentation;
pub mod codegen;

pub use machine::*;
// Core machine types
pub use events::Event;
pub use guards::{Guard, GuardBuilder, GuardEvaluation, GuardEvaluator};
pub use history::{HistoryTracker, HistoryMachine, HistoryType};
pub use actions::{Action, ActionBuilder, ActionExecution, ActionExecutor};
pub use persistence::{MachinePersistence, PersistentMachine, PersistenceConfig};
pub use visualization::{MachineVisualizer, VisualizedMachine, VisualizationConfig};
pub use testing::{MachineTestRunner, TestConfig, TestCase, TestResult};
pub use performance::{OptimizedMachine, PerformanceConfig, PerformanceProfiler};
pub use integration::{IntegrationManager, IntegrationConfig, IntegrationAdapter};
pub use documentation::{DocumentationGenerator, DocumentationConfig, GeneratedDocument};
pub use codegen::{CodeGenerator, CodeGenConfig, GeneratedFile, MachineCodeGenExt};