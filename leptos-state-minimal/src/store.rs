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
    middlewares: crate::middleware::MiddlewareStack<S>,
    #[cfg(all(feature = "web", feature = "devtools"))]
    devtools: Option<crate::devtools::DevToolsIntegration>,
    #[cfg(feature = "performance")]
    performance_monitor: Option<crate::performance::PerformanceMonitor>,
}

impl<S: State> Store<S> {
    /// Create a new store with initial state
    pub fn new(initial: S) -> Self {
        Self {
            signal: RwSignal::new(initial),
            subscribers: Vec::new(),
            middlewares: crate::middleware::MiddlewareStack::new(),
            #[cfg(all(feature = "web", feature = "devtools"))]
            devtools: None,
            #[cfg(feature = "performance")]
            performance_monitor: None,
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

    /// Add middleware to this store
    pub fn with_middleware<M: crate::middleware::Middleware<S> + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.middlewares = self.middlewares.add(middleware);
        self
    }

    /// Update the state using a closure with middleware processing
    ///
    /// Middleware will be executed in priority order before the state is updated.
    /// If any middleware sets should_continue to false, the update is cancelled.
    pub fn update_with_middleware<F>(&self, updater: F) -> StoreResult<()>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        let old_state = self.signal.get_untracked();
        let mut new_state = old_state.clone();

        // Apply updater
        updater(&mut new_state);

        // Create middleware context
        let mut ctx = crate::middleware::MiddlewareContext::<S, ()>::new(
            crate::middleware::Operation::StoreUpdate {
                old_state: old_state.clone(),
                new_state: new_state.clone(),
            }
        );

        // Process middleware
        self.middlewares.process(&mut ctx)?;

        if ctx.should_continue {
            // Apply the final state
            self.signal.set(new_state);

            // Notify subscribers
            let current = self.signal.get_untracked();
            for subscriber in &self.subscribers {
                subscriber(&current);
            }

            Ok(())
        } else {
            Err(crate::StoreError::UpdateFailed {
                reason: "Middleware cancelled the update".to_string(),
            })
        }
    }

    /// Reset the store to default state with middleware processing
    ///
    /// Requires the state type to implement Default.
    pub fn reset_with_middleware(&self) -> StoreResult<()>
    where
        S: Default,
    {
        let old_state = self.signal.get_untracked();
        let new_state = S::default();

        let mut ctx = crate::middleware::MiddlewareContext::<S, ()>::new(
            crate::middleware::Operation::StoreReset {
                old_state: old_state.clone(),
                new_state: new_state.clone(),
            }
        );

        self.middlewares.process(&mut ctx)?;

        if ctx.should_continue {
            self.signal.set(new_state);

            // Notify subscribers
            let current = self.signal.get_untracked();
            for subscriber in &self.subscribers {
                subscriber(&current);
            }

            Ok(())
        } else {
            Err(crate::StoreError::UpdateFailed {
                reason: "Middleware cancelled the reset".to_string(),
            })
        }
    }

    /// Get the middleware stack for this store
    pub fn middlewares(&self) -> &crate::middleware::MiddlewareStack<S> {
        &self.middlewares
    }

    /// Get a mutable reference to the middleware stack
    pub fn middlewares_mut(&mut self) -> &mut crate::middleware::MiddlewareStack<S> {
        &mut self.middlewares
    }

    /// Enable DevTools integration for this store
    ///
    /// Requires the devtools feature to be enabled.
    /// This allows real-time state inspection and debugging in browser DevTools.
    #[cfg(all(feature = "web", feature = "devtools"))]
    pub fn with_devtools(mut self, name: &str) -> Result<Self, crate::devtools::DevToolsError> {
        self.devtools = Some(crate::devtools::DevToolsIntegration::new(name.to_string())?);
        Ok(self)
    }

    /// Check if DevTools integration is enabled
    #[cfg(all(feature = "web", feature = "devtools"))]
    pub fn has_devtools(&self) -> bool {
        self.devtools.is_some()
    }

    /// Get the DevTools integration (if enabled)
    #[cfg(all(feature = "web", feature = "devtools"))]
    pub fn devtools(&self) -> Option<&crate::devtools::DevToolsIntegration> {
        self.devtools.as_ref()
    }

