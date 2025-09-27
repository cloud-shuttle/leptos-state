# 🏪 Store Architecture Design

## Overview
Design document for the reactive store system - the core state management component of leptos-state.

## Core Principles

### 1. Reactive by Default
All store state changes automatically trigger UI updates through Leptos signals.

### 2. Type Safety
Full Rust type safety with compile-time guarantees and zero-cost abstractions.

### 3. Minimal Boilerplate
Simple, intuitive API that doesn't require excessive setup or configuration.

## Store API Design

### Core Store Interface

```rust
pub trait Store<T> {
    /// Get current state value
    fn get(&self) -> T;
    
    /// Update state with new value
    fn set(&self, value: T);
    
    /// Update state through mutation
    fn update(&self, f: impl FnOnce(&mut T));
    
    /// Subscribe to state changes
    fn subscribe(&self, callback: impl Fn(&T) + 'static) -> SubscriptionHandle;
}
```

### Store Creation API

```rust
// Simple store creation
pub fn create_store<T>(initial: T) -> (ReadSignal<T>, WriteSignal<T>)
where T: Clone + 'static;

// Store with actions
pub fn create_store_with_actions<T, A>(initial: T, actions: A) -> (ReadSignal<T>, A)
where 
    T: Clone + 'static,
    A: StoreActions<T>;

// Store with middleware
pub fn create_store_with_middleware<T>(
    initial: T, 
    middleware: MiddlewareChain<T>
) -> (ReadSignal<T>, WriteSignal<T>)
where T: Clone + 'static;
```

### Store Hook API

```rust
// Primary hook for components
pub fn use_store<T>() -> (ReadSignal<T>, StoreActions<T>)
where T: StoreState + 'static;

// Selector hook for performance
pub fn use_store_selector<T, R>(
    selector: impl Fn(&T) -> R + 'static
) -> ReadSignal<R>
where 
    T: StoreState + 'static,
    R: Clone + PartialEq + 'static;

// Store context hook
pub fn use_store_context<T>() -> Option<(ReadSignal<T>, StoreActions<T>)>
where T: StoreState + 'static;
```

## Store Types

### 1. Basic Store
```rust
pub struct BasicStore<T> {
    state: RwSignal<T>,
    middleware: Option<MiddlewareChain<T>>,
}

impl<T> BasicStore<T> 
where T: Clone + 'static
{
    pub fn new(initial: T) -> Self {
        Self {
            state: create_rw_signal(initial),
            middleware: None,
        }
    }
    
    pub fn with_middleware(mut self, middleware: MiddlewareChain<T>) -> Self {
        self.middleware = Some(middleware);
        self
    }
}
```

### 2. Async Store
```rust
pub struct AsyncStore<T> {
    resource: Resource<(), T>,
    loading: ReadSignal<bool>,
    error: ReadSignal<Option<Error>>,
}

impl<T> AsyncStore<T> 
where T: Clone + 'static
{
    pub fn new<F, Fut>(fetcher: F) -> Self 
    where 
        F: Fn() -> Fut + 'static,
        Fut: Future<Output = Result<T, Error>> + 'static,
    {
        let resource = create_resource(|| (), move |_| fetcher());
        
        Self {
            resource,
            loading: resource.loading(),
            error: create_memo(move |_| {
                resource.get().and_then(|r| r.err())
            }).into(),
        }
    }
}
```

### 3. Persistent Store
```rust
pub struct PersistentStore<T> {
    store: BasicStore<T>,
    persistence: PersistenceBackend,
    config: PersistenceConfig,
}

impl<T> PersistentStore<T> 
where T: Clone + Serialize + DeserializeOwned + 'static
{
    pub fn new(initial: T, config: PersistenceConfig) -> Self {
        let store = BasicStore::new(initial);
        
        // Load from persistence on creation
        if let Ok(persisted) = config.backend.load(&config.key) {
            if let Ok(value) = serde_json::from_slice(&persisted) {
                store.set(value);
            }
        }
        
        // Auto-save on changes
        if config.auto_save {
            store.subscribe({
                let backend = config.backend.clone();
                let key = config.key.clone();
                move |state| {
                    if let Ok(data) = serde_json::to_vec(state) {
                        let _ = backend.save(&key, &data);
                    }
                }
            });
        }
        
        Self { store, persistence: config.backend, config }
    }
}
```

