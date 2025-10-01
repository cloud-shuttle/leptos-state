# Entry/Exit Actions Design

## Overview
Implement entry and exit actions for state machines to enable lifecycle management, automatic state transitions, and side effects during state changes.

## Current State
```rust
#[derive(Clone)]
pub struct StateNode<S: State, E: Event> {
    pub transitions: HashMap<String, Transition<S, E>>,
}

#[derive(Clone)]
pub struct Transition<S: State, E: Event> {
    pub target: String,
    pub actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,
}
```

## Proposed Enhancement
```rust
#[derive(Clone)]
pub struct StateNode<S: State, E: Event> {
    pub entry_actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,
    pub exit_actions: Option<Vec<Box<dyn Fn(&mut S, &E) + Send + Sync>>>,
    pub transitions: HashMap<String, Transition<S, E>>,
}
```

## Motivation

### State Lifecycle Management
- **Entry Actions**: Initialize state-specific resources
- **Exit Actions**: Clean up resources when leaving state
- **Automatic Transitions**: Trigger actions without explicit code
- **State Context**: Maintain state-specific data and behavior

### Use Cases
- Resource management (acquire/release locks, connections)
- UI state synchronization (show/hide elements)
- Logging and monitoring (entry/exit events)
- Data initialization (load state-specific data)
- Cleanup operations (save data, reset timers)

## Implementation Details

### Action Types
```rust
type EntryAction<S, E> = Box<dyn Fn(&mut S, &E) + Send + Sync>;
type ExitAction<S, E> = Box<dyn Fn(&mut S, &E) + Send + Sync>;

#[derive(Clone)]
pub struct StateNode<S: State, E: Event> {
    pub entry_actions: Option<Vec<EntryAction<S, E>>>,
    pub exit_actions: Option<Vec<ExitAction<S, E>>>,
    pub transitions: HashMap<String, Transition<S, E>>,
}
```

### Builder Pattern
```rust
impl<S: State, E: Event> StateNode<S, E> {
    pub fn new() -> Self {
        Self {
            entry_actions: None,
            exit_actions: None,
            transitions: HashMap::new(),
        }
    }

    pub fn on_entry<F>(mut self, action: F) -> Self
    where
        F: Fn(&mut S, &E) + Send + Sync + 'static,
    {
        self.entry_actions
            .get_or_insert_with(Vec::new)
            .push(Box::new(action));
        self
    }

    pub fn on_exit<F>(mut self, action: F) -> Self
    where
        F: Fn(&mut S, &E) + Send + Sync + 'static,
    {
        self.exit_actions
            .get_or_insert_with(Vec::new)
            .push(Box::new(action));
        self
    }
}
```

## API Design

### Machine Integration
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn send(&mut self, event: E) -> Result<(), MachineError> {
        let current_state_name = self.current_state.clone();

        // Get current state node
        let current_node = self.states.get(&current_state_name)
            .ok_or_else(|| MachineError::InvalidState {
                state: current_state_name.clone()
            })?;

        // Find transition
        if let Some(transition) = current_node.transitions.get(&event.event_type()) {
            let target_state = transition.target.clone();

            // Execute EXIT actions for current state
            if let Some(exit_actions) = &current_node.exit_actions {
                for action in exit_actions {
                    action(&mut self.context, &event);
                }
            }

            // Execute transition actions
            if let Some(transition_actions) = &transition.actions {
                for action in transition_actions {
                    action(&mut self.context, &event);
                }
            }

            // Update current state
            self.current_state = target_state.clone();

            // Execute ENTRY actions for target state
            if let Some(target_node) = self.states.get(&target_state) {
                if let Some(entry_actions) = &target_node.entry_actions {
                    for action in entry_actions {
                        action(&mut self.context, &event);
                    }
                }
            }

            Ok(())
        } else {
            Err(MachineError::InvalidTransition {
                from: current_state_name,
                to: event.event_type(),
            })
        }
    }
}
```

### Fluent API
```rust
let machine = Machine::new("idle", AppContext::default());

let idle_state = StateNode::new()
    .on_entry(|ctx, _| {
        ctx.status = "Ready".to_string();
        log::info!("Entering idle state");
    })
    .on_exit(|ctx, _| {
        ctx.last_active = Some(Utc::now());
        log::info!("Exiting idle state");
    })
    .on(StartEvent, "running");

