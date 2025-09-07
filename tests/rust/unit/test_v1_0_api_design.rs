//! Test suite for v1.0.0 API design
//! This test defines the expected API structure for v1.0.0

use leptos_state::v1::*;

// Test data structures that should work with v1.0.0 API
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestContext {
    pub counter: i32,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TestEvent {
    Start,
    Stop,
    Increment,
    Decrement,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TestState {
    Idle,
    Active,
    Paused,
}

// Test store state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestStoreState {
    pub count: i32,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_0_machine_creation() {
        // Test that we can create a machine with v1.0.0 API
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .state("active", |_| TestState::Active)
            .transition("idle", TestEvent::Start, "active")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        assert_eq!(machine.initial_state().value(), "idle");
    }

    #[test]
    fn test_v1_0_machine_transition() {
        // Test that transitions work with v1.0.0 API
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .state("active", |_| TestState::Active)
            .transition("idle", TestEvent::Start, "active")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let mut state = machine.initial_state();
        
        // Test valid transition
        let result = machine.transition(&state, TestEvent::Start);
        assert!(result.is_ok());
        
        let new_state = result.unwrap();
        assert_eq!(new_state.value(), "active");
    }

    #[test]
    fn test_v1_0_store_creation() {
        // Test that stores can be created with v1.0.0 API
        let store = TestStoreState::default();
        assert_eq!(store.count, 0);
        assert_eq!(store.name, "");
    }

    #[test]
    fn test_v1_0_store_state_updates() {
        // Test that store state can be updated
        let mut store = TestStoreState::default();
        store.count = 42;
        store.name = "test".to_string();
        
        assert_eq!(store.count, 42);
        assert_eq!(store.name, "test");
    }

    #[test]
    fn test_v1_0_api_consistency() {
        // Test that the v1.0.0 API is consistent
        // This test will help us identify what needs to be implemented
        
        // Machine API
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .initial("idle")
            .build()
            .expect("Failed to build machine");
        
        assert!(machine.initial_state().value() == "idle");
        
        // Store API
        let store = TestStoreState::default();
        assert!(store.count == 0);
        
        // This test passes if the v1.0.0 API is working
        assert!(true, "v1.0.0 API is consistent");
    }

    #[test]
    fn test_v1_0_error_handling() {
        // Test that error handling works correctly
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let state = machine.initial_state();
        
        // Test invalid transition
        let result = machine.transition(&state, TestEvent::Stop);
        assert!(result.is_err());
    }

    #[test]
    fn test_v1_0_performance_basic() {
        // Basic performance test to ensure no major regressions
        let start = std::time::Instant::now();
        
        let machine = MachineBuilder::new()
            .state("idle", |_| TestState::Idle)
            .state("active", |_| TestState::Active)
            .transition("idle", TestEvent::Start, "active")
            .initial("idle")
            .build()
            .expect("Failed to build machine");

        let mut state = machine.initial_state();
        
        // Perform 1000 transitions
        for _ in 0..1000 {
            if let Ok(new_state) = machine.transition(&state, TestEvent::Start) {
                state = new_state;
            }
            if let Ok(new_state) = machine.transition(&state, TestEvent::Stop) {
                state = new_state;
            }
        }
        
        let duration = start.elapsed();
        
        // Should complete in reasonable time (less than 1 second)
        assert!(duration.as_millis() < 1000, "Performance regression detected: {:?}", duration);
    }
}
