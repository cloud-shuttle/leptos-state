# Design Document: Porting Zustand & XState Patterns to Leptos

## Executive Summary

This document outlines the design for implementing state management patterns inspired by Zustand's simplicity and XState's state machine capabilities in Rust's Leptos framework. The goal is to create a ergonomic, type-safe state management solution that leverages Rust's ownership system and Leptos's reactive primitives.

## Core Concepts to Port

### From Zustand
- **Minimal boilerplate** - Simple store creation without providers
- **Hook-based access** - Direct store subscription in components
- **Immutable updates** - Functional state updates
- **Devtools integration** - Time-travel debugging support
- **Middleware system** - Extensible store enhancement

### From XState
- **Finite state machines** - Explicit state transitions
- **Hierarchical states** - Nested state management
- **Guards and actions** - Transition conditions and side effects
- **Parallel states** - Multiple simultaneous state machines
- **State visualization** - Machine introspection capabilities

## Architecture Overview

```
┌─────────────────────────────────────────┐
│          Application Layer              │
├─────────────────────────────────────────┤
│     Leptos Components & Hooks           │
├─────────────────────────────────────────┤
│         State Management API            │
├──────────────┬──────────────────────────┤
│  Store Layer │    Machine Layer         │
│  (Zustand)   │    (XState)              │
├──────────────┴──────────────────────────┤
│       Leptos Reactive Primitives        │
│    (Signals, Memos, Resources)          │
└─────────────────────────────────────────┘
```

## Module Structure

```rust
leptos_state/
├── lib.rs           // Public API exports
├── store/           // Zustand-inspired store
│   ├── mod.rs
│   ├── store.rs     // Core store implementation
│   ├── middleware.rs // Middleware system
│   └── devtools.rs  // DevTools integration
├── machine/         // XState-inspired state machines
│   ├── mod.rs
│   ├── machine.rs   // State machine core
│   ├── states.rs    // State definitions
│   ├── events.rs    // Event system
│   └── guards.rs    // Transition guards
├── hooks/           // Leptos integration hooks
│   ├── mod.rs
│   ├── use_store.rs
│   └── use_machine.rs
└── utils/           // Shared utilities
    ├── mod.rs
    └── types.rs
```

## Core Implementation Design

### 1. Store Layer (Zustand-inspired)

```rust
// Core store trait
pub trait Store: Clone + 'static {
    type State: Clone + PartialEq + 'static;
    
    fn create() -> Self::State;
    fn use_store() -> (ReadSignal<Self::State>, WriteSignal<Self::State>);
}

// Macro for easy store creation
#[macro_export]
macro_rules! create_store {
    ($name:ident, $state:ty, $init:expr) => {
        #[derive(Clone)]
        pub struct $name;
        
        impl Store for $name {
            type State = $state;
            
            fn create() -> Self::State {
                $init
            }
            
            fn use_store() -> (ReadSignal<Self::State>, WriteSignal<Self::State>) {
                use_context::<StoreContext<Self::State>>()
                    .expect("Store not provided")
                    .signals()
            }
        }
    };
}

// Slices for partial state subscriptions
pub trait StoreSlice<T: Store> {
    type Output: PartialEq + Clone;
    fn select(state: &T::State) -> Self::Output;
}

// Middleware trait
pub trait Middleware<S: Store> {
    fn wrap(&self, next: impl Fn(&S::State) -> S::State) 
        -> impl Fn(&S::State) -> S::State;
}
```

### 2. State Machine Layer (XState-inspired)

```rust
// Core machine types
#[derive(Debug, Clone, PartialEq)]
pub enum StateValue {
    Simple(String),
    Compound {
        parent: String,
        child: Box<StateValue>,
    },
    Parallel(Vec<StateValue>),
}

pub trait StateMachine: Sized + 'static {
    type Context: Clone + PartialEq;
    type Event: Clone;
    type State: MachineState<Context = Self::Context>;
    
    fn initial() -> Self::State;
    fn transition(state: &Self::State, event: Self::Event) -> Self::State;
}

pub trait MachineState {
    type Context;
    
    fn value(&self) -> &StateValue;
    fn context(&self) -> &Self::Context;
    fn matches(&self, pattern: &str) -> bool;
    fn can_transition_to(&self, target: &str) -> bool;
}

// Builder pattern for machine definition
pub struct MachineBuilder<C, E> {
    states: HashMap<String, StateNode<C, E>>,
    initial: String,
}

impl<C: Clone, E: Clone> MachineBuilder<C, E> {
    pub fn state(mut self, id: &str) -> StateBuilder<C, E> {
        StateBuilder::new(self, id.to_string())
    }
    
    pub fn build(self) -> impl StateMachine<Context = C, Event = E> {
        // Implementation
    }
}

// Guards and actions
pub trait Guard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool;
}

pub trait Action<C, E> {
    fn execute(&self, context: &mut C, event: &E);
}
```

### 3. Leptos Integration Hooks

