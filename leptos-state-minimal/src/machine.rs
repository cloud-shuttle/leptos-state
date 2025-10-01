//! State machine implementation

use crate::{State, Event, MachineError, MachineResult};
use std::collections::HashMap;

/// Result of executing an action during a transition
#[derive(Clone, Debug, PartialEq)]
pub enum ActionResult {
    /// Continue with the transition normally
    Continue,
    /// Cancel the transition and stay in current state
    Cancel,
    /// Redirect to a different state instead
    Redirect(String),
    /// Error condition that prevents the transition
    Error(String),
}

/// A state machine that manages state transitions
///
/// The machine is generic over state and event types, requiring only
/// that they live as long as the application.
pub struct Machine<S: State, E: Event> {
    states: HashMap<String, StateNode<S, E>>,
    current_state: String,
    context: S,
    middlewares: crate::middleware::MiddlewareStack<S, E>,
    #[cfg(all(feature = "web", feature = "devtools"))]
    devtools: Option<crate::devtools::DevToolsIntegration>,
}

impl<S: State, E: Event> Machine<S, E> {
    /// Create a new state machine
    pub fn new(initial_state: &str, context: S) -> Self {
        Self {
            states: HashMap::new(),
            current_state: initial_state.to_string(),
            context,
            middlewares: crate::middleware::MiddlewareStack::new(),
            #[cfg(all(feature = "web", feature = "devtools"))]
            devtools: None,
        }
    }

    /// Add a state to the machine
    pub fn add_state(&mut self, name: &str, state: StateNode<S, E>) {
        self.states.insert(name.to_string(), state);
    }

    /// Send an event to the machine
    ///
    /// This will attempt to transition to a new state based on the current
    /// state and the event. Returns an error if the transition is invalid.
    pub fn send(&mut self, event: E) -> MachineResult<()> {
        let current_state_name = self.current_state.clone();

        // Find the transition first (without borrowing the state)
        let target_state = if let Some(current_state) = self.states.get(&current_state_name) {
            if let Some(transition) = current_state.transitions.get(event.event_type()) {
                Some(transition.target.clone())
            } else {
                None
            }
        } else {
            return Err(MachineError::InvalidState { state: current_state_name });
        };

        let target_state = target_state.ok_or_else(|| MachineError::InvalidTransition {
            from: current_state_name.clone(),
            to: format!("no transition for event {:?}", event.event_type()),
        })?;

        // Execute exit actions
        if let Some(current_state) = self.states.get(&current_state_name) {
            if let Some(exit_actions) = &current_state.exit_actions {
                for action in exit_actions {
                    // For backward compatibility, ignore the ActionResult from old actions
                    let _ = action(&mut self.context, &event);
                }
            }
        }

        // Execute transition actions
        if let Some(current_state) = self.states.get(&current_state_name) {
            if let Some(transition) = current_state.transitions.get(event.event_type()) {
                if let Some(transition_actions) = &transition.actions {
                    for action in transition_actions {
                        // For backward compatibility, ignore the ActionResult from old actions
                        let _ = action(&mut self.context, &event);
                    }
                }
            }
        }

        // Update to new state
        self.current_state = target_state.clone();

        // Execute entry actions for new state
        if let Some(new_state) = self.states.get(&target_state) {
            if let Some(entry_actions) = &new_state.entry_actions {
                for action in entry_actions {
                    // For backward compatibility, ignore the ActionResult from old actions
                    let _ = action(&mut self.context, &event);
                }
            }
        }

        Ok(())
    }

