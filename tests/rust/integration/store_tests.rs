//! Integration tests for store functionality

use leptos::prelude::*;
use leptos_state::*;
use wasm_bindgen_test::*;

use super::fixtures::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn store_creation_and_access() {
    // Runtime not needed in Leptos 0.8
    
    provide_store::<TestStore>(TestState::default());
    let (state, _) = use_store::<TestStore>();
    
    assert_eq!(state.get().count, 0);
    assert_eq!(state.get().name, "test");
    assert_eq!(state.get().enabled, true);
}

#[wasm_bindgen_test]
fn store_state_updates() {
    // Runtime not needed in Leptos 0.8
    
    provide_store::<TestStore>(TestState::default());
    let (state, set_state) = use_store::<TestStore>();
    
    // Test state mutation
    set_state.update(|s| {
        s.count = 42;
        s.name = "updated".to_string();
        s.enabled = false;
    });
    
    assert_eq!(state.get().count, 42);
    assert_eq!(state.get().name, "updated");
    assert_eq!(state.get().enabled, false);
}

#[wasm_bindgen_test]
fn store_reactivity() {
    // Runtime not needed in Leptos 0.8
    
    provide_store::<TestStore>(TestState::default());
    let (state, set_state) = use_store::<TestStore>();
    let (trigger_effect, effect_count) = track_effect_count();
    
    Effect::new(move |_| {
        state.get(); // Subscribe to state changes
        trigger_effect();
    });
    
    // Initial effect execution
    assert_eq!(effect_count.get(), 1);
    
    // Update state should trigger effect
    set_state.update(|s| s.count += 1);
    assert_eq!(effect_count.get(), 2);
    
    // Another update
    set_state.update(|s| s.name = "changed".to_string());
    assert_eq!(effect_count.get(), 3);
}

#[wasm_bindgen_test]
fn computed_state_selectors() {
    // Runtime not needed in Leptos 0.8
    
    provide_store::<TestStore>(TestState::default());
    let (state, set_state) = use_store::<TestStore>();
    
    let doubled_count = create_computed::<TestStore, _>(|s| s.count * 2);
    let name_length = create_computed::<TestStore, _>(|s| s.name.len());
    
    assert_eq!(doubled_count.get(), 0); // 0 * 2
    assert_eq!(name_length.get(), 4); // "test".len()
    
    set_state.update(|s| {
        s.count = 10;
        s.name = "hello world".to_string();
    });
    
    assert_eq!(doubled_count.get(), 20); // 10 * 2
    assert_eq!(name_length.get(), 11); // "hello world".len()
}

#[cfg(feature = "persist")]
#[wasm_bindgen_test]
fn store_persistence() {
    use leptos_state::store::{save_to_storage, load_from_storage};
    
    let test_state = TestState {
        count: 42,
        name: "persisted".to_string(),
        enabled: false,
    };
    
    // Save state
    let result = save_to_storage("test_key", &test_state);
    assert!(result.is_ok());
    
    // Load state
    let loaded: Result<TestState, _> = load_from_storage("test_key");
    assert!(loaded.is_ok());
    
    let loaded_state = loaded.unwrap();
    assert_eq!(loaded_state.count, 42);
    assert_eq!(loaded_state.name, "persisted");
    assert_eq!(loaded_state.enabled, false);
}

#[test]
fn store_middleware_chain() {
    // Runtime not needed in Leptos 0.8
    
    // Test that middleware can be created without runtime errors
    let logger = LoggerMiddleware::<TestStore>::new("test");
    let validator = ValidationMiddleware::new(|s: &TestState| s.count >= 0);
    
    let chain = MiddlewareChain::<TestStore>::new()
        .add(Box::new(logger))
        .add(Box::new(validator));
    
    // Test middleware application
    let wrapped = chain.apply(|state| TestState {
        count: state.count + 1,
        ..state.clone()
    });
    
    let initial_state = TestState::default();
    let result = wrapped(&initial_state);
    
    assert_eq!(result.count, 1);
    assert_eq!(result.name, "test");
}