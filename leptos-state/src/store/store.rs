use leptos::prelude::*;
use std::marker::PhantomData;

/// Core trait for defining stores
pub trait Store: Clone + 'static {
    /// The state type for this store
    type State: Clone + PartialEq + Send + Sync + 'static;

    /// Create the initial state
    fn create() -> Self::State;

    /// Get the store's signals (read and write)
    fn use_store() -> (ReadSignal<Self::State>, WriteSignal<Self::State>) {
        use_context::<StoreContext<Self::State>>()
            .expect("Store not provided - did you forget to call provide_store?")
            .signals()
    }
}

/// Context wrapper for store state
#[derive(Clone, Copy)]
pub struct StoreContext<T: Clone + PartialEq + 'static> {
    read: ReadSignal<T>,
    write: WriteSignal<T>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> StoreContext<T> {
    pub fn new(initial: T) -> Self {
        let (read, write) = signal(initial);
        Self { read, write }
    }

    pub fn signals(self) -> (ReadSignal<T>, WriteSignal<T>) {
        (self.read, self.write)
    }
}

/// Provide a store context to child components
pub fn provide_store<S: Store>(initial: S::State) {
    let context = StoreContext::new(initial);
    provide_context(context);
}

/// Provide a store with persistence loading from localStorage
#[cfg(feature = "persist")]
pub fn provide_store_with_persistence<S: Store>(key: &str)
where
    S::State: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    let initial_state = load_from_storage::<S::State>(key).unwrap_or_else(|_| S::create());

    let context = StoreContext::new(initial_state);
    provide_context(context);
}

/// Load state from localStorage
#[cfg(feature = "persist")]
pub fn load_from_storage<T>(key: &str) -> Result<T, crate::utils::StateError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(data)) = storage.get_item(key) {
                    return serde_json::from_str(&data).map_err(|e| {
                        crate::utils::StateError::new(&format!("Failed to deserialize: {}", e))
                    });
                }
            }
        }
        Err(crate::utils::StateError::store_not_found(key))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-WASM targets, use file-based storage
        let storage_path = format!("./storage/{}.json", key);
        if let Ok(data) = std::fs::read_to_string(&storage_path) {
            serde_json::from_str(&data).map_err(|e| {
                crate::utils::StateError::new(&format!("Failed to deserialize: {}", e))
            })
        } else {
            Err(crate::utils::StateError::store_not_found(key))
        }
    }
}

/// Save state to localStorage
#[cfg(feature = "persist")]
pub fn save_to_storage<T>(key: &str, state: &T) -> Result<(), crate::utils::StateError>
where
    T: serde::Serialize,
{
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let data = serde_json::to_string(state).map_err(|e| {
                    crate::utils::StateError::new(&format!("Failed to serialize: {}", e))
                })?;
                storage.set_item(key, &data).map_err(|e| {
                    crate::utils::StateError::new(&format!(
                        "Failed to save to localStorage: {:?}",
                        e
                    ))
                })?;
                return Ok(());
            }
        }
        Err(crate::utils::StateError::new("localStorage not available"))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-WASM targets, use file-based storage
        let storage_path = format!("./storage/{}.json", key);
        if let Err(e) = std::fs::create_dir_all("./storage") {
            return Err(crate::utils::StateError::new(&format!(
                "Failed to create storage directory: {}",
                e
            )));
        }
        let data = serde_json::to_string(state)
            .map_err(|e| crate::utils::StateError::new(&format!("Failed to serialize: {}", e)))?;
        std::fs::write(&storage_path, data).map_err(|e| {
            crate::utils::StateError::new(&format!("Failed to write to file: {}", e))
        })?;
        Ok(())
    }
}

/// Trait for selecting slices of store state
pub trait StoreSlice<T: Store> {
    type Output: PartialEq + Clone + Send + Sync + 'static;

    fn select(state: &T::State) -> Self::Output;
}

/// Create a memoized selector for a store slice
pub fn use_store_slice<S: Store, Slice: StoreSlice<S>>() -> Memo<Slice::Output> {
    let (state, _) = S::use_store();
    Memo::new(move |_| Slice::select(&state.get()))
}

/// Create a computed value from store state
pub fn create_computed<S: Store, T: PartialEq + Clone + Send + Sync + 'static>(
    selector: impl Fn(&S::State) -> T + Send + Sync + 'static,
) -> Memo<T> {
    let (state, _) = S::use_store();
    Memo::new(move |_| selector(&state.get()))
}

/// Create a store with the given state type and initial value
pub fn create_store<T>(initial: T) -> SimpleStore<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    SimpleStore { initial }
}

/// A simple store implementation that stores the initial value
pub struct SimpleStore<T> {
    initial: T,
}

impl<T> Clone for SimpleStore<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            initial: self.initial.clone(),
        }
    }
}

impl<T> Store for SimpleStore<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    type State = T;

    fn create() -> Self::State {
        // This won't be called since we provide the initial value
        unreachable!("create_store should provide initial value")
    }
}

impl<T: Clone> SimpleStore<T> {
    /// Get the initial value stored in this store
    pub fn get_initial(&self) -> T {
        self.initial.clone()
    }
}

/// Macro for creating store implementations
#[macro_export]
macro_rules! create_store {
    ($name:ident, $state:ty, $init:expr) => {
        #[derive(Clone)]
        pub struct $name;

        impl Store for $name {
            type State = $state;

            fn create() -> Self::State {
                $init
            }
        }
    };
}

