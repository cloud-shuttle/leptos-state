# Incremental Feature Restoration Roadmap

## Overview

This roadmap outlines the phased restoration of advanced features from the original `leptos-state` library back into `leptos-state-minimal`. Each phase adds functionality incrementally while maintaining clean compilation and avoiding the cascade of trait bound errors that plagued the original implementation.

## Current State (Phase 0)
✅ **leptos-state-minimal** with core functionality:
- Minimal trait bounds (`Send + Sync + Clone + 'static`)
- Reactive stores with RwSignal
- Basic state machines with transitions
- Leptos hooks integration
- WASM compatibility

## Phase 1: Enhanced Trait Bounds (Low Risk)

### 1.1 Add Debug Support
**Goal**: Enable development debugging and logging
**Files**: `src/traits.rs`, `src/error.rs`
**Changes**:
```rust
// Current: Send + Sync + Clone + 'static'
// Add: Debug
pub trait State: Send + Sync + Clone + Debug + 'static {}
pub trait Event: Send + Sync + Clone + Debug + 'static {}
```

**Testing**: All existing demos still work + debug output in console
**Risk Level**: 🟢 Low - Debug is widely supported

### 1.2 Add Default Support
**Goal**: Enable default state initialization
**Files**: `src/traits.rs`, `src/store.rs`
**Changes**:
```rust
pub trait State: Send + Sync + Clone + Debug + Default + 'static {}
```

**Testing**: Default state initialization works
**Risk Level**: 🟢 Low - Default is fundamental

### 1.3 Add Eq + PartialEq Support
**Goal**: Enable state comparison for reactivity optimization
**Files**: `src/traits.rs`, `src/store.rs`
**Changes**:
```rust
pub trait State: Send + Sync + Clone + Debug + Default + Eq + PartialEq + 'static {}
pub trait Event: Send + Sync + Clone + Debug + Eq + PartialEq + 'static {}
```

**Testing**: State change detection works correctly
**Risk Level**: 🟡 Medium - Eq bounds can be restrictive for complex types

## Phase 2: State Machine Enhancements (Medium Risk)

### 2.1 Add Entry/Exit Actions with Context
**Goal**: Restore full state machine lifecycle management
**Files**: `src/machine.rs`, `src/hooks.rs`
**Changes**:
- Add context tracking to Machine struct
- Implement entry/exit action callbacks
- Update MachineActions to handle context

**Testing**: Traffic light demo shows entry/exit logging
**Risk Level**: 🟡 Medium - Context management complexity

### 2.2 Add Guard Conditions
**Goal**: Enable conditional transitions
**Files**: `src/machine.rs`
**Changes**:
- Add guard function type: `Fn(&S, &E) -> bool`
- Update Transition struct to include guards
- Implement guard evaluation in `send()` method

**Testing**: State machine with conditional transitions
**Risk Level**: 🟡 Medium - Guard logic complexity

### 2.3 Add Action System
**Goal**: Enable side effects during transitions
**Files**: `src/machine.rs`
**Changes**:
- Add action function type: `Fn(&mut S, &E)`
- Update Transition struct to include actions
- Implement action execution in transition logic

**Testing**: State machine with transition actions
**Risk Level**: 🟡 Medium - Mutable context handling

## Phase 3: Persistence Layer (High Risk)

### 3.1 Add Basic Serialization Support
**Goal**: Enable state serialization
**Files**: `src/traits.rs`, `src/store.rs`
**Changes**:
- Add serde features to Cargo.toml
- Update traits to include Serialize/Deserialize (optional)
- Add serialization methods to Store

**Testing**: Store can serialize/deserialize state
**Risk Level**: 🟡 Medium - Serde bounds can be restrictive

### 3.2 Add Local Storage Backend
**Goal**: Browser localStorage persistence
**Files**: `src/persistence.rs` (new file)
**Changes**:
- Create persistence trait and localStorage implementation
- Add persistence methods to Store
- Implement automatic save/load

**Testing**: State persists across browser refreshes
**Risk Level**: 🟡 Medium - Browser API integration

### 3.3 Add IndexedDB Backend
**Goal**: Advanced browser storage
**Files**: `src/persistence/indexeddb.rs`
**Changes**:
- Implement IndexedDB storage backend
- Add migration support
- Implement batch operations

**Testing**: Large state objects persist correctly
**Risk Level**: 🔴 High - Complex async browser APIs

## Phase 4: Middleware System (High Risk)

### 4.1 Add Basic Middleware Trait
**Goal**: Extensible middleware architecture
**Files**: `src/middleware.rs` (new file)
**Changes**:
- Define middleware trait: `Fn(&mut Store<S>, Action) -> Result<(), Error>`
- Add middleware registration to Store
- Implement middleware execution pipeline

**Testing**: Logging middleware works
**Risk Level**: 🔴 High - Complex type system interactions

### 4.2 Add Logging Middleware
**Goal**: Automatic state change logging
**Files**: `src/middleware/logging.rs`
**Changes**:
- Implement logging middleware
- Add configurable log levels
- Include state diff tracking

**Testing**: State changes are logged to console
**Risk Level**: 🟡 Medium - Straightforward implementation

### 4.3 Add Validation Middleware
**Goal**: State validation and constraints
**Files**: `src/middleware/validation.rs`
**Changes**:
- Implement validation middleware
- Add constraint definitions
- Implement validation error handling

