# Guard Conditions Design

## Overview
Implement guard conditions for state machine transitions to enable conditional state changes based on context, enabling more sophisticated state machine logic.

## Current State
```rust
#[derive(Clone)]
pub struct Transition<S: State, E: Event> {
    pub target: String,
    pub actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,
}
```

## Proposed Enhancement
```rust
#[derive(Clone)]
pub struct Transition<S: State, E: Event> {
    pub target: String,
    pub guard: Option<Box<dyn Fn(&S, &E) -> bool + Send + Sync>>,
    pub actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,
}
```

## Motivation

### Conditional Logic
- **Context-Aware Transitions**: Only transition when conditions are met
- **Validation**: Prevent invalid state changes
- **Business Rules**: Enforce application-specific constraints
- **Error Prevention**: Avoid transitions that would leave system in invalid state

### Use Cases
- User authentication (only allow certain transitions when logged in)
- Resource availability (only transition when resources are ready)
- Time-based conditions (only transition during certain time windows)
- Data validation (only transition when data is in valid state)
- Permission checks (only allow transitions user has permission for)

## Implementation Details

### Guard Function Type
```rust
type GuardFn<S, E> = Box<dyn Fn(&S, &E) -> bool + Send + Sync>;

#[derive(Clone)]
pub struct Transition<S: State, E: Event> {
    pub target: String,
    pub guard: Option<GuardFn<S, E>>,
    pub actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,
}
```

### Builder Pattern
```rust
impl<S: State, E: Event> Transition<S, E> {
    pub fn new(target: &str) -> Self {
        Self {
            target: target.to_string(),
            guard: None,
            actions: None,
        }
    }

    pub fn with_guard<F>(mut self, guard: F) -> Self
    where
        F: Fn(&S, &E) -> bool + Send + Sync + 'static,
    {
        self.guard = Some(Box::new(guard));
        self
    }

    pub fn with_actions<F>(mut self, actions: Vec<F>) -> Self
    where
        F: Fn(&mut S, &E) + Send + Sync + 'static,
    {
        self.actions = Some(actions.into_iter()
            .map(|a| Box::new(a) as Box<dyn Fn(&mut S, &E) + Send + Sync>)
            .collect());
        self
    }
}
```

### Fluent State Node API
```rust
impl<S: State, E: Event> StateNode<S, E> {
    pub fn on_guarded<F>(mut self, event: E, target: &str, guard: F) -> Self
    where
        F: Fn(&S, &E) -> bool + Send + Sync + 'static,
    {
        let transition = Transition::new(target).with_guard(guard);
        self.transitions.insert(event.event_type(), transition);
        self
    }

    pub fn on_guarded_with_actions<F, G>(
        mut self,
        event: E,
        target: &str,
        guard: F,
        actions: Vec<G>
    ) -> Self
    where
        F: Fn(&S, &E) -> bool + Send + Sync + 'static,
        G: Fn(&mut S, &E) + Send + Sync + 'static,
    {
        let transition = Transition::new(target)
            .with_guard(guard)
            .with_actions(actions);
        self.transitions.insert(event.event_type(), transition);
        self
    }
}
```

## API Design

