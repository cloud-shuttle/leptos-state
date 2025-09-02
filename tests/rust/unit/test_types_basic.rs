use leptos_state::utils::types::{StateError, StateResult};

#[test]
fn test_state_error_creation() {
    let error = StateError::new("Test error message");
    assert_eq!(error.to_string(), "Test error message");
}

#[test]
fn test_state_error_clone() {
    let error = StateError::new("Test error message");
    let cloned = error.clone();
    assert_eq!(error.to_string(), cloned.to_string());
}

#[test]
fn test_state_error_debug() {
    let error = StateError::new("Test error message");
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Test error message"));
}

#[test]
fn test_state_error_partial_eq() {
    let error1 = StateError::new("Test error message");
    let error2 = StateError::new("Test error message");
    let error3 = StateError::new("Different error message");
    
    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
}

#[test]
fn test_state_result_ok() {
    let result: StateResult<i32> = Ok(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_state_result_err() {
    let result: StateResult<i32> = Err(StateError::new("Test error"));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Test error");
}

#[test]
fn test_state_error_display() {
    let error = StateError::new("Display test");
    let display_str = format!("{}", error);
    assert_eq!(display_str, "Display test");
}

#[test]
fn test_state_error_custom() {
    let error = StateError::custom("Custom error message");
    assert_eq!(error.to_string(), "Custom error message");
}

#[test]
fn test_state_error_with_context() {
    let error = StateError::new("Base error").with_context("Additional context");
    let error_str = error.to_string();
    assert!(error_str.contains("Base error"));
    assert!(error_str.contains("Additional context"));
}

#[test]
fn test_state_error_chain() {
    let error1 = StateError::new("First error");
    let error2 = StateError::new("Second error");
    let chained = error1.chain(error2);
    
    let error_str = chained.to_string();
    assert!(error_str.contains("First error"));
    assert!(error_str.contains("Second error"));
}

#[test]
fn test_state_error_map() {
    let error = StateError::new("Original error");
    let mapped = error.map(|msg| format!("Mapped: {}", msg));
    assert_eq!(mapped.to_string(), "Mapped: Original error");
}

#[test]
fn test_state_error_recovery() {
    let error = StateError::new("Recoverable error");
    let recovered = error.recover_with("Recovery message");
    assert_eq!(recovered.to_string(), "Recovery message");
}

#[test]
fn test_state_result_map() {
    let result: StateResult<i32> = Ok(42);
    let mapped = result.map(|x| x * 2);
    assert_eq!(mapped.unwrap(), 84);
}

#[test]
fn test_state_result_map_err() {
    let result: StateResult<i32> = Err(StateError::new("Original error"));
    let mapped = result.map_err(|e| StateError::new("Mapped error"));
    assert_eq!(mapped.unwrap_err().to_string(), "Mapped error");
}

#[test]
fn test_state_result_and_then() {
    let result: StateResult<i32> = Ok(42);
    let chained = result.and_then(|x| Ok(x * 2));
    assert_eq!(chained.unwrap(), 84);
}

#[test]
fn test_state_result_or_else() {
    let result: StateResult<i32> = Err(StateError::new("Error"));
    let fallback = result.or_else(|_| Ok(42));
    assert_eq!(fallback.unwrap(), 42);
}

#[test]
fn test_state_error_hash() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    let error = StateError::new("Test error");
    
    map.insert(error.clone(), "value");
    assert_eq!(map.get(&error), Some(&"value"));
}

#[test]
fn test_state_error_ordering() {
    let error1 = StateError::new("A");
    let error2 = StateError::new("B");
    let error3 = StateError::new("A");
    
    assert!(error1 < error2);
    assert!(error2 > error1);
    assert!(error1 == error3);
}
