//! Composite action implementations

use super::*;

/// Conditional action that only executes when a condition is met
pub struct ConditionalAction<C, E, F> {
    /// The condition function
    pub condition: F,
    /// The action to execute if condition is true
    pub action: Box<dyn Action<C, E>>,
    /// The action to execute if condition is false (optional)
    pub else_action: Option<Box<dyn Action<C, E>>>,
    /// Description of the conditional action
    pub description: String,
}

impl<C, E, F> ConditionalAction<C, E, F>
where
    F: Fn(&C, &E) -> bool + 'static,
{
    /// Create a new conditional action
    pub fn new(condition: F, action: Box<dyn Action<C, E>>) -> Self {
        Self {
            condition,
            action,
            else_action: None,
            description: "Conditional Action".to_string(),
        }
    }

    /// Add an else action
    pub fn with_else(mut self, else_action: Box<dyn Action<C, E>>) -> Self {
        self.else_action = Some(else_action);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C: Send + Sync + 'static, E: Send + Sync + 'static, F> Action<C, E> for ConditionalAction<C, E, F>
where
    F: Fn(&C, &E) -> bool + Clone + Send + Sync + 'static,
{
    fn execute(&self, context: &mut C, event: &E) {
        if (self.condition)(context, event) {
            self.action.execute(context, event);
        } else if let Some(ref else_action) = self.else_action {
            else_action.execute(context, event);
        }
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn clone_action(&self) -> Box<dyn Action<C, E>> {
        Box::new(Self {
            condition: self.condition.clone(),
            action: self.action.clone_action(),
            else_action: self.else_action.as_ref().map(|a| a.clone_action()),
            description: self.description.clone(),
        })
    }
}

/// Sequential action that executes multiple actions in order
pub struct SequentialAction<C, E> {
    /// The actions to execute in sequence
    pub actions: Vec<Box<dyn Action<C, E>>>,
    /// Whether to continue executing if one action fails
    pub continue_on_error: bool,
    /// Description of the sequential action
    pub description: String,
}

impl<C, E> SequentialAction<C, E> {
    /// Create a new sequential action
    pub fn new(actions: Vec<Box<dyn Action<C, E>>>) -> Self {
        Self {
            actions,
            continue_on_error: false,
            description: "Sequential Action".to_string(),
        }
    }

    /// Continue executing even if an action fails
    pub fn continue_on_error(mut self) -> Self {
        self.continue_on_error = true;
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add an action to the sequence
    pub fn add_action(&mut self, action: Box<dyn Action<C, E>>) {
        self.actions.push(action);
    }
}

impl<C: Send + Sync + 'static, E: Send + Sync + 'static> Action<C, E> for SequentialAction<C, E> {
    fn execute(&self, context: &mut C, event: &E) {
        for action in &self.actions {
            // In a real implementation, we might want to handle errors
            // For now, we just execute all actions
            action.execute(context, event);
        }
    }

    fn description(&self) -> String {
        format!("{} ({} actions)", self.description, self.actions.len())
    }

    fn clone_action(&self) -> Box<dyn Action<C, E>> {
        Box::new(Self {
            actions: self.actions.iter().map(|a| a.clone_action()).collect(),
            continue_on_error: self.continue_on_error,
            description: self.description.clone(),
        })
    }
}

/// Parallel action that could execute multiple actions concurrently (placeholder)
pub struct ParallelAction<C, E> {
    /// The actions to execute in parallel
    pub actions: Vec<Box<dyn Action<C, E>>>,
    /// Maximum number of concurrent executions
    pub max_concurrent: usize,
    /// Description of the parallel action
    pub description: String,
}

impl<C, E> ParallelAction<C, E> {
    /// Create a new parallel action
    pub fn new(actions: Vec<Box<dyn Action<C, E>>>) -> Self {
        Self {
            actions,
            max_concurrent: 4,
            description: "Parallel Action".to_string(),
        }
    }

    /// Set maximum concurrent executions
    pub fn with_max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C: Send + Sync + 'static, E: Send + Sync + 'static> Action<C, E> for ParallelAction<C, E>
where
    C: Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    fn execute(&self, context: &mut C, event: &E) {
        // For now, execute actions sequentially
        // In a real implementation, this would use async/parallel execution
        for action in &self.actions {
            action.execute(context, event);
        }
    }

    fn description(&self) -> String {
        format!("{} ({} parallel actions)", self.description, self.actions.len())
    }

    fn clone_action(&self) -> Box<dyn Action<C, E>> {
        Box::new(Self {
            actions: self.actions.iter().map(|a| a.clone_action()).collect(),
            max_concurrent: self.max_concurrent,
            description: self.description.clone(),
        })
    }
}

/// Composite action that combines multiple actions with custom logic
pub struct CompositeAction<C, E> {
    /// The actions to combine
    pub actions: Vec<Box<dyn Action<C, E>>>,
    /// The logic for combining actions
    pub logic: CompositeLogic,
    /// Description of the composite action
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompositeLogic {
    /// Execute all actions regardless of results
    All,
    /// Execute until one action succeeds
    UntilSuccess,
    /// Execute until one action fails
    UntilFailure,
    /// Execute a random action
    Random,
    /// Execute actions based on weights
    Weighted(Vec<f64>),
}

impl<C, E> CompositeAction<C, E> {
    /// Create a new composite action
    pub fn new(actions: Vec<Box<dyn Action<C, E>>>, logic: CompositeLogic) -> Self {
        Self {
            actions,
            logic,
            description: "Composite Action".to_string(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C: Send + Sync + 'static, E: Send + Sync + 'static> Action<C, E> for CompositeAction<C, E> {
    fn execute(&self, context: &mut C, event: &E) {
        match &self.logic {
            CompositeLogic::All => {
                for action in &self.actions {
                    action.execute(context, event);
                }
            }
            CompositeLogic::UntilSuccess => {
                // Execute until one action succeeds
                // For now, just execute all
                for action in &self.actions {
                    action.execute(context, event);
                }
            }
            CompositeLogic::UntilFailure => {
                // Execute until one action fails
                // For now, just execute all
                for action in &self.actions {
                    action.execute(context, event);
                }
            }
            CompositeLogic::Random => {
                // Execute a random action
                if !self.actions.is_empty() {
                    let index = (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as usize) % self.actions.len();
                    if let Some(action) = self.actions.get(index) {
                        action.execute(context, event);
                    }
                }
            }
            CompositeLogic::Weighted(weights) => {
                // Execute based on weights
                // This is a simplified implementation
                if !self.actions.is_empty() && !weights.is_empty() {
                    let total_weight: f64 = weights.iter().sum();
                    let mut random = (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as f64) % total_weight;

                    for (i, weight) in weights.iter().enumerate() {
                        random -= weight;
                        if random <= 0.0 {
                            if let Some(action) = self.actions.get(i) {
                                action.execute(context, event);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    fn description(&self) -> String {
        format!("{} ({:?})", self.description, self.logic)
    }

    fn clone_action(&self) -> Box<dyn Action<C, E>> {
        Box::new(Self {
            actions: self.actions.iter().map(|a| a.clone_action()).collect(),
            logic: self.logic.clone(),
            description: self.description.clone(),
        })
    }
}
