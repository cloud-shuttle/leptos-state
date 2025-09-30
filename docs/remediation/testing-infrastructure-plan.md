# 🧪 Testing Infrastructure Plan - September 20, 2025

## Executive Summary

**Current Status**: Core library compiles, tests fail compilation
**Goal**: Comprehensive testing framework with 95%+ coverage
**Timeline**: 4-5 days for complete testing infrastructure
**Success Criteria**: All test types working, high coverage, CI/CD integration

## Current Testing Status

### ✅ Working Components
- Core library compiles successfully
- Basic test framework infrastructure exists
- Some unit tests defined (but failing compilation)

### ❌ Broken/Missing Components
- Test compilation fails (1767 errors)
- No integration tests
- No property-based testing
- No WASM/browser testing
- No performance regression tests
- No API contract testing

## Comprehensive Testing Strategy

### 1. Test Categories & Coverage Goals

| Test Type | Current Status | Target Coverage | Priority |
|-----------|----------------|-----------------|----------|
| Unit Tests | Partial (compilation broken) | 90%+ | 🔴 CRITICAL |
| Integration Tests | Missing | 85%+ | 🔴 CRITICAL |
| Property Tests | Missing | 75%+ | 🟡 HIGH |
| WASM Tests | Missing | 80%+ | 🟡 HIGH |
| Performance Tests | Missing | 100% | 🟡 MEDIUM |
| Contract Tests | Missing | 100% | 🟡 MEDIUM |

### 2. Testing Pyramid Structure

```
┌─────────────────────────────────┐  100 total tests
│    Contract Tests (20%)         │  API stability guarantees
│    ┌─────────────────────────┐   │
│    │  Integration Tests      │   │  End-to-end workflows
│    │     (30%)               │   │
│    │  ┌───────────────────┐   │   │
│    │  │  Component Tests  │   │   │  Feature-level testing
│    │  │     (25%)         │   │   │
│    │  │  ┌─────────────┐   │   │   │
│    │  │  │ Unit Tests  │   │   │   │  Function/method level
│    │  │  │   (25%)     │   │   │   │
│    │  │  └─────────────┘   │   │   │
│    │  └───────────────────┘   │   │
│    └─────────────────────────┘   │
└─────────────────────────────────┘
```

## Implementation Plan

### Phase 1: Fix & Expand Unit Tests (Days 1-2)

#### 1. Fix Test Compilation
**Current Issue**: 1767 test compilation errors
**Root Cause**: Test code uses outdated APIs and missing trait bounds

**Fix Strategy**:
```rust
// Before: Broken test
#[derive(Debug, Clone, PartialEq)]
enum TestEvent { Start, Stop }

// After: Working test
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TestEvent { Start, Stop }
```

**Files to Fix**:
- `leptos-state/src/machine/machine.rs` - Test events missing traits
- `leptos-state/src/machine/machine_state_impl.rs` - Same issue
- `tests/rust/` - Integration test files with outdated APIs

#### 2. Core Unit Test Coverage
**Target**: 90% coverage of core functionality

**Test Structure**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(TestEvent::Start, "idle", "running")]
    #[case(TestEvent::Stop, "running", "idle")]
    fn machine_transitions(
        #[case] event: TestEvent,
        #[case] from_state: &str,
        #[case] to_state: &str,
    ) {
        let machine = create_test_machine();
        let initial_state = MachineStateImpl::with_value(StateValue::Simple(from_state.to_string()));
        let new_state = machine.transition(&initial_state, event);

        assert_eq!(new_state.value(), &StateValue::Simple(to_state.to_string()));
    }
}
```

### Phase 2: Integration Testing Framework (Day 3)

#### 1. End-to-End Example Testing
**Goal**: Validate complete user workflows work correctly

**Test Categories**:
```rust
mod integration_tests {
    use super::*;

    #[test]
    fn counter_app_workflow() {
        // Test complete counter example
        // 1. Create store with initial state
        // 2. Verify initial render
        // 3. Simulate user interactions
        // 4. Verify state updates propagate
        // 5. Test persistence across sessions
    }

    #[test]
    fn todo_app_workflow() {
        // Test todo creation/deletion
        // Test filtering functionality
        // Test persistence
        // Test complex state interactions
    }

    #[test]
    fn state_machine_traffic_light() {
        // Test complete traffic light state machine
        // Verify all transitions work
        // Test guard conditions
        // Test action execution
    }
}
```

#### 2. Cross-Component Integration
```rust
#[test]
fn store_machine_integration() {
    // Test store updates triggering machine transitions
    // Test machine state changes updating store
    // Test reactive propagation across components
}
```

### Phase 3: Property-Based Testing (Day 4)

#### 1. State Machine Invariants
**Use proptest to verify fundamental properties**:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn state_machine_never_enters_invalid_state(
        states in prop::collection::vec("[a-z]+", 1..10),
        transitions in prop::collection::vec(
            ("[a-z]+", "[a-z]+"),
            0..20
        )
    ) {
        let machine = create_machine_from_data(states, transitions);
        let mut state = machine.initial_state();

        // Perform random transitions
        for _ in 0..100 {
            let event = generate_random_event();
            state = machine.transition(&state, event);

            // Property: Machine should always be in a valid state
            prop_assert!(machine.get_states().contains(&state.value().to_string()));
        }
    }

    #[test]
    fn transitions_are_deterministic(
        machine in arb_machine(),
        state in arb_state(),
        event in arb_event()
    ) {
        // Property: Same input should always produce same output
        let result1 = machine.transition(&state, event.clone());
        let result2 = machine.transition(&state, event);

        prop_assert_eq!(result1.value(), result2.value());
    }
}
```

