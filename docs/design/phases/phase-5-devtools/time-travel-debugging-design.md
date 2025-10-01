# Time Travel Debugging Design

## Overview
Implement time travel debugging capabilities that allow developers to navigate through state history, replay operations, and debug state changes interactively.

## Current State
```rust
// No time travel capabilities
impl<S: State> Store<S> {
    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        self.signal.update(updater);
        Ok(())
    }
}
```

## Proposed Enhancement
```rust
pub struct TimeTravelDebugger<S: State> {
    store: Store<S>,
    history: Vec<StateSnapshot<S>>,
    current_index: usize,
    bookmarks: HashMap<String, usize>,
    replay_mode: bool,
}

impl<S: State> Store<S> {
    pub fn with_time_travel(self) -> (Self, TimeTravelHandle<S>) {
        // Enable time travel debugging
    }
}
```

## Motivation

### Advanced Debugging
- **Historical Navigation**: Jump to any point in state history
- **Operation Replay**: Re-run operations with different parameters
- **State Comparison**: Compare states at different points in time
- **Bug Reproduction**: Recreate exact conditions that caused bugs
- **Interactive Debugging**: Modify state and see cascading effects

### Development Workflow
- **Rapid Iteration**: Test changes without restarting application
- **Hypothesis Testing**: Try different state transitions
- **Performance Analysis**: Compare operation performance over time
- **State Exploration**: Understand complex state interactions

### Use Cases
- Debugging race conditions and timing issues
- Understanding complex state machine flows
- Performance regression analysis
- User interaction replay and debugging
- Learning state management patterns

## Implementation Details

### Time Travel Core
```rust
#[derive(Clone)]
pub struct StateSnapshot<S> {
    pub state: S,
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub operation_index: usize,
    pub metadata: HashMap<String, serde_json::Value>,
    pub stack_trace: Option<String>,
}

pub struct TimeTravelDebugger<S: State> {
    store: Store<S>,
    history: Vec<StateSnapshot<S>>,
    current_index: usize,
    bookmarks: HashMap<String, usize>,
    replay_mode: bool,
    max_history_size: usize,
    auto_snapshot: bool,
}

impl<S: State + Clone> TimeTravelDebugger<S> {
    pub fn new(store: Store<S>, max_history_size: usize) -> Self {
        let initial_snapshot = StateSnapshot {
            state: store.get().get_untracked(),
            timestamp: Utc::now(),
            operation: "initial".to_string(),
            operation_index: 0,
            metadata: HashMap::new(),
            stack_trace: None,
        };

        Self {
            store,
            history: vec![initial_snapshot],
            current_index: 0,
            bookmarks: HashMap::new(),
            replay_mode: false,
            max_history_size,
            auto_snapshot: true,
        }
    }

    pub fn record_operation<F>(
        &mut self,
        operation_name: &str,
        operation: F,
        metadata: HashMap<String, serde_json::Value>
    ) -> Result<(), TimeTravelError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        if !self.auto_snapshot && !self.replay_mode {
            return Ok(());
        }

        let old_state = self.store.get().get_untracked();
        let operation_index = self.history.len();

        // Execute the operation
        self.store.update(operation)?;

        let new_state = self.store.get().get_untracked();

        // Create snapshot
        let snapshot = StateSnapshot {
            state: new_state,
            timestamp: Utc::now(),
            operation: operation_name.to_string(),
            operation_index,
            metadata,
            stack_trace: self.capture_stack_trace(),
        };

        // Remove future history if we're not at the end (time travel was used)
        self.history.truncate(self.current_index + 1);

        // Add new snapshot
        self.history.push(snapshot);
        self.current_index = self.history.len() - 1;

        // Limit history size
        if self.history.len() > self.max_history_size {
            self.history.remove(0);
            self.current_index -= 1;

            // Update bookmarks that might be affected
            self.adjust_bookmarks_after_truncation();
        }

        Ok(())
    }

    pub fn time_travel_to(&mut self, index: usize) -> Result<(), TimeTravelError> {
        if index >= self.history.len() {
            return Err(TimeTravelError::InvalidIndex(index));
        }

        let target_state = &self.history[index].state;
        self.store.set(target_state.clone())?;
        self.current_index = index;
        self.replay_mode = true;

        Ok(())
    }

    pub fn step_back(&mut self) -> Result<(), TimeTravelError> {
        if self.current_index == 0 {
            return Err(TimeTravelError::CannotGoBack);
        }
        self.time_travel_to(self.current_index - 1)
    }

    pub fn step_forward(&mut self) -> Result<(), TimeTravelError> {
        if self.current_index >= self.history.len() - 1 {
            return Err(TimeTravelError::CannotGoForward);
        }
        self.time_travel_to(self.current_index + 1)
    }

    pub fn jump_to_bookmark(&mut self, name: &str) -> Result<(), TimeTravelError> {
        if let Some(&index) = self.bookmarks.get(name) {
            self.time_travel_to(index)
        } else {
            Err(TimeTravelError::BookmarkNotFound(name.to_string()))
        }
    }

    pub fn create_bookmark(&mut self, name: String) -> Result<(), TimeTravelError> {
        if self.bookmarks.contains_key(&name) {
            return Err(TimeTravelError::BookmarkExists(name));
        }
        self.bookmarks.insert(name, self.current_index);
        Ok(())
    }

    pub fn get_current_snapshot(&self) -> &StateSnapshot<S> {
        &self.history[self.current_index]
    }

    pub fn get_history(&self) -> &[StateSnapshot<S>] {
        &self.history
    }

    pub fn get_bookmarks(&self) -> &HashMap<String, usize> {
        &self.bookmarks
    }

    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.current_index < self.history.len() - 1
    }

    pub fn exit_replay_mode(&mut self) {
        self.replay_mode = false;
    }

    fn capture_stack_trace(&self) -> Option<String> {
        // Capture stack trace for debugging
        // This is platform-dependent and may require external crates
        None // Placeholder
    }

    fn adjust_bookmarks_after_truncation(&mut self) {
        // Adjust bookmark indices after history truncation
        self.bookmarks.retain(|_, index| *index < self.history.len());
        for index in self.bookmarks.values_mut() {
            *index -= 1;
        }
    }
}
```

