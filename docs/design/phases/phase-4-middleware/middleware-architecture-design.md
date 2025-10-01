# Middleware Architecture Design

## Overview
Implement an extensible middleware system for state management, enabling cross-cutting concerns like logging, validation, caching, and monitoring to be added to stores and machines.

## Current State
```rust
// No middleware capabilities
impl<S: State> Store<S> {
    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        self.signal.update(updater);
        Ok(())
    }
}
```

## Proposed Enhancement
```rust
pub trait Middleware<S: State, E: Event = ()>: Send + Sync {
    fn name(&self) -> &'static str;
    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError>;
}

pub struct MiddlewareStack<S: State, E: Event = ()> {
    middlewares: Vec<Box<dyn Middleware<S, E>>>,
}

impl<S: State, E: Event> Store<S> {
    pub fn with_middleware<M: Middleware<S, E> + 'static>(self, middleware: M) -> Self {
        // Add middleware to store
    }
}
```

## Motivation

### Separation of Concerns
- **Cross-cutting Logic**: Handle logging, validation, caching independently of business logic
- **Reusability**: Middleware components can be reused across different stores/machines
- **Maintainability**: Business logic stays clean while infrastructure concerns are separated
- **Extensibility**: New middleware can be added without modifying core logic

### Use Cases
- Logging all state changes for debugging/auditing
- Validating state transitions against business rules
- Caching expensive computations or external data
- Monitoring performance and usage metrics
- Implementing undo/redo functionality
- Synchronizing state with external systems

## Implementation Details

### Core Middleware Traits
```rust
pub trait Middleware<S: State, E: Event = ()>: Send + Sync {
    fn name(&self) -> &'static str;

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError>;

    fn priority(&self) -> MiddlewarePriority {
        MiddlewarePriority::Normal
    }

    fn should_process(&self, ctx: &MiddlewareContext<S, E>) -> bool {
        true
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MiddlewarePriority {
    Highest = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Lowest = 4,
}

pub struct MiddlewareContext<'a, S: State, E: Event = ()> {
    pub operation: Operation<'a, S, E>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub should_continue: bool,
}

#[derive(Clone, Debug)]
pub enum Operation<'a, S: State, E: Event = ()> {
    StoreUpdate {
        old_state: &'a S,
        new_state: &'a mut S,
        updater: &'a dyn Fn(&mut S),
    },
    MachineTransition {
        machine: &'a mut Machine<S, E>,
        event: &'a E,
        transition: &'a Transition<S, E>,
    },
    StoreInit {
        initial_state: &'a S,
    },
    StoreReset {
        old_state: &'a S,
        new_state: &'a S,
    },
}
```

### Middleware Stack
```rust
pub struct MiddlewareStack<S: State, E: Event = ()> {
    middlewares: Vec<Box<dyn Middleware<S, E>>>,
}

impl<S: State, E: Event> MiddlewareStack<S, E> {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    pub fn add<M: Middleware<S, E> + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self.middlewares.sort_by_key(|m| m.priority());
        self
    }

    pub fn remove(&mut self, name: &str) {
        self.middlewares.retain(|m| m.name() != name);
    }

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

    pub fn middleware_names(&self) -> Vec<&str> {
        self.middlewares.iter().map(|m| m.name()).collect()
    }
}
```

### Store Integration
```rust
impl<S: State> Store<S> {
    pub fn with_middleware<M: Middleware<S> + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.add(middleware);
        self
    }

    pub fn update_with_middleware<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        let old_state = self.signal.get_untracked();
        let mut new_state = old_state.clone();

        // Apply updater
        updater(&mut new_state);

        // Create middleware context
        let mut ctx = MiddlewareContext {
            operation: Operation::StoreUpdate {
                old_state: &old_state,
                new_state: &mut new_state,
                updater: &updater,
            },
            metadata: HashMap::new(),
            should_continue: true,
        };

        // Process middleware
        self.middlewares.process(&mut ctx)?;

        if ctx.should_continue {
            // Apply the final state
            self.signal.set(new_state);
            Ok(())
        } else {
            Err(StoreError::MiddlewareCancelled)
        }
    }

    pub fn reset_with_middleware(&self) -> Result<(), StoreError>
    where
        S: Default,
    {
        let old_state = self.signal.get_untracked();
        let new_state = S::default();

        let mut ctx = MiddlewareContext {
            operation: Operation::StoreReset {
                old_state: &old_state,
                new_state: &new_state,
            },
            metadata: HashMap::new(),
            should_continue: true,
        };

        self.middlewares.process(&mut ctx)?;

        if ctx.should_continue {
            self.signal.set(new_state);
            Ok(())
        } else {
            Err(StoreError::MiddlewareCancelled)
        }
    }
}
```