// Advanced selector implementations
pub struct FieldSelector<T, F, O> {
    selector_fn: F,
    _phantom: PhantomData<(T, O)>,
}

impl<T, F, O> FieldSelector<T, F, O>
where
    F: Fn(&T) -> O,
    O: PartialEq + Clone + 'static,
{
    pub fn new(selector_fn: F) -> Self {
        Self {
            selector_fn,
            _phantom: PhantomData,
        }
    }

    pub fn select(&self, state: &T) -> O {
        (self.selector_fn)(state)
    }
}

/// Deep path selector for nested field access
pub struct PathSelector {
    _path: Vec<String>,
}

impl PathSelector {
    pub fn new(path: &str) -> Self {
        Self {
            _path: path.split('.').map(|s| s.to_string()).collect(),
        }
    }

    /// Select a nested field using JSON path notation
    #[cfg(all(feature = "serde", feature = "serde_json"))]
    pub fn select<T>(&self, state: &T) -> Option<serde_json::Value>
    where
        T: serde::Serialize,
    {
        let json = serde_json::to_value(state).ok()?;
        self.select_from_json(&json)
    }

    #[cfg(all(feature = "serde", feature = "serde_json"))]
    fn select_from_json(&self, json: &serde_json::Value) -> Option<serde_json::Value> {
        let mut current = json;

        for segment in &self._path {
            current = match current {
                serde_json::Value::Object(map) => map.get(segment)?,
                serde_json::Value::Array(arr) => {
                    let index: usize = segment.parse().ok()?;
                    arr.get(index)?
                }
                _ => return None,
            };
        }

        Some(current.clone())
    }
}

/// Macro for creating field selectors
#[macro_export]
macro_rules! select_field {
    ($field:ident) => {
        |state| state.$field.clone()
    };
    ($field:ident.$nested:ident) => {
        |state| state.$field.$nested.clone()
    };
    ($field:ident.$nested:ident.$deep:ident) => {
        |state| state.$field.$nested.$deep.clone()
    };
}

/// Memoized selector that prevents unnecessary recalculations
pub struct MemoizedSelector<T, O> {
    last_input: std::cell::RefCell<Option<T>>,
    last_output: std::cell::RefCell<Option<O>>,
    selector_fn: Box<dyn Fn(&T) -> O>,
}

impl<T, O> MemoizedSelector<T, O>
where
    T: Clone + PartialEq,
    O: Clone,
{
    pub fn new<F>(selector_fn: F) -> Self
    where
        F: Fn(&T) -> O + 'static,
    {
        Self {
            last_input: std::cell::RefCell::new(None),
            last_output: std::cell::RefCell::new(None),
            selector_fn: Box::new(selector_fn),
        }
    }

    pub fn select(&self, input: &T) -> O {
        let mut last_input = self.last_input.borrow_mut();
        let mut last_output = self.last_output.borrow_mut();

        if let (Some(prev_input), Some(prev_output)) = (last_input.as_ref(), last_output.as_ref()) {
            if prev_input == input {
                return prev_output.clone();
            }
        }

        let output = (self.selector_fn)(input);
        *last_input = Some(input.clone());
        *last_output = Some(output.clone());

        output
    }
}

/// Combine multiple selectors into a single result
#[cfg(all(feature = "persist", feature = "serde_json"))]
pub struct CombinedSelector<T> {
    selectors: Vec<Box<dyn Fn(&T) -> serde_json::Value>>,
}

#[cfg(feature = "persist")]
impl<T> CombinedSelector<T> {
    pub fn new() -> Self {
        Self {
            selectors: Vec::new(),
        }
    }

    pub fn add_selector<F, O>(mut self, selector: F) -> Self
    where
        F: Fn(&T) -> O + 'static,
        O: serde::Serialize,
    {
        let boxed_selector = Box::new(move |state: &T| {
            serde_json::to_value(selector(state)).unwrap_or(serde_json::Value::Null)
        });
        self.selectors.push(boxed_selector);
        self
    }

    pub fn select(&self, state: &T) -> Vec<serde_json::Value> {
        self.selectors
            .iter()
            .map(|selector| selector(state))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
    pub struct TestState {
        count: i32,
        name: String,
    }

    create_store!(
        TestStore,
        TestState,
        TestState {
            count: 0,
            name: "test".to_string()
        }
    );

    #[test]
    fn store_creation_works() {
        let state = TestStore::create();
        assert_eq!(state.count, 0);
        assert_eq!(state.name, "test");
    }

    #[cfg(feature = "persist")]
    #[test]
    fn test_local_storage_save_load() {
        let test_data = TestState {
            count: 42,
            name: "test_persistence".to_string(),
        };
        let key = "test_key";

        // Test save
        let save_result = save_to_storage(key, &test_data);
        assert!(save_result.is_ok());

        // Test load
        let load_result = load_from_storage::<TestState>(key);
        assert!(load_result.is_ok());

        let loaded_data = load_result.unwrap();
        assert_eq!(loaded_data.count, 42);
        assert_eq!(loaded_data.name, "test_persistence");
    }

    #[test]
    fn test_create_store_actually_works() {
        let initial_state = TestState {
            count: 42,
            name: "test".to_string(),
        };

        // This should create a store that can actually store the initial value
        let store = create_store(initial_state.clone());

        // The store should be able to return the initial state
        let created_state = store.get_initial();
        assert_eq!(created_state, initial_state);
    }

    #[cfg(feature = "persist")]
    #[test]
    fn test_local_storage_not_found() {
        let result = load_from_storage::<TestState>("nonexistent_key");
        assert!(result.is_err());
    }
}
