use leptos::*;
use leptos::prelude::*;

// Simple reactive state management demonstrating leptos-state concepts
#[derive(Clone, PartialEq, Debug)]
pub struct AppState {
    count: i32,
    step: i32,
    user_name: String,
}

// Store-like pattern for state management
#[derive(Clone)]
pub struct AppStore {
    state: std::sync::Arc<std::sync::RwLock<AppState>>,
}

impl AppStore {
    pub fn new(initial: AppState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::RwLock::new(initial)),
        }
    }

    pub fn get(&self) -> AppState {
        self.state.read().unwrap().clone()
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut AppState),
    {
        let mut state = self.state.write().unwrap();
        f(&mut state);
    }
}

// Create a store macro (simplified version of leptos-state pattern)
macro_rules! create_store {
    ($store_name:ident, $state_type:ty, $initial:expr) => {
        #[derive(Clone)]
        pub struct $store_name {
            inner: AppStore,
        }

        impl $store_name {
            pub fn create() -> Self {
                Self {
                    inner: AppStore::new($initial),
                }
            }

            pub fn get(&self) -> $state_type {
                self.inner.get()
            }

            pub fn update<F>(&self, f: F)
            where
                F: FnOnce(&mut $state_type),
            {
                self.inner.update(f);
            }
        }

        impl std::ops::Deref for $store_name {
            type Target = AppStore;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}

create_store!(
    CounterStore,
    AppState,
    AppState {
        count: 0,
        step: 1,
        user_name: "Guest".to_string()
    }
);

// Hook for using computed values (selector pattern)
fn use_computed<F, T>(store: &CounterStore, selector: F) -> std::sync::Arc<dyn Fn() -> T + Send + Sync>
where
    F: Fn(&AppState) -> T + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    let store = store.clone();
    std::sync::Arc::new(move || selector(&store.get()))
}

#[component]
fn Counter() -> impl IntoView {
    let store = CounterStore::create();

    let increment = {
        let store = store.clone();
        move |_| {
            store.update(|s| s.count += s.step);
        }
    };

    let decrement = {
        let store = store.clone();
        move |_| {
            store.update(|s| s.count -= s.step);
        }
    };

    let reset = {
        let store = store.clone();
        move |_| {
            store.update(|s| s.count = 0);
        }
    };

    let set_step = {
        let store = store.clone();
        move |ev| {
            let value = event_target_value(&ev).parse::<i32>().unwrap_or(1);
            store.update(|s| s.step = value);
        }
    };

    let set_name = {
        let store = store.clone();
        move |ev| {
            let value = event_target_value(&ev);
            store.update(|s| s.user_name = value);
        }
    };

    view! {
        <div style="font-family: Arial, sans-serif; max-width: 400px; margin: 0 auto; padding: 20px;">
            <h1 style="color: #333; text-align: center;">"🚀 Rust WASM State Demo"</h1>
            <div style="text-align: center; margin: 20px 0;">
                <span style="font-size: 48px; font-weight: bold; color: #007acc; display: block;">
                    {move || store.get().count}
                </span>
            </div>
            <div style="display: flex; gap: 10px; justify-content: center; margin: 20px 0;">
                <button
                    style="padding: 10px 20px; background: #dc3545; color: white; border: none; border-radius: 5px; cursor: pointer;"
                    on:click=decrement
                >
                    "-"
                </button>
                <button
                    style="padding: 10px 20px; background: #28a745; color: white; border: none; border-radius: 5px; cursor: pointer;"
                    on:click=increment
                >
                    "+"
                </button>
                <button
                    style="padding: 10px 20px; background: #6c757d; color: white; border: none; border-radius: 5px; cursor: pointer;"
                    on:click=reset
                >
                    "Reset"
                </button>
            </div>
            <div style="margin: 20px 0;">
                <label style="display: block; margin-bottom: 5px;">
                    "Step: "
                    <input
                        type="number"
                        value=move || store.get().step
                        on:input=set_step
                        min="1"
                        style="padding: 5px; border: 1px solid #ccc; border-radius: 3px;"
                    />
                </label>
            </div>
            <div style="margin: 20px 0;">
                <label style="display: block; margin-bottom: 5px;">
                    "Your Name: "
                    <input
                        type="text"
                        value=move || store.get().user_name.clone()
                        on:input=set_name
                        placeholder="Enter your name"
                        style="padding: 5px; border: 1px solid #ccc; border-radius: 3px; width: 100%;"
                    />
                </label>
                <div style="margin-top: 10px; padding: 10px; background: #f8f9fa; border-radius: 3px;">
                    {move || format!("Hello, {}!", store.get().user_name)}
                </div>
            </div>
        </div>
    }
}

#[component]
fn CounterDisplay() -> impl IntoView {
    let store = CounterStore::create();

    // Demonstrate selector usage with computed values
    let count_doubled = use_computed(&store, |state| state.count * 2);
    let is_even = use_computed(&store, |state| state.count % 2 == 0);

    view! {
        <div style="background: #e9ecef; padding: 15px; border-radius: 5px; margin: 20px 0;">
            <h3 style="margin: 0 0 10px 0; color: #495057;">"📊 Live Statistics"</h3>
            <p style="margin: 5px 0;">
                "Doubled: " <strong>{move || count_doubled()}</strong>
            </p>
            <p style="margin: 5px 0;">
                "Is Even: " <strong>{move || if is_even() { "Yes ✅" } else { "No ❌" }}</strong>
            </p>
            <p style="margin: 5px 0; font-size: 12px; color: #6c757d;">
                "✨ Powered by Rust WASM reactive state management"
            </p>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; padding: 20px;">
            <div style="background: white; border-radius: 10px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); overflow: hidden;">
                <Counter />
                <CounterDisplay />
            </div>

            <div style="text-align: center; margin-top: 20px; color: white;">
                <h2>"🎯 Rust WASM State Management Demo"</h2>
                <p>"This demo showcases thread-safe reactive state patterns in Rust compiled to WebAssembly"</p>
                <div style="display: flex; justify-content: center; gap: 20px; margin-top: 20px;">
                    <div style="background: rgba(255,255,255,0.1); padding: 15px; border-radius: 5px;">
                        <h4 style="margin: 0; color: #ffd700;">"⚡ Reactive"</h4>
                        <p style="margin: 5px 0; font-size: 14px;">"Automatic UI updates"</p>
                    </div>
                    <div style="background: rgba(255,255,255,0.1); padding: 15px; border-radius: 5px;">
                        <h4 style="margin: 0; color: #ffd700;">"🔄 Thread-Safe"</h4>
                        <p style="margin: 5px 0; font-size: 14px;">"Send + Sync stores"</p>
                    </div>
                    <div style="background: rgba(255,255,255,0.1); padding: 15px; border-radius: 5px;">
                        <h4 style="margin: 0; color: #ffd700;">"🎨 Rust WASM"</h4>
                        <p style="margin: 5px 0; font-size: 14px;">"Native performance"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
