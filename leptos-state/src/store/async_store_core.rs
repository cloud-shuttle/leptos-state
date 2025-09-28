//! Core async store traits and implementations

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
