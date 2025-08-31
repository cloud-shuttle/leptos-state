//! Unit tests for store module

use leptos_state::{
    store::{Store, StoreSlice},
    utils::StateResult,
};
use tests_common::{TestContext, TestState, TestStore};

#[test]
fn test_store_creation() {
    let state = TestStore::create();
    assert_state!(state, TestState::Idle);
}

#[test]
fn test_store_clone() {
    let store = TestStore;
    let cloned = store.clone();
    assert_eq!(TestStore::create(), cloned.create());
}

#[test]
fn test_store_slice() {
    let state = TestState::Counting;
    
    struct StateSlice;
    
    impl StoreSlice<TestStore> for StateSlice {
        type Output = String;
        
        fn select(state: &TestState) -> Self::Output {
            match state {
                TestState::Idle => "idle".to_string(),
                TestState::Counting => "counting".to_string(),
                TestState::Error => "error".to_string(),
            }
        }
    }
    
    let slice_result = StateSlice::select(&state);
    assert_eq!(slice_result, "counting");
}

#[test]
fn test_store_context() {
    use leptos_state::store::StoreContext;
    
    let context = StoreContext::new(TestState::Idle);
    assert_state!(*context.read.get(), TestState::Idle);
}

#[test]
fn test_store_validation() {
    use leptos_state::store::ValidationMiddleware;
    
    let validator = |context: &TestContext| context.counter >= 0;
    let middleware = ValidationMiddleware::new(validator);
    
    let valid_context = TestContext {
        counter: 10,
        name: "valid".to_string(),
        is_active: true,
    };
    
    let next = Box::new(|state: &TestContext| state.clone());
    let wrapped = middleware.wrap(next);
    
    let result = wrapped(&valid_context);
    assert_eq!(result, valid_context);
}

#[test]
fn test_store_error_handling() {
    use leptos_state::utils::StateError;
    
    let error = StateError::store_not_found("test_store");
    assert!(error.to_string().contains("test_store"));
    
    let error = StateError::invalid_transition("idle", "counting");
    assert!(error.to_string().contains("idle"));
    assert!(error.to_string().contains("counting"));
}
