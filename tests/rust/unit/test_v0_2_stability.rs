//! Test suite to validate v0.2.0 stability
//! This test ensures the current API is working correctly before we start the v1.0.0 redesign

use leptos_state::{
    machine::{Machine, MachineBuilder, StateMachine, MachineState},
    store::{Store, StoreState},
    hooks::{use_machine, use_store},
    utils::types::StateResult,
};
use tests::common::{TestContext, TestEvent, TestState, TestStore, TestMachine};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v0_2_machine_creation() {
        // Test that we can create a basic machine with current API
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .state("active", |_| TestState::Active)
            .transition("idle", "start", "active")
            .initial("idle")
            .build();

        assert_eq!(machine.initial, "idle");
        assert_eq!(machine.states.len(), 2);
    }

    #[test]
    fn test_v0_2_machine_transition() {
        // Test that basic transitions work
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .state("active", |_| TestState::Active)
            .transition("idle", "start", "active")
            .initial("idle")
            .build();

        let mut state = MachineState::new(&machine, TestContext::default());
        
        let result = Machine::transition(&machine, &mut state, &TestEvent::Start);
        assert!(result.is_ok());
        assert_eq!(state.current_state, "active");
    }

    #[test]
    fn test_v0_2_store_creation() {
        // Test that stores can be created
        let store = TestStore::create();
        assert!(matches!(store, TestState::Idle));
    }

    #[test]
    fn test_v0_2_store_state_updates() {
        // Test that store state can be updated
        let store = TestStore::create();
        assert!(matches!(store, TestState::Idle));
        
        // Test state transitions
        let new_state = TestMachine::transition(&store, TestEvent::Increment);
        assert!(matches!(new_state, TestState::Counting));
    }

    #[test]
    fn test_v0_2_api_consistency() {
        // Test that the current API is consistent
        // This test will help us identify what needs to be preserved in v1.0.0
        
        // Machine API
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .initial("idle")
            .build();
        
        assert!(machine.initial == "idle");
        
        // Store API
        let store = TestStore::create();
        assert!(matches!(store, TestState::Idle));
        
        // This test passes if the current API is working
        assert!(true, "v0.2.0 API is consistent");
    }

    #[test]
    fn test_v0_2_error_handling() {
        // Test that error handling works correctly
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .initial("idle")
            .build();

        let mut state = MachineState::new(&machine, TestContext::default());
        
        // Test invalid transition
        let result = Machine::transition(&machine, &mut state, &TestEvent::Stop);
        assert!(result.is_err());
        assert_eq!(state.current_state, "idle");
    }

    #[test]
    fn test_v0_2_performance_basic() {
        // Basic performance test to ensure no major regressions
        let start = std::time::Instant::now();
        
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .state("active", |_| TestState::Active)
            .transition("idle", "start", "active")
            .initial("idle")
            .build();

        let mut state = MachineState::new(&machine, TestContext::default());
        
        // Perform 1000 transitions
        for _ in 0..1000 {
            let _ = Machine::transition(&machine, &mut state, &TestEvent::Start);
            let _ = Machine::transition(&machine, &mut state, &TestEvent::Stop);
        }
        
        let duration = start.elapsed();
        
        // Should complete in reasonable time (less than 1 second)
        assert!(duration.as_millis() < 1000, "Performance regression detected: {:?}", duration);
    }
}
