# Validation Middleware Design

## Overview
Implement validation middleware to enforce business rules, data constraints, and state invariants during state operations and transitions.

## Current State
```rust
// Manual validation only
impl<S: State> Store<S> {
    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        let mut new_state = self.signal.get_untracked();
        updater(&mut new_state);

        // Manual validation would go here
        if new_state.is_valid() {
            self.signal.set(new_state);
            Ok(())
        } else {
            Err(StoreError::ValidationError("Invalid state".to_string()))
        }
    }
}
```

## Proposed Enhancement
```rust
pub struct ValidationMiddleware<S: State, E: Event = ()> {
    rules: Vec<Box<dyn ValidationRule<S, E> + Send + Sync>>,
    on_failure: ValidationFailureAction,
}

pub trait ValidationRule<S: State, E: Event> {
    fn name(&self) -> &'static str;
    fn validate(&self, ctx: &MiddlewareContext<S, E>) -> ValidationResult;
}

impl<S: State> Store<S> {
    pub fn with_validation(self, rules: Vec<Box<dyn ValidationRule<S> + Send + Sync>>) -> Self {
        self.with_middleware(ValidationMiddleware::new(rules))
    }
}
```

## Motivation

### Data Integrity
- **Business Rules**: Enforce domain-specific constraints
- **State Invariants**: Maintain valid state at all times
- **Input Validation**: Prevent invalid data from entering the system
- **Consistency**: Ensure state transitions maintain consistency

### Error Prevention
- **Fail Fast**: Catch invalid operations early
- **User Feedback**: Provide clear error messages for invalid actions
- **System Stability**: Prevent cascading failures from invalid state
- **Debugging**: Isolate validation issues from business logic

### Use Cases
- Form validation before state updates
- Business rule enforcement (account balances, permissions)
- Data type and range validation
- Referential integrity checks
- State machine transition preconditions
- API input validation

## Implementation Details

### Validation Framework
```rust
#[derive(Clone, Debug)]
pub enum ValidationResult {
    Valid,
    Invalid { reason: String, code: Option<String> },
}

#[derive(Clone, Debug)]
pub enum ValidationFailureAction {
    Cancel,                    // Cancel the operation
    Warn,                     // Log warning but continue
    Error(String),            // Custom error message
    Transform(Box<dyn Fn(&mut dyn Any) + Send + Sync>), // Transform the data
}

pub trait ValidationRule<S: State, E: Event> {
    fn name(&self) -> &'static str;
    fn validate(&self, ctx: &MiddlewareContext<S, E>) -> ValidationResult;
    fn priority(&self) -> ValidationPriority {
        ValidationPriority::Normal
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum ValidationPriority {
    Highest = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Lowest = 4,
}

pub struct ValidationMiddleware<S: State, E: Event = ()> {
    rules: Vec<Box<dyn ValidationRule<S, E> + Send + Sync>>,
    on_failure: ValidationFailureAction,
    stop_on_first_failure: bool,
}

impl<S: State, E: Event> ValidationMiddleware<S, E> {
    pub fn new(rules: Vec<Box<dyn ValidationRule<S, E> + Send + Sync>>) -> Self {
        Self {
            rules,
            on_failure: ValidationFailureAction::Cancel,
            stop_on_first_failure: true,
        }
    }

    pub fn with_failure_action(mut self, action: ValidationFailureAction) -> Self {
        self.on_failure = action;
        self
    }

    pub fn continue_on_failure(mut self, continue_on_failure: bool) -> Self {
        self.stop_on_first_failure = !continue_on_failure;
        self
    }

    fn validate_all(&self, ctx: &MiddlewareContext<S, E>) -> ValidationResult {
        let mut failures = Vec::new();

        for rule in &self.rules {
            match rule.validate(ctx) {
                ValidationResult::Valid => continue,
                ValidationResult::Invalid { reason, code } => {
                    failures.push((rule.name(), reason, code));

                    if self.stop_on_first_failure {
                        break;
                    }
                }
            }
        }

        if failures.is_empty() {
            ValidationResult::Valid
        } else if failures.len() == 1 {
            let (rule, reason, code) = failures.into_iter().next().unwrap();
            ValidationResult::Invalid {
                reason: format!("{}: {}", rule, reason),
                code,
            }
        } else {
            let reasons = failures.into_iter()
                .map(|(rule, reason, _)| format!("{}: {}", rule, reason))
                .collect::<Vec<_>>()
                .join("; ");
            ValidationResult::Invalid {
                reason: format!("Multiple validation failures: {}", reasons),
                code: Some("MULTIPLE_VALIDATION_FAILURES".to_string()),
            }
        }
    }
}

impl<S: State, E: Event> Middleware<S, E> for ValidationMiddleware<S, E> {
    fn name(&self) -> &'static str {
        "validation"
    }

    fn process(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        match self.validate_all(ctx) {
            ValidationResult::Valid => Ok(()),
            ValidationResult::Invalid { reason, code } => {
                match &self.on_failure {
                    ValidationFailureAction::Cancel => {
                        ctx.should_continue = false;
                        Err(MiddlewareError::ValidationFailed { reason, code })
                    }
                    ValidationFailureAction::Warn => {
                        log::warn!("Validation failed but continuing: {}", reason);
                        Ok(())
                    }
                    ValidationFailureAction::Error(custom_msg) => {
                        ctx.should_continue = false;
                        Err(MiddlewareError::ValidationFailed {
                            reason: custom_msg.clone(),
                            code,
                        })
                    }
                    ValidationFailureAction::Transform(transform_fn) => {
                        // Apply transformation to fix the data
                        self.apply_transformation(ctx, transform_fn)?;
                        Ok(())
                    }
                }
            }
        }
    }

    fn apply_transformation(
        &self,
        ctx: &mut MiddlewareContext<S, E>,
        transform_fn: &Box<dyn Fn(&mut dyn Any) + Send + Sync>
    ) -> Result<(), MiddlewareError> {
        match &mut ctx.operation {
            Operation::StoreUpdate { new_state, .. } => {
                let state_any = new_state as &mut dyn Any;
                transform_fn(state_any);
                Ok(())
            }
            Operation::MachineTransition { machine, .. } => {
                let context_any = machine.context_mut() as &mut dyn Any;
                transform_fn(context_any);
                Ok(())
            }
            _ => Err(MiddlewareError::UnsupportedTransformation),
        }
    }
}
```

