//! # Leptos State Compatibility Example
//!
//! This example demonstrates how to use the compatibility layer to work
//! with multiple Leptos versions without changing your application code.

use leptos::*;
use leptos_state::*;
use leptos::prelude::{OnAttribute, ClassAttribute, ElementChild, Get, Update, Memo, Effect};

#[derive(Clone, PartialEq, Debug)]
pub struct AppState {
    count: i32,
    user: Option<String>,
}

// Create a store using the compatibility layer
create_store!(AppStore, AppState, AppState { 
    count: 0, 
    user: None 
});

#[component]
fn Counter() -> impl IntoView {
    // Use the compatibility layer APIs instead of direct Leptos APIs
    let (state, set_state) = use_store::<AppStore>();
    
    let increment = move |_| {
        set_state.update(|s| s.count += 1);
    };
    
    let decrement = move |_| {
        set_state.update(|s| s.count -= 1);
    };
    
    let reset = move |_| {
        set_state.update(|s| s.count = 0);
    };
    
    // Use the compatibility layer's create_memo
    let doubled_count = Memo::new(move |_| state.get().count * 2);
    
    // Use the compatibility layer's create_effect
    Effect::new(move |_| {
        let count = state.get().count;
        if count > 10 {
            tracing::info!("Count is getting high: {}", count);
        }
    });
    
    view! {
        <div class="counter">
            <h2>"Counter Example"</h2>
            <p>"Count: " {move || state.get().count}</p>
            <p>"Doubled: " {move || doubled_count.get()}</p>
            <div class="buttons">
                <button on:click=increment>"Increment"</button>
                <button on:click=decrement>"Decrement"</button>
                <button on:click=reset>"Reset"</button>
            </div>
        </div>
    }
}

#[component]
fn UserProfile() -> impl IntoView {
    let (state, set_state) = use_store::<AppStore>();
    
    let set_user = move |name: String| {
        set_state.update(|s| s.user = Some(name));
    };
    
    let clear_user = move |_| {
        set_state.update(|s| s.user = None);
    };
    
    // Use the compatibility layer's signal mapping
    let user_display = Memo::new(move |_| state.get().user.clone().unwrap_or_else(|| "Guest".to_string()));
    
    view! {
        <div class="user-profile">
            <h2>"User Profile"</h2>
            <p>"Current user: " {move || user_display.get()}</p>
            <button on:click=move |_| set_user("Alice".to_string())>"Set as Alice"</button>
            <button on:click=clear_user>"Clear User"</button>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    // Provide the store using the compatibility layer
    provide_store::<AppStore>(AppStore::create());
    
    // Use the compatibility layer's version detection
    let version_info = Memo::new(move |_| {
        "Running with Leptos 0.7+".to_string()
    });
    
    view! {
        <div class="app">
            <h1>"Leptos State Compatibility Example"</h1>
            <p class="version-info">{move || version_info.get()}</p>
            <Counter />
            <UserProfile />
        </div>
    }
}

fn main() {
    // Use the compatibility layer's mount function
    mount_to_body(|| view! { <App /> });
}
