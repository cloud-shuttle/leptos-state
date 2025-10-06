//! Core action trait and basic action implementations

use super::*;

/// Function-based action implementation
pub struct FunctionAction<C, E, F> {
    /// The function to execute
    pub func: F,
    /// Description of the action
    pub description: String,
    /// Phantom data for unused type parameters
    pub _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, F> std::fmt::Debug for FunctionAction<C, E, F>
where
    C: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionAction")
            .field("description", &self.description)
            .field("_phantom", &self._phantom)
            .finish()
    }
}

impl<C, E, F> FunctionAction<C, E, F>
where
    F: Fn(&mut C, &E) + 'static,
{
    /// Create a new function action
    pub fn new(func: F) -> Self {
        Self {
            func,
            description: "Function Action".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new function action with description
    pub fn with_description(func: F, description: String) -> Self {
        Self {
            func,
            description,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C: Send + Sync + std::fmt::Debug + 'static, E: Send + Sync + std::fmt::Debug + PartialEq + 'static, F> Action<C, E> for FunctionAction<C, E, F>
where
    F: Fn(&mut C, &E) + Send + Sync + 'static,
{
    fn execute(&self, context: &mut C, event: &E) {
        (self.func)(context, event);
    }

    fn name(&self) -> &str {
        "function"
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn clone_action(&self) -> Box<dyn Action<C, E>> {
        // Note: Cannot clone function types. Create a no-op action.
        // Function-based actions should not be cloned if possible.
        Box::new(crate::machine::actions::NoOpAction::new())
    }
}

/// Assign action that updates context fields
pub struct AssignAction<C, E, T, F> {
    /// The assignment function
    pub assign_fn: F,
    /// Description of the assignment
    pub description: String,
    /// Phantom data for type parameters
    _phantom: std::marker::PhantomData<(C, E, T)>,
}

impl<C, E, T, F> std::fmt::Debug for AssignAction<C, E, T, F>
where
    C: std::fmt::Debug,
    E: std::fmt::Debug,
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssignAction")
            .field("description", &self.description)
            .field("_phantom", &self._phantom)
            .finish()
    }
}

impl<C, E, T, F> AssignAction<C, E, T, F>
where
    F: Fn(&mut C, &E) -> T + 'static,
{
    /// Create a new assign action
    pub fn new(assign_fn: F) -> Self {
        Self {
            assign_fn,
            description: "Assign Action".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new assign action with description
    pub fn with_description(assign_fn: F, description: String) -> Self {
        Self {
            assign_fn,
            description,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C: Send + Sync + std::fmt::Debug + 'static, E: Send + Sync + std::fmt::Debug + PartialEq + 'static, T: Send + Sync + std::fmt::Debug + 'static, F> Action<C, E>
    for AssignAction<C, E, T, F>
where
    F: Fn(&mut C, &E) -> T + Send + Sync + 'static,
{
    fn execute(&self, context: &mut C, event: &E) {
        let _result = (self.assign_fn)(context, event);
        // In a real implementation, this might assign the result to a field
        // For now, we just execute the function
    }

    fn name(&self) -> &str {
        "assign"
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn clone_action(&self) -> Box<dyn Action<C, E>> {
        // Note: Cannot clone function types. Create a no-op action.
        Box::new(crate::machine::actions::NoOpAction::new())
    }
}

/// No-op action that does nothing
#[derive(Debug, Clone)]
pub struct NoOpAction;

impl NoOpAction {
    /// Create a new no-op action
    pub fn new() -> Self {
        Self
    }
}

impl<C, E> Action<C, E> for NoOpAction {
    fn execute(&self, _context: &mut C, _event: &E) {
        // Do nothing
    }

    fn name(&self) -> &str {
        "no_op"
    }

    fn description(&self) -> String {
        "No operation action".to_string()
    }

    fn has_side_effects(&self) -> bool {
        false
    }

    fn clone_action(&self) -> Box<dyn Action<C, E>> {
        Box::new(Self::new())
    }
}

/// Log action for debugging and monitoring
#[derive(Debug, Clone)]
pub struct LogAction {
    /// The log message template
    pub message: String,
    /// The log level
    pub level: LogLevel,
    /// Whether to include context information
    pub include_context: bool,
    /// Whether to include event information
    pub include_event: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    /// Debug level logging
    Debug,
    /// Info level logging
    Info,
    /// Warning level logging
    Warn,
    /// Error level logging
    Error,
}

impl LogAction {
    /// Create a new log action
    pub fn new(message: String) -> Self {
        Self {
            message,
            level: LogLevel::Info,
            include_context: false,
            include_event: false,
        }
    }

    /// Create a new log action with level
    pub fn with_level(message: String, level: LogLevel) -> Self {
        Self {
            message,
            level,
            include_context: false,
            include_event: false,
        }
    }

    /// Include context information in the log
    pub fn with_context(mut self) -> Self {
        self.include_context = true;
        self
    }

    /// Include event information in the log
    pub fn with_event(mut self) -> Self {
        self.include_event = true;
        self
    }
}

impl<C: std::fmt::Debug, E: std::fmt::Debug + PartialEq> Action<C, E> for LogAction
where
    C: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn execute(&self, context: &mut C, event: &E) {
        let mut message = self.message.clone();

        if self.include_context {
            message.push_str(&format!(" Context: {:?}", context));
        }

        if self.include_event {
            message.push_str(&format!(" Event: {:?}", event));
        }

        match self.level {
            LogLevel::Debug => println!("[DEBUG] {}", message),
            LogLevel::Info => println!("[INFO] {}", message),
            LogLevel::Warn => println!("[WARN] {}", message),
            LogLevel::Error => println!("[ERROR] {}", message),
        }
    }

    fn name(&self) -> &str {
        "log"
    }

    fn description(&self) -> String {
        format!("Log Action: {}", self.message)
    }

    fn has_side_effects(&self) -> bool {
        false // Logging is considered a side effect but doesn't modify state
    }

    fn clone_action(&self) -> Box<dyn Action<C, E>> {
        Box::new(self.clone())
    }
}


/// Pure action that doesn't modify context
pub struct PureAction<F> {
    /// The pure function to execute
    pub func: F,
    /// Description of the action
    pub description: String,
    /// Phantom data for unused type parameter
    _phantom: std::marker::PhantomData<F>,
}

impl<F> std::fmt::Debug for PureAction<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PureAction")
            .field("description", &self.description)
            .finish()
    }
}

impl<F> PureAction<F>
where
    F: Fn() + 'static,
{
    /// Create a new pure action
    pub fn new(func: F) -> Self {
        Self {
            func,
            description: "Pure Action".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new pure action with description
    pub fn with_description(func: F, description: String) -> Self {
        Self {
            func,
            description,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C: Send + Sync + std::fmt::Debug + 'static, E: Send + Sync + std::fmt::Debug + PartialEq + 'static, F> Action<C, E> for PureAction<F>
where
    F: Fn() + Send + Sync + 'static,
{
    fn execute(&self, _context: &mut C, _event: &E) {
        (self.func)();
    }

    fn name(&self) -> &str {
        "pure"
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn has_side_effects(&self) -> bool {
        false // Pure actions don't modify context
    }

    fn clone_action(&self) -> Box<dyn Action<C, E>> {
        // Note: Cannot clone function types. Create a no-op action.
        Box::new(crate::machine::actions::NoOpAction::new())
    }
}
