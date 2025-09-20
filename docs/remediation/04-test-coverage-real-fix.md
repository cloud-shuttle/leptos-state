# P1: Fix Test Coverage System (False Positives)

**Priority**: P1 (Quality Assurance Blocker)  
**Timeline**: 1 week  
**Assignee**: TBD

## Problem Statement

Current test coverage reporting is providing false positives:

1. **Tarpaulin runs only on default features** (which is empty `[]`)
2. **Feature-gated code never tested** despite showing "100% coverage" 
3. **Shell tests** only verify tools exist, not library behavior
4. **Contract tests missing** for critical invariants

## Current Coverage False Positives

```bash
# This runs with NO features enabled, missing huge code paths
cargo tarpaulin --out xml

# Shell tests like this don't test library code:
assert!(Command::new("cargo").arg("tarpaulin").status().unwrap().success());
```

**Result**: "100% coverage" claim in README is meaningless.

## Solution Design

### 1. Real Coverage with Feature Matrix

**New CI Coverage Strategy**:
```yaml
name: Coverage
strategy:
  matrix:
    features:
      - ""                    # default
      - "persist"            # persistence only  
      - "devtools"           # devtools only
      - "persist,devtools"   # common combination
      - "full"               # all features

steps:
  - name: Coverage for ${{ matrix.features }}
    run: |
      cargo tarpaulin \
        --features "${{ matrix.features }}" \
        --out xml \
        --output-dir coverage-${{ matrix.features }} \
        --fail-under 85
```

### 2. Contract-Based Testing

**Property Tests for Core Invariants**:

```rust
use quickcheck::*;

// Persistence round-trip laws
quickcheck! {
    fn persistence_roundtrip(state: MachineState, backend: TestBackend) -> bool {
        let key = "test_key";
        let serialized = state.serialize().unwrap();
        backend.save(key, &serialized).unwrap();
        let loaded = backend.load(key).unwrap().unwrap();
        let deserialized = MachineState::deserialize(&loaded).unwrap();
        state == deserialized
    }
}

// State transition laws
quickcheck! {
    fn transition_deterministic(machine: TestMachine, event: TestEvent) -> bool {
        let state1 = machine.clone().handle_event(event.clone());
        let state2 = machine.clone().handle_event(event);
        state1 == state2  // Same event = same result
    }
}

// Clone preservation laws  
quickcheck! {
    fn clone_preserves_behavior(machine: TestMachine) -> bool {
        let cloned = machine.clone();
        machine.get_current_state() == cloned.get_current_state() &&
        machine.get_guards().len() == cloned.get_guards().len() &&
        machine.get_actions().len() == cloned.get_actions().len()
    }
}
```

### 3. Integration Test Categories

**A. State Machine Contracts**:
- Hierarchical state consistency
- Parallel state independence  
- Guard evaluation order
- Action execution atomicity

**B. Store Contracts**:
- Leptos signal updates exactly once per change
- Middleware chain execution order
- Devtools synchronization

**C. Persistence Contracts**:
- All backends implement same behavior
- Serialization round-trips preserve semantics
- Error handling consistency

### 4. Replace Shell Tests

**Before** (BROKEN):
```rust
#[test] 
fn test_coverage_enforcement() {
    let output = Command::new("cargo")
        .args(&["tarpaulin", "--fail-under", "100"])
        .output()
        .unwrap();
    assert!(output.status.success());
}
```

**After** (PROPER):
```rust
#[cfg(feature = "testing")]
mod coverage_tests {
    // Real unit tests that exercise library code
    
    #[test]
    fn memory_backend_stores_and_retrieves() {
        let backend = MemoryBackend::new();
        backend.save("key", b"data").unwrap();
        assert_eq!(backend.load("key").unwrap(), Some(b"data".to_vec()));
    }
}
```

## Implementation Plan

### Day 1: CI Pipeline Fix
- [ ] Update GitHub Actions with feature matrix
- [ ] Configure separate coverage reports per feature set
- [ ] Set realistic coverage thresholds (start at 70%, target 90%)

### Day 2-3: Property Test Framework
- [ ] Add `quickcheck` dependency with `testing` feature
- [ ] Implement core property test generators
- [ ] Add round-trip tests for persistence
- [ ] Add determinism tests for state machines

### Day 4-5: Integration Test Suite
- [ ] Create comprehensive integration test scenarios
- [ ] Test all feature combinations that make sense
- [ ] Add performance regression tests
- [ ] Test error conditions and edge cases

## Testing Infrastructure

### Test Utilities Module
```rust
// tests/utils/mod.rs
pub mod generators {
    use quickcheck::{Arbitrary, Gen};
    
    #[derive(Clone, Debug, PartialEq)]
    pub struct TestMachine { /* ... */ }
    
    impl Arbitrary for TestMachine {
        fn arbitrary(g: &mut Gen) -> Self {
            // Generate valid test machines
        }
    }
}
```

### Coverage Collection
```toml
[package.metadata.tarpaulin]
exclude = ["tests/", "examples/", "benchmarks/"]
fail-under = 85
follow-exec = true
```

### Performance Baselines
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_state_transition(c: &mut Criterion) {
    let machine = create_benchmark_machine();
    c.bench_function("state_transition", |b| {
        b.iter(|| machine.handle_event(black_box(TestEvent::Next)))
    });
}
```

## Success Metrics

### Coverage Targets by Feature
- **Default features**: 90%+ (core functionality)
- **Persist feature**: 85%+ (includes I/O error paths)  
- **Devtools feature**: 80%+ (UI integration complexity)
- **Full features**: 75%+ (integration complexity)

### Contract Test Coverage
- [ ] 20+ property-based tests for core invariants
- [ ] Round-trip tests for all persistence backends
- [ ] State machine behavioral laws verified
- [ ] Error propagation contracts tested

### Integration Test Coverage
- [ ] All public APIs exercised in realistic scenarios
- [ ] Feature combination testing (persist + devtools, etc.)
- [ ] Performance regression detection
- [ ] Error handling and recovery paths

## Acceptance Criteria

- [ ] CI reports real coverage for each feature combination
- [ ] No shell tests remain - all tests exercise library code
- [ ] Property-based tests catch clone/persistence bugs
- [ ] Integration tests cover multi-feature scenarios
- [ ] Coverage numbers are accurate and meaningful
- [ ] Performance benchmarks catch regressions

## Dependencies

- Requires feature flag system to be fixed first
- May require stub implementation fixes to avoid test failures
- Needs consensus on realistic coverage targets

## Risks

- Real coverage may initially be much lower than reported 100%
- Property tests may uncover additional bugs in core logic
- CI pipeline complexity increases with feature matrix
- Performance test baseline establishment needed
