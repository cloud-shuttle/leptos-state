# Logging Middleware Design

## Overview
Implement logging middleware to automatically track state changes, transitions, and operations for debugging, auditing, and monitoring purposes.

## Current State
```rust
// Manual logging only
impl<S: State> Store<S> {
    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        self.signal.update(updater);
        // Manual logging would go here
        log::debug!("State updated");
        Ok(())
    }
}
```

## Proposed Enhancement
```rust
pub struct LoggingMiddleware<S: State, E: Event = ()> {
    level: LogLevel,
    include_state_diffs: bool,
    include_timestamps: bool,
    filter: Option<Box<dyn Fn(&MiddlewareContext<S, E>) -> bool + Send + Sync>>,
}

impl<S: State, E: Event> Store<S> {
    pub fn with_logging(self, level: LogLevel) -> Self {
        self.with_middleware(LoggingMiddleware::new(level))
    }
}
```

## Motivation

### Debugging and Development
- **State Change Tracking**: Automatically log all state mutations
- **Transition Logging**: Track state machine transitions
- **Performance Monitoring**: Measure operation timing
- **Error Tracing**: Log failed operations with context

### Auditing and Compliance
- **Change History**: Record who/what/when for state changes
- **Compliance Logging**: Meet regulatory requirements
- **Security Monitoring**: Track sensitive state access
- **Usage Analytics**: Understand application usage patterns

### Use Cases
- Development debugging of state changes
- Production monitoring and alerting
- Audit trails for compliance
- Performance profiling of state operations
- User behavior analytics
- Troubleshooting production issues

## Implementation Details

### Logging Middleware
```rust
#[derive(Clone, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

pub struct LoggingMiddleware<S: State, E: Event = ()> {
    level: LogLevel,
    include_state_diffs: bool,
    include_timestamps: bool,
    include_metadata: bool,
    filter: Option<Box<dyn Fn(&MiddlewareContext<S, E>) -> bool + Send + Sync>>,
    logger: Box<dyn Fn(LogLevel, &str) + Send + Sync>,
}

impl<S: State, E: Event> LoggingMiddleware<S, E> {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            include_state_diffs: true,
            include_timestamps: true,
            include_metadata: false,
            filter: None,
            logger: Box::new(|level, message| {
                match level {
                    LogLevel::Trace => log::trace!("{}", message),
                    LogLevel::Debug => log::debug!("{}", message),
                    LogLevel::Info => log::info!("{}", message),
                    LogLevel::Warn => log::warn!("{}", message),
                    LogLevel::Error => log::error!("{}", message),
                    LogLevel::Off => {}
                }
            }),
        }
    }

    pub fn with_state_diffs(mut self, include: bool) -> Self {
        self.include_state_diffs = include;
        self
    }

    pub fn with_timestamps(mut self, include: bool) -> Self {
        self.include_timestamps = include;
        self
    }

    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    pub fn with_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&MiddlewareContext<S, E>) -> bool + Send + Sync + 'static,
    {
        self.filter = Some(Box::new(filter));
        self
    }

    pub fn with_custom_logger<F>(mut self, logger: F) -> Self
    where
        F: Fn(LogLevel, &str) + Send + Sync + 'static,
    {
        self.logger = Box::new(logger);
        self
    }

    fn should_log(&self, level: &LogLevel) -> bool {
        matches!(
            (&self.level, level),
            (LogLevel::Trace, _) |
            (LogLevel::Debug, LogLevel::Debug | LogLevel::Info | LogLevel::Warn | LogLevel::Error) |
            (LogLevel::Info, LogLevel::Info | LogLevel::Warn | LogLevel::Error) |
            (LogLevel::Warn, LogLevel::Warn | LogLevel::Error) |
            (LogLevel::Error, LogLevel::Error)
        )
    }

    fn format_state_diff(&self, old_state: &S, new_state: &S) -> String
    where
        S: Debug,
    {
        if self.include_state_diffs {
            format!(" ({} -> {})", format!("{:?}", old_state), format!("{:?}", new_state))
        } else {
            String::new()
        }
    }

    fn format_timestamp(&self) -> String {
        if self.include_timestamps {
            format!(" [{}]", Utc::now().to_rfc3339())
        } else {
            String::new()
        }
    }

    fn format_metadata(&self, ctx: &MiddlewareContext<S, E>) -> String {
        if self.include_metadata && !ctx.metadata.is_empty() {
            format!(" metadata={:?}", ctx.metadata)
        } else {
            String::new()
        }
    }
}

impl<S: State, E: Event> Middleware<S, E> for LoggingMiddleware<S, E> {
    fn name(&self) -> &'static str {
        "logging"
    }

    fn should_process(&self, ctx: &MiddlewareContext<S, E>) -> bool {
        if let Some(ref filter) = self.filter {
            filter(ctx)
        } else {
            true
        }
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        let message = match &ctx.operation {
            Operation::StoreUpdate { old_state, new_state, .. } => {
                let level = LogLevel::Debug;
                if self.should_log(&level) {
                    let timestamp = self.format_timestamp();
                    let state_diff = self.format_state_diff(old_state, new_state);
                    let metadata = self.format_metadata(ctx);
                    format!("Store updated{}{}{}", timestamp, state_diff, metadata)
                } else {
                    return Ok(());
                }
            }
            Operation::MachineTransition { machine, event, transition } => {
                let level = LogLevel::Info;
                if self.should_log(&level) {
                    let timestamp = self.format_timestamp();
                    let metadata = self.format_metadata(ctx);
                    format!(
                        "Machine transition: {} --({})--> {}{}{}",
                        machine.current_state(),
                        event.event_type(),
                        transition.target,
                        timestamp,
                        metadata
                    )
                } else {
                    return Ok(());
                }
            }
            Operation::StoreInit { initial_state } => {
                let level = LogLevel::Info;
                if self.should_log(&level) {
                    let timestamp = self.format_timestamp();
                    let metadata = self.format_metadata(ctx);
                    format!("Store initialized with state{:?}{}{}", initial_state, timestamp, metadata)
                } else {
                    return Ok(());
                }
            }
            Operation::StoreReset { old_state, new_state } => {
                let level = LogLevel::Warn;
                if self.should_log(&level) {
                    let timestamp = self.format_timestamp();
                    let state_diff = self.format_state_diff(old_state, new_state);
                    let metadata = self.format_metadata(ctx);
                    format!("Store reset{}{}{}", timestamp, state_diff, metadata)
                } else {
                    return Ok(());
                }
            }
        };

        (self.logger)(LogLevel::Info, &message);
        Ok(())
    }
}
```

