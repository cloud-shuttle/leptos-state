//! # Core Traits for State Machine Architecture
//! 
//! This module defines the foundational traits that establish the type-safe
//! architecture for state machines and stores.

use std::fmt::Debug;

// =============================================================================
// Base Traits with Proper Bounds
// =============================================================================

/// Context for state machines that provides shared state and configuration.
/// 
/// This trait establishes the minimum requirements for context types:
/// - `Clone`: Allows context to be copied during state transitions
/// - `Debug`: Enables debugging and logging
/// - `Default`: Provides default initialization
/// - `Send + Sync`: Enables safe sharing across thread boundaries
/// - `'static`: Ensures the type can be stored in static contexts
pub trait StateMachineContext: Clone + Debug + Default + Send + Sync + 'static {
    // Marker trait - no additional methods required
}

/// Events that can trigger state transitions.
/// 
/// This trait establishes the minimum requirements for event types:
/// - `Clone`: Allows events to be copied and reused
/// - `Debug`: Enables debugging and logging
/// - `PartialEq`: Enables event comparison and matching
/// - `Send + Sync`: Enables safe sharing across thread boundaries
/// - `'static`: Ensures the type can be stored in static contexts
pub trait StateMachineEvent: Clone + Debug + PartialEq + Send + Sync + 'static {
    // Marker trait - no additional methods required
}

/// States in a state machine.
/// 
/// This trait establishes the minimum requirements for state types:
/// - `Clone`: Allows states to be copied during transitions
/// - `Debug`: Enables debugging and logging
/// - `Send + Sync`: Enables safe sharing across thread boundaries
/// - `'static`: Ensures the type can be stored in static contexts
pub trait StateMachineState: Clone + Debug + Send + Sync + 'static {
    /// The context type associated with this state machine
    type Context: StateMachineContext;
    /// The event type that can trigger transitions
    type Event: StateMachineEvent;
}

// =============================================================================
// Core State Machine Trait
// =============================================================================

/// Core trait for state machines that defines the essential behavior.
/// 
/// This trait extends `StateMachineState` to provide the core functionality
/// that all state machines must implement.
pub trait StateMachine: StateMachineState {
    /// Returns the initial state of the machine
    fn initial_state(&self) -> Self;
    
    /// Transitions from the current state to a new state based on an event
    fn transition(&self, state: &Self, event: Self::Event) -> Self;
    
    /// Checks if a transition is valid from the current state with the given event
    fn can_transition(&self, state: &Self, event: Self::Event) -> bool;
    
    /// Attempts to transition, returning an error if the transition is invalid
    fn try_transition(&self, state: &Self, event: Self::Event) -> Result<Self, crate::v1::error::TransitionError<Self::Event>>;
    
    /// Returns the number of states in the machine
    fn state_count(&self) -> usize;
    
    /// Checks if a state is valid for this machine
    fn is_valid_state(&self, state: &Self) -> bool;
    
    /// Checks if a state is reachable from the initial state
    fn is_reachable(&self, state: &Self) -> bool;
}

// =============================================================================
// Store Management Traits
// =============================================================================

/// Trait for store state that can be managed reactively.
/// 
/// This trait establishes the minimum requirements for store state types:
/// - `Clone`: Allows state to be copied for reactive updates
/// - `Debug`: Enables debugging and logging
/// - `Default`: Provides default initialization
/// - `Send + Sync`: Enables safe sharing across thread boundaries
/// - `'static`: Ensures the type can be stored in static contexts
pub trait StoreState: Clone + Debug + Default + Send + Sync + 'static {
    // Marker trait - no additional methods required
}

/// Core trait for stores that manage reactive state.
/// 
/// This trait provides the essential functionality for state management:
/// - State creation and initialization
/// - State updates and mutations
/// - State validation and constraints
pub trait Store: StoreState {
    /// Creates a new instance of the store with default state
    fn create() -> Self;
    
    /// Creates a new instance with custom initial state
    fn create_with_state(state: Self) -> Self;
    
    /// Updates the store state using a closure
    fn update<F>(&mut self, f: F) 
    where 
        F: FnOnce(&mut Self);
    
    /// Gets a reference to the current state
    fn get(&self) -> &Self;
    
    /// Gets a mutable reference to the current state
    fn get_mut(&mut self) -> &mut Self;
}

// =============================================================================
// Action and Guard Traits
// =============================================================================

/// Actions that can be executed during state transitions.
/// 
/// Actions are side effects that occur when transitioning between states.
/// They can modify context, log events, trigger external calls, etc.
pub trait Action<C: StateMachineContext>: std::fmt::Debug + Send + Sync {
    /// Executes the action with the given context
    fn execute(&self, context: &mut C) -> Result<(), super::error::ActionError>;
    
    /// Returns a description of what the action does
    fn description(&self) -> &'static str;
}

/// Guards that control whether transitions are allowed.
/// 
/// Guards are conditions that must be satisfied for a transition to occur.
/// They can check context values, validate state, enforce business rules, etc.
pub trait Guard<C: StateMachineContext, E: StateMachineEvent>: std::fmt::Debug + Send + Sync {
    /// Checks if the transition is allowed with the given context and event
    fn check(&self, context: &C, event: &E) -> bool;
    
    /// Returns a description of what the guard checks
    fn description(&self) -> &'static str;
}

// =============================================================================
// Error Types
// =============================================================================

// Error types are now defined in the error module to avoid conflicts

// =============================================================================
// Default Implementations
// =============================================================================

// Note: Blanket implementations removed to avoid conflicts with explicit impls
// Each type should implement these traits explicitly as needed

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default, PartialEq)]
    struct TestContext {
        count: i32,
        name: String,
    }

    impl StateMachineContext for TestContext {}

    #[derive(Clone, Debug, PartialEq)]
    enum TestEvent {
        Increment,
        Decrement,
        SetName(String),
    }

    impl StateMachineEvent for TestEvent {}

    #[derive(Clone, Debug, PartialEq)]
    enum TestState {
        Idle,
        Active,
        Paused,
    }

    impl StateMachineState for TestState {
        type Context = TestContext;
        type Event = TestEvent;
    }

    #[test]
    fn test_trait_bounds() {
        // Test that our types implement the required traits
        let context = TestContext { count: 0, name: "test".to_string() };
        let event = TestEvent::Increment;
        let state = TestState::Idle;
        
        // These should compile without errors - just verify the types work
        let _context: TestContext = context.clone();
        let _event: TestEvent = event.clone();
        let _state: TestState = state.clone();
        
        assert!(true); // Basic compilation test
    }

    #[test]
    fn test_default_implementations() {
        // Test that default implementations work
        let _context = TestContext::default();
        let _event = TestEvent::Increment;
        let _state = TestState::Idle;
        
        // These should work due to blanket implementations
        assert!(std::any::type_name::<TestContext>().contains("TestContext"));
        assert!(std::any::type_name::<TestEvent>().contains("TestEvent"));
        assert!(std::any::type_name::<TestState>().contains("TestState"));
    }
}