```rust
// Store hook with selector support
pub fn use_store<S: Store>() -> (ReadSignal<S::State>, WriteSignal<S::State>) {
    S::use_store()
}

pub fn use_store_slice<S: Store, Slice: StoreSlice<S>>(
) -> ReadSignal<Slice::Output> {
    let (state, _) = use_store::<S>();
    create_memo(move |_| Slice::select(&state.get()))
}

// Machine hook with derived signals
pub fn use_machine<M: StateMachine>() -> MachineHandle<M> {
    let (state, set_state) = create_signal(M::initial());
    
    let send = move |event: M::Event| {
        set_state.update(|s| *s = M::transition(s, event));
    };
    
    let matches = move |pattern: &str| {
        create_memo(move |_| state.get().matches(pattern))
    };
    
    MachineHandle {
        state,
        send: Box::new(send),
        matches: Box::new(matches),
    }
}

pub struct MachineHandle<M: StateMachine> {
    pub state: ReadSignal<M::State>,
    pub send: Box<dyn Fn(M::Event)>,
    pub matches: Box<dyn Fn(&str) -> Memo<bool>>,
}
```

## Usage Examples

### Simple Store (Zustand-style)

```rust
#[derive(Clone, PartialEq)]
struct AppState {
    count: i32,
    user: Option<String>,
}

create_store!(AppStore, AppState, AppState { 
    count: 0, 
    user: None 
});

// In component
#[component]
fn Counter() -> impl IntoView {
    let (state, set_state) = use_store::<AppStore>();
    
    let increment = move |_| {
        set_state.update(|s| AppState {
            count: s.count + 1,
            ..s.clone()
        });
    };
    
    view! {
        <button on:click=increment>
            "Count: " {move || state.get().count}
        </button>
    }
}
```

### State Machine (XState-style)

```rust
#[derive(Clone, PartialEq)]
struct ToggleContext {
    count: i32,
}

#[derive(Clone)]
enum ToggleEvent {
    Toggle,
    Reset,
}

let machine = MachineBuilder::<ToggleContext, ToggleEvent>::new()
    .state("inactive")
        .on(ToggleEvent::Toggle, "active")
        .on_entry(|ctx| ctx.count += 1)
    .state("active")
        .on(ToggleEvent::Toggle, "inactive")
        .on(ToggleEvent::Reset, "inactive")
        .with_guard(|ctx, _| ctx.count < 10)
    .initial("inactive")
    .build();

// In component
#[component]
fn ToggleButton() -> impl IntoView {
    let machine = use_machine::<ToggleMachine>();
    let is_active = machine.matches("active");
    
    view! {
        <button on:click=move |_| machine.send(ToggleEvent::Toggle)>
            {move || if is_active.get() { "ON" } else { "OFF" }}
        </button>
    }
}
```

## Advanced Features

### 1. Persist Middleware

```rust
pub struct PersistMiddleware {
    key: String,
}

impl<S: Store> Middleware<S> for PersistMiddleware 
where 
    S::State: Serialize + DeserializeOwned,
{
    fn wrap(&self, next: impl Fn(&S::State) -> S::State) 
        -> impl Fn(&S::State) -> S::State {
        let key = self.key.clone();
        move |state| {
            let new_state = next(state);
            // Save to localStorage
            if let Ok(json) = serde_json::to_string(&new_state) {
                window().local_storage()
                    .unwrap()
                    .unwrap()
                    .set_item(&key, &json)
                    .ok();
            }
            new_state
        }
    }
}
```

### 2. Time Travel Debugging

```rust
pub struct TimeTravel<S: Store> {
    history: RwSignal<Vec<S::State>>,
    current: RwSignal<usize>,
}

impl<S: Store> TimeTravel<S> {
    pub fn undo(&self) {
        self.current.update(|c| *c = c.saturating_sub(1));
    }
    
    pub fn redo(&self) {
        self.current.update(|c| {
            let history = self.history.get();
            *c = (*c + 1).min(history.len() - 1)
        });
    }
    
    pub fn jump_to(&self, index: usize) {
        let max = self.history.get().len() - 1;
        self.current.set(index.min(max));
    }
}
```

### 3. Computed/Derived State

```rust
pub fn create_computed<S: Store, T: PartialEq + Clone + 'static>(
    selector: impl Fn(&S::State) -> T + 'static,
) -> Memo<T> {
    let (state, _) = use_store::<S>();
    create_memo(move |_| selector(&state.get()))
}

// Usage
let double_count = create_computed::<AppStore, _>(|s| s.count * 2);
```

## Performance Optimizations

### 1. Batch Updates
- Implement transaction boundaries for multiple state updates
- Leverage Leptos's batch update mechanisms

### 2. Selective Subscriptions
- Use memos and slices to prevent unnecessary re-renders
- Implement equality checks for complex state shapes

### 3. Lazy Initialization
- Support async store initialization with Resources
- Implement code-splitting for large state machines

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn store_updates_correctly() {
        // Test store mutations
    }
    
    #[test]
    fn machine_transitions_correctly() {
        // Test state transitions
    }
}
```

### Integration Tests
- Test Leptos component integration
- Verify reactive updates
- Test middleware chain

## Migration Path

### Phase 1: Core Implementation
- Implement basic store functionality
- Create fundamental machine types
- Basic Leptos hooks

### Phase 2: Advanced Features
- Add middleware system
- Implement hierarchical states
- Add devtools support

### Phase 3: Ecosystem
- Create derive macros for boilerplate reduction
- Build visualization tools
- Create comprehensive examples

## API Stability Considerations

- Use semantic versioning
- Maintain backward compatibility within major versions
- Provide migration guides for breaking changes
- Use feature flags for experimental features

## Conclusion

This design brings the best of JavaScript state management to Rust's Leptos framework while leveraging Rust's type safety and performance. The combination of Zustand's simplicity and XState's power provides a flexible foundation for complex application state management.