    /// Enable performance monitoring for this store
    ///
    /// Requires the performance feature to be enabled.
    /// This allows tracking performance metrics for store operations.
    #[cfg(feature = "performance")]
    pub fn with_performance_monitoring(mut self, monitor: crate::performance::PerformanceMonitor) -> Self {
        self.performance_monitor = Some(monitor);
        self
    }

    /// Check if performance monitoring is enabled
    #[cfg(feature = "performance")]
    pub fn has_performance_monitoring(&self) -> bool {
        self.performance_monitor.is_some()
    }

    /// Get the performance monitor (if enabled)
    #[cfg(feature = "performance")]
    pub fn performance_monitor(&self) -> Option<&crate::performance::PerformanceMonitor> {
        self.performance_monitor.as_ref()
    }

    /// Get performance metrics for this store
    #[cfg(feature = "performance")]
    pub fn get_performance_metrics(&self) -> Option<crate::performance::PerformanceMetrics> {
        self.performance_monitor.as_ref().map(|pm| pm.get_metrics())
    }

    /// Get performance bottlenecks for this store
    #[cfg(feature = "performance")]
    pub fn get_performance_bottlenecks(&self) -> Vec<(String, crate::performance::DurationStats)> {
        self.performance_monitor
            .as_ref()
            .map(|pm| pm.get_bottlenecks())
            .unwrap_or_default()
    }

    /// Get performance recommendations for this store
    #[cfg(feature = "performance")]
    pub fn get_performance_recommendations(&self) -> Vec<String> {
        self.performance_monitor
            .as_ref()
            .map(|pm| pm.get_recommendations())
            .unwrap_or_default()
    }

    /// Serialize the current state to JSON string
    ///
    /// Requires the serde feature and SerializableState bound.
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String, crate::StoreError>
    where
        S: crate::SerializableState,
    {
        let state = self.signal.get_untracked();
        serde_json::to_string(&state)
            .map_err(|e| crate::StoreError::SerializationError {
                message: e.to_string(),
            })
    }

    /// Deserialize state from JSON string and update the store
    ///
    /// Requires the serde feature and SerializableState bound.
    #[cfg(feature = "serde")]
    pub fn from_json(&self, json: &str) -> Result<(), crate::StoreError>
    where
        S: crate::SerializableState,
    {
        let state: S = serde_json::from_str(json)
            .map_err(|e| crate::StoreError::DeserializationError {
                message: e.to_string(),
            })?;
        self.set(state)
    }

    /// Export state as a snapshot with metadata
    ///
    /// Requires the serde feature and SerializableState bound.
    #[cfg(feature = "serde")]
    pub fn export_snapshot(&self) -> Result<crate::StateSnapshot<S>, crate::StoreError>
    where
        S: crate::SerializableState,
    {
        Ok(crate::StateSnapshot {
            data: self.signal.get_untracked(),
            timestamp: std::time::SystemTime::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }

    /// Import state from a snapshot
    ///
    /// Validates version compatibility before importing.
    /// Requires the serde feature and SerializableState bound.
    #[cfg(feature = "serde")]
    pub fn import_snapshot(&self, snapshot: crate::StateSnapshot<S>) -> Result<(), crate::StoreError>
    where
        S: crate::SerializableState,
    {
        // Version compatibility check
        let current_version = env!("CARGO_PKG_VERSION");
        if snapshot.version != current_version {
            return Err(crate::StoreError::VersionMismatch {
                expected: current_version.to_string(),
                found: snapshot.version,
            });
        }

        self.set(snapshot.data)
    }
}

impl<S: State> Clone for Store<S> {
    fn clone(&self) -> Self {
        Self {
            signal: self.signal,
            subscribers: self.subscribers.clone(),
            middlewares: self.middlewares.clone(),
            #[cfg(all(feature = "web", feature = "devtools"))]
            devtools: None, // Don't clone DevTools integration
            #[cfg(feature = "performance")]
            performance_monitor: self.performance_monitor.clone(), // Clone performance monitor
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

    #[derive(Clone, Default, Debug, Eq, PartialEq)]
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
