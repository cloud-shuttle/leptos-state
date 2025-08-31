use leptos_state::utils::types::{StateError, StateResult};
use tests::common::{TestContext, TestEvent};

#[test]
fn test_state_error_creation() {
    let error = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    assert!(matches!(error, StateError::InvalidTransition { .. }));
}

#[test]
fn test_state_error_invalid_transition() {
    let error = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    let message = error.to_string();
    assert!(message.contains("idle"));
    assert!(message.contains("start"));
}

#[test]
fn test_state_error_invalid_state() {
    let error = StateError::InvalidState("nonexistent".to_string());
    
    let message = error.to_string();
    assert!(message.contains("nonexistent"));
}

#[test]
fn test_state_error_guard_failed() {
    let error = StateError::GuardFailed {
        guard: "test_guard".to_string(),
        reason: "Counter too low".to_string(),
    };
    
    let message = error.to_string();
    assert!(message.contains("test_guard"));
    assert!(message.contains("Counter too low"));
}

#[test]
fn test_state_error_action_failed() {
    let error = StateError::ActionFailed {
        action: "test_action".to_string(),
        reason: "Database error".to_string(),
    };
    
    let message = error.to_string();
    assert!(message.contains("test_action"));
    assert!(message.contains("Database error"));
}

#[test]
fn test_state_error_serialization_error() {
    let error = StateError::SerializationError("Invalid JSON".to_string());
    
    let message = error.to_string();
    assert!(message.contains("Invalid JSON"));
}

#[test]
fn test_state_error_deserialization_error() {
    let error = StateError::DeserializationError("Missing field".to_string());
    
    let message = error.to_string();
    assert!(message.contains("Missing field"));
}

#[test]
fn test_state_error_storage_error() {
    let error = StateError::StorageError("Disk full".to_string());
    
    let message = error.to_string();
    assert!(message.contains("Disk full"));
}

#[test]
fn test_state_error_validation_error() {
    let error = StateError::ValidationError("Invalid data".to_string());
    
    let message = error.to_string();
    assert!(message.contains("Invalid data"));
}

#[test]
fn test_state_error_custom() {
    let error = StateError::Custom("Custom error message".to_string());
    
    let message = error.to_string();
    assert!(message.contains("Custom error message"));
}

#[test]
fn test_state_error_clone() {
    let error = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    let cloned = error.clone();
    assert!(matches!(cloned, StateError::InvalidTransition { .. }));
}

#[test]
fn test_state_error_debug() {
    let error = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("InvalidTransition"));
    assert!(debug_str.contains("idle"));
    assert!(debug_str.contains("start"));
}

#[test]
fn test_state_result_ok() {
    let result: StateResult<()> = Ok(());
    assert!(result.is_ok());
}

#[test]
fn test_state_result_err() {
    let error = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    let result: StateResult<()> = Err(error);
    assert!(result.is_err());
}

#[test]
fn test_state_error_from_string() {
    let error = StateError::from("Test error message");
    assert!(matches!(error, StateError::Custom(_)));
}

#[test]
fn test_state_error_from_boxed_error() {
    let boxed_error: Box<dyn std::error::Error + Send + Sync> = 
        Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
    
    let error = StateError::from(boxed_error);
    assert!(matches!(error, StateError::Custom(_)));
}

#[test]
fn test_state_error_equality() {
    let error1 = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    let error2 = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    let error3 = StateError::InvalidTransition {
        from: "active".to_string(),
        event: "start".to_string(),
    };
    
    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
}

#[test]
fn test_state_error_hash() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    let error = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    map.insert(error.clone(), "test_value");
    assert_eq!(map.get(&error), Some(&"test_value"));
}

#[test]
fn test_state_error_ordering() {
    let error1 = StateError::InvalidState("a".to_string());
    let error2 = StateError::InvalidState("b".to_string());
    let error3 = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    assert!(error1 < error2);
    assert!(error1 < error3);
    assert!(error2 < error3);
}

#[test]
fn test_state_error_context() {
    let error = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    // Test that we can use the error in a Result
    let result: StateResult<()> = Err(error);
    assert!(result.is_err());
    
    if let Err(StateError::InvalidTransition { from, event }) = result {
        assert_eq!(from, "idle");
        assert_eq!(event, "start");
    } else {
        panic!("Expected InvalidTransition error");
    }
}

#[test]
fn test_state_error_conversion() {
    // Test conversion from different error types
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let state_error: StateError = io_error.into();
    assert!(matches!(state_error, StateError::Custom(_)));
}

#[test]
fn test_state_error_with_context() {
    let error = StateError::GuardFailed {
        guard: "test_guard".to_string(),
        reason: "Counter must be greater than 5".to_string(),
    };
    
    let context = TestContext { counter: 3 };
    let event = TestEvent::Start;
    
    // Test that the error contains meaningful information
    let message = error.to_string();
    assert!(message.contains("test_guard"));
    assert!(message.contains("Counter must be greater than 5"));
}

#[test]
fn test_state_error_chain() {
    let error1 = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    let error2 = StateError::GuardFailed {
        guard: "test_guard".to_string(),
        reason: "Validation failed".to_string(),
    };
    
    // Test that we can chain errors
    let result: StateResult<()> = Err(error1);
    let chained_result = result.and_then(|_| Err(error2));
    
    assert!(chained_result.is_err());
}

#[test]
fn test_state_error_recovery() {
    let error = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    let result: StateResult<()> = Err(error);
    
    // Test error recovery patterns
    let recovered = result.unwrap_or_else(|_| ());
    assert_eq!(recovered, ());
}

#[test]
fn test_state_error_mapping() {
    let error = StateError::InvalidTransition {
        from: "idle".to_string(),
        event: "start".to_string(),
    };
    
    let result: StateResult<()> = Err(error);
    
    // Test mapping over errors
    let mapped = result.map_err(|e| {
        if let StateError::InvalidTransition { from, event } = e {
            StateError::Custom(format!("Transition from {} with event {} failed", from, event))
        } else {
            e
        }
    });
    
    assert!(mapped.is_err());
    if let Err(StateError::Custom(msg)) = mapped {
        assert!(msg.contains("Transition from idle with event start failed"));
    } else {
        panic!("Expected Custom error");
    }
}
