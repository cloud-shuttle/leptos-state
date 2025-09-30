//! Action executor trait implementations

use super::*;

/// Enhanced action executor with advanced features
pub struct EnhancedActionExecutor<C, E> {
    /// Action queue
    pub queue: Vec<Box<dyn Action<C, E>>>,
    /// Execution history
    pub history: Vec<ActionExecution>,
    /// Execution statistics
    pub stats: ActionExecutionStats,
    /// Error handling strategy
    pub error_strategy: ErrorHandlingStrategy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorHandlingStrategy {
    /// Stop execution on first error
    StopOnError,
    /// Continue execution on errors
    ContinueOnError,
    /// Retry failed actions
    RetryOnError,
    /// Skip failed actions
    SkipOnError,
}

#[derive(Debug, Clone)]
pub struct ActionExecutionStats {
    /// Total actions executed
    pub total_executed: usize,
    /// Successful executions
    pub successful: usize,
    /// Failed executions
    pub failed: usize,
    /// Total execution time
    pub total_time: std::time::Duration,
    /// Average execution time
    pub average_time: std::time::Duration,
    /// Peak memory usage
    pub peak_memory: usize,
}

impl Default for ActionExecutionStats {
    fn default() -> Self {
        Self {
            total_executed: 0,
            successful: 0,
            failed: 0,
            total_time: std::time::Duration::from_nanos(0),
            average_time: std::time::Duration::from_nanos(0),
            peak_memory: 0,
        }
    }
}

impl<C, E> EnhancedActionExecutor<C, E> {
    /// Create a new enhanced action executor
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            history: Vec::new(),
            stats: ActionExecutionStats::default(),
            error_strategy: ErrorHandlingStrategy::ContinueOnError,
        }
    }

    /// Set error handling strategy
    pub fn with_error_strategy(mut self, strategy: ErrorHandlingStrategy) -> Self {
        self.error_strategy = strategy;
        self
    }

    /// Add an action to the queue
    pub fn add_action(&mut self, action: Box<dyn Action<C, E>>) {
        self.queue.push(action);
    }

    /// Add multiple actions to the queue
    pub fn add_actions(&mut self, actions: Vec<Box<dyn Action<C, E>>>) {
        self.queue.extend(actions);
    }

    /// Execute all actions in the queue
    pub fn execute_all(&mut self, context: &mut C, event: &E) -> Vec<ActionExecution> {
        let mut results = Vec::new();

        for action in &self.queue {
            let result = self.execute_single(action.as_ref(), context, event);
            results.push(result.clone());
            self.history.push(result);
        }

        // Update statistics
        self.update_stats(&results);

        results
    }

    /// Execute a single action
    pub fn execute_single(
        &self,
        action: &dyn Action<C, E>,
        context: &mut C,
        event: &E,
    ) -> ActionExecution {
        let mut execution = ActionExecution::new(action.description());

        // Record memory before (simplified)
        let memory_before = 0; // Would get actual memory usage

        execution.start_time = std::time::Instant::now();

        // Execute the action
        action.execute(context, event);

        execution.end_time = std::time::Instant::now();

        // Record memory after (simplified)
        let memory_after = 0; // Would get actual memory usage

        execution.with_memory(memory_before, memory_after).success()
    }

    /// Execute actions with advanced error handling
    pub fn execute_with_error_handling(
        &mut self,
        context: &mut C,
        event: &E,
    ) -> ExecutionResult<C, E> {
        let mut successful_actions = Vec::new();
        let mut failed_actions = Vec::new();
        let mut results = Vec::new();

        for action in &self.queue {
            let result = match self.error_strategy {
                ErrorHandlingStrategy::StopOnError => {
                    // Try to execute, stop on error
                    let execution = self.execute_single(action.as_ref(), context, event);
                    if !execution.success {
                        failed_actions.push((action.clone_action(), execution));
                        break;
                    }
                    execution
                }
                ErrorHandlingStrategy::ContinueOnError => {
                    // Execute and continue regardless of errors
                    self.execute_single(action.as_ref(), context, event)
                }
                ErrorHandlingStrategy::RetryOnError => {
                    // Retry logic would go here
                    self.execute_single(action.as_ref(), context, event)
                }
                ErrorHandlingStrategy::SkipOnError => {
                    // Skip logic would go here
                    self.execute_single(action.as_ref(), context, event)
                }
            };

            results.push(result.clone());
            self.history.push(result.clone());

            if result.success {
                successful_actions.push(action.clone_action());
            } else {
                failed_actions.push((action.clone_action(), result));
            }
        }

        // Update statistics
        self.update_stats(&results);

        ExecutionResult {
            successful_actions,
            failed_actions,
            total_execution_time: results.iter().map(|r| r.duration()).sum(),
            error_strategy: self.error_strategy.clone(),
        }
    }

    /// Get execution history
    pub fn history(&self) -> &[ActionExecution] {
        &self.history
    }

    /// Get execution statistics
    pub fn stats(&self) -> &ActionExecutionStats {
        &self.stats
    }

    /// Clear the execution history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Clear the action queue
    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }

    /// Update execution statistics
    fn update_stats(&mut self, results: &[ActionExecution]) {
        for result in results {
            self.stats.total_executed += 1;
            if result.success {
                self.stats.successful += 1;
            } else {
                self.stats.failed += 1;
            }
            self.stats.total_time += result.duration();

            if result.memory_after > self.stats.peak_memory {
                self.stats.peak_memory = result.memory_after;
            }
        }

        if self.stats.total_executed > 0 {
            self.stats.average_time = self.stats.total_time / self.stats.total_executed as u32;
        }
    }
}

