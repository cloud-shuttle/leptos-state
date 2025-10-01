//! Error types for leptos-state-minimal

/// Errors that can occur in state machines
#[derive(Debug, Clone, thiserror::Error)]
pub enum MachineError {
    #[error("Invalid state: {state}")]
    InvalidState { state: String },

    #[error("Invalid transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },

    #[error("Guard condition failed for transition from {state} on event {event}")]
    GuardFailed { state: String, event: String },

    #[error("Guard function panicked during evaluation")]
    GuardPanic { state: String, event: String },

    #[error("Action cancelled transition from {state} on event {event}")]
    ActionCancelled { state: String, event: String },

    #[error("Action redirected transition from {from} to {to}")]
    InvalidRedirect { from: String, to: String },

    #[error("Action failed: {message}")]
    ActionError { state: String, event: String, message: String },

    #[error("Action panicked in state {state} during {action_type}")]
    ActionPanic { state: String, action_type: String },

    #[cfg(feature = "serde")]
    #[error("Serialization failed: {message}")]
    SerializationError { message: String },

    #[cfg(feature = "serde")]
    #[error("Deserialization failed: {message}")]
    DeserializationError { message: String },
}

/// Errors that can occur in stores
#[derive(Debug, Clone, thiserror::Error)]
pub enum StoreError {
    #[error("State update failed: {reason}")]
    UpdateFailed { reason: String },

    #[error("Subscription failed: {reason}")]
    SubscriptionFailed { reason: String },

    #[cfg(feature = "serde")]
    #[error("Serialization failed: {message}")]
    SerializationError { message: String },

    #[cfg(feature = "serde")]
    #[error("Deserialization failed: {message}")]
    DeserializationError { message: String },

    #[cfg(feature = "serde")]
    #[error("Version mismatch: expected {expected}, found {found}")]
    VersionMismatch { expected: String, found: String },
}

/// Result type for machine operations
pub type MachineResult<T> = Result<T, MachineError>;

/// Result type for store operations
pub type StoreResult<T> = Result<T, StoreError>;

#[cfg(feature = "web")]
impl From<crate::persistence::StorageError> for StoreError {
    fn from(error: crate::persistence::StorageError) -> Self {
        match error {
            crate::persistence::StorageError::Serialization { message } => StoreError::SerializationError { message },
            crate::persistence::StorageError::Deserialization { message } => StoreError::DeserializationError { message },
            _ => StoreError::UpdateFailed { reason: format!("Storage error: {:?}", error) },
        }
    }
}
