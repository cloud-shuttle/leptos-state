# 🧪 Test Coverage - Fix Testing Infrastructure

## Overview
Legacy unit tests won't compile and test coverage has significant gaps. Fix testing infrastructure and achieve comprehensive coverage.

## Current Test Issues

### 1. Compilation Failures
**Issue:** Legacy tests reference removed/changed APIs
```bash
cargo test --workspace 2>&1 | grep "error\|failed"
```

**Common Failure Patterns:**
- Import paths don't match current module structure
- Function signatures have changed
- Mock implementations reference removed traits
- Test utilities use deprecated APIs

### 2. Test Infrastructure Gaps

**Missing Test Categories:**
- Integration tests for core workflows
- WASM-specific functionality tests
- Browser localStorage tests
- State machine property-based tests
- Performance regression tests

### 3. Coverage Analysis

**Current Test Structure:**
```
tests/
├── rust/
│   ├── integration/     # Some integration tests exist
│   └── unit/           # Minimal unit tests
└── web/               # Playwright tests exist but limited
```

**Coverage Gaps Identified:**
- Store persistence mechanisms
- State machine edge cases
- Error handling paths
- Async store functionality
- Middleware system
- Visualization features

## Test Fix Strategy

### Phase 1: Make Tests Compile (Week 1)

#### Update Import Paths
```rust
// Old imports that fail
use leptos_state::machine::Machine;
use leptos_state::store::Store;

// Updated imports
use leptos_state::{Machine, Store};
use leptos_state::machine::builder::MachineBuilder;
```

#### Fix Test Signatures
```rust
// Old test pattern
#[test]
fn test_store_creation() {
    let store = Store::new(CounterState::default());
    // Test implementation
}

// Updated test pattern
#[test]
fn test_store_creation() {
    let store = create_store(CounterState::default());
    // Updated test implementation
}
```

#### Update Mock Implementations
```rust
// Remove references to deleted traits
// Update to match current API signatures
// Add missing trait implementations
```

### Phase 2: Core Functionality Tests (Week 2)

#### Store Testing
```rust
mod store_tests {
    use super::*;
    use leptos_state::*;
    
    #[test]
    fn store_creation_and_updates() {
        // Test store creation
        // Test state updates
        // Test subscription mechanics
    }
    
    #[test]
    fn store_persistence() {
        // Test localStorage integration
        // Test serialization/deserialization
        // Test error handling
    }
    
    #[wasm_bindgen_test]
    fn store_browser_persistence() {
        // Browser-specific localStorage tests
    }
}
```

#### State Machine Testing
```rust
mod machine_tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn basic_state_transitions() {
        // Test state creation
        // Test transition logic
        // Test guard evaluation
    }
    
    proptest! {
        #[test]
        fn state_machine_invariants(
            states in prop::collection::vec(".*", 1..10),
            transitions in prop::collection::vec((".*", ".*"), 0..20)
        ) {
            // Property-based testing for state machines
            // Verify invariants hold under random inputs
        }
    }
    
    #[test]
    fn hierarchical_states() {
        // Test nested state functionality
        // Test parent-child relationships
    }
}
```

### Phase 3: Integration Tests (Week 3)

#### End-to-End Workflows
```rust
mod integration_tests {
    use super::*;
    
    #[test]
    fn counter_app_workflow() {
        // Test complete counter example
        // Verify state updates propagate
        // Test persistence across sessions
    }
    
    #[test]
    fn todo_app_workflow() {
        // Test todo creation/deletion
        // Test filtering functionality
        // Test persistence
    }
    
    #[test]
    fn traffic_light_workflow() {
        // Test state machine transitions
        // Test guard conditions
        // Test visualization generation
    }
}
```

#### WASM Integration Tests
```rust
mod wasm_tests {
    use wasm_bindgen_test::*;
    
    #[wasm_bindgen_test]
    fn browser_localStorage_integration() {
        // Test actual browser localStorage
        // Test error handling in browser
    }
    
    #[wasm_bindgen_test]
    fn performance_in_browser() {
        // Performance benchmarks
        // Memory usage tests
    }
}
```

### Phase 4: Advanced Testing (Week 4)

#### Property-Based Testing
```rust
use proptest::prelude::*;

// Generate valid state machines
fn arb_machine() -> impl Strategy<Value = Machine> {
    prop::collection::vec("state_[a-z]+", 1..10)
        .prop_flat_map(|states| {
            let transitions = prop::collection::vec(
                (prop::sample::select(states.clone()), prop::sample::select(states)),
                0..states.len() * 2
            );
            (Just(states), transitions)
        })
        .prop_map(|(states, transitions)| {
            let mut builder = MachineBuilder::new();
            for state in states {
                builder = builder.state(&state);
            }
            for (from, to) in transitions {
                builder = builder.transition(&from, &to);
            }
            builder.build()
        })
}

proptest! {
    #[test]
    fn machine_never_enters_invalid_state(machine in arb_machine()) {
        // Property: Machine should never enter a state that doesn't exist
    }
}
```

#### Performance Tests
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_store_updates(c: &mut Criterion) {
    let store = create_store(CounterState::default());
    
    c.bench_function("store_update", |b| {
        b.iter(|| {
            store.update(|state| {
                state.count = black_box(state.count + 1);
            });
        })
    });
}

