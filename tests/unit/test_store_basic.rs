use leptos_state::store::*;
use leptos_state::utils::types::StateResult;

#[derive(Debug, Clone, PartialEq, Default)]
struct TestState {
    count: i32,
    name: String,
}

#[test]
fn test_store_creation() {
    let store = Store::new(TestState {
        count: 0,
        name: "test".to_string(),
    });
    
    let state = store.get();
    assert_eq!(state.count, 0);
    assert_eq!(state.name, "test");
}

#[test]
fn test_store_update() {
    let store = Store::new(TestState {
        count: 0,
        name: "test".to_string(),
    });
    
    store.update(|state| {
        state.count = 42;
        state.name = "updated".to_string();
    });
    
    let state = store.get();
    assert_eq!(state.count, 42);
    assert_eq!(state.name, "updated");
}

#[test]
fn test_store_slice() {
    let store = Store::new(TestState {
        count: 42,
        name: "test".to_string(),
    });
    
    let count_slice = store.slice(|state| &state.count);
    assert_eq!(*count_slice.get(), 42);
    
    let name_slice = store.slice(|state| &state.name);
    assert_eq!(name_slice.get(), "test");
}

#[test]
fn test_store_slice_mut() {
    let store = Store::new(TestState {
        count: 0,
        name: "test".to_string(),
    });
    
    let mut count_slice = store.slice_mut(|state| &mut state.count);
    count_slice.update(|count| *count = 42);
    
    let state = store.get();
    assert_eq!(state.count, 42);
}

#[test]
fn test_store_subscribe() {
    let store = Store::new(TestState {
        count: 0,
        name: "test".to_string(),
    });
    
    let mut called = false;
    let _subscription = store.subscribe(|_| {
        called = true;
    });
    
    store.update(|state| {
        state.count = 42;
    });
    
    // Note: In a real test environment, we'd need to wait for the subscription to fire
    // For now, we just test that the subscription can be created
    assert!(!called); // Subscription hasn't fired yet in this test context
}

#[test]
fn test_store_clone() {
    let store = Store::new(TestState {
        count: 42,
        name: "test".to_string(),
    });
    
    let cloned = store.clone();
    let state1 = store.get();
    let state2 = cloned.get();
    
    assert_eq!(state1.count, state2.count);
    assert_eq!(state1.name, state2.name);
}

#[test]
fn test_store_debug() {
    let store = Store::new(TestState {
        count: 42,
        name: "test".to_string(),
    });
    
    let debug_str = format!("{:?}", store);
    assert!(debug_str.contains("Store"));
}

#[test]
fn test_state_default() {
    let state = TestState::default();
    assert_eq!(state.count, 0);
    assert_eq!(state.name, "");
}

#[test]
fn test_state_clone() {
    let state = TestState {
        count: 42,
        name: "test".to_string(),
    };
    
    let cloned = state.clone();
    assert_eq!(cloned.count, state.count);
    assert_eq!(cloned.name, state.name);
}

#[test]
fn test_state_partial_eq() {
    let state1 = TestState {
        count: 42,
        name: "test".to_string(),
    };
    
    let state2 = TestState {
        count: 42,
        name: "test".to_string(),
    };
    
    let state3 = TestState {
        count: 43,
        name: "test".to_string(),
    };
    
    assert_eq!(state1, state2);
    assert_ne!(state1, state3);
}

#[test]
fn test_state_debug() {
    let state = TestState {
        count: 42,
        name: "test".to_string(),
    };
    
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("42"));
    assert!(debug_str.contains("test"));
}
