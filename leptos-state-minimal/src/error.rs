//! Error types for leptos-state-minimal

/// Errors that can occur in state machines
#[derive(Debug, Clone, thiserror::Error)]
pub enum MachineError {
    #[error("Invalid state: {state}")]
    InvalidState { state: String },

    #[error("Invalid transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },

    #[error("Guard evaluation failed: {reason}")]
    GuardFailed { reason: String },

    #[error("Action execution failed: {reason}")]
    ActionFailed { reason: String },
}

/// Errors that can occur in stores
#[derive(Debug, Clone, thiserror::Error)]
pub enum StoreError {
    #[error("State update failed: {reason}")]
    UpdateFailed { reason: String },

    #[error("Subscription failed: {reason}")]
    SubscriptionFailed { reason: String },
}

/// Result type for machine operations
pub type MachineResult<T> = Result<T, MachineError>;

/// Result type for store operations
pub type StoreResult<T> = Result<T, StoreError>;
