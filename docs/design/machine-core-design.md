# 🎯 Machine Core Design - September 20, 2025

## Executive Summary

**Component**: State Machine Core Engine
**Status**: Core compilation fixed, architecture validated
**Complexity**: High (handles state transitions, guards, actions)
**Dependencies**: State types, event system, action framework

## Architecture Overview

### Core Components

```
MachineCore
├── Machine (static definition)
├── MachineStateImpl (runtime state)
├── Transition (state changes)
├── StateNode (state definitions)
├── Guards (transition conditions)
├── Actions (state change effects)
└── Events (transition triggers)
```

### Design Principles

#### 1. **Separation of Concerns**
- **Machine**: Static definition of possible states/transitions
- **MachineStateImpl**: Runtime execution state
- **Transition**: Atomic state change operations
- **Guards/Actions**: Conditional logic and side effects

#### 2. **Type Safety First**
- Compile-time guarantees for state transitions
- Strong typing for events, contexts, and states
- Trait-based extensibility for guards and actions

#### 3. **Performance Optimized**
- Minimal runtime overhead for transitions
- Efficient state storage and lookup
- Lazy evaluation where appropriate

## Core Types Design

### 1. Machine Definition

```rust
/// Static state machine definition
#[derive(Debug)]
pub struct Machine<C, E, S> {
    /// All possible states in the machine
    pub states: HashMap<String, StateNode<C, E, S>>,

    /// Initial state identifier
    pub initial: String,

    /// Type-level phantom data
    pub _phantom: PhantomData<S>,
}
```

**Design Decisions**:
- **HashMap for O(1) lookup**: States accessed by string identifiers
- **Generic over context (C)**: Allows any context type
- **Generic over events (E)**: Type-safe event handling
- **PhantomData for S**: Preserves state type information

### 2. Runtime State

```rust
/// Runtime state implementation
#[derive(Debug, Clone, PartialEq)]
pub struct MachineStateImpl<C: Send + Sync> {
    /// Current state value (simple, compound, or parallel)
    pub value: StateValue,

    /// Current context data
    pub context: C,
}
```

**Design Decisions**:
- **Value-based state**: Efficient comparison and storage
- **Context separation**: State logic independent of context
- **Send + Sync bounds**: Thread-safe for async operations
- **Clone + PartialEq**: Easy testing and comparison

### 3. State Value Representation

```rust
/// Represents different types of state values
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateValue {
    /// Simple state with identifier
    Simple(String),

    /// Compound state with parent and active child
    Compound {
        parent: String,
        child: Box<StateValue>,
    },

    /// Parallel states with multiple active regions
    Parallel(Vec<StateValue>),
}
```

**Design Decisions**:
- **Recursive structure**: Supports hierarchical state machines
- **Box for indirection**: Prevents infinite type size
- **Hash + Eq**: Efficient storage in HashMaps
- **Simple enum**: Easy pattern matching

## Transition System Design

### 1. Transition Definition

```rust
/// Defines a state transition
#[derive(Debug)]
pub struct Transition<C, E> {
    /// Event that triggers this transition
    pub event: E,

    /// Target state identifier
    pub target: String,

    /// Conditions that must be met
    pub guards: Vec<Box<dyn Guard<C, E>>>,

    /// Actions to execute on transition
    pub actions: Vec<Box<dyn Action<C, E>>>,
}
```

**Design Decisions**:
- **Trait objects for guards/actions**: Runtime polymorphism
- **Vec for multiple conditions/effects**: Flexible composition
- **Box for heap allocation**: Handles varying sizes
- **Generic bounds**: Type-safe context and event handling

### 2. Transition Execution Flow

```
1. Event Received
   ↓
2. Find Matching Transitions
   ↓
3. Evaluate Guards (ALL must pass)
   ↓
4. Execute Exit Actions (current state)
   ↓
5. Execute Transition Actions
   ↓
6. Execute Entry Actions (target state)
   ↓
7. Update State
```

**Performance Characteristics**:
- **Guard evaluation**: Short-circuiting AND logic
- **Action execution**: Sequential, no parallelism
- **State updates**: Atomic from external perspective

## Guard System Design

### 1. Guard Trait

```rust
/// Condition for state transitions
pub trait Guard<C, E>: Send + Sync + std::fmt::Debug {
    /// Evaluate the guard condition
    fn check(&self, context: &C, event: &E) -> bool;
}
```

**Design Decisions**:
- **Simple boolean interface**: Easy to compose
- **Immutable context access**: Prevents side effects
- **Event access**: Allows event-based conditions
- **Send + Sync**: Thread-safe evaluation

### 2. Guard Composition

```rust
/// Composite guard implementations
pub enum CompositeGuard<C, E> {
    /// All guards must pass
    And(Vec<Box<dyn Guard<C, E>>>),

    /// At least one guard must pass
    Or(Vec<Box<dyn Guard<C, E>>>),

    /// Negate guard result
    Not(Box<dyn Guard<C, E>>),

    /// Custom logic function
    Function(Box<dyn Fn(&C, &E) -> bool + Send + Sync>),
}
```

