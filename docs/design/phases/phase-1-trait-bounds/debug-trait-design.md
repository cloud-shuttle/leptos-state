# Debug Trait Design

## Overview
Add `Debug` trait bound to enable development debugging and logging capabilities while maintaining minimal compilation requirements.

## Current State
```rust
// Current minimal bounds
pub trait State: Send + Sync + Clone + 'static {}
pub trait Event: Send + Sync + Clone + 'static {}
```

## Proposed Enhancement
```rust
// Enhanced bounds with Debug
pub trait State: Send + Sync + Clone + Debug + 'static {}
pub trait Event: Send + Sync + Clone + Debug + 'static {}
```

## Motivation

### Development Experience
- **Logging**: State changes can be logged for debugging
- **Error Messages**: Meaningful error messages with state contents
- **Development Tools**: DevTools can display state information
- **Testing**: Test failures can show state differences

### Use Cases
- Console logging during development
- Error reporting with state context
- Development middleware and debugging tools
- Test failure diagnostics

## Implementation Details

### Auto-Derive for Common Types
For most user types, Debug can be automatically derived:
```rust
#[derive(Clone, Debug)]
struct CounterState {
    count: i32,
    step: i32,
}
```

### Complex Types Handling
For types that can't derive Debug (e.g., containing non-Debug fields):
```rust
// Option 1: Manual Debug implementation
impl Debug for ComplexState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ComplexState")
            .field("debuggable_field", &self.debuggable_field)
            .field("complex_field", &"<complex>")
            .finish()
    }
}

// Option 2: Wrapper types
#[derive(Clone)]
struct ComplexState {
    data: NonDebugType,
}

impl Debug for ComplexState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ComplexState {{ data: <non-debug> }}")
    }
}
```

## API Design

### Store Integration
```rust
impl<S: State> Store<S> {
    pub fn debug_state(&self) -> String {
        format!("{:?}", self.signal.get_untracked())
    }

    pub fn log_state_change(&self, old_state: &S, new_state: &S) {
        log::debug!("State changed: {:?} -> {:?}", old_state, new_state);
    }
}
```

### Hook Integration
```rust
pub fn use_store_debug<S: State + Default>() -> (ReadSignal<S>, StoreActions<S>, impl Fn() -> String) {
    let (state, actions) = use_store::<S>();
    let debug_fn = move || format!("{:?}", state.get());

    (state, actions, debug_fn)
}
```

## Error Handling

### Compilation Errors
When Debug can't be satisfied:
```rust
error[E0277]: the trait bound `MyComplexType: Debug` is not satisfied
help: consider deriving Debug: `#[derive(Debug)]`
help: or implement Debug manually
```

### Runtime Behavior
- Debug formatting should never panic
- Complex fields should be represented as `<complex>` or similar
- Debug output should be developer-friendly, not user-facing

## Testing Strategy

### Unit Tests
```rust
#[test]
fn debug_formatting_works() {
    let state = CounterState { count: 42, step: 2 };
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("42"));
    assert!(debug_str.contains("2"));
}
```

### Integration Tests
```rust
#[test]
fn store_debug_functionality() {
    let store = Store::new(CounterState::default());
    let debug_output = store.debug_state();
    assert!(!debug_output.is_empty());
}
```

## Migration Guide

### For Existing Code
Most existing code will work unchanged:
```rust
#[derive(Clone)]  // Old code
struct MyState { /* ... */ }

// Becomes:
#[derive(Clone, Debug)]  // Add Debug
struct MyState { /* ... */ }
```

### Breaking Changes
- None expected for auto-derivable types
- Manual implementations may be needed for complex types
- Custom derive macros may need updates

## Performance Impact

### Compile Time
- **Minimal**: Debug derive is fast and widely optimized
- **Impact**: ~1-2% increase in compile time

### Runtime
- **Zero cost**: Debug formatting only called when needed
- **Memory**: No additional memory usage
- **Binary size**: Small increase (~1-5KB) for debug symbols

## Security Considerations

### Information Disclosure
- Debug output should not contain sensitive information
- Production builds can strip debug symbols
- Consider implementing redacted Debug for sensitive types

### Privacy
```rust
impl Debug for SensitiveState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SensitiveState {{ <redacted> }}")
    }
}
```

## Future Extensions

### Conditional Compilation
```rust
#[cfg_attr(debug_assertions, derive(Debug))]
#[cfg_attr(not(debug_assertions), derive(Clone))]
pub struct MyState { /* ... */ }
```

### Custom Debug Levels
```rust
impl MyState {
    pub fn debug_detailed(&self) -> String { /* full debug */ }
    pub fn debug_summary(&self) -> String { /* summary only */ }
}
```

## Risk Assessment

### Likelihood: Low
- Debug is a fundamental trait
- Most types can derive it automatically
- Well-established Rust ecosystem patterns

### Impact: Low
- Easy to implement for most use cases
- Clear error messages guide users
- Backward compatible for existing code

### Mitigation
- Provide comprehensive documentation
- Include migration examples
- Offer helper macros for complex cases
