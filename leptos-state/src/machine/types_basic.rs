use std::collections::HashMap;
use std::fmt::Debug;

/// Common types and enums used across the machine module
pub type StateValue = String;

/// Result type for machine operations
pub type MachineResult<T> = Result<T, MachineError>;

/// Errors that can occur during machine operations
#[derive(Debug, Clone)]
pub enum MachineError {
    InvalidState(String),
    InvalidTransition,
    GuardFailed(String),
    MissingGuard(String),
    MissingAction(String),
    ContextError(String),
    StateNotFound(String),
    EventNotFound(String),
    CircularDependency(String),
}

impl std::fmt::Display for MachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineError::InvalidState(s) => write!(f, "Invalid state: {}", s),
            MachineError::InvalidTransition => write!(f, "Invalid transition"),
            MachineError::GuardFailed(s) => write!(f, "Guard failed: {}", s),
            MachineError::MissingGuard(s) => write!(f, "Missing guard: {}", s),
            MachineError::MissingAction(s) => write!(f, "Missing action: {}", s),
            MachineError::ContextError(s) => write!(f, "Context error: {}", s),
            MachineError::StateNotFound(s) => write!(f, "State not found: {}", s),
            MachineError::EventNotFound(s) => write!(f, "Event not found: {}", s),
            MachineError::CircularDependency(s) => write!(f, "Circular dependency: {}", s),
        }
    }
}

impl std::error::Error for MachineError {}

/// State types in the state machine hierarchy
#[derive(Debug, Clone, PartialEq)]
pub enum StateType {
    Atomic,
    Compound,
    Parallel,
    History,
    Final,
}