### Built-in Validation Rules
```rust
pub struct RangeValidation<S, F> {
    field_extractor: F,
    min: Option<i64>,
    max: Option<i64>,
    field_name: String,
}

impl<S, F> RangeValidation<S, F>
where
    F: Fn(&S) -> i64 + Send + Sync,
    S: State,
{
    pub fn new(field_name: String, field_extractor: F) -> Self {
        Self {
            field_extractor,
            min: None,
            max: None,
            field_name,
        }
    }

    pub fn min(mut self, min: i64) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: i64) -> Self {
        self.max = Some(max);
        self
    }
}

impl<S, F, E> ValidationRule<S, E> for RangeValidation<S, F>
where
    F: Fn(&S) -> i64 + Send + Sync,
    S: State,
    E: Event,
{
    fn name(&self) -> &'static str {
        "range_validation"
    }

    fn validate(&self, ctx: &MiddlewareContext<S, E>) -> ValidationResult {
        let value = match &ctx.operation {
            Operation::StoreUpdate { new_state, .. } => (self.field_extractor)(new_state),
            Operation::MachineTransition { machine, .. } => (self.field_extractor)(machine.context()),
            _ => return ValidationResult::Valid, // Skip for other operations
        };

        if let Some(min) = self.min {
            if value < min {
                return ValidationResult::Invalid {
                    reason: format!("{} value {} is below minimum {}", self.field_name, value, min),
                    code: Some("BELOW_MINIMUM".to_string()),
                };
            }
        }

        if let Some(max) = self.max {
            if value > max {
                return ValidationResult::Invalid {
                    reason: format!("{} value {} is above maximum {}", self.field_name, value, max),
                    code: Some("ABOVE_MAXIMUM".to_string()),
                };
            }
        }

        ValidationResult::Valid
    }
}

pub struct RequiredFieldValidation<S, F> {
    field_extractor: F,
    field_name: String,
}

impl<S, F> RequiredFieldValidation<S, F>
where
    F: Fn(&S) -> bool + Send + Sync, // Returns true if field is present/valid
    S: State,
{
    pub fn new(field_name: String, field_extractor: F) -> Self {
        Self { field_extractor, field_name }
    }
}

impl<S, F, E> ValidationRule<S, E> for RequiredFieldValidation<S, F>
where
    F: Fn(&S) -> bool + Send + Sync,
    S: State,
    E: Event,
{
    fn name(&self) -> &'static str {
        "required_field"
    }

    fn validate(&self, ctx: &MiddlewareContext<S, E>) -> ValidationResult {
        let is_present = match &ctx.operation {
            Operation::StoreUpdate { new_state, .. } => (self.field_extractor)(new_state),
            Operation::MachineTransition { machine, .. } => (self.field_extractor)(machine.context()),
            _ => return ValidationResult::Valid,
        };

        if !is_present {
            ValidationResult::Invalid {
                reason: format!("Required field '{}' is missing or invalid", self.field_name),
                code: Some("REQUIRED_FIELD_MISSING".to_string()),
            }
        } else {
            ValidationResult::Valid
        }
    }
}

pub struct StateMachineTransitionValidation<E> {
    valid_transitions: HashMap<String, Vec<String>>,
}

impl<E> StateMachineTransitionValidation<E> {
    pub fn new(valid_transitions: HashMap<String, Vec<String>>) -> Self {
        Self { valid_transitions }
    }

    pub fn allow_transition(mut self, from: &str, to: &str) -> Self {
        self.valid_transitions.entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
        self
    }
}

impl<S, E> ValidationRule<S, E> for StateMachineTransitionValidation<E>
where
    S: State,
    E: Event,
{
    fn name(&self) -> &'static str {
        "state_machine_transition"
    }

    fn validate(&self, ctx: &MiddlewareContext<S, E>) -> ValidationResult {
        if let Operation::MachineTransition { machine, transition, .. } = &ctx.operation {
            let current_state = machine.current_state();

            if let Some(valid_targets) = self.valid_transitions.get(current_state) {
                if !valid_targets.contains(&transition.target) {
                    return ValidationResult::Invalid {
                        reason: format!(
                            "Invalid transition from '{}' to '{}'",
                            current_state,
                            transition.target
                        ),
                        code: Some("INVALID_TRANSITION".to_string()),
                    };
                }
            }
        }

        ValidationResult::Valid
    }
}
```