/// Execution result with detailed information
#[derive(Debug)]
pub struct ExecutionResult<C, E> {
    /// Successfully executed actions
    pub successful_actions: Vec<Box<dyn Action<C, E>>>,
    /// Failed actions with their execution results
    pub failed_actions: Vec<(Box<dyn Action<C, E>>, ActionExecution)>,
    /// Total execution time
    pub total_execution_time: std::time::Duration,
    /// Error handling strategy used
    pub error_strategy: ErrorHandlingStrategy,
}

impl<C, E> ExecutionResult<C, E> {
    /// Check if all actions executed successfully
    pub fn all_successful(&self) -> bool {
        self.failed_actions.is_empty()
    }

    /// Get success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        let total = self.successful_actions.len() + self.failed_actions.len();
        if total == 0 {
            0.0
        } else {
            self.successful_actions.len() as f64 / total as f64 * 100.0
        }
    }

    /// Get a summary of the execution
    pub fn summary(&self) -> String {
        format!(
            "Execution Summary: {} successful, {} failed, {:.1}% success rate, total time: {:?}",
            self.successful_actions.len(),
            self.failed_actions.len(),
            self.success_rate(),
            self.total_execution_time
        )
    }
}

/// Batch action executor for concurrent execution
pub struct BatchActionExecutor<C, E> {
    /// Executors for different contexts
    pub executors: Vec<EnhancedActionExecutor<C, E>>,
    /// Maximum concurrent executions
    pub max_concurrent: usize,
}

impl<C, E> BatchActionExecutor<C, E>
where
    C: Send + Sync + Clone + 'static,
    E: Send + Sync + Clone + 'static,
{
    /// Create a new batch executor
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            executors: Vec::new(),
            max_concurrent,
        }
    }

    /// Add an executor to the batch
    pub fn add_executor(&mut self, executor: EnhancedActionExecutor<C, E>) {
        self.executors.push(executor);
    }

    /// Execute all executors concurrently
    pub fn execute_batch(&self, contexts: &mut [C], events: &[E]) -> Vec<ExecutionResult<C, E>> {
        // For now, execute sequentially
        // In a real implementation, this would use parallel execution
        let mut results = Vec::new();

        for (i, executor) in self.executors.iter().enumerate() {
            if i < contexts.len() && i < events.len() {
                let mut executor_clone = EnhancedActionExecutor {
                    queue: executor.queue.clone(),
                    history: Vec::new(),
                    stats: ActionExecutionStats::default(),
                    error_strategy: executor.error_strategy.clone(),
                };

                let result =
                    executor_clone.execute_with_error_handling(&mut contexts[i], &events[i]);
                results.push(result);
            }
        }

        results
    }
}

/// Action scheduling and prioritization
pub struct ActionScheduler<C, E> {
    /// Action queue with priorities
    pub queue: std::collections::BinaryHeap<PrioritizedAction<C, E>>,
}

#[derive(Debug)]
pub struct PrioritizedAction<C, E> {
    /// Action priority (higher numbers = higher priority)
    pub priority: i32,
    /// The action to execute
    pub action: Box<dyn Action<C, E>>,
}

impl<C, E> PartialEq for PrioritizedAction<C, E> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<C, E> Eq for PrioritizedAction<C, E> {}

impl<C, E> std::hash::Hash for PrioritizedAction<C, E> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.priority.hash(state);
    }
}

impl<C, E> PartialOrd for PrioritizedAction<C, E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C, E> Ord for PrioritizedAction<C, E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<C, E> ActionScheduler<C, E> {
    /// Create a new action scheduler
    pub fn new() -> Self {
        Self {
            queue: std::collections::BinaryHeap::new(),
        }
    }

    /// Add an action with priority
    pub fn add_action(&mut self, action: Box<dyn Action<C, E>>, priority: i32) {
        self.queue.push(PrioritizedAction { priority, action });
    }

    /// Execute actions in priority order
    pub fn execute_in_order(&mut self, context: &mut C, event: &E) -> Vec<ActionExecution> {
        let mut results = Vec::new();

        while let Some(prioritized) = self.queue.pop() {
            let execution = ActionExecution::new(prioritized.action.description());
            prioritized.action.execute(context, event);
            results.push(execution.success());
        }

        results
    }
}