### Specialized Loggers
```rust
pub struct StructuredLoggingMiddleware<S: State, E: Event> {
    logger: Box<dyn Fn(&LogEntry) + Send + Sync>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub operation: String,
    pub details: serde_json::Value,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl<S: State, E: Event> StructuredLoggingMiddleware<S, E> {
    pub fn new<F>(logger: F) -> Self
    where
        F: Fn(&LogEntry) + Send + Sync + 'static,
    {
        Self {
            logger: Box::new(logger),
        }
    }

    pub fn with_json_file_logger(file_path: &str) -> Result<Self, MiddlewareError> {
        // Create JSON file logger
        todo!()
    }
}

impl<S: State, E: Event> Middleware<S, E> for StructuredLoggingMiddleware<S, E> {
    fn name(&self) -> &'static str {
        "structured_logging"
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: "info".to_string(),
            operation: self.operation_type(ctx),
            details: self.operation_details(ctx),
            metadata: ctx.metadata.clone(),
        };

        (self.logger)(&entry);
        Ok(())
    }

    fn operation_type(&self, ctx: &MiddlewareContext<S, E>) -> String {
        match ctx.operation {
            Operation::StoreUpdate { .. } => "store_update".to_string(),
            Operation::MachineTransition { .. } => "machine_transition".to_string(),
            Operation::StoreInit { .. } => "store_init".to_string(),
            Operation::StoreReset { .. } => "store_reset".to_string(),
        }
    }

    fn operation_details(&self, ctx: &MiddlewareContext<S, E>) -> serde_json::Value {
        match &ctx.operation {
            Operation::StoreUpdate { old_state, new_state, .. } => {
                json!({
                    "old_state": format!("{:?}", old_state),
                    "new_state": format!("{:?}", new_state)
                })
            }
            Operation::MachineTransition { machine, event, transition } => {
                json!({
                    "from_state": machine.current_state(),
                    "event": event.event_type(),
                    "to_state": &transition.target
                })
            }
            Operation::StoreInit { initial_state } => {
                json!({
                    "initial_state": format!("{:?}", initial_state)
                })
            }
            Operation::StoreReset { old_state, new_state } => {
                json!({
                    "old_state": format!("{:?}", old_state),
                    "new_state": format!("{:?}", new_state)
                })
            }
        }
    }
}
```

