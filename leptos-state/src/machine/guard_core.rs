//! Core guard trait and basic guard implementations

use super::*;

/// Trait for transition guards
pub trait GuardEvaluator<C, E>: Send + Sync {
    /// Check if the guard allows the transition
    fn check(&self, context: &C, event: &E) -> bool;

    /// Get a description of the guard for debugging
    fn description(&self) -> String {
        "Guard".to_string()
    }

    /// Clone this guard
    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>>;
}

/// Function-based guard implementation
pub struct FunctionGuard<C, E, F> {
    /// The guard function
    pub func: F,
    /// Description of the guard
    pub description: String,
}

impl<C, E, F> FunctionGuard<C, E, F>
where
    F: Fn(&C, &E) -> bool + 'static,
{
    /// Create a new function guard
    pub fn new(func: F) -> Self {
        Self {
            func,
            description: "Function Guard".to_string(),
        }
    }

    /// Create a new function guard with description
    pub fn with_description(func: F, description: String) -> Self {
        Self { func, description }
    }
}

impl<C: std::fmt::Debug + 'static, E: std::fmt::Debug + PartialEq + 'static, F> GuardEvaluator<C, E> for FunctionGuard<C, E, F>
where
    F: Fn(&C, &E) -> bool + Clone + 'static,
{
    fn check(&self, context: &C, event: &E) -> bool {
        (self.func)(context, event)
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            func: self.func.clone(),
            description: self.description.clone(),
        })
    }
}

/// Always true guard (allow all transitions)
pub struct AlwaysGuard;

impl AlwaysGuard {
    /// Create a new always guard
    pub fn new() -> Self {
        Self
    }
}

impl<C: std::fmt::Debug + 'static, E: std::fmt::Debug + PartialEq + 'static> GuardEvaluator<C, E> for AlwaysGuard {
    fn check(&self, _context: &C, _event: &E) -> bool {
        true
    }

    fn description(&self) -> String {
        "Always Allow".to_string()
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self)
    }
}

impl Clone for AlwaysGuard {
    fn clone(&self) -> Self {
        Self
    }
}

/// Never true guard (block all transitions)
pub struct NeverGuard;

impl NeverGuard {
    /// Create a new never guard
    pub fn new() -> Self {
        Self
    }
}

impl<C: std::fmt::Debug + 'static, E: std::fmt::Debug + PartialEq + 'static> GuardEvaluator<C, E> for NeverGuard {
    fn check(&self, _context: &C, _event: &E) -> bool {
        false
    }

    fn description(&self) -> String {
        "Never Allow".to_string()
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self)
    }
}

impl Clone for NeverGuard {
    fn clone(&self) -> Self {
        Self
    }
}

/// Extension trait for batch guard evaluation
pub trait GuardBatchEvaluator<C, E> {
    /// Evaluate all guards in the batch
    fn evaluate_batch(&self, context: &C, event: &E) -> Vec<(String, bool)>;

    /// Check if all guards in the batch pass
    fn check_all(&self, context: &C, event: &E) -> bool {
        self.evaluate_batch(context, event)
            .iter()
            .all(|(_, result)| *result)
    }

    /// Check if any guard in the batch passes
    fn check_any(&self, context: &C, event: &E) -> bool {
        self.evaluate_batch(context, event)
            .iter()
            .any(|(_, result)| *result)
    }
}

impl<C: std::fmt::Debug + 'static, E: std::fmt::Debug + PartialEq + 'static> GuardBatchEvaluator<C, E> for Vec<Box<dyn GuardEvaluator<C, E>>> {
    fn evaluate_batch(&self, context: &C, event: &E) -> Vec<(String, bool)> {
        self.iter()
            .map(|guard| (guard.description(), guard.check(context, event)))
            .collect()
    }
}
