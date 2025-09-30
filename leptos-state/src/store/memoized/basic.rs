//! Basic memoized selectors and core functionality

use crate::store::Store;

/// Memoized selector that prevents unnecessary recalculations
pub struct MemoizedSelector<T: Store, O> {
    /// The selector function
    pub selector: Box<dyn Fn(&T::State) -> O + Send + Sync>,
    /// Cache for the last computed value
    pub cache: std::sync::Mutex<Option<(T::State, O)>>,
}

impl<T: Store, O: Clone + PartialEq + 'static> MemoizedSelector<T, O> {
    /// Create a new memoized selector
    pub fn new<F>(selector: F) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
    {
        Self {
            selector: Box::new(selector),
            cache: std::sync::Mutex::new(None),
        }
    }

    /// Get the selected value, using memoization
    pub fn select(&self, state: &T::State) -> O {
        let mut cache = self.cache.lock().unwrap();

        if let Some((ref cached_state, ref cached_value)) = *cache {
            if cached_state == state {
                return cached_value.clone();
            }
        }

        let value = (self.selector)(state);
        *cache = Some((state.clone(), value.clone()));
        value
    }

    /// Clear the memoization cache
    pub fn clear_cache(&self) {
        *self.cache.lock().unwrap() = None;
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (bool, usize) {
        let cache = self.cache.lock().unwrap();
        (cache.is_some(), if cache.is_some() { 1 } else { 0 })
    }

    /// Check if cache is valid for the given state
    pub fn is_cache_valid(&self, state: &T::State) -> bool {
        let cache = self.cache.lock().unwrap();
        if let Some((ref cached_state, _)) = *cache {
            cached_state == state
        } else {
            false
        }
    }

    /// Force refresh the cache
    pub fn refresh(&self, state: &T::State) -> O {
        let value = (self.selector)(state);
        *self.cache.lock().unwrap() = Some((state.clone(), value.clone()));
        value
    }

    /// Get cached value without recomputing (returns None if cache miss)
    pub fn get_cached(&self, state: &T::State) -> Option<O> {
        let cache = self.cache.lock().unwrap();
        if let Some((ref cached_state, ref cached_value)) = *cache {
            if cached_state == state {
                Some(cached_value.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> std::fmt::Debug for MemoizedSelector<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (has_cache, cache_size) = self.cache_stats();
        f.debug_struct("MemoizedSelector")
            .field("has_cache", &has_cache)
            .field("cache_size", &cache_size)
            .finish()
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> std::fmt::Display for MemoizedSelector<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (has_cache, _) = self.cache_stats();
        write!(f, "MemoizedSelector(cached: {})", has_cache)
    }
}

/// Cache invalidation strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheStrategy {
    /// Always check state equality
    Strict,
    /// Use generational caching
    Generational,
    /// No caching
    None,
}

impl Default for CacheStrategy {
    fn default() -> Self {
        Self::Strict
    }
}

/// Selector configuration
#[derive(Debug, Clone)]
pub struct SelectorConfig {
    /// Cache invalidation strategy
    pub cache_strategy: CacheStrategy,
    /// Maximum cache size (0 = unlimited)
    pub max_cache_size: usize,
    /// Enable debug logging
    pub debug_logging: bool,
}

impl Default for SelectorConfig {
    fn default() -> Self {
        Self {
            cache_strategy: CacheStrategy::Strict,
            max_cache_size: 0,
            debug_logging: false,
        }
    }
}

impl SelectorConfig {
    /// Create a new selector configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set cache strategy
    pub fn cache_strategy(mut self, strategy: CacheStrategy) -> Self {
        self.cache_strategy = strategy;
        self
    }

    /// Set maximum cache size
    pub fn max_cache_size(mut self, size: usize) -> Self {
        self.max_cache_size = size;
        self
    }

    /// Enable debug logging
    pub fn debug_logging(mut self, enable: bool) -> Self {
        self.debug_logging = enable;
        self
    }
}

/// Selector builder for fluent construction
pub struct MemoizedSelectorBuilder<T: Store, O> {
    selector: Option<Box<dyn Fn(&T::State) -> O + Send + Sync + 'static>>,
    config: SelectorConfig,
    _phantom: std::marker::PhantomData<(T, O)>,
}

impl<T: Store, O: Clone + PartialEq + 'static> MemoizedSelectorBuilder<T, O> {
    /// Create a new selector builder
    pub fn new() -> Self {
        Self {
            selector: None,
            config: SelectorConfig::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the selector function
    pub fn with_selector<F>(mut self, selector: F) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
    {
        self.selector = Some(Box::new(selector));
        self
    }

    /// Set the cache strategy
    pub fn cache_strategy(mut self, strategy: CacheStrategy) -> Self {
        self.config.cache_strategy = strategy;
        self
    }

    /// Set maximum cache size
    pub fn max_cache_size(mut self, size: usize) -> Self {
        self.config.max_cache_size = size;
        self
    }

    /// Enable debug logging
    pub fn debug_logging(mut self, enable: bool) -> Self {
        self.config.debug_logging = enable;
        self
    }

    /// Build the memoized selector
    pub fn build(self) -> Result<MemoizedSelector<T, O>, String> {
        let selector = self.selector.ok_or_else(|| "Selector function must be provided".to_string())?;
        let mut memoized = MemoizedSelector::new(selector);

        // Apply configuration
        if self.config.debug_logging {
            // In a real implementation, this would enable logging
        }

        Ok(memoized)
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> Default for MemoizedSelectorBuilder<T, O> {
    fn default() -> Self {
        Self::new()
    }
}
