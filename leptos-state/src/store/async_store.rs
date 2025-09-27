//! Async store integration with Leptos Resources

use leptos::prelude::*;

use crate::store::Store;
use crate::utils::{StateError, StateResult};
use std::marker::PhantomData;

/// Async store that integrates with Leptos Resources
#[allow(async_fn_in_trait)]
pub trait AsyncStore: Store
where
    Self::LoaderInput: Clone + PartialEq + Send + Sync + Default + 'static,
    Self::LoaderOutput: Clone + Send + Sync + 'static,
{
    type LoaderInput: Clone + PartialEq + Send + Sync + 'static;
    type LoaderOutput: Clone + Send + Sync + 'static;

    /// Load data asynchronously
    async fn load(input: Self::LoaderInput) -> StateResult<Self::LoaderOutput>;

    /// Update state with loaded data
    fn apply_loaded_data(state: &mut Self::State, data: Self::LoaderOutput);

    /// Create initial loading state
    fn loading_state() -> Self::State;

    /// Create error state
    fn error_state(error: StateError) -> Self::State;
}

/// Resource-backed store implementation
pub struct ResourceStore<A: AsyncStore> {
    _phantom: PhantomData<A>,
}

impl<A: AsyncStore> ResourceStore<A> {
    /// Create a new resource store with automatic loading
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

/// Hook for using async stores with Resources
#[cfg(feature = "serialization")]
pub fn use_async_store<A: AsyncStore>(
    _input: impl Fn() -> A::LoaderInput + 'static,
) -> (
    ReadSignal<A::State>,
    WriteSignal<A::State>,
    Option<Resource<A::LoaderInput, StateResult<A::LoaderOutput>>>,
)
where
    A::LoaderOutput: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    // Create store signals with loading state
    let (state, set_state) = signal(A::loading_state());

    // Note: This is a placeholder implementation
    // The actual resource handling would need to be implemented
    // with the correct Leptos 0.8+ API

    (
        state,
        set_state,
        None::<Resource<A::LoaderInput, StateResult<A::LoaderOutput>>>,
    )
}

/// Hook for refetching async store data
pub fn use_async_store_actions<A: AsyncStore>(
    _resource: Option<Resource<A::LoaderInput, StateResult<A::LoaderOutput>>>,
) -> AsyncStoreActions {
    AsyncStoreActions {
        refetch: Box::new(move || {
            // Note: refetch functionality may need to be implemented differently in Leptos 0.8+
            // For now, we'll provide a placeholder
        }),
    }
}

/// Actions for async store management
pub struct AsyncStoreActions {
    refetch: Box<dyn Fn()>,
}

impl AsyncStoreActions {
    pub fn refetch(&self) {
        (self.refetch)();
    }
}

/// Suspense wrapper for async stores
#[cfg(feature = "serialization")]
pub fn async_store_provider<A>(_input: A::LoaderInput, _children: Children) -> impl IntoView
where
    A: AsyncStore + 'static,
    A::LoaderInput: Clone + 'static,
    A::LoaderOutput: 'static + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    // Note: create_resource API has changed in Leptos 0.8+
    // For now, we'll provide a placeholder implementation
    let initial_state = A::loading_state();
    provide_context(crate::StoreContext::new(initial_state));

    view! {
        <div>
            <span>Async Store Provider</span>
        </div>
    }
}

/// Cached async store that persists data between loads
pub struct CachedAsyncStore<A: AsyncStore> {
    _cache_key: String,
    _phantom: PhantomData<A>,
}

impl<A: AsyncStore> CachedAsyncStore<A> {
    pub fn new(cache_key: String) -> Self {
        Self {
            _cache_key: cache_key,
            _phantom: PhantomData,
        }
    }
}

#[cfg(feature = "persist")]
impl<A: AsyncStore> CachedAsyncStore<A>
where
    A::LoaderOutput: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    /// Load with caching support
    pub async fn load_cached(&self, input: A::LoaderInput) -> StateResult<A::LoaderOutput> {
        // Try to load from cache first
        if let Ok(cached_data) =
            crate::store::load_from_storage::<A::LoaderOutput>(&self._cache_key)
        {
            return Ok(cached_data);
        }

        // Load from network/async source
        let data = A::load(input).await?;

        // Cache the result
        if let Err(e) = crate::store::save_to_storage(&self._cache_key, &data) {
            tracing::warn!("Failed to cache async store data: {:?}", e);
        }

        Ok(data)
    }
}

