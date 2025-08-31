use leptos_state::machine::*;
use crate::models::*;
use uuid::Uuid;

/// Context for todo editing state machine
#[derive(Debug, Clone, PartialEq)]
pub struct TodoEditContext {
    pub todo_id: Uuid,
    pub original_todo: Option<Todo>,
    pub edited_todo: Option<Todo>,
    pub validation_errors: Vec<String>,
    pub is_saving: bool,
    pub save_attempts: u32,
}

impl Default for TodoEditContext {
    fn default() -> Self {
        Self {
            todo_id: Uuid::nil(),
            original_todo: None,
            edited_todo: None,
            validation_errors: Vec::new(),
            is_saving: false,
            save_attempts: 0,
        }
    }
}

/// Events for todo editing state machine
#[derive(Debug, Clone, PartialEq)]
pub enum TodoEditEvent {
    StartEdit(Uuid),
    UpdateTitle(String),
    UpdateDescription(Option<String>),
    UpdatePriority(Priority),
    UpdateTags(Vec<String>),
    UpdateDueDate(Option<chrono::DateTime<chrono::Utc>>),
    Validate,
    Save,
    Cancel,
    SaveSuccess,
    SaveError(String),
    Retry,
}

impl Event for TodoEditEvent {
    fn event_type(&self) -> &str {
        match self {
            TodoEditEvent::StartEdit(_) => "start_edit",
            TodoEditEvent::UpdateTitle(_) => "update_title",
            TodoEditEvent::UpdateDescription(_) => "update_description",
            TodoEditEvent::UpdatePriority(_) => "update_priority",
            TodoEditEvent::UpdateTags(_) => "update_tags",
            TodoEditEvent::UpdateDueDate(_) => "update_due_date",
            TodoEditEvent::Validate => "validate",
            TodoEditEvent::Save => "save",
            TodoEditEvent::Cancel => "cancel",
            TodoEditEvent::SaveSuccess => "save_success",
            TodoEditEvent::SaveError(_) => "save_error",
            TodoEditEvent::Retry => "retry",
        }
    }
}

/// Create the todo editing state machine
pub fn create_todo_edit_machine() -> Machine<TodoEditContext, TodoEditEvent> {
    MachineBuilder::new()
        .initial("idle")
        .state("idle")
            .on(TodoEditEvent::StartEdit(Uuid::nil()), "editing")
        .state("editing")
            .on(TodoEditEvent::UpdateTitle("".to_string()), "editing")
            .on(TodoEditEvent::UpdateDescription(None), "editing")
            .on(TodoEditEvent::UpdatePriority(Priority::Medium), "editing")
            .on(TodoEditEvent::UpdateTags(vec![]), "editing")
            .on(TodoEditEvent::UpdateDueDate(None), "editing")
            .on(TodoEditEvent::Validate, "validating")
            .on(TodoEditEvent::Save, "saving")
            .on(TodoEditEvent::Cancel, "idle")
        .state("validating")
            .on(TodoEditEvent::Save, "saving")
            .on(TodoEditEvent::Cancel, "idle")
        .state("saving")
            .on(TodoEditEvent::SaveSuccess, "idle")
            .on(TodoEditEvent::SaveError("".to_string()), "error")
            .on(TodoEditEvent::Cancel, "idle")
        .state("error")
            .on(TodoEditEvent::Retry, "saving")
            .on(TodoEditEvent::Cancel, "idle")
        .build()
}

/// Context for bulk operations state machine
#[derive(Debug, Clone, PartialEq)]
pub struct BulkOpContext {
    pub selected_todos: Vec<Uuid>,
    pub operation: Option<BulkAction>,
    pub progress: f64,
    pub completed_count: usize,
    pub total_count: usize,
    pub errors: Vec<String>,
}

impl Default for BulkOpContext {
    fn default() -> Self {
        Self {
            selected_todos: Vec::new(),
            operation: None,
            progress: 0.0,
            completed_count: 0,
            total_count: 0,
            errors: Vec::new(),
        }
    }
}

