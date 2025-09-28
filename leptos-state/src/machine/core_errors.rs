/// Machine errors
#[derive(Debug, Clone)]
pub enum MachineError {
    InvalidState(String),
    InvalidTransition,
    GuardFailed(String),
    MissingGuard(String),
    MissingAction(String),
    ContextError(String),
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
        }
    }
}

impl std::error::Error for MachineError {}

pub type MachineResult<T> = Result<T, MachineError>;
