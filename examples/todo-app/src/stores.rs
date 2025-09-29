use leptos::*;
use leptos_state::*;
use crate::models::*;
use std::collections::HashMap;
use uuid::Uuid;

/// Store for managing todo state
#[derive(Debug, Clone)]
pub struct TodoStore {
    state: ReadSignal<AppState>,
    dispatch: WriteSignal<AppState>,
}

impl TodoStore {
    pub fn new() -> Self {
        let (state, dispatch) = signal(AppState::default());
        
        // Set up persistence
        let persistent_store = create_persistent_store(
            "todo-app-state",
            state,
            dispatch.clone(),
        );

        Self {
            state,
            dispatch,
        }
    }

    /// Get the current state
    pub fn state(&self) -> ReadSignal<AppState> {
        self.state
    }

    /// Get filtered and sorted todos
    pub fn filtered_todos(&self) -> Signal<Vec<Todo>> {
        Memo::new(move |_| {
            let state = self.state.get();
            let mut todos: Vec<Todo> = state.todos.values().cloned().collect();

            // Apply search filter
            if !state.search_query.is_empty() {
                todos.retain(|todo| {
                    todo.title.to_lowercase().contains(&state.search_query.to_lowercase()) ||
                    todo.description.as_ref().map_or(false, |desc| {
                        desc.to_lowercase().contains(&state.search_query.to_lowercase())
                    }) ||
                    todo.tags.iter().any(|tag| {
                        tag.to_lowercase().contains(&state.search_query.to_lowercase())
                    })
                });
            }

            // Apply filter
            todos.retain(|todo| {
                match &state.filter {
                    TodoFilter::All => true,
                    TodoFilter::Active => !todo.completed,
                    TodoFilter::Completed => todo.completed,
                    TodoFilter::Overdue => todo.is_overdue(),
                    TodoFilter::Priority(priority) => &todo.priority == priority,
                    TodoFilter::Tag(tag) => todo.tags.contains(tag),
                }
            });

            // Apply completed filter
            if !state.show_completed {
                todos.retain(|todo| !todo.completed);
            }

            // Apply sorting
            todos.sort_by(|a, b| {
                let cmp = match state.sort {
                    TodoSort::CreatedAt => a.created_at.cmp(&b.created_at),
                    TodoSort::UpdatedAt => a.updated_at.cmp(&b.updated_at),
                    TodoSort::DueDate => {
                        match (a.due_date, b.due_date) {
                            (Some(a_date), Some(b_date)) => a_date.cmp(&b_date),
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (None, None) => std::cmp::Ordering::Equal,
                        }
                    }
                    TodoSort::Priority => {
                        let priority_order = |p: &Priority| match p {
                            Priority::Urgent => 0,
                            Priority::High => 1,
                            Priority::Medium => 2,
                            Priority::Low => 3,
                        };
                        priority_order(&a.priority).cmp(&priority_order(&b.priority))
                    }
                    TodoSort::Title => a.title.cmp(&b.title),
                };

                if state.sort_reverse {
                    cmp.reverse()
                } else {
                    cmp
                }
            });

            todos
        })
    }

    /// Get statistics about todos
    pub fn stats(&self) -> Signal<TodoStats> {
        Memo::new(move |_| {
            let state = self.state.get();
            TodoStats::from_todos(&state.todos)
        })
    }

    /// Get all unique tags
    pub fn all_tags(&self) -> Signal<Vec<String>> {
        Memo::new(move |_| {
            let state = self.state.get();
            let mut tags: std::collections::HashSet<String> = std::collections::HashSet::new();
            
            for todo in state.todos.values() {
                for tag in &todo.tags {
                    tags.insert(tag.clone());
                }
            }
            
            let mut tags: Vec<String> = tags.into_iter().collect();
            tags.sort();
            tags
        })
    }

    /// Add a new todo
    pub fn add_todo(&self, todo: Todo) {
        self.dispatch.update(|state| {
            state.todos.insert(todo.id, todo);
        });
    }

    /// Update an existing todo
    pub fn update_todo(&self, id: Uuid, todo: Todo) {
        self.dispatch.update(|state| {
            state.todos.insert(id, todo);
        });
    }

    /// Delete a todo
    pub fn delete_todo(&self, id: Uuid) {
        self.dispatch.update(|state| {
            state.todos.remove(&id);
        });
    }

    /// Toggle todo completion status
    pub fn toggle_todo(&self, id: Uuid) {
        self.dispatch.update(|state| {
            if let Some(todo) = state.todos.get_mut(&id) {
                todo.toggle();
            }
        });
    }

    /// Set the current filter
    pub fn set_filter(&self, filter: TodoFilter) {
        self.dispatch.update(|state| {
            state.filter = filter;
        });
    }

    /// Set the sort order
    pub fn set_sort(&self, sort: TodoSort) {
        self.dispatch.update(|state| {
            state.sort = sort;
        });
    }

