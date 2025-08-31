use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a single todo item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            priority: Priority::Medium,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            due_date: None,
        }
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
        self.updated_at = Utc::now();
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            !self.completed && Utc::now() > due_date
        } else {
            false
        }
    }
}

/// Priority levels for todo items
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Urgent => "urgent",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            Priority::Low => "priority-low",
            Priority::Medium => "priority-medium",
            Priority::High => "priority-high",
            Priority::Urgent => "priority-urgent",
        }
    }
}

/// Filter options for displaying todos
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TodoFilter {
    All,
    Active,
    Completed,
    Overdue,
    Priority(Priority),
    Tag(String),
}

impl TodoFilter {
    pub fn as_str(&self) -> String {
        match self {
            TodoFilter::All => "all".to_string(),
            TodoFilter::Active => "active".to_string(),
            TodoFilter::Completed => "completed".to_string(),
            TodoFilter::Overdue => "overdue".to_string(),
            TodoFilter::Priority(priority) => format!("priority-{}", priority.as_str()),
            TodoFilter::Tag(tag) => format!("tag-{}", tag),
        }
    }
}

/// Sort options for todo items
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TodoSort {
    CreatedAt,
    UpdatedAt,
    DueDate,
    Priority,
    Title,
}

/// Application state for the todo app
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    pub todos: HashMap<Uuid, Todo>,
    pub filter: TodoFilter,
    pub sort: TodoSort,
    pub sort_reverse: bool,
    pub search_query: String,
    pub show_completed: bool,
    pub auto_save: bool,
    pub theme: Theme,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            todos: HashMap::new(),
            filter: TodoFilter::All,
            sort: TodoSort::CreatedAt,
            sort_reverse: false,
            search_query: String::new(),
            show_completed: true,
            auto_save: true,
            theme: Theme::Light,
        }
    }
}

/// Theme options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

/// Statistics about the todo list
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoStats {
    pub total: usize,
    pub completed: usize,
    pub active: usize,
    pub overdue: usize,
    pub completion_rate: f64,
}

impl TodoStats {
    pub fn from_todos(todos: &HashMap<Uuid, Todo>) -> Self {
        let total = todos.len();
        let completed = todos.values().filter(|todo| todo.completed).count();
        let overdue = todos.values().filter(|todo| todo.is_overdue()).count();
        let active = total - completed;
        let completion_rate = if total > 0 {
            completed as f64 / total as f64
        } else {
            0.0
        };

        Self {
            total,
            completed,
            active,
            overdue,
            completion_rate,
        }
    }
}

/// Events that can be triggered in the todo app
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TodoEvent {
    AddTodo(Todo),
    UpdateTodo(Uuid, Todo),
    DeleteTodo(Uuid),
    ToggleTodo(Uuid),
    SetFilter(TodoFilter),
    SetSort(TodoSort),
    ToggleSort,
    SetSearchQuery(String),
    ToggleShowCompleted,
    ToggleAutoSave,
    SetTheme(Theme),
    ClearCompleted,
    BulkAction(Vec<Uuid>, BulkAction),
}

/// Bulk actions that can be performed on multiple todos
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BulkAction {
    Complete,
    Uncomplete,
    Delete,
    SetPriority(Priority),
    AddTag(String),
    RemoveTag(String),
}
