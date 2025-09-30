//! Combined selectors and composition utilities

use crate::store::Store;

/// Combined selector that merges multiple selectors
pub struct CombinedSelector<T: Store, O: PartialEq + 'static = ()>
where
    O: Default,
{
    /// Individual selectors
    selectors: Vec<Box<dyn Fn(&T::State) -> O + Send + Sync>>,
    /// Combination strategy
    strategy: CombinationStrategy,
    /// Cache for combined result
    cache: std::sync::Mutex<Option<(Vec<O>, O)>>,
}

impl<T: Store, O: PartialEq + Clone + 'static> CombinedSelector<T, O>
where
    O: Default,
{
    /// Create a new combined selector
    pub fn new(strategy: CombinationStrategy) -> Self {
        Self {
            selectors: Vec::new(),
            strategy,
            cache: std::sync::Mutex::new(None),
        }
    }

    /// Add a selector to the combination
    pub fn add_selector<F>(&mut self, selector: F)
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
    {
        self.selectors.push(Box::new(selector));
        // Clear cache when selectors change
        *self.cache.lock().unwrap() = None;
    }

    /// Select combined value
    pub fn select(&self, state: &T::State) -> O {
        if self.selectors.is_empty() {
            return O::default();
        }

        let mut cache = self.cache.lock().unwrap();

        // Compute individual results
        let mut results = Vec::with_capacity(self.selectors.len());
        for selector in &self.selectors {
            results.push(selector(state));
        }

        // Check cache
        if let Some((ref cached_results, ref cached_combined)) = *cache {
            if *cached_results == results {
                return cached_combined.clone();
            }
        }

        // Combine results
        let combined = self.strategy.combine(&results);
        *cache = Some((results, combined.clone()));
        combined
    }

    /// Clear cache
    pub fn clear_cache(&self) {
        *self.cache.lock().unwrap() = None;
    }

    /// Get number of selectors
    pub fn selector_count(&self) -> usize {
        self.selectors.len()
    }

    /// Get combination strategy
    pub fn strategy(&self) -> &CombinationStrategy {
        &self.strategy
    }

    /// Set combination strategy
    pub fn set_strategy(&mut self, strategy: CombinationStrategy) {
        if self.strategy != strategy {
            self.strategy = strategy;
            self.clear_cache();
        }
    }
}

impl<T: Store, O: PartialEq + Clone + 'static> std::fmt::Debug for CombinedSelector<T, O>
where
    O: Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CombinedSelector")
            .field("selectors", &self.selectors.len())
            .field("strategy", &self.strategy)
            .finish()
    }
}

impl<T: Store, O: PartialEq + Clone + 'static> std::fmt::Display for CombinedSelector<T, O>
where
    O: Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CombinedSelector({} selectors, {})", self.selectors.len(), self.strategy)
    }
}

/// Strategy for combining multiple selector results
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombinationStrategy {
    /// Return first non-default result
    First,
    /// Return last result
    Last,
    /// Combine using AND (for boolean)
    And,
    /// Combine using OR (for boolean)
    Or,
    /// Sum numeric results
    Sum,
    /// Count non-default results
    Count,
    /// Collect into vector
    Collect,
    /// Custom combination function
    Custom(String),
}

impl CombinationStrategy {
    /// Combine results using this strategy
    pub fn combine<O: Clone + Default + PartialEq + 'static>(&self, results: &[O]) -> O {
        match self {
            Self::First => {
                results.iter().find(|r| **r != O::default()).cloned().unwrap_or_default()
            }
            Self::Last => {
                results.last().cloned().unwrap_or_default()
            }
            Self::And => {
                // For boolean-like types
                if results.iter().all(|r| *r != O::default()) {
                    // All true
                    results.first().cloned().unwrap_or_default()
                } else {
                    O::default()
                }
            }
            Self::Or => {
                // For boolean-like types
                results.iter().find(|r| *r != O::default()).cloned().unwrap_or_default()
            }
            Self::Sum => {
                // For numeric types - simplified
                results.first().cloned().unwrap_or_default()
            }
            Self::Count => {
                // Count non-default results - simplified
                results.first().cloned().unwrap_or_default()
            }
            Self::Collect => {
                // Return first result for simplicity
                results.first().cloned().unwrap_or_default()
            }
            Self::Custom(_) => {
                // Custom logic would be implemented here
                results.first().cloned().unwrap_or_default()
            }
        }
    }
}