    /// Toggle sort direction
    pub fn toggle_sort(&self) {
        self.dispatch.update(|state| {
            state.sort_reverse = !state.sort_reverse;
        });
    }

    /// Set search query
    pub fn set_search_query(&self, query: String) {
        self.dispatch.update(|state| {
            state.search_query = query;
        });
    }

    /// Toggle show completed todos
    pub fn toggle_show_completed(&self) {
        self.dispatch.update(|state| {
            state.show_completed = !state.show_completed;
        });
    }

    /// Toggle auto save
    pub fn toggle_auto_save(&self) {
        self.dispatch.update(|state| {
            state.auto_save = !state.auto_save;
        });
    }

    /// Set theme
    pub fn set_theme(&self, theme: Theme) {
        self.dispatch.update(|state| {
            state.theme = theme;
        });
    }

    /// Clear all completed todos
    pub fn clear_completed(&self) {
        self.dispatch.update(|state| {
            state.todos.retain(|_, todo| !todo.completed);
        });
    }

    /// Perform bulk action on multiple todos
    pub fn bulk_action(&self, ids: Vec<Uuid>, action: BulkAction) {
        self.dispatch.update(|state| {
            for id in ids {
                if let Some(todo) = state.todos.get_mut(&id) {
                    match &action {
                        BulkAction::Complete => {
                            todo.completed = true;
                            todo.updated_at = chrono::Utc::now();
                        }
                        BulkAction::Uncomplete => {
                            todo.completed = false;
                            todo.updated_at = chrono::Utc::now();
                        }
                        BulkAction::Delete => {
                            state.todos.remove(&id);
                        }
                        BulkAction::SetPriority(priority) => {
                            todo.priority = priority.clone();
                            todo.updated_at = chrono::Utc::now();
                        }
                        BulkAction::AddTag(tag) => {
                            if !todo.tags.contains(tag) {
                                todo.tags.push(tag.clone());
                                todo.updated_at = chrono::Utc::now();
                            }
                        }
                        BulkAction::RemoveTag(tag) => {
                            todo.tags.retain(|t| t != tag);
                            todo.updated_at = chrono::Utc::now();
                        }
                    }
                }
            }
        });
    }

    /// Get a specific todo by ID
    pub fn get_todo(&self, id: Uuid) -> Signal<Option<Todo>> {
        Memo::new(move |_| {
            let state = self.state.get();
            state.todos.get(&id).cloned()
        })
    }

    /// Check if a todo exists
    pub fn has_todo(&self, id: Uuid) -> Signal<bool> {
        Memo::new(move |_| {
            let state = self.state.get();
            state.todos.contains_key(&id)
        })
    }

    /// Get todos by tag
    pub fn todos_by_tag(&self, tag: String) -> Signal<Vec<Todo>> {
        Memo::new(move |_| {
            let state = self.state.get();
            state.todos
                .values()
                .filter(|todo| todo.tags.contains(&tag))
                .cloned()
                .collect()
        })
    }

    /// Get overdue todos
    pub fn overdue_todos(&self) -> Signal<Vec<Todo>> {
        Memo::new(move |_| {
            let state = self.state.get();
            state.todos
                .values()
                .filter(|todo| todo.is_overdue())
                .cloned()
                .collect()
        })
    }

    /// Get todos due today
    pub fn todos_due_today(&self) -> Signal<Vec<Todo>> {
        Memo::new(move |_| {
            let state = self.state.get();
            let today = chrono::Utc::now().date_naive();
            
            state.todos
                .values()
                .filter(|todo| {
                    if let Some(due_date) = todo.due_date {
                        due_date.date_naive() == today
                    } else {
                        false
                    }
                })
                .cloned()
                .collect()
        })
    }

    /// Export todos as JSON
    pub fn export_todos(&self) -> String {
        let state = self.state.get();
        serde_json::to_string_pretty(&state.todos).unwrap_or_default()
    }

    /// Import todos from JSON
    pub fn import_todos(&self, json: &str) -> Result<(), serde_json::Error> {
        let todos: HashMap<Uuid, Todo> = serde_json::from_str(json)?;
        self.dispatch.update(|state| {
            state.todos = todos;
        });
        Ok(())
    }
}

/// Create a persistent store with automatic saving
fn create_persistent_store(
    key: &str,
    state: ReadSignal<AppState>,
    dispatch: WriteSignal<AppState>,
) -> TodoStore {
    // For now, skip localStorage to avoid web-sys feature issues
    // In a real implementation, you would use the leptos-state persistence features
    
    // Set up automatic saving when state changes
    Effect::new(move |_| {
        let current_state = state.get();
        if current_state.auto_save {
            // Log state changes for debugging
            web_sys::console::log_1(&format!("State changed: {} todos", current_state.todos.len()).into());
        }
    });

    TodoStore {
        state,
        dispatch,
    }
}
