//! Integration tests for feature combinations
//! 
//! These tests verify that different feature combinations work correctly together
//! and that the state machine behaves consistently across different configurations.

use leptos_state::{
    machine::{MachineBuilder, MachineState},
    machine::states::StateValue,
};

/// Test context for feature combination tests
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FeatureTestContext {
    pub counter: i32,
    pub name: String,
    pub enabled: bool,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Test events for feature combination tests
#[derive(Debug, Clone, PartialEq)]
pub enum FeatureTestEvent {
    Increment,
    Decrement,
    SetName(String),
    Toggle,
    Reset,
    SetMetadata(String, String),
}

impl Default for FeatureTestEvent {
    fn default() -> Self {
        FeatureTestEvent::Increment
    }
}

impl leptos_state::machine::events::Event for FeatureTestEvent {
    fn event_type(&self) -> &str {
        match self {
            FeatureTestEvent::Increment => "Increment",
            FeatureTestEvent::Decrement => "Decrement",
            FeatureTestEvent::SetName(_) => "SetName",
            FeatureTestEvent::Toggle => "Toggle",
            FeatureTestEvent::Reset => "Reset",
            FeatureTestEvent::SetMetadata(_, _) => "SetMetadata",
        }
    }
}

/// Test states for feature combination tests
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum FeatureTestState {
    Idle,
    Active,
    Paused,
    Counting,
    Processing,
}

impl MachineState for FeatureTestState {
    type Context = FeatureTestContext;

    fn value(&self) -> &StateValue {
        static IDLE_VALUE: std::sync::LazyLock<StateValue> = std::sync::LazyLock::new(|| StateValue::Simple("idle".to_string()));
        static ACTIVE_VALUE: std::sync::LazyLock<StateValue> = std::sync::LazyLock::new(|| StateValue::Simple("active".to_string()));
        static PAUSED_VALUE: std::sync::LazyLock<StateValue> = std::sync::LazyLock::new(|| StateValue::Simple("paused".to_string()));
        static COUNTING_VALUE: std::sync::LazyLock<StateValue> = std::sync::LazyLock::new(|| StateValue::Simple("counting".to_string()));
        static PROCESSING_VALUE: std::sync::LazyLock<StateValue> = std::sync::LazyLock::new(|| StateValue::Simple("processing".to_string()));
        
        match self {
            FeatureTestState::Idle => &IDLE_VALUE,
            FeatureTestState::Active => &ACTIVE_VALUE,
            FeatureTestState::Paused => &PAUSED_VALUE,
            FeatureTestState::Counting => &COUNTING_VALUE,
            FeatureTestState::Processing => &PROCESSING_VALUE,
        }
    }

    fn context(&self) -> &Self::Context {
        static DEFAULT_CONTEXT: std::sync::LazyLock<FeatureTestContext> = std::sync::LazyLock::new(|| FeatureTestContext::default());
        &DEFAULT_CONTEXT
    }

    fn matches(&self, other: &str) -> bool {
        match self {
            FeatureTestState::Idle => other == "idle",
            FeatureTestState::Active => other == "active",
            FeatureTestState::Paused => other == "paused",
            FeatureTestState::Counting => other == "counting",
            FeatureTestState::Processing => other == "processing",
        }
    }

    fn can_transition_to(&self, target: &str) -> bool {
        match (self, target) {
            (FeatureTestState::Idle, "active") => true,
            (FeatureTestState::Active, "counting") => true,
            (FeatureTestState::Active, "paused") => true,
            (FeatureTestState::Active, "idle") => true,
            (FeatureTestState::Paused, "active") => true,
            (FeatureTestState::Counting, "processing") => true,
            (FeatureTestState::Processing, "idle") => true,
            _ => false,
        }
    }
}

/// Create a test machine for feature combination tests
pub fn create_feature_test_machine() -> leptos_state::machine::Machine<FeatureTestContext, FeatureTestEvent> {
    MachineBuilder::<FeatureTestContext, FeatureTestEvent>::new()
        .state("idle")
            .on(FeatureTestEvent::Increment, "active")
        .state("active")
            .on(FeatureTestEvent::Increment, "counting")
            .on(FeatureTestEvent::Decrement, "idle")
            .on(FeatureTestEvent::Toggle, "paused")
        .state("paused")
            .on(FeatureTestEvent::Toggle, "active")
        .state("counting")
            .on(FeatureTestEvent::Increment, "processing")
            .on(FeatureTestEvent::Reset, "idle")
        .state("processing")
            .on(FeatureTestEvent::Reset, "idle")
        .initial("idle")
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_state_transitions() {
        let machine = create_feature_test_machine();
        let initial_state = machine.initial_state();
        
        // Test basic transitions using the Machine API
        let active_state = machine.transition(&initial_state, FeatureTestEvent::Increment);
        assert_eq!(active_state.value(), &StateValue::Simple("active".to_string()));
        
        let counting_state = machine.transition(&active_state, FeatureTestEvent::Increment);
        assert_eq!(counting_state.value(), &StateValue::Simple("counting".to_string()));
        
        let idle_state = machine.transition(&counting_state, FeatureTestEvent::Reset);
        assert_eq!(idle_state.value(), &StateValue::Simple("idle".to_string()));
    }

    #[test]
    fn test_state_validation() {
        let machine = create_feature_test_machine();
        
        // Test that all states are valid
        assert!(machine.is_valid_state("idle"));
        assert!(machine.is_valid_state("active"));
        assert!(machine.is_valid_state("paused"));
        assert!(machine.is_valid_state("counting"));
        assert!(machine.is_valid_state("processing"));
        
        // Test that invalid states are rejected
        assert!(!machine.is_valid_state("invalid"));
        assert!(!machine.is_valid_state("nonexistent"));
    }

    #[test]
    fn test_transition_validation() {
        let machine = create_feature_test_machine();
        let idle_state = machine.initial_state();
        
        // Test valid transitions
        assert!(machine.can_transition(&idle_state, &FeatureTestEvent::Increment));
        
        // Test invalid transitions
        assert!(!machine.can_transition(&idle_state, &FeatureTestEvent::Decrement));
        assert!(!machine.can_transition(&idle_state, &FeatureTestEvent::Toggle));
    }

    #[test]
    fn test_machine_statistics() {
        let machine = create_feature_test_machine();
        
        // Test state count
        assert_eq!(machine.state_count(), 5);
        
        // Test transition count (updated to reflect actual transitions)
        assert_eq!(machine.transition_count(), 8);
    }

    #[test]
    fn test_context_handling() {
        let machine = create_feature_test_machine();
        let state = machine.initial_state();
        
        // Test that context is preserved
        let original_context = state.context();
        let new_state = machine.transition(&state, FeatureTestEvent::Increment);
        
        // Context should be preserved through transitions
        assert_eq!(new_state.context(), original_context);
    }

    #[test]
    fn test_complex_state_sequence() {
        let machine = create_feature_test_machine();
        let mut state = machine.initial_state();
        
        // Test a complex sequence of transitions
        state = machine.transition(&state, FeatureTestEvent::Increment); // idle -> active
        assert_eq!(state.value(), &StateValue::Simple("active".to_string()));
        
        state = machine.transition(&state, FeatureTestEvent::Toggle); // active -> paused
        assert_eq!(state.value(), &StateValue::Simple("paused".to_string()));
        
        state = machine.transition(&state, FeatureTestEvent::Toggle); // paused -> active
        assert_eq!(state.value(), &StateValue::Simple("active".to_string()));
        
        state = machine.transition(&state, FeatureTestEvent::Increment); // active -> counting
        assert_eq!(state.value(), &StateValue::Simple("counting".to_string()));
        
        state = machine.transition(&state, FeatureTestEvent::Increment); // counting -> processing
        assert_eq!(state.value(), &StateValue::Simple("processing".to_string()));
        
        state = machine.transition(&state, FeatureTestEvent::Reset); // processing -> idle
        assert_eq!(state.value(), &StateValue::Simple("idle".to_string()));
    }
}
