# 🧪 Testing Guide

This guide covers testing strategies, best practices, and tools for the `leptos-state` library.

## 🎯 **Testing Philosophy**

### **Core Principles**
- **Comprehensive Coverage**: Test all public APIs and edge cases
- **Property-Based Testing**: Use generative testing for complex logic
- **Performance Testing**: Ensure no performance regressions
- **Integration Testing**: Test features work together correctly
- **Documentation Testing**: Ensure examples compile and run

### **Testing Pyramid**
```
    🔺 E2E Tests (Few, Slow)
   🔺🔺 Integration Tests (Some, Medium)
  🔺🔺🔺 Unit Tests (Many, Fast)
```

## 🚀 **Getting Started**

### **Running Tests**
```bash
# Run all tests
cargo test

# Run with specific features
cargo test --features "testing,persist,devtools"

# Run specific test modules
cargo test --package leptos-state --lib

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4
```

### **Running Benchmarks**
```bash
# Run all benchmarks
cargo bench

# Run specific benchmarks
cargo bench --bench performance_benchmarks

# Run benchmarks with specific features
cargo bench --features "testing,persist"
```

### **Code Coverage**
```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run coverage analysis
cargo tarpaulin --features "testing,persist,devtools"

# Generate HTML report
cargo tarpaulin --features "testing,persist,devtools" --out Html
```

## 🧪 **Unit Testing**

### **Test Structure**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test data setup
    fn create_test_data() -> TestData {
        TestData::default()
    }
    
    // Basic functionality tests
    #[test]
    fn test_basic_functionality() {
        let data = create_test_data();
        let result = function_under_test(data);
        assert!(result.is_ok());
    }
    
    // Edge case tests
    #[test]
    fn test_edge_cases() {
        // Test boundary conditions
        let edge_data = TestData::edge_case();
        let result = function_under_test(edge_data);
        assert!(result.is_ok());
    }
    
    // Error condition tests
    #[test]
    fn test_error_conditions() {
        let invalid_data = TestData::invalid();
        let result = function_under_test(invalid_data);
        assert!(result.is_err());
    }
}
```

### **Test Organization**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Group related tests
    mod basic_operations {
        use super::*;
        
        #[test]
        fn test_create() { /* ... */ }
        
        #[test]
        fn test_read() { /* ... */ }
        
        #[test]
        fn test_update() { /* ... */ }
        
        #[test]
        fn test_delete() { /* ... */ }
    }
    
    mod edge_cases {
        use super::*;
        
        #[test]
        fn test_empty_input() { /* ... */ }
        
        #[test]
        fn test_large_input() { /* ... */ }
        
        #[test]
        fn test_invalid_input() { /* ... */ }
    }
    
    mod error_handling {
        use super::*;
        
        #[test]
        fn test_validation_errors() { /* ... */ }
        
        #[test]
        fn test_system_errors() { /* ... */ }
        
        #[test]
        fn test_recovery_from_errors() { /* ... */ }
    }
}
```

### **Test Utilities**
```rust
#[cfg(test)]
mod test_utils {
    use super::*;
    
    // Helper functions for creating test data
    pub fn create_test_machine() -> Machine<TestContext, TestEvent, TestState> {
        Machine::new(TestState::Initial, TestContext::default())
    }
    
    pub fn create_test_context() -> TestContext {
        TestContext {
            counter: 0,
            name: "test".to_string(),
        }
    }
    
    // Helper functions for assertions
    pub fn assert_state_machine_valid(machine: &Machine<TestContext, TestEvent, TestState>) {
        assert!(machine.is_valid());
        assert!(machine.state_count() > 0);
    }
    
    // Helper functions for cleanup
    pub fn cleanup_test_data() {
        // Clean up any test data
    }
}
```

## 🔄 **Property-Based Testing**

