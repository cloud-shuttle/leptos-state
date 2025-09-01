use leptos::*;
use leptos_state::*;
use crate::stores::TodoStore;
use crate::models::*;
use uuid::Uuid;

/// Todo list component
#[component]
pub fn TodoList() -> impl IntoView {
    let store = use_context::<TodoStore>().expect("TodoStore not found");
    let todos = store.filtered_todos();
    
    view! {
        <div class="todo-list-container">
            {move || {
                let todo_list = todos.get();
                if todo_list.is_empty() {
                    view! {
                        <div class="empty-state">
                            <div class="empty-icon">"📝"</div>
                            <h3>"No todos found"</h3>
                            <p>"Create your first todo to get started!"</p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="todo-items">
                            {todo_list.into_iter().map(|todo| {
                                view! {
                                    <TodoItem todo=todo />
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

/// Individual todo item component
#[component]
pub fn TodoItem(todo: Todo) -> impl IntoView {
    let store = use_context::<TodoStore>().expect("TodoStore not found");
            let (is_editing, set_is_editing) = create_signal(false);
        let (is_expanded, set_is_expanded) = create_signal(false);
        
        // Create signals for editing
        let (edit_title, set_edit_title) = create_signal(todo.title.clone());
        let (edit_description, set_edit_description) = create_signal(todo.description.clone());
        let (edit_priority, set_edit_priority) = create_signal(todo.priority.clone());
        let (edit_tags, set_edit_tags) = create_signal(todo.tags.clone());
    
    // Handle save
    let handle_save = move |_| {
        let mut updated_todo = todo.clone();
        updated_todo.update_title(edit_title.get());
        updated_todo.update_description(edit_description.get());
        updated_todo.priority = edit_priority.get();
        updated_todo.tags = edit_tags.get();
        updated_todo.updated_at = chrono::Utc::now();
        
        store.update_todo(todo.id, updated_todo);
        set_is_editing.set(false);
    };
    
    // Handle cancel
    let handle_cancel = move |_| {
        set_edit_title.set(todo.title.clone());
        set_edit_description.set(todo.description.clone());
        set_edit_priority.set(todo.priority.clone());
        set_edit_tags.set(todo.tags.clone());
        set_is_editing.set(false);
    };
    
    view! {
        <div class=move || {
            let mut classes = vec!["todo-item"];
            if todo.completed { classes.push("completed"); }
            if todo.is_overdue() { classes.push("overdue"); }
            classes.join(" ")
        }>
            {move || if is_editing.get() {
                view! {
                    <div class="todo-edit-form">
                        <div class="edit-header">
                            <input
                                type="text"
                                class="edit-title-input"
                                prop:value=edit_title
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_edit_title.set(value);
                                }
                            />
                            <div class="edit-actions">
                                <button class="save-btn" on:click=handle_save>"Save"</button>
                                <button class="cancel-btn" on:click=handle_cancel>"Cancel"</button>
                            </div>
                        </div>
                        
                        <textarea
                            class="edit-description-input"
                            placeholder="Add description..."
                            prop:value=edit_description
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_edit_description.set(Some(value));
                            }
                        />
                        
                        <div class="edit-meta">
                            <select
                                class="priority-select"
                                prop:value=move || edit_priority.get().as_str()
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    let priority = match value.as_str() {
                                        "low" => Priority::Low,
                                        "medium" => Priority::Medium,
                                        "high" => Priority::High,
                                        "urgent" => Priority::Urgent,
                                        _ => Priority::Medium,
                                    };
                                    set_edit_priority.set(priority);
                                }
                            >
                                <option value="low">"Low"</option>
                                <option value="medium">"Medium"</option>
                                <option value="high">"High"</option>
                                <option value="urgent">"Urgent"</option>
                            </select>
                            
                            <input
                                type="text"
                                class="tags-input"
                                placeholder="Tags (comma separated)"
                                prop:value=move || edit_tags.get().join(", ")
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    let tags: Vec<String> = value
                                        .split(',')
                                        .map(|s| s.trim().to_string())
                                        .filter(|s| !s.is_empty())
                                        .collect();
                                    set_edit_tags.set(tags);
                                }
                            />
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="todo-content">
                        <div class="todo-header">
                            <div class="todo-checkbox">
                                <input
                                    type="checkbox"
                                    checked=todo.completed
                                    on:change=move |_| store.toggle_todo(todo.id)
                                />
                            </div>
                            
                            <div class="todo-main" on:click=move |_| set_is_expanded.update(|e| *e = !*e)>
                                <div class="todo-title">
                                    <span class=move || if todo.completed { "title completed" } else { "title" }>
                                        {todo.title}
                                    </span>
                                    <span class=move || format!("priority-badge {}", todo.priority.color_class())>
                                        {move || todo.priority.as_str().to_uppercase()}
                                    </span>
                                </div>
                                
                                {move || if let Some(description) = &todo.description {
                                    view! {
                                        <div class="todo-description">
                                            {description}
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {}.into_view()
                                }}
                                
                                {move || if !todo.tags.is_empty() {
                                    view! {
                                        <div class="todo-tags">
                                            {todo.tags.iter().map(|tag| {
                                                view! {
                                                    <span class="tag">{tag}</span>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {}.into_view()
                                }}
                            </div>
                            
                            <div class="todo-actions">
                                <button 
                                    class="edit-btn"
                                    on:click=move |_| set_is_editing.set(true)
                                >
                                    "✏️"
                                </button>
                                <button 
                                    class="delete-btn"
                                    on:click=move |_| store.delete_todo(todo.id)
                                >
                                    "🗑️"
                                </button>
                            </div>
                        </div>
                        
                        {move || if is_expanded.get() {
                            view! {
                                <div class="todo-details">
                                    <div class="detail-item">
                                        <span class="detail-label">"Created:"</span>
                                        <span class="detail-value">{todo.created_at.format("%Y-%m-%d %H:%M")}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="detail-label">"Updated:"</span>
                                        <span class="detail-value">{todo.updated_at.format("%Y-%m-%d %H:%M")}</span>
                                    </div>
                                    {move || if let Some(due_date) = todo.due_date {
                                        view! {
                                            <div class="detail-item">
                                                <span class="detail-label">"Due:"</span>
                                                <span class=move || if todo.is_overdue() { "detail-value overdue" } else { "detail-value" }>
                                                    {due_date.format("%Y-%m-%d %H:%M")}
                                                </span>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }}
                                </div>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }}
                    </div>
                }.into_view()
            }}
        </div>
    }
}

/// Add todo form component
#[component]
pub fn AddTodoForm() -> impl IntoView {
    let store = use_context::<TodoStore>().expect("TodoStore not found");
    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (priority, set_priority) = create_signal(Priority::Medium);
    let (tags, set_tags) = create_signal(String::new());
    let (show_advanced, set_show_advanced) = create_signal(false);
    
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let title_value = title.get();
        if title_value.trim().is_empty() {
            return;
        }
        
        let description_value = description.get();
        let description_opt = if description_value.trim().is_empty() {
            None
        } else {
            Some(description_value)
        };
        
        let tags_value = tags.get();
        let tags_vec: Vec<String> = tags_value
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        let new_todo = Todo::new(title_value, description_opt)
            .with_priority(priority.get())
            .with_tags(tags_vec);
        
        store.add_todo(new_todo);
        
        // Reset form
        set_title.set(String::new());
        set_description.set(String::new());
        set_priority.set(Priority::Medium);
        set_tags.set(String::new());
        set_show_advanced.set(false);
    };
    
    view! {
        <form class="add-todo-form" on:submit=handle_submit>
            <div class="form-header">
                <h3>"Add New Todo"</h3>
                <button 
                    type="button"
                    class="advanced-toggle"
                    on:click=move |_| set_show_advanced.update(|s| *s = !*s)
                >
                    {move || if show_advanced.get() { "Hide Advanced" } else { "Show Advanced" }}
                </button>
            </div>
            
            <div class="form-main">
                <input
                    type="text"
                    class="title-input"
                    placeholder="What needs to be done?"
                    prop:value=title
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        set_title.set(value);
                    }
                    required=true
                />
                
                {move || if show_advanced.get() {
                    view! {
                        <div class="advanced-fields">
                            <textarea
                                class="description-input"
                                placeholder="Add description (optional)"
                                prop:value=description
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_description.set(value);
                                }
                            />
                            
                            <div class="meta-fields">
                                <select
                                    class="priority-select"
                                    prop:value=move || priority.get().as_str()
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        let new_priority = match value.as_str() {
                                            "low" => Priority::Low,
                                            "medium" => Priority::Medium,
                                            "high" => Priority::High,
                                            "urgent" => Priority::Urgent,
                                            _ => Priority::Medium,
                                        };
                                        set_priority.set(new_priority);
                                    }
                                >
                                    <option value="low">"Low Priority"</option>
                                    <option value="medium">"Medium Priority"</option>
                                    <option value="high">"High Priority"</option>
                                    <option value="urgent">"Urgent"</option>
                                </select>
                                
                                <input
                                    type="text"
                                    class="tags-input"
                                    placeholder="Tags (comma separated)"
                                    prop:value=tags
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_tags.set(value);
                                    }
                                />
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }}
            </div>
            
            <div class="form-actions">
                <button type="submit" class="add-btn">"Add Todo"</button>
            </div>
        </form>
    }
}

/// Sidebar component
#[component]
pub fn Sidebar() -> impl IntoView {
    let store = use_context::<TodoStore>().expect("TodoStore not found");
    let all_tags = store.all_tags();
    let overdue_todos = store.overdue_todos();
    let today_todos = store.todos_due_today();
    
    view! {
        <div class="sidebar-content">
            <div class="sidebar-section">
                <h3>"Quick Filters"</h3>
                <ul class="quick-filters">
                    <li>
                        <button 
                            class="quick-filter-btn"
                            on:click=move |_| store.set_filter(TodoFilter::Overdue)
                        >
                            <span class="filter-icon">"⚠️"</span>
                            <span class="filter-text">"Overdue"</span>
                            <span class="filter-count">{move || overdue_todos.get().len()}</span>
                        </button>
                    </li>
                    <li>
                        <button 
                            class="quick-filter-btn"
                            on:click=move |_| store.set_filter(TodoFilter::All)
                        >
                            <span class="filter-icon">"📅"</span>
                            <span class="filter-text">"Due Today"</span>
                            <span class="filter-count">{move || today_todos.get().len()}</span>
                        </button>
                    </li>
                </ul>
            </div>
            
            <div class="sidebar-section">
                <h3>"Tags"</h3>
                <ul class="tag-filters">
                    {move || {
                        let tags = all_tags.get();
                        if tags.is_empty() {
                            view! {
                                <li class="no-tags">"No tags yet"</li>
                            }.into_view()
                        } else {
                            tags.into_iter().map(|tag| {
                                view! {
                                    <li>
                                        <button 
                                            class="tag-filter-btn"
                                            on:click=move |_| store.set_filter(TodoFilter::Tag(tag.clone()))
                                        >
                                            <span class="tag-icon">"🏷️"</span>
                                            <span class="tag-text">{tag}</span>
                                        </button>
                                    </li>
                                }
                            }).collect::<Vec<_>>()
                        }
                    }}
                </ul>
            </div>
            
            <div class="sidebar-section">
                <h3>"Actions"</h3>
                <div class="sidebar-actions">
                    <button 
                        class="sidebar-btn"
                        on:click=move |_| store.clear_completed()
                    >
                        "Clear Completed"
                    </button>
                    <button 
                        class="sidebar-btn"
                        on:click=move |_| {
                            let json = store.export_todos();
                            // In a real app, you'd trigger a download
                            web_sys::console::log_1(&format!("Export: {}", json).into());
                        }
                    >
                        "Export Todos"
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Settings modal component
#[component]
pub fn SettingsModal(
    show: ReadSignal<bool>,
    on_close: Callback<()>,
) -> impl IntoView {
    let store = use_context::<TodoStore>().expect("TodoStore not found");
    let app_state = store.state();
    
    view! {
        {move || if show.get() {
            view! {
                <div class="modal-overlay" on:click=move |_| on_close.call(())>
                    <div class="modal-content" on:click=move |ev| ev.stop_propagation()>
                        <div class="modal-header">
                            <h2>"Settings"</h2>
                            <button class="close-btn" on:click=move |_| on_close.call(())>"×"</button>
                        </div>
                        
                        <div class="modal-body">
                            <div class="setting-group">
                                <h3>"Display"</h3>
                                <div class="setting-item">
                                    <label>
                                        <input
                                            type="checkbox"
                                            checked=move || app_state.get().show_completed
                                            on:change=move |_| store.toggle_show_completed()
                                        />
                                        "Show completed todos"
                                    </label>
                                </div>
                                
                                <div class="setting-item">
                                    <label>"Theme:"</label>
                                    <select
                                        prop:value=move || match app_state.get().theme {
                                            Theme::Light => "light",
                                            Theme::Dark => "dark",
                                            Theme::Auto => "auto",
                                        }
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            let theme = match value.as_str() {
                                                "light" => Theme::Light,
                                                "dark" => Theme::Dark,
                                                "auto" => Theme::Auto,
                                                _ => Theme::Light,
                                            };
                                            store.set_theme(theme);
                                        }
                                    >
                                        <option value="light">"Light"</option>
                                        <option value="dark">"Dark"</option>
                                        <option value="auto">"Auto"</option>
                                    </select>
                                </div>
                            </div>
                            
                            <div class="setting-group">
                                <h3>"Data"</h3>
                                <div class="setting-item">
                                    <label>
                                        <input
                                            type="checkbox"
                                            checked=move || app_state.get().auto_save
                                            on:change=move |_| store.toggle_auto_save()
                                        />
                                        "Auto-save changes"
                                    </label>
                                </div>
                                
                                <div class="setting-item">
                                    <button 
                                        class="danger-btn"
                                        on:click=move |_| {
                                            if web_sys::window()
                                                .and_then(|w| w.confirm_with_message("Are you sure you want to clear all data?").ok())
                                                .unwrap_or(false)
                                            {
                                                // Clear all data
                                                store.dispatch.update(|state| {
                                                    state.todos.clear();
                                                });
                                            }
                                        }
                                    >
                                        "Clear All Data"
                                    </button>
                                </div>
                            </div>
                        </div>
                        
                        <div class="modal-footer">
                            <button class="cancel-btn" on:click=move |_| on_close.call(())>"Cancel"</button>
                        </div>
                    </div>
                </div>
            }.into_view()
        } else {
            view! {}.into_view()
        }}
    }
}
