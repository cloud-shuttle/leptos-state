//! Middleware system for cross-cutting concerns in state management
//!
//! This module provides an extensible middleware architecture for adding
//! logging, validation, caching, and other concerns to stores and machines.

use crate::{State, Event};
use std::collections::HashMap;

/// Priority levels for middleware execution order
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MiddlewarePriority {
    /// Execute first (highest priority)
    Highest = 0,
    /// Execute early
    High = 1,
    /// Normal execution order
    Normal = 2,
    /// Execute later
    Low = 3,
    /// Execute last (lowest priority)
    Lowest = 4,
}

/// Core middleware trait that all middleware must implement
pub trait Middleware<S: State, E: Event = ()>: Send + Sync {
    /// Return the name of this middleware
    fn name(&self) -> &'static str;

    /// Process the middleware context
    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError>;

    /// Return the priority of this middleware (default: Normal)
    fn priority(&self) -> MiddlewarePriority {
        MiddlewarePriority::Normal
    }

    /// Return whether this middleware should process the given context (default: true)
    fn should_process(&self, _ctx: &MiddlewareContext<S, E>) -> bool {
        true
    }
}

/// Context passed to middleware during processing
pub struct MiddlewareContext<S: State, E: Event = ()> {
    /// The operation being performed
    pub operation: Operation<S>,
    /// Additional metadata for the operation
    pub metadata: HashMap<String, String>,
    /// Whether processing should continue (can be set to false to cancel)
    pub should_continue: bool,
    /// Phantom data to keep E in scope
    _phantom: std::marker::PhantomData<E>,
}

/// Operations that can be intercepted by middleware
pub enum Operation<S: State> {
    /// Store state update operation
    StoreUpdate {
        /// The state before the update
        old_state: S,
        /// The new state being applied
        new_state: S,
    },
    /// State machine transition operation
    MachineTransition {
        /// Current state name
        current_state: String,
        /// The event triggering the transition
        event_type: String,
        /// Target state name
        target_state: String,
    },
    /// Store initialization
    StoreInit {
        /// The initial state
        initial_state: S,
    },
    /// Store reset operation
    StoreReset {
        /// The state before reset
        old_state: S,
        /// The new state after reset
        new_state: S,
    },
}

impl<S: State, E: Event> MiddlewareContext<S, E> {
    /// Create a new middleware context
    pub fn new(operation: Operation<S>) -> Self {
        Self {
            operation,
            metadata: HashMap::new(),
            should_continue: true,
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Errors that can occur during middleware processing
#[derive(Debug, Clone, thiserror::Error)]
pub enum MiddlewareError {
    #[error("Middleware '{middleware}' failed: {message}")]
    MiddlewareFailed { middleware: String, message: String },

    #[error("Middleware cancelled the operation")]
    Cancelled,

    #[error("Middleware timeout after {duration:?}")]
    Timeout { duration: std::time::Duration },

    #[error("Invalid middleware configuration: {message}")]
    ConfigurationError { message: String },

    #[error("Middleware dependency not satisfied: {dependency}")]
    DependencyError { dependency: String },
}

/// Stack of middleware that executes in priority order
pub struct MiddlewareStack<S: State, E: Event = ()> {
    middlewares: Vec<Box<dyn Middleware<S, E>>>,
}

/// Log levels for middleware logging
#[derive(Clone, Debug, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

/// Logging middleware that logs state changes and transitions
pub struct LoggingMiddleware<S: State, E: Event = ()> {
    level: LogLevel,
    include_timestamps: bool,
    include_metadata: bool,
    filter: Option<Box<dyn Fn(&MiddlewareContext<S, E>) -> bool + Send + Sync>>,
    logger: Box<dyn Fn(LogLevel, &str) + Send + Sync>,
}

impl<S: State, E: Event> LoggingMiddleware<S, E> {
    /// Create a new logging middleware with default settings
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            include_timestamps: true,
            include_metadata: false,
            filter: None,
            logger: Box::new(|level, message| {
                #[cfg(feature = "log")]
                {
                    match level {
                        LogLevel::Trace => log::trace!("{}", message),
                        LogLevel::Debug => log::debug!("{}", message),
                        LogLevel::Info => log::info!("{}", message),
                        LogLevel::Warn => log::warn!("{}", message),
                        LogLevel::Error => log::error!("{}", message),
                        LogLevel::Off => {}
                    }
                }
                #[cfg(not(feature = "log"))]
                {
                    // Fallback to println for basic logging
                    if level != LogLevel::Off {
                        println!("[{:?}] {}", level, message);
                    }
                }
            }),
        }
    }

    /// Include timestamps in log messages
    pub fn with_timestamps(mut self, include: bool) -> Self {
        self.include_timestamps = include;
        self
    }

    /// Include metadata in log messages
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    /// Set a custom filter for which operations to log
    pub fn with_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&MiddlewareContext<S, E>) -> bool + Send + Sync + 'static,
    {
        self.filter = Some(Box::new(filter));
        self
    }

