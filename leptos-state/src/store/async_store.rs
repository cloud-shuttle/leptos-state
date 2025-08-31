//! Async store integration with Leptos Resources

use leptos::*;
use std::marker::PhantomData;
use crate::store::{Store, StoreContext};
use crate::utils::{StateError, StateResult};

/// Async store that integrates with Leptos Resources
pub trait AsyncStore: Store 
where
    Self::LoaderInput: Clone + PartialEq + 'static,
    Self::LoaderOutput: Clone + 'static,
{
    type LoaderInput: Clone + PartialEq + 'static;
    type LoaderOutput: Clone + 'static;
    
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
#[cfg(feature = "serde")]
pub fn use_async_store<A: AsyncStore>(
    input: impl Fn() -> A::LoaderInput + 'static,
) -> (ReadSignal<A::State>, WriteSignal<A::State>, Resource<A::LoaderInput, StateResult<A::LoaderOutput>>) 
where
    A::LoaderOutput: leptos::server_fn::serde::Serialize + for<'de> leptos::server_fn::serde::Deserialize<'de>,
{
    // Create the resource for async loading
    let resource = create_resource(
        input,
        |input| async move { A::load(input).await },
    );
    
    // Create store signals with loading state
    let (state, set_state) = create_signal(A::loading_state());
    
    // Update state based on resource status
    create_effect(move |_| {
        match resource.get() {
            Some(Ok(data)) => {
                set_state.update(|s| A::apply_loaded_data(s, data));
            }
            Some(Err(error)) => {
                set_state.set(A::error_state(error));
            }
            None => {
                // Still loading - keep current state or set loading state
                if matches!(resource.loading().get(), true) {
                    set_state.set(A::loading_state());
                }
            }
        }
    });
    
    (state, set_state, resource)
}

/// Hook for refetching async store data
pub fn use_async_store_actions<A: AsyncStore>(
    resource: Resource<A::LoaderInput, StateResult<A::LoaderOutput>>,
) -> AsyncStoreActions {
    AsyncStoreActions {
        refetch: Box::new(move || resource.refetch()),
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
#[cfg(feature = "serde")]
#[component]
pub fn AsyncStoreProvider<A: AsyncStore>(
    input: A::LoaderInput,
    children: Children,
) -> impl IntoView
where
    A::LoaderInput: Clone,
    A::LoaderOutput: leptos::server_fn::serde::Serialize + for<'de> leptos::server_fn::serde::Deserialize<'de>,
    A: 'static,
{
    let resource = create_resource(
        move || input.clone(),
        |input| async move { A::load(input).await },
    );
    
    let children_clone = children.clone();
    
    view! {
        <Suspense fallback=move || view! { <div>"Loading..."</div> }>
            {move || {
                resource.get().map(|result| match result {
                    Ok(_data) => {
                        let initial_state = A::loading_state();
                        provide_context(StoreContext::new(initial_state));
                        children_clone().into_view()
                    }
                    Err(_error) => {
                        view! { <div>"Error loading data"</div> }.into_view()
                    }
                })
            }}
        </Suspense>
    }
}

/// Cached async store that persists data between loads
pub struct CachedAsyncStore<A: AsyncStore> {
    cache_key: String,
    _phantom: PhantomData<A>,
}

impl<A: AsyncStore> CachedAsyncStore<A> {
    pub fn new(cache_key: String) -> Self {
        Self {
            cache_key,
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
        if let Ok(cached_data) = crate::store::load_from_storage::<A::LoaderOutput>(&self.cache_key) {
            return Ok(cached_data);
        }
        
        // Load from network/async source
        let data = A::load(input).await?;
        
        // Cache the result
        if let Err(e) = crate::store::save_to_storage(&self.cache_key, &data) {
            tracing::warn!("Failed to cache async store data: {:?}", e);
        }
        
        Ok(data)
    }
}

/// Infinite loading store for paginated data
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
    I::Page: leptos::server_fn::serde::Serialize + for<'de> leptos::server_fn::serde::Deserialize<'de>,
{
    let (state, set_state) = create_signal(I::loading_state());
    let (loading_more, set_loading_more) = create_signal(false);
    
    // Load initial page
    create_resource(
        move || initial_input.clone(),
        move |input| async move {
            match I::load_page(input).await {
                Ok(page) => {
                    set_state.update(|s| I::append_page(s, page));
                }
                Err(error) => {
                    set_state.set(I::error_state(error));
                }
            }
        },
    );
    
    let load_more = {
        let state = state.clone();
        let set_state = set_state.clone();
        
        Box::new(move || {
            if loading_more.get() || !I::has_more_pages(&state.get()) {
                return;
            }
            
            if let Some(next_input) = I::next_page_input(&state.get()) {
                set_loading_more.set(true);
                
                spawn_local(async move {
                    match I::load_page(next_input).await {
                        Ok(page) => {
                            set_state.update(|s| I::append_page(s, page));
                        }
                        Err(error) => {
                            tracing::error!("Failed to load more data: {:?}", error);
                        }
                    }
                    set_loading_more.set(false);
                });
            }
        })
    };
    
    (state, set_state, load_more)
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
}