### Operation Replay
```rust
pub struct OperationReplay<S: State> {
    debugger: TimeTravelDebugger<S>,
    recorded_operations: Vec<RecordedOperation>,
}

#[derive(Clone)]
pub struct RecordedOperation {
    pub name: String,
    pub parameters: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub original_state: serde_json::Value,
}

impl<S: State + Clone> OperationReplay<S> {
    pub fn new(debugger: TimeTravelDebugger<S>) -> Self {
        Self {
            debugger,
            recorded_operations: Vec::new(),
        }
    }

    pub fn record_operation_with_params<P>(
        &mut self,
        name: &str,
        params: P,
        operation: impl FnOnce(&mut S, P) + Send + 'static
    ) -> Result<(), TimeTravelError>
    where
        P: Serialize + Clone + Send + 'static,
    {
        let params_value = serde_json::to_value(&params)?;
        let original_state = serde_json::to_value(self.debugger.get_current_snapshot().state.clone())?;

        let recorded = RecordedOperation {
            name: name.to_string(),
            parameters: params_value,
            timestamp: Utc::now(),
            original_state,
        };

        self.recorded_operations.push(recorded);

        let params_clone = params.clone();
        self.debugger.record_operation(name, move |state| {
            operation(state, params_clone);
        }, HashMap::new())?;

        Ok(())
    }

    pub fn replay_operation(&mut self, index: usize) -> Result<(), TimeTravelError> {
        if index >= self.recorded_operations.len() {
            return Err(TimeTravelError::InvalidReplayIndex(index));
        }

        let operation = &self.recorded_operations[index];

        // Restore original state
        let original_state: S = serde_json::from_value(operation.original_state.clone())?;
        self.debugger.store.set(original_state)?;

        // Replay the operation
        match operation.name.as_str() {
            "increment" => {
                let amount: i32 = serde_json::from_value(operation.parameters.clone())?;
                self.debugger.record_operation("replay_increment", move |state| {
                    // Assuming CounterState has count field
                    if let Some(count) = state.as_mut().downcast_mut::<CounterState>() {
                        count.count += amount;
                    }
                }, HashMap::new())?;
            }
            // Handle other operation types...
            _ => return Err(TimeTravelError::UnknownOperation(operation.name.clone())),
        }

        Ok(())
    }

    pub fn replay_with_different_params<P>(
        &mut self,
        index: usize,
        new_params: P
    ) -> Result<(), TimeTravelError>
    where
        P: Serialize + Clone,
    {
        if index >= self.recorded_operations.len() {
            return Err(TimeTravelError::InvalidReplayIndex(index));
        }

        let operation = &self.recorded_operations[index];

        // Restore original state
        let original_state: S = serde_json::from_value(operation.original_state.clone())?;
        self.debugger.store.set(original_state)?;

        // Replay with new parameters
        match operation.name.as_str() {
            "increment" => {
                let amount: i32 = serde_json::from_value(serde_json::to_value(&new_params)?)?;
                self.debugger.record_operation("replay_increment_modified", move |state| {
                    if let Some(count) = state.as_mut().downcast_mut::<CounterState>() {
                        count.count += amount;
                    }
                }, HashMap::new())?;
            }
            _ => return Err(TimeTravelError::UnknownOperation(operation.name.clone())),
        }

        Ok(())
    }

    pub fn get_recorded_operations(&self) -> &[RecordedOperation] {
        &self.recorded_operations
    }
}
```