### **Using Proptest**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_state_machine_properties(
        events in prop::collection::vec(any::<Event>(), 0..100)
    ) {
        let machine = create_test_machine();
        let mut state = machine.initial_state();
        
        for event in events {
            if machine.can_transition(&state, event.clone()) {
                state = machine.transition(&state, event);
                assert!(machine.is_valid_state(&state));
            }
        }
    }
    
    #[test]
    fn test_store_properties(
        operations in prop::collection::vec(any::<Operation>(), 0..50)
    ) {
        let mut store = TestStore::default();
        
        for operation in operations {
            store.apply(operation);
            assert!(store.is_valid());
        }
    }
}
```

### **Custom Generators**
```rust
use proptest::prelude::*;

// Custom generator for complex types
fn arb_state_machine() -> impl Strategy<Value = Machine<TestContext, TestEvent, TestState>> {
    (any::<TestState>(), any::<TestContext>())
        .prop_map(|(state, context)| Machine::new(state, context))
}

// Custom generator for events
fn arb_events() -> impl Strategy<Value = Vec<TestEvent>> {
    prop::collection::vec(any::<TestEvent>(), 0..100)
}

proptest! {
    #[test]
    fn test_with_custom_generators(
        machine in arb_state_machine(),
        events in arb_events()
    ) {
        // Test implementation using custom generators
    }
}
```

### **Property Testing Best Practices**
```rust
proptest! {
    #[test]
    fn test_state_machine_invariants(
        machine in arb_state_machine(),
        events in arb_events()
    ) {
        // Test that invariants are maintained
        let mut current_state = machine.initial_state();
        
        for event in events {
            // Invariant 1: State is always valid
            assert!(machine.is_valid_state(&current_state));
            
            // Invariant 2: Transitions maintain validity
            if machine.can_transition(&current_state, event.clone()) {
                current_state = machine.transition(&current_state, event);
                assert!(machine.is_valid_state(&current_state));
            }
            
            // Invariant 3: Context remains consistent
            assert!(machine.context().is_valid());
        }
    }
}
```

## 📊 **Performance Testing**

### **Benchmarking with Criterion**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_state_transitions(c: &mut Criterion) {
    let machine = create_test_machine();
    let events = create_test_events(1000);
    
    c.bench_function("state_transitions", |b| {
        b.iter(|| {
            let mut state = machine.initial_state();
            for event in &events {
                if machine.can_transition(&state, event.clone()) {
                    state = machine.transition(&state, event.clone());
                }
            }
            black_box(state);
        });
    });
}

fn benchmark_store_operations(c: &mut Criterion) {
    let mut store = TestStore::default();
    let operations = create_test_operations(1000);
    
    c.bench_function("store_operations", |b| {
        b.iter(|| {
            for operation in &operations {
                store.apply(operation.clone());
            }
            black_box(store.clone());
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.01);
    targets =
        benchmark_state_transitions,
        benchmark_store_operations
}
criterion_main!(benches);
```

### **Performance Regression Testing**
```rust
#[test]
fn test_performance_regression() {
    let start = std::time::Instant::now();
    
    // Perform operations
    let machine = create_test_machine();
    for _ in 0..1000 {
        let _ = machine.transition(&machine.current_state(), TestEvent::Next);
    }
    
    let duration = start.elapsed();
    
    // Ensure performance hasn't regressed
    // This threshold should be updated based on performance improvements
    assert!(duration.as_millis() < 100, "Performance regression detected: {:?}", duration);
}
```

### **Memory Usage Testing**
```rust
#[test]
fn test_memory_usage() {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct TestAllocator {
        allocated: AtomicUsize,
    }
    
    unsafe impl GlobalAlloc for TestAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            self.allocated.fetch_add(layout.size(), Ordering::SeqCst);
            System.alloc(layout)
        }
        
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            self.allocated.fetch_sub(layout.size(), Ordering::SeqCst);
            System.dealloc(ptr, layout);
        }
    }
    
    #[global_allocator]
    static ALLOCATOR: TestAllocator = TestAllocator {
        allocated: AtomicUsize::new(0),
    };
    
    let initial_memory = ALLOCATOR.allocated.load(Ordering::SeqCst);
    
    // Perform operations
    let machine = create_test_machine();
    for _ in 0..100 {
        let _ = machine.transition(&machine.current_state(), TestEvent::Next);
    }
    
    let final_memory = ALLOCATOR.allocated.load(Ordering::SeqCst);
    let memory_used = final_memory - initial_memory;
    
    // Ensure memory usage is reasonable
    assert!(memory_used < 1024 * 1024, "Excessive memory usage: {} bytes", memory_used);
}
```