let running_state = StateNode::new()
    .on_entry(|ctx, _| {
        ctx.start_time = Some(Utc::now());
        ctx.status = "Running".to_string();
    })
    .on_exit(|ctx, _| {
        ctx.end_time = Some(Utc::now());
    })
    .on(StopEvent, "idle");

machine.add_state("idle", idle_state);
machine.add_state("running", running_state);
```

## Context Management

### Shared Context
```rust
#[derive(Clone)]
struct AppContext {
    status: String,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    last_active: Option<DateTime<Utc>>,
    resources: Vec<ResourceHandle>,
}

impl AppContext {
    fn acquire_resource(&mut self, resource: ResourceHandle) {
        self.resources.push(resource);
    }

    fn release_resources(&mut self) {
        for resource in self.resources.drain(..) {
            resource.cleanup();
        }
    }
}
```

### State-Specific Context
```rust
#[derive(Clone)]
enum AppContext {
    Idle {
        last_active: Option<DateTime<Utc>>,
    },
    Running {
        start_time: DateTime<Utc>,
        active_resources: Vec<ResourceHandle>,
    },
    Error {
        error_message: String,
        error_time: DateTime<Utc>,
    },
}
```

## Error Handling

### Action Failure Handling
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn send_with_error_handling(&mut self, event: E) -> Result<(), MachineError> {
        let result = self.send(event);

        // If transition failed, execute error recovery actions
        if result.is_err() {
            if let Some(error_state) = self.states.get("error") {
                if let Some(entry_actions) = &error_state.entry_actions {
                    for action in entry_actions {
                        // Note: Using a dummy event for error recovery
                        let dummy_event = E::default();
                        action(&mut self.context, &dummy_event);
                    }
                }
            }
        }

        result
    }
}
```