/// Events for bulk operations state machine
#[derive(Debug, Clone, PartialEq)]
pub enum BulkOpEvent {
    SelectTodos(Vec<Uuid>),
    SetOperation(BulkAction),
    StartOperation,
    OperationProgress(f64),
    OperationComplete,
    OperationError(String),
    Reset,
}

impl Event for BulkOpEvent {
    fn event_type(&self) -> &str {
        match self {
            BulkOpEvent::SelectTodos(_) => "select_todos",
            BulkOpEvent::SetOperation(_) => "set_operation",
            BulkOpEvent::StartOperation => "start_operation",
            BulkOpEvent::OperationProgress(_) => "operation_progress",
            BulkOpEvent::OperationComplete => "operation_complete",
            BulkOpEvent::OperationError(_) => "operation_error",
            BulkOpEvent::Reset => "reset",
        }
    }
}

/// Create the bulk operations state machine
pub fn create_bulk_op_machine() -> Machine<BulkOpContext, BulkOpEvent> {
    MachineBuilder::new()
        .initial("idle")
        .state("idle")
            .on(BulkOpEvent::SelectTodos(vec![]), "selected")
        .state("selected")
            .on(BulkOpEvent::SetOperation(BulkAction::Complete), "selected")
            .on(BulkOpEvent::StartOperation, "processing")
            .on(BulkOpEvent::Reset, "idle")
        .state("processing")
            .on(BulkOpEvent::OperationProgress(0.0), "processing")
            .on(BulkOpEvent::OperationComplete, "completed")
            .on(BulkOpEvent::OperationError("".to_string()), "error")
        .state("completed")
            .on(BulkOpEvent::Reset, "idle")
        .state("error")
            .on(BulkOpEvent::Reset, "idle")
        .build()
}

/// Context for sync state machine
#[derive(Debug, Clone, PartialEq)]
pub struct SyncContext {
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub pending_changes: Vec<TodoEvent>,
    pub sync_errors: Vec<String>,
    pub is_online: bool,
    pub retry_count: u32,
}

impl Default for SyncContext {
    fn default() -> Self {
        Self {
            last_sync: None,
            pending_changes: Vec::new(),
            sync_errors: Vec::new(),
            is_online: true,
            retry_count: 0,
        }
    }
}

/// Events for sync state machine
#[derive(Debug, Clone, PartialEq)]
pub enum SyncEvent {
    Online,
    Offline,
    LocalChange(TodoEvent),
    SyncRequest,
    SyncStart,
    SyncProgress(f64),
    SyncSuccess,
    SyncError(String),
    RetrySync,
    Reset,
}

impl Event for SyncEvent {
    fn event_type(&self) -> &str {
        match self {
            SyncEvent::Online => "online",
            SyncEvent::Offline => "offline",
            SyncEvent::LocalChange(_) => "local_change",
            SyncEvent::SyncRequest => "sync_request",
            SyncEvent::SyncStart => "sync_start",
            SyncEvent::SyncProgress(_) => "sync_progress",
            SyncEvent::SyncSuccess => "sync_success",
            SyncEvent::SyncError(_) => "sync_error",
            SyncEvent::RetrySync => "retry_sync",
            SyncEvent::Reset => "reset",
        }
    }
}

/// Create the sync state machine
pub fn create_sync_machine() -> Machine<SyncContext, SyncEvent> {
    MachineBuilder::new()
        .initial("idle")
        .state("idle")
            .on(SyncEvent::LocalChange(TodoEvent::AddTodo(Todo::new("".to_string(), None))), "pending")
            .on(SyncEvent::SyncRequest, "syncing")
        .state("pending")
            .on(SyncEvent::LocalChange(TodoEvent::AddTodo(Todo::new("".to_string(), None))), "pending")
            .on(SyncEvent::SyncRequest, "syncing")
            .on(SyncEvent::Offline, "offline")
        .state("syncing")
            .on(SyncEvent::SyncProgress(0.0), "syncing")
            .on(SyncEvent::SyncSuccess, "idle")
            .on(SyncEvent::SyncError("".to_string()), "error")
            .on(SyncEvent::Offline, "offline")
        .state("error")
            .on(SyncEvent::RetrySync, "syncing")
            .on(SyncEvent::Reset, "idle")
        .state("offline")
            .on(SyncEvent::Online, "pending")
            .on(SyncEvent::Reset, "idle")
        .build()
}

