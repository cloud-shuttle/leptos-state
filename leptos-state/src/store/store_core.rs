//! Core store trait and context wrapper

use super::*;

/// Core trait for defining stores
pub trait Store: Send + Sync + 'static {
    /// The state type this store manages
    type State: Clone + PartialEq + Send + Sync + 'static;

    /// Get the current state
    fn get(&self) -> Self::State;

    /// Set the state
    fn set(&self, state: Self::State);

    /// Update the state using a function
    fn update_boxed(&self, f: Box<dyn FnOnce(Self::State) -> Self::State + Send + Sync>);

    /// Update the state using a function (convenience method)
    fn update<F>(&self, f: F)
    where
        F: FnOnce(Self::State) -> Self::State + Send + Sync + 'static,
    {
        self.update_boxed(Box::new(f));
    }
}

/// Context wrapper for store state
#[derive(Clone, Debug)]
pub struct StoreContext<T: Clone + PartialEq + 'static> {
    /// The store instance
    pub store: std::rc::Rc<dyn Store<State = T>>,
}

impl<T: Clone + PartialEq + 'static> StoreContext<T> {
    /// Create a new store context
    pub fn new(store: impl Store<State = T> + 'static) -> Self {
        Self {
            store: std::rc::Rc::new(store),
        }
    }

    /// Get the current state
    pub fn get(&self) -> T {
        self.store.get()
    }

    /// Set the state
    pub fn set(&self, state: T) {
        self.store.set(state);
    }

    /// Update the state using a function
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        self.store.update(f);
    }
}

/// Create a store with the given state type and initial value
pub fn create_store<T: Clone + PartialEq + 'static>(initial: T) -> StoreContext<T> {
    StoreContext::new(SimpleStore::new(initial))
}

/// Provide a store context to child components
#[macro_export]
macro_rules! provide_store {
    ($store:expr) => {
        leptos::provide_context($store);
    };
}

/// Provide a store with persistence loading from localStorage
#[macro_export]
macro_rules! provide_persistent_store {
    ($key:expr, $initial:expr) => {{
        let initial_state = $crate::store::load_from_local_storage($key).unwrap_or($initial);
        let store = $crate::store::create_store(initial_state);

        // Set up persistence
        $crate::store::persist_to_local_storage($key, store.clone());

        leptos::provide_context(store);
    }};
}
