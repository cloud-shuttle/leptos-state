//! State and event-based guard implementations

use super::*;

/// Event type guard - checks the event type
pub struct EventTypeGuard<C, E> {
    /// Expected event type (as string representation)
    pub expected_type: String,
    /// Description of the guard
    pub description: String,
}

impl<C, E> EventTypeGuard<C, E> {
    /// Create a new event type guard
    pub fn new(expected_type: String) -> Self {
        Self {
            expected_type: expected_type.clone(),
            description: format!("Event Type: {}", expected_type),
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C, E> GuardEvaluator<C, E> for EventTypeGuard<C, E>
where
    E: std::fmt::Debug,
{
    fn check(&self, _context: &C, event: &E) -> bool {
        format!("{:?}", event).contains(&self.expected_type)
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            expected_type: self.expected_type.clone(),
            description: self.description.clone(),
        })
    }
}

/// State-based guard - checks if we're in a specific state
pub struct StateGuard<C, E> {
    /// Expected state name
    pub expected_state: String,
    /// Current state getter function
    pub state_getter: Box<dyn Fn(&C) -> String + Send + Sync>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> StateGuard<C, E> {
    /// Create a new state guard
    pub fn new<F>(expected_state: String, state_getter: F) -> Self
    where
        F: Fn(&C) -> String + Send + Sync + 'static,
    {
        Self {
            expected_state: expected_state.clone(),
            state_getter: Box::new(state_getter),
            description: format!("State: {}", expected_state),
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C, E> GuardEvaluator<C, E> for StateGuard<C, E> {
    fn check(&self, context: &C, _event: &E) -> bool {
        let current_state = (self.state_getter)(context);
        current_state == self.expected_state
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            expected_state: self.expected_state.clone(),
            state_getter: self.state_getter.clone(),
            description: self.description.clone(),
        })
    }
}

/// State transition guard - checks if we're transitioning from/to specific states
pub struct StateTransitionGuard<C, E> {
    /// Required source state (None means any)
    pub from_state: Option<String>,
    /// Required target state (None means any)
    pub to_state: Option<String>,
    /// Current state getter function
    pub current_state_getter: Box<dyn Fn(&C) -> String + Send + Sync>,
    /// Target state getter function (from event or context)
    pub target_state_getter: Box<dyn Fn(&C, &E) -> String + Send + Sync>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> StateTransitionGuard<C, E> {
    /// Create a new state transition guard
    pub fn new<F1, F2>(current_state_getter: F1, target_state_getter: F2) -> Self
    where
        F1: Fn(&C) -> String + Send + Sync + 'static,
        F2: Fn(&C, &E) -> String + Send + Sync + 'static,
    {
        Self {
            from_state: None,
            to_state: None,
            current_state_getter: Box::new(current_state_getter),
            target_state_getter: Box::new(target_state_getter),
            description: "State Transition Guard".to_string(),
        }
    }

    /// Require a specific source state
    pub fn from(mut self, state: String) -> Self {
        self.from_state = Some(state);
        self
    }

    /// Require a specific target state
    pub fn to(mut self, state: String) -> Self {
        self.to_state = Some(state);
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C, E> GuardEvaluator<C, E> for StateTransitionGuard<C, E> {
    fn check(&self, context: &C, event: &E) -> bool {
        let current_state = (self.current_state_getter)(context);
        let target_state = (self.target_state_getter)(context, event);

        let from_ok = self.from_state.as_ref()
            .map_or(true, |expected| current_state == *expected);

        let to_ok = self.to_state.as_ref()
            .map_or(true, |expected| target_state == *expected);

        from_ok && to_ok
    }

    fn description(&self) -> String {
        let from_str = self.from_state.as_ref()
            .map_or("*".to_string(), |s| s.clone());
        let to_str = self.to_state.as_ref()
            .map_or("*".to_string(), |s| s.clone());

        format!("{} ({} -> {})", self.description, from_str, to_str)
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            from_state: self.from_state.clone(),
            to_state: self.to_state.clone(),
            current_state_getter: self.current_state_getter.clone(),
            target_state_getter: self.target_state_getter.clone(),
            description: self.description.clone(),
        })
    }
}

/// Event data guard - checks event content
pub struct EventDataGuard<C, E, F> {
    /// Event data extractor function
    pub data_extractor: F,
    /// Expected data value
    pub expected_data: String,
    /// Description of the guard
    pub description: String,
    /// Phantom data for type parameters
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, F> EventDataGuard<C, E, F>
where
    F: Fn(&E) -> String + 'static,
{
    /// Create a new event data guard
    pub fn new(data_extractor: F, expected_data: String) -> Self {
        Self {
            data_extractor,
            expected_data: expected_data.clone(),
            description: format!("Event Data: {}", expected_data),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C, E, F> GuardEvaluator<C, E> for EventDataGuard<C, E, F>
where
    F: Fn(&E) -> String + Clone + 'static,
{
    fn check(&self, _context: &C, event: &E) -> bool {
        let event_data = (self.data_extractor)(event);
        event_data == self.expected_data
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            data_extractor: self.data_extractor.clone(),
            expected_data: self.expected_data.clone(),
            description: self.description.clone(),
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Context state guard - checks context and state combination
pub struct ContextStateGuard<C, E, F> {
    /// Context and state checker function
    pub checker: F,
    /// Description of the guard
    pub description: String,
    /// Phantom data for type parameters
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, F> ContextStateGuard<C, E, F>
where
    F: Fn(&C, &E) -> bool + 'static,
{
    /// Create a new context state guard
    pub fn new(checker: F) -> Self {
        Self {
            checker,
            description: "Context State Guard".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C, E, F> GuardEvaluator<C, E> for ContextStateGuard<C, E, F>
where
    F: Fn(&C, &E) -> bool + Clone + 'static,
{
    fn check(&self, context: &C, event: &E) -> bool {
        (self.checker)(context, event)
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            checker: self.checker.clone(),
            description: self.description.clone(),
            _phantom: std::marker::PhantomData,
        })
    }
}