/// Context for search state machine
#[derive(Debug, Clone, PartialEq)]
pub struct SearchContext {
    pub query: String,
    pub search_history: Vec<String>,
    pub search_results: Vec<Uuid>,
    pub is_searching: bool,
    pub search_time: std::time::Duration,
}

impl Default for SearchContext {
    fn default() -> Self {
        Self {
            query: String::new(),
            search_history: Vec::new(),
            search_results: Vec::new(),
            is_searching: false,
            search_time: std::time::Duration::ZERO,
        }
    }
}

/// Events for search state machine
#[derive(Debug, Clone, PartialEq)]
pub enum SearchEvent {
    UpdateQuery(String),
    Search,
    SearchComplete(Vec<Uuid>),
    SearchError(String),
    Clear,
}

impl Event for SearchEvent {
    fn event_type(&self) -> &str {
        match self {
            SearchEvent::UpdateQuery(_) => "update_query",
            SearchEvent::Search => "search",
            SearchEvent::SearchComplete(_) => "search_complete",
            SearchEvent::SearchError(_) => "search_error",
            SearchEvent::Clear => "clear",
        }
    }
}

/// Create the search state machine
pub fn create_search_machine() -> Machine<SearchContext, SearchEvent> {
    MachineBuilder::new()
        .initial("idle")
        .state("idle")
            .on(SearchEvent::UpdateQuery("".to_string()), "typing")
        .state("typing")
            .on(SearchEvent::UpdateQuery("".to_string()), "typing")
            .on(SearchEvent::Search, "searching")
            .on(SearchEvent::Clear, "idle")
        .state("searching")
            .on(SearchEvent::SearchComplete(vec![]), "results")
            .on(SearchEvent::SearchError("".to_string()), "error")
        .state("results")
            .on(SearchEvent::UpdateQuery("".to_string()), "typing")
            .on(SearchEvent::Clear, "idle")
        .state("error")
            .on(SearchEvent::Clear, "idle")
        .build()
}

/// State machine manager for the todo app
pub struct TodoStateMachines {
    pub edit_machine: Machine<TodoEditContext, TodoEditEvent>,
    pub bulk_op_machine: Machine<BulkOpContext, BulkOpEvent>,
    pub sync_machine: Machine<SyncContext, SyncEvent>,
    pub search_machine: Machine<SearchContext, SearchEvent>,
}

impl TodoStateMachines {
    pub fn new() -> Self {
        Self {
            edit_machine: create_todo_edit_machine(),
            bulk_op_machine: create_bulk_op_machine(),
            sync_machine: create_sync_machine(),
            search_machine: create_search_machine(),
        }
    }

    /// Get the current state of all machines
    pub fn get_states(&self) -> TodoMachineStates {
        TodoMachineStates {
            edit_state: self.edit_machine.initial_state(),
            bulk_op_state: self.bulk_op_machine.initial_state(),
            sync_state: self.sync_machine.initial_state(),
            search_state: self.search_machine.initial_state(),
        }
    }
}

/// Combined state of all machines
#[derive(Debug, Clone)]
pub struct TodoMachineStates {
    pub edit_state: MachineStateImpl<TodoEditContext>,
    pub bulk_op_state: MachineStateImpl<BulkOpContext>,
    pub sync_state: MachineStateImpl<SyncContext>,
    pub search_state: MachineStateImpl<SearchContext>,
}
