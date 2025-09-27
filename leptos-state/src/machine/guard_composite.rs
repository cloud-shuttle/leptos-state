//! Composite guard implementations

use super::*;

/// Composite guard that combines multiple guards with custom logic
pub struct CompositeGuard<C, E> {
    /// Child guards
    pub guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    /// Logic for combining guards
    pub logic: CompositeLogic,
    /// Description of the composite guard
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompositeLogic {
    /// All guards must pass
    All,
    /// Any guard must pass
    Any,
    /// None of the guards must pass
    None,
    /// Exactly one guard must pass
    ExactlyOne,
    /// At least N guards must pass
    AtLeast(usize),
    /// At most N guards must pass
    AtMost(usize),
    /// Majority of guards must pass
    Majority,
}

impl<C, E> CompositeGuard<C, E> {
    /// Create a new composite guard
    pub fn new(guards: Vec<Box<dyn GuardEvaluator<C, E>>>, logic: CompositeLogic) -> Self {
        Self {
            guards,
            logic,
            description: "Composite Guard".to_string(),
        }
    }

    /// Create an AND composite guard
    pub fn all(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self::new(guards, CompositeLogic::All)
    }

    /// Create an OR composite guard
    pub fn any(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self::new(guards, CompositeLogic::Any)
    }

    /// Create a NOT composite guard (none must pass)
    pub fn none(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self::new(guards, CompositeLogic::None)
    }

    /// Create an XOR composite guard (exactly one must pass)
    pub fn exactly_one(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self::new(guards, CompositeLogic::ExactlyOne)
    }

    /// Create an "at least N" composite guard
    pub fn at_least(guards: Vec<Box<dyn GuardEvaluator<C, E>>>, n: usize) -> Self {
        Self::new(guards, CompositeLogic::AtLeast(n))
    }

    /// Create an "at most N" composite guard
    pub fn at_most(guards: Vec<Box<dyn GuardEvaluator<C, E>>>, n: usize) -> Self {
        Self::new(guards, CompositeLogic::AtMost(n))
    }

    /// Create a majority composite guard
    pub fn majority(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self::new(guards, CompositeLogic::Majority)
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add a guard to the composite
    pub fn add_guard(&mut self, guard: Box<dyn GuardEvaluator<C, E>>) {
        self.guards.push(guard);
    }
}

impl<C, E> GuardEvaluator<C, E> for CompositeGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        let results: Vec<bool> = self.guards.iter()
            .map(|guard| guard.check(context, event))
            .collect();

        let passing_count = results.iter().filter(|&&r| r).count();

        match &self.logic {
            CompositeLogic::All => passing_count == self.guards.len(),
            CompositeLogic::Any => passing_count > 0,
            CompositeLogic::None => passing_count == 0,
            CompositeLogic::ExactlyOne => passing_count == 1,
            CompositeLogic::AtLeast(n) => passing_count >= *n,
            CompositeLogic::AtMost(n) => passing_count <= *n,
            CompositeLogic::Majority => passing_count > self.guards.len() / 2,
        }
    }

    fn description(&self) -> String {
        format!("{} ({:?} of {} guards)", self.description, self.logic, self.guards.len())
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            guards: self.guards.iter().map(|g| g.clone_guard()).collect(),
            logic: self.logic.clone(),
            description: self.description.clone(),
        })
    }
}

/// Weighted composite guard - guards have different weights
pub struct WeightedCompositeGuard<C, E> {
    /// Weighted guards (guard, weight)
    pub weighted_guards: Vec<(Box<dyn GuardEvaluator<C, E>>, f64)>,
    /// Minimum total weight required to pass
    pub min_weight: f64,
    /// Description of the weighted guard
    pub description: String,
}

