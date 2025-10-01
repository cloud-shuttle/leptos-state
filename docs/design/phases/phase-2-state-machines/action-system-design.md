# Action System Design

## Overview
Implement a comprehensive action system for state machine transitions to enable side effects, context mutations, and complex state management logic during state changes.

## Current State
```rust
#[derive(Clone)]
pub struct Transition<S: State, E: Event> {
    pub target: String,
    pub actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,
}
```

## Enhanced Design
```rust
pub type ActionFn<S, E> = Box<dyn Fn(&mut S, &E) + Send + Sync>;

#[derive(Clone)]
pub struct Transition<S: State, E: Event> {
    pub target: String,
    pub guard: Option<GuardFn<S, E>>,
    pub actions: Option<Vec<ActionFn<S, E>>>,
}

#[derive(Clone)]
pub enum ActionResult {
    Continue,           // Continue with transition
    Cancel,            // Cancel the transition
    Redirect(String),  // Redirect to different state
    Error(String),     // Error condition
}
```

## Motivation

### Side Effect Management
- **Context Updates**: Modify state context during transitions
- **Resource Management**: Acquire/release resources
- **Event Logging**: Track state changes and transitions
- **UI Synchronization**: Update UI state based on machine state
- **External Integration**: Trigger external system updates

### Use Cases
- Update context data during transitions
- Trigger UI updates and animations
- Log state changes for debugging/auditing
- Send notifications or API calls
- Manage resources (database connections, file handles)
- Update caches or derived state

## Implementation Details

### Action Types
```rust
type ActionFn<S, E> = Box<dyn Fn(&mut S, &E) -> ActionResult + Send + Sync>;

#[derive(Clone)]
pub struct Transition<S: State, E: Event> {
    pub target: String,
    pub guard: Option<GuardFn<S, E>>,
    pub actions: Option<Vec<ActionFn<S, E>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionResult {
    Continue,
    Cancel,
    Redirect(String),
    Error(String),
}
```

### Builder Pattern
```rust
impl<S: State, E: Event> Transition<S, E> {
    pub fn with_action<F>(mut self, action: F) -> Self
    where
        F: Fn(&mut S, &E) -> ActionResult + Send + Sync + 'static,
    {
        self.actions.get_or_insert_with(Vec::new).push(Box::new(action));
        self
    }

    pub fn with_actions(mut self, actions: Vec<ActionFn<S, E>>) -> Self {
        self.actions = Some(actions);
        self
    }
}
```

### Fluent API Extensions
```rust
impl<S: State, E: Event> StateNode<S, E> {
    pub fn on_with_action<F>(
        mut self,
        event: E,
        target: &str,
        action: F
    ) -> Self
    where
        F: Fn(&mut S, &E) -> ActionResult + Send + Sync + 'static,
    {
        let transition = Transition::new(target).with_action(action);
        self.transitions.insert(event.event_type(), transition);
        self
    }

    pub fn on_with_actions<F>(
        mut self,
        event: E,
        target: &str,
        actions: Vec<F>
    ) -> Self
    where
        F: Fn(&mut S, &E) -> ActionResult + Send + Sync + 'static,
    {
        let boxed_actions: Vec<ActionFn<S, E>> = actions.into_iter()
            .map(|a| Box::new(a) as ActionFn<S, E>)
            .collect();
        let transition = Transition::new(target).with_actions(boxed_actions);
        self.transitions.insert(event.event_type(), transition);
        self
    }
}
```

## API Design

