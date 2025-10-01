use leptos::*;
use leptos::prelude::*;
use leptos_state_minimal::use_store;
use wasm_bindgen::prelude::wasm_bindgen;

/// Counter state
#[derive(Clone, Default, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
struct CounterState {
    count: i32,
    step: i32,
}

#[component]
fn Counter() -> impl IntoView {
    // Use the store hook
    let (state, actions) = use_store::<CounterState>();

    // Increment handler
    let increment_actions = actions.clone();
    let increment = move |_| {
        increment_actions.update(|s| s.count += s.step).unwrap();
    };

    // Decrement handler
    let decrement_actions = actions.clone();
    let decrement = move |_| {
        decrement_actions.update(|s| s.count -= s.step).unwrap();
    };

    // Change step handler
    let set_step_actions = actions.clone();
    let set_step = move |new_step: i32| {
        let actions = set_step_actions.clone();
        move |_| {
            let step = new_step;
            actions.update(move |s| s.step = step).unwrap();
        }
    };

    view! {
        <div style="text-align: center; padding: 20px;">
            <h1>"Counter Example - Leptos State Minimal"</h1>
            <div style="font-size: 48px; margin: 20px;">
                {move || state.get().count}
            </div>
            <div style="margin: 20px;">
                <button on:click=increment style="font-size: 24px; margin: 10px; padding: 10px 20px;">
                    "+"
                </button>
                <button on:click=decrement style="font-size: 24px; margin: 10px; padding: 10px 20px;">
                    "-"
                </button>
            </div>
            <div style="margin: 20px;">
                <p>"Step size: " {move || state.get().step}</p>
                <button on:click=set_step(1) style="margin: 5px; padding: 5px 10px;">"1"</button>
                <button on:click=set_step(2) style="margin: 5px; padding: 5px 10px;">"2"</button>
                <button on:click=set_step(5) style="margin: 5px; padding: 5px 10px;">"5"</button>
                <button on:click=set_step(10) style="margin: 5px; padding: 5px 10px;">"10"</button>
            </div>
            <div style="margin-top: 40px; padding: 20px; background-color: #f5f5f5; border-radius: 8px;">
                <h3>"Leptos State Minimal Demo"</h3>
                <p>"This counter demonstrates reactive state management with minimal trait bounds."</p>
                <p><strong>"Features:"</strong></p>
                <ul style="text-align: left; display: inline-block;">
                    <li>"Reactive state updates"</li>
                    <li>"Simple trait bounds (Send + Sync + Clone + 'static)"</li>
                    <li>"Type-safe state management"</li>
                    <li>"Easy Leptos integration"</li>
                </ul>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Counter />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
