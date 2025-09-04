use crate::v1::traits::StoreState;
use leptos::prelude::*;

/// Provide a store context to child components
pub fn provide_store<S: StoreState + Clone + 'static>(initial: S) {
    let (state, set_state) = signal(initial);
    provide_context((state, set_state));
}

/// Hook to access a store's state and setter
pub fn use_store<S: StoreState + Clone + 'static>() -> (ReadSignal<S>, WriteSignal<S>) {
    use_context::<(ReadSignal<S>, WriteSignal<S>)>()
        .expect("Store not provided - did you forget to call provide_store?")
}

/// Hook to access a store's state and setter with a default value
pub fn use_store_with_default<S: StoreState + Clone + 'static>(default: S) -> (ReadSignal<S>, WriteSignal<S>) {
    use_context::<(ReadSignal<S>, WriteSignal<S>)>()
        .unwrap_or_else(|| {
            let (state, set_state) = signal(default);
            (state, set_state)
        })
}

/// Hook to create a computed value from store state
pub fn use_computed<S: StoreState, T: PartialEq + Clone + Send + Sync + 'static>(
    selector: impl Fn(&S) -> T + Send + Sync + 'static,
) -> Memo<T> {
    let (state, _) = use_store::<S>();
    Memo::new(move |_| selector(&state.get()))
}

/// Hook for store actions (functions that update store state)
pub fn use_store_actions<S: StoreState + Clone + Send + Sync + 'static>() -> StoreActions<S> {
    let (_, set_state) = use_store::<S>();
    StoreActions::new(set_state)
}

/// Hook to access a slice of store state
pub fn use_store_slice<S: StoreState + Clone + 'static, T: PartialEq + Clone + Send + Sync + 'static>(
    selector: impl Fn(&S) -> T + Send + Sync + 'static,
) -> Memo<T> {
    use_computed(selector)
}

/// Hook for batch store updates
pub fn use_store_batch<S: StoreState + Clone + Send + Sync + 'static>() -> StoreBatch<S> {
    let (_, set_state) = use_store::<S>();
    StoreBatch::new(set_state)
}

/// Hook for store history
pub fn use_store_history<S: StoreState + Clone + Send + Sync + 'static>() -> StoreHistory<S> {
    let (state, set_state) = use_store::<S>();
    StoreHistory::new(state, set_state)
}

/// Hook for store persistence
pub fn use_store_persistence<S: StoreState + Clone + Send + Sync + 'static>(
    key: &'static str,
) -> StorePersistence<S> {
    let (state, set_state) = use_store::<S>();
    StorePersistence::new(key, state, set_state)
}

/// Hook for store subscriptions
pub fn use_store_subscription<S: StoreState + Clone + Send + Sync + 'static, F>(
    callback: F,
) where
    F: Fn(&S) + 'static,
{
    let (state, _) = use_store::<S>();
    Effect::new(move |_| {
        let current_state = state.get();
        callback(&current_state);
    });
}

/// Hook for store middleware
pub fn use_store_middleware<S: StoreState + Clone + Send + Sync + 'static>() -> StoreMiddleware<S> {
    let (state, set_state) = use_store::<S>();
    StoreMiddleware::new(state, set_state)
}

/// Helper struct for common store actions
pub struct StoreActions<T: Clone + Send + Sync + 'static> {
    setter: WriteSignal<T>,
}

impl<T: Clone + Send + Sync> StoreActions<T> {
    pub fn new(setter: WriteSignal<T>) -> Self {
        Self { setter }
    }

    /// Set the entire state
    pub fn set(&self, new_state: T) {
        self.setter.set(new_state);
    }

    /// Update state with a function
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.setter.update(f);
    }

    /// Update state with a mapping function
    pub fn map(&self, f: impl FnOnce(T) -> T) {
        self.setter.update(|state| *state = f(state.clone()));
    }

    /// Reset to initial state
    pub fn reset(&self) {
        // Note: This would need the actual type T to implement Default
        // For now, this is a placeholder
    }
}

/// Helper struct for batch store updates
pub struct StoreBatch<T: Clone + Send + Sync + 'static> {
    setter: WriteSignal<T>,
}

impl<T: Clone + Send + Sync> StoreBatch<T> {
    pub fn new(setter: WriteSignal<T>) -> Self {
        Self { setter }
    }

    /// Apply multiple updates in a batch
    pub fn batch(&self, updates: Vec<impl FnOnce(&mut T)>) {
        self.setter.update(|state| {
            for update in updates {
                update(state);
            }
        });
    }
}

/// Helper struct for store history
pub struct StoreHistory<T: Clone + Send + Sync + 'static> {
    state: ReadSignal<T>,
    setter: WriteSignal<T>,
    history: Vec<T>,
}

impl<T: Clone + Send + Sync> StoreHistory<T> {
    pub fn new(state: ReadSignal<T>, setter: WriteSignal<T>) -> Self {
        Self {
            state,
            setter,
            history: Vec::new(),
        }
    }

    /// Save current state to history
    pub fn save(&mut self) {
        self.history.push(self.state.get());
    }

    /// Restore previous state from history
    pub fn undo(&mut self) -> Option<T> {
        self.history.pop()
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.history.clear();
    }
}

/// Helper struct for store persistence
pub struct StorePersistence<T: Clone + Send + Sync + 'static> {
    key: &'static str,
    state: ReadSignal<T>,
    setter: WriteSignal<T>,
}

impl<T: Clone + Send + Sync> StorePersistence<T> {
    pub fn new(key: &'static str, state: ReadSignal<T>, set_state: WriteSignal<T>) -> Self {
        Self {
            key,
            state,
            setter: set_state,
        }
    }

    /// Load state from storage
    pub fn load(&self) {
        // TODO: Implement actual storage loading
        // This is a placeholder for the persistence feature
    }

    /// Save state to storage
    pub fn save(&self) {
        // TODO: Implement actual storage saving
        // This is a placeholder for the persistence feature
    }
}

/// Helper struct for store middleware
pub struct StoreMiddleware<T: Clone + Send + Sync + 'static> {
    state: ReadSignal<T>,
    setter: WriteSignal<T>,
}

impl<T: Clone + Send + Sync> StoreMiddleware<T> {
    pub fn new(state: ReadSignal<T>, set_state: WriteSignal<T>) -> Self {
        Self {
            state,
            setter: set_state,
        }
    }

    /// Apply middleware transformation
    pub fn transform<F>(&self, transformer: F)
    where
        F: FnOnce(T) -> T + 'static,
    {
        self.setter.update(|state| *state = transformer(state.clone()));
    }
}
