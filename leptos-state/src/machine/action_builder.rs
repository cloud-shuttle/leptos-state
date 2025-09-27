//! Action builder for complex action combinations

use super::*;

/// Builder for complex action combinations
pub struct ActionBuilder<C, E> {
    /// Built actions
    pub actions: Vec<Box<dyn Action<C, E>>>,
    /// Description of the builder
    pub description: String,
}

impl<C, E> ActionBuilder<C, E> {
    /// Create a new action builder
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            description: "Action Builder".to_string(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add a function action
    pub fn function<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut C, &E) + Clone + 'static,
    {
        self.actions.push(Box::new(FunctionAction::new(func)));
        self
    }

    /// Add an assign action
    pub fn assign<T, F>(mut self, assign_fn: F) -> Self
    where
        F: Fn(&mut C, &E) -> T + Clone + 'static,
    {
        self.actions.push(Box::new(AssignAction::new(assign_fn)));
        self
    }

    /// Add a log action
    pub fn log(mut self, message: String) -> Self {
        self.actions.push(Box::new(LogAction::new(message)));
        self
    }

    /// Add a pure action
    pub fn pure<F>(mut self, func: F) -> Self
    where
        F: Fn() + Clone + 'static,
    {
        self.actions.push(Box::new(PureAction::new(func)));
        self
    }

    /// Add a conditional action
    pub fn conditional<F>(mut self, condition: F, action: Box<dyn Action<C, E>>) -> Self
    where
        F: Fn(&C, &E) -> bool + Clone + 'static,
    {
        self.actions.push(Box::new(ConditionalAction::new(condition, action)));
        self
    }

    /// Add a sequential action
    pub fn sequential(mut self, actions: Vec<Box<dyn Action<C, E>>>) -> Self {
        self.actions.push(Box::new(SequentialAction::new(actions)));
        self
    }

    /// Add a parallel action
    pub fn parallel(mut self, actions: Vec<Box<dyn Action<C, E>>>) -> Self {
        self.actions.push(Box::new(ParallelAction::new(actions)));
        self
    }

    /// Add a retry action
    pub fn retry(mut self, action: Box<dyn Action<C, E>>, max_attempts: usize) -> Self {
        self.actions.push(Box::new(RetryAction::new(action, max_attempts)));
        self
    }

    /// Add a timer action
    pub fn timer(mut self, action: Box<dyn Action<C, E>>, timer_name: String) -> Self {
        self.actions.push(Box::new(TimerAction::new(action, timer_name)));
        self
    }

    /// Add a metrics action
    pub fn metrics(mut self, action: Box<dyn Action<C, E>>, metrics_name: String) -> Self {
        self.actions.push(Box::new(MetricsAction::new(action, metrics_name)));
        self
    }

    /// Add a timeout action
    pub fn timeout(mut self, action: Box<dyn Action<C, E>>, timeout: std::time::Duration) -> Self {
        self.actions.push(Box::new(TimeoutAction::new(action, timeout)));
        self
    }

    /// Add a circuit breaker action
    pub fn circuit_breaker(mut self, action: Box<dyn Action<C, E>>, name: String) -> Self {
        self.actions.push(Box::new(CircuitBreakerAction::new(action, name)));
        self
    }

    /// Add a composite action
    pub fn composite(mut self, actions: Vec<Box<dyn Action<C, E>>>, logic: CompositeLogic) -> Self {
        self.actions.push(Box::new(CompositeAction::new(actions, logic)));
        self
    }

    /// Build a sequential action from all added actions
    pub fn build_sequential(self) -> Box<dyn Action<C, E>> {
        Box::new(SequentialAction::new(self.actions)
            .with_description(self.description))
    }

    /// Build a parallel action from all added actions
    pub fn build_parallel(self) -> Box<dyn Action<C, E>> {
        Box::new(ParallelAction::new(self.actions)
            .with_description(self.description))
    }

    /// Build a composite action from all added actions
    pub fn build_composite(self, logic: CompositeLogic) -> Box<dyn Action<C, E>> {
        Box::new(CompositeAction::new(self.actions, logic)
            .with_description(self.description))
    }

    /// Get all built actions
    pub fn get_actions(self) -> Vec<Box<dyn Action<C, E>>> {
        self.actions
    }

    /// Create a conditional action builder
    pub fn when<F>(condition: F) -> ConditionalActionBuilder<C, E, F>
    where
        F: Fn(&C, &E) -> bool + Clone + 'static,
    {
        ConditionalActionBuilder::new(condition)
    }
}

/// Conditional action builder
pub struct ConditionalActionBuilder<C, E, F> {
    condition: F,
}

impl<C, E, F> ConditionalActionBuilder<C, E, F>
where
    F: Fn(&C, &E) -> bool + Clone + 'static,
{
    /// Create a new conditional action builder
    pub fn new(condition: F) -> Self {
        Self { condition }
    }

    /// Specify the action to execute when condition is true
    pub fn then(self, action: Box<dyn Action<C, E>>) -> ConditionalAction<C, E, F> {
        ConditionalAction::new(self.condition, action)
    }
}

