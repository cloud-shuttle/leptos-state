# 🚀 Leptos 0.8+ Migration Roadmap

## Overview

This document outlines the comprehensive migration strategy for upgrading `leptos-state` from Leptos 0.6 to Leptos 0.8+. The migration involves significant architectural changes and requires careful planning to maintain stability and functionality.

## Migration Goals

- ✅ **Maintain API compatibility** where possible
- ✅ **Leverage new Leptos 0.8+ features** for better performance
- ✅ **Ensure thread safety** with proper `Send + Sync` bounds
- ✅ **Preserve existing functionality** while improving architecture
- ✅ **Provide clear migration path** for users

## Phase 1: Foundation & Core Traits (Week 1)

### 1.1 Update Core Trait Bounds
**Priority**: Critical  
**Effort**: 2-3 days

#### Tasks:
- [ ] Update `StateMachine` trait with `Send + Sync` bounds
- [ ] Update `MachineState` trait with `Send + Sync` bounds
- [ ] Update `Event` trait with `Send + Sync` bounds
- [ ] Update `Store` trait with `Send + Sync` bounds
- [ ] Update `AsyncStore` trait with `Send + Sync` bounds

#### Files to modify:
```
leptos-state/src/machine/machine.rs
leptos-state/src/store/store.rs
leptos-state/src/store/async_store.rs
leptos-state/src/utils/types.rs
```

#### Example changes:
```rust
// Before
pub trait StateMachine: Sized + 'static {
    type Context: Clone + PartialEq;
    type Event: Clone;
    type State: MachineState<Context = Self::Context> + Clone;
}

// After
pub trait StateMachine: Sized + 'static {
    type Context: Clone + PartialEq + Send + Sync + 'static;
    type Event: Clone + Send + Sync + 'static;
    type State: MachineState<Context = Self::Context> + Clone + Send + Sync + 'static;
}
```

### 1.2 Update Signal Creation APIs
**Priority**: Critical  
**Effort**: 1-2 days

#### Tasks:
- [ ] Replace `create_signal` with `create_rw_signal`
- [ ] Update signal type annotations
- [ ] Fix signal storage compatibility
- [ ] Update signal access patterns

#### Files to modify:
```
leptos-state/src/hooks/use_machine.rs
leptos-state/src/hooks/use_store.rs
leptos-state/src/store/store.rs
leptos-state/src/store/async_store.rs
```

#### Example changes:
```rust
// Before
let (state, set_state) = create_signal(M::initial());

// After
let (state, set_state) = create_rw_signal(M::initial());
```

### 1.3 Update Callback API
**Priority**: Critical  
**Effort**: 1 day

#### Tasks:
- [ ] Update `Callback::new` usage
- [ ] Replace `callback.call()` with direct invocation
- [ ] Update callback type signatures

#### Files to modify:
```
leptos-state/src/hooks/use_machine.rs
leptos-state/src/store/async_store.rs
```

#### Example changes:
```rust
// Before
self.send.call(event);

// After
self.send(event);
```

## Phase 2: Store System Refactoring (Week 2)

### 2.1 Update Store Implementations
**Priority**: High  
**Effort**: 3-4 days

#### Tasks:
- [ ] Refactor `StoreContext` for new signal types
- [ ] Update `create_computed` implementations
- [ ] Fix signal storage compatibility
- [ ] Update store state management

#### Files to modify:
```
leptos-state/src/store/store.rs
leptos-state/src/store/async_store.rs
leptos-state/src/hooks/use_store.rs
```

### 2.2 Update Async Store Features
**Priority**: High  
**Effort**: 2-3 days

#### Tasks:
- [ ] Update resource integration
- [ ] Fix async signal handling
- [ ] Update error handling patterns
- [ ] Improve async state management

#### Files to modify:
```
leptos-state/src/store/async_store.rs
leptos-state/src/hooks/use_store.rs
```

### 2.3 Update Store History
**Priority**: Medium  
**Effort**: 1-2 days

#### Tasks:
- [ ] Fix history signal storage
- [ ] Update history state management
- [ ] Improve history performance
- [ ] Add thread-safe history operations

#### Files to modify:
```
leptos-state/src/hooks/use_store.rs
```

## Phase 3: Machine System Updates (Week 3)

### 3.1 Update Machine Builder API
**Priority**: High  
**Effort**: 2-3 days

#### Tasks:
- [ ] Update builder trait bounds
- [ ] Fix state node implementations
- [ ] Update transition logic
- [ ] Improve builder performance

#### Files to modify:
```
leptos-state/src/machine/machine.rs
leptos-state/src/machine/states.rs
leptos-state/src/machine/transitions.rs
```

### 3.2 Update Machine Hooks
**Priority**: High  
**Effort**: 2-3 days

#### Tasks:
- [ ] Fix `use_machine` hook
- [ ] Update machine handle implementations
- [ ] Fix machine subscription hooks
- [ ] Improve machine effect hooks

#### Files to modify:
```
leptos-state/src/hooks/use_machine.rs
```

### 3.3 Update Machine Features
**Priority**: Medium  
**Effort**: 3-4 days

#### Tasks:
- [ ] Update persistence features
- [ ] Fix visualization features
- [ ] Update testing framework
- [ ] Update performance features

#### Files to modify:
```
leptos-state/src/machine/persistence.rs
leptos-state/src/machine/visualization.rs
leptos-state/src/machine/testing.rs
leptos-state/src/machine/performance.rs
```

## Phase 4: Examples & Documentation (Week 4)

### 4.1 Update Examples
**Priority**: High  
**Effort**: 2-3 days

#### Tasks:
- [ ] Update Todo App example
- [ ] Update Analytics Dashboard example
- [ ] Fix example dependencies
- [ ] Add new examples showcasing 0.8+ features

