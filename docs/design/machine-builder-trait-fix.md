# 🔧 Machine Builder Trait Signature Fix Design

## Problem
`MachineBuilder` trait has conflicting signatures between core and builder modules, causing compilation failures.

## Current Issues

### 1. Trait Definition Conflicts
```rust
// In core.rs - different signature
pub trait MachineBuilder {
    type State;
    type Event;
    type Context;

    fn new() -> Self;
    fn state<Name: Into<String>>(self, name: Name) -> Self;
    fn initial<Name: Into<String>>(self, state: Name) -> Self;
    fn transition<E, S>(self, from: S, event: E, to: S) -> Self;
    fn build_with_context(self, context: Self::Context) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>;
}

// In builder/mod.rs - conflicting signature
pub trait MachineBuilder {
    type State;
    type Event;
    type Context;

    fn new() -> Self;
    fn state<Name: Into<String>>(self, name: Name) -> Self;
    fn initial<Name: Into<String>>(self, state: Name) -> Self;
    fn transition<E, S>(self, from: S, event: E, to: S) -> Self;
    fn build(self) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>; // Different!
}
```

### 2. Method Signature Mismatches
- `build()` vs `build_with_context(context)`
- Different return types
- Conflicting generic parameter usage

## Solution Design

### Option 1: Unify Core and Builder Traits (Recommended)
```rust
// Single unified trait in core module
pub trait MachineBuilder {
    type State;
    type Event;
    type Context;

    fn new() -> Self;
    fn state<Name: Into<String>>(self, name: Name) -> Self;
    fn initial<Name: Into<String>>(self, state: Name) -> Self;
    fn transition<E, S>(self, from: S, event: E, to: S) -> Self;
    fn build_with_context(self, context: Self::Context) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>;
    fn build(self) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>
    where
        Self::Context: Default,
    {
        self.build_with_context(Self::Context::default())
    }
}

// Builder implementation uses the unified trait
impl<S, E, C> MachineBuilder for MachineBuilderImpl<S, E, C> { ... }
```

### Option 2: Separate Traits with Clear Naming
```rust
// Core trait for machine construction
pub trait CoreMachineBuilder {
    fn build_with_context(self, context: Self::Context) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>;
}

// Fluent builder trait
pub trait FluentMachineBuilder: CoreMachineBuilder {
    fn state<Name: Into<String>>(self, name: Name) -> Self;
    fn initial<Name: Into<String>>(self, state: Name) -> Self;
    fn transition<E, S>(self, from: S, event: E, to: S) -> Self;
}

// Default implementation for convenience
impl<T> CoreMachineBuilder for T
where
    T: FluentMachineBuilder,
    T::Context: Default,
{
    fn build_with_context(self, context: Self::Context) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>> {
        // Implementation that uses context
    }
}
```

### Option 3: Trait Inheritance Approach
```rust
// Base trait with core functionality
pub trait BaseMachineBuilder {
    type State;
    type Event;
    type Context;

    fn build_with_context(self, context: Self::Context) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>;
}

// Extended trait with fluent API
pub trait MachineBuilder: BaseMachineBuilder {
    fn state<Name: Into<String>>(self, name: Name) -> Self;
    fn initial<Name: Into<String>>(self, state: Name) -> Self;
    fn transition<E, S>(self, from: S, event: E, to: S) -> Self;
    fn build(self) -> MachineResult<Machine<Self::State, Self::Event, Self::Context>>
    where
        Self::Context: Default,
    {
        self.build_with_context(Self::Context::default())
    }
}
```

## Implementation Strategy

### Phase 1: Trait Design Decision
1. **Choose unified approach** - Single trait with both methods
2. **Update trait definitions** - Align signatures across modules
3. **Update implementations** - Ensure compatibility

### Phase 2: Code Migration
1. **Update core.rs** - Modify trait definition
2. **Update builder/mod.rs** - Align implementation
3. **Update all usages** - Fix calling code
4. **Remove conflicts** - Eliminate duplicate trait definitions

### Phase 3: Testing
1. **Compilation tests** - Ensure no trait conflicts
2. **Functionality tests** - Verify builder works correctly
3. **API tests** - Test both build methods

## Risk Assessment

**High Risk:**
- Changing trait signatures may break existing code
- Multiple modules depend on builder trait

**Mitigation:**
- Maintain backward compatibility where possible
- Use trait inheritance for gradual migration
- Comprehensive testing before committing

## Success Criteria

- [ ] No trait signature conflicts
- [ ] Both build methods work correctly
- [ ] All existing code compiles
- [ ] New functionality works as expected
- [ ] No breaking changes to public API

## Files to Modify

- `leptos-state/src/machine/core.rs` - Core trait definition
- `leptos-state/src/machine/builder/mod.rs` - Builder implementation
- All files using `MachineBuilder` trait - Update usage patterns
- Test files - Update test expectations

## Migration Plan

1. **Add new method** to existing trait (non-breaking)
2. **Update implementations** to support both methods
3. **Gradually migrate** calling code to use new method
4. **Remove deprecated method** in future version

## Testing Requirements

- [ ] Trait method resolution works correctly
- [ ] Both build methods produce equivalent results
- [ ] Error handling works for both methods
- [ ] Performance characteristics are maintained