impl<C, E> WeightedCompositeGuard<C, E> {
    /// Create a new weighted composite guard
    pub fn new(weighted_guards: Vec<(Box<dyn GuardEvaluator<C, E>>, f64)>, min_weight: f64) -> Self {
        Self {
            weighted_guards,
            min_weight,
            description: "Weighted Composite Guard".to_string(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add a weighted guard
    pub fn add_guard(&mut self, guard: Box<dyn GuardEvaluator<C, E>>, weight: f64) {
        self.weighted_guards.push((guard, weight));
    }
}

impl<C, E> GuardEvaluator<C, E> for WeightedCompositeGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        let total_weight: f64 = self.weighted_guards.iter()
            .map(|(guard, weight)| {
                if guard.check(context, event) {
                    *weight
                } else {
                    0.0
                }
            })
            .sum();

        total_weight >= self.min_weight
    }

    fn description(&self) -> String {
        format!("{} (min weight {:.1})", self.description, self.min_weight)
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            weighted_guards: self.weighted_guards.iter()
                .map(|(guard, weight)| (guard.clone_guard(), *weight))
                .collect(),
            min_weight: self.min_weight,
            description: self.description.clone(),
        })
    }
}

/// Sequential guard - evaluates guards in sequence, short-circuiting
pub struct SequentialGuard<C, E> {
    /// Guards to evaluate in sequence
    pub guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    /// Whether to require all guards to pass (AND) or stop at first pass (OR)
    pub require_all: bool,
    /// Description of the sequential guard
    pub description: String,
}

impl<C, E> SequentialGuard<C, E> {
    /// Create a new sequential guard that requires all guards to pass
    pub fn all(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self {
            guards,
            require_all: true,
            description: "Sequential AND Guard".to_string(),
        }
    }

    /// Create a new sequential guard that passes if any guard passes
    pub fn any(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self {
            guards,
            require_all: false,
            description: "Sequential OR Guard".to_string(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C, E> GuardEvaluator<C, E> for SequentialGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        if self.require_all {
            // All must pass
            self.guards.iter().all(|guard| guard.check(context, event))
        } else {
            // Any can pass (short-circuit)
            self.guards.iter().any(|guard| guard.check(context, event))
        }
    }

    fn description(&self) -> String {
        let logic_str = if self.require_all { "ALL" } else { "ANY" };
        format!("{} (sequential {})", self.description, logic_str)
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            guards: self.guards.iter().map(|g| g.clone_guard()).collect(),
            require_all: self.require_all,
            description: self.description.clone(),
        })
    }
}

/// Conditional composite guard - applies different logic based on conditions
pub struct ConditionalCompositeGuard<C, E, F> {
    /// Condition function
    pub condition: F,
    /// Guards to use if condition is true
    pub true_guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    /// Guards to use if condition is false
    pub false_guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    /// Logic to apply to the selected guard set
    pub logic: CompositeLogic,
    /// Description of the conditional guard
    pub description: String,
}

impl<C, E, F> ConditionalCompositeGuard<C, E, F>
where
    F: Fn(&C, &E) -> bool + 'static,
{
    /// Create a new conditional composite guard
    pub fn new(
        condition: F,
        true_guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
        false_guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
        logic: CompositeLogic,
    ) -> Self {
        Self {
            condition,
            true_guards,
            false_guards,
            logic,
            description: "Conditional Composite Guard".to_string(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C, E, F> GuardEvaluator<C, E> for ConditionalCompositeGuard<C, E, F>
where
    F: Fn(&C, &E) -> bool + Clone + 'static,
{
    fn check(&self, context: &C, event: &E) -> bool {
        let guards = if (self.condition)(context, event) {
            &self.true_guards
        } else {
            &self.false_guards
        };

        let results: Vec<bool> = guards.iter()
            .map(|guard| guard.check(context, event))
            .collect();

        let passing_count = results.iter().filter(|&&r| r).count();

        match &self.logic {
            CompositeLogic::All => passing_count == guards.len(),
            CompositeLogic::Any => passing_count > 0,
            CompositeLogic::None => passing_count == 0,
            CompositeLogic::ExactlyOne => passing_count == 1,
            CompositeLogic::AtLeast(n) => passing_count >= *n,
            CompositeLogic::AtMost(n) => passing_count <= *n,
            CompositeLogic::Majority => passing_count > guards.len() / 2,
        }
    }

    fn description(&self) -> String {
        format!("{} (conditional {:?})", self.description, self.logic)
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            condition: self.condition.clone(),
            true_guards: self.true_guards.iter().map(|g| g.clone_guard()).collect(),
            false_guards: self.false_guards.iter().map(|g| g.clone_guard()).collect(),
            logic: self.logic.clone(),
            description: self.description.clone(),
        })
    }
}