**Testing**: Invalid state changes are prevented
**Risk Level**: 🟡 Medium - Validation logic complexity

## Phase 5: Development Tools (Medium Risk)

### 5.1 Add DevTools Detection
**Goal**: Detect when running in development
**Files**: `src/devtools.rs` (new file)
**Changes**:
- Add development mode detection
- Create DevTools message format
- Implement browser console integration

**Testing**: DevTools messages appear in browser console
**Risk Level**: 🟢 Low - Simple feature detection

### 5.2 Add State Inspector
**Goal**: Runtime state inspection
**Files**: `src/devtools/inspector.rs`
**Changes**:
- Implement state serialization for DevTools
- Add state tree visualization
- Implement state history tracking

**Testing**: State can be inspected in browser DevTools
**Risk Level**: 🟡 Medium - Serialization complexity

### 5.3 Add Time Travel Debugging
**Goal**: State history navigation
**Files**: `src/devtools/timetravel.rs`
**Changes**:
- Implement state snapshot system
- Add undo/redo functionality
- Create DevTools integration

**Testing**: Can navigate through state history
**Risk Level**: 🔴 High - Complex state management

## Phase 6: Advanced Features (Very High Risk)

### 6.1 Add State Machine Visualization
**Goal**: Generate state diagrams
**Files**: `src/visualization.rs` (new file)
**Changes**:
- Implement DOT graph generation
- Add state machine introspection
- Create diagram rendering

**Testing**: State machine diagrams render correctly
**Risk Level**: 🔴 High - Complex graph algorithms

### 6.2 Add Testing Utilities
**Goal**: Comprehensive testing framework
**Files**: `src/testing.rs` (new file)
**Changes**:
- Add state machine testing DSL
- Implement property-based testing
- Create integration test utilities

**Testing**: Full test suite passes
**Risk Level**: 🟡 Medium - Testing framework complexity

### 6.3 Add Performance Monitoring
**Goal**: Runtime performance tracking
**Files**: `src/performance.rs` (new file)
**Changes**:
- Implement render time tracking
- Add memory usage monitoring
- Create performance reports

**Testing**: Performance metrics are collected
**Risk Level**: 🟡 Medium - Performance API integration

## Implementation Strategy

### 📋 **Per-Phase Checklist:**
- ✅ **Compile cleanly** - No trait bound errors
- ✅ **Pass existing tests** - All current functionality works
- ✅ **Add comprehensive tests** - New features are tested
- ✅ **Update documentation** - README and API docs updated
- ✅ **Working demo** - Feature demonstrated in browser
- ✅ **Performance check** - No significant regressions

### 🔄 **Rollback Strategy:**
- Each phase is a separate commit
- Failed phases can be reverted independently
- Core functionality always remains working
- Feature flags for experimental features

### 📊 **Success Metrics:**
- **Compilation time**: <30 seconds for full build
- **Bundle size**: <500KB for basic features
- **Test coverage**: >90% for added features
- **API compatibility**: 100% backward compatibility

## Risk Assessment

### 🟢 **Low Risk Phases:**
- Phase 1: Trait bound additions (Debug, Default)
- Phase 5.1: DevTools detection

### 🟡 **Medium Risk Phases:**
- Phase 1.3: Eq + PartialEq (comparison constraints)
- Phase 2: State machine enhancements
- Phase 3.1: Basic serialization
- Phase 4.2: Logging middleware
- Phase 5.2: State inspection

### 🔴 **High Risk Phases:**
- Phase 3.2+: Advanced persistence
- Phase 4.1+: Complex middleware
- Phase 5.3: Time travel debugging
- Phase 6: Advanced features

## Timeline Estimate

### **Month 1-2**: Foundation (Phases 1-2)
- Enhanced trait bounds
- Full state machine functionality
- **Milestone**: Feature-complete basic library

### **Month 3-4**: Persistence (Phase 3)
- Serialization support
- Storage backends
- **Milestone**: Data persistence working

### **Month 5-6**: Ecosystem (Phases 4-5)
- Middleware system
- Development tools
- **Milestone**: Developer-friendly library

### **Month 7-8**: Advanced Features (Phase 6)
- Visualization, testing, performance
- **Milestone**: Enterprise-ready library

## Alternative Approaches

### **Option B: Gradual Trait Bound Expansion**
Instead of adding all bounds at once, expand them incrementally per feature:
```rust
// Start with minimal bounds
pub trait State: Send + Sync + Clone + 'static {}

// Add bounds as needed per feature
pub trait State: Send + Sync + Clone + Debug + 'static {}  // For logging
pub trait State: Send + Sync + Clone + Debug + Default + 'static {}  // For initialization
```

### **Option C: Feature Gates**
Use Cargo features to enable/disable advanced functionality:
```toml
[features]
default = []
debug = ["dep:debug-trait"]
persistence = ["debug", "serde"]
middleware = ["debug", "async-trait"]
devtools = ["debug", "serde", "wasm-bindgen"]
```

## Conclusion

This roadmap provides a **safe, incremental path** to restore advanced functionality while maintaining the architectural improvements of `leptos-state-minimal`. Each phase builds upon the previous one, ensuring that the library remains **compilable and functional** at every step.

The focus is on **small, testable increments** that can be rolled back if issues arise, providing a robust path to a fully-featured state management library.