### Custom Validation Rules
```rust
pub struct CustomValidation<S, E, F> {
    name: String,
    validator: F,
}

impl<S, E, F> CustomValidation<S, E, F>
where
    F: Fn(&MiddlewareContext<S, E>) -> ValidationResult + Send + Sync,
    S: State,
    E: Event,
{
    pub fn new(name: impl Into<String>, validator: F) -> Self {
        Self {
            name: name.into(),
            validator,
        }
    }
}

impl<S, E, F> ValidationRule<S, E> for CustomValidation<S, E, F>
where
    F: Fn(&MiddlewareContext<S, E>) -> ValidationResult + Send + Sync,
    S: State,
    E: Event,
{
    fn name(&self) -> &'static str {
        &self.name
    }

    fn validate(&self, ctx: &MiddlewareContext<S, E>) -> ValidationResult {
        (self.validator)(ctx)
    }
}

// Usage example
let age_validation = CustomValidation::new("age_validation", |ctx| {
    match &ctx.operation {
        Operation::StoreUpdate { new_state, .. } => {
            // Assume state has age field
            if new_state.age < 0 {
                ValidationResult::Invalid {
                    reason: "Age cannot be negative".to_string(),
                    code: Some("NEGATIVE_AGE".to_string()),
                }
            } else if new_state.age > 150 {
                ValidationResult::Invalid {
                    reason: "Age cannot be greater than 150".to_string(),
                    code: Some("AGE_TOO_HIGH".to_string()),
                }
            } else {
                ValidationResult::Valid
            }
        }
        _ => ValidationResult::Valid,
    }
});
```

## Error Handling

### Validation Error Types
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum MiddlewareError {
    #[error("Validation failed: {reason}")]
    ValidationFailed { reason: String, code: Option<String> },

    #[error("Validation transformation failed")]
    TransformationFailed,

    #[error("Unsupported operation for transformation")]
    UnsupportedTransformation,

    #[error("Validation rule error: {message}")]
    RuleError { message: String },
}
```

### Recovery Strategies
```rust
impl<S: State, E: Event> ValidationMiddleware<S, E> {
    pub fn with_recovery_strategy(
        mut self,
        strategy: ValidationRecoveryStrategy
    ) -> Self {
        self.recovery_strategy = strategy;
        self
    }

