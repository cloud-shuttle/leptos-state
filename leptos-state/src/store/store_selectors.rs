//! Store selectors for slicing state

use super::*;

/// Trait for selecting slices of store state
pub trait StoreSlice<T: Store> {
    /// The output type of this selector
    type Output: Clone + PartialEq + 'static;

    /// Select a value from the store state
    fn select(&self, state: &T::State) -> Self::Output;
}

/// Create a memoized selector for a store slice
#[macro_export]
macro_rules! create_selector {
    ($store:expr, $selector:expr) => {{
        use leptos::*;
        use std::rc::Rc;

        let store = $store.clone();
        let selector = Rc::new($selector);

        create_memo(move |_| {
            let state = store.get();
            selector.select(&state)
        })
    }};
}

/// Create a computed value from store state
#[macro_export]
macro_rules! create_computed {
    ($store:expr, $computer:expr) => {{
        use leptos::*;
        use std::rc::Rc;

        let store = $store.clone();
        let computer = Rc::new($computer);

        create_memo(move |_| {
            let state = store.get();
            computer(&state)
        })
    }};
}

/// Field selector for accessing struct fields
pub struct FieldSelector<T, F, O> {
    /// The field accessor function
    pub accessor: F,
    /// Phantom data for type parameters
    _phantom: std::marker::PhantomData<(T, O)>,
}