## 🔗 **Integration Testing**

### **Feature Integration Tests**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_persistence_with_state_machine() {
        // Test that persistence works with state machines
        let machine = create_test_machine();
        let persistence_manager = PersistenceManager::new();
        
        // Save state
        persistence_manager.save("test_machine", &machine).unwrap();
        
        // Load state
        let loaded_machine = persistence_manager.load("test_machine").unwrap();
        
        // Verify state is preserved
        assert_eq!(machine.current_state(), loaded_machine.current_state());
        assert_eq!(machine.context(), loaded_machine.context());
    }
    
    #[test]
    fn test_devtools_integration() {
        // Test that DevTools work with all features
        let machine = create_test_machine();
        let store = create_test_store();
        
        // Verify DevTools can inspect both
        let devtools = DevTools::new();
        assert!(devtools.can_inspect(&machine));
        assert!(devtools.can_inspect(&store));
    }
}
```

### **Cross-Feature Tests**
```rust
#[test]
fn test_all_features_together() {
    // Test that all features work together
    let machine = create_test_machine();
    let store = create_test_store();
    let persistence_manager = PersistenceManager::new();
    let devtools = DevTools::new();
    
    // Test state machine operations
    let new_state = machine.transition(&machine.current_state(), TestEvent::Next);
    assert!(machine.is_valid_state(&new_state));
    
    // Test store operations
    store.update(|state| state.counter += 1);
    assert_eq!(store.get().counter, 1);
    
    // Test persistence
    persistence_manager.save("test_data", &store).unwrap();
    let loaded_store = persistence_manager.load("test_data").unwrap();
    assert_eq!(store.get().counter, loaded_store.get().counter);
    
    // Test DevTools
    assert!(devtools.can_inspect(&machine));
    assert!(devtools.can_inspect(&store));
}
```

## 📚 **Documentation Testing**

### **Example Code Testing**
```rust
/// # Examples
///
/// ```rust
/// use leptos_state::v1::*;
///
/// let machine = Machine::new(InitialState, Context::default());
/// machine.send(Event::Start)?;
/// ```
pub struct Machine<C, E, S> {
    // Implementation
}

#[cfg(test)]
mod doc_tests {
    use super::*;
    
    #[test]
    fn test_examples_compile() {
        // This test ensures that all examples in documentation compile
        // It's automatically run by cargo test
    }
}
```

### **API Documentation Tests**
```rust
#[test]
fn test_api_documentation() {
    // Test that all public APIs are documented
    let public_items = get_public_items();
    
    for item in public_items {
        assert!(
            has_documentation(item),
            "Public item {} is not documented",
            item.name()
        );
        
        if item.has_examples() {
            assert!(
                examples_compile(item),
                "Examples for {} do not compile",
                item.name()
            );
        }
    }
}
```

## 🧹 **Test Maintenance**

### **Test Data Management**
```rust
#[cfg(test)]
mod test_data {
    use super::*;
    
    // Centralized test data creation
    pub fn create_test_machine() -> Machine<TestContext, TestEvent, TestState> {
        Machine::new(TestState::Initial, TestContext::default())
    }
    
    pub fn create_test_events(count: usize) -> Vec<TestEvent> {
        (0..count).map(|i| TestEvent::Custom(i)).collect()
    }
    
    pub fn create_test_context() -> TestContext {
        TestContext {
            counter: 0,
            name: "test".to_string(),
        }
    }
    