    /// Send an event with enhanced action handling
    ///
    /// This method supports ActionResult-returning actions that can:
    /// - Continue: Normal transition
    /// - Cancel: Abort the transition
    /// - Redirect: Redirect to a different state
    /// - Error: Error condition
    pub fn send_with_actions(&mut self, event: E) -> MachineResult<()> {
        let current_state_name = self.current_state.clone();

        // Get current state node
        let current_node = self.states.get(&current_state_name)
            .ok_or_else(|| MachineError::InvalidState {
                state: current_state_name.clone()
            })?;

        // Find transition
        if let Some(transition) = current_node.transitions.get(event.event_type()) {
            let target_state = transition.target.clone();

            // Execute exit actions
            if let Some(current_state) = self.states.get(&current_state_name) {
                if let Some(exit_actions) = &current_state.exit_actions {
                    for action in exit_actions {
                        let _ = action(&mut self.context, &event);
                    }
                }
            }

            // Execute transition actions
            if let Some(actions) = &transition.actions {
                for action in actions {
                    match action(&mut self.context, &event) {
                        ActionResult::Continue => continue,
                        ActionResult::Cancel => {
                            return Err(MachineError::ActionCancelled {
                                state: current_state_name,
                                event: event.event_type().to_string(),
                            });
                        }
                        ActionResult::Redirect(new_target) => {
                            if let Some(target_node) = self.states.get(&new_target) {
                                self.current_state = new_target.clone();
                                // Execute entry actions for redirected state
                                if let Some(entry_actions) = &target_node.entry_actions {
                                    for action in entry_actions {
                                        let _ = action(&mut self.context, &event);
                                    }
                                }
                                return Ok(());
                            } else {
                                return Err(MachineError::InvalidRedirect {
                                    from: current_state_name,
                                    to: new_target,
                                });
                            }
                        }
                        ActionResult::Error(message) => {
                            return Err(MachineError::ActionError {
                                state: current_state_name,
                                event: event.event_type().to_string(),
                                message,
                            });
                        }
                    }
                }
            }

            // Update current state
            self.current_state = target_state.clone();

            // Execute entry actions for target state
            if let Some(new_state) = self.states.get(&target_state) {
                if let Some(entry_actions) = &new_state.entry_actions {
                    for action in entry_actions {
                        let _ = action(&mut self.context, &event);
                    }
                }
            }

            Ok(())
        } else {
            Err(MachineError::InvalidTransition {
                from: current_state_name,
                to: event.event_type().to_string(),
            })
        }
    }

    /// Send an event with guard condition checking
    ///
    /// This method checks guard conditions before allowing transitions.
    /// Returns an error if the guard fails.
    pub fn send_guarded(&mut self, event: E) -> MachineResult<()> {
        if !self.can_transition(&event) {
            return Err(MachineError::GuardFailed {
                state: self.current_state.clone(),
                event: event.event_type().to_string(),
            });
        }

        self.send(event)
    }


    /// Get the current state name
    pub fn current_state(&self) -> &str {
        &self.current_state
    }

    /// Get a reference to the context
    pub fn context(&self) -> &S {
        &self.context
    }

    /// Get a mutable reference to the context
    pub fn context_mut(&mut self) -> &mut S {
        &mut self.context
    }

    /// Check if a transition is valid without executing it
    pub fn can_transition(&self, event: &E) -> bool {
        if let Some(current_state) = self.states.get(&self.current_state) {
            if let Some(transition) = current_state.transitions.get(event.event_type()) {
                // Check guard condition if present
                if let Some(ref guard) = transition.guard {
                    guard(&self.context, event)
                } else {
                    true  // No guard means transition is always allowed
                }
            } else {
                false  // No transition defined for this event
            }
        } else {
            false  // Current state not found
        }
    }

    /// Get all possible states
    pub fn states(&self) -> Vec<&str> {
        self.states.keys().map(|s| s.as_str()).collect()
    }