### Machine Integration
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn can_transition(&self, event: &E) -> bool {
        if let Some(current_node) = self.states.get(&self.current_state) {
            if let Some(transition) = current_node.transitions.get(&event.event_type()) {
                // Check guard condition if present
                if let Some(ref guard) = transition.guard {
                    guard(&self.context, event)
                } else {
                    true  // No guard means transition is always allowed
                }
            } else {
                false  // No transition defined for this event
            }
        } else {
            false  // Current state not found
        }
    }

    pub fn send_guarded(&mut self, event: E) -> Result<(), MachineError> {
        if !self.can_transition(&event) {
            return Err(MachineError::GuardFailed {
                state: self.current_state.clone(),
                event: event.event_type(),
            });
        }

        self.send(event)
    }

    pub fn get_available_transitions(&self) -> Vec<(String, bool)> {
        if let Some(current_node) = self.states.get(&self.current_state) {
            current_node.transitions.iter()
                .map(|(event_type, transition)| {
                    let can_transition = if let Some(ref guard) = transition.guard {
                        guard(&self.context, &E::default())  // Use default event for checking
                    } else {
                        true
                    };
                    (event_type.clone(), can_transition)
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}
```

### Example Usage
```rust
#[derive(Clone)]
enum AuthEvent {
    Login { user_id: String },
    Logout,
    AccessResource { resource: String },
}

#[derive(Clone)]
struct AuthContext {
    user_id: Option<String>,
    permissions: Vec<String>,
    login_attempts: u32,
}

let auth_machine = Machine::new("unauthenticated", AuthContext::default());

let authenticated_state = StateNode::new()
    .on_guarded(
        AuthEvent::AccessResource { resource: "admin".to_string() },
        "access_granted",
        |ctx, event| {
            if let AuthEvent::AccessResource { resource } = event {
                ctx.permissions.contains(resource)
            } else {
                false
            }
        }
    )
    .on_guarded(
        AuthEvent::Logout,
        "unauthenticated",
        |ctx, _| ctx.user_id.is_some()  // Only allow logout if logged in
    );

auth_machine.add_state("authenticated", authenticated_state);
```

## Guard Design Patterns

### Context-Based Guards
```rust
// Resource availability guard
let guard = |ctx: &AppContext, _: &AppEvent| {
    ctx.available_resources > 0
};

// Time-based guard
let guard = |ctx: &AppContext, _: &AppEvent| {
    let now = Utc::now();
    now.hour() >= 9 && now.hour() <= 17  // Business hours only
};

// Data validation guard
let guard = |ctx: &FormContext, _: &FormEvent| {
    ctx.is_valid && ctx.required_fields_filled
};
```

### Event-Based Guards
```rust
// Event parameter validation
let guard = |_: &AppContext, event: &PaymentEvent| {
    if let PaymentEvent::Process { amount, .. } = event {
        *amount > 0.0 && *amount <= 10000.0  // Reasonable payment limits
    } else {
        false
    }
};

// User permission guard
let guard = |ctx: &AuthContext, event: &FileEvent| {
    if let FileEvent::Delete { file_id } = event {
        ctx.can_delete_file(file_id)
    } else {
        true  // Allow other file operations
    }
};
```

### Complex Guards
```rust
// Multi-condition guard
let guard = |ctx: &WorkflowContext, event: &WorkflowEvent| {
    // Must be in correct state
    ctx.current_step == "review" &&
    // Must have required permissions
    ctx.user_permissions.contains(&Permission::Approve) &&
    // Must be within deadline
    ctx.deadline.map_or(true, |d| Utc::now() < d) &&
    // Event-specific validation
    if let WorkflowEvent::Approve { comment } = event {
        !comment.is_empty() && comment.len() <= 1000
    } else {
        false
    }
};
```

## Error Handling

### Guard Failure Types
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum MachineError {
    #[error("Guard condition failed for transition from {state} on event {event}")]
    GuardFailed { state: String, event: String },

    #[error("Guard function panicked during evaluation")]
    GuardPanic { state: String, event: String },
}
```

### Safe Guard Evaluation
```rust
impl<S: State, E: Event> Machine<S, E> {
    fn evaluate_guard(&self, guard: &GuardFn<S, E>, event: &E) -> Result<bool, MachineError> {
        std::panic::catch_unwind(|| guard(&self.context, event))
            .map_err(|_| MachineError::GuardPanic {
                state: self.current_state.clone(),
                event: event.event_type(),
            })
    }

    pub fn can_transition_safe(&self, event: &E) -> Result<bool, MachineError> {
        if let Some(current_node) = self.states.get(&self.current_state) {
            if let Some(transition) = current_node.transitions.get(&event.event_type()) {
                if let Some(ref guard) = transition.guard {
                    self.evaluate_guard(guard, event)
                } else {
                    Ok(true)
                }
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn guard_blocks_invalid_transition() {
    let mut machine = Machine::new("idle", TestContext::default());

    let idle_state = StateNode::new()
        .on_guarded(
            StartEvent,
            "running",
            |ctx, _| ctx.resources_available
        );

    machine.add_state("idle", idle_state);

    // Should fail when resources not available
    machine.context_mut().resources_available = false;
    assert!(!machine.can_transition(&StartEvent));

    let result = machine.send_guarded(StartEvent);
    assert!(matches!(result, Err(MachineError::GuardFailed { .. })));
}

#[test]
fn guard_allows_valid_transition() {
    let mut machine = Machine::new("idle", TestContext::default());

    let idle_state = StateNode::new()
        .on_guarded(StartEvent, "running", |ctx, _| ctx.resources_available);

    machine.add_state("idle", idle_state);

    // Should succeed when resources are available
    machine.context_mut().resources_available = true;
    assert!(machine.can_transition(&StartEvent));

    machine.send_guarded(StartEvent).unwrap();
    assert_eq!(machine.current_state(), "running");
}
```

### Property-Based Testing
```rust
proptest! {
    #[test]
    fn guard_evaluation_is_deterministic(context: TestContext, event: TestEvent) {
        let machine = Machine::new("idle", context.clone());

        // Guard should return same result for same inputs
        let result1 = machine.can_transition(&event);
        let result2 = machine.can_transition(&event);

        prop_assert_eq!(result1, result2);
    }

    #[test]
    fn guard_never_panics(context: TestContext, event: TestEvent) {
        let machine = Machine::new("idle", context);

        // Using safe version that catches panics
        let result = machine.can_transition_safe(&event);

        // Should either return bool or GuardPanic error, never panic
        match result {
            Ok(_) | Err(MachineError::GuardPanic { .. }) => {},
            _ => prop_assert!(false, "Unexpected error type"),
        }
    }
}
```

### Integration Tests
```rust
#[test]
fn complex_guard_conditions() {
    let mut machine = Machine::new("draft", DocumentContext::default());

    let draft_state = StateNode::new()
        .on_guarded(
            SubmitEvent,
            "under_review",
            |ctx, _| {
                ctx.word_count >= 100 &&
                ctx.author.is_some() &&
                ctx.deadline.map_or(true, |d| Utc::now() < d)
            }
        )
        .on_guarded(
            PublishEvent,
            "published",
            |ctx, _| {
                ctx.is_approved &&
                ctx.reviewer.is_some() &&
                ctx.final_edits_complete
            }
        );

    machine.add_state("draft", draft_state);

    // Test various conditions
    let context = machine.context_mut();

    // Incomplete document - should fail
    context.word_count = 50;
    assert!(!machine.can_transition(&SubmitEvent));

    // Complete document - should succeed
    context.word_count = 150;
    context.author = Some("John Doe".to_string());
    context.deadline = Some(Utc::now() + Duration::days(1));
    assert!(machine.can_transition(&SubmitEvent));

    // Try to publish without approval - should fail
    assert!(!machine.can_transition(&PublishEvent));

    // Complete approval process - should succeed
    context.is_approved = true;
    context.reviewer = Some("Jane Smith".to_string());
    context.final_edits_complete = true;
    assert!(machine.can_transition(&PublishEvent));
}
```

## Performance Impact

### Evaluation Cost
- **Low**: Guards are simple boolean functions
- **Cached**: Can cache guard results for repeated evaluations
- **Selective**: Only evaluated when transition is attempted

### Optimization Opportunities
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn precompute_guards(&mut self) {
        // Cache guard results for current context
        // Useful when same guards evaluated multiple times
        todo!()
    }

    pub fn invalidate_guard_cache(&mut self) {
        // Clear cache when context changes
        todo!()
    }
}
```

## Security Considerations

### Guard Bypass Prevention
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn send_secure(&mut self, event: E, credentials: &Credentials) -> Result<(), MachineError> {
        // Verify user has permission to trigger this event
        self.verify_permissions(&event, credentials)?;

        // Then check guards
        self.send_guarded(event)
    }
}
```

### Information Leakage
Avoid guards that leak sensitive information:
```rust
// Bad: Timing attack possible
let guard = |ctx, _| {
    if ctx.secret_key.starts_with("expected_prefix") {
        true
    } else {
        // Different timing for different failures
        false
    }
};

// Good: Constant-time evaluation
let guard = |ctx, _| {
    use subtle::ConstantTimeEq;
    ctx.hashed_key.ct_eq(&expected_hash).into()
};
```

## Future Extensions

### Asynchronous Guards
```rust
type AsyncGuardFn<S, E> = Box<dyn Fn(&S, &E) -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + Sync>;

impl<S: State, E: Event> Transition<S, E> {
    pub fn with_async_guard<F>(mut self, guard: F) -> Self
    where
        F: Fn(&S, &E) -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + Sync + 'static,
    {
        self.async_guard = Some(Box::new(guard));
        self
    }
}
```

### Guard Composition
```rust
impl<S: State, E: Event> GuardFn<S, E> {
    pub fn and(self, other: Self) -> Self {
        Box::new(move |ctx, event| self(ctx, event) && other(ctx, event))
    }

    pub fn or(self, other: Self) -> Self {
        Box::new(move |ctx, event| self(ctx, event) || other(ctx, event))
    }

    pub fn not(self) -> Self {
        Box::new(move |ctx, event| !self(ctx, event))
    }
}
```

### Guard Metadata
```rust
#[derive(Clone)]
pub struct Guard<S: State, E: Event> {
    pub condition: GuardFn<S, E>,
    pub description: String,
    pub severity: GuardSeverity,
}

#[derive(Clone)]
pub enum GuardSeverity {
    Info,      // Just logging
    Warning,   // User notification
    Error,     // Block transition
    Critical,  // System error
}
```

## Migration Guide

### From Simple Transitions
```rust
// Before - unguarded transition
let state = StateNode::new()
    .on(LoginEvent, "authenticated");

// After - add guard
let state = StateNode::new()
    .on_guarded(LoginEvent, "authenticated", |ctx, event| {
        // Validate credentials
        ctx.credentials_valid
    });
```

### Gradual Adoption
```rust
// Phase 1: Add logging guards (non-blocking)
let state = StateNode::new()
    .on_guarded(LoginEvent, "authenticated", |ctx, event| {
        log::info!("Login attempt for user: {}", event.username);
        true  // Always allow, just log
    });

// Phase 2: Add validation guards
let state = StateNode::new()
    .on_guarded(LoginEvent, "authenticated", |ctx, event| {
        log::info!("Login attempt for user: {}", event.username);
        ctx.validate_credentials(&event.username, &event.password)
    });
```

## Risk Assessment

### Likelihood: Medium
- Guard functions are user-provided code
- Potential for panics in guard evaluation
- Complex boolean logic can have bugs

### Impact: Medium
- Failed guards prevent valid transitions
- Panic in guards can crash the machine
- Performance impact from guard evaluation

### Mitigation
- Comprehensive guard testing
- Panic-safe guard evaluation
- Guard result caching
- Clear error messages for failed guards
- Timeout mechanisms for long-running guards