### Machine Integration
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn with_middleware<M: Middleware<S, E> + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.add(middleware);
        self
    }

    pub fn send_with_middleware(&mut self, event: E) -> Result<(), MachineError> {
        // Find transition first
        let current_state_name = self.current_state.clone();
        let current_node = self.states.get(&current_state_name)
            .ok_or_else(|| MachineError::InvalidState {
                state: current_state_name.clone()
            })?;

        if let Some(transition) = current_node.transitions.get(&event.event_type()) {
            let target_state = transition.target.clone();

            // Create middleware context
            let mut ctx = MiddlewareContext {
                operation: Operation::MachineTransition {
                    machine: self,
                    event: &event,
                    transition,
                },
                metadata: HashMap::new(),
                should_continue: true,
            };

            // Process middleware
            self.middlewares.process(&mut ctx)?;

            if ctx.should_continue {
                // Execute the transition
                self.send(event)
            } else {
                Err(MachineError::MiddlewareCancelled)
            }
        } else {
            Err(MachineError::InvalidTransition {
                from: current_state_name,
                to: event.event_type(),
            })
        }
    }
}
```

## Built-in Middleware Types

### Conditional Middleware
```rust
pub struct ConditionalMiddleware<M: Middleware<S, E>, P: Fn(&MiddlewareContext<S, E>) -> bool, S: State, E: Event> {
    middleware: M,
    predicate: P,
}

impl<M, P, S, E> ConditionalMiddleware<M, P, S, E>
where
    M: Middleware<S, E>,
    P: Fn(&MiddlewareContext<S, E>) -> bool + Send + Sync,
    S: State,
    E: Event,
{
    pub fn new(middleware: M, predicate: P) -> Self {
        Self { middleware, predicate }
    }
}

impl<M, P, S, E> Middleware<S, E> for ConditionalMiddleware<M, P, S, E>
where
    M: Middleware<S, E>,
    P: Fn(&MiddlewareContext<S, E>) -> bool + Send + Sync,
    S: State,
    E: Event,
{
    fn name(&self) -> &'static str {
        self.middleware.name()
    }

    fn should_process(&self, ctx: &MiddlewareContext<S, E>) -> bool {
        (self.predicate)(ctx)
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        self.middleware.process(ctx)
    }
}
```

### Composite Middleware
```rust
pub struct CompositeMiddleware<S: State, E: Event> {
    middlewares: Vec<Box<dyn Middleware<S, E>>>,
    name: String,
}

impl<S: State, E: Event> CompositeMiddleware<S, E> {
    pub fn new(name: String) -> Self {
        Self {
            middlewares: Vec::new(),
            name,
        }
    }

    pub fn add<M: Middleware<S, E> + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }
}

impl<S: State, E: Event> Middleware<S, E> for CompositeMiddleware<S, E> {
    fn name(&self) -> &'static str {
        &self.name
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
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
}
```

### Async Middleware (Future Extension)
```rust
#[cfg(feature = "async-middleware")]
pub trait AsyncMiddleware<S: State, E: Event = ()>: Send + Sync {
    fn name(&self) -> &'static str;

    fn process<'a>(&'a self, ctx: &'a mut MiddlewareContext<'a, S, E>) -> Pin<Box<dyn Future<Output = Result<(), MiddlewareError>> + Send + 'a>>;

    fn priority(&self) -> MiddlewarePriority {
        MiddlewarePriority::Normal
    }

    fn should_process(&self, ctx: &MiddlewareContext<S, E>) -> bool {
        true
    }
}
```

## Error Handling

### Middleware Errors
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum MiddlewareError {
    #[error("Middleware '{middleware}' failed: {message}")]
    MiddlewareFailed { middleware: String, message: String },

    #[error("Middleware cancelled operation")]
    Cancelled,

    #[error("Middleware timeout after {duration:?}")]
    Timeout { duration: Duration },

    #[error("Invalid middleware configuration: {message}")]
    ConfigurationError { message: String },

    #[error("Middleware dependency not satisfied: {dependency}")]
    DependencyError { dependency: String },
}
```