    /// Get all possible transitions from current state
    pub fn possible_transitions(&self) -> Vec<String> {
        if let Some(current_state) = self.states.get(&self.current_state) {
            current_state.transitions.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Add middleware to this machine
    pub fn with_middleware<M: crate::middleware::Middleware<S, E> + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.middlewares = self.middlewares.add(middleware);
        self
    }

    /// Send an event with middleware processing
    ///
    /// Middleware will be executed before the transition is processed.
    /// If any middleware sets should_continue to false, the transition is cancelled.
    pub fn send_with_middleware(&mut self, event: E) -> MachineResult<()> {
        // Find transition first
        let current_state_name = self.current_state.clone();
        let can_transition = self.states.get(&current_state_name)
            .and_then(|node| node.transitions.get(event.event_type()))
            .is_some();

        if can_transition {
            let transition = self.states.get(&current_state_name)
                .and_then(|node| node.transitions.get(event.event_type()))
                .unwrap(); // We know it exists

            // Create middleware context with cloned data to avoid borrowing issues
            let mut ctx = crate::middleware::MiddlewareContext::<S, E>::new(
                crate::middleware::Operation::MachineTransition {
                    current_state: current_state_name.clone(),
                    event_type: event.event_type().to_string(),
                    target_state: transition.target.clone(),
                }
            );

            // Process middleware
            self.middlewares.process(&mut ctx)?;

            if ctx.should_continue {
                // Execute the transition
                self.send(event)
            } else {
                Err(MachineError::InvalidTransition {
                    from: current_state_name,
                    to: "cancelled by middleware".to_string(),
                })
            }
        } else {
            Err(MachineError::InvalidTransition {
                from: current_state_name,
                to: event.event_type().to_string(),
            })
        }
    }

    /// Get the middleware stack for this machine
    pub fn middlewares(&self) -> &crate::middleware::MiddlewareStack<S, E> {
        &self.middlewares
    }

    /// Get a mutable reference to the middleware stack
    pub fn middlewares_mut(&mut self) -> &mut crate::middleware::MiddlewareStack<S, E> {
        &mut self.middlewares
    }

    /// Enable DevTools integration for this machine
    ///
    /// Requires the devtools feature to be enabled.
    /// This allows real-time state inspection and debugging in browser DevTools.
    #[cfg(all(feature = "web", feature = "devtools"))]
    pub fn with_devtools(mut self, name: &str) -> Result<Self, crate::devtools::DevToolsError> {
        self.devtools = Some(crate::devtools::DevToolsIntegration::new(name.to_string())?);
        Ok(self)
    }

    /// Check if DevTools integration is enabled
    #[cfg(all(feature = "web", feature = "devtools"))]
    pub fn has_devtools(&self) -> bool {
        self.devtools.is_some()
    }

    /// Get the DevTools integration (if enabled)
    #[cfg(all(feature = "web", feature = "devtools"))]
    pub fn devtools(&self) -> Option<&crate::devtools::DevToolsIntegration> {
        self.devtools.as_ref()
    }

    /// Serialize the current machine state to JSON string
    ///
    /// Requires the serde feature and SerializableState bound.
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String, MachineError>
    where
        S: crate::SerializableState,
    {
        let snapshot = crate::MachineSnapshot {
            current_state: self.current_state.clone(),
            context: self.context.clone(),
            timestamp: std::time::SystemTime::now(),
        };

        serde_json::to_string(&snapshot)
            .map_err(|e| MachineError::SerializationError {
                message: e.to_string(),
            })
    }

    /// Deserialize machine state from JSON string and update the machine
    ///
    /// Requires the serde feature and SerializableState bound.
    #[cfg(feature = "serde")]
    pub fn from_json(&mut self, json: &str) -> Result<(), MachineError>
    where
        S: crate::SerializableState,
    {
        let snapshot: crate::MachineSnapshot<S> = serde_json::from_str(json)
            .map_err(|e| MachineError::DeserializationError {
                message: e.to_string(),
            })?;

        self.current_state = snapshot.current_state;
        self.context = snapshot.context;

        Ok(())
    }

    /// Export machine state as a snapshot with metadata
    ///
    /// Requires the serde feature and SerializableState bound.
    #[cfg(feature = "serde")]
    pub fn export_snapshot(&self) -> Result<crate::MachineSnapshot<S>, MachineError>
    where
        S: crate::SerializableState,
    {
        Ok(crate::MachineSnapshot {
            current_state: self.current_state.clone(),
            context: self.context.clone(),
            timestamp: std::time::SystemTime::now(),
        })
    }

    /// Import machine state from a snapshot
    ///
    /// Requires the serde feature and SerializableState bound.
    #[cfg(feature = "serde")]
    pub fn import_snapshot(&mut self, snapshot: crate::MachineSnapshot<S>) -> Result<(), MachineError>
    where
        S: crate::SerializableState,
    {
        self.current_state = snapshot.current_state;
        self.context = snapshot.context;

        Ok(())
    }
}

/// A node in the state machine representing a state
pub struct StateNode<S: State, E: Event> {
    /// Actions to execute when entering this state
    pub entry_actions: Option<Vec<Box<dyn Fn(&mut S, &E) -> ActionResult + Send + Sync>>>,