/// Action execution result with detailed information
#[derive(Debug, Clone)]
pub struct ActionExecution {
    /// Action description
    pub action_description: String,
    /// Execution start time
    pub start_time: std::time::Instant,
    /// Execution end time
    pub end_time: std::time::Instant,
    /// Whether execution succeeded
    pub success: bool,
    /// Error message if execution failed
    pub error_message: Option<String>,
    /// Memory usage before execution
    pub memory_before: usize,
    /// Memory usage after execution
    pub memory_after: usize,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for ActionExecution {
    fn default() -> Self {
        Self {
            action_description: String::new(),
            start_time: std::time::Instant::now(),
            end_time: std::time::Instant::now(),
            success: true,
            error_message: None,
            memory_before: 0,
            memory_after: 0,
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl ActionExecution {
    /// Create a new action execution result
    pub fn new(action_description: String) -> Self {
        Self {
            action_description,
            start_time: std::time::Instant::now(),
            ..Default::default()
        }
    }

    /// Mark execution as completed successfully
    pub fn success(mut self) -> Self {
        self.end_time = std::time::Instant::now();
        self.success = true;
        self
    }

    /// Mark execution as failed
    pub fn failure(mut self, error_message: String) -> Self {
        self.end_time = std::time::Instant::now();
        self.success = false;
        self.error_message = Some(error_message);
        self
    }

    /// Set memory usage
    pub fn with_memory(mut self, before: usize, after: usize) -> Self {
        self.memory_before = before;
        self.memory_after = after;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get execution duration
    pub fn duration(&self) -> std::time::Duration {
        self.end_time.duration_since(self.start_time)
    }

    /// Get memory delta
    pub fn memory_delta(&self) -> isize {
        self.memory_after as isize - self.memory_before as isize
    }
}

/// Extension trait for executing actions with detailed results
pub trait ActionExecutor<C, E> {
    /// Execute an action and return detailed execution information
    fn execute_with_result(&self, action: &dyn Action<C, E>, context: &mut C, event: &E) -> ActionExecution;

    /// Execute multiple actions and return their results
    fn execute_batch(&self, actions: &[Box<dyn Action<C, E>>], context: &mut C, event: &E) -> Vec<ActionExecution>;

    /// Execute actions with error handling
    fn execute_safe(&self, actions: &[Box<dyn Action<C, E>>], context: &mut C, event: &E) -> Vec<Result<ActionExecution, String>>;
}

impl<C, E> ActionExecutor<C, E> for Vec<Box<dyn Action<C, E>>> {
    fn execute_with_result(&self, action: &dyn Action<C, E>, context: &mut C, event: &E) -> ActionExecution {
        let mut execution = ActionExecution::new(action.description());

        // Record memory before (simplified)
        let memory_before = 0; // Would get actual memory usage

        execution.start_time = std::time::Instant::now();

        // Execute the action
        action.execute(context, event);

        execution.end_time = std::time::Instant::now();

        // Record memory after (simplified)
        let memory_after = 0; // Would get actual memory usage

        execution
            .with_memory(memory_before, memory_after)
            .success()
    }

    fn execute_batch(&self, actions: &[Box<dyn Action<C, E>>], context: &mut C, event: &E) -> Vec<ActionExecution> {
        actions.iter()
            .map(|action| self.execute_with_result(action.as_ref(), context, event))
            .collect()
    }

    fn execute_safe(&self, actions: &[Box<dyn Action<C, E>>], context: &mut C, event: &E) -> Vec<Result<ActionExecution, String>> {
        actions.iter()
            .map(|action| {
                // In a real implementation, this would catch panics and errors
                Ok(self.execute_with_result(action.as_ref(), context, event))
            })
            .collect()
    }
}

/// Fluent action creation utilities
pub mod actions {
    use super::*;

    /// Create a function action
    pub fn function<C, E, F>(func: F) -> Box<dyn Action<C, E>>
    where
        F: Fn(&mut C, &E) + Clone + 'static,
    {
        Box::new(FunctionAction::new(func))
    }

    /// Create a log action
    pub fn log<C, E>(message: String) -> Box<dyn Action<C, E>> {
        Box::new(LogAction::new(message))
    }

    /// Create a pure action
    pub fn pure<C, E, F>(func: F) -> Box<dyn Action<C, E>>
    where
        F: Fn() + Clone + 'static,
    {
        Box::new(PureAction::new(func))
    }

    /// Create a conditional action
    pub fn conditional<C, E, F>(condition: F, action: Box<dyn Action<C, E>>) -> Box<dyn Action<C, E>>
    where
        F: Fn(&C, &E) -> bool + Clone + 'static,
    {
        Box::new(ConditionalAction::new(condition, action))
    }

    /// Create a sequential action
    pub fn sequential<C, E>(actions: Vec<Box<dyn Action<C, E>>>) -> Box<dyn Action<C, E>> {
        Box::new(SequentialAction::new(actions))
    }

    /// Create a parallel action
    pub fn parallel<C, E>(actions: Vec<Box<dyn Action<C, E>>>) -> Box<dyn Action<C, E>> {
        Box::new(ParallelAction::new(actions))
    }

    /// Create a retry action
    pub fn retry<C, E>(action: Box<dyn Action<C, E>>, max_attempts: usize) -> Box<dyn Action<C, E>> {
        Box::new(RetryAction::new(action, max_attempts))
    }

    /// Create a timer action
    pub fn timer<C, E>(action: Box<dyn Action<C, E>>, timer_name: String) -> Box<dyn Action<C, E>> {
        Box::new(TimerAction::new(action, timer_name))
    }

    /// Create a metrics action
    pub fn metrics<C, E>(action: Box<dyn Action<C, E>>, metrics_name: String) -> Box<dyn Action<C, E>> {
        Box::new(MetricsAction::new(action, metrics_name))
    }
}
