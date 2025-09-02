use leptos::prelude::{event_target_value, ClassAttribute, ElementChild, Get, OnAttribute, Update};
use leptos::*;
use leptos_state::hooks::use_store::{use_computed, use_store};
use leptos_state::store::provide_store;
use leptos_state::{create_store, mount_to_body, Store};

#[derive(Clone, PartialEq, Debug)]
pub struct CounterState {
    count: i32,
    step: i32,
}

create_store!(
    CounterStore,
    CounterState,
    CounterState { count: 0, step: 1 }
);

#[component]
fn Counter() -> impl IntoView {
    let (state, set_state) = use_store::<CounterStore>();

    let increment = move |_| {
        set_state.update(|s| s.count += s.step);
    };

    let decrement = move |_| {
        set_state.update(|s| s.count -= s.step);
    };

    let reset = move |_| {
        set_state.update(|s| s.count = 0);
    };

    let set_step = move |ev| {
        let value = event_target_value(&ev).parse::<i32>().unwrap_or(1);
        set_state.update(|s| s.step = value);
    };

    view! {
        <div class="counter">
            <h1>"Counter Example"</h1>
            <div class="counter-display">
                <span class="count">{move || state.get().count}</span>
            </div>
            <div class="controls">
                <button on:click=decrement>"-"</button>
                <button on:click=increment>"+"</button>
                <button on:click=reset>"Reset"</button>
            </div>
            <div class="step-control">
                <label>
                    "Step: "
                    <input
                        type="number"
                        value=move || state.get().step
                        on:input=set_step
                        min="1"
                    />
                </label>
            </div>
        </div>
    }
}

#[component]
fn CounterDisplay() -> impl IntoView {
    // Demonstrate selector usage
    let count_doubled = use_computed::<CounterStore, _>(|state| state.count * 2);
    let is_even = use_computed::<CounterStore, _>(|state| state.count % 2 == 0);

    view! {
        <div class="counter-info">
            <p>"Doubled: " {move || count_doubled.get()}</p>
            <p>"Is Even: " {move || if is_even.get() { "Yes" } else { "No" }}</p>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_store::<CounterStore>(CounterStore::create());

    view! {
        <div class="app">
            <Counter />
            <CounterDisplay />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
