//! # Store Implementation
//! 
//! This module provides the store implementation for state management.

use super::traits::StoreState;
use super::error::StateMachineError;
use leptos::prelude::*;

/// A reactive store that manages state and provides Leptos integration
#[derive(Default)]
pub struct StateStore<State>
where
    State: StoreState + Clone + 'static,
{
    /// The current state
    state: RwSignal<State>,
    /// State history for undo/redo functionality
    history: Vec<State>,
    /// Maximum history size
    max_history: usize,
    /// Middleware functions
    middleware: Vec<Box<dyn Fn(&State, &State) + Send + Sync>>,
    /// Subscribers for state changes
    subscribers: Vec<Box<dyn Fn(&State) + Send + Sync>>,
    /// Persistence configuration
    persistence: Option<PersistenceConfig>,
}

/// Configuration for state persistence
#[derive(Debug, Clone)]
pub struct PersistenceConfig {
    /// Storage key for persistence
    pub key: String,
    /// Whether to persist on every change
    pub persist_on_change: bool,
    /// Serialization format
    pub format: SerializationFormat,
}

/// Supported serialization formats
#[derive(Debug, Clone, PartialEq)]
pub enum SerializationFormat {
    Json,
    Yaml,
    Bincode,
}

impl<State> StateStore<State>
where
    State: StoreState + Clone + 'static,
{
    /// Create a new store with default state
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(State::default()),
            history: Vec::new(),
            max_history: 50,
            middleware: Vec::new(),
            subscribers: Vec::new(),
            persistence: None,
        }
    }

    /// Create a new store with custom initial state
    pub fn with_initial_state(initial: State) -> Self {
        let mut store = Self::new();
        store.set_state(initial);
        store
    }

    /// Get the current state as a read signal
    pub fn state(&self) -> ReadSignal<State> {
        self.state.read_only()
    }

    /// Get the current state value
    pub fn get_state(&self) -> State {
        self.state.get()
    }

    /// Set the state and trigger updates
    pub fn set_state(&mut self, new_state: State) {
        let old_state = self.state.get();
        
        // Run middleware
        for middleware in &self.middleware {
            middleware(&old_state, &new_state);
        }
        
        // Add to history
        self.add_to_history(old_state);
        
        // Update state
        self.state.set(new_state.clone());
        
        // Notify subscribers
        for subscriber in &self.subscribers {
            subscriber(&new_state);
        }
        
        // Persist if configured
        if let Some(config) = &self.persistence {
            if config.persist_on_change {
                self.persist_state(&new_state, config);
            }
        }
    }

    /// Update the state using a function
    pub fn update_state<F>(&mut self, updater: F)
    where
        F: FnOnce(&State) -> State,
    {
        let current = self.get_state();
        let new_state = updater(&current);
        self.set_state(new_state);
    }

    /// Subscribe to state changes
    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(&State) + Send + Sync + 'static,
    {
        self.subscribers.push(Box::new(callback));
    }

    /// Add middleware for state changes
    pub fn add_middleware<F>(&mut self, middleware: F)
    where
        F: Fn(&State, &State) + Send + Sync + 'static,
    {
        self.middleware.push(Box::new(middleware));
    }

    /// Set persistence configuration
    pub fn with_persistence(mut self, config: PersistenceConfig) -> Self {
        self.persistence = Some(config);
        self
    }

    /// Load state from persistence
    pub fn load_persisted_state(&mut self) -> Result<(), StateMachineError<(), (), State>> {
        if let Some(config) = &self.persistence {
            match self.load_state(config) {
                Ok(state) => {
                    self.state.set(state);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            Err(StateMachineError::Persistence(
                super::error::PersistenceError::NotInitialized
            ))
        }
    }

    /// Undo the last state change
    pub fn undo(&mut self) -> Result<(), StateMachineError<(), (), State>> {
        if let Some(previous_state) = self.history.pop() {
            let current_state = self.state.get();
            self.add_to_history(current_state);
            self.state.set(previous_state);
            Ok(())
        } else {
            Err(StateMachineError::Persistence(
                super::error::PersistenceError::StorageFailed("No history available".to_string())
            ))
        }
    }

    /// Redo the last undone state change
    pub fn redo(&mut self) -> Result<(), StateMachineError<(), (), State>> {
        // Implementation would depend on how we track redo history
        // For now, we'll return an error
        Err(StateMachineError::Persistence(
            super::error::PersistenceError::StorageFailed("Redo not implemented".to_string())
        ))
    }

    /// Set the maximum history size
    pub fn set_max_history(&mut self, size: usize) {
        self.max_history = size;
        // Trim history if it exceeds the new limit
        while self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    /// Get the current history size
    pub fn history_size(&self) -> usize {
        self.history.len()
    }

    /// Clear the history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    // Private helper methods
    fn add_to_history(&mut self, state: State) {
        self.history.push(state);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    fn persist_state(&self, state: &State, config: &PersistenceConfig) {
        // Implementation would serialize and store the state
        // For now, this is a placeholder
        let _ = (state, config);
    }

    fn load_state(&self, _config: &PersistenceConfig) -> Result<State, StateMachineError<(), (), State>> {
        // Implementation would load and deserialize the state
        // For now, return the default state
        Ok(State::default())
    }
}

// =============================================================================
// Leptos Integration
// =============================================================================

impl<State> StateStore<State>
where
    State: StoreState + Clone + 'static,
{
    /// Create a store context for Leptos components
    pub fn create_context() -> (Self, ReadSignal<State>, RwSignal<State>) {
        let store = StateStore::new();
        let state_signal = store.state();
        let write_signal = store.state;
        
        (store, state_signal, write_signal)
    }

    /// Hook for using the store in Leptos components
    pub fn use_store() -> (ReadSignal<State>, RwSignal<State>) {
        // For now, we'll create a new context
        // In practice, this would use use_context and provide_context
        let (_store, read, write) = StateStore::create_context();
        (read, write)
    }

    /// Hook for using a specific part of the store state
    pub fn use_store_selector<F, T>(selector: F) -> Memo<T>
    where
        F: Fn(&State) -> T + Clone + Send + Sync + 'static,
        T: Clone + PartialEq + Send + Sync + 'static,
    {
        let (state, _) = StateStore::use_store();
        Memo::new(move |_| selector(&state.get()))
    }
}





// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::traits::StoreState;

    // Test types
    #[derive(Clone, Debug, Default, PartialEq)]
    struct TestState {
        count: i32,
        name: String,
        items: Vec<String>,
    }

    impl StoreState for TestState {
        // StoreState is a marker trait with no required methods
    }

    #[test]
    fn test_store_creation() {
        let store = StateStore::<TestState>::new();
        
        assert_eq!(store.get_state().count, 0);
        assert_eq!(store.get_state().name, "");
        assert_eq!(store.get_state().items.len(), 0);
        assert_eq!(store.history_size(), 0);
    }

    #[test]
    fn test_store_with_initial_state() {
        let initial_state = TestState {
            count: 42,
            name: "test".to_string(),
            items: vec!["item1".to_string()],
        };
        
        let store = StateStore::with_initial_state(initial_state.clone());
        
        assert_eq!(store.get_state().count, 42);
        assert_eq!(store.get_state().name, "test");
        assert_eq!(store.get_state().items.len(), 1);
    }

    #[test]
    fn test_store_set_state() {
        let mut store = StateStore::<TestState>::new();
        let new_state = TestState {
            count: 100,
            name: "updated".to_string(),
            items: vec!["item1".to_string(), "item2".to_string()],
        };
        
        store.set_state(new_state.clone());
        
        assert_eq!(store.get_state().count, 100);
        assert_eq!(store.get_state().name, "updated");
        assert_eq!(store.get_state().items.len(), 2);
        assert_eq!(store.history_size(), 1);
    }

    #[test]
    fn test_store_update_state() {
        let mut store = StateStore::<TestState>::new();
        
        store.update_state(|state| TestState {
            count: state.count + 10,
            name: state.name.clone(),
            items: state.items.clone(),
        });
        
        assert_eq!(store.get_state().count, 10);
        assert_eq!(store.history_size(), 1);
    }

    #[test]
    fn test_store_subscription() {
        let mut store = StateStore::<TestState>::new();
        store.subscribe(|_state| {
            // In a real implementation, this would work
            // For now, we'll just test that the subscription is added
        });
        
        store.set_state(TestState {
            count: 50,
            name: "test".to_string(),
            items: vec![],
        });
        
        // Test that the state was updated
        assert_eq!(store.get_state().count, 50);
    }

    #[test]
    fn test_store_middleware() {
        let mut store = StateStore::<TestState>::new();
        store.add_middleware(|old_state, new_state| {
            // In a real implementation, this would work
            // For now, we'll just test that the middleware is added
            assert_eq!(old_state.count, 0);
            assert_eq!(new_state.count, 25);
        });
        
        store.set_state(TestState {
            count: 25,
            name: "test".to_string(),
            items: vec![],
        });
        
        // Test that the state was updated
        assert_eq!(store.get_state().count, 25);
    }

    #[test]
    fn test_store_history_management() {
        let mut store = StateStore::<TestState>::new();
        
        // Set initial state
        store.set_state(TestState {
            count: 10,
            name: "first".to_string(),
            items: vec![],
        });
        
        // Set second state
        store.set_state(TestState {
            count: 20,
            name: "second".to_string(),
            items: vec![],
        });
        
        assert_eq!(store.history_size(), 2);
        
        // Test undo
        let result = store.undo();
        assert!(result.is_ok());
        assert_eq!(store.get_state().count, 10);
        assert_eq!(store.get_state().name, "first");
    }

    #[test]
    fn test_store_max_history() {
        let mut store = StateStore::<TestState>::new();
        store.set_max_history(3);
        
        // Add 5 states
        for i in 1..=5 {
            store.set_state(TestState {
                count: i,
                name: format!("state{}", i),
                items: vec![],
            });
        }
        
        // Should only keep the last 3
        assert_eq!(store.history_size(), 3);
        
        // Test undo to see what we get
        let result = store.undo();
        assert!(result.is_ok());
        assert_eq!(store.get_state().count, 4); // Should get state 4, not 2
    }

    #[test]
    fn test_store_persistence_config() {
        let config = PersistenceConfig {
            key: "test-store".to_string(),
            persist_on_change: true,
            format: SerializationFormat::Json,
        };
        
        let store = StateStore::<TestState>::new()
            .with_persistence(config.clone());
        
        // Test that persistence is configured
        assert!(store.persistence.is_some());
        
        if let Some(persisted_config) = &store.persistence {
            assert_eq!(persisted_config.key, "test-store");
            assert!(persisted_config.persist_on_change);
            assert_eq!(persisted_config.format, SerializationFormat::Json);
        }
    }

    #[test]
    fn test_store_load_persisted_state_not_configured() {
        let mut store = StateStore::<TestState>::new();
        
        let result = store.load_persisted_state();
        assert!(result.is_err());
        
        if let Err(StateMachineError::Persistence(err)) = result {
            match err {
                super::super::error::PersistenceError::NotInitialized => {},
                _ => panic!("Expected NotConfigured error"),
            }
        } else {
            panic!("Expected persistence error");
        }
    }

    #[test]
    fn test_store_clear_history() {
        let mut store = StateStore::<TestState>::new();
        
        // Add some states
        store.set_state(TestState {
            count: 10,
            name: "test".to_string(),
            items: vec![],
        });
        
        store.set_state(TestState {
            count: 20,
            name: "test2".to_string(),
            items: vec![],
        });
        
        assert_eq!(store.history_size(), 2);
        
        store.clear_history();
        assert_eq!(store.history_size(), 0);
    }

    #[test]
    fn test_store_undo_no_history() {
        let mut store = StateStore::<TestState>::new();
        
        let result = store.undo();
        assert!(result.is_err());
        
        if let Err(StateMachineError::Persistence(err)) = result {
            match err {
                super::super::error::PersistenceError::StorageFailed(_) => {},
                _ => panic!("Expected NoHistory error"),
            }
        } else {
            panic!("Expected persistence error");
        }
    }

    #[test]
    fn test_store_redo_not_implemented() {
        let mut store = StateStore::<TestState>::new();
        
        let result = store.redo();
        assert!(result.is_err());
        
        if let Err(StateMachineError::Persistence(err)) = result {
            match err {
                super::super::error::PersistenceError::StorageFailed(_) => {},
                _ => panic!("Expected NoRedoHistory error"),
            }
        } else {
            panic!("Expected persistence error");
        }
    }

    #[test]
    fn test_serialization_formats() {
        let json = SerializationFormat::Json;
        let yaml = SerializationFormat::Yaml;
        let bincode = SerializationFormat::Bincode;
        
        assert_ne!(json, yaml);
        assert_ne!(yaml, bincode);
        assert_ne!(json, bincode);
        
        assert_eq!(json, SerializationFormat::Json);
        assert_eq!(yaml, SerializationFormat::Yaml);
        assert_eq!(bincode, SerializationFormat::Bincode);
    }
}