### State Diffing and Analysis
```rust
pub struct StateDiffer<S: State> {
    differ: Box<dyn Fn(&S, &S) -> StateDiff>,
}

#[derive(Clone, Debug)]
pub struct StateDiff {
    pub changes: Vec<FieldChange>,
    pub added_fields: Vec<String>,
    pub removed_fields: Vec<String>,
    pub summary: String,
}

#[derive(Clone, Debug)]
pub struct FieldChange {
    pub field: String,
    pub old_value: serde_json::Value,
    pub new_value: serde_json::Value,
    pub change_type: ChangeType,
}

#[derive(Clone, Debug)]
pub enum ChangeType {
    Modified,
    Added,
    Removed,
}

impl<S: State> StateDiffer<S> {
    pub fn new<F>(differ: F) -> Self
    where
        F: Fn(&S, &S) -> StateDiff + 'static,
    {
        Self {
            differ: Box::new(differ),
        }
    }

    pub fn default_differ() -> Self
    where
        S: Serialize,
    {
        Self::new(|old, new| {
            let old_json = serde_json::to_value(old).unwrap_or_default();
            let new_json = serde_json::to_value(new).unwrap_or_default();

            Self::diff_json_values(&old_json, &new_json, "".to_string())
        })
    }

    fn diff_json_values(old: &serde_json::Value, new: &serde_json::Value, path: String) -> StateDiff {
        match (old, new) {
            (serde_json::Value::Object(old_obj), serde_json::Value::Object(new_obj)) => {
                let mut changes = Vec::new();
                let mut added = Vec::new();
                let mut removed = Vec::new();

                // Find changes
                for (key, new_val) in new_obj {
                    if let Some(old_val) = old_obj.get(key) {
                        if old_val != new_val {
                            changes.push(FieldChange {
                                field: format!("{}{}", path, key),
                                old_value: old_val.clone(),
                                new_value: new_val.clone(),
                                change_type: ChangeType::Modified,
                            });
                        }
                    } else {
                        added.push(format!("{}{}", path, key));
                    }
                }

                // Find removals
                for key in old_obj.keys() {
                    if !new_obj.contains_key(key) {
                        removed.push(format!("{}{}", path, key));
                    }
                }

                StateDiff {
                    changes,
                    added_fields: added,
                    removed_fields: removed,
                    summary: format!("{} changes, {} added, {} removed",
                                   changes.len(), added.len(), removed.len()),
                }
            }
            _ => {
                if old != new {
                    StateDiff {
                        changes: vec![FieldChange {
                            field: path,
                            old_value: old.clone(),
                            new_value: new.clone(),
                            change_type: ChangeType::Modified,
                        }],
                        added_fields: Vec::new(),
                        removed_fields: Vec::new(),
                        summary: "Value changed".to_string(),
                    }
                } else {
                    StateDiff {
                        changes: Vec::new(),
                        added_fields: Vec::new(),
                        removed_fields: Vec::new(),
                        summary: "No changes".to_string(),
                    }
                }
            }
        }
    }

    pub fn diff(&self, old_state: &S, new_state: &S) -> StateDiff {
        (self.differ)(old_state, new_state)
    }
}

impl<S: State> TimeTravelDebugger<S> {
    pub fn compare_states(&self, index1: usize, index2: usize) -> Result<StateDiff, TimeTravelError>
    where
        S: Serialize,
    {
        if index1 >= self.history.len() || index2 >= self.history.len() {
            return Err(TimeTravelError::InvalidIndex(index1.max(index2)));
        }

        let state1 = &self.history[index1].state;
        let state2 = &self.history[index2].state;

        let differ = StateDiffer::default_differ();
        Ok(differ.diff(state1, state2))
    }

    pub fn find_state_changes(&self, predicate: impl Fn(&StateSnapshot<S>) -> bool) -> Vec<usize> {
        self.history.iter()
            .enumerate()
            .filter_map(|(index, snapshot)| {
                if predicate(snapshot) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_state_at_time(&self, timestamp: DateTime<Utc>) -> Option<usize> {
        self.history.iter()
            .enumerate()
            .find(|(_, snapshot)| snapshot.timestamp >= timestamp)
            .map(|(index, _)| index)
    }
}
```

