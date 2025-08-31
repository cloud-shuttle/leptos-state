use leptos::*;
use leptos_state::*;
use crate::stores::TodoStore;
use crate::state_machines::TodoStateMachines;
use crate::components::*;

/// Main Todo App component
#[component]
pub fn TodoApp() -> impl IntoView {
    // Initialize the store
    let store = TodoStore::new();
    
    // Initialize state machines
    let state_machines = TodoStateMachines::new();
    
    // Provide context for child components
    provide_context(store.clone());
    provide_context(state_machines);
    
    // Create signals for reactive state
    let (show_sidebar, set_show_sidebar) = create_signal(false);
    let (show_settings, set_show_settings) = create_signal(false);
    
    // Get app state
    let app_state = store.state();
    let todos = store.filtered_todos();
    let stats = store.stats();
    
    view! {
        <div class="todo-app">
            // Header
            <header class="app-header">
                <div class="header-left">
                    <button 
                        class="sidebar-toggle"
                        on:click=move |_| set_show_sidebar.update(|s| *s = !*s)
                    >
                        "☰"
                    </button>
                    <h1>"Todo App"</h1>
                </div>
                
                <div class="header-center">
                    <SearchBar />
                </div>
                
                <div class="header-right">
                    <button 
                        class="settings-button"
                        on:click=move |_| set_show_settings.update(|s| *s = !*s)
                    >
                        "⚙️"
                    </button>
                </div>
            </header>
            
            <div class="app-content">
                // Sidebar
                <aside class=move || if show_sidebar.get() { "sidebar open" } else { "sidebar" }>
                    <Sidebar />
                </aside>
                
                // Main content
                <main class="main-content">
                    // Stats bar
                    <div class="stats-bar">
                        <StatsDisplay />
                    </div>
                    
                    // Filters and actions
                    <div class="controls-bar">
                        <FilterBar />
                        <BulkActions />
                    </div>
                    
                    // Todo list
                    <div class="todo-list">
                        <Suspense fallback=view! { <div class="loading">"Loading todos..."</div> }>
                            <TodoList />
                        </Suspense>
                    </div>
                    
                    // Add todo form
                    <div class="add-todo-section">
                        <AddTodoForm />
                    </div>
                </main>
            </div>
            
            // Modals
            <Suspense>
                <SettingsModal 
                    show=show_settings
                    on_close=move |_| set_show_settings.set(false)
                />
            </Suspense>
            
            // Notifications
            <NotificationContainer />
        </div>
    }
}

/// Search bar component
#[component]
fn SearchBar() -> impl IntoView {
    let store = use_context::<TodoStore>().expect("TodoStore not found");
    let (query, set_query) = create_signal(String::new());
    
    // Debounced search
    let debounced_query = create_memo(move |_| query.get());
    
    create_effect(move |_| {
        let q = debounced_query.get();
        store.set_search_query(q);
    });
    
    view! {
        <div class="search-bar">
            <input
                type="text"
                placeholder="Search todos..."
                prop:value=query
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_query.set(value);
                }
                class="search-input"
            />
            <button class="search-button">"🔍"</button>
        </div>
    }
}

/// Stats display component
#[component]
fn StatsDisplay() -> impl IntoView {
    let store = use_context::<TodoStore>().expect("TodoStore not found");
    let stats = store.stats();
    
    view! {
        <div class="stats-display">
            <div class="stat-item">
                <span class="stat-label">"Total:"</span>
                <span class="stat-value">{move || stats.get().total}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">"Active:"</span>
                <span class="stat-value">{move || stats.get().active}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">"Completed:"</span>
                <span class="stat-value">{move || stats.get().completed}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">"Overdue:"</span>
                <span class="stat-value overdue">{move || stats.get().overdue}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">"Progress:"</span>
                <span class="stat-value">{move || format!("{:.1}%", stats.get().completion_rate * 100.0)}</span>
            </div>
        </div>
    }
}

/// Filter bar component
#[component]
fn FilterBar() -> impl IntoView {
    let store = use_context::<TodoStore>().expect("TodoStore not found");
    let app_state = store.state();
    
    view! {
        <div class="filter-bar">
            <div class="filter-group">
                <button 
                    class=move || if app_state.get().filter == TodoFilter::All { "filter-btn active" } else { "filter-btn" }
                    on:click=move |_| store.set_filter(TodoFilter::All)
                >
                    "All"
                </button>
                <button 
                    class=move || if app_state.get().filter == TodoFilter::Active { "filter-btn active" } else { "filter-btn" }
                    on:click=move |_| store.set_filter(TodoFilter::Active)
                >
                    "Active"
                </button>
                <button 
                    class=move || if app_state.get().filter == TodoFilter::Completed { "filter-btn active" } else { "filter-btn" }
                    on:click=move |_| store.set_filter(TodoFilter::Completed)
                >
                    "Completed"
                </button>
                <button 
                    class=move || if app_state.get().filter == TodoFilter::Overdue { "filter-btn active" } else { "filter-btn" }
                    on:click=move |_| store.set_filter(TodoFilter::Overdue)
                >
                    "Overdue"
                </button>
            </div>
            
            <div class="sort-group">
                <select 
                    class="sort-select"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let sort = match value.as_str() {
                            "created" => TodoSort::CreatedAt,
                            "updated" => TodoSort::UpdatedAt,
                            "due" => TodoSort::DueDate,
                            "priority" => TodoSort::Priority,
                            "title" => TodoSort::Title,
                            _ => TodoSort::CreatedAt,
                        };
                        store.set_sort(sort);
                    }
                >
                    <option value="created">"Created"</option>
                    <option value="updated">"Updated"</option>
                    <option value="due">"Due Date"</option>
                    <option value="priority">"Priority"</option>
                    <option value="title">"Title"</option>
                </select>
                
                <button 
                    class="sort-toggle"
                    on:click=move |_| store.toggle_sort()
                >
                    {move || if app_state.get().sort_reverse { "↓" } else { "↑" }}
                </button>
            </div>
        </div>
    }
}

/// Bulk actions component
#[component]
fn BulkActions() -> impl IntoView {
    let store = use_context::<TodoStore>().expect("TodoStore not found");
    let (selected_todos, set_selected_todos) = create_signal(Vec::new());
    
    view! {
        <div class="bulk-actions">
            <button 
                class="bulk-btn"
                disabled=move || selected_todos.get().is_empty()
                on:click=move |_| {
                    let ids = selected_todos.get();
                    if !ids.is_empty() {
                        store.bulk_action(ids, BulkAction::Complete);
                        set_selected_todos.set(Vec::new());
                    }
                }
            >
                "Complete Selected"
            </button>
            
            <button 
                class="bulk-btn"
                disabled=move || selected_todos.get().is_empty()
                on:click=move |_| {
                    let ids = selected_todos.get();
                    if !ids.is_empty() {
                        store.bulk_action(ids, BulkAction::Delete);
                        set_selected_todos.set(Vec::new());
                    }
                }
            >
                "Delete Selected"
            </button>
            
            <button 
                class="bulk-btn"
                on:click=move |_| store.clear_completed()
            >
                "Clear Completed"
            </button>
        </div>
    }
}

/// Notification container component
#[component]
fn NotificationContainer() -> impl IntoView {
    view! {
        <div class="notification-container" id="notifications">
            // Notifications will be dynamically added here
        </div>
    }
}