#### 2. Store Invariants
```rust
proptest! {
    #[test]
    fn store_updates_are_atomic(
        initial_state in arb_counter_state(),
        updates in prop::collection::vec(arb_update(), 1..10)
    ) {
        let store = create_store(initial_state);

        // Apply multiple updates
        for update in updates {
            store.update(update);
        }

        // Property: Store should never be in inconsistent state
        let state = store.get();
        prop_assert!(state.count >= 0); // Example invariant
    }
}
```

### Phase 4: WASM & Browser Testing (Day 5)

#### 1. WASM Compilation Tests
```rust
// tests/wasm/mod.rs
#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn machine_works_in_browser() {
        let machine = create_test_machine();
        let state = machine.initial_state();

        // Test basic functionality in browser environment
        assert_eq!(state.value().to_string(), "idle");
    }

    #[wasm_bindgen_test]
    fn localstorage_persistence() {
        // Test actual browser localStorage integration
        let store = create_persistent_store();

        // Set value
        store.update(|state| state.count = 42);

        // Simulate page reload (in real scenario)
        let new_store = create_persistent_store();

        // Verify persistence
        assert_eq!(new_store.get().count, 42);
    }
}
```

#### 2. Browser-Specific Features
```rust
#[wasm_bindgen_test]
fn reactive_updates_in_dom() {
    // Test that Leptos reactivity works in browser
    // Test component re-rendering on state changes
    // Test event handling in browser context
}
```

## Coverage Analysis & Tools

### 1. Coverage Measurement
```toml
# Cargo.toml additions
[dev-dependencies]
cargo-tarpaulin = "0.25"
grcov = "0.8"
```

**Coverage Targets by Module**:
```bash
# Generate coverage report
cargo tarpaulin --workspace --out Html

# Coverage goals:
# - Core types: 95%+
# - Machine logic: 90%+
# - Store system: 90%+
# - Hooks: 85%+
# - Utilities: 80%+
# - Overall: 90%+
```

### 2. Test Organization
```
tests/
├── unit/               # Unit tests by module
│   ├── machine/
│   ├── store/
│   └── hooks/
├── integration/        # End-to-end tests
├── property/          # Property-based tests
├── wasm/              # Browser-specific tests
└── contracts/         # API contract tests
```

### 3. Test Utilities
```rust
// tests/common/mod.rs
pub mod test_helpers {
    use leptos_state::*;

    pub fn create_test_machine() -> Machine<TestContext, TestEvent> {
        MachineBuilder::new()
            .initial("idle")
            .state("idle")
            .on(TestEvent::Start, "running")
            .state("running")
            .on(TestEvent::Stop, "idle")
            .build()
            .unwrap()
    }

    pub fn create_test_store() -> StoreContext<CounterState> {
        create_store(CounterState { count: 0 })
    }
}
```

## CI/CD Integration

### 1. GitHub Actions Workflow
```yaml
# .github/workflows/test.yml
name: Comprehensive Testing
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, beta]

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust ${{ matrix.rust }}
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}

      - name: Run Unit Tests
        run: cargo test --lib

      - name: Run Integration Tests
        run: cargo test --test integration

      - name: Run Property Tests
        run: cargo test --test property

      - name: Generate Coverage
        run: cargo tarpaulin --workspace --out Lcov

      - name: Upload Coverage
        uses: codecov/codecov-action@v3
        with:
          file: ./lcov.info

  wasm-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Run WASM Tests
        run: wasm-pack test --headless --chrome
```

### 2. Performance Regression Detection
```rust
// benches/machine_performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_machine_transitions(c: &mut Criterion) {
    let machine = create_large_machine();
    let state = machine.initial_state();

    c.bench_function("machine_transition", |b| {
        b.iter(|| {
            let new_state = machine.transition(&state, black_box(TestEvent::Next));
            black_box(new_state);
        })
    });
}
```

## Success Metrics

### Quantitative Goals
- [ ] **95%+ unit test coverage** (currently broken)
- [ ] **85%+ integration test coverage** (currently 0%)
- [ ] **75%+ property test coverage** (currently 0%)
- [ ] **80%+ WASM test coverage** (currently 0%)
- [ ] **All tests pass** in CI/CD pipeline

### Qualitative Goals
- [ ] **Fast test execution** (<30 seconds for unit tests)
- [ ] **Reliable CI/CD** (no flaky tests)
- [ ] **Good test documentation** (clear test names and purposes)
- [ ] **Maintainable test code** (DRY, well-organized)

## Risk Mitigation

### High-Risk Areas

#### 1. Test Compilation Failures
- **Risk**: Large number of test errors masks real issues
- **Mitigation**: Fix compilation errors systematically, one module at a time
- **Validation**: `cargo test --no-run` to check compilation

#### 2. Performance Impact
- **Risk**: Comprehensive tests slow down development
- **Mitigation**: Fast unit tests, integration tests in CI only
- **Validation**: Measure test execution time, optimize slow tests

#### 3. Test Maintenance Burden
- **Risk**: Tests become outdated as code evolves
- **Mitigation**: Test code reviews, automated test updates where possible
- **Validation**: Regular test health checks

## Future Enhancements

### Advanced Testing Features
1. **Mutation Testing**: Ensure tests catch real bugs
2. **Chaos Engineering**: Test system resilience
3. **Visual Regression**: UI component testing
4. **Load Testing**: Performance under stress
5. **Fuzz Testing**: Automated input generation

### Tooling Improvements
1. **Test Parallelization**: Faster execution
2. **Test Result Analytics**: Identify flaky/problematic tests
3. **Test Impact Analysis**: Run only affected tests
4. **Test Data Management**: Better test fixture handling

---

*Comprehensive testing infrastructure plan created September 20, 2025 - Targeting 95%+ test coverage*
