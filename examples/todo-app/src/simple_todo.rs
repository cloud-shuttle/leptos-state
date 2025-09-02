use leptos::*;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            completed: false,
        }
    }
}

#[component]
pub fn SimpleTodoApp() -> impl IntoView {
    let (todos, set_todos) = signal(HashMap::new());
    let (new_todo_title, set_new_todo_title) = signal(String::new());

    let handle_add_todo = move |ev: SubmitEvent| {
        ev.prevent_default();
        let title = new_todo_title.get();
        if !title.trim().is_empty() {
            let todo = Todo::new(title);
            set_todos.update(|todos| {
                todos.insert(todo.id, todo);
            });
            set_new_todo_title.set(String::new());
        }
    };

    view! {
        <div style="max-width: 600px; margin: 0 auto; padding: 2rem;">
            <h1>"Simple Todo App"</h1>
            <p>"Built with Leptos and Rust"</p>
            
            <form on:submit=handle_add_todo>
                <input
                    type="text"
                    placeholder="What needs to be done?"
                    prop:value=new_todo_title
                    on:input=move |ev| {
                        let value = ev.target().unwrap().value_of().as_string().unwrap_or_default();
                        set_new_todo_title.set(value);
                    }
                    required=true
                    style="width: 100%; padding: 0.5rem; margin-bottom: 1rem;"
                />
                <button type="submit" style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 0.25rem;">
                    "Add Todo"
                </button>
            </form>
            
            <div>
                <p>"Todo List:"</p>
                {move || {
                    let todo_list: Vec<Todo> = todos.get().values().cloned().collect();
                    view! {
                        <div>
                            {todo_list.into_iter().map(|todo| {
                                let todo_id = todo.id;
                                let completed = todo.completed;
                                let title = todo.title.clone();
                                
                                view! {
                                    <div style="display: flex; align-items: center; padding: 0.5rem; border: 1px solid #e2e8f0; margin-bottom: 0.5rem; border-radius: 0.25rem;">
                                        <input
                                            type="checkbox"
                                            checked=completed
                                            on:change=move |_| {
                                                set_todos.update(|todos| {
                                                    if let Some(todo) = todos.get_mut(&todo_id) {
                                                        todo.completed = !todo.completed;
                                                    }
                                                });
                                            }
                                            style="margin-right: 0.5rem;"
                                        />
                                        <span style=move || if completed { "text-decoration: line-through; color: #64748b; flex: 1;" } else { "flex: 1;" }>
                                            {title}
                                        </span>
                                        <button
                                            on:click=move |_| {
                                                set_todos.update(|todos| {
                                                    todos.remove(&todo_id);
                                                });
                                            }
                                            style="background: #ef4444; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 0.25rem; cursor: pointer;"
                                        >
                                            "Delete"
                                        </button>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_view()
                }}
            </div>
        </div>
    }
}
