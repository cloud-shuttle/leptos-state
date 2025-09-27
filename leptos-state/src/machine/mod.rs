//! State machine implementation inspired by XState
//!
//! Provides finite state machines with hierarchical states, guards, and actions.

pub mod actions;
pub mod builder;
pub mod child_state_builder;
pub mod child_transition_builder;
pub mod codegen;
pub mod core;
pub mod core_types;
pub mod documentation;
pub mod events;
pub mod guards;
pub mod history;
pub mod integration;
pub mod machine;
pub mod machine_builder;
pub mod machine_state_impl;
pub mod performance;
// #[cfg(feature = "serialization")]
// pub mod persistence;
pub mod state_builder;
pub mod states;
pub mod testing;
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
pub use codegen::{CodeGenConfig, CodeGenerator, GeneratedFile, MachineCodeGenExt};
pub use documentation::{DocumentationConfig, DocumentationGenerator, GeneratedDocument};
pub use events::Event;
pub use guards::{GuardBuilder, GuardEvaluation, GuardEvaluator};
pub use history::{HistoryMachine, HistoryTracker, HistoryType};
pub use integration::{IntegrationAdapter, IntegrationManager};
pub use performance::{OptimizedMachine, PerformanceProfiler};
// #[cfg(feature = "serialization")]
// pub use persistence::{MachinePersistence, PersistenceConfig, PersistentMachine};
pub use testing::{MachineTestRunner, TestCase, TestConfig, TestResult};
// #[cfg(feature = "serialization")]
// pub use visualization::{MachineVisualizer, VisualizationConfig, VisualizedMachine};
