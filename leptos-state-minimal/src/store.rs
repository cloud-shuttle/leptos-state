//! Reactive store implementation

use crate::{State, StoreResult};
use leptos::prelude::*;
use std::sync::Arc;

/// A reactive store that manages application state
///
/// The store uses Leptos signals internally for reactivity and provides
/// a simple API for state management.
pub struct Store<S: State> {
    signal: RwSignal<S>,
    subscribers: Vec<Arc<dyn Fn(&S) + Send + Sync>>,
}

impl<S: State> Store<S> {
    /// Create a new store with initial state
    pub fn new(initial: S) -> Self {
        Self {
            signal: RwSignal::new(initial),
            subscribers: Vec::new(),
        }
    }

    /// Get a read-only signal for the current state
    pub fn get(&self) -> ReadSignal<S> {
        self.signal.read_only()
    }

    /// Update the state using a closure
    ///
    /// The closure receives a mutable reference to the current state
    /// and can modify it as needed.
    pub fn update<F>(&self, updater: F) -> StoreResult<()>
    where
        F: FnOnce(&mut S) + 'static,
    {
        // Update the signal
        self.signal.update(updater);

        // Notify subscribers
        let current = self.signal.get_untracked();
        for subscriber in &self.subscribers {
            subscriber(&current);
        }

        Ok(())
    }

    /// Set the state to a new value
    pub fn set(&self, new_state: S) -> StoreResult<()> {
        self.signal.set(new_state);

        // Notify subscribers
        let current = self.signal.get_untracked();
        for subscriber in &self.subscribers {
            subscriber(&current);
        }

        Ok(())
    }

    /// Add a subscriber that gets notified of state changes
    ///
    /// Subscribers are called after each state update.
    pub fn subscribe<F>(&mut self, callback: F) -> StoreResult<()>
    where
        F: Fn(&S) + Send + Sync + 'static,
    {
        self.subscribers.push(Arc::new(callback));
        Ok(())
    }

    /// Get the current state value (not reactive)
    ///
    /// This is useful for one-time reads outside of reactive contexts.
    pub fn current(&self) -> S
    where
        S: Clone,
    {
        self.signal.get_untracked()
    }

    /// Create a store with default state
    ///
    /// Requires the state type to implement Default.
    pub fn default() -> Self
    where
        S: Default,
    {
        Self::new(S::default())
    }
}

impl<S: State> Clone for Store<S> {
    fn clone(&self) -> Self {
        Self {
            signal: self.signal,
            subscribers: self.subscribers.clone(),
        }
    }
}

/// Actions for interacting with a store
///
/// This provides a convenient interface for updating store state
/// in Leptos components.
pub struct StoreActions<S: State> {
    store: Store<S>,
}

impl<S: State> StoreActions<S> {
    /// Create new store actions for a store
    pub fn new(store: Store<S>) -> Self {
        Self { store }
    }

    /// Update the store state
    pub fn update<F>(&self, updater: F) -> StoreResult<()>
    where
        F: FnOnce(&mut S) + 'static,
    {
        self.store.update(updater)
    }

    /// Set the store state to a new value
    pub fn set(&self, new_state: S) -> StoreResult<()> {
        self.store.set(new_state)
    }

    /// Reset the store to default state
    pub fn reset(&self) -> StoreResult<()>
    where
        S: Default,
    {
        self.set(S::default())
    }

    /// Get the current state (non-reactive)
    pub fn current(&self) -> S
    where
        S: Clone,
    {
        self.store.current()
    }
}

impl<S: State> Clone for StoreActions<S> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug)]
    struct TestState {
        count: i32,
    }

    impl TestState {
        fn new(count: i32) -> Self {
            Self { count }
        }
    }

    #[test]
    fn store_creation_works() {
        let store = Store::new(TestState::new(0));
        let current = store.current();
        assert_eq!(current.count, 0);
    }

    #[test]
    fn store_update_works() {
        let store = Store::new(TestState::new(0));
        store.update(|state| state.count = 42).unwrap();
        let current = store.current();
        assert_eq!(current.count, 42);
    }

    #[test]
    fn store_set_works() {
        let store = Store::new(TestState::new(0));
        store.set(TestState::new(100)).unwrap();
        let current = store.current();
        assert_eq!(current.count, 100);
    }
}
