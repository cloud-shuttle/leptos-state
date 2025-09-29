//! Memoized selectors and advanced selection patterns

use super::*;

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
}

/// Dependency-tracked memoized selector
pub struct DependencyTrackedSelector<T: Store, O> {
    /// The selector function
    pub selector: Box<dyn Fn(&T::State) -> O + Send + Sync>,
    /// Dependencies that this selector reads
    pub dependencies: Vec<String>,
    /// Cache
    pub cache: std::sync::Mutex<Option<(u64, O)>>,
    /// Dependency hash calculator
    pub dep_hasher: Box<dyn Fn(&T::State) -> u64 + Send + Sync>,
}

impl<T: Store, O: Clone + PartialEq + 'static> DependencyTrackedSelector<T, O> {
    /// Create a new dependency-tracked selector
    pub fn new<F, H>(selector: F, dependencies: Vec<String>, dep_hasher: H) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        H: Fn(&T::State) -> u64 + Send + Sync + 'static,
    {
        Self {
            selector: Box::new(selector),
            dependencies,
            cache: std::sync::Mutex::new(None),
            dep_hasher: Box::new(dep_hasher),
        }
    }

    /// Select with dependency tracking
    pub fn select(&self, state: &T::State) -> O {
        let dep_hash = (self.dep_hasher)(state);
        let mut cache = self.cache.lock().unwrap();

        if let Some((cached_hash, ref cached_value)) = *cache {
            if cached_hash == dep_hash {
                return cached_value.clone();
            }
        }

        let value = (self.selector)(state);
        *cache = Some((dep_hash, value.clone()));
        value
    }

    /// Invalidate cache when dependencies change
    pub fn invalidate(&self) {
        *self.cache.lock().unwrap() = None;
    }
}

/// Combined selector that merges multiple selectors
pub struct CombinedSelector<T: Store, O = Box<dyn std::any::Any>> {
    /// The selectors to combine
    pub selectors: Vec<Box<dyn StoreSlice<T, Output = O>>>,
    /// The combination function
    pub combiner: Box<dyn Fn(Vec<Box<dyn std::any::Any>>) -> O + Send + Sync>,
}

impl<T: Store, O> CombinedSelector<T, O> {
    /// Create a new combined selector
    pub fn new<F>(selectors: Vec<Box<dyn StoreSlice<T, Output = O>>>, combiner: F) -> Self
    where
        F: Fn(Vec<Box<dyn std::any::Any>>) -> O + Send + Sync + 'static,
    {
        Self {
            selectors,
            combiner: Box::new(combiner),
        }
    }

    /// Select combined values
    pub fn select(&self, state: &T::State) -> O {
        let values: Vec<Box<dyn std::any::Any>> = self.selectors.iter()
            .map(|selector| {
                // In a real implementation, this would need to handle different types
                // For now, this is a simplified version
                Box::new(()) as Box<dyn std::any::Any>
            })
            .collect();

        (self.combiner)(values)
    }
}

/// Selector composition utilities
pub mod composition {
    use super::*;

    /// Compose two selectors
    pub fn compose<A, B, T, F>(
        first: impl StoreSlice<T, Output = A>,
        second: F,
    ) -> impl StoreSlice<T, Output = B>
    where
        T: Store,
        A: Clone + 'static,
        B: Clone + PartialEq + 'static,
        F: Fn(A) -> B + Clone + 'static,
    {
        struct ComposedSelector<S, F> {
            first: S,
            second: F,
        }

        impl<T, A, B, S, F> StoreSlice<T> for ComposedSelector<S, F>
        where
            T: Store,
            A: Clone + 'static,
            B: Clone + PartialEq + 'static,
            S: StoreSlice<T, Output = A>,
            F: Fn(A) -> B + Clone,
        {
            type Output = B;

            fn select(&self, state: &T::State) -> Self::Output {
                let intermediate = self.first.select(state);
                (self.second)(intermediate)
            }
        }

        ComposedSelector { first, second }
    }

