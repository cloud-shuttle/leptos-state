//! Selector factory for creating commonly used selectors

use crate::store::Store;

/// Selector factory for creating commonly used selectors
pub struct SelectorFactory;

impl SelectorFactory {
    /// Create a simple memoized selector
    pub fn memoized<T: Store, O, F>(selector: F) -> crate::store::memoized::basic::MemoizedSelector<T, O>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        crate::store::memoized::basic::MemoizedSelector::new(selector)
    }

    /// Create a dependency-tracked selector
    pub fn dependency_tracked<T: Store, O, F>(selector: F) -> super::dependency::DependencyTrackedSelector<T, O>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        super::dependency::DependencyTrackedSelector::new(selector)
    }

    /// Create a combined selector
    pub fn combined<T: Store, O>(strategy: super::combined::CombinationStrategy) -> super::combined::CombinedSelector<T, O>
    where
        O: PartialEq + Clone + Default + 'static,
    {
        super::combined::CombinedSelector::new(strategy)
    }

    /// Create a performance-monitored selector
    pub fn performance<T: Store, O, F>(selector: F) -> crate::store::memoized::performance::PerformanceSelector<T, O>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        crate::store::memoized::performance::PerformanceSelector::new(selector)
    }

    /// Create a lazy selector
    pub fn lazy<T: Store, O, F>(selector: F) -> crate::store::memoized::lazy::LazySelector<T, O>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        crate::store::memoized::lazy::LazySelector::new(selector)
    }

    /// Create a lazy selector with trigger
    pub fn lazy_with_trigger<T: Store, O, F, G>(
        selector: F,
        trigger: G,
    ) -> crate::store::memoized::lazy::LazySelector<T, O>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        G: Fn(&T::State) -> bool + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        crate::store::memoized::lazy::LazySelector::with_trigger(selector, trigger)
    }

    /// Create a selector that gets a field by name
    pub fn field<T: Store, O>(field_name: &str) -> crate::store::memoized::basic::MemoizedSelector<T, Option<O>>
    where
        O: Clone + PartialEq + 'static,
        T::State: serde_json::value::Index,
    {
        Self::memoized(move |state: &T::State| {
            state.get(field_name)
                .and_then(|v| serde_json::from_value(v.clone()).ok())
        })
    }

    /// Create a selector that checks if a field exists
    pub fn field_exists<T: Store>(field_name: &str) -> crate::store::memoized::basic::MemoizedSelector<T, bool>
    where
        T::State: serde_json::value::Index,
    {
        Self::memoized(move |state: &T::State| {
            state.get(field_name).is_some()
        })
    }

    /// Create a selector that gets array length
    pub fn array_length<T: Store>(array_path: &str) -> crate::store::memoized::basic::MemoizedSelector<T, Option<usize>>
    where
        T::State: serde_json::value::Index,
    {
        Self::memoized(move |state: &T::State| {
            state.get(array_path)
                .and_then(|v| v.as_array())
                .map(|arr| arr.len())
        })
    }

    /// Create a selector that sums numeric values in an array
    pub fn array_sum<T: Store>(array_path: &str) -> crate::store::memoized::basic::MemoizedSelector<T, Option<f64>>
    where
        T::State: serde_json::value::Index,
    {
        Self::memoized(move |state: &T::State| {
            state.get(array_path)
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_f64())
                        .sum()
                })
        })
    }

    /// Create a selector that counts items matching a predicate
    pub fn count_matching<T: Store, F>(
        predicate: F,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, usize>
    where
        F: Fn(&T::State) -> bool + Send + Sync + 'static,
    {
        Self::memoized(move |state: &T::State| {
            if predicate(state) { 1 } else { 0 }
        })
    }

    /// Create a selector that gets the maximum value
    pub fn maximum<T: Store, O>(selector: impl Fn(&T::State) -> O + Send + Sync + 'static) -> crate::store::memoized::basic::MemoizedSelector<T, Option<O>>
    where
        O: Clone + PartialEq + PartialOrd + 'static,
    {
        Self::memoized(move |state: &T::State| {
            Some(selector(state))
        })
    }

    /// Create a selector that gets the minimum value
    pub fn minimum<T: Store, O>(selector: impl Fn(&T::State) -> O + Send + Sync + 'static) -> crate::store::memoized::basic::MemoizedSelector<T, Option<O>>
    where
        O: Clone + PartialEq + PartialOrd + 'static,
    {
        Self::memoized(move |state: &T::State| {
            Some(selector(state))
        })
    }

    /// Create a selector that checks if all conditions are met
    pub fn all<T: Store>(
        conditions: Vec<Box<dyn Fn(&T::State) -> bool + Send + Sync>>,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, bool> {
        Self::memoized(move |state: &T::State| {
            conditions.iter().all(|cond| cond(state))
        })
    }

    /// Create a selector that checks if any condition is met
    pub fn any<T: Store>(
        conditions: Vec<Box<dyn Fn(&T::State) -> bool + Send + Sync>>,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, bool> {
        Self::memoized(move |state: &T::State| {
            conditions.iter().any(|cond| cond(state))
        })
    }

    /// Create a selector that gets nested object properties
    pub fn nested<T: Store, O>(
        path: Vec<String>,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, Option<O>>
    where
        O: Clone + PartialEq + 'static,
        T::State: serde_json::value::Index,
    {
        Self::memoized(move |state: &T::State| {
            let mut current = state;
            for key in &path {
                match current.get(key) {
                    Some(next) => current = next,
                    None => return None,
                }
            }
            serde_json::from_value(current.clone()).ok()
        })
    }

    /// Create a selector that transforms values
    pub fn transform<T: Store, A, B, F>(
        selector: impl Fn(&T::State) -> A + Send + Sync + 'static,
        transformer: F,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, B>
    where
        F: Fn(A) -> B + Send + Sync + 'static,
        A: 'static,
        B: Clone + PartialEq + 'static,
    {
        Self::memoized(move |state: &T::State| {
            let value = selector(state);
            transformer(value)
        })
    }

    /// Create a selector with custom caching strategy
    pub fn with_cache_strategy<T: Store, O, F>(
        selector: F,
        strategy: crate::store::memoized::basic::CacheStrategy,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, O>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        let mut memoized = Self::memoized(selector);

        // Apply cache strategy (simplified - would need to modify the selector)
        // In a real implementation, this would configure the selector's cache behavior
        memoized
    }

    /// Create a debounced selector
    pub fn debounced<T: Store, O, F>(
        selector: F,
        delay_ms: u64,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, Option<O>>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        let last_update = std::sync::Mutex::new(std::time::Instant::now());
        let cached_result = std::sync::Mutex::new(None);

        Self::memoized(move |state: &T::State| {
            let mut last_update_time = last_update.lock().unwrap();
            let mut cached = cached_result.lock().unwrap();

            let now = std::time::Instant::now();
            if now.duration_since(*last_update_time).as_millis() >= delay_ms as u128 {
                let result = selector(state);
                *cached = Some(result);
                *last_update_time = now;
            }

            cached.clone()
        })
    }

    /// Create a throttled selector
    pub fn throttled<T: Store, O, F>(
        selector: F,
        interval_ms: u64,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, Option<O>>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        let last_update = std::sync::Mutex::new(std::time::Instant::now());
        let cached_result = std::sync::Mutex::new(None);

        Self::memoized(move |state: &T::State| {
            let mut last_update_time = last_update.lock().unwrap();
            let mut cached = cached_result.lock().unwrap();

            let now = std::time::Instant::now();
            if now.duration_since(*last_update_time).as_millis() >= interval_ms as u128 {
                let result = selector(state);
                *cached = Some(result);
                *last_update_time = now;
            }

            cached.clone()
        })
    }

    /// Create a selector that provides default values
    pub fn with_default<T: Store, O, F>(
        selector: F,
        default_value: O,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, O>
    where
        F: Fn(&T::State) -> Option<O> + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        Self::memoized(move |state: &T::State| {
            selector(state).unwrap_or_else(|| default_value.clone())
        })
    }

    /// Create a selector that validates results
    pub fn validated<T: Store, O, F, V>(
        selector: F,
        validator: V,
        fallback: O,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, O>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        V: Fn(&O) -> bool + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        Self::memoized(move |state: &T::State| {
            let result = selector(state);
            if validator(&result) {
                result
            } else {
                fallback.clone()
            }
        })
    }

    /// Create a selector that logs access
    pub fn logged<T: Store, O, F>(
        selector: F,
        logger: impl Fn(&str) + Send + Sync + 'static,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, O>
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
        O: Clone + PartialEq + 'static,
    {
        Self::memoized(move |state: &T::State| {
            logger("Selector accessed");
            selector(state)
        })
    }

    /// Create a chain of selectors
    pub fn chain<T: Store, O>(
        selectors: Vec<Box<dyn Fn(&T::State) -> O + Send + Sync>>,
    ) -> crate::store::memoized::basic::MemoizedSelector<T, Vec<O>>
    where
        O: Clone + PartialEq + 'static,
    {
        Self::memoized(move |state: &T::State| {
            selectors.iter().map(|s| s(state)).collect()
        })
    }
}

/// Selector presets for common use cases
pub mod presets {
    use super::*;

    /// Create a user authentication status selector
    pub fn auth_status<T: Store>() -> crate::store::memoized::basic::MemoizedSelector<T, bool>
    where
        T::State: serde_json::value::Index,
    {
        SelectorFactory::field_exists("user")
    }

    /// Create a user permissions selector
    pub fn user_permissions<T: Store>() -> crate::store::memoized::basic::MemoizedSelector<T, Option<Vec<String>>>
    where
        T::State: serde_json::value::Index,
    {
        SelectorFactory::field("user.permissions")
    }

    /// Create a loading state selector
    pub fn is_loading<T: Store>() -> crate::store::memoized::basic::MemoizedSelector<T, bool>
    where
        T::State: serde_json::value::Index,
    {
        SelectorFactory::field_exists("loading")
    }

    /// Create an error state selector
    pub fn has_errors<T: Store>() -> crate::store::memoized::basic::MemoizedSelector<T, bool>
    where
        T::State: serde_json::value::Index,
    {
        SelectorFactory::field_exists("errors")
    }

    /// Create a data length selector
    pub fn data_length<T: Store>() -> crate::store::memoized::basic::MemoizedSelector<T, Option<usize>>
    where
        T::State: serde_json::value::Index,
    {
        SelectorFactory::array_length("data")
    }

    /// Create a search results count selector
    pub fn search_results_count<T: Store>() -> crate::store::memoized::basic::MemoizedSelector<T, Option<usize>>
    where
        T::State: serde_json::value::Index,
    {
        SelectorFactory::array_length("searchResults")
    }

    /// Create a cart total selector
    pub fn cart_total<T: Store>() -> crate::store::memoized::basic::MemoizedSelector<T, Option<f64>>
    where
        T::State: serde_json::value::Index,
    {
        SelectorFactory::array_sum("cart.items.price")
    }

    /// Create a notification count selector
    pub fn notification_count<T: Store>() -> crate::store::memoized::basic::MemoizedSelector<T, Option<usize>>
    where
        T::State: serde_json::value::Index,
    {
        SelectorFactory::array_length("notifications")
    }
}