    /// Actions to execute when exiting this state
    pub exit_actions: Option<Vec<Box<dyn Fn(&mut S, &E) -> ActionResult + Send + Sync>>>,

    /// Transitions from this state (keyed by event type)
    pub transitions: HashMap<String, Transition<S, E>>,
}

impl<S: State, E: Event> StateNode<S, E> {
    /// Create a new state node
    pub fn new() -> Self {
        Self {
            entry_actions: None,
            exit_actions: None,
            transitions: HashMap::new(),
        }
    }

    /// Add an entry action
    pub fn on_entry<F>(mut self, action: F) -> Self
    where
        F: Fn(&mut S, &E) -> ActionResult + Send + Sync + 'static,
    {
        self.entry_actions.get_or_insert_with(Vec::new).push(Box::new(action));
        self
    }

    /// Add an exit action
    pub fn on_exit<F>(mut self, action: F) -> Self
    where
        F: Fn(&mut S, &E) -> ActionResult + Send + Sync + 'static,
    {
        self.exit_actions.get_or_insert_with(Vec::new).push(Box::new(action));
        self
    }

    /// Add a transition
    pub fn on(mut self, event: E, target: &str) -> Self {
        let transition = Transition::new(target);
        self.transitions.insert(event.event_type().to_string(), transition);
        self
    }

    /// Add a transition with actions
    pub fn on_with_actions<F>(mut self, event: E, target: &str, actions: Vec<F>) -> Self
    where
        F: Fn(&mut S, &E) -> ActionResult + Send + Sync + 'static,
    {
        let transition = Transition::new(target).with_actions(actions);
        self.transitions.insert(event.event_type().to_string(), transition);
        self
    }

    /// Add a guarded transition
    pub fn on_guarded<F>(mut self, event: E, target: &str, guard: F) -> Self
    where
        F: Fn(&S, &E) -> bool + Send + Sync + 'static,
    {
        let transition = Transition::new(target).with_guard(guard);
        self.transitions.insert(event.event_type().to_string(), transition);
        self
    }

    /// Add a guarded transition with actions
    pub fn on_guarded_with_actions<F, G>(
        mut self,
        event: E,
        target: &str,
        guard: F,
        actions: Vec<G>
    ) -> Self
    where
        F: Fn(&S, &E) -> bool + Send + Sync + 'static,
        G: Fn(&mut S, &E) -> ActionResult + Send + Sync + 'static,
    {
        let transition = Transition::new(target)
            .with_guard(guard)
            .with_actions(actions);
        self.transitions.insert(event.event_type().to_string(), transition);
        self
    }
}

impl<S: State, E: Event> Default for StateNode<S, E> {
    fn default() -> Self {
        Self::new()
    }
}

/// A transition between states
pub struct Transition<S: State, E: Event> {
    /// Target state name
    pub target: String,

    /// Guard condition that must be true for transition to occur
    pub guard: Option<Box<dyn Fn(&S, &E) -> bool + Send + Sync>>,

    /// Actions to execute during transition
    pub actions: Option<Vec<Box<dyn Fn(&mut S, &E) -> ActionResult + Send + Sync>>>,
}

impl<S: State, E: Event> Transition<S, E> {
    /// Create a new transition
    pub fn new(target: &str) -> Self {
        Self {
            target: target.to_string(),
            guard: None,
            actions: None,
        }
    }

    /// Add a guard condition to the transition
    pub fn with_guard<F>(mut self, guard: F) -> Self
    where
        F: Fn(&S, &E) -> bool + Send + Sync + 'static,
    {
        self.guard = Some(Box::new(guard));
        self
    }

