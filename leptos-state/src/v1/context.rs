//! # Context Implementation
//! 
//! This module provides the context implementation for state machines.

use super::traits::StateMachineContext;
use super::error::ContextError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Context for state machines that provides shared state and configuration
pub struct Context<C>
where
    C: StateMachineContext,
{
    /// The actual context data
    data: Arc<Mutex<C>>,
    /// Configuration options
    config: ContextConfig,
    /// Lifecycle timestamps
    lifecycle: LifecycleInfo,
    /// Metadata and tags
    metadata: HashMap<String, String>,
}

/// Configuration for the context
#[derive(Debug, Clone)]
pub struct ContextConfig {
    /// Whether to enable automatic persistence
    pub auto_persist: bool,
    /// Persistence interval
    pub persist_interval: Option<Duration>,
    /// Whether to enable logging
    pub enable_logging: bool,
    /// Whether to enable metrics collection
    pub enable_metrics: bool,
    /// Maximum context size in bytes
    pub max_size_bytes: Option<usize>,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            auto_persist: false,
            persist_interval: None,
            enable_logging: true,
            enable_metrics: true,
            max_size_bytes: None,
        }
    }
}

/// Lifecycle information for the context
#[derive(Debug, Clone)]
pub struct LifecycleInfo {
    /// When the context was created
    pub created_at: Instant,
    /// When the context was last accessed
    pub last_accessed: Instant,
    /// When the context was last modified
    pub last_modified: Instant,
    /// Number of times the context has been accessed
    pub access_count: usize,
    /// Number of times the context has been modified
    pub modification_count: usize,
}

impl Default for LifecycleInfo {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            created_at: now,
            last_accessed: now,
            last_modified: now,
            access_count: 0,
            modification_count: 0,
        }
    }
}

