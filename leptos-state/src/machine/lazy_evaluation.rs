//! Lazy evaluation wrapper for expensive operations

use super::*;
use std::sync::{Arc, Mutex};

/// Lazy evaluation wrapper for expensive operations
#[derive(Debug)]
pub struct LazyEvaluator<T, F> {
    /// The computation function
    computation: F,
    /// Cached result
    result: Arc<Mutex<Option<T>>>,
    /// Whether the computation has been performed
    computed: Arc<Mutex<bool>>,
}

impl<T, F> LazyEvaluator<T, F>
where
    T: Clone,
    F: Fn() -> T,
{
    /// Create a new lazy evaluator
    pub fn new(computation: F) -> Self {
        Self {
            computation,
            result: Arc::new(Mutex::new(None)),
            computed: Arc::new(Mutex::new(false)),
        }
    }

    /// Get the result, computing it if necessary
    pub fn get(&self) -> T {
        let mut computed = self.computed.lock().unwrap();
        if !*computed {
            let result = (self.computation)();
            *self.result.lock().unwrap() = Some(result);
            *computed = true;
        }

        self.result.lock().unwrap().as_ref().unwrap().clone()
    }

    /// Check if the computation has been performed
    pub fn is_computed(&self) -> bool {
        *self.computed.lock().unwrap()
    }

    /// Force recomputation
    pub fn recompute(&self) -> T {
        let result = (self.computation)();
        *self.result.lock().unwrap() = Some(result.clone());
        *self.computed.lock().unwrap() = true;
        result
    }

    /// Clear the cached result
    pub fn clear(&self) {
        *self.result.lock().unwrap() = None;
        *self.computed.lock().unwrap() = false;
    }

    /// Get the result without computing if not available
    pub fn get_cached(&self) -> Option<T> {
        self.result.lock().unwrap().clone()
    }
}

impl<T, F> Clone for LazyEvaluator<T, F>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            computation: self.computation.clone(),
            result: Arc::clone(&self.result),
            computed: Arc::clone(&self.computed),
        }
    }
}

/// Lazy value that can be computed on demand
pub struct Lazy<T> {
    /// The lazy evaluator
    evaluator: LazyEvaluator<T, Box<dyn Fn() -> T + Send + Sync>>,
}

impl<T> Lazy<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Create a new lazy value
    pub fn new<F>(computation: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            evaluator: LazyEvaluator::new(Box::new(computation)),
        }
    }

    /// Get the value
    pub fn get(&self) -> T {
        self.evaluator.get()
    }

    /// Check if computed
    pub fn is_computed(&self) -> bool {
        self.evaluator.is_computed()
    }

    /// Force recomputation
    pub fn recompute(&self) -> T {
        self.evaluator.recompute()
    }

    /// Clear cache
    pub fn clear(&self) {
        self.evaluator.clear()
    }
}

impl<T> Clone for Lazy<T> {
    fn clone(&self) -> Self {
        Self {
            evaluator: self.evaluator.clone(),
        }
    }
}

/// Lazy computation result with metadata
#[derive(Debug, Clone)]
pub struct LazyResult<T> {
    /// The computed value
    pub value: T,
    /// Time when computation started
    pub start_time: std::time::Instant,
    /// Time when computation finished
    pub end_time: std::time::Instant,
    /// Whether the result came from cache
    pub from_cache: bool,
    /// Computation duration
    pub duration: std::time::Duration,
}

impl<T> LazyResult<T> {
    /// Create a new lazy result
    pub fn new(value: T, duration: std::time::Duration, from_cache: bool) -> Self {
        let now = std::time::Instant::now();
        Self {
            value,
            start_time: now - duration,
            end_time: now,
            from_cache,
            duration,
        }
    }

    /// Get the computation time
    pub fn computation_time(&self) -> std::time::Duration {
        self.end_time - self.start_time
    }
}

/// Lazy computation with result metadata
pub struct LazyWithMetadata<T> {
    /// The lazy evaluator
    evaluator: LazyEvaluator<LazyResult<T>, Box<dyn Fn() -> LazyResult<T> + Send + Sync>>,
}

impl<T> LazyWithMetadata<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Create a new lazy computation with metadata
    pub fn new<F>(computation: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        let wrapped_computation = move || {
            let start = std::time::Instant::now();
            let value = computation();
            let duration = start.elapsed();
            LazyResult::new(value, duration, false)
        };

        Self {
            evaluator: LazyEvaluator::new(Box::new(wrapped_computation)),
        }
    }

    /// Get the result with metadata
    pub fn get(&self) -> LazyResult<T> {
        self.evaluator.get()
    }

    /// Get just the value
    pub fn get_value(&self) -> T {
        self.evaluator.get().value
    }

    /// Check if computed
    pub fn is_computed(&self) -> bool {
        self.evaluator.is_computed()
    }

    /// Get cached result if available
    pub fn get_cached(&self) -> Option<LazyResult<T>> {
        self.evaluator.get_cached()
    }

    /// Force recomputation
    pub fn recompute(&self) -> LazyResult<T> {
        self.evaluator.recompute()
    }
}

impl<T> Clone for LazyWithMetadata<T> {
    fn clone(&self) -> Self {
        Self {
            evaluator: self.evaluator.clone(),
        }
    }
}

/// Performance-aware lazy computation
pub struct PerformanceLazy<T> {
    /// The computation
    lazy: LazyWithMetadata<T>,
    /// Performance threshold - if computation takes longer than this, cache it
    threshold: std::time::Duration,
}

impl<T> PerformanceLazy<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Create a new performance-aware lazy computation
    pub fn new<F>(computation: F, threshold: std::time::Duration) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            lazy: LazyWithMetadata::new(computation),
            threshold,
        }
    }

    /// Get the value, considering performance
    pub fn get(&self) -> T {
        if let Some(cached) = self.lazy.get_cached() {
            // If we have a cached result and it's fast, return it
            if cached.duration < self.threshold {
                return cached.value;
            }
        }

        // Otherwise compute and cache
        self.lazy.get_value()
    }

    /// Get result with metadata
    pub fn get_with_metadata(&self) -> LazyResult<T> {
        self.lazy.get()
    }

    /// Force recomputation if it would be slow
    pub fn get_performance_aware(&self) -> T {
        if let Some(cached) = self.lazy.get_cached() {
            if cached.duration >= self.threshold {
                // It's slow, so recompute (might have improved)
                return self.lazy.recompute().value;
            } else {
                return cached.value;
            }
        }

        // No cache, compute
        self.lazy.get_value()
    }
}

impl<T> Clone for PerformanceLazy<T> {
    fn clone(&self) -> Self {
        Self {
            lazy: self.lazy.clone(),
            threshold: self.threshold,
        }
    }
}