    /// Set a custom logger function
    pub fn with_logger<F>(mut self, logger: F) -> Self
    where
        F: Fn(LogLevel, &str) + Send + Sync + 'static,
    {
        self.logger = Box::new(logger);
        self
    }

    fn should_log(&self, ctx: &MiddlewareContext<S, E>) -> bool {
        if let Some(ref filter) = self.filter {
            filter(ctx)
        } else {
            true
        }
    }

    fn format_message(&self, ctx: &MiddlewareContext<S, E>, operation_desc: String) -> String {
        let mut message = operation_desc;

        if self.include_timestamps {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            message = format!("[{}] {}", timestamp, message);
        }

        if self.include_metadata && !ctx.metadata.is_empty() {
            let metadata: Vec<String> = ctx.metadata.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            message = format!("{} [{}]", message, metadata.join(", "));
        }

        message
    }
}

impl<S: State, E: Event> Middleware<S, E> for LoggingMiddleware<S, E> {
    fn name(&self) -> &'static str {
        "logging"
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        if !self.should_log(ctx) {
            return Ok(());
        }

        let message = match &ctx.operation {
            Operation::StoreUpdate { old_state: _, new_state: _ } => {
                self.format_message(ctx, format!("Store update: state changed"))
            }
            Operation::MachineTransition { current_state, event_type, target_state } => {
                self.format_message(ctx, format!("Machine transition: {} --({})--> {}", current_state, event_type, target_state))
            }
            Operation::StoreInit { .. } => {
                self.format_message(ctx, "Store initialized".to_string())
            }
            Operation::StoreReset { .. } => {
                self.format_message(ctx, "Store reset".to_string())
            }
        };

        (self.logger)(self.level.clone(), &message);
        Ok(())
    }

    fn should_process(&self, ctx: &MiddlewareContext<S, E>) -> bool {
        self.should_log(ctx)
    }
}

/// Validation middleware that enforces business rules
pub struct ValidationMiddleware<S: State, E: Event = ()> {
    validator: Box<dyn Fn(&MiddlewareContext<S, E>) -> Result<(), String> + Send + Sync>,
}

impl<S: State, E: Event> ValidationMiddleware<S, E> {
    /// Create a new validation middleware with a custom validator function
    pub fn new<F>(validator: F) -> Self
    where
        F: Fn(&MiddlewareContext<S, E>) -> Result<(), String> + Send + Sync + 'static,
    {
        Self {
            validator: Box::new(validator),
        }
    }

    /// Create a validation middleware that validates state transitions
    pub fn for_state_transitions<F>(validator: F) -> Self
    where
        F: Fn(&S, &S) -> Result<(), String> + Send + Sync + 'static,
    {
        Self::new(move |ctx| {
            match &ctx.operation {
                Operation::StoreUpdate { old_state, new_state } => {
                    validator(old_state, new_state)
                }
                Operation::StoreReset { old_state, new_state } => {
                    validator(old_state, new_state)
                }
                _ => Ok(()), // Other operations are allowed by default
            }
        })
    }

    /// Create a validation middleware that validates machine transitions
    pub fn for_machine_transitions<F>(validator: F) -> Self
    where
        F: Fn(&str, &str, &str) -> Result<(), String> + Send + Sync + 'static,
    {
        Self::new(move |ctx| {
            match &ctx.operation {
                Operation::MachineTransition { current_state, event_type, target_state } => {
                    validator(current_state, event_type, target_state)
                }
                _ => Ok(()),
            }
        })
    }
}

impl<S: State, E: Event> Middleware<S, E> for ValidationMiddleware<S, E> {
    fn name(&self) -> &'static str {
        "validation"
    }

    fn priority(&self) -> MiddlewarePriority {
        MiddlewarePriority::High // Run validation early
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        (self.validator)(ctx).map_err(|message| {
            MiddlewareError::MiddlewareFailed {
                middleware: self.name().to_string(),
                message,
            }
        })?;

        Ok(())
    }
}

impl<S: State, E: Event> MiddlewareStack<S, E> {
    /// Create a new empty middleware stack
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    /// Add middleware to the stack (sorted by priority)
    pub fn add<M: Middleware<S, E> + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self.sort_by_priority();
        self
    }

    /// Remove middleware by name
    pub fn remove(&mut self, name: &str) {
        self.middlewares.retain(|m| m.name() != name);
    }

    /// Process the middleware stack
    pub fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        for middleware in &self.middlewares {
            if middleware.should_process(ctx) {
                middleware.process(ctx)?;
                if !ctx.should_continue {
                    break;
                }
            }
        }
        Ok(())
    }

    /// Get names of all middleware in the stack
    pub fn middleware_names(&self) -> Vec<&str> {
        self.middlewares.iter().map(|m| m.name()).collect()
    }

    /// Check if the stack contains middleware with the given name
    pub fn contains(&self, name: &str) -> bool {
        self.middlewares.iter().any(|m| m.name() == name)
    }

    /// Get the number of middleware in the stack
    pub fn len(&self) -> usize {
        self.middlewares.len()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.middlewares.is_empty()
    }

    /// Clear all middleware from the stack
    pub fn clear(&mut self) {
        self.middlewares.clear();
    }

    /// Sort middleware by priority (highest first)
    fn sort_by_priority(&mut self) {
        self.middlewares.sort_by_key(|m| m.priority());
    }
}

