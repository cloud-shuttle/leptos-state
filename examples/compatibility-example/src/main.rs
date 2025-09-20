//! # Leptos State v1.0.0 Example
//!
//! This example demonstrates how to use the new v1.0.0 architecture
//! with proper store management and hooks.

use leptos::*;
use leptos::prelude::{ElementChild, Update, Get, OnAttribute, ClassAttribute};
use leptos_state::v1::*;
use leptos_state::{
    use_store,
    use_computed,
    use_store_subscription,
    provide_store,
};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct AppState {
    count: i32,
    user: Option<String>,
}

impl StoreState for AppState {}

impl Store for AppState {
    fn create() -> Self {
        Self {
            count: 0,
            user: None,
        }
    }

    fn create_with_state(state: Self) -> Self {
        state
    }

    fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        f(self);
    }

    fn get(&self) -> &Self {
        self
    }

    fn get_mut(&mut self) -> &mut Self {
        self
    }
}

#[component]
fn Counter() -> impl IntoView {
    let (state, set_state) = use_store::<AppState>();

    let increment = move |_| {
        set_state.update(|state| state.count += 1);
    };

    let decrement = move |_| {
        set_state.update(|state| state.count -= 1);
    };

    let reset = move |_| {
        set_state.update(|state| state.count = 0);
    };

    // Use the new v1.0.0 computed hook
    let doubled_count = use_computed::<AppState, i32>(|state| state.count * 2);

    // Use the new v1.0.0 subscription hook
    use_store_subscription::<AppState, _>(|state| {
        if state.count > 10 {
            println!("Count is getting high: {}", state.count);
        }
    });

    view! {
        <div class="counter">
            <h2>"Counter Example (v1.0.0)"</h2>
            <p>"Count: " {move || state.get().count}</p>
            <p>"Doubled: " {move || doubled_count.get()}</p>
            <div class="controls">
                <button on:click=increment>"Increment"</button>
                <button on:click=decrement>"Decrement"</button>
                <button on:click=reset>"Reset"</button>
            </div>
        </div>
    }
}

#[component]
fn UserProfile() -> impl IntoView {
    let (_state, set_state) = use_store::<AppState>();

    let set_user = move |name: String| {
        set_state.update(|state| state.user = Some(name));
    };

    let clear_user = move |_| {
        set_state.update(|state| state.user = None);
    };

    // Use the new v1.0.0 computed hook
    let user_display = use_computed::<AppState, String>(|state| {
        state.user.clone().unwrap_or_else(|| "Guest".to_string())
    });

    view! {
        <div class="user-profile">
            <h2>"User Profile (v1.0.0)"</h2>
            <p>"Current user: " {move || user_display.get()}</p>
            <button on:click=move |_| set_user("Alice".to_string())>"Set as Alice"</button>
            <button on:click=move |_| set_user("Bob".to_string())>"Set as Bob"</button>
            <button on:click=clear_user>"Clear User"</button>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    // Provide the store using the new v1.0.0 API
    provide_store(AppState::create());

    // Use the new v1.0.0 computed hook
    let version_info = use_computed::<AppState, String>(|_| "Running with Leptos State v1.0.0".to_string());

    view! {
        <div class="app">
            <h1>"Leptos State v1.0.0 Example"</h1>
            <p class="version-info">{move || version_info.get()}</p>
            <Counter />
            <UserProfile />
            <style>
                "
                .app {
                    max-width: 800px;
                    margin: 0 auto;
                    padding: 2rem;
                    font-family: Arial, sans-serif;
                }
                
                .counter, .user-profile {
                    margin: 2rem 0;
                    padding: 1.5rem;
                    border: 1px solid #ddd;
                    border-radius: 8px;
                    background: #f9f9f9;
                }
                
                .controls {
                    display: flex;
                    gap: 0.5rem;
                    margin-top: 1rem;
                }
                
                button {
                    padding: 0.5rem 1rem;
                    border: none;
                    border-radius: 4px;
                    background: #007bff;
                    color: white;
                    cursor: pointer;
                }
                
                button:hover {
                    background: #0056b3;
                }
                
                .version-info {
                    color: #666;
                    font-style: italic;
                }
                "
            </style>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> });
}