### Panic Safety
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn send_safe(&mut self, event: E) -> Result<(), MachineError> {
        // Execute exit actions with panic handling
        let exit_result = self.execute_exit_actions(&event);

        // Execute transition
        let transition_result = self.execute_transition(&event);

        // Execute entry actions with panic handling
        let entry_result = self.execute_entry_actions(&event);

        // Combine results with proper error precedence
        entry_result.and(transition_result).and(exit_result)
    }

    fn execute_exit_actions(&mut self, event: &E) -> Result<(), MachineError> {
        std::panic::catch_unwind(|| {
            if let Some(current_node) = self.states.get(&self.current_state) {
                if let Some(exit_actions) = &current_node.exit_actions {
                    for action in exit_actions {
                        action(&mut self.context, event);
                    }
                }
            }
        }).map_err(|_| MachineError::ActionPanic {
            state: self.current_state.clone(),
            action_type: "exit",
        })?;

        Ok(())
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn entry_actions_execute_on_transition() {
    let mut machine = Machine::new("idle", TestContext::default());

    let mut entry_called = false;
    let running_state = StateNode::new()
        .on_entry(|ctx, _| {
            ctx.value = 42;
            entry_called = true;
        })
        .on(NextEvent, "idle");

    machine.add_state("running", running_state);

    // Transition to running state
    machine.send(NextEvent).unwrap();

    assert!(entry_called);
    assert_eq!(machine.context().value, 42);
}

#[test]
fn exit_actions_execute_before_transition() {
    let mut machine = Machine::new("running", TestContext::default());

    let mut exit_called = false;
    let running_state = StateNode::new()
        .on_exit(|ctx, _| {
            ctx.cleanup_count += 1;
            exit_called = true;
        })
        .on(StopEvent, "idle");

    machine.add_state("running", running_state);

    // Transition away from running state
    machine.send(StopEvent).unwrap();

    assert!(exit_called);
    assert_eq!(machine.context().cleanup_count, 1);
}
```

### Integration Tests
```rust
#[test]
fn lifecycle_actions_full_workflow() {
    let mut machine = Machine::new("idle", WorkflowContext::default());

    // Define states with lifecycle actions
    let idle_state = StateNode::new()
        .on_entry(|ctx, _| ctx.log_event("entered_idle"))
        .on_exit(|ctx, _| ctx.log_event("exited_idle"))
        .on(StartEvent, "running");

    let running_state = StateNode::new()
        .on_entry(|ctx, _| {
            ctx.acquire_resources();
            ctx.log_event("entered_running");
        })
        .on_exit(|ctx, _| {
            ctx.release_resources();
            ctx.log_event("exited_running");
        })
        .on(StopEvent, "idle");

    machine.add_state("idle", idle_state);
    machine.add_state("running", running_state);

    // Execute workflow
    machine.send(StartEvent).unwrap();
    machine.send(StopEvent).unwrap();

    // Verify lifecycle
    let events = machine.context().events;
    assert_eq!(events, vec![
        "entered_idle",
        "exited_idle",
        "entered_running",
        "exited_running",
        "entered_idle"
    ]);
}
```

## Performance Impact

### Execution Overhead
- **Minimal**: Actions are only executed during transitions
- **Memory**: Small increase for action storage
- **CPU**: Linear with number of actions per transition

### Optimization Opportunities
```rust
impl<S: State, E: Event> StateNode<S, E> {
    pub fn with_optimized_actions(mut self) -> Self {
        // Coalesce similar actions
        // Remove no-op actions
        // Optimize action order
        self
    }
}
```

## Security Considerations

### Action Isolation
- Actions should not interfere with machine state
- Context mutations should be validated
- Resource access should be controlled

### Panic Prevention
```rust
impl<S: State, E: Event> Machine<S, E> {
    pub fn add_safe_entry_action<F>(&mut self, state_name: &str, action: F)
    where
        F: Fn(&mut S, &E) + Send + Sync + 'static,
    {
        // Wrap action with panic boundary
        let safe_action = move |ctx: &mut S, event: &E| {
            std::panic::catch_unwind(|| action(ctx, event))
                .unwrap_or_else(|_| {
                    log::error!("Entry action panicked in state {}", state_name);
                });
        };

        // Add to state node
    }
}
```

## Future Extensions

### Asynchronous Actions
```rust
type AsyncEntryAction<S, E> = Box<dyn Fn(&mut S, &E) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

impl<S: State, E: Event> StateNode<S, E> {
    pub async fn execute_async_entry_actions(&self, context: &mut S, event: &E) {
        if let Some(actions) = &self.async_entry_actions {
            for action in actions {
                action(context, event).await;
            }
        }
    }
}
```

### Conditional Actions
```rust
impl<S: State, E: Event> StateNode<S, E> {
    pub fn on_entry_if<F, P>(mut self, predicate: P, action: F) -> Self
    where
        F: Fn(&mut S, &E) + Send + Sync + 'static,
        P: Fn(&S, &E) -> bool + Send + Sync + 'static,
    {
        let conditional_action = move |ctx: &mut S, event: &E| {
            if predicate(ctx, event) {
                action(ctx, event);
            }
        };

        self.on_entry(conditional_action)
    }
}
```

### Action Priorities
```rust
#[derive(Clone)]
pub struct PrioritizedAction<S, E> {
    pub priority: i32,
    pub action: Box<dyn Fn(&mut S, &E) + Send + Sync>,
}

impl<S: State, E: Event> StateNode<S, E> {
    pub fn on_entry_prioritized(mut self, priority: i32, action: impl Fn(&mut S, &E) + Send + Sync + 'static) -> Self {
        // Insert action in priority order
        todo!()
    }
}
```

## Migration Guide

### From Basic Transitions
```rust
// Before - basic state machine
let machine = Machine::new("idle", context);
let idle_state = StateNode::new().on(StartEvent, "running");
machine.add_state("idle", idle_state);

// After - with lifecycle actions
let idle_state = StateNode::new()
    .on_entry(|ctx, _| log::info!("Entering idle"))
    .on_exit(|ctx, _| log::info!("Exiting idle"))
    .on(StartEvent, "running");
```

### Gradual Adoption
```rust
// Start with logging actions
let state = StateNode::new()
    .on_entry(|ctx, _| log::info!("Entered {}", state_name))
    .on_exit(|ctx, _| log::info!("Exited {}", state_name))
    .on(event, target);

// Add resource management later
let state = StateNode::new()
    .on_entry(|ctx, _| {
        log::info!("Entered {}", state_name);
        ctx.acquire_resources();
    })
    .on_exit(|ctx, _| {
        ctx.release_resources();
        log::info!("Exited {}", state_name);
    })
    .on(event, target);
```

## Risk Assessment

### Likelihood: Low
- Well-established state machine patterns
- Action system is additive and optional
- Comprehensive testing possible

### Impact: Low
- Backward compatible (existing code works unchanged)
- Actions are opt-in per state
- Clear error boundaries

### Mitigation
- Comprehensive testing of action execution order
- Panic safety mechanisms
- Performance monitoring for action overhead
- Documentation with examples and best practices
