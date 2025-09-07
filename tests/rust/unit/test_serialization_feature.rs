//! TDD Tests for Serialization Feature
//! 
//! These tests define the expected behavior of the serialization feature
//! and will guide the implementation.

use leptos_state::{
    machine::{MachineBuilder, MachineStateImpl},
    machine::states::StateValue,
    utils::types::StateResult,
};

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize, serde::Deserialize))]
struct TestContext {
    count: i32,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize, serde::Deserialize))]
enum TestEvent {
    #[default]
    Increment,
    Decrement,
    SetName(String),
}

impl leptos_state::machine::events::Event for TestEvent {
    fn event_type(&self) -> &str {
        match self {
            TestEvent::Increment => "increment",
            TestEvent::Decrement => "decrement",
            TestEvent::SetName(_) => "set_name",
        }
    }
}

/// Test that serialization feature is properly defined in Cargo.toml
#[test]
fn test_serialization_feature_exists() {
    // This test will fail if the serialization feature doesn't exist
    // We expect it to be defined in Cargo.toml
    #[cfg(feature = "serialization")]
    {
        // Feature exists - test passes
        assert!(true);
    }
    
    #[cfg(not(feature = "serialization"))]
    {
        // Feature doesn't exist - this is the problem we're fixing
        panic!("Serialization feature is not defined in Cargo.toml");
    }
}

/// Test that serialization works when feature is enabled
#[test]
fn test_serialization_works_when_enabled() {
    #[cfg(feature = "serialization")]
    {
        let context = TestContext {
            count: 42,
            name: "test".to_string(),
        };

        let state = MachineStateImpl::new(StateValue::Simple("idle".to_string()), context);

        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        // Test that we can serialize the machine state
        let serialized = serde_json::to_string(&state).expect("Should serialize successfully");
        assert!(!serialized.is_empty());
        
        // Test that we can deserialize it back
        let deserialized: MachineStateImpl<TestContext> = serde_json::from_str(&serialized)
            .expect("Should deserialize successfully");
        assert_eq!(deserialized.context().count, 42);
        assert_eq!(deserialized.context().name, "test");
    }
    
    #[cfg(not(feature = "serialization"))]
    {
        // When feature is not enabled, this test should be skipped
        println!("Skipping serialization test - feature not enabled");
    }
}

/// Test that persistence tests work with serialization feature
#[test]
fn test_persistence_with_serialization() {
    #[cfg(all(feature = "serialization", feature = "persist"))]
    {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        // Test that persistence works with serialization
        let config = leptos_state::machine::persistence::PersistenceConfig {
            enabled: true,
            storage_key: "test_machine".to_string(),
            auto_save: false,
            auto_restore: false,
            ..Default::default()
        };

        let persistent_machine = machine
            .with_persistence(config)
            .initialize()
            .expect("Should initialize with persistence");

        // Test that we can save and restore
        persistent_machine.save().expect("Should save successfully");
        
        // This should work without the current workaround
        let new_machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let restored_machine = new_machine
            .with_persistence(config)
            .initialize()
            .expect("Should restore successfully");

        assert_eq!(
            *restored_machine.current_state().unwrap().value(),
            StateValue::Simple("idle".to_string())
        );
    }
    
    #[cfg(not(all(feature = "serialization", feature = "persist")))]
    {
        println!("Skipping persistence test - required features not enabled");
    }
}

/// Test that serialization feature provides proper trait bounds
#[test]
fn test_serialization_trait_bounds() {
    #[cfg(feature = "serialization")]
    {
        // Test that our types implement the required traits
        let context = TestContext {
            count: 42,
            name: "test".to_string(),
        };

        // This should compile without issues
        let _serialized = serde_json::to_string(&context).expect("Should serialize");
        let _deserialized: TestContext = serde_json::from_str(&_serialized).expect("Should deserialize");
        
        // Test that we can use these types in generic contexts
        fn serialize_any<T: serde::Serialize>(value: &T) -> String {
            serde_json::to_string(value).unwrap()
        }
        
        let _result = serialize_any(&context);
        assert!(!_result.is_empty());
    }
    
    #[cfg(not(feature = "serialization"))]
    {
        println!("Skipping trait bounds test - serialization feature not enabled");
    }
}