#### Files to modify:
```
examples/todo-app/src/
examples/analytics-dashboard/src/
examples/todo-app/Cargo.toml
examples/analytics-dashboard/Cargo.toml
```

### 4.2 Update Documentation
**Priority**: Medium  
**Effort**: 2-3 days

#### Tasks:
- [ ] Update API documentation
- [ ] Create migration guide
- [ ] Update examples documentation
- [ ] Add breaking changes documentation

#### Files to modify:
```
README.md
docs/
examples/README.md
```

### 4.3 Update Tests
**Priority**: High  
**Effort**: 2-3 days

#### Tasks:
- [ ] Fix existing tests
- [ ] Add new tests for 0.8+ features
- [ ] Update test utilities
- [ ] Add integration tests

#### Files to modify:
```
tests/
leptos-state/src/machine/testing.rs
```

## Phase 5: Integration & Testing (Week 5)

### 5.1 Comprehensive Testing
**Priority**: Critical  
**Effort**: 3-4 days

#### Tasks:
- [ ] Run full test suite
- [ ] Fix failing tests
- [ ] Add performance benchmarks
- [ ] Test with real-world examples

### 5.2 Performance Optimization
**Priority**: Medium  
**Effort**: 2-3 days

#### Tasks:
- [ ] Profile performance
- [ ] Optimize signal usage
- [ ] Improve memory usage
- [ ] Add performance tests

### 5.3 Documentation & Release
**Priority**: High  
**Effort**: 2-3 days

#### Tasks:
- [ ] Finalize documentation
- [ ] Create release notes
- [ ] Update changelog
- [ ] Prepare for release

## Technical Challenges & Solutions

### Challenge 1: Thread Safety
**Problem**: All reactive primitives now require `Send + Sync` bounds.

**Solution**:
- Add `Send + Sync` bounds to all trait definitions
- Use `Arc` for shared state where needed
- Implement proper thread-safe patterns

### Challenge 2: Signal Storage Changes
**Problem**: New signal storage system requires different patterns.

**Solution**:
- Use `create_rw_signal` instead of `create_signal`
- Update signal access patterns
- Implement proper signal lifecycle management

### Challenge 3: API Breaking Changes
**Problem**: Leptos 0.8+ has breaking API changes.

**Solution**:
- Create compatibility layer where possible
- Provide clear migration documentation
- Use feature flags for gradual migration

### Challenge 4: Performance Impact
**Problem**: New architecture may impact performance.

**Solution**:
- Profile and optimize critical paths
- Use new Leptos 0.8+ performance features
- Implement proper memoization

## Migration Strategy

### Approach: Gradual Migration
1. **Maintain 0.6 compatibility** in main branch
2. **Develop 0.8+ version** in feature branch
3. **Incremental feature migration**
4. **Comprehensive testing** at each phase
5. **Community feedback** integration

### Feature Flags
```rust
#[cfg(feature = "leptos-0-8")]
pub mod v08 {
    // 0.8+ specific implementations
}

#[cfg(not(feature = "leptos-0-8"))]
pub mod v06 {
    // 0.6 compatible implementations
}
```

### Backward Compatibility
- Maintain 0.6 API where possible
- Provide migration utilities
- Clear documentation of breaking changes
- Gradual deprecation of old APIs

## Success Criteria

### Technical Criteria
- [ ] 100% compilation success
- [ ] All tests passing
- [ ] Performance maintained or improved
- [ ] No memory leaks
- [ ] Thread safety verified

### User Experience Criteria
- [ ] Clear migration path
- [ ] Comprehensive documentation
- [ ] Working examples
- [ ] Community feedback positive

### Release Criteria
- [ ] Stable API
- [ ] Comprehensive test coverage
- [ ] Performance benchmarks
- [ ] Documentation complete
- [ ] Community validation

## Risk Mitigation

### Technical Risks
- **API instability**: Use feature flags and compatibility layers
- **Performance regressions**: Continuous benchmarking
- **Breaking changes**: Comprehensive testing and documentation

### User Risks
- **Migration complexity**: Clear documentation and examples
- **Breaking changes**: Gradual migration path
- **Community confusion**: Clear communication and feedback

## Timeline

### Week 1: Foundation
- Core trait updates
- Signal API updates
- Basic compilation fixes

### Week 2: Store System
- Store refactoring
- Async store updates
- History system fixes

### Week 3: Machine System
- Machine builder updates
- Hook system fixes
- Feature updates

### Week 4: Examples & Docs
- Example updates
- Documentation updates
- Test fixes

### Week 5: Integration
- Comprehensive testing
- Performance optimization
- Release preparation

## Next Steps

1. **Create migration branch** from current stable version
2. **Set up development environment** with Leptos 0.8+
3. **Begin Phase 1** with core trait updates
4. **Establish testing framework** for migration
5. **Create community communication** plan

## Resources

### Documentation
- [Leptos 0.8 Migration Guide](https://leptos.dev/book/0.8/migration.html)
- [Leptos 0.8 API Reference](https://docs.rs/leptos/0.8/)
- [Leptos 0.8 Examples](https://github.com/leptos-rs/leptos/tree/main/examples)

### Community
- [Leptos Discord](https://discord.gg/YdRAhS7eQB)
- [Leptos GitHub](https://github.com/leptos-rs/leptos)
- [Leptos Forum](https://github.com/leptos-rs/leptos/discussions)

---

**Status**: Ready to begin Phase 1  
**Next Action**: Create migration branch and start core trait updates  
**Estimated Completion**: 5 weeks  
**Risk Level**: Medium-High (manageable with proper planning)