impl<C> Context<C>
where
    C: StateMachineContext + PartialEq,
{
    /// Create a new context with default configuration
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(C::default())),
            config: ContextConfig::default(),
            lifecycle: LifecycleInfo::default(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new context with custom data
    pub fn with_data(data: C) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
            config: ContextConfig::default(),
            lifecycle: LifecycleInfo::default(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new context with custom configuration
    pub fn with_config(data: C, config: ContextConfig) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
            config,
            lifecycle: LifecycleInfo::default(),
            metadata: HashMap::new(),
        }
    }

    /// Get a reference to the context data
    pub fn get(&self) -> Result<C, ContextError<C>> {
        let mut lifecycle = self.lifecycle.clone();
        lifecycle.last_accessed = Instant::now();
        lifecycle.access_count += 1;
        
        self.data
            .lock()
            .map_err(|_| ContextError::AccessDenied)
            .map(|guard| guard.clone())
    }

    /// Get a mutable reference to the context data
    pub fn get_mut(&mut self) -> Result<C, ContextError<C>> {
        let mut lifecycle = self.lifecycle.clone();
        lifecycle.last_accessed = Instant::now();
        lifecycle.last_modified = Instant::now();
        lifecycle.access_count += 1;
        lifecycle.modification_count += 1;
        
        self.data
            .lock()
            .map_err(|_| ContextError::AccessDenied)
            .map(|guard| guard.clone())
    }

    /// Update the context data
    pub fn update<F>(&mut self, f: F) -> Result<(), ContextError<C>>
    where
        F: FnOnce(&mut C) -> Result<(), ContextError<C>>,
    {
        let mut lifecycle = self.lifecycle.clone();
        lifecycle.last_accessed = Instant::now();
        lifecycle.last_modified = Instant::now();
        lifecycle.access_count += 1;
        lifecycle.modification_count += 1;

        let mut guard = self.data
            .lock()
            .map_err(|_| ContextError::AccessDenied)?;

        f(&mut *guard)?;
        
        self.lifecycle = lifecycle;
        Ok(())
    }

    /// Set the context data
    pub fn set(&mut self, data: C) -> Result<(), ContextError<C>> {
        let mut lifecycle = self.lifecycle.clone();
        lifecycle.last_accessed = Instant::now();
        lifecycle.last_modified = Instant::now();
        lifecycle.access_count += 1;
        lifecycle.modification_count += 1;

        let mut guard = self.data
            .lock()
            .map_err(|_| ContextError::AccessDenied)?;

        *guard = data;
        
        self.lifecycle = lifecycle;
        Ok(())
    }

    /// Get the configuration
    pub fn config(&self) -> &ContextConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, f: impl FnOnce(&mut ContextConfig)) {
        f(&mut self.config);
    }

    /// Get lifecycle information
    pub fn lifecycle(&self) -> &LifecycleInfo {
        &self.lifecycle
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Remove metadata
    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        self.metadata.remove(key)
    }

    /// Check if context is empty (has default values)
    pub fn is_empty(&self) -> bool {
        self.data
            .lock()
            .map(|guard| *guard == C::default())
            .unwrap_or(true)
    }

    /// Reset context to default values
    pub fn reset(&mut self) -> Result<(), ContextError<C>> {
        self.set(C::default())
    }

    /// Clone the context
    pub fn clone_context(&self) -> Self {
        let data = self.data
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| C::default());

        Self {
            data: Arc::new(Mutex::new(data)),
            config: self.config.clone(),
            lifecycle: self.lifecycle.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl<C> Clone for Context<C>
where
    C: StateMachineContext + PartialEq,
{
    fn clone(&self) -> Self {
        self.clone_context()
    }
}



impl<C> Default for Context<C>
where
    C: StateMachineContext + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default, PartialEq)]
    struct TestContext {
        count: i32,
        name: String,
    }

    impl StateMachineContext for TestContext {}

    #[test]
    fn test_context_creation() {
        let context = Context::<TestContext>::new();
        assert!(context.is_empty());
        assert_eq!(context.lifecycle().access_count, 0);
    }

    #[test]
    fn test_context_with_data() {
        let test_data = TestContext {
            count: 42,
            name: "test".to_string(),
        };
        let context = Context::with_data(test_data.clone());
        assert!(!context.is_empty());
        assert_eq!(context.get().unwrap(), test_data);
    }

    #[test]
    fn test_context_update() {
        let mut context = Context::<TestContext>::new();
        
        context.update(|data| {
            data.count = 100;
            data.name = "updated".to_string();
            Ok(())
        }).unwrap();

        let data = context.get().unwrap();
        assert_eq!(data.count, 100);
        assert_eq!(data.name, "updated");
        assert_eq!(context.lifecycle().modification_count, 1);
    }

    #[test]
    fn test_context_set() {
        let mut context = Context::<TestContext>::new();
        let test_data = TestContext {
            count: 999,
            name: "set_test".to_string(),
        };

        context.set(test_data.clone()).unwrap();
        assert_eq!(context.get().unwrap(), test_data);
    }

    #[test]
    fn test_context_metadata() {
        let mut context = Context::<TestContext>::new();
        
        context.add_metadata("key1", "value1");
        context.add_metadata("key2", "value2");
        
        assert_eq!(context.get_metadata("key1"), Some(&"value1".to_string()));
        assert_eq!(context.get_metadata("key2"), Some(&"value2".to_string()));
        assert_eq!(context.get_metadata("key3"), None);
        
        let removed = context.remove_metadata("key1");
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(context.get_metadata("key1"), None);
    }

    #[test]
    fn test_context_config() {
        let mut context = Context::<TestContext>::new();
        
        context.update_config(|config| {
            config.auto_persist = true;
            config.enable_logging = false;
        });
        
        assert!(context.config().auto_persist);
        assert!(!context.config().enable_logging);
    }

    #[test]
    fn test_context_reset() {
        let mut context = Context::with_data(TestContext {
            count: 999,
            name: "test".to_string(),
        });
        
        assert!(!context.is_empty());
        context.reset().unwrap();
        assert!(context.is_empty());
    }

    #[test]
    fn test_context_clone() {
        let mut context = Context::with_data(TestContext {
            count: 42,
            name: "original".to_string(),
        });
        
        context.add_metadata("key", "value");
        
        let cloned = context.clone();
        assert_eq!(cloned.get().unwrap(), context.get().unwrap());
        assert_eq!(cloned.get_metadata("key"), Some(&"value".to_string()));
        
        // Modifying original shouldn't affect clone
        context.update(|data| {
            data.count = 999;
            Ok(())
        }).unwrap();
        
        assert_ne!(cloned.get().unwrap().count, context.get().unwrap().count);
    }
}