### Performance Logging
```rust
pub struct PerformanceLoggingMiddleware<S: State, E: Event> {
    slow_operation_threshold: Duration,
    logger: Box<dyn Fn(&PerformanceLog) + Send + Sync>,
}

#[derive(Clone, Debug)]
pub struct PerformanceLog {
    pub operation: String,
    pub duration: Duration,
    pub timestamp: DateTime<Utc>,
    pub is_slow: bool,
}

impl<S: State, E: Event> PerformanceLoggingMiddleware<S, E> {
    pub fn new(threshold: Duration) -> Self {
        Self {
            slow_operation_threshold: threshold,
            logger: Box::new(|log| {
                if log.is_slow {
                    log::warn!("Slow operation: {} took {:?}", log.operation, log.duration);
                } else {
                    log::debug!("Operation: {} took {:?}", log.operation, log.duration);
                }
            }),
        }
    }
}

impl<S: State, E: Event> Middleware<S, E> for PerformanceLoggingMiddleware<S, E> {
    fn name(&self) -> &'static str {
        "performance_logging"
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        let start = Instant::now();
        let operation_name = self.operation_name(ctx);

        // Continue with normal processing
        // (The actual operation timing would need to be handled at the store/machine level)

        let duration = start.elapsed();
        let is_slow = duration > self.slow_operation_threshold;

        let log_entry = PerformanceLog {
            operation: operation_name,
            duration,
            timestamp: Utc::now(),
            is_slow,
        };

        (self.logger)(&log_entry);
        Ok(())
    }

    fn operation_name(&self, ctx: &MiddlewareContext<S, E>) -> String {
        match ctx.operation {
            Operation::StoreUpdate { .. } => "store_update".to_string(),
            Operation::MachineTransition { .. } => "machine_transition".to_string(),
            Operation::StoreInit { .. } => "store_init".to_string(),
            Operation::StoreReset { .. } => "store_reset".to_string(),
        }
    }
}
```

## Error Handling

### Logging Failures
```rust
impl<S: State, E: Event> Middleware<S, E> for LoggingMiddleware<S, E> {
    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        match self.try_log(ctx) {
            Ok(()) => Ok(()),
            Err(e) => {
                // Don't fail the operation due to logging errors
                // Log the logging failure itself (carefully to avoid recursion)
                eprintln!("Logging middleware failed: {:?}", e);
                Ok(())
            }
        }
    }

    fn try_log(&self, ctx: &MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        // Main logging logic that can fail
        let message = self.format_message(ctx)?;
        (self.logger)(LogLevel::Info, &message);
        Ok(())
    }

    fn format_message(&self, ctx: &MiddlewareContext<S, E>) -> Result<String, MiddlewareError> {
        // Formatting logic that might fail (e.g., serialization)
        match &ctx.operation {
            Operation::StoreUpdate { old_state, new_state, .. } => {
                let state_diff = if self.include_state_diffs {
                    format!(" ({} -> {})", self.format_state(old_state)?, self.format_state(new_state)?)
                } else {
                    String::new()
                };
                Ok(format!("Store updated{}{}", self.format_timestamp(), state_diff))
            }
            // Other cases...
        }
    }

    fn format_state(&self, state: &S) -> Result<String, MiddlewareError>
    where
        S: Debug,
    {
        Ok(format!("{:?}", state))
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn logging_middleware_formats_messages_correctly() {
    let middleware = LoggingMiddleware::<TestState, TestEvent>::new(LogLevel::Debug)
        .with_state_diffs(true)
        .with_timestamps(false);

    let old_state = TestState { count: 5 };
    let mut new_state = TestState { count: 10 };

    let ctx = MiddlewareContext {
        operation: Operation::StoreUpdate {
            old_state: &old_state,
            new_state: &mut new_state,
            updater: &|_| {},
        },
        metadata: HashMap::new(),
        should_continue: true,
    };

    // Test formatting without executing middleware
    let formatted = middleware.format_message(&ctx).unwrap();
    assert!(formatted.contains("Store updated"));
    assert!(formatted.contains("5"));
    assert!(formatted.contains("10"));
}

#[test]
fn logging_middleware_respects_log_level() {
    let debug_middleware = LoggingMiddleware::<TestState, TestEvent>::new(LogLevel::Debug);
    let info_middleware = LoggingMiddleware::<TestState, TestEvent>::new(LogLevel::Info);

    // Debug middleware should log debug messages
    assert!(debug_middleware.should_log(&LogLevel::Debug));
    // Info middleware should not log debug messages
    assert!(!info_middleware.should_log(&LogLevel::Debug));
    // Both should log info messages
    assert!(debug_middleware.should_log(&LogLevel::Info));
    assert!(info_middleware.should_log(&LogLevel::Info));
}
```