## State Management Patterns

### 1. Actions Pattern
```rust
pub trait StoreActions<T> {
    fn update_state(&self, updater: impl FnOnce(&mut T));
}

// Example implementation
#[derive(Clone)]
pub struct CounterActions {
    setter: WriteSignal<CounterState>,
}

impl StoreActions<CounterState> for CounterActions {
    fn update_state(&self, updater: impl FnOnce(&mut CounterState)) {
        self.setter.update(updater);
    }
}

impl CounterActions {
    pub fn increment(&self) {
        self.update_state(|state| state.count += 1);
    }
    
    pub fn decrement(&self) {
        self.update_state(|state| state.count -= 1);
    }
    
    pub fn set_count(&self, count: i32) {
        self.update_state(|state| state.count = count);
    }
}
```

### 2. Reducer Pattern
```rust
pub trait Reducer<State, Action> {
    fn reduce(&self, state: &State, action: Action) -> State;
}

pub fn create_reducer_store<S, A, R>(
    initial: S,
    reducer: R,
) -> (ReadSignal<S>, impl Fn(A))
where
    S: Clone + 'static,
    A: 'static,
    R: Reducer<S, A> + 'static,
{
    let (state, set_state) = create_signal(initial);
    
    let dispatch = move |action: A| {
        set_state.update(|current| {
            *current = reducer.reduce(current, action);
        });
    };
    
    (state, dispatch)
}
```

### 3. Selector Pattern
```rust
pub fn create_selector<T, R>(
    signal: ReadSignal<T>,
    selector: impl Fn(&T) -> R + 'static,
) -> ReadSignal<R>
where
    T: 'static,
    R: Clone + PartialEq + 'static,
{
    create_memo(move |_| selector(&signal.get())).into()
}

// Usage example
let count_display = create_selector(
    counter_state,
    |state| format!("Count: {}", state.count)
);
```

## Middleware System

### Middleware Interface
```rust
pub trait Middleware<T> {
    fn before_update(&self, current: &T, next: &T) -> Result<(), MiddlewareError>;
    fn after_update(&self, previous: &T, current: &T);
    fn on_error(&self, error: &MiddlewareError);
}

pub struct MiddlewareChain<T> {
    middlewares: Vec<Box<dyn Middleware<T>>>,
}

impl<T> MiddlewareChain<T> {
    pub fn new() -> Self {
        Self { middlewares: Vec::new() }
    }
    
    pub fn add<M>(mut self, middleware: M) -> Self 
    where M: Middleware<T> + 'static
    {
        self.middlewares.push(Box::new(middleware));
        self
    }
    
    pub fn process_update(&self, current: &T, next: &T) -> Result<(), MiddlewareError> {
        // Run before_update for all middleware
        for middleware in &self.middlewares {
            middleware.before_update(current, next)?;
        }
        
        // Update happens here
        
        // Run after_update for all middleware
        for middleware in &self.middlewares {
            middleware.after_update(current, next);
        }
        
        Ok(())
    }
}
```

### Built-in Middleware

#### Logger Middleware
```rust
pub struct LoggerMiddleware {
    level: LogLevel,
}

impl<T> Middleware<T> for LoggerMiddleware 
where T: std::fmt::Debug
{
    fn before_update(&self, current: &T, next: &T) -> Result<(), MiddlewareError> {
        tracing::debug!("Store update: {:?} -> {:?}", current, next);
        Ok(())
    }
    
    fn after_update(&self, _previous: &T, current: &T) {
        tracing::info!("Store updated to: {:?}", current);
    }
}
```