    // Test data cleanup
    pub fn cleanup_test_data() {
        // Clean up any persistent test data
        if let Ok(entries) = std::fs::read_dir("test_data") {
            for entry in entries.flatten() {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
}
```

### **Test Configuration**
```rust
#[cfg(test)]
mod test_config {
    use super::*;
    
    // Test configuration
    pub struct TestConfig {
        pub test_data_dir: String,
        pub max_test_iterations: usize,
        pub performance_threshold_ms: u128,
    }
    
    impl Default for TestConfig {
        fn default() -> Self {
            Self {
                test_data_dir: "test_data".to_string(),
                max_test_iterations: 1000,
                performance_threshold_ms: 100,
            }
        }
    }
    
    // Global test configuration
    lazy_static! {
        pub static ref TEST_CONFIG: TestConfig = TestConfig::default();
    }
}
```

## 🚨 **Common Testing Issues**

### **Flaky Tests**
```rust
// Avoid flaky tests by using deterministic data
#[test]
fn test_deterministic_behavior() {
    // Use fixed seed for random number generation
    let mut rng = StdRng::seed_from_u64(42);
    
    // Generate deterministic test data
    let events: Vec<TestEvent> = (0..100)
        .map(|_| rng.gen())
        .collect();
    
    // Test with deterministic data
    let machine = create_test_machine();
    let mut state = machine.initial_state();
    
    for event in events {
        if machine.can_transition(&state, event.clone()) {
            state = machine.transition(&state, event);
        }
    }
    
    // This should always produce the same result
    assert_eq!(state, expected_final_state);
}
```

### **Slow Tests**
```rust
// Use smaller datasets for faster tests
#[test]
fn test_fast_operations() {
    // Use smaller dataset for fast tests
    let events = create_test_events(10); // Small dataset
    
    let machine = create_test_machine();
    let mut state = machine.initial_state();
    
    for event in events {
        if machine.can_transition(&state, event.clone()) {
            state = machine.transition(&state, event);
        }
    }
    
    assert!(machine.is_valid_state(&state));
}

// Separate slow tests into their own module
#[cfg(test)]
mod slow_tests {
    use super::*;
    
    #[test]
    #[ignore] // Ignore by default, run with --ignored
    fn test_large_dataset() {
        // This test uses a large dataset and may be slow
        let events = create_test_events(10000);
        // ... test implementation
    }
}
```

### **Resource Cleanup**
```rust
// Ensure proper cleanup in tests
#[test]
fn test_with_cleanup() {
    // Set up test data
    let test_file = "test_file.txt";
    std::fs::write(test_file, "test data").unwrap();
    
    // Ensure cleanup happens even if test fails
    defer! {
        let _ = std::fs::remove_file(test_file);
    };
    
    // Test implementation
    let result = function_under_test(test_file);
    assert!(result.is_ok());
    
    // Cleanup will happen automatically
}
```

## 📈 **Test Metrics**

### **Coverage Tracking**
```bash
# Generate coverage report
cargo tarpaulin --features "testing,persist,devtools" --out Html

# View coverage in browser
open target/tarpaulin/html/index.html
```

### **Performance Tracking**
```bash
# Run benchmarks and save results
cargo bench --bench performance_benchmarks -- --save-baseline main

# Compare with previous results
cargo bench --bench performance_benchmarks -- --baseline main
```

### **Test Statistics**
```bash
# Run tests with statistics
cargo test --features "testing,persist,devtools" -- --test-threads=1 --nocapture

# Generate test report
cargo test --features "testing,persist,devtools" -- --format=json | jq '.'
```

## 🎯 **Testing Checklist**

### **Before Committing**
- [ ] All tests pass locally
- [ ] New functionality has tests
- [ ] Tests cover edge cases
- [ ] Performance tests pass
- [ ] Documentation examples compile

### **Before Release**
- [ ] All tests pass in CI
- [ ] Coverage meets targets
- [ ] Performance benchmarks pass
- [ ] Integration tests pass
- [ ] Documentation tests pass

### **Regular Maintenance**
- [ ] Update test data generators
- [ ] Review and update performance thresholds
- [ ] Clean up old test data
- [ ] Update test documentation
- [ ] Review test coverage reports

---

**Remember: Good tests are the foundation of reliable software. Write tests that are fast, reliable, and maintainable.** 🧪✨
