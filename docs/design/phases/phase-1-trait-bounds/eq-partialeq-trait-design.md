# Eq and PartialEq Trait Design

## Overview
Add `Eq` and `PartialEq` trait bounds to enable state comparison, optimization of reactivity, and equality-based operations while maintaining compilation safety.

## Current State
```rust
// Current minimal bounds
pub trait State: Send + Sync + Clone + 'static {}
pub trait Event: Send + Sync + Clone + 'static {}
```

## Proposed Enhancement
```rust
// Enhanced bounds with equality
pub trait State: Send + Sync + Clone + Eq + PartialEq + 'static {}
pub trait Event: Send + Sync + Clone + Eq + PartialEq + 'static {}
```

## Motivation

### Reactivity Optimization
- **Change Detection**: Only trigger updates when state actually changes
- **Performance**: Avoid unnecessary re-renders and computations
- **Memory Efficiency**: Reduce signal propagation overhead

### Equality Operations
- **State Comparison**: Check if states are equivalent
- **Testing**: Assert state equality in tests
- **Debugging**: Compare state snapshots
- **Caching**: Determine if expensive operations need re-running

### Use Cases
- Reactive UI updates only on actual changes
- State persistence optimization
- Undo/redo functionality
- State diffing and debugging
- Test assertions and validation

## Implementation Details

### Auto-Derive for Simple Types
```rust
#[derive(Clone, PartialEq, Eq)]
struct CounterState {
    count: i32,
    step: i32,
}
```

### Complex Types with Eq
For types containing floats or other non-Eq types:
```rust
#[derive(Clone, PartialEq)]  // Only PartialEq, not Eq
struct FloatState {
    value: f64,  // f64 is not Eq
    name: String,
}

// Or use ordered comparisons
#[derive(Clone, PartialEq, Eq)]
struct PreciseState {
    value: i32,  // Precise integer instead of float
    name: String,
}
```

### Manual Equality Implementation
For complex equality logic:
```rust
#[derive(Clone)]
struct ComplexState {
    data: Vec<Item>,
    timestamp: DateTime<Utc>,
}

impl PartialEq for ComplexState {
    fn eq(&self, other: &Self) -> bool {
        // Custom equality logic
        self.data.len() == other.data.len() &&
        self.data.iter().zip(&other.data).all(|(a, b)| a.id == b.id) &&
        (self.timestamp - other.timestamp).num_seconds().abs() < 1
    }
}

impl Eq for ComplexState {}  // If total equality is possible
```

## API Design

### Store Integration
```rust
impl<S: State> Store<S> {
    /// Update only if state actually changes
    pub fn update_if_changed<F>(&self, updater: F) -> Result<bool, StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        let old_state = self.signal.get_untracked();
        self.update(updater)?;
        let new_state = self.signal.get_untracked();

        Ok(old_state != new_state)
    }

    /// Check if state has changed since last check
    pub fn has_changed(&self) -> bool {
        // Implementation with cached previous state
        // Returns true if current state != cached state
        todo!()
    }

    /// Get state diff
    pub fn diff(&self, other: &S) -> StateDiff {
        if &self.signal.get_untracked() == other {
            StateDiff::None
        } else {
            StateDiff::Changed
        }
    }
}
```

### Reactive Optimizations
```rust
impl<S: State> RwSignal<S> {
    /// Optimized update that only triggers if changed
    pub fn update_eq<F>(&self, updater: F) -> bool
    where
        F: FnOnce(&mut S),
    {
        let old_value = self.get_untracked();
        self.update(updater);
        let new_value = self.get_untracked();

        old_value != new_value
    }
}
```

### Testing Utilities
```rust
impl<S: State> Store<S> {
    /// Assert state equals expected value
    pub fn assert_eq(&self, expected: &S) {
        let actual = self.signal.get_untracked();
        assert_eq!(&actual, expected, "Store state mismatch");
    }

    /// Wait for state to equal expected value
    pub async fn wait_for_eq(&self, expected: &S) -> Result<(), StoreError> {
        // Implementation that waits for state to match
        todo!()
    }
}
```

## State Machine Integration

### Transition Optimization
```rust
impl<S: State, E: Event> Machine<S, E> {
    /// Send event only if it would change state
    pub fn send_if_changes(&mut self, event: E) -> Result<bool, MachineError> {
        let current_state = self.current_state.clone();
        let would_change = self.can_transition(&event);

        if would_change {
            self.send(event)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Check if transition would change context
    pub fn transition_changes_context(&self, event: &E) -> bool {
        // Simulate transition and check if context changes
        todo!()
    }
}
```

## Performance Optimizations

### Memoization
```rust
use std::collections::HashMap;

struct MemoizedComputation<S: State, T> {
    cache: HashMap<S, T>,
    compute: Box<dyn Fn(&S) -> T>,
}

impl<S: State, T> MemoizedComputation<S, T> {
    pub fn get(&mut self, state: &S) -> &T {
        self.cache.entry(state.clone()).or_insert_with(|| (self.compute)(state))
    }
}
```

### Change Detection
```rust
struct ChangeDetector<S: State> {
    previous_state: Option<S>,
}

impl<S: State> ChangeDetector<S> {
    pub fn has_changed(&mut self, new_state: &S) -> bool {
        let changed = self.previous_state.as_ref() != Some(new_state);
        if changed {
            self.previous_state = Some(new_state.clone());
        }
        changed
    }
}
```

