//! Error handling structures for integration events

/// Error handling strategy
#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ErrorHandlingStrategy {
    /// Fail immediately on error
    FailFast,
    /// Retry with exponential backoff
    Retry { max_attempts: u32, base_delay: std::time::Duration },
    /// Retry with circuit breaker
    CircuitBreaker { failure_threshold: u32, recovery_timeout: std::time::Duration },
    /// Defer to dead letter queue
    DeadLetter,
    /// Ignore and continue
    Ignore,
    /// Custom error handler
    Custom(String),
}

impl ErrorHandlingStrategy {
    /// Create retry strategy with defaults
    pub fn retry() -> Self {
        Self::Retry {
            max_attempts: 3,
            base_delay: std::time::Duration::from_millis(100),
        }
    }

    /// Create circuit breaker strategy with defaults
    pub fn circuit_breaker() -> Self {
        Self::CircuitBreaker {
            failure_threshold: 5,
            recovery_timeout: std::time::Duration::from_secs(60),
        }
    }

    /// Check if strategy allows retries
    pub fn allows_retry(&self) -> bool {
        matches!(self, Self::Retry { .. } | Self::CircuitBreaker { .. })
    }

    /// Get max attempts for retry strategies
    pub fn max_attempts(&self) -> Option<u32> {
        match self {
            Self::Retry { max_attempts, .. } => Some(*max_attempts),
            Self::CircuitBreaker { failure_threshold, .. } => Some(*failure_threshold),
            _ => None,
        }
    }

    /// Get base delay for retry strategies
    pub fn base_delay(&self) -> Option<std::time::Duration> {
        match self {
            Self::Retry { base_delay, .. } => Some(*base_delay),
            _ => None,
        }
    }

    /// Check if strategy is terminal (no further processing)
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::FailFast | Self::DeadLetter)
    }

    /// Check if strategy allows continuation
    pub fn allows_continuation(&self) -> bool {
        matches!(self, Self::Ignore)
    }

    /// Get strategy description
    pub fn description(&self) -> String {
        match self {
            Self::FailFast => "Fail immediately on error".to_string(),
            Self::Retry { max_attempts, base_delay } => {
                format!("Retry up to {} times with {}ms base delay", max_attempts, base_delay.as_millis())
            }
            Self::CircuitBreaker { failure_threshold, recovery_timeout } => {
                format!("Circuit breaker with {} failure threshold and {}s recovery", failure_threshold, recovery_timeout.as_secs())
            }
            Self::DeadLetter => "Send to dead letter queue".to_string(),
            Self::Ignore => "Ignore and continue".to_string(),
            Self::Custom(name) => format!("Custom strategy: {}", name),
        }
    }
}

impl Clone for ErrorHandlingStrategy {
    fn clone(&self) -> Self {
        match self {
            Self::FailFast => Self::FailFast,
            Self::Retry { max_attempts, base_delay } => Self::Retry {
                max_attempts: *max_attempts,
                base_delay: *base_delay,
            },
            Self::CircuitBreaker { failure_threshold, recovery_timeout } => Self::CircuitBreaker {
                failure_threshold: *failure_threshold,
                recovery_timeout: *recovery_timeout,
            },
            Self::DeadLetter => Self::DeadLetter,
            Self::Ignore => Self::Ignore,
            Self::Custom(name) => Self::Custom(name.clone()),
        }
    }
}

impl std::fmt::Debug for ErrorHandlingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailFast => write!(f, "FailFast"),
            Self::Retry { max_attempts, base_delay } => {
                write!(f, "Retry {{ max_attempts: {}, base_delay: {:?} }}", max_attempts, base_delay)
            }
            Self::CircuitBreaker { failure_threshold, recovery_timeout } => {
                write!(f, "CircuitBreaker {{ failure_threshold: {}, recovery_timeout: {:?} }}", failure_threshold, recovery_timeout)
            }
            Self::DeadLetter => write!(f, "DeadLetter"),
            Self::Ignore => write!(f, "Ignore"),
            Self::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

impl Default for ErrorHandlingStrategy {
    fn default() -> Self {
        Self::FailFast
    }
}

/// Error action to take
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ErrorAction {
    /// Retry the operation
    Retry,
    /// Skip and continue
    Skip,
    /// Fail the entire operation
    Fail,
    /// Send to dead letter queue
    DeadLetter,
    /// Log and continue
    Log,
    /// Custom action
    Custom(String),
}

