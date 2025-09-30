//! Lazy selector that only computes when needed

use crate::store::Store;

/// Lazy selector that only computes when needed
pub struct LazySelector<T: Store, O> {
    /// The selector function
    selector: Box<dyn Fn(&T::State) -> O + Send + Sync>,
    /// Cached result
    cached_result: std::sync::Mutex<Option<O>>,
    /// Whether the selector has been evaluated
    evaluated: std::sync::Mutex<bool>,
    /// Evaluation trigger condition
    trigger: Option<Box<dyn Fn(&T::State) -> bool + Send + Sync>>,
}

impl<T: Store, O: Clone + PartialEq + 'static> LazySelector<T, O> {
    /// Create a new lazy selector
    pub fn new<F>(selector: F) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
    {
        Self {
            selector: Box::new(selector),
            cached_result: std::sync::Mutex::new(None),
            evaluated: std::sync::Mutex::new(false),
            trigger: None,
        }
    }

    /// Create a lazy selector with a trigger condition
    pub fn with_trigger<F, G>(selector: F, trigger: G) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        G: Fn(&T::State) -> bool + Send + Sync + 'static,
    {
        Self {
            selector: Box::new(selector),
            cached_result: std::sync::Mutex::new(None),
            evaluated: std::sync::Mutex::new(false),
            trigger: Some(Box::new(trigger)),
        }
    }

    /// Get the selected value, computing lazily
    pub fn select(&self, state: &T::State) -> Option<O> {
        // Check trigger condition first
        if let Some(ref trigger) = self.trigger {
            if !trigger(state) {
                return None; // Don't evaluate yet
            }
        }

        let mut evaluated = self.evaluated.lock().unwrap();
        if !*evaluated {
            let result = (self.selector)(state);
            *self.cached_result.lock().unwrap() = Some(result);
            *evaluated = true;
        }

        self.cached_result.lock().unwrap().clone()
    }

    /// Force evaluation of the selector
    pub fn force_evaluate(&self, state: &T::State) -> O {
        let result = (self.selector)(state);
        *self.cached_result.lock().unwrap() = Some(result.clone());
        *self.evaluated.lock().unwrap() = true;
        result
    }

    /// Check if the selector has been evaluated
    pub fn is_evaluated(&self) -> bool {
        *self.evaluated.lock().unwrap()
    }

    /// Clear the cached result
    pub fn clear_cache(&self) {
        *self.cached_result.lock().unwrap() = None;
        *self.evaluated.lock().unwrap() = false;
    }

    /// Get cached result without evaluation
    pub fn get_cached(&self) -> Option<O> {
        self.cached_result.lock().unwrap().clone()
    }

    /// Reset the selector to unevaluated state
    pub fn reset(&self) {
        self.clear_cache();
    }

    /// Set a new trigger condition
    pub fn set_trigger<F>(&mut self, trigger: F)
    where
        F: Fn(&T::State) -> bool + Send + Sync + 'static,
    {
        self.trigger = Some(Box::new(trigger));
        self.clear_cache(); // Reset when trigger changes
    }

    /// Remove trigger condition
    pub fn remove_trigger(&mut self) {
        self.trigger = None;
        self.clear_cache();
    }

    /// Check if selector has a trigger condition
    pub fn has_trigger(&self) -> bool {
        self.trigger.is_some()
    }

    /// Get selector statistics
    pub fn stats(&self) -> LazySelectorStats {
        LazySelectorStats {
            evaluated: self.is_evaluated(),
            has_trigger: self.has_trigger(),
            has_cached_result: self.cached_result.lock().unwrap().is_some(),
        }
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> std::fmt::Debug for LazySelector<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazySelector")
            .field("evaluated", &self.is_evaluated())
            .field("has_trigger", &self.has_trigger())
            .finish()
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> std::fmt::Display for LazySelector<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats = self.stats();
        write!(
            f,
            "LazySelector(evaluated: {}, trigger: {}, cached: {})",
            stats.evaluated, stats.has_trigger, stats.has_cached_result
        )
    }
}

/// Statistics for lazy selector
#[derive(Debug, Clone)]
pub struct LazySelectorStats {
    /// Whether the selector has been evaluated
    pub evaluated: bool,
    /// Whether the selector has a trigger condition
    pub has_trigger: bool,
    /// Whether there's a cached result
    pub has_cached_result: bool,
}

impl std::fmt::Display for LazySelectorStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "evaluated: {}, trigger: {}, cached: {}",
            self.evaluated, self.has_trigger, self.has_cached_result
        )
    }
}

/// Lazy evaluation manager for coordinating multiple lazy selectors
pub struct LazyEvaluationManager<T: Store> {
    /// Registered lazy selectors
    selectors: std::sync::Mutex<std::collections::HashMap<String, Box<dyn LazySelectorTrait<T>>>>,
    /// Global evaluation trigger
    global_trigger: Option<Box<dyn Fn(&T::State) -> bool + Send + Sync>>,
}

