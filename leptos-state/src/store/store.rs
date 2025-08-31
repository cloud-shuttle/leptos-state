use leptos::*;
use std::marker::PhantomData;

/// Core trait for defining stores
pub trait Store: Clone + 'static {
    /// The state type for this store
    type State: Clone + PartialEq + 'static;
    
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

impl<T: Clone + PartialEq + 'static> StoreContext<T> {
    pub fn new(initial: T) -> Self {
        let (read, write) = create_signal(initial);
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
    let initial_state = load_from_storage::<S::State>(key)
        .unwrap_or_else(|_| S::create());
    
    let context = StoreContext::new(initial_state);
    provide_context(context);
}

/// Load state from localStorage
#[cfg(feature = "persist")]
pub fn load_from_storage<T>(key: &str) -> Result<T, crate::utils::StateError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    // TODO: Implement localStorage functionality
    Err(crate::utils::StateError::store_not_found(key))
}

/// Save state to localStorage
#[cfg(feature = "persist")]
pub fn save_to_storage<T>(key: &str, state: &T) -> Result<(), crate::utils::StateError>
where
    T: serde::Serialize,
{
    // TODO: Implement localStorage functionality
    Ok(())
}

/// Trait for selecting slices of store state
pub trait StoreSlice<T: Store> {
    type Output: PartialEq + Clone + 'static;
    
    fn select(state: &T::State) -> Self::Output;
}

/// Create a memoized selector for a store slice
pub fn use_store_slice<S: Store, Slice: StoreSlice<S>>() -> Memo<Slice::Output> {
    let (state, _) = S::use_store();
    create_memo(move |_| Slice::select(&state.get()))
}

/// Create a computed value from store state
pub fn create_computed<S: Store, T: PartialEq + Clone + 'static>(
    selector: impl Fn(&S::State) -> T + 'static,
) -> Memo<T> {
    let (state, _) = S::use_store();
    create_memo(move |_| selector(&state.get()))
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
    path: Vec<String>,
}

impl PathSelector {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.split('.').map(|s| s.to_string()).collect(),
        }
    }
    
    /// Select a nested field using JSON path notation
    #[cfg(feature = "serde")]
    pub fn select<T>(&self, state: &T) -> Option<serde_json::Value>
    where 
        T: serde::Serialize,
    {
        let json = serde_json::to_value(state).ok()?;
        self.select_from_json(&json)
    }
    
    #[cfg(feature = "serde")]
    fn select_from_json(&self, json: &serde_json::Value) -> Option<serde_json::Value> {
        let mut current = json;
        
        for segment in &self.path {
            current = match current {
                serde_json::Value::Object(map) => map.get(segment)?,
                serde_json::Value::Array(arr) => {
                    let index: usize = segment.parse().ok()?;
                    arr.get(index)?
                },
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
#[cfg(feature = "persist")]
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
        self.selectors.iter().map(|selector| selector(state)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug)]
    pub struct TestState {
        count: i32,
        name: String,
    }

    create_store!(TestStore, TestState, TestState {
        count: 0,
        name: "test".to_string()
    });

    #[test]
    fn store_creation_works() {
        let state = TestStore::create();
        assert_eq!(state.count, 0);
        assert_eq!(state.name, "test");
    }
}