### Error Recovery
```rust
impl<S: State, E: Event> MiddlewareStack<S, E> {
    pub fn process_with_recovery(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        let mut last_error = None;

        for middleware in &self.middlewares {
            if middleware.should_process(ctx) {
                match middleware.process(ctx) {
                    Ok(()) => {
                        if !ctx.should_continue {
                            break;
                        }
                    }
                    Err(e) => {
                        last_error = Some(e);
                        // Continue with other middleware or handle error
                        match e {
                            MiddlewareError::Cancelled => {
                                ctx.should_continue = false;
                                break;
                            }
                            _ => {
                                // Log error and continue
                                log::error!("Middleware '{}' failed: {:?}", middleware.name(), e);
                            }
                        }
                    }
                }
            }
        }

        last_error.map_or(Ok(()), Err)
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn middleware_stack_execution_order() {
    let mut stack = MiddlewareStack::<TestState, TestEvent>::new();

    let mut execution_order = Vec::new();

    let middleware1 = TestMiddleware::new("first", |ctx| {
        execution_order.push("first");
        Ok(())
    });

    let middleware2 = TestMiddleware::new("second", |ctx| {
        execution_order.push("second");
        Ok(())
    });

    stack = stack.add(middleware1).add(middleware2);

    let mut ctx = MiddlewareContext {
        operation: Operation::StoreInit { initial_state: &TestState::default() },
        metadata: HashMap::new(),
        should_continue: true,
    };

    stack.process(&mut ctx).unwrap();
    assert_eq!(execution_order, vec!["first", "second"]);
}

#[test]
fn middleware_can_cancel_operation() {
    let mut stack = MiddlewareStack::<TestState, TestEvent>::new();

    let cancelling_middleware = TestMiddleware::new("canceller", |ctx| {
        ctx.should_continue = false;
        Ok(())
    });

    let should_not_run = TestMiddleware::new("should_not_run", |_| {
        panic!("This middleware should not run");
    });

    stack = stack.add(cancelling_middleware).add(should_not_run);

    let mut ctx = MiddlewareContext {
        operation: Operation::StoreInit { initial_state: &TestState::default() },
        metadata: HashMap::new(),
        should_continue: true,
    };

    stack.process(&mut ctx).unwrap();
    assert!(!ctx.should_continue);
}
```

### Integration Tests
```rust
#[test]
fn store_with_logging_middleware() {
    let mut store = Store::new(TestState { count: 0 });
    store = store.with_middleware(LoggingMiddleware::new());

    // This should log the state change
    store.update_with_middleware(|s| s.count = 42).unwrap();

    // Verify state was updated
    assert_eq!(store.get().get_untracked().count, 42);
}

#[test]
fn machine_with_validation_middleware() {
    let mut machine = Machine::new("idle", TestContext::default());
    machine = machine.with_middleware(ValidationMiddleware::new());

    // Add a state
    let idle_state = StateNode::new().on(TestEvent::Start, "running");
    machine.add_state("idle", idle_state);

    // This should validate the transition
    machine.send_with_middleware(TestEvent::Start).unwrap();
    assert_eq!(machine.current_state(), "running");
}
```

### Property-Based Testing
```rust
proptest! {
    #[test]
    fn middleware_stack_idempotent(state: TestState, middlewares: Vec<TestMiddleware>) {
        let mut stack = MiddlewareStack::new();
        for middleware in middlewares {
            stack = stack.add(middleware);
        }

        let mut ctx1 = MiddlewareContext {
            operation: Operation::StoreInit { initial_state: &state },
            metadata: HashMap::new(),
            should_continue: true,
        };

        let mut ctx2 = ctx1.clone();

        // Processing multiple times should be idempotent
        stack.process(&mut ctx1).unwrap();
        stack.process(&mut ctx2).unwrap();

        // Results should be the same
        prop_assert_eq!(ctx1.should_continue, ctx2.should_continue);
        prop_assert_eq!(ctx1.metadata, ctx2.metadata);
    }
}
```

## Performance Impact

### Execution Overhead
- **Per-operation**: Middleware stack processing adds overhead
- **Linear Scaling**: Overhead increases with number of middleware
- **Conditional Processing**: `should_process` can skip unnecessary work

### Optimization Strategies
```rust
impl<S: State, E: Event> MiddlewareStack<S, E> {
    pub fn optimize(&mut self) {
        // Remove no-op middleware
        self.middlewares.retain(|m| {
            // Check if middleware actually does anything
            true // Placeholder
        });

        // Coalesce similar middleware
        // Reorder for better performance
    }

    pub fn with_caching(&mut self, cache_size: usize) -> CachedMiddlewareStack<S, E> {
        // Cache middleware results when possible
        todo!()
    }
}
```

## Security Considerations

### Middleware Isolation
- Middleware should not interfere with each other
- Context mutations should be controlled
- Access to sensitive data should be restricted

### Safe Middleware Execution
```rust
impl<S: State, E: Event> MiddlewareStack<S, E> {
    pub fn process_isolated(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        for middleware in &self.middlewares {
            if middleware.should_process(ctx) {
                // Execute in isolation to prevent interference
                std::panic::catch_unwind(|| {
                    middleware.process(ctx)
                }).map_err(|_| MiddlewareError::MiddlewarePanic {
                    middleware: middleware.name().to_string(),
                })?;
            }
        }
        Ok(())
    }
}
```