impl<T: Store> LazyEvaluationManager<T> {
    /// Create a new lazy evaluation manager
    pub fn new() -> Self {
        Self {
            selectors: std::sync::Mutex::new(std::collections::HashMap::new()),
            global_trigger: None,
        }
    }

    /// Register a lazy selector
    pub fn register_selector(&self, name: String, selector: Box<dyn LazySelectorTrait<T>>) {
        self.selectors.lock().unwrap().insert(name, selector);
    }

    /// Unregister a lazy selector
    pub fn unregister_selector(&self, name: &str) -> Option<Box<dyn LazySelectorTrait<T>>> {
        self.selectors.lock().unwrap().remove(name)
    }

    /// Evaluate all registered selectors
    pub fn evaluate_all(&self, state: &T::State) {
        let selectors = self.selectors.lock().unwrap();

        for selector in selectors.values() {
            selector.evaluate_if_triggered(state);
        }
    }

    /// Evaluate selector by name
    pub fn evaluate_selector(&self, name: &str, state: &T::State) {
        if let Some(selector) = self.selectors.lock().unwrap().get(name) {
            selector.force_evaluate(state);
        }
    }

    /// Check if global trigger condition is met
    pub fn should_evaluate(&self, state: &T::State) -> bool {
        if let Some(ref trigger) = self.global_trigger {
            trigger(state)
        } else {
            true // No global trigger means always evaluate when requested
        }
    }

    /// Set global evaluation trigger
    pub fn set_global_trigger<F>(&mut self, trigger: F)
    where
        F: Fn(&T::State) -> bool + Send + Sync + 'static,
    {
        self.global_trigger = Some(Box::new(trigger));
    }

    /// Clear all cached results
    pub fn clear_all_caches(&self) {
        let selectors = self.selectors.lock().unwrap();

        for selector in selectors.values() {
            selector.clear_cache();
        }
    }

    /// Get registered selector names
    pub fn selector_names(&self) -> Vec<String> {
        self.selectors.lock().unwrap().keys().cloned().collect()
    }

    /// Get evaluation statistics
    pub fn stats(&self) -> LazyEvaluationStats {
        let selectors = self.selectors.lock().unwrap();
        let total_selectors = selectors.len();

        let mut evaluated_count = 0;
        let mut with_triggers = 0;
        let mut with_cache = 0;

        for selector in selectors.values() {
            if selector.is_evaluated() {
                evaluated_count += 1;
            }
            if selector.has_trigger() {
                with_triggers += 1;
            }
            if selector.has_cached_result() {
                with_cache += 1;
            }
        }

        LazyEvaluationStats {
            total_selectors,
            evaluated_selectors: evaluated_count,
            selectors_with_triggers: with_triggers,
            selectors_with_cache: with_cache,
        }
    }
}

impl<T: Store> Default for LazyEvaluationManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for lazy selectors
pub trait LazySelectorTrait<T: Store> {
    /// Evaluate if trigger condition is met
    fn evaluate_if_triggered(&self, state: &T::State);

    /// Force evaluation
    fn force_evaluate(&self, state: &T::State);

    /// Check if evaluated
    fn is_evaluated(&self) -> bool;

    /// Clear cache
    fn clear_cache(&self);

    /// Check if has trigger
    fn has_trigger(&self) -> bool;

    /// Check if has cached result
    fn has_cached_result(&self) -> bool;
}

/// Statistics for lazy evaluation manager
#[derive(Debug, Clone)]
pub struct LazyEvaluationStats {
    /// Total number of registered selectors
    pub total_selectors: usize,
    /// Number of evaluated selectors
    pub evaluated_selectors: usize,
    /// Number of selectors with triggers
    pub selectors_with_triggers: usize,
    /// Number of selectors with cached results
    pub selectors_with_cache: usize,
}

impl std::fmt::Display for LazyEvaluationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LazyEvaluation(total: {}, evaluated: {}, triggers: {}, cached: {})",
            self.total_selectors,
            self.evaluated_selectors,
            self.selectors_with_triggers,
            self.selectors_with_cache
        )
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> LazySelectorTrait<T> for LazySelector<T, O> {
    fn evaluate_if_triggered(&self, state: &T::State) {
        self.select(state); // This will evaluate if trigger condition is met
    }

    fn force_evaluate(&self, state: &T::State) {
        self.force_evaluate(state);
    }

    fn is_evaluated(&self) -> bool {
        self.is_evaluated()
    }

    fn clear_cache(&self) {
        self.clear_cache();
    }

    fn has_trigger(&self) -> bool {
        self.has_trigger()
    }

    fn has_cached_result(&self) -> bool {
        self.cached_result.lock().unwrap().is_some()
    }
}