### Machine Integration
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn send_with_actions(&mut self, event: E) -> Result<(), MachineError> {
        let current_state_name = self.current_state.clone();

        // Get current state node
        let current_node = self.states.get(&current_state_name)
            .ok_or_else(|| MachineError::InvalidState {
                state: current_state_name.clone()
            })?;

        // Find transition
        if let Some(transition) = current_node.transitions.get(&event.event_type()) {
            let target_state = transition.target.clone();

            // Execute exit actions
            self.execute_actions(&current_node.exit_actions, &event)?;

            // Execute transition actions
            if let Some(actions) = &transition.actions {
                for action in actions {
                    match action(&mut self.context, &event) {
                        ActionResult::Continue => continue,
                        ActionResult::Cancel => {
                            return Err(MachineError::ActionCancelled {
                                state: current_state_name,
                                event: event.event_type(),
                            });
                        }
                        ActionResult::Redirect(new_target) => {
                            if let Some(target_node) = self.states.get(&new_target) {
                                self.current_state = new_target.clone();
                                // Execute entry actions for redirected state
                                self.execute_actions(&target_node.entry_actions, &event)?;
                                return Ok(());
                            } else {
                                return Err(MachineError::InvalidRedirect {
                                    from: current_state_name,
                                    to: new_target,
                                });
                            }
                        }
                        ActionResult::Error(message) => {
                            return Err(MachineError::ActionError {
                                state: current_state_name,
                                event: event.event_type(),
                                message,
                            });
                        }
                    }
                }
            }

            // Update current state
            self.current_state = target_state.clone();

            // Execute entry actions for target state
            if let Some(target_node) = self.states.get(&target_state) {
                self.execute_actions(&target_node.entry_actions, &event)?;
            }

            Ok(())
        } else {
            Err(MachineError::InvalidTransition {
                from: current_state_name,
                to: event.event_type(),
            })
        }
    }

    fn execute_actions(
        &mut self,
        actions: &Option<Vec<ActionFn<S, E>>>,
        event: &E
    ) -> Result<(), MachineError> {
        if let Some(actions) = actions {
            for action in actions {
                std::panic::catch_unwind(|| {
                    action(&mut self.context, event)
                }).map_err(|_| MachineError::ActionPanic {
                    state: self.current_state.clone(),
                    action_type: "transition",
                })?;
            }
        }
        Ok(())
    }
}
```

## Action Patterns

### Context Update Actions
```rust
// Simple context updates
let update_timestamp = |ctx: &mut AppContext, _: &AppEvent| {
    ctx.last_updated = Utc::now();
    ActionResult::Continue
};

let increment_counter = |ctx: &mut AppContext, _: &AppEvent| {
    ctx.transition_count += 1;
    ActionResult::Continue
};
```

### Validation Actions
```rust
// Actions that can cancel transitions
let validate_payment = |ctx: &mut PaymentContext, event: &PaymentEvent| {
    if let PaymentEvent::Process { amount, .. } = event {
        if *amount > ctx.daily_limit {
            return ActionResult::Error("Amount exceeds daily limit".to_string());
        }
        if ctx.balance < *amount {
            return ActionResult::Cancel;  // Silent cancellation
        }
    }
    ActionResult::Continue
};
```

### Redirect Actions
```rust
// Conditional redirects based on context
let route_based_on_priority = |ctx: &mut WorkflowContext, event: &WorkflowEvent| {
    if let WorkflowEvent::Submit { priority, .. } = event {
        match priority {
            Priority::High => ActionResult::Redirect("fast_track".to_string()),
            Priority::Normal => ActionResult::Redirect("standard".to_string()),
            Priority::Low => ActionResult::Continue, // Stay in current transition
        }
    } else {
        ActionResult::Continue
    }
};
```

### Complex Multi-Step Actions
```rust
// Actions that perform multiple operations
let process_order = |ctx: &mut OrderContext, event: &OrderEvent| {
    if let OrderEvent::Submit { order_id } = event {
        // Validate order
        if !ctx.validate_order(order_id) {
            return ActionResult::Error("Invalid order".to_string());
        }

        // Reserve inventory
        if !ctx.reserve_inventory(order_id) {
            return ActionResult::Error("Insufficient inventory".to_string());
        }

        // Calculate shipping
        ctx.calculate_shipping(order_id);

        // Send confirmation email
        ctx.send_confirmation_email(order_id);

        ActionResult::Continue
    } else {
        ActionResult::Continue
    }
};
```

## Error Handling

### Action Failure Types
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum MachineError {
    #[error("Action cancelled transition from {state} on event {event}")]
    ActionCancelled { state: String, event: String },

    #[error("Action redirected transition from {from} to {to}")]
    InvalidRedirect { from: String, to: String },

    #[error("Action failed: {message}")]
    ActionError { state: String, event: String, message: String },

    #[error("Action panicked in state {state} during {action_type}")]
    ActionPanic { state: String, action_type: String },
}
```