### Integration Tests
```rust
#[test]
fn store_with_logging_middleware() {
    let mut store = Store::new(TestState { count: 0 });
    store = store.with_middleware(LoggingMiddleware::new(LogLevel::Debug));

    // Capture log output (in real tests, you'd use a test logger)
    let mut log_messages = Vec::new();
    let logging_middleware = LoggingMiddleware::new(LogLevel::Debug)
        .with_custom_logger(|level, message| {
            log_messages.push((level, message.to_string()));
        });

    let mut store_with_custom = Store::new(TestState { count: 0 });
    store_with_custom = store_with_custom.with_middleware(logging_middleware);

    // Perform operation
    store_with_custom.update_with_middleware(|s| s.count = 42).unwrap();

    // Verify logging occurred
    assert!(!log_messages.is_empty());
    let (level, message) = &log_messages[0];
    assert_eq!(*level, LogLevel::Debug);
    assert!(message.contains("Store updated"));
}
```

### Performance Tests
```rust
#[test]
fn logging_middleware_performance_impact() {
    let store_without_logging = Store::new(TestState { count: 0 });
    let store_with_logging = Store::new(TestState { count: 0 })
        .with_middleware(LoggingMiddleware::new(LogLevel::Debug));

    let iterations = 1000;

    // Benchmark without logging
    let start = Instant::now();
    for _ in 0..iterations {
        store_without_logging.update(|s| s.count += 1).unwrap();
    }
    let without_logging = start.elapsed();

    // Benchmark with logging
    let start = Instant::now();
    for _ in 0..iterations {
        store_with_logging.update_with_middleware(|s| s.count += 1).unwrap();
    }
    let with_logging = start.elapsed();

    let overhead = with_logging.as_nanos() as f64 / without_logging.as_nanos() as f64;
    println!("Logging overhead: {:.2}x", overhead);

    // Overhead should be reasonable (less than 10x)
    assert!(overhead < 10.0);
}
```

## Performance Impact

### Logging Overhead
- **Minimal for Off level**: No overhead when logging is disabled
- **Low for simple logging**: String formatting and log calls
- **Higher for state diffs**: Serialization of complex state
- **Configurable**: Can disable expensive features

### Optimization Strategies
```rust
impl<S: State, E: Event> LoggingMiddleware<S, E> {
    pub fn optimized_for_production() -> Self {
        Self::new(LogLevel::Warn)
            .with_state_diffs(false)
            .with_metadata(false)
    }

    pub fn optimized_for_development() -> Self {
        Self::new(LogLevel::Debug)
            .with_state_diffs(true)
            .with_timestamps(true)
    }

    pub fn with_sampling_rate(mut self, rate: f64) -> Self {
        // Only log a percentage of operations
        self.filter = Some(Box::new(move |_| rand::random::<f64>() < rate));
        self
    }
}
```

## Security Considerations

### Sensitive Data Logging
- Avoid logging sensitive information in state diffs
- Redact sensitive fields in log output
- Use different log levels for different sensitivity

```rust
#[derive(Clone)]
struct UserState {
    user_id: String,
    #[serde(skip)]  // Don't serialize in normal logs
    password_hash: String,
    preferences: HashMap<String, String>,
}

impl Debug for UserState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("UserState")
            .field("user_id", &self.user_id)
            .field("preferences_count", &self.preferences.len())
            .field("password_hash", &"[REDACTED]")
            .finish()
    }
}
```

### Log Injection Prevention
- Sanitize log messages to prevent log injection attacks
- Use structured logging to avoid format string vulnerabilities
- Validate metadata keys and values