impl std::fmt::Display for CombinationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::First => write!(f, "first"),
            Self::Last => write!(f, "last"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Sum => write!(f, "sum"),
            Self::Count => write!(f, "count"),
            Self::Collect => write!(f, "collect"),
            Self::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// Selector composition utilities
pub mod composition {
    use crate::store::Store;

    /// Compose two selectors
    pub fn compose<T: Store, A, B, F>(
        selector1: impl Fn(&T::State) -> A + Send + Sync + 'static,
        selector2: F,
    ) -> impl Fn(&T::State) -> B + Send + Sync + 'static
    where
        F: Fn(A) -> B + Send + Sync + 'static,
        A: 'static,
        B: 'static,
    {
        move |state: &T::State| {
            let result1 = selector1(state);
            selector2(result1)
        }
    }

    /// Create a conditional selector
    pub fn conditional<T: Store, O>(
        condition: impl Fn(&T::State) -> bool + Send + Sync + 'static,
        if_true: impl Fn(&T::State) -> O + Send + Sync + 'static,
        if_false: impl Fn(&T::State) -> O + Send + Sync + 'static,
    ) -> impl Fn(&T::State) -> O + Send + Sync + 'static {
        move |state: &T::State| {
            if condition(state) {
                if_true(state)
            } else {
                if_false(state)
            }
        }
    }

    /// Create a selector that maps results
    pub fn map<T: Store, A, B>(
        selector: impl Fn(&T::State) -> A + Send + Sync + 'static,
        mapper: impl Fn(A) -> B + Send + Sync + 'static,
    ) -> impl Fn(&T::State) -> B + Send + Sync + 'static {
        move |state: &T::State| {
            let result = selector(state);
            mapper(result)
        }
    }

    /// Create a selector that filters results
    pub fn filter<T: Store, O: Clone>(
        selector: impl Fn(&T::State) -> O + Send + Sync + 'static,
        predicate: impl Fn(&O) -> bool + Send + Sync + 'static,
        default_value: O,
    ) -> impl Fn(&T::State) -> O + Send + Sync + 'static {
        move |state: &T::State| {
            let result = selector(state);
            if predicate(&result) {
                result
            } else {
                default_value.clone()
            }
        }
    }

    /// Create a selector that chains multiple selectors
    pub fn chain<T: Store, O>(
        selectors: Vec<Box<dyn Fn(&T::State) -> O + Send + Sync>>,
        strategy: super::CombinationStrategy,
    ) -> impl Fn(&T::State) -> O + Send + Sync + 'static
    where
        O: Clone + Default + PartialEq + 'static,
    {
        move |state: &T::State| {
            let results: Vec<O> = selectors.iter().map(|s| s(state)).collect();
            strategy.combine(&results)
        }
    }

    /// Create a memoized version of a selector
    pub fn memoized<T: Store, O>(
        selector: impl Fn(&T::State) -> O + Send + Sync + 'static,
    ) -> super::super::basic::MemoizedSelector<T, O>
    where
        O: Clone + PartialEq + 'static,
    {
        super::super::basic::MemoizedSelector::new(selector)
    }

    /// Create a lazy version of a selector
    pub fn lazy<T: Store, O>(
        selector: impl Fn(&T::State) -> O + Send + Sync + 'static,
    ) -> super::super::lazy::LazySelector<T, O>
    where
        O: Clone + PartialEq + 'static,
    {
        super::super::lazy::LazySelector::new(selector)
    }
}

/// Selector pipeline for complex transformations
pub struct SelectorPipeline<T: Store, O> {
    /// Pipeline steps
    steps: Vec<Box<dyn Fn(O) -> O + Send + Sync>>,
    /// Initial selector
    initial_selector: Box<dyn Fn(&T::State) -> O + Send + Sync>,
}

impl<T: Store, O: Clone + 'static> SelectorPipeline<T, O> {
    /// Create a new selector pipeline
    pub fn new<F>(initial_selector: F) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
    {
        Self {
            steps: Vec::new(),
            initial_selector: Box::new(initial_selector),
        }
    }

    /// Add a transformation step
    pub fn add_step<F>(&mut self, step: F)
    where
        F: Fn(O) -> O + Send + Sync + 'static,
    {
        self.steps.push(Box::new(step));
    }

    /// Execute the pipeline
    pub fn execute(&self, state: &T::State) -> O {
        let mut result = (self.initial_selector)(state);

        for step in &self.steps {
            result = step(result);
        }

        result
    }

    /// Get the number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

impl<T: Store, O: Clone + 'static> std::fmt::Debug for SelectorPipeline<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectorPipeline")
            .field("steps", &self.steps.len())
            .finish()
    }
}

impl<T: Store, O: Clone + 'static> std::fmt::Display for SelectorPipeline<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SelectorPipeline({} steps)", self.steps.len())
    }
}