    /// Chain multiple selectors
    pub fn chain<T, S, O>(selectors: Vec<S>) -> impl StoreSlice<T, Output = O>
    where
        T: Store,
        S: StoreSlice<T, Output = O> + 'static,
        O: Clone + 'static,
    {
        struct ChainedSelector<T, O> {
            selectors: Vec<Box<dyn StoreSlice<T, Output = O>>>,
        }

        impl<T: Store, O> StoreSlice<T> for ChainedSelector<T, O>
        where
            O: Clone + 'static,
        {
            type Output = Vec<O>;

            fn select(&self, state: &T::State) -> Self::Output {
                self.selectors.iter()
                    .map(|selector| selector.select(state))
                    .collect()
            }
        }

        ChainedSelector {
            selectors: selectors.into_iter()
                .map(|s| Box::new(s) as Box<dyn StoreSlice<T, Output = O>>)
                .collect(),
        }
    }

    /// Create a conditional selector
    pub fn conditional<T, S1, S2, F>(
        condition: F,
        then_selector: S1,
        else_selector: S2,
    ) -> impl StoreSlice<T>
    where
        T: Store,
        S1: StoreSlice<T> + 'static,
        S2: StoreSlice<T> + 'static,
        F: Fn(&T::State) -> bool + Clone + 'static,
        S1::Output: Clone + PartialEq + 'static,
        S2::Output: Clone + PartialEq + 'static,
    {
        struct ConditionalSelector<S1, S2, F> {
            condition: F,
            then_selector: S1,
            else_selector: S2,
        }

        impl<T, S1, S2, F> StoreSlice<T> for ConditionalSelector<S1, S2, F>
        where
            T: Store,
            S1: StoreSlice<T> + 'static,
            S2: StoreSlice<T> + 'static,
            F: Fn(&T::State) -> bool + Clone + 'static,
            S1::Output: Clone + PartialEq + 'static,
            S2::Output: Clone + PartialEq + 'static,
        {
            type Output = Box<dyn std::any::Any>;

            fn select(&self, state: &T::State) -> Self::Output {
                if (self.condition)(state) {
                    Box::new(self.then_selector.select(state))
                } else {
                    Box::new(self.else_selector.select(state))
                }
            }
        }

        ConditionalSelector {
            condition,
            then_selector,
            else_selector,
        }
    }
}

/// Performance-optimized selector with statistics
pub struct PerformanceSelector<T: Store, O> {
    /// The underlying selector
    pub selector: Box<dyn StoreSlice<T, Output = O>>,
    /// Performance statistics
    pub stats: std::sync::Mutex<SelectorStats>,
}

#[derive(Debug, Clone, Default)]
pub struct SelectorStats {
    /// Total number of selections
    pub total_selections: u64,
    /// Total time spent selecting
    pub total_time: std::time::Duration,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
    /// Last selection time
    pub last_selection: Option<std::time::Instant>,
}

impl<T: Store, O: Clone + PartialEq + 'static> PerformanceSelector<T, O> {
    /// Create a new performance selector
    pub fn new(selector: impl StoreSlice<T, Output = O> + 'static) -> Self {
        Self {
            selector: Box::new(selector),
            stats: std::sync::Mutex::new(SelectorStats::default()),
        }
    }

    /// Select with performance tracking
    pub fn select(&self, state: &T::State) -> O {
        let start = std::time::Instant::now();
        let result = self.selector.select(state);
        let duration = start.elapsed();

        let mut stats = self.stats.lock().unwrap();
        stats.total_selections += 1;
        stats.total_time += duration;
        stats.last_selection = Some(std::time::Instant::now());

        result
    }

    /// Get performance statistics
    pub fn stats(&self) -> SelectorStats {
        self.stats.lock().unwrap().clone()
    }

    /// Get average selection time
    pub fn avg_selection_time(&self) -> std::time::Duration {
        let stats = self.stats.lock().unwrap();
        if stats.total_selections == 0 {
            std::time::Duration::from_nanos(0)
        } else {
            stats.total_time / stats.total_selections as u32
        }
    }
}