### Interactive Debugging
```rust
pub struct InteractiveDebugger<S: State> {
    debugger: TimeTravelDebugger<S>,
    breakpoints: Vec<Breakpoint<S>>,
    current_breakpoint: Option<usize>,
}

#[derive(Clone)]
pub struct Breakpoint<S> {
    pub name: String,
    pub condition: Box<dyn Fn(&StateSnapshot<S>) -> bool + Send + Sync>,
    pub action: BreakpointAction,
}

#[derive(Clone)]
pub enum BreakpointAction {
    Pause,
    Log(String),
    ModifyState(Box<dyn Fn(&mut S) + Send + Sync>),
}

impl<S: State + Clone> InteractiveDebugger<S> {
    pub fn new(debugger: TimeTravelDebugger<S>) -> Self {
        Self {
            debugger,
            breakpoints: Vec::new(),
            current_breakpoint: None,
        }
    }

    pub fn add_breakpoint(&mut self, breakpoint: Breakpoint<S>) {
        self.breakpoints.push(breakpoint);
    }

    pub fn check_breakpoints(&mut self, snapshot: &StateSnapshot<S>) -> Option<&BreakpointAction> {
        for (index, breakpoint) in self.breakpoints.iter().enumerate() {
            if (breakpoint.condition)(snapshot) {
                self.current_breakpoint = Some(index);
                return Some(&breakpoint.action);
            }
        }
        None
    }

    pub fn continue_execution(&mut self) {
        self.current_breakpoint = None;
    }

    pub fn run_with_breakpoints(&mut self) -> Result<(), TimeTravelError> {
        for (index, snapshot) in self.debugger.history.iter().enumerate() {
            if let Some(action) = self.check_breakpoints(snapshot) {
                match action {
                    BreakpointAction::Pause => {
                        // Pause execution and wait for user input
                        self.current_breakpoint = Some(index);
                        break;
                    }
                    BreakpointAction::Log(message) => {
                        log::info!("Breakpoint '{}' triggered at index {}: {}",
                                 self.breakpoints[self.current_breakpoint.unwrap()].name,
                                 index, message);
                    }
                    BreakpointAction::ModifyState(modifier) => {
                        let mut modified_state = snapshot.state.clone();
                        modifier(&mut modified_state);

                        // Replace the state at this point
                        self.debugger.history[index].state = modified_state;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_current_breakpoint(&self) -> Option<&Breakpoint<S>> {
        self.current_breakpoint.and_then(|index| self.breakpoints.get(index))
    }
}
```

## Error Handling