### Access Control
```rust
pub trait AccessControlledMiddleware<S: State, E: Event>: Middleware<S, E> {
    fn required_permissions(&self) -> Vec<String>;

    fn check_permissions(&self, user_permissions: &[String]) -> bool {
        self.required_permissions().iter()
            .all(|required| user_permissions.contains(required))
    }
}
```

## Future Extensions

### Middleware Dependencies
```rust
pub trait DependentMiddleware<S: State, E: Event>: Middleware<S, E> {
    fn dependencies(&self) -> Vec<String>;

    fn init_with_dependencies(&mut self, available_middleware: &[String]) -> Result<(), MiddlewareError> {
        for dep in self.dependencies() {
            if !available_middleware.contains(&dep.to_string()) {
                return Err(MiddlewareError::DependencyError {
                    dependency: dep.to_string(),
                });
            }
        }
        Ok(())
    }
}
```

### Dynamic Middleware Loading
```rust
#[cfg(feature = "dynamic-middleware")]
pub struct DynamicMiddlewareLoader<S: State, E: Event> {
    loaded_middleware: HashMap<String, Box<dyn Middleware<S, E>>>,
}

#[cfg(feature = "dynamic-middleware")]
impl<S: State, E: Event> DynamicMiddlewareLoader<S, E> {
    pub async fn load_from_url(&mut self, url: &str) -> Result<(), MiddlewareError> {
        // Load middleware from URL (WASM plugin system)
        // This would require a plugin architecture
        todo!()
    }

    pub fn unload(&mut self, name: &str) {
        self.loaded_middleware.remove(name);
    }
}
```

### Middleware Metrics
```rust
#[derive(Clone, Debug)]
pub struct MiddlewareMetrics {
    pub name: String,
    pub execution_count: u64,
    pub total_execution_time: Duration,
    pub error_count: u64,
    pub last_execution: Option<DateTime<Utc>>,
}

impl<S: State, E: Event> MiddlewareStack<S, E> {
    pub fn collect_metrics(&self) -> Vec<MiddlewareMetrics> {
        // Collect performance metrics from middleware
        todo!()
    }

    pub fn with_metrics(mut self, enable: bool) -> Self {
        // Enable metrics collection
        todo!()
    }
}
```

## Migration Guide

### Adding Middleware to Existing Code
```rust
// Before - no middleware
let store = Store::new(initial_state);
store.update(|s| s.count += 1).unwrap();

// After - with middleware
let store = Store::new(initial_state)
    .with_middleware(LoggingMiddleware::new())
    .with_middleware(ValidationMiddleware::new());

store.update_with_middleware(|s| s.count += 1).unwrap();
```

### Gradual Adoption
```rust
// Phase 1: Add optional middleware
pub fn create_store_with_middleware<S: State>(initial: S, enable_middleware: bool) -> Store<S> {
    let store = Store::new(initial);

    if enable_middleware {
        store.with_middleware(LoggingMiddleware::new())
    } else {
        store
    }
}

// Phase 2: Make middleware default
pub fn create_store<S: State>(initial: S) -> Store<S> {
    Store::new(initial)
        .with_middleware(LoggingMiddleware::new())
        .with_middleware(ValidationMiddleware::new())
}
```

### Middleware Configuration
```rust
pub struct MiddlewareConfig {
    pub enable_logging: bool,
    pub enable_validation: bool,
    pub enable_caching: bool,
    pub log_level: LogLevel,
}

pub fn create_store_with_config<S: State>(initial: S, config: &MiddlewareConfig) -> Store<S> {
    let mut store = Store::new(initial);

    if config.enable_logging {
        store = store.with_middleware(LoggingMiddleware::with_level(config.log_level));
    }

    if config.enable_validation {
        store = store.with_middleware(ValidationMiddleware::new());
    }

    if config.enable_caching {
        store = store.with_middleware(CachingMiddleware::new());
    }

    store
}
```

## Risk Assessment

### Likelihood: Medium
- Middleware execution order dependencies
- Complex interactions between middleware
- Performance impact of middleware stack

### Impact: Medium
- Middleware can affect application performance
- Complex debugging when multiple middleware interact
- Potential for middleware to break application logic

### Mitigation
- Clear middleware execution order guarantees
- Comprehensive testing of middleware combinations
- Performance monitoring and optimization
- Middleware isolation and error boundaries
- Documentation of middleware interactions
- Opt-in middleware adoption
