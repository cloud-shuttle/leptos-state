# 🏪 Store Core Design

## Overview
Core reactive store implementation with state management, subscriptions, and middleware support.

## Architecture

### Core Components
```
store/
├── core.rs        # Store struct and basic operations (150 lines)
├── state.rs       # State management and updates (120 lines)
├── signals.rs     # Reactive signals integration (100 lines)
├── middleware.rs  # Middleware chain system (180 lines)
└── types.rs       # Type definitions (50 lines)
```

## Core Store Struct

```rust
pub struct Store<T> {
    state: RwSignal<T>,
    subscribers: SubscriberList,
    middleware_chain: MiddlewareChain,
    history: Option<StoreHistory>,
}

impl<T> Store<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    pub fn new(initial: T) -> Self {
        Self {
            state: create_rw_signal(initial),
            subscribers: SubscriberList::new(),
            middleware_chain: MiddlewareChain::new(),
            history: None,
        }
    }

    pub fn get(&self) -> ReadSignal<T> {
        self.state.read_only()
    }

    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        self.middleware_chain.before_update();
        self.state.update(updater);
        self.middleware_chain.after_update();
        self.notify_subscribers();
    }
}
```

## State Management

```rust
pub struct StateManager<T> {
    current: T,
    previous: Option<T>,
    version: u64,
}

impl<T> StateManager<T>
where
    T: Clone + PartialEq,
{
    pub fn new(initial: T) -> Self {
        Self {
            current: initial,
            previous: None,
            version: 0,
        }
    }

    pub fn update<F>(&mut self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        self.previous = Some(self.current.clone());
        updater(&mut self.current);
        self.version += 1;
    }

    pub fn rollback(&mut self) -> Result<(), StateError> {
        if let Some(prev) = self.previous.take() {
            self.current = prev;
            self.version += 1;
            Ok(())
        } else {
            Err(StateError::NoPreviousState)
        }
    }
}
```

## Reactive Integration

```rust
pub struct StoreSignals<T> {
    read_signal: ReadSignal<T>,
    write_signal: WriteSignal<T>,
    effect_handle: Option<EffectHandle>,
}

impl<T> StoreSignals<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(store: &Store<T>) -> Self {
        let (read, write) = create_signal(store.get().get());
        let store_ref = store.clone();

        let effect_handle = create_effect(move |_| {
            let current = store_ref.get().get();
            if read.get() != current {
                write.set(current);
            }
        });

        Self {
            read_signal: read,
            write_signal: write,
            effect_handle: Some(effect_handle),
        }
    }
}
```

## Middleware System

```rust
pub trait Middleware: Send + Sync {
    fn before_update(&self, state: &dyn Any) -> Result<(), MiddlewareError>;
    fn after_update(&self, state: &dyn Any) -> Result<(), MiddlewareError>;
}

pub struct MiddlewareChain {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    pub fn add<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }

    pub fn before_update(&self) {
        for middleware in &self.middlewares {
            // Call middleware
        }
    }
}
```

## Type Definitions

```rust
pub type StoreResult<T> = Result<T, StoreError>;

#[derive(Debug, Clone)]
pub enum StoreError {
    InvalidState,
    MiddlewareError(String),
    SerializationError(String),
}

pub trait Storable: Clone + PartialEq + Send + Sync + 'static {}

impl<T> Storable for T where T: Clone + PartialEq + Send + Sync + 'static {}
```

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_creation_works() {
        let store = Store::new(CounterState::default());
        assert_eq!(store.get().get().count, 0);
    }

    #[test]
    fn state_updates_propagate() {
        let store = Store::new(CounterState::default());
        store.update(|state| state.count = 42);
        assert_eq!(store.get().get().count, 42);
    }

    #[test]
    fn middleware_chain_executes() {
        let store = Store::new(CounterState::default())
            .with_middleware(LoggerMiddleware::new());

        store.update(|state| state.count = 1);
        // Verify middleware was called
    }
}
```

## Performance Considerations

- **Signal Efficiency:** Minimize signal creation overhead
- **Update Batching:** Batch multiple updates when possible
- **Memory Management:** Clean up old history entries
- **Middleware Overhead:** Profile middleware impact

## Future Extensions

- [ ] Async state updates
- [ ] Computed values
- [ ] State validation
- [ ] Undo/redo system