### Time Travel Errors
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum TimeTravelError {
    #[error("Invalid history index: {0}")]
    InvalidIndex(usize),

    #[error("Cannot go back from beginning of history")]
    CannotGoBack,

    #[error("Cannot go forward from end of history")]
    CannotGoForward,

    #[error("Bookmark not found: {0}")]
    BookmarkNotFound(String),

    #[error("Bookmark already exists: {0}")]
    BookmarkExists(String),

    #[error("Invalid replay index: {0}")]
    InvalidReplayIndex(usize),

    #[error("Unknown operation for replay: {0}")]
    UnknownOperation(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Store operation failed: {0}")]
    StoreError(String),
}
```

### Safe Time Travel
```rust
impl<S: State> TimeTravelDebugger<S> {
    pub fn time_travel_safely(&mut self, index: usize) -> Result<(), TimeTravelError> {
        // Validate index bounds
        self.validate_index(index)?;

        // Check if time travel would cause issues
        self.validate_time_travel_safety(index)?;

        // Perform time travel
        self.time_travel_to(index)?;

        Ok(())
    }

    fn validate_index(&self, index: usize) -> Result<(), TimeTravelError> {
        if index >= self.history.len() {
            return Err(TimeTravelError::InvalidIndex(index));
        }
        Ok(())
    }

    fn validate_time_travel_safety(&self, index: usize) -> Result<(), TimeTravelError> {
        // Check for any constraints that would make time travel unsafe
        // For example, external side effects that can't be undone
        Ok(())
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn time_travel_basic_navigation() {
    let store = Store::new(TestState { count: 0 });
    let mut debugger = TimeTravelDebugger::new(store, 100);

    // Record some operations
    debugger.record_operation("set_5", |s| s.count = 5, HashMap::new()).unwrap();
    debugger.record_operation("set_10", |s| s.count = 10, HashMap::new()).unwrap();
    debugger.record_operation("set_15", |s| s.count = 15, HashMap::new()).unwrap();

    assert_eq!(debugger.get_current_snapshot().state.count, 15);

    // Time travel back
    debugger.time_travel_to(1).unwrap();
    assert_eq!(debugger.get_current_snapshot().state.count, 10);

    // Time travel forward
    debugger.time_travel_to(2).unwrap();
    assert_eq!(debugger.get_current_snapshot().state.count, 15);
}

#[test]
fn bookmark_functionality() {
    let store = Store::new(TestState { count: 0 });
    let mut debugger = TimeTravelDebugger::new(store, 100);

    debugger.record_operation("init", |s| s.count = 0, HashMap::new()).unwrap();
    debugger.create_bookmark("start".to_string()).unwrap();

    debugger.record_operation("increment", |s| s.count = 1, HashMap::new()).unwrap();
    debugger.create_bookmark("middle".to_string()).unwrap();

    debugger.record_operation("increment", |s| s.count = 2, HashMap::new()).unwrap();
    debugger.create_bookmark("end".to_string()).unwrap();

    // Jump to bookmark
    debugger.jump_to_bookmark("middle").unwrap();
    assert_eq!(debugger.get_current_snapshot().state.count, 1);
}

#[test]
fn operation_replay() {
    let store = Store::new(TestState { count: 0 });
    let debugger = TimeTravelDebugger::new(store, 100);
    let mut replay = OperationReplay::new(debugger);

    // Record operations
    replay.record_operation_with_params("add", 5, |s, amount| s.count += amount).unwrap();
    replay.record_operation_with_params("add", 3, |s, amount| s.count += amount).unwrap();

    assert_eq!(replay.debugger.get_current_snapshot().state.count, 8);

    // Replay with different parameters
    replay.replay_with_different_params(0, 10).unwrap();
    assert_eq!(replay.debugger.get_current_snapshot().state.count, 10);
}
```

### Integration Tests
```rust
#[test]
fn complex_time_travel_workflow() {
    let store = Store::new(ComplexState::default());
    let mut debugger = TimeTravelDebugger::new(store, 100);

    // Simulate complex workflow
    debugger.record_operation("initialize", |s| s.initialize(), HashMap::new()).unwrap();
    debugger.record_operation("process_data", |s| s.process_data(), HashMap::new()).unwrap();
    debugger.record_operation("validate", |s| s.validate(), HashMap::new()).unwrap();
    debugger.record_operation("finalize", |s| s.finalize(), HashMap::new()).unwrap();

    // Create bookmarks at key points
    debugger.create_bookmark("initialized".to_string()).unwrap();
    debugger.create_bookmark("processed".to_string()).unwrap();

    // Time travel to different points and verify state
    debugger.jump_to_bookmark("initialized").unwrap();
    assert!(debugger.get_current_snapshot().state.is_initialized);

    debugger.jump_to_bookmark("processed").unwrap();
    assert!(debugger.get_current_snapshot().state.data_processed);

    // Compare states
    let diff = debugger.compare_states(1, 3).unwrap();
    assert!(!diff.changes.is_empty());
}
```

## Performance Impact

### Memory Usage
- **History Storage**: Linear growth with number of operations
- **Configurable Limits**: Max history size prevents unbounded growth
- **Compression**: Optional compression of old snapshots
- **Selective Recording**: Can disable recording for performance-critical paths

### CPU Overhead
- **Snapshot Creation**: Serialization cost per operation
- **Time Travel**: State restoration cost
- **Diffing**: Comparison cost when analyzing changes
- **Background Processing**: Move heavy operations to background threads

### Optimization Strategies
```rust
impl<S: State> TimeTravelDebugger<S> {
    pub fn with_compression(mut self) -> Self {
        // Compress old snapshots to save memory
        self.compression_enabled = true;
        self
    }

    pub fn with_sampling(mut self, sample_rate: f64) -> Self {
        // Only record a percentage of operations
        self.sampling_rate = sample_rate;
        self
    }

    pub fn with_async_recording(mut self) -> Self {
        // Record operations asynchronously
        self.async_recording = true;
        self
    }

    pub fn optimize_for_performance(mut self) -> Self {
        self.max_history_size = 50; // Smaller history
        self.auto_snapshot = false; // Manual snapshots only
        self.compression_enabled = true;
        self
    }
}
```

## Security Considerations

### State Exposure
- Time travel may expose sensitive historical data
- Filter sensitive information in snapshots
- Access control for time travel operations

```rust
impl<S: State> TimeTravelDebugger<S> {
    pub fn with_data_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&S) -> S + Send + Sync + 'static,
    {
        self.data_filter = Some(Box::new(filter));
        self
    }

    fn filter_snapshot(&self, snapshot: &StateSnapshot<S>) -> StateSnapshot<S>
    where
        S: Clone,
    {
        if let Some(ref filter) = self.data_filter {
            StateSnapshot {
                state: filter(&snapshot.state),
                ..snapshot.clone()
            }
        } else {
            snapshot.clone()
        }
    }
}
```

### Operation Safety
- Prevent dangerous operations during replay
- Validate parameters before replay
- Sandbox replay operations

## Future Extensions

### Collaborative Debugging
```rust
#[cfg(feature = "collaborative-debugging")]
pub struct CollaborativeDebugger<S: State> {
    debugger: TimeTravelDebugger<S>,
    session_id: String,
    collaborators: Vec<String>,
    shared_history: Arc<RwLock<Vec<StateSnapshot<S>>>>,
}

#[cfg(feature = "collaborative-debugging")]
impl<S: State> CollaborativeDebugger<S> {
    pub async fn join_session(&mut self, session_id: &str) -> Result<(), TimeTravelError> {
        // Join collaborative debugging session
        // Sync history with other developers
        todo!()
    }

    pub async fn broadcast_time_travel(&self, index: usize) -> Result<(), TimeTravelError> {
        // Broadcast time travel to all collaborators
        todo!()
    }
}
```

### Automated Testing
```rust
#[cfg(feature = "automated-testing")]
pub struct TestScenario<S: State> {
    debugger: TimeTravelDebugger<S>,
    test_cases: Vec<TestCase<S>>,
}

#[derive(Clone)]
pub struct TestCase<S> {
    pub name: String,
    pub initial_state: S,
    pub operations: Vec<TestOperation>,
    pub expected_final_state: S,
}

impl<S: State + PartialEq> TestScenario<S> {
    pub fn run_tests(&self) -> Vec<TestResult> {
        self.test_cases.iter().map(|test_case| {
            // Reset to initial state
            self.debugger.time_travel_to(0).unwrap();
            self.debugger.store.set(test_case.initial_state.clone()).unwrap();

            // Execute operations
            for operation in &test_case.operations {
                operation.execute(&self.debugger);
            }

            // Check final state
            let final_state = self.debugger.get_current_snapshot().state.clone();
            let passed = final_state == test_case.expected_final_state;

            TestResult {
                test_name: test_case.name.clone(),
                passed,
                actual_state: final_state,
                expected_state: test_case.expected_final_state.clone(),
            }
        }).collect()
    }
}
```

### Visual Time Travel
```rust
#[cfg(feature = "visual-debugging")]
pub struct VisualTimeTravel<S: State> {
    debugger: TimeTravelDebugger<S>,
    visualizer: Box<dyn TimeTravelVisualizer<S>>,
}

#[cfg(feature = "visual-debugging")]
impl<S: State> VisualTimeTravel<S> {
    pub fn render_timeline(&self) -> Result<String, TimeTravelError> {
        // Render interactive timeline visualization
        self.visualizer.render_timeline(&self.debugger.history)
    }

    pub fn render_state_diff(&self, index1: usize, index2: usize) -> Result<String, TimeTravelError> {
        // Render visual diff between two states
        let diff = self.debugger.compare_states(index1, index2)?;
        self.visualizer.render_diff(&diff)
    }

    pub fn render_operation_flow(&self) -> Result<String, TimeTravelError> {
        // Render flowchart of state transitions
        self.visualizer.render_flow(&self.debugger.history)
    }
}
```

## Migration Guide

### Adding Time Travel to Existing Stores
```rust
// Before - basic store
let store = Store::new(initial_state);

// After - with time travel
let (store, debugger) = store.with_time_travel(100); // Max history size

// Wrap operations to record them
store.update_with_recording("increment", |s| s.count += 1, debugger).unwrap();
```

### Gradual Adoption
```rust
// Phase 1: Add basic time travel
let (store, debugger) = create_store_with_time_travel(initial_state);

// Phase 2: Add bookmarks for key states
debugger.create_bookmark("initial".to_string()).unwrap();
store.update_with_recording("setup", |s| s.setup(), debugger).unwrap();
debugger.create_bookmark("ready".to_string()).unwrap();

// Phase 3: Add operation replay
let mut replay = OperationReplay::new(debugger);
replay.record_operation_with_params("configure", config, |s, cfg| s.configure(cfg)).unwrap();

// Phase 4: Add interactive debugging
let mut interactive = InteractiveDebugger::new(replay.debugger);
interactive.add_breakpoint(Breakpoint {
    name: "error_condition".to_string(),
    condition: Box::new(|snapshot| snapshot.state.has_error()),
    action: BreakpointAction::Pause,
});
```

### Configuration-Based Time Travel
```rust
#[derive(Deserialize)]
pub struct TimeTravelConfig {
    pub enable_time_travel: bool,
    pub max_history_size: usize,
    pub enable_bookmarks: bool,
    pub enable_replay: bool,
    pub enable_visualization: bool,
    pub compression_enabled: bool,
}

pub fn create_store_with_time_travel_config<S: State>(
    initial: S,
    config: &TimeTravelConfig
) -> (Store<S>, Option<TimeTravelHandle<S>>) {
    let store = Store::new(initial);

    if !config.enable_time_travel {
        return (store, None);
    }

    let (store, debugger) = store.with_time_travel(config.max_history_size);

    // Configure additional features
    if config.enable_bookmarks {
        // Bookmarks are always available
    }

    if config.enable_replay {
        let replay = OperationReplay::new(debugger);
        // Store replay handle
    }

    if config.compression_enabled {
        debugger.set_compression_enabled(true);
    }

    (store, Some(debugger.into_handle()))
}
```

## Risk Assessment

### Likelihood: High
- Time travel introduces complex state management
- History manipulation can lead to inconsistencies
- Performance impact from history storage
- Memory leaks from unbounded history growth

### Impact: High
- Time travel can break application assumptions
- Large history sizes can cause memory issues
- Replay operations may have unintended side effects
- Complex debugging can hide real issues

### Mitigation
- Clear documentation on time travel limitations
- Configurable history limits with warnings
- Validation of time travel operations
- Clear indicators when in "time travel mode"
- Comprehensive testing of time travel scenarios
- Fallback to normal operation when time travel fails
- Access controls and safety checks