impl<T, F, O> FieldSelector<T, F, O>
where
    T: Store,
    F: Fn(&T::State) -> &O,
    O: Clone + PartialEq + 'static,
{
    /// Create a new field selector
    pub fn new(accessor: F) -> Self {
        Self {
            accessor,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, F, O> StoreSlice<T> for FieldSelector<T, F, O>
where
    T: Store,
    F: Fn(&T::State) -> &O,
    O: Clone + PartialEq + 'static,
{
    type Output = O;

    fn select(&self, state: &T::State) -> Self::Output {
        (self.accessor)(state).clone()
    }
}

/// Deep path selector for nested field access
#[derive(Clone)]
pub struct PathSelector {
    /// The path segments (field names)
    pub path: Vec<String>,
}

impl PathSelector {
    /// Create a new path selector
    pub fn new(path: Vec<String>) -> Self {
        Self { path }
    }

    /// Create a path selector from a dot-separated string
    pub fn from_string(path: &str) -> Self {
        Self {
            path: path.split('.').map(|s| s.to_string()).collect(),
        }
    }

    /// Select a value from a JSON-like structure
    pub fn select_json(&self, value: &serde_json::Value) -> Option<serde_json::Value> {
        let mut current = value;

        for segment in &self.path {
            match current {
                serde_json::Value::Object(map) => {
                    if let Some(next) = map.get(segment) {
                        current = next;
                    } else {
                        return None;
                    }
                }
                serde_json::Value::Array(arr) => {
                    if let Ok(index) = segment.parse::<usize>() {
                        if let Some(next) = arr.get(index) {
                            current = next;
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }

        Some(current.clone())
    }
}

/// Macro for creating field selectors
#[macro_export]
macro_rules! selector {
    ($field:ident) => {{
        $crate::store::FieldSelector::new(|state| &state.$field)
    }};
    ($($field:ident).+) => {{
        $crate::store::PathSelector::from_string(stringify!($($field).+))
    }};
}

/// Memoized selector that prevents unnecessary recalculations
pub struct MemoizedSelector<T: Store, O> {
    /// The selector function
    pub selector: Box<dyn Fn(&T::State) -> O + Send + Sync>,
    /// The last computed value
    pub last_value: std::sync::Mutex<Option<O>>,
    /// The last input state
    pub last_state: std::sync::Mutex<Option<T::State>>,
}

impl<T: Store, O: Clone + PartialEq + 'static> MemoizedSelector<T, O> {
    /// Create a new memoized selector
    pub fn new<F>(selector: F) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
    {
        Self {
            selector: Box::new(selector),
            last_value: std::sync::Mutex::new(None),
            last_state: std::sync::Mutex::new(None),
        }
    }

    /// Get the selected value, using memoization
    pub fn get(&self, state: &T::State) -> O {
        let mut last_state = self.last_state.lock().unwrap();
        let mut last_value = self.last_value.lock().unwrap();

        // Check if state has changed
        if last_state.as_ref() != Some(state) {
            *last_state = Some(state.clone());
            *last_value = Some((self.selector)(state));
        }

        last_value.clone().unwrap()
    }

    /// Clear the memoization cache
    pub fn clear_cache(&self) {
        *self.last_state.lock().unwrap() = None;
        *self.last_value.lock().unwrap() = None;
    }
}

/// Combine multiple selectors into a single result
pub struct CombinedSelector<T> {
    /// The combination function
    pub combiner: Box<dyn Fn(&[Box<dyn std::any::Any>]) -> Box<dyn std::any::Any> + Send + Sync>,
    /// The selectors to combine
    pub selectors: Vec<Box<dyn StoreSlice<T>>>,
}

impl<T: Store> CombinedSelector<T> {
    /// Create a new combined selector
    pub fn new<F, S>(combiners: F, selectors: Vec<S>) -> Self
    where
        F: Fn(&[Box<dyn std::any::Any>]) -> Box<dyn std::any::Any> + Send + Sync + 'static,
        S: StoreSlice<T> + 'static,
    {
        Self {
            combiner: Box::new(combiners),
            selectors: selectors.into_iter()
                .map(|s| Box::new(s) as Box<dyn StoreSlice<T>>)
                .collect(),
        }
    }

    /// Select combined values
    pub fn select(&self, state: &T::State) -> Box<dyn std::any::Any> {
        let values: Vec<Box<dyn std::any::Any>> = self.selectors.iter()
            .map(|selector| {
                // This is a simplified implementation
                // In practice, we'd need to handle the different output types
                Box::new(()) as Box<dyn std::any::Any>
            })
            .collect();

        (self.combiner)(&values)
    }
}

/// Selector utilities for common patterns
pub mod selectors {
    use super::*;

    /// Create a selector that returns a boolean value
    pub fn boolean<T, F>(selector: F) -> impl StoreSlice<T>
    where
        T: Store,
        F: Fn(&T::State) -> bool + Clone + 'static,
    {
        struct BooleanSelector<F>(F);

        impl<T, F> StoreSlice<T> for BooleanSelector<F>
        where
            T: Store,
            F: Fn(&T::State) -> bool + Clone,
        {
            type Output = bool;

            fn select(&self, state: &T::State) -> Self::Output {
                (self.0)(state)
            }
        }

        BooleanSelector(selector)
    }

    /// Create a selector that returns a string value
    pub fn string<T, F>(selector: F) -> impl StoreSlice<T>
    where
        T: Store,
        F: Fn(&T::State) -> String + Clone + 'static,
    {
        struct StringSelector<F>(F);

        impl<T, F> StoreSlice<T> for StringSelector<F>
        where
            T: Store,
            F: Fn(&T::State) -> String + Clone,
        {
            type Output = String;

            fn select(&self, state: &T::State) -> Self::Output {
                (self.0)(state)
            }
        }

        StringSelector(selector)
    }

    /// Create a selector that returns a numeric value
    pub fn number<T, F, N>(selector: F) -> impl StoreSlice<T>
    where
        T: Store,
        F: Fn(&T::State) -> N + Clone + 'static,
        N: Clone + PartialEq + 'static,
    {
        struct NumberSelector<F>(F);

        impl<T, F, N> StoreSlice<T> for NumberSelector<F>
        where
            T: Store,
            F: Fn(&T::State) -> N + Clone,
            N: Clone + PartialEq + 'static,
        {
            type Output = N;

            fn select(&self, state: &T::State) -> Self::Output {
                (self.0)(state)
            }
        }

        NumberSelector(selector)
    }

    /// Create a selector that filters an array
    pub fn filter<T, F, I, P>(items_selector: F, predicate: P) -> impl StoreSlice<T>
    where
        T: Store,
        F: Fn(&T::State) -> I + Clone + 'static,
        I: IntoIterator + Clone + 'static,
        I::Item: Clone + 'static,
        P: Fn(&I::Item) -> bool + Clone + 'static,
    {
        struct FilterSelector<F, P>(F, P);

        impl<T, F, I, P> StoreSlice<T> for FilterSelector<F, P>
        where
            T: Store,
            F: Fn(&T::State) -> I + Clone,
            I: IntoIterator + Clone,
            I::Item: Clone + 'static,
            P: Fn(&I::Item) -> bool + Clone,
        {
            type Output = Vec<I::Item>;

            fn select(&self, state: &T::State) -> Self::Output {
                let items = (self.0)(state);
                items.into_iter()
                    .filter(|item| (self.1)(item))
                    .collect()
            }
        }

        FilterSelector(items_selector, predicate)
    }

    /// Create a selector that counts items matching a predicate
    pub fn count<T, F, I, P>(items_selector: F, predicate: P) -> impl StoreSlice<T>
    where
        T: Store,
        F: Fn(&T::State) -> I + Clone + 'static,
        I: IntoIterator + Clone + 'static,
        P: Fn(&I::Item) -> bool + Clone + 'static,
    {
        struct CountSelector<F, P>(F, P);

        impl<T, F, I, P> StoreSlice<T> for CountSelector<F, P>
        where
            T: Store,
            F: Fn(&T::State) -> I + Clone,
            I: IntoIterator + Clone,
            P: Fn(&I::Item) -> bool + Clone,
        {
            type Output = usize;

            fn select(&self, state: &T::State) -> Self::Output {
                let items = (self.0)(state);
                items.into_iter()
                    .filter(|item| (self.1)(item))
                    .count()
            }
        }

        CountSelector(items_selector, predicate)
    }
}