    /// Add actions to the transition
    pub fn with_actions<F>(mut self, actions: Vec<F>) -> Self
    where
        F: Fn(&mut S, &E) -> ActionResult + Send + Sync + 'static,
    {
        self.actions = Some(actions.into_iter().map(|f| Box::new(f) as Box<dyn Fn(&mut S, &E) -> ActionResult + Send + Sync>).collect());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Default, Debug, Eq, PartialEq)]
    struct TestContext {
        value: i32,
    }

    #[derive(Clone, Default, Debug, Eq, PartialEq)]
    enum TestEvent {
        #[default]
        Increment,
        Decrement,
        Reset,
    }

    impl TestEvent {
        fn event_type(&self) -> &'static str {
            match self {
                TestEvent::Increment => "increment",
                TestEvent::Decrement => "decrement",
                TestEvent::Reset => "reset",
            }
        }
    }

    #[test]
    fn machine_creation_works() {
        let context = TestContext { value: 0 };
        let machine = Machine::<TestContext, TestEvent>::new("idle", context);
        assert_eq!(machine.current_state(), "idle");
        assert_eq!(machine.context().value, 0);
    }

    #[test]
    fn state_node_creation_works() {
        let state_node = StateNode::<TestContext, TestEvent>::new();
        assert!(state_node.entry_actions.is_none());
        assert!(state_node.transitions.is_empty());
    }

    #[test]
    fn transition_creation_works() {
        let transition = Transition::<TestContext, TestEvent>::new("running");
        assert_eq!(transition.target, "running");
        assert!(transition.actions.is_none());
    }

    #[test]
    fn simple_machine_transition() {
        let context = TestContext { value: 0 };
        let mut machine = Machine::new("idle", context);

        // Add a simple state
        let idle_state = StateNode::new().on(TestEvent::Increment, "running");
        machine.add_state("idle", idle_state);

        // Add running state
        let running_state = StateNode::new();
        machine.add_state("running", running_state);

        // Test transition
        assert_eq!(machine.current_state(), "idle");
        machine.send(TestEvent::Increment).unwrap();
        assert_eq!(machine.current_state, "running");
    }

    #[test]
    fn invalid_transition_returns_error() {
        let context = TestContext { value: 0 };
        let mut machine = Machine::new("idle", context);

        let idle_state = StateNode::new();
        machine.add_state("idle", idle_state);

        let result = machine.send(TestEvent::Increment);
        assert!(result.is_err());
    }

    // Phase 2: Entry/Exit Actions Tests
    #[test]
    fn entry_actions_execute_on_state_entry() {
        let mut machine = Machine::new("idle", TestContext { value: 0 });

        let idle_state = StateNode::new()
            .on_entry(|ctx: &mut TestContext, _| {
                ctx.value = 42;
                ActionResult::Continue
            })
            .on(TestEvent::Increment, "running");

        let running_state = StateNode::new()
            .on_entry(|ctx: &mut TestContext, _| {
                ctx.value = 100; // This should be the final value
                ActionResult::Continue
            });

        machine.add_state("idle", idle_state);
        machine.add_state("running", running_state);

        // Transition from idle to running - both exit and entry actions should execute
        machine.send(TestEvent::Increment).unwrap();
        assert_eq!(machine.context.value, 100); // Final value should be from running state entry
    }

    #[test]
    fn exit_actions_execute_on_state_exit() {
        let mut machine = Machine::new("idle", TestContext { value: 0 });

        let idle_state = StateNode::new()
            .on_exit(|ctx: &mut TestContext, _| {
                ctx.value = 99;
                ActionResult::Continue
            })
            .on(TestEvent::Increment, "running");

        let running_state = StateNode::new();
        machine.add_state("idle", idle_state);
        machine.add_state("running", running_state);

        machine.send(TestEvent::Increment).unwrap();
        assert_eq!(machine.context.value, 99); // Exit action should modify the context
    }

    // Phase 2: Guard Conditions Tests
    #[test]
    fn guard_blocks_invalid_transition() {
        let mut machine = Machine::new("idle", TestContext { value: 0 });

        let idle_state = StateNode::new()
            .on_guarded(TestEvent::Increment, "running", |ctx: &TestContext, _| ctx.value > 10);

        machine.add_state("idle", idle_state);

        // Should fail when guard condition is false
        assert!(!machine.can_transition(&TestEvent::Increment));
        let result = machine.send_guarded(TestEvent::Increment);
        assert!(matches!(result, Err(MachineError::GuardFailed { .. })));
        assert_eq!(machine.current_state, "idle"); // Still in idle
    }

    #[test]
    fn guard_allows_valid_transition() {
        let mut machine = Machine::new("idle", TestContext { value: 15 });

        let idle_state = StateNode::new()
            .on_guarded(TestEvent::Increment, "running", |ctx: &TestContext, _| ctx.value > 10);

        let running_state = StateNode::new();
        machine.add_state("idle", idle_state);
        machine.add_state("running", running_state);

        // Should succeed when guard condition is true
        assert!(machine.can_transition(&TestEvent::Increment));
        machine.send_guarded(TestEvent::Increment).unwrap();
        assert_eq!(machine.current_state, "running");
    }

    // Phase 2: Action Results Tests
    #[test]
    fn action_can_cancel_transition() {
        let mut machine = Machine::new("idle", TestContext { value: 0 });

        let idle_state = StateNode::new()
            .on_guarded_with_actions(
                TestEvent::Increment,
                "running",
                |_ctx, _event| true, // Guard always passes
                vec![|ctx: &mut TestContext, _event: &TestEvent| {
                    ctx.value = 100;
                    ActionResult::Cancel // Cancel the transition
                }]
            );

        machine.add_state("idle", idle_state);

        let result = machine.send_with_actions(TestEvent::Increment);
        assert!(matches!(result, Err(MachineError::ActionCancelled { .. })));
        assert_eq!(machine.current_state, "idle"); // Still in idle
        assert_eq!(machine.context.value, 100); // Action executed but transition cancelled
    }

    #[test]
    fn action_can_redirect_transition() {
        let mut machine = Machine::new("idle", TestContext { value: 0 });

        let idle_state = StateNode::new()
            .on_guarded_with_actions(
                TestEvent::Increment,
                "running", // Original target
                |_, _| true,
                vec![|ctx: &mut TestContext, _: &TestEvent| {
                    ctx.value = 50;
                    ActionResult::Redirect("error".to_string()) // Redirect to error state
                }]
            );

        let error_state = StateNode::new();
        machine.add_state("idle", idle_state);
        machine.add_state("error", error_state);

        machine.send_with_actions(TestEvent::Increment).unwrap();
        assert_eq!(machine.current_state(), "error"); // Redirected to error
        assert_eq!(machine.context().value, 50);
    }

    #[test]
    fn action_can_return_error() {
        let mut machine = Machine::new("idle", TestContext { value: 0 });

        let idle_state = StateNode::new()
            .on_guarded_with_actions(
                TestEvent::Increment,
                "running",
                |_, _| true,
                vec![|ctx: &mut TestContext, _: &TestEvent| {
                    ctx.value = 75;
                    ActionResult::Error("Validation failed".to_string())
                }]
            );

        machine.add_state("idle", idle_state);

        let result = machine.send_with_actions(TestEvent::Increment);
        assert!(matches!(result, Err(MachineError::ActionError { .. })));
        assert_eq!(machine.current_state, "idle"); // Still in idle
        assert_eq!(machine.context.value, 75); // Action executed but transition failed
    }

    #[test]
    fn action_continue_allows_normal_transition() {
        let mut machine = Machine::new("idle", TestContext { value: 0 });

        let idle_state = StateNode::new()
            .on_guarded_with_actions(
                TestEvent::Increment,
                "running",
                |_, _| true,
                vec![|ctx: &mut TestContext, _: &TestEvent| {
                    ctx.value = 25;
                    ActionResult::Continue // Allow transition
                }]
            );

        let running_state = StateNode::new();
        machine.add_state("idle", idle_state);
        machine.add_state("running", running_state);

        machine.send_with_actions(TestEvent::Increment).unwrap();
        assert_eq!(machine.current_state, "running");
        assert_eq!(machine.context.value, 25);
    }
}