/// Lazy selector that only computes when needed
pub struct LazySelector<T: Store, O> {
    /// The selector function
    pub selector: Box<dyn Fn(&T::State) -> O + Send + Sync>,
    /// Cached result
    pub cache: std::sync::Mutex<Option<(T::State, O)>>,
    /// Whether to always recompute
    pub always_recompute: bool,
}

impl<T: Store, O: Clone + PartialEq + 'static> LazySelector<T, O> {
    /// Create a new lazy selector
    pub fn new<F>(selector: F) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
    {
        Self {
            selector: Box::new(selector),
            cache: std::sync::Mutex::new(None),
            always_recompute: false,
        }
    }

    /// Select lazily
    pub fn select(&self, state: &T::State) -> O {
        if self.always_recompute {
            return (self.selector)(state);
        }

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

    /// Invalidate cache
    pub fn invalidate(&self) {
        *self.cache.lock().unwrap() = None;
    }

    /// Set always recompute mode
    pub fn always_recompute(mut self, always: bool) -> Self {
        self.always_recompute = always;
        self
    }
}

/// Selector factory for creating commonly used selectors
pub mod factory {
    use super::*;

    /// Create a counter selector
    pub fn counter<T, F, I>(items_selector: F) -> impl StoreSlice<T, Output = usize>
    where
        T: Store,
        F: Fn(&T::State) -> I + Clone + 'static,
        I: IntoIterator + Clone + 'static,
    {
        struct CounterSelector<F>(F);

        impl<T, F, I> StoreSlice<T> for CounterSelector<F>
        where
            T: Store,
            F: Fn(&T::State) -> I + Clone,
            I: IntoIterator + Clone,
        {
            type Output = usize;

            fn select(&self, state: &T::State) -> Self::Output {
                let items = (self.0)(state);
                items.into_iter().count()
            }
        }

        CounterSelector(items_selector)
    }

    /// Create a sum selector for numeric fields
    pub fn sum<T, F, I, N>(items_selector: F, value_selector: impl Fn(&I::Item) -> N + Clone) -> impl StoreSlice<T, Output = N>
    where
        T: Store,
        F: Fn(&T::State) -> I + Clone + 'static,
        I: IntoIterator + Clone + 'static,
        N: std::iter::Sum + Default + Clone + PartialEq + 'static,
    {
        struct SumSelector<F, V> {
            items_selector: F,
            value_selector: V,
        }

        impl<T, F, I, N, V> StoreSlice<T> for SumSelector<F, V>
        where
            T: Store,
            F: Fn(&T::State) -> I + Clone,
            I: IntoIterator + Clone,
            N: std::iter::Sum + Default + Clone + PartialEq + 'static,
            V: Fn(&I::Item) -> N + Clone,
        {
            type Output = N;

            fn select(&self, state: &T::State) -> Self::Output {
                let items = (self.items_selector)(state);
                items.into_iter()
                    .map(|item| (self.value_selector)(&item))
                    .sum()
            }
        }

        SumSelector {
            items_selector,
            value_selector,
        }
    }

    /// Create a max selector
    pub fn max<T, F, I, N>(items_selector: F, value_selector: impl Fn(&I::Item) -> N + Clone) -> impl StoreSlice<T, Output = Option<N>>
    where
        T: Store,
        F: Fn(&T::State) -> I + Clone + 'static,
        I: IntoIterator + Clone + 'static,
        N: PartialOrd + Clone + PartialEq + 'static,
    {
        struct MaxSelector<F, V> {
            items_selector: F,
            value_selector: V,
        }

        impl<T, F, I, N, V> StoreSlice<T> for MaxSelector<F, V>
        where
            T: Store,
            F: Fn(&T::State) -> I + Clone,
            I: IntoIterator + Clone,
            N: PartialOrd + Clone + PartialEq + 'static,
            V: Fn(&I::Item) -> N + Clone,
        {
            type Output = Option<N>;

            fn select(&self, state: &T::State) -> Self::Output {
                let items = (self.items_selector)(state);
                items.into_iter()
                    .map(|item| (self.value_selector)(&item))
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            }
        }

        MaxSelector {
            items_selector,
            value_selector,
        }
    }
}
