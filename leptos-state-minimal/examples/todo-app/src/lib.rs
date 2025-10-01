use leptos::*;
use leptos::prelude::*;
use leptos_state_minimal::use_store;
use wasm_bindgen::prelude::wasm_bindgen;

/// Todo item structure
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

/// Todo app state
#[derive(Clone, Default, Debug, Eq, PartialEq)]
struct TodoState {
    todos: Vec<TodoItem>,
    next_id: usize,
    new_todo_text: String,
}

#[component]
fn TodoApp() -> impl IntoView {
    // Use the store hook
    let (state, actions) = use_store::<TodoState>();

    // Add todo handler
    let add_todo_actions = actions.clone();
    let add_todo = move |_| {
        add_todo_actions.update(|s| {
            let text = s.new_todo_text.trim().to_string();
            if !text.is_empty() {
                let new_todo = TodoItem {
                    id: s.next_id,
                    text: text.clone(),
                    completed: false,
                };
                s.todos.push(new_todo);
                s.next_id += 1;
                s.new_todo_text = String::new();
            }
        }).unwrap();
    };

    // Toggle todo completion
    let toggle_actions = actions.clone();
    let toggle_todo = move |id: usize| {
        let actions = toggle_actions.clone();
        move |_| {
            actions.update(move |s| {
                if let Some(todo) = s.todos.iter_mut().find(|t| t.id == id) {
                    todo.completed = !todo.completed;
                }
            }).unwrap();
        }
    };

    // Remove todo
    let remove_actions = actions.clone();
    let remove_todo = move |id: usize| {
        let actions = remove_actions.clone();
        move |_| {
            actions.update(move |s| {
                s.todos.retain(|t| t.id != id);
            }).unwrap();
        }
    };

    // Clear completed todos
    let clear_actions = actions.clone();
    let clear_completed = move |_| {
        clear_actions.update(|s| {
            s.todos.retain(|t| !t.completed);
        }).unwrap();
    };

    // Update new todo text
    let update_text_actions = actions.clone();
    let update_text = move |ev| {
        let value = event_target_value(&ev);
        update_text_actions.update(move |s| {
            s.new_todo_text = value;
        }).unwrap();
    };

    // Computed values
    let completed_count = move || {
        state.get().todos.iter().filter(|t| t.completed).count()
    };

    let total_count = move || state.get().todos.len();

    let remaining_count = move || total_count() - completed_count();

    view! {
        <div style="max-width: 600px; margin: 0 auto; padding: 20px; font-family: Arial, sans-serif;">
            <h1 style="text-align: center; color: #333; margin-bottom: 30px;">
                "📝 Todo App - Leptos State Minimal"
            </h1>

            // Add new todo
            <div style="display: flex; gap: 10px; margin-bottom: 20px;">
                <input
                    type="text"
                    placeholder="What needs to be done?"
                    value=move || state.get().new_todo_text.clone()
                    on:input=update_text
                    style="flex: 1; padding: 12px; font-size: 16px; border: 2px solid #ddd; border-radius: 6px; outline: none;"
                />
                <button
                    on:click=add_todo
                    style="padding: 12px 24px; background-color: #4CAF50; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 16px;"
                >
                    "Add Todo"
                </button>
            </div>

            // Todo list
            <div style="border: 1px solid #ddd; border-radius: 8px; overflow: hidden;">
                {move || {
                    let todos = state.get().todos.clone();
                    if todos.is_empty() {
                        view! {
                            <div style="padding: 40px; text-align: center; color: #999;">
                                "No todos yet. Add one above!"
                            </div>
                        }.into_any()
                    } else {
                        todos.into_iter().map(|todo| {
                            let toggle_fn = toggle_todo(todo.id);
                            let remove_fn = remove_todo(todo.id);

                            view! {
                                <div
                                    style=move || format!(
                                        "display: flex; align-items: center; padding: 12px 16px; border-bottom: 1px solid #eee; background-color: {}; transition: background-color 0.2s;",
                                        if todo.completed { "#f8f9fa" } else { "white" }
                                    )
                                >
                                    <input
                                        type="checkbox"
                                        checked=todo.completed
                                        on:change=toggle_fn
                                        style="margin-right: 12px; transform: scale(1.2);"
                                    />
                                    <span
                                        style=move || format!(
                                            "flex: 1; font-size: 16px; {}; text-decoration: {};",
                                            if todo.completed { "color: #999;" } else { "color: #333;" },
                                            if todo.completed { "line-through" } else { "none" }
                                        )
                                    >
                                        {todo.text}
                                    </span>
                                    <button
                                        on:click=remove_fn
                                        style="background: none; border: none; color: #ff4444; cursor: pointer; font-size: 20px; padding: 4px;"
                                        title="Remove todo"
                                    >
                                        "×"
                                    </button>
                                </div>
                            }
                        }).collect::<Vec<_>>().into_any()
                    }
                }}
            </div>

            // Footer with stats and clear button
            <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 20px; padding: 16px; background-color: #f8f9fa; border-radius: 8px;">
                <div style="font-size: 14px; color: #666;">
                    {move || format!("{} of {} remaining", remaining_count(), total_count())}
                </div>
                {move || {
                    let clear_fn = clear_completed.clone();
                    if completed_count() > 0 {
                        view! {
                            <button
                                on:click=clear_fn
                                style="padding: 8px 16px; background-color: #ff9800; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;"
                            >
                                {move || format!("Clear completed ({})", completed_count())}
                            </button>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>

            // Demo info
            <div style="margin-top: 40px; padding: 20px; background-color: #e8f5e8; border-radius: 8px; border: 1px solid #4CAF50;">
                <h3 style="margin-top: 0; color: #2E7D32;">"🎯 Leptos State Minimal Demo"</h3>
                <p style="margin-bottom: 10px;">"This todo app demonstrates reactive state management with minimal trait bounds."</p>
                <p style="margin-bottom: 10px;"><strong>"Features:"</strong></p>
                <ul style="margin: 0; padding-left: 20px; color: #2E7D32;">
                    <li>"Reactive state updates"</li>
                    <li>"Simple trait bounds (Send + Sync + Clone + 'static)"</li>
                    <li>"Type-safe state management"</li>
                    <li>"Easy Leptos integration"</li>
                    <li>"CRUD operations (Create, Read, Update, Delete)"</li>
                    <li>"Real-time UI updates"</li>
                </ul>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <TodoApp />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