## Error Handling

### Compilation Errors
When Eq/PartialEq can't be satisfied:
```rust
error[E0277]: the trait bound `MyFloatState: Eq` is not satisfied
help: consider using PartialEq instead: `#[derive(PartialEq)]`
help: or implement Eq manually if total equality is possible
help: note: `f64` does not implement `Eq` due to NaN
```

### Float Handling
Special consideration for floating point types:
```rust
#[derive(Clone)]
struct FloatState {
    value: f64,
    tolerance: f64,
}

impl PartialEq for FloatState {
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() < self.tolerance
    }
}

// Note: Cannot implement Eq due to approximate equality
```

## Testing Strategy

### Equality Tests
```rust
#[test]
fn state_equality_works() {
    let state1 = CounterState { count: 5, step: 2 };
    let state2 = CounterState { count: 5, step: 2 };
    let state3 = CounterState { count: 6, step: 2 };

    assert_eq!(state1, state2);
    assert_ne!(state1, state3);
}

#[test]
fn store_change_detection() {
    let store = Store::new(CounterState::default());

    // Initially no change
    assert!(!store.has_changed());

    // Make change
    store.update(|s| s.count = 1).unwrap();
    assert!(store.has_changed());

    // Check again (should be false since we checked)
    assert!(!store.has_changed());
}
```

### Property-Based Testing
```rust
proptest! {
    #[test]
    fn state_equality_reflexive(state: CounterState) {
        prop_assert_eq!(state, state);
    }

    #[test]
    fn state_equality_symmetric(a: CounterState, b: CounterState) {
        prop_assert_eq!(a == b, b == a);
    }

    #[test]
    fn store_update_change_detection(state: CounterState) {
        let store = Store::new(state.clone());

        // Update to same state should not trigger change
        let changed = store.update_if_changed(|s| *s = state.clone()).unwrap();
        prop_assert!(!changed);

        // Update to different state should trigger change
        let changed = store.update_if_changed(|s| s.count += 1).unwrap();
        prop_assert!(changed);
    }
}
```

## Migration Guide

### Simple Migration
```rust
// Before
#[derive(Clone)]
struct MyState {
    count: i32,
    name: String,
}

// After - add equality traits
#[derive(Clone, PartialEq, Eq)]
struct MyState {
    count: i32,
    name: String,
}
```

### Complex Types Migration
```rust
// Before
#[derive(Clone)]
struct ComplexState {
    data: Vec<f64>,  // Contains floats, can't derive Eq
    timestamp: DateTime<Utc>,
}

// After - custom implementation
#[derive(Clone)]
struct ComplexState {
    data: Vec<f64>,
    timestamp: DateTime<Utc>,
}

impl PartialEq for ComplexState {
    fn eq(&self, other: &Self) -> bool {
        self.data.len() == other.data.len() &&
        self.data.iter().zip(&other.data)
            .all(|(a, b)| (a - b).abs() < f64::EPSILON) &&
        self.timestamp == other.timestamp
    }
}
```

## Performance Impact

### Compile Time
- **Moderate**: Equality derives add some complexity
- **Impact**: ~2-5% increase in compile time

### Runtime
- **Low cost**: Equality checks are fast for most types
- **Memory**: No additional memory usage
- **Binary size**: Small increase for comparison logic

### Optimization Benefits
- **Reactivity**: Reduced unnecessary updates
- **Caching**: More efficient memoization
- **Debugging**: Faster state diffing

## Security Considerations

### Timing Attacks
Equality comparisons should not leak information through timing:
```rust
// Secure comparison for sensitive data
impl PartialEq for SecureState {
    fn eq(&self, other: &Self) -> bool {
        // Use constant-time comparison
        use subtle::ConstantTimeEq;
        self.encrypted_data.ct_eq(&other.encrypted_data).into()
    }
}
```

### Information Disclosure
Avoid equality operations on sensitive state in logs:
```rust
impl Debug for SecureState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SecureState {{ <encrypted> }}")
    }
}
```

## Future Extensions

### Custom Comparers
```rust
trait StateComparer<S> {
    fn deep_equal(&self, a: &S, b: &S) -> bool;
    fn shallow_equal(&self, a: &S, b: &S) -> bool;
    fn semantic_equal(&self, a: &S, b: &S) -> bool;
}
```

### Change Callbacks
```rust
struct ChangeCallback<S: State> {
    previous: Option<S>,
    on_change: Box<dyn Fn(&S, &S)>,
}

impl<S: State> ChangeCallback<S> {
    pub fn notify_if_changed(&mut self, new_state: &S) {
        if let Some(ref prev) = self.previous {
            if prev != new_state {
                (self.on_change)(prev, new_state);
            }
        }
        self.previous = Some(new_state.clone());
    }
}
```

## Risk Assessment

### Likelihood: Medium
- Eq/PartialEq are standard traits
- Many types can derive them automatically
- Float handling requires special care

### Impact: Medium
- Some types may need manual implementations
- Float comparisons require careful design
- Performance optimizations are valuable

### Mitigation
- Provide comprehensive documentation
- Include helper macros for common patterns
- Offer migration assistance
- Start with PartialEq only, add Eq later if needed

### Rollback Plan
- Remove Eq bound, keep PartialEq
- Provide alternative comparison methods
- Use runtime feature detection
