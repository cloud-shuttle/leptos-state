# Core State Machine Design
## Fundamental Architecture for Leptos-State

**Version:** 1.0  
**Date:** September 20, 2025  
**Status:** DRAFT - Requires Implementation  

---

## Overview

The core state machine provides the fundamental abstractions for modeling stateful systems in Leptos applications. This design establishes the type-safe, performant foundation that all other components build upon.

## Core Concepts

### 1. State Machine Definition
```rust
pub trait StateMachine<C, E, S> {
    /// Get the current state
    fn current_state(&self) -> &S;

    /// Get the machine context
    fn context(&self) -> &C;

    /// Transition to a new state
    fn transition(&mut self, event: E) -> Result<(), TransitionError>;

    /// Check if a transition is possible
    fn can_transition(&self, event: &E) -> bool;
}
```

### 2. State Representation
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct StateValue {
    /// The state identifier
    pub id: String,
    /// Optional parent state for hierarchical machines
    pub parent: Option<String>,
    /// State metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct MachineState<C> {
    /// The current state value
    pub value: StateValue,
    /// Machine context data
    pub context: C,
    /// State entry timestamp
    pub entered_at: SystemTime,
    /// Previous state (for history)
    pub previous_state: Option<StateValue>,
}
```

### 3. Event System
```rust
#[derive(Debug, Clone)]
pub struct Event<E> {
    /// Event type identifier
    pub event_type: E,
    /// Event payload data
    pub payload: Option<serde_json::Value>,
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event source identifier
    pub source: Option<String>,
}

pub trait EventHandler<E> {
    fn handle_event(&mut self, event: &Event<E>) -> Result<(), EventError>;
}
```

## Architecture Layers

### 1. Type System Layer
**File:** `core/types.rs` (<200 lines)
- Fundamental type definitions
- Generic constraints
- Type aliases for common patterns

### 2. State Management Layer
**File:** `core/state.rs` (<250 lines)
- StateValue and MachineState implementations
- State validation logic
- State transition tracking

### 3. Event Processing Layer
**File:** `core/events.rs` (<200 lines)
- Event definition and handling
- Event routing logic
- Event serialization/deserialization

### 4. Transition Engine Layer
**File:** `core/transitions.rs` (<300 lines)
- Transition evaluation logic
- Guard condition processing
- Action execution coordination

### 5. Machine Controller Layer
**File:** `core/machine.rs` (<250 lines)
- Main Machine struct implementation
- Public API coordination
- Error handling and recovery

## API Contracts

### Machine Creation Contract
```rust
pub trait MachineBuilder<C, E, S> {
    fn new() -> Self;
    fn state<F>(self, id: &str, config: F) -> Self
    where
        F: FnOnce(StateBuilder<C, E, S>) -> StateBuilder<C, E, S>;
    fn initial_state(self, id: &str) -> Self;
    fn build(self) -> Result<Machine<C, E, S>, MachineError>;
}
```

### State Configuration Contract
```rust
pub trait StateConfig<C, E, S> {
    fn on_entry<A>(self, action: A) -> Self
    where
        A: Action<C, E> + 'static;

    fn on_exit<A>(self, action: A) -> Self
    where
        A: Action<C, E> + 'static;

    fn transition<E2, G, A>(
        self,
        event: E2,
        target: &str,
        guard: Option<G>,
        action: Option<A>
    ) -> Self
    where
        G: Guard<C, E2> + 'static,
        A: Action<C, E2> + 'static;
}
```

## Error Handling Strategy

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum MachineError {
    #[error("Invalid state transition: {from} -> {to}")]
    InvalidTransition { from: String, to: String },

    #[error("Guard condition failed: {reason}")]
    GuardFailed { reason: String },

    #[error("Action execution failed: {action}")]
    ActionFailed { action: String },

    #[error("State not found: {state}")]
    StateNotFound { state: String },
}
```

### Error Recovery
- **Validation Phase:** Check transition validity before execution
- **Rollback Phase:** Restore previous state on action failure
- **Logging Phase:** Record all errors with context
- **Recovery Phase:** Attempt automatic recovery where possible

## Performance Characteristics

### Memory Usage
- **State Storage:** O(S) where S = number of states
- **Transition Table:** O(T) where T = number of transitions
- **Context Data:** Configurable, user-defined size

### Execution Performance
- **Transition Lookup:** O(1) average case with HashMap
- **Guard Evaluation:** O(G) where G = number of guards
- **Action Execution:** O(A) where A = number of actions

### Concurrency Model
- **Single Writer:** Only one transition at a time
- **Multiple Readers:** State inspection is thread-safe
- **Async Support:** All operations support async execution

## Testing Strategy

### Unit Tests
- State transition logic
- Guard condition evaluation
- Action execution
- Error handling paths

### Integration Tests
- Full state machine workflows
- Event processing pipelines
- Context data flow

### Property-Based Tests
- State machine invariants
- Transition safety properties
- Error recovery guarantees

## Implementation Plan

### Phase 1: Core Types (Week 1)
- [ ] Define fundamental types
- [ ] Implement basic traits
- [ ] Create error handling foundation

### Phase 2: State Management (Week 2)
- [ ] Implement StateValue and MachineState
- [ ] Add state validation logic
- [ ] Create state transition tracking

### Phase 3: Event System (Week 3)
- [ ] Implement Event and EventHandler
- [ ] Add event routing logic
- [ ] Create event serialization

### Phase 4: Transition Engine (Week 4)
- [ ] Implement transition evaluation
- [ ] Add guard processing
- [ ] Coordinate action execution

### Phase 5: Machine Controller (Week 5)
- [ ] Implement main Machine struct
- [ ] Coordinate all components
- [ ] Add comprehensive error handling

### Phase 6: API Contracts (Week 6)
- [ ] Define builder traits
- [ ] Implement configuration APIs
- [ ] Add contract tests

## Success Criteria

- [ ] All components compile without errors
- [ ] Comprehensive unit test coverage (>90%)
- [ ] Performance benchmarks established
- [ ] API contracts defined and tested
- [ ] Documentation complete with examples
- [ ] Integration tests passing

## Dependencies

- **Internal:** None (this is the foundation)
- **External:** serde, thiserror, tokio

## Risk Assessment

### High Risk
- Complex generic type interactions
- Performance overhead from trait objects

### Mitigation
- Extensive compile-time testing
- Performance profiling from day one
- Incremental API design with feedback

---

**Design Author:** Senior Rust Engineer  
**Review Date:** September 20, 2025  
**Implementation Estimate:** 6 weeks  
**Test Coverage Target:** 90%  
**Performance Target:** <1ms average transition time