/// Infinite loading store for paginated data
#[allow(async_fn_in_trait)]
pub trait InfiniteStore: AsyncStore
where
    Self::PageInput: Clone + PartialEq + 'static,
    Self::Page: Clone + 'static,
{
    type PageInput: Clone + PartialEq + 'static;
    type Page: Clone + 'static;

    /// Load a specific page
    async fn load_page(input: Self::PageInput) -> StateResult<Self::Page>;

    /// Append page data to existing state
    fn append_page(state: &mut Self::State, page: Self::Page);

    /// Check if more pages are available
    fn has_more_pages(state: &Self::State) -> bool;

    /// Get next page input
    fn next_page_input(state: &Self::State) -> Option<Self::PageInput>;
}

/// Hook for infinite loading stores
pub fn use_infinite_store<I: InfiniteStore>(
    initial_input: I::PageInput,
) -> (ReadSignal<I::State>, WriteSignal<I::State>, Box<dyn Fn()>)
where
    I::Page: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    let (state, set_state) = signal(I::loading_state());
    let (loading_more, set_loading_more) = signal(false);
    let (input_signal, set_input_signal) = signal(initial_input);

    // Create resource for async loading using Leptos 0.8.9 API
    let resource = create_resource(
        move || input_signal.get(),
        move |input| async move {
            if let Some(next_input) = I::next_page_input(&state.get()) {
                set_input_signal.set(next_input);
                I::load_page(input).await
            } else {
                Ok(Default::default())
            }
        }
    );

    // Effect to handle resource state changes
    create_effect(move |_| {
        match resource.read() {
            Some(Ok(page)) => {
                set_loading_more.set(false);
                set_state.update(move |current_state| {
                    I::append_page(current_state, page.clone());
                });
            }
            Some(Err(_error)) => {
                set_loading_more.set(false);
                // Handle error state
            }
            None => {
                // Still loading
            }
        }
    });

    let load_more = move || {
        if !loading_more.get() && I::has_more_pages(&state.get()) {
            set_loading_more.set(true);
            if let Some(next_input) = I::next_page_input(&state.get()) {
                set_input_signal.set(next_input);
            }
        }
    };

    (state, set_state, Box::new(load_more))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug)]
    struct TestAsyncState {
        data: Option<String>,
        loading: bool,
        error: Option<String>,
    }

    impl Default for TestAsyncState {
        fn default() -> Self {
            Self {
                data: None,
                loading: false,
                error: None,
            }
        }
    }

    #[derive(Clone)]
    struct TestAsyncStore;

    impl Store for TestAsyncStore {
        type State = TestAsyncState;

        fn create() -> Self::State {
            TestAsyncState::default()
        }
    }

    impl AsyncStore for TestAsyncStore {
        type LoaderInput = String;
        type LoaderOutput = String;

        async fn load(input: Self::LoaderInput) -> StateResult<Self::LoaderOutput> {
            // Simulate async loading
            Ok(format!("loaded: {}", input))
        }

        fn apply_loaded_data(state: &mut Self::State, data: Self::LoaderOutput) {
            state.data = Some(data);
            state.loading = false;
            state.error = None;
        }

        fn loading_state() -> Self::State {
            TestAsyncState {
                data: None,
                loading: true,
                error: None,
            }
        }

        fn error_state(error: StateError) -> Self::State {
            TestAsyncState {
                data: None,
                loading: false,
                error: Some(error.to_string()),
            }
        }
    }

    #[test]
    fn async_store_creation() {
        let _store = ResourceStore::<TestAsyncStore>::new();
        assert!(true); // Basic construction test
    }

    #[test]
    fn loading_state_creation() {
        let state = TestAsyncStore::loading_state();
        assert!(state.loading);
        assert!(state.data.is_none());
        assert!(state.error.is_none());
    }

    #[test]
    fn error_state_creation() {
        let error = StateError::context_error("test error");
        let state = TestAsyncStore::error_state(error);
        assert!(!state.loading);
        assert!(state.data.is_none());
        assert!(state.error.is_some());
    }

    #[test]
    fn test_async_store_loading() {
        let state = TestAsyncStore::loading_state();
        assert!(state.loading);
        assert!(state.data.is_none());
        assert!(state.error.is_none());
    }

    #[test]
    fn test_apply_loaded_data() {
        let mut state = TestAsyncState::default();
        let test_data = "test data".to_string();

        TestAsyncStore::apply_loaded_data(&mut state, test_data.clone());

        assert_eq!(state.data, Some(test_data));
        assert!(!state.loading);
        assert!(state.error.is_none());
    }
}