### Recovery Strategies
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn send_with_recovery(&mut self, event: E) -> Result<(), MachineError> {
        match self.send_with_actions(event) {
            Ok(()) => Ok(()),
            Err(MachineError::ActionError { state, event, message }) => {
                // Log error and attempt recovery
                log::error!("Action error in {} on {}: {}", state, event, message);

                // Execute recovery actions
                self.execute_recovery_actions(&state, &event)?;

                // Stay in current state
                Ok(())
            }
            Err(e) => Err(e), // Re-throw other errors
        }
    }

    fn execute_recovery_actions(&mut self, state: &str, event: &str) -> Result<(), MachineError> {
        // Attempt to recover from failed action
        todo!()
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn action_updates_context() {
    let mut machine = Machine::new("idle", TestContext::default());

    let idle_state = StateNode::new()
        .on_with_action(
            StartEvent,
            "running",
            |ctx, _| {
                ctx.started_at = Some(Utc::now());
                ActionResult::Continue
            }
        );

    machine.add_state("idle", idle_state);
    machine.send_with_actions(StartEvent).unwrap();

    assert!(machine.context().started_at.is_some());
    assert_eq!(machine.current_state(), "running");
}

#[test]
fn action_can_cancel_transition() {
    let mut machine = Machine::new("idle", TestContext::default());

    let idle_state = StateNode::new()
        .on_with_action(
            StartEvent,
            "running",
            |ctx, _| {
                if !ctx.resources_ready {
                    ActionResult::Cancel
                } else {
                    ActionResult::Continue
                }
            }
        );

    machine.add_state("idle", idle_state);

    // Should fail when resources not ready
    machine.context_mut().resources_ready = false;
    let result = machine.send_with_actions(StartEvent);
    assert!(matches!(result, Err(MachineError::ActionCancelled { .. })));
    assert_eq!(machine.current_state(), "idle"); // Still in idle
}

#[test]
fn action_can_redirect_transition() {
    let mut machine = Machine::new("idle", TestContext::default());

    let idle_state = StateNode::new()
        .on_with_action(
            ProcessEvent,
            "normal",  // Default target
            |ctx, _| {
                if ctx.is_urgent {
                    ActionResult::Redirect("urgent".to_string())
                } else {
                    ActionResult::Continue
                }
            }
        );

    machine.add_state("idle", idle_state);
    machine.add_state("urgent", StateNode::new());
    machine.add_state("normal", StateNode::new());

    // Should redirect to urgent
    machine.context_mut().is_urgent = true;
    machine.send_with_actions(ProcessEvent).unwrap();
    assert_eq!(machine.current_state(), "urgent");
}
```

### Integration Tests
```rust
#[test]
fn complex_action_workflow() {
    let mut machine = Machine::new("draft", DocumentContext::default());

    let draft_state = StateNode::new()
        .on_with_actions(
            SubmitEvent,
            "submitted",
            vec![
                // Validate document
                |ctx, _| {
                    if ctx.content.len() < 100 {
                        ActionResult::Error("Document too short".to_string())
                    } else {
                        ActionResult::Continue
                    }
                },
                // Set submission timestamp
                |ctx, _| {
                    ctx.submitted_at = Some(Utc::now());
                    ActionResult::Continue
                },
                // Send notifications
                |ctx, _| {
                    ctx.notifications_sent = true;
                    ActionResult::Continue
                },
            ]
        );

    machine.add_state("draft", draft_state);
    machine.add_state("submitted", StateNode::new());

    // Should fail validation
    machine.context_mut().content = "Too short".to_string();
    let result = machine.send_with_actions(SubmitEvent);
    assert!(matches!(result, Err(MachineError::ActionError { .. })));

    // Should succeed with valid content
    machine.context_mut().content = "This is a valid document with enough content to pass validation.".to_string();
    machine.send_with_actions(SubmitEvent).unwrap();

    let ctx = machine.context();
    assert!(ctx.submitted_at.is_some());
    assert!(ctx.notifications_sent);
    assert_eq!(machine.current_state(), "submitted");
}
```

## Performance Impact

### Execution Overhead
- **Variable**: Depends on action complexity
- **Memory**: Small increase for action storage
- **CPU**: Linear with number of actions and their complexity

### Optimization Opportunities
```rust
impl<S: State, E: Event> Transition<S, E> {
    pub fn optimize_actions(mut self) -> Self {
        if let Some(actions) = self.actions.as_mut() {
            // Remove no-op actions
            actions.retain(|action| {
                // Test if action is no-op (this is complex to determine statically)
                true  // For now, keep all actions
            });

            // Coalesce similar actions
            // Reorder for better performance
        }
        self
    }
}
```

## Security Considerations

### Action Isolation
- Actions should validate inputs
- Context mutations should be safe
- Resource access should be controlled

### Safe Action Execution
```rust
impl<S: State, E: Event> Machine<S, E> {
    fn execute_action_safe(
        &mut self,
        action: &ActionFn<S, E>,
        event: &E
    ) -> Result<ActionResult, MachineError> {
        std::panic::catch_unwind(|| action(&mut self.context, event))
            .map_err(|_| MachineError::ActionPanic {
                state: self.current_state.clone(),
                action_type: "transition",
            })
    }
}
```

### Resource Limits
```rust
#[derive(Clone)]
pub struct ActionLimits {
    pub max_execution_time: Duration,
    pub max_memory_usage: usize,
    pub allow_network_calls: bool,
}

impl<S: State, E: Event> Machine<S, E> {
    pub fn execute_action_with_limits(
        &mut self,
        action: &ActionFn<S, E>,
        event: &E,
        limits: &ActionLimits
    ) -> Result<ActionResult, MachineError> {
        // Implement timeout and resource limits
        todo!()
    }
}
```

## Future Extensions

### Asynchronous Actions
```rust
type AsyncActionFn<S, E> = Box<dyn Fn(&mut S, &E) -> Pin<Box<dyn Future<Output = ActionResult> + Send>> + Send + Sync>;

impl<S: State, E: Event> Transition<S, E> {
    pub async fn execute_async_actions(&self, context: &mut S, event: &E) -> Result<(), MachineError> {
        if let Some(actions) = &self.async_actions {
            for action in actions {
                let result = action(context, event).await;
                match result {
                    ActionResult::Continue => continue,
                    ActionResult::Cancel => return Err(MachineError::ActionCancelled { /* ... */ }),
                    // Handle other results
                }
            }
        }
        Ok(())
    }
}
```

### Action Composition
```rust
impl<S: State, E: Event> ActionFn<S, E> {
    pub fn then(self, other: Self) -> Self {
        Box::new(move |ctx, event| {
            match self(ctx, event) {
                ActionResult::Continue => other(ctx, event),
                result => result,
            }
        })
    }

    pub fn and_then<F>(self, f: F) -> Self
    where
        F: Fn(ActionResult) -> ActionResult + Send + Sync + 'static,
    {
        Box::new(move |ctx, event| {
            f(self(ctx, event))
        })
    }
}
```

### Action Metadata and Debugging
```rust
#[derive(Clone)]
pub struct Action<S: State, E: Event> {
    pub name: String,
    pub description: String,
    pub function: ActionFn<S, E>,
    pub timeout: Option<Duration>,
    pub required_permissions: Vec<String>,
}

impl<S: State, E: Event> Action<S, E> {
    pub fn execute_with_metadata(&self, context: &mut S, event: &E) -> Result<ActionResult, ActionError> {
        // Log execution start
        let start = Instant::now();

        // Check permissions
        // Execute with timeout
        // Log execution end and duration

        todo!()
    }
}
```

## Migration Guide

### From Simple Transitions
```rust
// Before - transition without actions
let state = StateNode::new()
    .on(LoginEvent, "authenticated");

// After - add context update action
let state = StateNode::new()
    .on_with_action(LoginEvent, "authenticated", |ctx, event| {
        ctx.login_time = Some(Utc::now());
        ctx.user_id = Some(event.user_id.clone());
        ActionResult::Continue
    });
```

### Gradual Adoption
```rust
// Phase 1: Logging actions
let state = StateNode::new()
    .on_with_action(LoginEvent, "authenticated", |ctx, event| {
        log::info!("User {} logged in", event.username);
        ActionResult::Continue
    });

// Phase 2: Add validation
let state = StateNode::new()
    .on_with_action(LoginEvent, "authenticated", |ctx, event| {
        log::info!("User {} logged in", event.username);

        if ctx.validate_credentials(&event.username, &event.password) {
            ctx.user_id = Some(event.user_id.clone());
            ActionResult::Continue
        } else {
            ActionResult::Error("Invalid credentials".to_string())
        }
    });
```

## Risk Assessment

### Likelihood: Medium
- Actions are user-provided code with side effects
- Potential for panics or resource leaks
- Complex action logic can have bugs

### Impact: High
- Failed actions can leave system in inconsistent state
- Resource leaks from unhandled errors
- Performance impact from slow actions

### Mitigation
- Comprehensive action testing
- Panic-safe action execution
- Resource cleanup guarantees
- Action timeouts and limits
- Clear error handling and recovery
- Action composition and reuse patterns
