//! # Error Types for State Machine Architecture
//! 
//! This module defines comprehensive error types that provide detailed
//! information about what went wrong and how to fix it.



#[cfg(feature = "persist")]
use serde_json;

// =============================================================================
// Core Error Types
// =============================================================================

/// Comprehensive error type for state machine operations
#[derive(Debug, thiserror::Error)]
pub enum StateMachineError<C, E, S> {
    /// Error during state machine construction
    #[error("Construction error: {0}")]
    Construction(#[from] ConstructionError),
    
    /// Error during state transitions
    #[error("Transition error: {0}")]
    Transition(#[from] TransitionError<E>),
    
    /// Error during action execution
    #[error("Action error: {0}")]
    Action(#[from] ActionError),
    
    /// Error during guard evaluation
    #[error("Guard error: {0}")]
    Guard(#[from] GuardError<E>),
    
    /// Error during context operations
    #[error("Context error: {0}")]
    Context(#[from] ContextError<C>),
    
    /// Error during state validation
    #[error("State error: {0}")]
    State(#[from] StateError<S>),
    
    /// Error during serialization/deserialization
    #[error("Serialization error: {0}")]
    Serialization(#[from] SerializationError),
    
    /// Error during persistence operations
    #[error("Persistence error: {0}")]
    Persistence(#[from] PersistenceError),
    
    /// Error during visualization generation
    #[error("Visualization error: {0}")]
    Visualization(#[from] VisualizationError),
    
    /// Error during testing operations
    #[error("Testing error: {0}")]
    Testing(#[from] TestingError),
}

/// Error that occurs during state machine construction
#[derive(Debug, thiserror::Error)]
pub enum ConstructionError {
    #[error("No initial state specified")]
    NoInitialState,
    
    #[error("Initial state '{0}' not found in state definitions")]
    InitialStateNotFound(String),
    
    #[error("State '{0}' has no outgoing transitions")]
    StateWithNoTransitions(String),
    
    #[error("State '{0}' has no incoming transitions and is not initial")]
    UnreachableState(String),
    
    #[error("Duplicate state definition: '{0}'")]
    DuplicateState(String),
    
    #[error("Invalid state name: '{0}' (must be alphanumeric and start with letter)")]
    InvalidStateName(String),
    
    #[error("State machine must have at least one state")]
    NoStates,
    
    #[error("Circular dependency detected in state transitions")]
    CircularDependency,
    
    #[error("Missing required context type")]
    MissingContextType,
    
    #[error("Missing required event type")]
    MissingEventType,
}

/// Error that occurs during state transitions
#[derive(Debug, thiserror::Error)]
pub enum TransitionError<E> {
    #[error("Invalid transition: event {0:?} is not allowed in the current state")]
    InvalidTransition(E),
    
    #[error("State machine is in an invalid state")]
    InvalidState,
    
    #[error("Event {0:?} is not recognized by this state machine")]
    UnknownEvent(E),
    
    #[error("Transition blocked by guard condition")]
    GuardBlocked,
    
    #[error("Transition would result in invalid state")]
    InvalidResultState,
    
    #[error("Transition requires context that is not available")]
    MissingContext,
    
    #[error("Transition timeout exceeded")]
    Timeout,
}

/// Error that occurs during action execution
#[derive(Debug, thiserror::Error)]
pub enum ActionError {
    #[error("Action execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Action validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Action requires context that is not available")]
    MissingContext,
    
    #[error("Action requires permissions that are not granted")]
    InsufficientPermissions,
    
    #[error("Action execution timeout")]
    Timeout,
    
    #[error("Action dependency not satisfied: {0}")]
    DependencyNotSatisfied(String),
    
    #[error("Action would cause invalid state")]
    InvalidStateChange,
}

/// Error that occurs during guard evaluation
#[derive(Debug, thiserror::Error)]
pub enum GuardError<E> {
    #[error("Guard evaluation failed: {0}")]
    EvaluationFailed(String),
    
    #[error("Guard requires context that is not available")]
    MissingContext,
    
    #[error("Guard condition is invalid: {0}")]
    InvalidCondition(String),
    
    #[error("Guard evaluation timeout")]
    Timeout,
    
    #[error("Guard blocked transition for event {0:?}")]
    TransitionBlocked(E),
}

/// Error that occurs during context operations
#[derive(Debug, thiserror::Error)]
pub enum ContextError<C> {
    #[error("Context validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Context is in invalid state: {0:?}")]
    InvalidState(C),
    
    #[error("Context update failed: {0}")]
    UpdateFailed(String),
    
    #[error("Context serialization failed: {0}")]
    SerializationFailed(String),
    
    #[error("Context deserialization failed: {0}")]
    DeserializationFailed(String),
    
    #[error("Context is read-only")]
    ReadOnly,
    
    #[error("Context access denied")]
    AccessDenied,
}

/// Error that occurs during state operations
#[derive(Debug, thiserror::Error)]
pub enum StateError<S> {
    #[error("State validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("State {0:?} is not valid for this machine")]
    InvalidState(S),
    
    #[error("State {0:?} is not reachable from initial state")]
    UnreachableState(S),
    
    #[error("State comparison failed: {0}")]
    ComparisonFailed(String),
    
    #[error("State serialization failed: {0}")]
    SerializationFailed(String),
    
    #[error("State deserialization failed: {0}")]
    DeserializationFailed(String),
}

/// Error that occurs during serialization operations
#[derive(Debug, thiserror::Error)]
pub enum SerializationError {
    #[error("Serialization format not supported: {0}")]
    UnsupportedFormat(String),
    
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),
    
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
    
    #[error("Data version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: String, actual: String },
    
    #[error("Required field missing: {0}")]
    MissingField(String),
    
    #[error("Field type mismatch: {0}")]
    TypeMismatch(String),
}

/// Error that occurs during persistence operations
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Storage backend not available: {0}")]
    BackendNotAvailable(String),
    
    #[error("Storage operation failed: {0}")]
    StorageFailed(String),
    
    #[error("Data corruption detected: {0}")]
    DataCorruption(String),
    
    #[error("Storage quota exceeded")]
    QuotaExceeded,
    
    #[error("Storage access denied")]
    AccessDenied,
    
    #[error("Storage timeout")]
    Timeout,
    
    #[error("Storage not initialized")]
    NotInitialized,
    
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),
    
    #[error("Migration failed: {0}")]
    MigrationFailed(String),
}

/// Migration-specific errors
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Migration analysis failed: {0}")]
    AnalysisFailed(String),
    
    #[error("Code transformation failed: {0}")]
    TransformationFailed(String),
    
    #[error("Migration validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Unsupported migration pattern: {0}")]
    UnsupportedPattern(String),
}

/// Error that occurs during visualization operations
#[derive(Debug, thiserror::Error)]
pub enum VisualizationError {
    #[error("Visualization format not supported: {0}")]
    UnsupportedFormat(String),
    
    #[error("Visualization generation failed: {0}")]
    GenerationFailed(String),
    
    #[error("Visualization export failed: {0}")]
    ExportFailed(String),
    
    #[error("Visualization template not found: {0}")]
    TemplateNotFound(String),
    
    #[error("Visualization data invalid: {0}")]
    InvalidData(String),
    
    #[error("Visualization rendering failed: {0}")]
    RenderingFailed(String),
}

/// Error that occurs during testing operations
#[derive(Debug, thiserror::Error)]
pub enum TestingError {
    #[error("Test case generation failed: {0}")]
    GenerationFailed(String),
    
    #[error("Test execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Test validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Test timeout exceeded")]
    Timeout,
    
    #[error("Test environment not available: {0}")]
    EnvironmentNotAvailable(String),
    
    #[error("Test data invalid: {0}")]
    InvalidTestData(String),
    
    #[error("Test assertion failed: {0}")]
    AssertionFailed(String),
}

// =============================================================================
// Error Conversion Implementations
// =============================================================================

impl<C, E, S> From<std::io::Error> for StateMachineError<C, E, S> {
    fn from(err: std::io::Error) -> Self {
        StateMachineError::Serialization(SerializationError::SerializationFailed(err.to_string()))
    }
}

#[cfg(feature = "persist")]
impl<C, E, S> From<serde_json::Error> for StateMachineError<C, E, S> {
    fn from(err: serde_json::Error) -> Self {
        StateMachineError::Serialization(SerializationError::SerializationFailed(err.to_string()))
    }
}

#[cfg(feature = "persist")]
impl<C, E, S> From<serde_yaml::Error> for StateMachineError<C, E, S> {
    fn from(err: serde_yaml::Error) -> Self {
        StateMachineError::Serialization(SerializationError::SerializationFailed(err.to_string()))
    }
}

// =============================================================================
// Error Context and Help
// =============================================================================

/// Provides additional context and help for errors
pub trait ErrorContext {
    /// Returns a human-readable description of the error
    fn description(&self) -> &str;
    
    /// Returns suggestions for fixing the error
    fn suggestions(&self) -> Vec<String>;
    
    /// Returns the error code for programmatic handling
    fn error_code(&self) -> &str;
    
    /// Returns additional context information
    fn context(&self) -> Option<&str>;
}

impl<C, E, S> ErrorContext for StateMachineError<C, E, S> {
    fn description(&self) -> &str {
        match self {
            StateMachineError::Construction(_) => "Error during state machine construction",
            StateMachineError::Transition(_) => "Error during state transition",
            StateMachineError::Action(_) => "Error during action execution",
            StateMachineError::Guard(_) => "Error during guard evaluation",
            StateMachineError::Context(_) => "Error during context operation",
            StateMachineError::State(_) => "Error during state operation",
            StateMachineError::Serialization(_) => "Error during serialization",
            StateMachineError::Persistence(_) => "Error during persistence operation",
            StateMachineError::Visualization(_) => "Error during visualization generation",
            StateMachineError::Testing(_) => "Error during testing operation",
        }
    }
    
    fn suggestions(&self) -> Vec<String> {
        match self {
            StateMachineError::Construction(err) => match err {
                ConstructionError::NoInitialState => vec![
                    "Specify an initial state using .initial()".to_string(),
                    "Ensure at least one state is defined".to_string(),
                ],
                ConstructionError::InitialStateNotFound(name) => vec![
                    format!("Check that state '{}' is defined", name),
                    "Verify state name spelling".to_string(),
                ],
                _ => vec!["Review state machine definition".to_string()],
            },
            StateMachineError::Transition(err) => match err {
                TransitionError::InvalidTransition(_) => vec![
                    "Check that the event is allowed in the current state".to_string(),
                    "Verify event type matches state machine definition".to_string(),
                ],
                _ => vec!["Review transition logic".to_string()],
            },
            _ => vec!["Check the error details above".to_string()],
        }
    }
    
    fn error_code(&self) -> &str {
        match self {
            StateMachineError::Construction(_) => "CONSTRUCTION_ERROR",
            StateMachineError::Transition(_) => "TRANSITION_ERROR",
            StateMachineError::Action(_) => "ACTION_ERROR",
            StateMachineError::Guard(_) => "GUARD_ERROR",
            StateMachineError::Context(_) => "CONTEXT_ERROR",
            StateMachineError::State(_) => "STATE_ERROR",
            StateMachineError::Serialization(_) => "SERIALIZATION_ERROR",
            StateMachineError::Persistence(_) => "PERSISTENCE_ERROR",
            StateMachineError::Visualization(_) => "VISUALIZATION_ERROR",
            StateMachineError::Testing(_) => "TESTING_ERROR",
        }
    }
    
    fn context(&self) -> Option<&str> {
        None
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err: StateMachineError<(), (), ()> = StateMachineError::Construction(ConstructionError::NoInitialState);
        assert_eq!(err.error_code(), "CONSTRUCTION_ERROR");
        assert!(err.description().contains("construction"));
    }

    #[test]
    fn test_error_suggestions() {
        let err: StateMachineError<(), (), ()> = StateMachineError::Construction(ConstructionError::NoInitialState);
        let suggestions = err.suggestions();
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("initial")));
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let state_err: StateMachineError<(), (), ()> = StateMachineError::from(io_err);
        
        match state_err {
            StateMachineError::Serialization(_) => {},
            _ => panic!("Expected serialization error"),
        }
    }
}
