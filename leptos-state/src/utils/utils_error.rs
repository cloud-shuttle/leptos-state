//! Error types and result types for leptos-state operations

use std::fmt;

/// Result type for leptos-state operations
pub type StateResult<T> = Result<T, StateError>;

/// Error types for leptos-state operations
#[derive(Debug, Clone)]
pub enum StateError {
    /// Invalid state transition
    InvalidTransition(String),
    /// State not found
    StateNotFound(String),
    /// Machine not found
    MachineNotFound(String),
    /// Store not found
    StoreNotFound(String),
    /// Event not found
    EventNotFound(String),
    /// Action failed
    ActionFailed(String),
    /// Guard failed
    GuardFailed(String),
    /// Configuration error
    ConfigError(String),
    /// Serialization error
    SerializationError(String),
    /// Deserialization error
    DeserializationError(String),
    /// Validation error
    ValidationError(String),
    /// Network error
    NetworkError(String),
    /// Timeout error
    TimeoutError(String),
    /// Authentication error
    AuthenticationError(String),
    /// Authorization error
    AuthorizationError(String),
    /// Rate limit exceeded
    RateLimitError(String),
    /// Internal error
    InternalError(String),
    /// Custom error with additional context
    Custom {
        /// Error type
        error_type: String,
        /// Error message
        message: String,
        /// Additional context
        context: std::collections::HashMap<String, String>,
    },
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateError::InvalidTransition(msg) => write!(f, "Invalid transition: {}", msg),
            StateError::StateNotFound(msg) => write!(f, "State not found: {}", msg),
            StateError::MachineNotFound(msg) => write!(f, "Machine not found: {}", msg),
            StateError::StoreNotFound(msg) => write!(f, "Store not found: {}", msg),
            StateError::EventNotFound(msg) => write!(f, "Event not found: {}", msg),
            StateError::ActionFailed(msg) => write!(f, "Action failed: {}", msg),
            StateError::GuardFailed(msg) => write!(f, "Guard failed: {}", msg),
            StateError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            StateError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            StateError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            StateError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            StateError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            StateError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
            StateError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            StateError::AuthorizationError(msg) => write!(f, "Authorization error: {}", msg),
            StateError::RateLimitError(msg) => write!(f, "Rate limit exceeded: {}", msg),
            StateError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            StateError::Custom { error_type, message, .. } => write!(f, "{}: {}", error_type, message),
        }
    }
}

impl std::error::Error for StateError {}

impl StateError {
    /// Create a new custom error
    pub fn custom(error_type: String, message: String) -> Self {
        Self::Custom {
            error_type,
            message,
            context: std::collections::HashMap::new(),
        }
    }

    /// Add context to a custom error
    pub fn with_context(mut self, key: String, value: String) -> Self {
        if let Self::Custom { ref mut context, .. } = self {
            context.insert(key, value);
        }
        self
    }

    /// Get the error type as a string
    pub fn error_type(&self) -> &str {
        match self {
            StateError::InvalidTransition(_) => "InvalidTransition",
            StateError::StateNotFound(_) => "StateNotFound",
            StateError::MachineNotFound(_) => "MachineNotFound",
            StateError::StoreNotFound(_) => "StoreNotFound",
            StateError::EventNotFound(_) => "EventNotFound",
            StateError::ActionFailed(_) => "ActionFailed",
            StateError::GuardFailed(_) => "GuardFailed",
            StateError::ConfigError(_) => "ConfigError",
            StateError::SerializationError(_) => "SerializationError",
            StateError::DeserializationError(_) => "DeserializationError",
            StateError::ValidationError(_) => "ValidationError",
            StateError::NetworkError(_) => "NetworkError",
            StateError::TimeoutError(_) => "TimeoutError",
            StateError::AuthenticationError(_) => "AuthenticationError",
            StateError::AuthorizationError(_) => "AuthorizationError",
            StateError::RateLimitError(_) => "RateLimitError",
            StateError::InternalError(_) => "InternalError",
            StateError::Custom { error_type, .. } => error_type,
        }
    }

    /// Check if this is a validation error
    pub fn is_validation_error(&self) -> bool {
        matches!(self, StateError::ValidationError(_))
    }

    /// Check if this is a configuration error
    pub fn is_config_error(&self) -> bool {
        matches!(self, StateError::ConfigError(_))
    }

    /// Check if this is a network error
    pub fn is_network_error(&self) -> bool {
        matches!(self, StateError::NetworkError(_))
    }

