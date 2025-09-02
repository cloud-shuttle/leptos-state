# 🚀 Leptos 0.8+ Migration Plan

## Current Status

**Library Version**: `leptos-state v0.1.0`
**Current Leptos Version**: `0.6` (stable)
**Target Leptos Version**: `0.8+` (latest)

## Migration Challenges

### 1. API Changes in Leptos 0.7+
- `create_signal` → `create_rw_signal`
- `Callback::new` → `Callback::new` (different signature)
- Signal trait bounds changed significantly
- Storage and reactivity system overhaul

### 2. Breaking Changes
- Signal types require `Send + Sync` bounds
- Memo and ReadSignal trait implementations changed
- Resource API modifications
- View macro syntax changes

### 3. Current Blockers
- Complex trait bound issues with `StateMachine` types
- Signal storage compatibility problems
- Callback API changes
- Resource integration issues

## Migration Strategy

### Phase 1: Core Library Updates ✅
- [x] Update workspace dependencies to Leptos 0.7
- [x] Fix import statements (`leptos::prelude::*`)
- [x] Update function names (`create_signal` → `create_rw_signal`)
- [x] Update examples to use new API

### Phase 2: Trait Bound Fixes 🔄
- [ ] Add `Send + Sync` bounds to `StateMachine` trait
- [ ] Update signal storage implementations
- [ ] Fix Memo and ReadSignal trait bounds
- [ ] Resolve Callback API compatibility

### Phase 3: Advanced Features 🔄
- [ ] Update Resource integration
- [ ] Fix async store implementations
- [ ] Update DevTools compatibility
- [ ] Test all examples

### Phase 4: Documentation & Testing 🔄
- [ ] Update documentation for new API
- [ ] Add migration guide
- [ ] Update examples
- [ ] Comprehensive testing

## Immediate Next Steps

### Option A: Gradual Migration (Recommended)
1. **Keep current stable version** (Leptos 0.6)
2. **Create migration branch** for 0.8+ work
3. **Incremental updates** with proper testing
4. **Release v0.2.0** with 0.8+ support

### Option B: Complete Rewrite
1. **Rewrite core components** for 0.8+ compatibility
2. **Simplify API** to reduce complexity
3. **Focus on essential features** first
4. **Gradual feature addition**

### Option C: Dual Version Support
1. **Maintain 0.6 compatibility** in main branch
2. **Create 0.8+ branch** for new development
3. **Feature flags** for version-specific code
4. **Gradual migration** of users

## Recommended Approach

### For Immediate Release
- **Keep Leptos 0.6** for v0.1.0 stability
- **Document migration path** for users
- **Create roadmap** for 0.8+ support

### For Long-term Success
- **Start migration branch** immediately
- **Incremental development** with proper testing
- **Community feedback** on API changes
- **Comprehensive documentation**

## Technical Details

### Required Changes
```rust
// Old API (Leptos 0.6)
let (state, set_state) = create_signal(initial);

// New API (Leptos 0.8+)
let (state, set_state) = create_rw_signal(initial);
```

### Trait Bound Updates
```rust
// Old bounds
impl<M: StateMachine> MachineHandle<M> {
    // ...
}

// New bounds (Leptos 0.8+)
impl<M: StateMachine> MachineHandle<M> 
where
    M::State: Send + Sync + 'static,
    M::Context: Send + Sync + 'static,
    M::Event: Send + Sync + 'static,
{
    // ...
}
```

### Signal Storage Changes
```rust
// Old storage
use leptos::*;

// New storage
use leptos::prelude::*;
```

## Timeline

### Week 1-2: Foundation
- [ ] Set up migration branch
- [ ] Fix core trait bounds
- [ ] Update basic signal usage

### Week 3-4: Core Features
- [ ] Fix state machine implementation
- [ ] Update store implementations
- [ ] Fix hook implementations

### Week 5-6: Advanced Features
- [ ] Fix async stores
- [ ] Update DevTools
- [ ] Fix examples

### Week 7-8: Testing & Documentation
- [ ] Comprehensive testing
- [ ] Update documentation
- [ ] Create migration guide

## Success Metrics

### Technical
- [ ] 100% compilation success
- [ ] All tests passing
- [ ] Examples working
- [ ] Performance maintained

### User Experience
- [ ] Clear migration path
- [ ] Backward compatibility (where possible)
- [ ] Comprehensive documentation
- [ ] Community feedback

## Risk Mitigation

### Technical Risks
- **API instability**: Use feature flags
- **Breaking changes**: Comprehensive testing
- **Performance regressions**: Benchmarking

### User Risks
- **Migration complexity**: Clear documentation
- **Breaking changes**: Gradual rollout
- **Community confusion**: Clear communication

## Conclusion

The migration to Leptos 0.8+ is **technically feasible** but **complex**. The recommended approach is:

1. **Maintain current stability** with Leptos 0.6
2. **Start migration work** in parallel
3. **Gradual rollout** with proper testing
4. **Community involvement** throughout the process

This ensures we maintain a stable, working library while building toward the latest Leptos compatibility.

---

**Next Action**: Create migration branch and begin Phase 2 work while maintaining current stable release.