    fn attempt_recovery(
        &self,
        ctx: &mut MiddlewareContext<S, E>,
        failure: &ValidationResult
    ) -> Result<(), MiddlewareError> {
        match &self.recovery_strategy {
            ValidationRecoveryStrategy::None => Err(MiddlewareError::ValidationFailed {
                reason: "Validation failed".to_string(),
                code: None,
            }),
            ValidationRecoveryStrategy::ClampValues => {
                self.clamp_invalid_values(ctx)?;
                Ok(())
            }
            ValidationRecoveryStrategy::UseDefaults => {
                self.apply_default_values(ctx)?;
                Ok(())
            }
            ValidationRecoveryStrategy::Custom(recovery_fn) => {
                recovery_fn(ctx, failure)?;
                Ok(())
            }
        }
    }

    fn clamp_invalid_values(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        // Implement value clamping logic
        match &mut ctx.operation {
            Operation::StoreUpdate { new_state, .. } => {
                // Clamp age between 0 and 150
                new_state.age = new_state.age.max(0).min(150);
                Ok(())
            }
            _ => Err(MiddlewareError::UnsupportedTransformation),
        }
    }

    fn apply_default_values(&self, ctx: &mut MiddlewareContext<S, E>) -> Result<(), MiddlewareError> {
        // Apply safe default values
        match &mut ctx.operation {
            Operation::StoreUpdate { new_state, .. } => {
                if new_state.age < 0 {
                    new_state.age = 18; // Default age
                }
                Ok(())
            }
            _ => Err(MiddlewareError::UnsupportedTransformation),
        }
    }
}

#[derive(Clone)]
pub enum ValidationRecoveryStrategy {
    None,
    ClampValues,
    UseDefaults,
    Custom(Box<dyn Fn(&mut MiddlewareContext<dyn State, dyn Event>, &ValidationResult) -> Result<(), MiddlewareError> + Send + Sync>),
}
```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn range_validation_works() {
    let validation = RangeValidation::new("count".to_string(), |s: &TestState| s.count as i64)
        .min(0)
        .max(100);

    let ctx = MiddlewareContext {
        operation: Operation::StoreUpdate {
            old_state: &TestState { count: 50 },
            new_state: &mut TestState { count: 150 },
            updater: &|_| {},
        },
        metadata: HashMap::new(),
        should_continue: true,
    };

    let result = validation.validate(&ctx);
    match result {
        ValidationResult::Invalid { reason, .. } => {
            assert!(reason.contains("above maximum"));
        }
        _ => panic!("Expected validation failure"),
    }
}

#[test]
fn validation_middleware_cancels_on_failure() {
    let rules: Vec<Box<dyn ValidationRule<TestState, TestEvent> + Send + Sync>> = vec![
        Box::new(RangeValidation::new("count".to_string(), |s| s.count as i64).max(10)),
    ];

    let middleware = ValidationMiddleware::new(rules);

    let mut ctx = MiddlewareContext {
        operation: Operation::StoreUpdate {
            old_state: &TestState { count: 5 },
            new_state: &mut TestState { count: 20 },
            updater: &|_| {},
        },
        metadata: HashMap::new(),
        should_continue: true,
    };

    let result = middleware.process(&mut ctx);
    assert!(matches!(result, Err(MiddlewareError::ValidationFailed { .. })));
    assert!(!ctx.should_continue);
}
```

### Integration Tests
```rust
#[test]
fn store_with_validation_middleware() {
    let rules: Vec<Box<dyn ValidationRule<TestState, TestEvent> + Send + Sync>> = vec![
        Box::new(RangeValidation::new("count".to_string(), |s| s.count as i64).min(0).max(10)),
    ];

    let mut store = Store::new(TestState { count: 5 });
    store = store.with_middleware(ValidationMiddleware::new(rules));

    // Valid update should succeed
    store.update_with_middleware(|s| s.count = 8).unwrap();
    assert_eq!(store.get().get_untracked().count, 8);

    // Invalid update should fail
    let result = store.update_with_middleware(|s| s.count = 15);
    assert!(matches!(result, Err(StoreError::MiddlewareError(MiddlewareError::ValidationFailed { .. }))));
    // State should remain unchanged
    assert_eq!(store.get().get_untracked().count, 8);
}
```

### Property-Based Testing
```rust
proptest! {
    #[test]
    fn range_validation_properties(min in 0..50, max in 51..100, value in 0..150) {
        let validation = RangeValidation::new("test".to_string(), move |_| value as i64)
            .min(min)
            .max(max);

        let ctx = MiddlewareContext {
            operation: Operation::StoreUpdate {
                old_state: &TestState { count: 0 },
                new_state: &mut TestState { count: value },
                updater: &|_| {},
            },
            metadata: HashMap::new(),
            should_continue: true,
        };

        let result = validation.validate(&ctx);

        if value < min || value > max {
            prop_assert!(matches!(result, ValidationResult::Invalid { .. }));
        } else {
            prop_assert!(matches!(result, ValidationResult::Valid));
        }
    }
}
```