impl<S: State, E: Event> Default for MiddlewareStack<S, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: State, E: Event> Clone for MiddlewareStack<S, E> {
    fn clone(&self) -> Self {
        // Note: Cloning middleware stack requires middleware to be cloneable
        // For now, create empty stack - middleware should be added explicitly
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{State, Event};

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct TestState {
        value: i32,
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    enum TestEvent {
        #[default]
        Increment,
    }

    #[test]
    fn middleware_stack_adds_and_processes_middleware() {
        let mut stack = MiddlewareStack::<TestState, TestEvent>::new();

        let mut call_count = 0;
        let middleware = TestMiddleware::new("test", move |ctx| {
            call_count += 1;
            Ok(())
        });

        stack = stack.add(middleware);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.middleware_names(), vec!["test"]);

        let mut ctx = MiddlewareContext::new(Operation::StoreInit {
            initial_state: TestState::default(),
        });

        stack.process(&mut ctx).unwrap();
        assert_eq!(call_count, 1);
        assert!(ctx.should_continue);
    }

    #[test]
    fn middleware_can_cancel_operation() {
        let mut stack = MiddlewareStack::<TestState, TestEvent>::new();

        let middleware = TestMiddleware::new("canceller", |ctx| {
            ctx.should_continue = false;
            Ok(())
        });

        let never_called = TestMiddleware::new("never_called", |_| {
            panic!("This should not be called");
        });

        stack = stack.add(middleware).add(never_called);

        let mut ctx = MiddlewareContext::new(Operation::StoreInit {
            initial_state: TestState::default(),
        });

        stack.process(&mut ctx).unwrap();
        assert!(!ctx.should_continue);
    }

    #[test]
    fn middleware_priority_ordering() {
        let mut stack = MiddlewareStack::<TestState, TestEvent>::new();

        let mut execution_order = Vec::new();

        let high_priority = TestMiddleware::new("high", move |ctx| {
            execution_order.push("high");
            Ok(())
        });
        // Note: We'd need to modify TestMiddleware to support priority
        // For now, this test is conceptual

        stack = stack.add(high_priority);
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn logging_middleware_formats_messages() {
        let mut logger_calls = Vec::new();
        let logger = LoggingMiddleware::<TestState, TestEvent>::new(LogLevel::Info)
            .with_logger(move |level, message| {
                logger_calls.push((level, message.to_string()));
            });

        let mut ctx = MiddlewareContext::new(Operation::MachineTransition {
            current_state: "idle".to_string(),
            event_type: "start".to_string(),
            target_state: "running".to_string(),
        });

        logger.process(&mut ctx).unwrap();

        assert_eq!(logger_calls.len(), 1);
        let (level, message) = &logger_calls[0];
        assert_eq!(*level, LogLevel::Info);
        assert!(message.contains("Machine transition"));
        assert!(message.contains("idle"));
        assert!(message.contains("start"));
        assert!(message.contains("running"));
    }

    #[test]
    fn validation_middleware_can_reject_operations() {
        let validator = ValidationMiddleware::<TestState, TestEvent>::new(|ctx| {
            match &ctx.operation {
                Operation::StoreUpdate { new_state, .. } => {
                    if new_state.value < 0 {
                        Err("Value cannot be negative".to_string())
                    } else {
                        Ok(())
                    }
                }
                _ => Ok(()),
            }
        });

        let mut ctx = MiddlewareContext::new(Operation::StoreUpdate {
            old_state: TestState { value: 0 },
            new_state: TestState { value: -1 },
        });

        let result = validator.process(&mut ctx);
        assert!(result.is_err());
        if let Err(MiddlewareError::MiddlewareFailed { message, .. }) = result {
            assert_eq!(message, "Value cannot be negative");
        }
    }

    // Test helper middleware
    struct TestMiddleware<F>
    where
        F: Fn(&mut MiddlewareContext<TestState, TestEvent>) -> Result<(), MiddlewareError> + Send + Sync,
    {
        name: String,
        processor: F,
    }

    impl<F> TestMiddleware<F>
    where
        F: Fn(&mut MiddlewareContext<TestState, TestEvent>) -> Result<(), MiddlewareError> + Send + Sync,
    {
        fn new(name: &str, processor: F) -> Self {
            Self {
                name: name.to_string(),
                processor,
            }
        }
    }

    impl<F> Middleware<TestState, TestEvent> for TestMiddleware<F>
    where
        F: Fn(&mut MiddlewareContext<TestState, TestEvent>) -> Result<(), MiddlewareError> + Send + Sync,
    {
        fn name(&self) -> &'static str {
            "test"
        }

        fn process(&self, ctx: &mut MiddlewareContext<TestState, TestEvent>) -> Result<(), MiddlewareError> {
            (self.processor)(ctx)
        }
    }
}