**Design Decisions**:
- **Boolean algebra**: Familiar logical operators
- **Recursive composition**: Build complex conditions
- **Function guards**: Maximum flexibility
- **Box for polymorphism**: Varying guard implementations

## Action System Design

### 1. Action Trait

```rust
/// Side effect for state transitions
pub trait Action<C, E>: Send + Sync + std::fmt::Debug {
    /// Execute the action
    fn execute(&self, context: &mut C, event: &E);
}
```

**Design Decisions**:
- **Mutable context**: Allows state modifications
- **Event access**: Context-aware actions
- **Send + Sync**: Thread-safe execution
- **No return value**: Fire-and-forget semantics

### 2. Action Composition

```rust
/// Composite action implementations
pub enum CompositeAction<C, E> {
    /// Execute actions in sequence
    Sequence(Vec<Box<dyn Action<C, E>>>),

    /// Execute actions in parallel (if supported)
    Parallel(Vec<Box<dyn Action<C, E>>>),

    /// Conditional action execution
    Conditional {
        condition: Box<dyn Fn(&C, &E) -> bool + Send + Sync>,
        action: Box<dyn Action<C, E>>,
    },

    /// Custom action function
    Function(Box<dyn Fn(&mut C, &E) + Send + Sync>),
}
```

**Design Decisions**:
- **Sequential by default**: Predictable execution order
- **Conditional actions**: Context-aware execution
- **Function actions**: Maximum flexibility
- **Composition patterns**: Build complex behaviors

## Error Handling Design

### 1. Error Types

```rust
/// State machine errors
#[derive(Debug, thiserror::Error)]
pub enum MachineError {
    #[error("Invalid state: {state}")]
    InvalidState { state: String },

    #[error("Invalid transition from {from} on event {event}")]
    InvalidTransition { from: String, event: String },

    #[error("Guard evaluation failed: {reason}")]
    GuardFailed { reason: String },

    #[error("Action execution failed: {reason}")]
    ActionFailed { reason: String },
}
```

**Design Decisions**:
- **Descriptive messages**: Clear error communication
- **Structured data**: Programmatic error handling
- **thiserror integration**: Automatic Display/Debug
- **Comprehensive coverage**: All failure modes

### 2. Result Types

```rust
/// State machine results
pub type MachineResult<T> = Result<T, MachineError>;

/// Transition result (may fail)
pub type TransitionResult<T> = Result<T, TransitionError>;
```

**Design Decisions**:
- **Type aliases**: Consistent error handling
- **Generic results**: Flexible error propagation
- **Specific error types**: Domain-specific failures

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| State Lookup | O(1) | HashMap access |
| Transition Search | O(n) | Linear scan of transitions |
| Guard Evaluation | O(g) | g = number of guards |
| Action Execution | O(a) | a = number of actions |
| State Update | O(1) | Simple assignment |

### Space Complexity

| Component | Space | Notes |
|-----------|-------|-------|
| Machine Definition | O(s + t) | s = states, t = transitions |
| Runtime State | O(1) | Fixed size per instance |
| Guards/Actions | O(g + a) | Heap allocated trait objects |
| Context | O(c) | User-defined context size |

### Optimization Opportunities

1. **Transition Caching**: Cache transition lookups by (state, event) pairs
2. **Guard Short-Circuiting**: Stop evaluation on first false guard
3. **Action Batching**: Group actions for better cache performance
4. **State Interning**: Share common state representations

## Testing Strategy

### 1. Unit Tests
- **State creation and validation**
- **Transition logic correctness**
- **Guard evaluation accuracy**
- **Action execution verification**

### 2. Property Tests
- **Transition determinism**: Same input → same output
- **State validity**: Machine never enters invalid states
- **Guard consistency**: Guards don't contradict each other
- **Action idempotency**: Repeated actions are safe

### 3. Integration Tests
- **End-to-end workflows**: Complete state machine execution
- **Error handling**: Proper failure propagation
- **Performance**: Realistic usage patterns

## Future Enhancements

### Advanced Features
1. **Hierarchical States**: Full SCXML support
2. **Parallel Regions**: Concurrent state execution
3. **History States**: State restoration capabilities
4. **Deferred Events**: Event queuing and timing

### Performance Improvements
1. **JIT Compilation**: Runtime optimization of state machines
2. **Memory Pool**: Reduce allocations in hot paths
3. **SIMD Guards**: Vectorized condition evaluation
4. **Async Transitions**: Non-blocking state changes

### Developer Experience
1. **Visual Debugging**: State machine visualization
2. **Hot Reloading**: Runtime state machine updates
3. **IntelliSense**: IDE support for state definitions
4. **Code Generation**: Automatic state machine code from diagrams

---

*Machine core design document created September 20, 2025 - Foundation for reliable state management*