## Performance Impact

### Validation Overhead
- **Per-operation**: Validation rules execute on each operation
- **Linear Scaling**: Overhead increases with number of rules
- **Early Exit**: Can stop on first failure to improve performance
- **Caching**: Can cache expensive validation results

### Optimization Strategies
```rust
impl<S: State, E: Event> ValidationMiddleware<S, E> {
    pub fn with_caching(mut self, cache_size: usize) -> Self {
        // Cache validation results for repeated operations
        self.caching_enabled = true;
        self.cache = Some(LruCache::new(cache_size));
        self
    }

    pub fn optimized_for_performance(mut self) -> Self {
        // Reorder rules by priority for early failure
        self.rules.sort_by_key(|r| r.priority());
        self.stop_on_first_failure = true;
        self
    }

    fn get_cache_key(&self, ctx: &MiddlewareContext<S, E>) -> Option<u64> {
        // Generate cache key from operation (if possible)
        match &ctx.operation {
            Operation::StoreUpdate { new_state, .. } => {
                Some(seahash::hash(&bincode::serialize(new_state).ok()?))
            }
            _ => None,
        }
    }
}
```

## Security Considerations

### Input Validation
- Prevent injection attacks through validation rules
- Validate all user inputs before processing
- Use safe parsing and validation libraries

### Information Disclosure
- Validation error messages should not leak sensitive information
- Use generic error codes for external APIs
- Log detailed errors internally but return sanitized versions

```rust
impl ValidationMiddleware<UserState, UserEvent> {
    pub fn sanitize_error(&self, error: &MiddlewareError) -> MiddlewareError {
        match error {
            MiddlewareError::ValidationFailed { reason, code } => {
                // Remove sensitive information from reason
                let sanitized_reason = if reason.contains("password") {
                    "Invalid credentials".to_string()
                } else {
                    reason.clone()
                };

                MiddlewareError::ValidationFailed {
                    reason: sanitized_reason,
                    code: code.clone(),
                }
            }
            other => other.clone(),
        }
    }
}
```

### Rate Limiting
```rust
pub struct RateLimitedValidation<S: State, E: Event> {
    inner: Box<dyn ValidationRule<S, E>>,
    limiter: Arc<Mutex<RateLimiter>>,
}

impl<S: State, E: Event> RateLimitedValidation<S, E> {
    pub fn new(inner: Box<dyn ValidationRule<S, E>>, requests_per_second: u32) -> Self {
        Self {
            inner,
            limiter: Arc::new(Mutex::new(RateLimiter::new(requests_per_second))),
        }
    }
}

impl<S: State, E: Event> ValidationRule<S, E> for RateLimitedValidation<S, E> {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn validate(&self, ctx: &MiddlewareContext<S, E>) -> ValidationResult {
        if !self.limiter.lock().unwrap().check() {
            return ValidationResult::Invalid {
                reason: "Rate limit exceeded".to_string(),
                code: Some("RATE_LIMIT_EXCEEDED".to_string()),
            };
        }

        self.inner.validate(ctx)
    }
}
```

## Future Extensions

### Asynchronous Validation
```rust
#[cfg(feature = "async-validation")]
pub trait AsyncValidationRule<S: State, E: Event> {
    fn name(&self) -> &'static str;

    fn validate_async<'a>(
        &'a self,
        ctx: &'a MiddlewareContext<S, E>
    ) -> Pin<Box<dyn Future<Output = ValidationResult> + Send + 'a>>;
}
```

