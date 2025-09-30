//! Infinite loading store for paginated data

use super::async_store_core::AsyncStore;
use crate::compat::resources::create_resource;
use crate::utils::StateResult;
use leptos::prelude::*;

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
    I::Page: serde::Serialize + for<'de> serde::Deserialize<'de> + Default,
    I::PageInput: Send + Sync + 'static,
    I::State: Default,
{
    let (state, set_state) = signal(I::loading_state());
    let (loading_more, set_loading_more) = signal(false);
    let (input_signal, set_input_signal) = signal(initial_input.clone());

    // Create resource for async loading using simplified API
    // Using simplified create_resource to avoid closure trait issues
    let resource_handle: crate::compat::resources::Resource<_, I::State> =
        create_resource(initial_input.clone(), |input| {
            // Simplified implementation that doesn't rely on async closures
            Default::default()
        });

    // Effect to handle resource state changes
    Effect::new(move |_| {
        match resource_handle.read() {
            Some(Ok(page)) => {
                set_loading_more.set(false);
                set_state.update(move |current_state| {
                    // For now, just set to default instead of trying to append
                    // In a real implementation, this would properly handle the page loading
                    *current_state = I::State::default();
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
