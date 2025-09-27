# 🔧 Event Type Trait Bounds Fix Design

## Problem
Event types throughout the machine module lack required trait bounds (`Hash`, `Eq`, `Send`, `Sync`) causing compilation failures.

## Current Issues

### 1. Missing Hash Trait
```rust
// Current problematic Event enums
#[derive(Clone, Debug, PartialEq)]
enum TestEvent {
    Next,
    Previous,
}

// Required for HashMap<Event, String> usage
// Error: the trait `Hash` is not implemented for `TestEvent`
```

### 2. Missing Eq Trait
```rust
// Required for HashMap key comparisons
// Error: the trait `std::cmp::Eq` is not implemented for `TestEvent`
```

### 3. Missing Send/Sync Traits
```rust
// Required for async/parallel usage
// Error: `TestEvent` cannot be sent between threads safely
// Error: `TestEvent` cannot be shared between threads safely
```

## Solution Design

### Event Type Requirements
All Event types must implement:
- `Clone` - For event copying
- `Debug` - For debugging/logging
- `PartialEq` - For equality comparisons
- `Eq` - For total equality (required by Hash)
- `Hash` - For use as HashMap keys
- `Send` - For thread safety
- `Sync` - For shared references

### Implementation Pattern
```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum EventType {
    Start,
    Stop,
    Pause,
    Resume,
}

impl Send for EventType {}
impl Sync for EventType {}
```

### Alternative: Manual Implementation
```rust
#[derive(Clone, Debug, PartialEq)]
enum ComplexEvent {
    WithData { value: i32 },
    Simple,
}

impl Eq for ComplexEvent {}

impl std::hash::Hash for ComplexEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ComplexEvent::WithData { value } => {
                0u8.hash(state);
                value.hash(state);
            }
            ComplexEvent::Simple => {
                1u8.hash(state);
            }
        }
    }
}

// Send and Sync are auto-implemented for this enum
```

## Implementation Strategy

### Phase 1: Identify All Event Types
1. **Search codebase** for all `enum` definitions used as Events
2. **Catalog current trait implementations**
3. **Identify missing bounds**

### Phase 2: Add Missing Traits
1. **Add derive macros** where possible
2. **Implement manually** where derive isn't sufficient
3. **Add Send/Sync** where needed

### Phase 3: Verification
1. **Compile check** - Ensure no trait bound errors
2. **Test compilation** - Verify all tests pass
3. **Runtime testing** - Ensure functionality works

## Risk Assessment

**Medium Risk:**
- Adding Hash/Eq may change equality semantics
- Send/Sync bounds may restrict usage patterns

**Mitigation:**
- Test thoroughly after changes
- Consider backward compatibility
- Document any behavioral changes

## Success Criteria

- [ ] All Event types implement required traits
- [ ] No compilation errors related to trait bounds
- [ ] All existing functionality preserved
- [ ] HashMap usage works correctly
- [ ] Async operations work correctly

## Files to Modify

- `leptos-state/src/machine/core.rs` - Core machine Event types
- `leptos-state/src/machine/builder/mod.rs` - Builder Event types
- `leptos-state/src/machine/history.rs` - History Event types
- `leptos-state/src/machine/testing.rs` - Test Event types
- `leptos-state/src/machine/performance.rs` - Performance Event types
- `leptos-state/src/machine/integration.rs` - Integration Event types

## Testing Requirements

- [ ] Unit tests for Event equality/hashing
- [ ] Integration tests for HashMap usage
- [ ] Async tests for Send/Sync behavior
- [ ] Performance tests for trait overhead