    /// Check if this is a timeout error
    pub fn is_timeout_error(&self) -> bool {
        matches!(self, StateError::TimeoutError(_))
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            StateError::NetworkError(_) => true,
            StateError::TimeoutError(_) => true,
            StateError::RateLimitError(_) => true,
            StateError::AuthenticationError(_) => false,
            StateError::AuthorizationError(_) => false,
            StateError::ConfigError(_) => false,
            StateError::ValidationError(_) => false,
            StateError::InternalError(_) => false,
            _ => true,
        }
    }

    /// Get context information for custom errors
    pub fn context(&self) -> Option<&std::collections::HashMap<String, String>> {
        match self {
            StateError::Custom { context, .. } => Some(context),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for StateError {
    fn from(error: serde_json::Error) -> Self {
        StateError::SerializationError(error.to_string())
    }
}

impl From<serde_yaml::Error> for StateError {
    fn from(error: serde_yaml::Error) -> Self {
        StateError::SerializationError(error.to_string())
    }
}

impl From<std::io::Error> for StateError {
    fn from(error: std::io::Error) -> Self {
        StateError::InternalError(error.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for StateError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        StateError::InternalError(error.to_string())
    }
}

/// Error recovery strategies
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorRecoveryStrategy {
    /// Retry the operation
    Retry {
        /// Maximum number of retries
        max_attempts: usize,
        /// Delay between retries
        delay: std::time::Duration,
    },
    /// Use a fallback value
    Fallback {
        /// Fallback value to use
        fallback: String,
    },
    /// Ignore the error
    Ignore,
    /// Fail fast
    Fail,
    /// Log and continue
    LogAndContinue,
}

impl ErrorRecoveryStrategy {
    /// Create a retry strategy
    pub fn retry(max_attempts: usize, delay: std::time::Duration) -> Self {
        Self::Retry { max_attempts, delay }
    }

    /// Create a fallback strategy
    pub fn fallback(fallback: String) -> Self {
        Self::Fallback { fallback }
    }

    /// Execute the recovery strategy
    pub async fn execute<T, F, Fut>(&self, operation: F) -> Result<T, StateError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, StateError>>,
    {
        match self {
            ErrorRecoveryStrategy::Retry { max_attempts, delay } => {
                let mut attempts = 0;
                loop {
                    match operation().await {
                        Ok(result) => return Ok(result),
                        Err(error) => {
                            attempts += 1;
                            if attempts >= *max_attempts || !error.is_recoverable() {
                                return Err(error);
                            }
                            tokio::time::sleep(*delay).await;
                        }
                    }
                }
            }
            ErrorRecoveryStrategy::Fallback { fallback } => {
                match operation().await {
                    Ok(result) => Ok(result),
                    Err(_) => Err(StateError::custom("Fallback".to_string(), fallback.clone())),
                }
            }
            ErrorRecoveryStrategy::Ignore => {
                match operation().await {
                    Ok(result) => Ok(result),
                    Err(_) => Ok(Default::default()),
                }
            }
            ErrorRecoveryStrategy::Fail => operation().await,
            ErrorRecoveryStrategy::LogAndContinue => {
                match operation().await {
                    Ok(result) => Ok(result),
                    Err(error) => {
                        eprintln!("Error (continuing): {}", error);
                        Ok(Default::default())
                    }
                }
            }
        }
    }
}

/// Error reporting and monitoring
pub struct ErrorReporter {
    /// Collected errors
    pub errors: std::sync::Mutex<Vec<StateError>>,
    /// Error counts by type
    pub error_counts: std::sync::Mutex<std::collections::HashMap<String, usize>>,
    /// Maximum number of errors to keep
    pub max_errors: usize,
}

impl ErrorReporter {
    /// Create a new error reporter
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: std::sync::Mutex::new(Vec::new()),
            error_counts: std::sync::Mutex::new(std::collections::HashMap::new()),
            max_errors,
        }
    }

    /// Report an error
    pub fn report(&self, error: StateError) {
        let mut errors = self.errors.lock().unwrap();
        let mut counts = self.error_counts.lock().unwrap();

        // Add to error list
        errors.push(error.clone());

        // Keep only the most recent errors
        if errors.len() > self.max_errors {
            errors.remove(0);
        }

        // Update error counts
        let error_type = error.error_type().to_string();
        *counts.entry(error_type).or_insert(0) += 1;
    }

    /// Get recent errors
    pub fn recent_errors(&self, limit: usize) -> Vec<StateError> {
        let errors = self.errors.lock().unwrap();
        errors.iter().rev().take(limit).cloned().collect()
    }

    /// Get error statistics
    pub fn error_stats(&self) -> std::collections::HashMap<String, usize> {
        self.error_counts.lock().unwrap().clone()
    }

    /// Clear all errors
    pub fn clear(&self) {
        let mut errors = self.errors.lock().unwrap();
        let mut counts = self.error_counts.lock().unwrap();
        errors.clear();
        counts.clear();
    }

    /// Get the total number of errors reported
    pub fn total_errors(&self) -> usize {
        self.errors.lock().unwrap().len()
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new(1000)
    }
}
