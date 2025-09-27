//! Logical guard implementations

use super::*;

/// And guard - all child guards must pass
pub struct AndGuard<C, E> {
    /// Child guards
    pub guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> AndGuard<C, E> {
    /// Create a new AND guard
    pub fn new(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self {
            guards,
            description: "AND Guard".to_string(),
        }
    }

    /// Create a new AND guard with description
    pub fn with_description(guards: Vec<Box<dyn GuardEvaluator<C, E>>>, description: String) -> Self {
        Self { guards, description }
    }

    /// Add a guard to the AND condition
    pub fn add_guard(&mut self, guard: Box<dyn GuardEvaluator<C, E>>) {
        self.guards.push(guard);
    }
}

impl<C, E> GuardEvaluator<C, E> for AndGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        self.guards.iter().all(|guard| guard.check(context, event))
    }

    fn description(&self) -> String {
        format!("{} ({} conditions)", self.description, self.guards.len())
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            guards: self.guards.iter().map(|g| g.clone_guard()).collect(),
            description: self.description.clone(),
        })
    }
}

/// Or guard - any child guard must pass
pub struct OrGuard<C, E> {
    /// Child guards
    pub guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> OrGuard<C, E> {
    /// Create a new OR guard
    pub fn new(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self {
            guards,
            description: "OR Guard".to_string(),
        }
    }

    /// Create a new OR guard with description
    pub fn with_description(guards: Vec<Box<dyn GuardEvaluator<C, E>>>, description: String) -> Self {
        Self { guards, description }
    }

    /// Add a guard to the OR condition
    pub fn add_guard(&mut self, guard: Box<dyn GuardEvaluator<C, E>>) {
        self.guards.push(guard);
    }
}

impl<C, E> GuardEvaluator<C, E> for OrGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        self.guards.iter().any(|guard| guard.check(context, event))
    }

    fn description(&self) -> String {
        format!("{} ({} conditions)", self.description, self.guards.len())
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            guards: self.guards.iter().map(|g| g.clone_guard()).collect(),
            description: self.description.clone(),
        })
    }
}

/// Not guard - inverts the result of the child guard
pub struct NotGuard<C, E> {
    /// Child guard to invert
    pub guard: Box<dyn GuardEvaluator<C, E>>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> NotGuard<C, E> {
    /// Create a new NOT guard
    pub fn new(guard: Box<dyn GuardEvaluator<C, E>>) -> Self {
        Self {
            guard,
            description: "NOT Guard".to_string(),
        }
    }

    /// Create a new NOT guard with description
    pub fn with_description(guard: Box<dyn GuardEvaluator<C, E>>, description: String) -> Self {
        Self { guard, description }
    }
}

impl<C, E> GuardEvaluator<C, E> for NotGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        !self.guard.check(context, event)
    }

    fn description(&self) -> String {
        format!("{} (NOT {})", self.description, self.guard.description())
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            guard: self.guard.clone_guard(),
            description: self.description.clone(),
        })
    }
}

/// XOR guard - exactly one child guard must pass
pub struct XorGuard<C, E> {
    /// Child guards
    pub guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> XorGuard<C, E> {
    /// Create a new XOR guard
    pub fn new(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self {
            guards,
            description: "XOR Guard".to_string(),
        }
    }

    /// Create a new XOR guard with description
    pub fn with_description(guards: Vec<Box<dyn GuardEvaluator<C, E>>>, description: String) -> Self {
        Self { guards, description }
    }

    /// Add a guard to the XOR condition
    pub fn add_guard(&mut self, guard: Box<dyn GuardEvaluator<C, E>>) {
        self.guards.push(guard);
    }
}

impl<C, E> GuardEvaluator<C, E> for XorGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        self.guards.iter().filter(|guard| guard.check(context, event)).count() == 1
    }

    fn description(&self) -> String {
        format!("{} (exactly one of {} conditions)", self.description, self.guards.len())
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            guards: self.guards.iter().map(|g| g.clone_guard()).collect(),
            description: self.description.clone(),
        })
    }
}

/// Majority guard - majority of child guards must pass
pub struct MajorityGuard<C, E> {
    /// Child guards
    pub guards: Vec<Box<dyn GuardEvaluator<C, E>>>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> MajorityGuard<C, E> {
    /// Create a new majority guard
    pub fn new(guards: Vec<Box<dyn GuardEvaluator<C, E>>>) -> Self {
        Self {
            guards,
            description: "Majority Guard".to_string(),
        }
    }

    /// Create a new majority guard with description
    pub fn with_description(guards: Vec<Box<dyn GuardEvaluator<C, E>>>, description: String) -> Self {
        Self { guards, description }
    }

    /// Add a guard to the majority condition
    pub fn add_guard(&mut self, guard: Box<dyn GuardEvaluator<C, E>>) {
        self.guards.push(guard);
    }
}

impl<C, E> GuardEvaluator<C, E> for MajorityGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        let passing_count = self.guards.iter().filter(|guard| guard.check(context, event)).count();
        passing_count > self.guards.len() / 2
    }

    fn description(&self) -> String {
        format!("{} (majority of {} conditions)", self.description, self.guards.len())
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            guards: self.guards.iter().map(|g| g.clone_guard()).collect(),
            description: self.description.clone(),
        })
    }
}