### Schema-Based Validation
```rust
#[cfg(feature = "schema-validation")]
pub struct SchemaValidation<S: State, E: Event> {
    schema: jsonschema::JSONSchema,
}

#[cfg(feature = "schema-validation")]
impl<S: State, E: Event> SchemaValidation<S, E> {
    pub fn from_json_schema(schema_json: &str) -> Result<Self, ValidationError> {
        let schema: serde_json::Value = serde_json::from_str(schema_json)?;
        let compiled = jsonschema::JSONSchema::compile(&schema)?;
        Ok(Self { schema: compiled })
    }
}

#[cfg(feature = "schema-validation")]
impl<S: State, E: Event> ValidationRule<S, E> for SchemaValidation<S, E>
where
    S: Serialize,
{
    fn validate(&self, ctx: &MiddlewareContext<S, E>) -> ValidationResult {
        let value = match serde_json::to_value(ctx) {
            Ok(v) => v,
            Err(_) => return ValidationResult::Invalid {
                reason: "Failed to serialize state for validation".to_string(),
                code: Some("SERIALIZATION_FAILED".to_string()),
            },
        };

        if let Err(errors) = self.schema.validate(&value) {
            let reasons: Vec<String> = errors.map(|e| e.to_string()).collect();
            ValidationResult::Invalid {
                reason: format!("Schema validation failed: {}", reasons.join(", ")),
                code: Some("SCHEMA_VALIDATION_FAILED".to_string()),
            }
        } else {
            ValidationResult::Valid
        }
    }
}
```

### Validation Rules as Code
```rust
pub fn age_validation() -> Box<dyn ValidationRule<PersonState, PersonEvent>> {
    Box::new(CustomValidation::new("age_must_be_positive", |ctx| {
        match &ctx.operation {
            Operation::StoreUpdate { new_state, .. } => {
                if new_state.age < 0 {
                    ValidationResult::Invalid {
                        reason: "Age cannot be negative".to_string(),
                        code: Some("NEGATIVE_AGE".to_string()),
                    }
                } else {
                    ValidationResult::Valid
                }
            }
            _ => ValidationResult::Valid,
        }
    }))
}

pub fn required_fields_validation() -> Box<dyn ValidationRule<PersonState, PersonEvent>> {
    Box::new(CompositeValidation::new("required_fields", vec![
        Box::new(RequiredFieldValidation::new("name".to_string(), |s| !s.name.is_empty())),
        Box::new(RequiredFieldValidation::new("email".to_string(), |s| !s.email.is_empty())),
    ]))
}
```

## Migration Guide

### Adding Validation to Existing Code
```rust
// Before - no validation
let store = Store::new(initial_state);

// After - with validation
let rules: Vec<Box<dyn ValidationRule<MyState> + Send + Sync>> = vec![
    Box::new(RangeValidation::new("count".to_string(), |s| s.count as i64).min(0))),
    Box::new(CustomValidation::new("custom_rule", |ctx| {
        // Custom validation logic
        ValidationResult::Valid
    })),
];

let store = Store::new(initial_state)
    .with_middleware(ValidationMiddleware::new(rules));
```

### Gradual Adoption
```rust
// Phase 1: Add basic validation that warns but doesn't block
let store = Store::new(initial_state)
    .with_middleware(ValidationMiddleware::new(rules)
        .with_failure_action(ValidationFailureAction::Warn));

// Phase 2: Make validation strict
let store = Store::new(initial_state)
    .with_middleware(ValidationMiddleware::new(rules)
        .with_failure_action(ValidationFailureAction::Cancel));
```

### Configuration-Based Validation
```rust
#[derive(Deserialize)]
pub struct ValidationConfig {
    pub strict_mode: bool,
    pub max_age: Option<i32>,
    pub required_fields: Vec<String>,
}

pub fn create_store_with_validation<S: State>(
    initial: S,
    config: &ValidationConfig
) -> Store<S> {
    let mut rules: Vec<Box<dyn ValidationRule<S> + Send + Sync>> = Vec::new();

    if let Some(max_age) = config.max_age {
        rules.push(Box::new(RangeValidation::new("age".to_string(), |s| s.age as i64).max(max_age as i64)));
    }

    for field in &config.required_fields {
        rules.push(Box::new(RequiredFieldValidation::new(
            field.clone(),
            move |s| !get_field_by_name(s, field).is_empty()
        )));
    }

    let failure_action = if config.strict_mode {
        ValidationFailureAction::Cancel
    } else {
        ValidationFailureAction::Warn
    };

    Store::new(initial)
        .with_middleware(ValidationMiddleware::new(rules)
            .with_failure_action(failure_action))
}
```

## Risk Assessment

### Likelihood: Medium
- Validation rules can have bugs or edge cases
- Complex business rules may be difficult to implement correctly
- Performance impact from validation overhead

### Impact: High
- Invalid data can break application functionality
- Validation failures can prevent valid operations
- Complex validation logic can be hard to debug

### Mitigation
- Comprehensive testing of validation rules
- Clear error messages and recovery strategies
- Performance monitoring and optimization
- Gradual adoption with warning modes first
- Extensive documentation and examples
- Validation rule composition and reuse