### Access Control
```rust
pub struct AccessControlledLoggingMiddleware<S: State, E: Event> {
    inner: LoggingMiddleware<S, E>,
    user_permissions: Vec<String>,
}

impl<S: State, E: Event> AccessControlledLoggingMiddleware<S, E> {
    pub fn new(level: LogLevel, permissions: Vec<String>) -> Self {
        Self {
            inner: LoggingMiddleware::new(level),
            user_permissions: permissions,
        }
    }

    fn can_log_sensitive_data(&self) -> bool {
        self.user_permissions.contains(&"log_sensitive_data".to_string())
    }
}

impl<S: State, E: Event> Middleware<S, E> for AccessControlledLoggingMiddleware<S, E> {
    fn name(&self) -> &'static str {
        "access_controlled_logging"
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        // Create redacted context based on permissions
        let redacted_ctx = if self.can_log_sensitive_data() {
            ctx.clone()
        } else {
            self.redact_context(ctx)
        };

        self.inner.process(&mut redacted_ctx)
    }

    fn redact_context(&self, ctx: &MiddlewareContext<S, E>) -> MiddlewareContext<S, E> {
        // Remove or redact sensitive information
        todo!()
    }
}
```

## Future Extensions

### Distributed Tracing
```rust
#[cfg(feature = "tracing")]
pub struct TracingMiddleware<S: State, E: Event> {
    tracer: opentelemetry::trace::Tracer,
}

#[cfg(feature = "tracing")]
impl<S: State, E: Event> TracingMiddleware<S, E> {
    pub fn new(tracer: opentelemetry::trace::Tracer) -> Self {
        Self { tracer }
    }
}

#[cfg(feature = "tracing")]
impl<S: State, E: Event> Middleware<S, E> for TracingMiddleware<S, E> {
    fn name(&self) -> &'static str {
        "tracing"
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        let span = self.tracer.start("state_operation");
        span.set_attribute("operation", self.operation_name(ctx));

        // Add span to context metadata for nested operations
        ctx.metadata.insert(
            "trace_id".to_string(),
            span.span_context().trace_id().to_string().into()
        );

        // Continue processing
        let result = Ok(()); // Normal processing

        span.end();
        result
    }
}
```

### Log Aggregation
```rust
pub struct AggregatingLoggingMiddleware<S: State, E: Event> {
    buffer: Arc<Mutex<Vec<LogEntry>>>,
    flush_interval: Duration,
    max_buffer_size: usize,
}

impl<S: State, E: Event> AggregatingLoggingMiddleware<S, E> {
    pub async fn flush(&self) {
        let entries = {
            let mut buffer = self.buffer.lock().unwrap();
            std::mem::take(&mut *buffer)
        };

        // Send batch to logging service
        self.send_batch(entries).await;
    }

    async fn send_batch(&self, entries: Vec<LogEntry>) {
        // Implement batch sending to external logging service
        todo!()
    }
}
```

## Migration Guide

### Adding Logging to Existing Stores
```rust
// Before
let store = Store::new(initial_state);

// After
let store = Store::new(initial_state)
    .with_middleware(LoggingMiddleware::new(LogLevel::Debug));
```

### Configuration-Based Logging
```rust
#[derive(Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub include_state_diffs: bool,
    pub include_timestamps: bool,
}

pub fn create_store_with_logging<S: State>(
    initial: S,
    config: &LoggingConfig
) -> Store<S> {
    let level = match config.level.as_str() {
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => LogLevel::Info,
    };

    let middleware = LoggingMiddleware::new(level)
        .with_state_diffs(config.include_state_diffs)
        .with_timestamps(config.include_timestamps);

    Store::new(initial).with_middleware(middleware)
}
```

### Environment-Based Configuration
```rust
pub fn logging_from_env() -> LoggingMiddleware<TestState, TestEvent> {
    let level = std::env::var("LOG_LEVEL")
        .map(|l| match l.to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        })
        .unwrap_or(LogLevel::Info);

    LoggingMiddleware::new(level)
        .with_state_diffs(std::env::var("LOG_STATE_DIFFS").is_ok())
        .with_timestamps(std::env::var("LOG_TIMESTAMPS").is_ok())
}
```

## Risk Assessment

### Likelihood: Low
- Logging is generally safe and well-understood
- Most issues are performance-related, not correctness
- Good defaults prevent common pitfalls

### Impact: Low
- Logging failures don't break application logic
- Configurable to minimize performance impact
- Can be disabled in production if needed

### Mitigation
- Graceful degradation when logging fails
- Performance monitoring and optimization
- Configurable log levels and filtering
- Security-conscious defaults (no sensitive data logging)
- Comprehensive testing of logging behavior
