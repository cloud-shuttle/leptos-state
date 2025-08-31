//! Advanced action system for state machine effects
//! 
//! This module provides a comprehensive action system that allows side effects
//! during state transitions, including context updates, logging, async operations,
//! and complex action compositions.

use super::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::fmt;

/// Action trait for side effects during transitions
pub trait Action<C, E> {
    /// Execute the action, potentially modifying the context
    fn execute(&self, context: &mut C, event: &E);
    
    /// Get a description of what this action does
    fn description(&self) -> &str {
        "Unknown action"
    }
    
    /// Check if this action can be executed (optional validation)
    fn can_execute(&self, context: &C, event: &E) -> bool {
        true
    }
}

/// Function-based action implementation
pub struct FunctionAction<C, E, F> {
    func: F,
    description: String,
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, F> FunctionAction<C, E, F>
where
    F: Fn(&mut C, &E),
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            description: "Function action".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E, F> Action<C, E> for FunctionAction<C, E, F>
where
    F: Fn(&mut C, &E),
{
    fn execute(&self, context: &mut C, event: &E) {
        (self.func)(context, event);
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Assign action that updates context fields
pub struct AssignAction<C, E, T, F> {
    field_updater: F,
    description: String,
    _phantom: std::marker::PhantomData<(C, E, T)>,
}

impl<C, E, T, F> AssignAction<C, E, T, F>
where
    F: Fn(&mut C, &E, T),
{
    pub fn new(field_updater: F) -> Self {
        Self {
            field_updater,
            description: "Assign action".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E, T, F> Action<C, E> for AssignAction<C, E, T, F>
where
    F: Fn(&mut C, &E, T),
    T: Default,
{
    fn execute(&self, context: &mut C, event: &E) {
        (self.field_updater)(context, event, T::default());
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Log action for debugging and monitoring
pub struct LogAction {
    message: String,
    level: LogLevel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogAction {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: LogLevel::Info,
        }
    }
    
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }
    
    pub fn debug(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: LogLevel::Debug,
        }
    }
    
    pub fn warn(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: LogLevel::Warn,
        }
    }
    
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: LogLevel::Error,
        }
    }
}

impl<C, E> Action<C, E> for LogAction
where
    C: fmt::Debug,
    E: fmt::Debug,
{
    fn execute(&self, context: &mut C, event: &E) {
        match self.level {
            LogLevel::Debug => {
                tracing::debug!(
                    "{} - Context: {:?}, Event: {:?}",
                    self.message,
                    context,
                    event
                );
            }
            LogLevel::Info => {
                tracing::info!(
                    "{} - Context: {:?}, Event: {:?}",
                    self.message,
                    context,
                    event
                );
            }
            LogLevel::Warn => {
                tracing::warn!(
                    "{} - Context: {:?}, Event: {:?}",
                    self.message,
                    context,
                    event
                );
            }
            LogLevel::Error => {
                tracing::error!(
                    "{} - Context: {:?}, Event: {:?}",
                    self.message,
                    context,
                    event
                );
            }
        }
    }
    
    fn description(&self) -> &str {
        &self.message
    }
}

/// Pure action that doesn't modify context
pub struct PureAction<F> {
    func: F,
    description: String,
}

impl<F> PureAction<F>
where
    F: Fn(),
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            description: "Pure action".to_string(),
        }
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E, F> Action<C, E> for PureAction<F>
where
    F: Fn(),
{
    fn execute(&self, _context: &mut C, _event: &E) {
        (self.func)();
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Conditional action that only executes when a condition is met
pub struct ConditionalAction<C, E, F> {
    condition: F,
    action: Box<dyn Action<C, E>>,
    description: String,
}

impl<C, E, F> ConditionalAction<C, E, F>
where
    F: Fn(&C, &E) -> bool,
{
    pub fn new(condition: F, action: Box<dyn Action<C, E>>) -> Self {
        Self {
            condition,
            action,
            description: "Conditional action".to_string(),
        }
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E, F> Action<C, E> for ConditionalAction<C, E, F>
where
    F: Fn(&C, &E) -> bool,
{
    fn execute(&self, context: &mut C, event: &E) {
        if (self.condition)(context, event) {
            self.action.execute(context, event);
        }
    }
    
    fn can_execute(&self, context: &C, event: &E) -> bool {
        (self.condition)(context, event)
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Sequential action that executes multiple actions in order
pub struct SequentialAction<C, E> {
    actions: Vec<Box<dyn Action<C, E>>>,
    description: String,
}

impl<C, E> SequentialAction<C, E> {
    pub fn new(actions: Vec<Box<dyn Action<C, E>>>) -> Self {
        Self {
            actions,
            description: "Sequential action".to_string(),
        }
    }
    
    pub fn add_action(mut self, action: Box<dyn Action<C, E>>) -> Self {
        self.actions.push(action);
        self
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E> Action<C, E> for SequentialAction<C, E> {
    fn execute(&self, context: &mut C, event: &E) {
        for action in &self.actions {
            action.execute(context, event);
        }
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Parallel action that could execute multiple actions concurrently (placeholder)
pub struct ParallelAction<C, E> {
    actions: Vec<Box<dyn Action<C, E>>>,
    description: String,
}

impl<C, E> ParallelAction<C, E> {
    pub fn new(actions: Vec<Box<dyn Action<C, E>>>) -> Self {
        Self {
            actions,
            description: "Parallel action".to_string(),
        }
    }
    
    pub fn add_action(mut self, action: Box<dyn Action<C, E>>) -> Self {
        self.actions.push(action);
        self
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E> Action<C, E> for ParallelAction<C, E> {
    fn execute(&self, context: &mut C, event: &E) {
        // For now, execute sequentially. In a real async implementation,
        // this would spawn tasks and wait for completion
        for action in &self.actions {
            action.execute(context, event);
        }
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Retry action that attempts to execute an action multiple times
pub struct RetryAction<C, E> {
    action: Box<dyn Action<C, E>>,
    max_attempts: usize,
    backoff_duration: Duration,
    description: String,
}

impl<C, E> RetryAction<C, E> {
    pub fn new(action: Box<dyn Action<C, E>>, max_attempts: usize) -> Self {
        Self {
            action,
            max_attempts,
            backoff_duration: Duration::from_millis(100),
            description: "Retry action".to_string(),
        }
    }
    
    pub fn with_backoff(mut self, duration: Duration) -> Self {
        self.backoff_duration = duration;
        self
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E> Action<C, E> for RetryAction<C, E> {
    fn execute(&self, context: &mut C, event: &E) {
        for attempt in 1..=self.max_attempts {
            if self.action.can_execute(context, event) {
                self.action.execute(context, event);
                return;
            }
            
            if attempt < self.max_attempts {
                std::thread::sleep(self.backoff_duration);
            }
        }
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Timer action that tracks execution time
pub struct TimerAction<C, E> {
    action: Box<dyn Action<C, E>>,
    description: String,
}

impl<C, E> TimerAction<C, E> {
    pub fn new(action: Box<dyn Action<C, E>>) -> Self {
        Self {
            action,
            description: "Timer action".to_string(),
        }
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E> Action<C, E> for TimerAction<C, E> {
    fn execute(&self, context: &mut C, event: &E) {
        let start = Instant::now();
        self.action.execute(context, event);
        let duration = start.elapsed();
        
        tracing::debug!(
            "Action '{}' executed in {:?}",
            self.action.description(),
            duration
        );
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Metrics action that records execution metrics
pub struct MetricsAction<C, E> {
    action: Box<dyn Action<C, E>>,
    metrics: Arc<std::sync::Mutex<HashMap<String, usize>>>,
    metric_name: String,
}

impl<C, E> MetricsAction<C, E> {
    pub fn new(action: Box<dyn Action<C, E>>, metric_name: impl Into<String>) -> Self {
        Self {
            action,
            metrics: Arc::new(std::sync::Mutex::new(HashMap::new())),
            metric_name: metric_name.into(),
        }
    }
    
    pub fn get_metrics(&self) -> HashMap<String, usize> {
        self.metrics.lock().unwrap().clone()
    }
}

impl<C, E> Action<C, E> for MetricsAction<C, E> {
    fn execute(&self, context: &mut C, event: &E) {
        self.action.execute(context, event);
        
        if let Ok(mut metrics) = self.metrics.lock() {
            *metrics.entry(self.metric_name.clone()).or_insert(0) += 1;
        }
    }
    
    fn description(&self) -> &str {
        self.action.description()
    }
}

/// Composite action that combines multiple actions with custom logic
pub struct CompositeAction<C, E> {
    actions: Vec<Box<dyn Action<C, E>>>,
    logic: CompositeLogic,
    description: String,
}

#[derive(Debug, Clone, Copy)]
pub enum CompositeLogic {
    All,        // Execute all actions
    First,      // Execute first successful action
    Any,        // Execute any action that can execute
    Conditional(usize), // Execute if at least N actions can execute
}

impl<C, E> CompositeAction<C, E> {
    pub fn new(logic: CompositeLogic) -> Self {
        Self {
            actions: Vec::new(),
            logic,
            description: "Composite action".to_string(),
        }
    }
    
    pub fn add_action(mut self, action: Box<dyn Action<C, E>>) -> Self {
        self.actions.push(action);
        self
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl<C, E> Action<C, E> for CompositeAction<C, E> {
    fn execute(&self, context: &mut C, event: &E) {
        match self.logic {
            CompositeLogic::All => {
                for action in &self.actions {
                    action.execute(context, event);
                }
            }
            CompositeLogic::First => {
                for action in &self.actions {
                    if action.can_execute(context, event) {
                        action.execute(context, event);
                        break;
                    }
                }
            }
            CompositeLogic::Any => {
                for action in &self.actions {
                    if action.can_execute(context, event) {
                        action.execute(context, event);
                    }
                }
            }
            CompositeLogic::Conditional(n) => {
                let executable_count = self.actions.iter()
                    .filter(|action| action.can_execute(context, event))
                    .count();
                
                if executable_count >= n {
                    for action in &self.actions {
                        if action.can_execute(context, event) {
                            action.execute(context, event);
                        }
                    }
                }
            }
        }
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

/// Builder for complex action combinations
pub struct ActionBuilder<C, E> {
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C: 'static + std::fmt::Debug + Clone, E: 'static + std::fmt::Debug + Clone> ActionBuilder<C, E> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn function<F>(func: F) -> Box<dyn Action<C, E>>
    where
        F: Fn(&mut C, &E) + 'static,
    {
        Box::new(FunctionAction::new(func))
    }
    
    pub fn function_with_description<F>(func: F, description: impl Into<String>) -> Box<dyn Action<C, E>>
    where
        F: Fn(&mut C, &E) + 'static,
    {
        Box::new(FunctionAction::new(func).with_description(description))
    }
    
    pub fn assign<T, F>(field_updater: F) -> Box<dyn Action<C, E>>
    where
        F: Fn(&mut C, &E, T) + 'static,
        T: Default + 'static,
    {
        Box::new(AssignAction::new(field_updater))
    }
    
    pub fn log(message: impl Into<String>) -> Box<dyn Action<C, E>> {
        Box::new(LogAction::new(message))
    }
    
    pub fn log_debug(message: impl Into<String>) -> Box<dyn Action<C, E>> {
        Box::new(LogAction::debug(message))
    }
    
    pub fn log_warn(message: impl Into<String>) -> Box<dyn Action<C, E>> {
        Box::new(LogAction::warn(message))
    }
    
    pub fn log_error(message: impl Into<String>) -> Box<dyn Action<C, E>> {
        Box::new(LogAction::error(message))
    }
    
    pub fn pure<F>(func: F) -> Box<dyn Action<C, E>>
    where
        F: Fn() + 'static,
    {
        Box::new(PureAction::new(func))
    }
    
    pub fn conditional<F>(condition: F, action: Box<dyn Action<C, E>>) -> Box<dyn Action<C, E>>
    where
        F: Fn(&C, &E) -> bool + 'static,
    {
        Box::new(ConditionalAction::new(condition, action))
    }
    
    pub fn sequential(actions: Vec<Box<dyn Action<C, E>>>) -> Box<dyn Action<C, E>> {
        Box::new(SequentialAction::new(actions))
    }
    
    pub fn parallel(actions: Vec<Box<dyn Action<C, E>>>) -> Box<dyn Action<C, E>> {
        Box::new(ParallelAction::new(actions))
    }
    
    pub fn retry(action: Box<dyn Action<C, E>>, max_attempts: usize) -> Box<dyn Action<C, E>> {
        Box::new(RetryAction::new(action, max_attempts))
    }
    
    pub fn timer(action: Box<dyn Action<C, E>>) -> Box<dyn Action<C, E>> {
        Box::new(TimerAction::new(action))
    }
    
    pub fn metrics(action: Box<dyn Action<C, E>>, metric_name: impl Into<String>) -> Box<dyn Action<C, E>> {
        Box::new(MetricsAction::new(action, metric_name))
    }
    
    pub fn composite(logic: CompositeLogic) -> CompositeAction<C, E> {
        CompositeAction::new(logic)
    }
}

impl<C: 'static + std::fmt::Debug + Clone, E: 'static + std::fmt::Debug + Clone> Default for ActionBuilder<C, E> {
    fn default() -> Self {
        Self::new()
    }
}

/// Action execution result with detailed information
#[derive(Debug, Clone)]
pub struct ActionExecution {
    pub executed: bool,
    pub action_descriptions: Vec<String>,
    pub execution_time: Option<Duration>,
    pub errors: Vec<String>,
}

impl ActionExecution {
    pub fn new() -> Self {
        Self {
            executed: true,
            action_descriptions: Vec::new(),
            execution_time: None,
            errors: Vec::new(),
        }
    }
    
    pub fn add_action(&mut self, description: String, executed: bool) {
        self.action_descriptions.push(description);
        if !executed {
            self.executed = false;
        }
    }
    
    pub fn set_execution_time(&mut self, duration: Duration) {
        self.execution_time = Some(duration);
    }
    
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.executed = false;
    }
}

/// Extension trait for executing actions with detailed results
pub trait ActionExecutor<C, E> {
    fn execute_actions(&self, context: &mut C, event: &E) -> ActionExecution;
}

impl<C, E> ActionExecutor<C, E> for Vec<Box<dyn Action<C, E>>> {
    fn execute_actions(&self, context: &mut C, event: &E) -> ActionExecution {
        let mut execution = ActionExecution::new();
        let start = Instant::now();
        
        for action in self {
            let can_execute = action.can_execute(context, event);
            execution.add_action(action.description().to_string(), can_execute);
            
            if can_execute {
                action.execute(context, event);
            }
        }
        
        execution.set_execution_time(start.elapsed());
        execution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestContext {
        count: i32,
        flag: bool,
        name: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Increment,
        Decrement,
        Toggle,
        SetName(String),
    }

    #[test]
    fn function_action_works() {
        let action = FunctionAction::new(|ctx: &mut TestContext, _| {
            ctx.count += 1;
        }).with_description("Increment count");
        
        let mut context = TestContext { count: 0, flag: false, name: "test".to_string() };
        let event = TestEvent::Increment;
        
        action.execute(&mut context, &event);
        assert_eq!(context.count, 1);
        assert_eq!(action.description(), "Increment count");
    }

    #[test]
    fn log_action_works() {
        let action = LogAction::new("Test log message");
        let mut context = TestContext { count: 0, flag: false, name: "test".to_string() };
        let event = TestEvent::Increment;
        
        // Should not panic
        action.execute(&mut context, &event);
    }

    #[test]
    fn conditional_action_works() {
        let inner_action = FunctionAction::new(|ctx: &mut TestContext, _| {
            ctx.flag = true;
        });
        
        let action = ConditionalAction::new(
            |ctx: &TestContext, _| ctx.count > 0,
            Box::new(inner_action)
        );
        
        let mut context = TestContext { count: 0, flag: false, name: "test".to_string() };
        let event = TestEvent::Increment;
        
        // Should not execute when count is 0
        action.execute(&mut context, &event);
        assert_eq!(context.flag, false);
        
        // Should execute when count is > 0
        context.count = 5;
        action.execute(&mut context, &event);
        assert_eq!(context.flag, true);
    }

    #[test]
    fn sequential_action_works() {
        let action1 = FunctionAction::new(|ctx: &mut TestContext, _| {
            ctx.count += 1;
        });
        
        let action2 = FunctionAction::new(|ctx: &mut TestContext, _| {
            ctx.flag = true;
        });
        
        let sequential = SequentialAction::new(vec![
            Box::new(action1),
            Box::new(action2),
        ]);
        
        let mut context = TestContext { count: 0, flag: false, name: "test".to_string() };
        let event = TestEvent::Increment;
        
        sequential.execute(&mut context, &event);
        assert_eq!(context.count, 1);
        assert_eq!(context.flag, true);
    }

    #[test]
    fn timer_action_works() {
        let inner_action = FunctionAction::new(|ctx: &mut TestContext, _| {
            ctx.count += 1;
        });
        
        let timer_action = TimerAction::new(Box::new(inner_action));
        let mut context = TestContext { count: 0, flag: false, name: "test".to_string() };
        let event = TestEvent::Increment;
        
        timer_action.execute(&mut context, &event);
        assert_eq!(context.count, 1);
    }

    #[test]
    fn metrics_action_works() {
        let inner_action = FunctionAction::new(|ctx: &mut TestContext, _| {
            ctx.count += 1;
        });
        
        let metrics_action = MetricsAction::new(Box::new(inner_action), "test_metric");
        let mut context = TestContext { count: 0, flag: false, name: "test".to_string() };
        let event = TestEvent::Increment;
        
        metrics_action.execute(&mut context, &event);
        assert_eq!(context.count, 1);
        
        let metrics = metrics_action.get_metrics();
        assert_eq!(metrics.get("test_metric"), Some(&1));
    }

    #[test]
    fn composite_action_works() {
        let action1 = FunctionAction::new(|ctx: &mut TestContext, _| {
            ctx.count += 1;
        });
        
        let action2 = FunctionAction::new(|ctx: &mut TestContext, _| {
            ctx.flag = true;
        });
        
        let composite = CompositeAction::new(CompositeLogic::All)
            .add_action(Box::new(action1))
            .add_action(Box::new(action2));
        
        let mut context = TestContext { count: 0, flag: false, name: "test".to_string() };
        let event = TestEvent::Increment;
        
        composite.execute(&mut context, &event);
        assert_eq!(context.count, 1);
        assert_eq!(context.flag, true);
    }

    #[test]
    fn action_builder_creates_actions() {
        let _function = ActionBuilder::<TestContext, TestEvent>::function(|ctx, _| {
            ctx.count += 1;
        });
        
        let _log: Box<dyn Action<TestContext, TestEvent>> = ActionBuilder::log("Test message");
        let _pure: Box<dyn Action<TestContext, TestEvent>> = ActionBuilder::pure(|| println!("Pure action"));
        let _timer = ActionBuilder::timer(Box::new(FunctionAction::new(|ctx: &mut TestContext, _: &TestEvent| {
            ctx.count += 1;
        })));
    }

    #[test]
    fn action_execution_works() {
        let actions: Vec<Box<dyn Action<TestContext, TestEvent>>> = vec![
            Box::new(FunctionAction::new(|ctx: &mut TestContext, _: &TestEvent| ctx.count += 1)),
            Box::new(FunctionAction::new(|ctx: &mut TestContext, _| ctx.flag = true)),
        ];
        
        let mut context = TestContext { count: 0, flag: false, name: "test".to_string() };
        let event = TestEvent::Increment;
        
        let execution = actions.execute_actions(&mut context, &event);
        
        assert!(execution.executed);
        assert_eq!(context.count, 1);
        assert_eq!(context.flag, true);
        assert!(execution.execution_time.is_some());
    }
}
