//! Performance optimization utilities for state machines
//! 
//! This module provides various optimization strategies for improving
//! state machine performance, including transition caching, lazy evaluation,
//! and batch updates.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use crate::machine::{Machine, MachineBuilder, Event};

/// Transition cache for optimizing repeated state transitions
pub struct OptimizationCache<C, E> {
    cache: Arc<RwLock<HashMap<TransitionKey, TransitionResult>>>,
    max_size: usize,
    _phantom: std::marker::PhantomData<(C, E)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TransitionKey {
    state: String,
    event: String,
    context_hash: u64,
}

#[derive(Debug, Clone)]
struct TransitionResult {
    success: bool,
    transition_time: f64,
}

impl<C, E> OptimizationCache<C, E>
where
    C: Clone + Hash + Send + Sync + std::fmt::Debug + 'static,
    E: Clone + Hash + Send + Sync + std::fmt::Debug + 'static,
{
    /// Create a new transition cache with the specified maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get a cached transition result
    pub fn get(&self, state: &str, event: &E, context: &C) -> Option<TransitionResult> {
        let key = self.create_key(state, event, context);
        self.cache.read().unwrap().get(&key).cloned()
    }

    /// Cache a transition result
    pub fn set(&self, state: &str, event: &E, context: &C, result: TransitionResult) {
        let key = self.create_key(state, event, context);
        let mut cache = self.cache.write().unwrap();
        
        // Evict old entries if cache is full
        if cache.len() >= self.max_size {
            self.evict_oldest(&mut cache);
        }
        
        cache.insert(key, result);
    }

    /// Clear the cache
    pub fn clear(&self) {
        self.cache.write().unwrap().clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let cache = self.cache.read().unwrap();
        CacheStats {
            size: cache.len(),
            max_size: self.max_size,
            hit_rate: 0.0, // TODO: Implement hit rate tracking
        }
    }

    fn create_key(&self, state: &str, event: &E, context: &C) -> TransitionKey {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        context.hash(&mut hasher);
        let context_hash = hasher.finish();

        TransitionKey {
            state: state.to_string(),
            event: format!("{:?}", event),
            context_hash,
        }
    }

    fn evict_oldest(&self, cache: &mut HashMap<TransitionKey, TransitionResult>) {
        // Simple LRU eviction - remove first entry
        if let Some(key) = cache.keys().next().cloned() {
            cache.remove(&key);
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub hit_rate: f64,
}

/// Batch update manager for grouping multiple state changes
#[derive(Debug, Clone)]
pub struct BatchUpdateManager<C, E> {
    pending_updates: Vec<PendingUpdate<C, E>>,
    batch_size: usize,
}

#[derive(Debug, Clone)]
struct PendingUpdate<C, E> {
    event: E,
    context: C,
}

impl<C, E> BatchUpdateManager<C, E>
where
    C: Clone + Send + Sync + std::fmt::Debug + 'static,
    E: Clone + Send + Sync + std::fmt::Debug + 'static,
{
    /// Create a new batch update manager
    pub fn new(batch_size: usize) -> Self {
        Self {
            pending_updates: Vec::new(),
            batch_size,
        }
    }

    /// Add an update to the batch
    pub fn add_update(&mut self, event: E, context: C) {
        self.pending_updates.push(PendingUpdate { event, context });
    }

    /// Process all pending updates
    pub fn process_batch(&mut self) -> Vec<(E, C)> {
        std::mem::take(&mut self.pending_updates)
            .into_iter()
            .map(|update| (update.event, update.context))
            .collect()
    }

    /// Check if batch is ready to process
    pub fn is_ready(&self) -> bool {
        self.pending_updates.len() >= self.batch_size
    }

    /// Get current batch size
    pub fn current_size(&self) -> usize {
        self.pending_updates.len()
    }
}

/// Lazy evaluation manager for deferring state computation
#[derive(Debug, Clone)]
pub struct LazyEvaluationManager<S> {
    computed_states: HashMap<String, S>,
    computation_cache: HashMap<String, bool>,
}

impl<S> LazyEvaluationManager<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Create a new lazy evaluation manager
    pub fn new() -> Self {
        Self {
            computed_states: HashMap::new(),
            computation_cache: HashMap::new(),
        }
    }

    /// Get a computed state or compute it lazily
    pub fn get_or_compute<F>(&mut self, key: &str, compute_fn: F) -> S
    where
        F: FnOnce() -> S,
    {
        if let Some(state) = self.computed_states.get(key) {
            state.clone()
        } else {
            let state = compute_fn();
            self.computed_states.insert(key.to_string(), state.clone());
            state
        }
    }

    /// Check if a state has been computed
    pub fn is_computed(&self, key: &str) -> bool {
        self.computed_states.contains_key(key)
    }

    /// Clear computed states
    pub fn clear(&mut self) {
        self.computed_states.clear();
        self.computation_cache.clear();
    }
}

impl<S> Default for LazyEvaluationManager<S> 
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Performance monitoring for state machines
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    transition_count: u64,
    cache_hits: u64,
    cache_misses: u64,
    average_transition_time: f64,
    total_transition_time: f64,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            transition_count: 0,
            cache_hits: 0,
            cache_misses: 0,
            average_transition_time: 0.0,
            total_transition_time: 0.0,
        }
    }

    /// Record a transition
    pub fn record_transition(&mut self, duration: std::time::Duration, cache_hit: bool) {
        self.transition_count += 1;
        if cache_hit {
            self.cache_hits += 1;
        } else {
            self.cache_misses += 1;
        }
        
        let duration_ms = duration.as_secs_f64() * 1000.0;
        self.total_transition_time += duration_ms;
        self.average_transition_time = self.total_transition_time / self.transition_count as f64;
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> PerformanceStats {
        let cache_hit_rate = if self.transition_count > 0 {
            self.cache_hits as f64 / self.transition_count as f64
        } else {
            0.0
        };

        PerformanceStats {
            transition_count: self.transition_count,
            cache_hit_rate,
            average_transition_time: self.average_transition_time,
            total_transition_time: self.total_transition_time,
        }
    }

    /// Reset the monitor
    pub fn reset(&mut self) {
        self.transition_count = 0;
        self.cache_hits = 0;
        self.cache_misses = 0;
        self.average_transition_time = 0.0;
        self.total_transition_time = 0.0;
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub transition_count: u64,
    pub cache_hit_rate: f64,
    pub average_transition_time: f64,
    pub total_transition_time: f64,
}

/// Extension traits for adding optimization features to machines
pub trait MachineOptimization<C, E>
where
    C: Clone + Hash + Send + Sync + Default + std::fmt::Debug + 'static,
    E: Clone + Hash + Send + Sync + Default + std::fmt::Debug + PartialEq + 'static,
{
    /// Add transition caching to the machine
    fn with_transition_cache(self, max_size: usize) -> OptimizedMachine<C, E>;
    
    /// Add batch update support to the machine
    fn with_batch_updates(self, batch_size: usize) -> OptimizedMachine<C, E>;
    
    /// Add lazy evaluation to the machine
    fn with_lazy_evaluation(self) -> OptimizedMachine<C, E>;
    
    /// Add performance monitoring to the machine
    fn with_performance_monitoring(self) -> OptimizedMachine<C, E>;
}

impl<C, E> MachineOptimization<C, E> for Machine<C, E>
where
    C: Clone + Hash + Send + Sync + Default + std::fmt::Debug + 'static,
    E: Clone + Hash + Send + Sync + Default + std::fmt::Debug + PartialEq + 'static,
{
    fn with_transition_cache(self, max_size: usize) -> OptimizedMachine<C, E> {
        OptimizedMachine {
            machine: self,
            cache: Some(OptimizationCache::new(max_size)),
            batch_manager: None,
            lazy_eval: None,
            performance_monitor: None,
        }
    }
    
    fn with_batch_updates(self, batch_size: usize) -> OptimizedMachine<C, E> {
        OptimizedMachine {
            machine: self,
            cache: None,
            batch_manager: Some(BatchUpdateManager::new(batch_size)),
            lazy_eval: None,
            performance_monitor: None,
        }
    }
    
    fn with_lazy_evaluation(self) -> OptimizedMachine<C, E> {
        OptimizedMachine {
            machine: self,
            cache: None,
            batch_manager: None,
            lazy_eval: Some(LazyEvaluationManager::new()),
            performance_monitor: None,
        }
    }
    
    fn with_performance_monitoring(self) -> OptimizedMachine<C, E> {
        OptimizedMachine {
            machine: self,
            cache: None,
            batch_manager: None,
            lazy_eval: None,
            performance_monitor: Some(PerformanceMonitor::new()),
        }
    }
}

/// An optimized machine with various performance enhancements
pub struct OptimizedMachine<C, E> 
where
    C: Clone + Hash + Send + Sync + Default + std::fmt::Debug + 'static,
    E: Clone + Hash + Send + Sync + Default + std::fmt::Debug + PartialEq + 'static,
{
    machine: Machine<C, E>,
    cache: Option<OptimizationCache<C, E>>,
    batch_manager: Option<BatchUpdateManager<C, E>>,
    lazy_eval: Option<LazyEvaluationManager<String>>,
    performance_monitor: Option<PerformanceMonitor>,
}

impl<C, E> OptimizedMachine<C, E>
where
    C: Clone + Hash + Send + Sync + Default + std::fmt::Debug + 'static,
    E: Clone + Hash + Send + Sync + Default + std::fmt::Debug + PartialEq + 'static,
{
    /// Get performance statistics
    pub fn get_performance_stats(&self) -> Option<PerformanceStats> {
        self.performance_monitor.as_ref().map(|m| m.get_stats())
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> Option<CacheStats> {
        self.cache.as_ref().map(|c| c.stats())
    }
    
    /// Get the underlying machine
    pub fn machine(&self) -> &Machine<C, E> {
        &self.machine
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Hash, Default)]
    struct TestContext {
        counter: u32,
    }

    #[derive(Clone, Debug, PartialEq, Hash, Default)]
    enum TestEvent {
        #[default]
        Increment,
        Decrement,
    }

    #[test]
    fn test_transition_cache() {
        let cache = OptimizationCache::<TestContext, TestEvent>::new(100);
        
        let context = TestContext::default();
        let event = TestEvent::Increment;
        let state = "idle";
        
        let result = TransitionResult {
            success: true,
            transition_time: 1.0,
        };
        
        cache.set(state, &event, &context, result.clone());
        let cached = cache.get(state, &event, &context);
        
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().success, result.success);
        
        let stats = cache.stats();
        assert_eq!(stats.size, 1);
    }

    #[test]
    fn test_performance_monitoring() {
        let mut monitor = PerformanceMonitor::new();
        
        monitor.record_transition(std::time::Duration::from_millis(1), false);
        monitor.record_transition(std::time::Duration::from_millis(2), true);
        
        let stats = monitor.get_stats();
        assert_eq!(stats.transition_count, 2);
        assert_eq!(stats.cache_hit_rate, 0.5);
        assert!(stats.average_transition_time > 0.0);
    }

    #[test]
    fn test_batch_update_manager() {
        let mut manager = BatchUpdateManager::<TestContext, TestEvent>::new(3);
        
        manager.add_update(TestEvent::Increment, TestContext::default());
        manager.add_update(TestEvent::Decrement, TestContext::default());
        
        assert_eq!(manager.current_size(), 2);
        assert!(!manager.is_ready());
        
        manager.add_update(TestEvent::Increment, TestContext::default());
        assert!(manager.is_ready());
        
        let updates = manager.process_batch();
        assert_eq!(updates.len(), 3);
        assert_eq!(manager.current_size(), 0);
    }

    #[test]
    fn test_lazy_evaluation_manager() {
        let mut manager = LazyEvaluationManager::<String>::new();
        
        let result1 = manager.get_or_compute("key1", || "computed1".to_string());
        let result2 = manager.get_or_compute("key1", || "computed2".to_string());
        
        assert_eq!(result1, "computed1");
        assert_eq!(result2, "computed1"); // Should be cached
        
        assert!(manager.is_computed("key1"));
        assert!(!manager.is_computed("key2"));
    }
}