#### Validation Middleware
```rust
pub struct ValidationMiddleware<T, V> {
    validator: V,
    _phantom: PhantomData<T>,
}

impl<T, V> Middleware<T> for ValidationMiddleware<T, V>
where V: Fn(&T) -> Result<(), ValidationError>
{
    fn before_update(&self, _current: &T, next: &T) -> Result<(), MiddlewareError> {
        (self.validator)(next)
            .map_err(|e| MiddlewareError::Validation(e))
    }
}
```

## Performance Optimizations

### 1. Memo-based Selectors
```rust
// Expensive computation cached automatically
let expensive_derived = create_memo(move |_| {
    expensive_computation(store_state.get())
});
```

### 2. Granular Updates
```rust
// Update only specific fields
pub struct AppState {
    user: RwSignal<User>,
    todos: RwSignal<Vec<Todo>>,
    ui: RwSignal<UiState>,
}

// Only user component re-renders when user changes
let user = app_state.user.get();
```

### 3. Batch Updates
```rust
pub fn batch_updates<F>(f: F) 
where F: FnOnce()
{
    // Use Leptos batching if available
    batch(f);
}

// Usage
batch_updates(|| {
    store.set_field1(value1);
    store.set_field2(value2);
    store.set_field3(value3);
});
```

## Integration Patterns

### 1. Context Provider Pattern
```rust
#[component]
pub fn StoreProvider<T>(
    initial: T,
    children: Children,
) -> impl IntoView 
where T: StoreState + 'static
{
    let store = create_store(initial);
    
    provide_context(store);
    
    children()
}

// Usage in component
#[component]
pub fn Counter() -> impl IntoView {
    let (count, actions) = use_store::<CounterState>();
    
    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| actions.increment()>
                "+"
            </button>
        </div>
    }
}
```

### 2. Multiple Store Composition
```rust
pub struct AppStores {
    pub user: UserStore,
    pub todos: TodoStore,
    pub ui: UiStore,
}

impl AppStores {
    pub fn new() -> Self {
        Self {
            user: create_store(UserState::default()),
            todos: create_store(TodoState::default()),
            ui: create_store(UiState::default()),
        }
    }
}
```

## Error Handling

### Store Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Middleware error: {0}")]
    Middleware(#[from] MiddlewareError),
    
    #[error("Persistence error: {0}")]
    Persistence(#[from] PersistenceError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Store not found in context")]
    NotFound,
}
```

### Error Recovery
```rust
impl<T> Store<T> {
    pub fn try_update<E>(&self, f: impl FnOnce(&mut T) -> Result<(), E>) -> Result<(), StoreError> 
    where E: Into<StoreError>
    {
        self.update(|state| {
            if let Err(e) = f(state) {
                // Log error and potentially revert state
                tracing::error!("Store update failed: {:?}", e);
                // Could implement state rollback here
            }
        });
        Ok(())
    }
}
```

## Testing Strategy

### Store Testing Utilities
```rust
pub mod testing {
    pub fn create_test_store<T>(initial: T) -> TestStore<T> {
        TestStore::new(initial)
    }
    
    pub struct TestStore<T> {
        store: BasicStore<T>,
        updates: Arc<Mutex<Vec<T>>>,
    }
    
    impl<T> TestStore<T> {
        pub fn get_update_history(&self) -> Vec<T> {
            self.updates.lock().unwrap().clone()
        }
        
        pub fn assert_updated_to(&self, expected: &T) 
        where T: PartialEq + Debug
        {
            assert_eq!(&self.store.get(), expected);
        }
    }
}
```

## Migration and Compatibility

### API Stability
- Core store interface is stable
- Middleware system may evolve
- Persistence backends can be added
- Testing utilities will expand

### Future Enhancements
1. **DevTools Integration:** Browser extension support
2. **Time Travel:** Undo/redo functionality
3. **Store Persistence:** Advanced backup strategies
4. **Performance Monitoring:** Built-in performance tracking

This architecture provides a solid foundation for reactive state management while maintaining flexibility for future enhancements.