impl ErrorAction {
    /// Check if action is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(self, Self::Retry | Self::Skip | Self::Log)
    }

    /// Check if action is terminal
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Fail | Self::DeadLetter)
    }

    /// Get action description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Retry => "Retry the operation",
            Self::Skip => "Skip and continue",
            Self::Fail => "Fail the entire operation",
            Self::DeadLetter => "Send to dead letter queue",
            Self::Log => "Log and continue",
            Self::Custom(_) => "Custom action",
        }
    }
}

impl Default for ErrorAction {
    fn default() -> Self {
        Self::Fail
    }
}

/// Integration error
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegrationError {
    /// Error message
    pub message: String,
    /// Error type
    pub error_type: IntegrationErrorType,
    /// Source component
    pub source: String,
    /// Event ID that caused the error
    pub event_id: Option<String>,
    /// Timestamp
    pub timestamp: std::time::Instant,
    /// Retry count
    pub retry_count: u32,
    /// Additional context
    pub context: std::collections::HashMap<String, String>,
}

impl IntegrationError {
    /// Create a new integration error
    pub fn new(message: String, error_type: IntegrationErrorType, source: String) -> Self {
        Self {
            message,
            error_type,
            source,
            event_id: None,
            timestamp: std::time::Instant::now(),
            retry_count: 0,
            context: std::collections::HashMap::new(),
        }
    }

    /// Create from a standard error
    pub fn from_error<E: std::error::Error>(error: E, error_type: IntegrationErrorType, source: String) -> Self {
        Self::new(error.to_string(), error_type, source)
    }

    /// Set event ID
    pub fn with_event_id(mut self, event_id: String) -> Self {
        self.event_id = Some(event_id);
        self
    }

    /// Set retry count
    pub fn with_retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    /// Add context
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }

    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }

    /// Check if error should be retried
    pub fn should_retry(&self, max_retries: u32) -> bool {
        self.retry_count < max_retries && self.error_type.is_recoverable()
    }

    /// Get age of the error
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed()
    }

    /// Get error summary
    pub fn summary(&self) -> String {
        format!(
            "IntegrationError {{ type: {}, source: {}, retries: {}, age: {:.2}s }}",
            self.error_type.as_str(),
            self.source,
            self.retry_count,
            self.age().as_secs_f64()
        )
    }
}

impl std::fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type.as_str(), self.message)
    }
}

impl std::error::Error for IntegrationError {}

impl Default for IntegrationError {
    fn default() -> Self {
        Self::new("Unknown error".to_string(), IntegrationErrorType::Unknown, "unknown".to_string())
    }
}

/// Integration error types
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IntegrationErrorType {
    /// Network connectivity issues
    Network,
    /// Authentication/authorization failures
    Authentication,
    /// Rate limiting
    RateLimit,
    /// Invalid data format
    InvalidData,
    /// Timeout
    Timeout,
    /// External service unavailable
    ServiceUnavailable,
    /// Configuration error
    Configuration,
    /// Internal processing error
    Internal,
    /// Unknown error
    Unknown,
}

impl IntegrationErrorType {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Network => "network",
            Self::Authentication => "authentication",
            Self::RateLimit => "rate_limit",
            Self::InvalidData => "invalid_data",
            Self::Timeout => "timeout",
            Self::ServiceUnavailable => "service_unavailable",
            Self::Configuration => "configuration",
            Self::Internal => "internal",
            Self::Unknown => "unknown",
        }
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::Network | Self::RateLimit | Self::Timeout | Self::ServiceUnavailable
        )
    }

    /// Check if error is client-related
    pub fn is_client_error(&self) -> bool {
        matches!(self, Self::Authentication | Self::InvalidData | Self::Configuration)
    }

    /// Check if error is server-related
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Self::Network | Self::Timeout | Self::ServiceUnavailable | Self::Internal
        )
    }

    /// Get suggested retry delay
    pub fn suggested_retry_delay(&self) -> std::time::Duration {
        match self {
            Self::RateLimit => std::time::Duration::from_secs(60),
            Self::Network | Self::ServiceUnavailable => std::time::Duration::from_secs(5),
            Self::Timeout => std::time::Duration::from_secs(1),
            _ => std::time::Duration::from_millis(100),
        }
    }
}

impl std::fmt::Display for IntegrationErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for IntegrationErrorType {
    fn default() -> Self {
        Self::Unknown
    }
}