criterion_group!(benches, benchmark_store_updates);
criterion_main!(benches);
```

## Test Infrastructure Setup

### Test Utilities Module
```rust
// tests/common/mod.rs
pub mod fixtures;
pub mod assertions;
pub mod helpers;

// Common test data
pub fn sample_counter_store() -> CounterStore { /* */ }
pub fn sample_traffic_light_machine() -> Machine { /* */ }

// Custom assertions
pub fn assert_state_transition(machine: &Machine, from: &str, to: &str) { /* */ }
pub fn assert_store_persisted(store: &Store<T>) { /* */ }
```

### Mock Implementations
```rust
// tests/mocks/mod.rs
pub struct MockStorage;
impl PersistenceBackend for MockStorage { /* */ }

pub struct MockMiddleware;
impl Middleware for MockMiddleware { /* */ }
```

### Test Configuration
```toml
# Cargo.toml test configuration
[dev-dependencies]
wasm-bindgen-test = { workspace = true }
criterion = { workspace = true, features = ["html_reports"] }
proptest = { workspace = true }
quickcheck = { workspace = true }
rstest = { workspace = true }
tokio-test = { workspace = true }
```

## Test Organization Strategy

### Directory Structure
```
tests/
├── rust/
│   ├── unit/
│   │   ├── store/
│   │   │   ├── basic_ops.rs
│   │   │   ├── persistence.rs
│   │   │   └── async_store.rs
│   │   ├── machine/
│   │   │   ├── transitions.rs
│   │   │   ├── guards.rs
│   │   │   └── hierarchical.rs
│   │   └── middleware/
│   │       ├── logger.rs
│   │       └── validation.rs
│   ├── integration/
│   │   ├── examples/
│   │   │   ├── counter.rs
│   │   │   ├── todo_app.rs
│   │   │   └── traffic_light.rs
│   │   └── workflows/
│   │       ├── persistence_workflows.rs
│   │       └── state_machine_workflows.rs
│   └── property/
│       ├── machine_properties.rs
│       └── store_properties.rs
├── web/
│   ├── browser_tests.rs
│   ├── localStorage_tests.rs
│   └── performance_tests.rs
├── benchmarks/
│   ├── store_benchmarks.rs
│   └── machine_benchmarks.rs
└── common/
    ├── fixtures.rs
    ├── assertions.rs
    └── mocks.rs
```

### Testing Framework Standards

#### Test Naming Convention
```rust
#[test]
fn should_update_counter_when_increment_called() { }

#[test] 
fn should_return_error_when_invalid_state_transition() { }

#[test]
fn should_persist_state_to_localStorage_when_configured() { }
```

#### Test Categories
```rust
// Unit tests - test individual functions
#[cfg(test)]
mod unit_tests { }

// Integration tests - test component interactions
#[cfg(test)]
mod integration_tests { }

// Property tests - test invariants
#[cfg(test)]
mod property_tests { }

// Performance tests - benchmarking
#[cfg(test)]
mod performance_tests { }

// Browser tests - WASM functionality
#[cfg(target_arch = "wasm32")]
mod browser_tests { }
```

## Coverage Goals and Metrics

### Target Coverage Levels
- **Unit Tests:** 95% line coverage
- **Integration Tests:** 90% workflow coverage
- **Property Tests:** All invariants tested
- **Browser Tests:** All WASM functionality tested

### Coverage Measurement
```bash
# Install coverage tools
cargo install cargo-tarpaulin

# Run coverage analysis
cargo tarpaulin --out Html --output-dir coverage

# WASM coverage (requires different approach)
wasm-pack test --chrome --headless
```

### Coverage Tracking
```rust
// Add to each module
#[cfg(test)]
mod tests {
    use super::*;
    
    // Ensure all public functions have tests
    // Ensure all error paths are tested
    // Ensure all feature flags are tested
}
```

## Test Automation

### CI/CD Integration
```yaml
# .github/workflows/test.yml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run Rust tests
        run: cargo test --workspace
      - name: Run WASM tests  
        run: wasm-pack test --chrome --headless
      - name: Run property tests
        run: cargo test --release -- --ignored
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

### Pre-commit Hooks
```bash
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: cargo-test
        name: cargo test
        entry: cargo test --workspace
        language: system
        pass_filenames: false
```

## Success Metrics

### Compilation Success
- [ ] `cargo test --workspace` compiles without errors
- [ ] All test modules import correctly
- [ ] No deprecated API usage in tests

### Coverage Achievement
- [ ] 95%+ unit test coverage
- [ ] 90%+ integration test coverage  
- [ ] All public APIs have tests
- [ ] All error paths tested

### Test Quality
- [ ] Property-based tests for core invariants
- [ ] Performance regression tests
- [ ] Browser compatibility tests
- [ ] Comprehensive error scenario tests

### CI/CD Integration
- [ ] Automated test runs on all PRs
- [ ] Coverage reporting integrated
- [ ] Performance benchmarks tracked
- [ ] WASM tests running in browser

**Next Steps:** After test coverage fixes, proceed to DEPENDENCY_UPDATE.md
