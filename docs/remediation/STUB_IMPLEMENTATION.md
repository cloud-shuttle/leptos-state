# 🚧 Stub Implementation - Complete Missing Features

## Overview
Major features are unimplemented stubs with TODO comments. Convert these to working implementations.

## Critical Stubs Found

### 1. Async Store Create Resource (CRITICAL)
**Location:** `store/async_store.rs:193`
**Issue:** `todo!("create_resource API needs to be updated for Leptos 0.7")`
**Impact:** 🔴 BLOCKS async state management

**Implementation Plan:**
```rust
// Current stub
pub fn create_async_store<T, F, Fut>() -> AsyncStore<T> 
where 
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = T> + 'static,
{
    todo!("create_resource API needs to be updated for Leptos 0.7")
}

// Target implementation
pub fn create_async_store<T, F, Fut>(fetcher: F) -> AsyncStore<T> 
where 
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = T> + 'static,
{
    let resource = create_resource(|| (), move |_| fetcher());
    AsyncStore::new(resource)
}
```

**Dependencies:** Leptos 0.7/0.8 create_resource API research
**Priority:** 🔴 CRITICAL
**Time Estimate:** 4 hours

### 2. LocalStorage Implementation (HIGH)
**Locations:** 
- `store/store.rs:62` - "TODO: Implement localStorage functionality"
- `store/store.rs:72` - "TODO: Implement localStorage functionality"
- `machine/persistence.rs:670-686` - 4 TODO comments about localStorage

**Impact:** 🟡 BLOCKS persistence features

**Implementation Plan:**
```rust
// Current stubs
fn save_to_local_storage(&self) {
    // TODO: Implement localStorage functionality
}

// Target implementation
fn save_to_local_storage(&self) -> Result<(), PersistenceError> {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        if let Some(storage) = window()?.local_storage()? {
            let serialized = serde_json::to_string(self)?;
            storage.set_item(&self.storage_key, &serialized)?;
            Ok(())
        } else {
            Err(PersistenceError::StorageNotAvailable)
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Use file-based storage for non-WASM targets
        std::fs::write(&self.storage_path, serde_json::to_string(self)?)?;
        Ok(())
    }
}
```

**Priority:** 🟡 HIGH
**Time Estimate:** 6 hours

### 3. Machine Clone Guards/Actions Bug (CRITICAL)
**Location:** `machine/visualization.rs:719` and `visualization.rs:738`
**Issue:** `TODO: This method is temporarily disabled because Machine doesn't implement Clone`
**Impact:** 🔴 BREAKS visualization and time travel debugging

**Root Cause Analysis:**
```rust
// Machine struct contains non-Clone types
pub struct Machine {
    guards: HashMap<String, Box<dyn Fn() -> bool>>,  // Function pointers not Clone
    actions: HashMap<String, Box<dyn Fn()>>,         // Function pointers not Clone
    // ... other fields
}
```

**Implementation Plan:**
```rust
// Option 1: Custom Clone implementation
impl Clone for Machine {
    fn clone(&self) -> Self {
        Self {
            // Clone data fields normally
            id: self.id.clone(),
            states: self.states.clone(),
            current_state: self.current_state.clone(),
            
            // For function pointers, create empty collections
            // and require re-registration of guards/actions
            guards: HashMap::new(),
            actions: HashMap::new(),
        }
    }
}

// Option 2: Separate cloneable data from function pointers
pub struct Machine {
    data: MachineData,           // Clone-able state data
    runtime: MachineRuntime,     // Non-cloneable function pointers
}

#[derive(Clone)]
pub struct MachineData {
    id: String,
    states: HashMap<String, State>,
    current_state: String,
}
```

**Priority:** 🔴 CRITICAL
**Time Estimate:** 8 hours

### 4. Effects Implementation (MEDIUM)
**Locations:**
- `compat/effects.rs:74` - "TODO: Implement proper debouncing"
- `compat/effects.rs:84` - "TODO: Implement proper throttling"

**Impact:** 🟡 LIMITS utility function effectiveness

**Implementation Plan:**
```rust
// Current stubs
pub fn use_debounced_effect<F>(callback: F, delay: u64) 
where F: Fn() + 'static 
{
    // TODO: Implement proper debouncing
}

// Target implementation
pub fn use_debounced_effect<F>(callback: F, delay: u64) 
where F: Fn() + 'static 
{
    let (trigger, set_trigger) = create_signal(0);
    let callback = Rc::new(callback);
    
    create_effect(move |_| {
        trigger.get(); // Subscribe to trigger signal
        
        let callback = callback.clone();
        let handle = set_timeout(
            move || callback(),
            Duration::from_millis(delay)
        );
        
        on_cleanup(move || clear_timeout(handle));
    });
    
    move || set_trigger.update(|t| *t += 1)
}
```

**Priority:** 🟡 MEDIUM
**Time Estimate:** 4 hours

### 5. Machine State Validation (MEDIUM)
**Location:** `machine/machine.rs:1131`
**Issue:** `TODO: Implement based on available transitions`

**Impact:** 🟡 LIMITS state machine safety

**Implementation Plan:**
```rust
// Current stub
fn validate_transition(&self, to_state: &str) -> bool {
    // TODO: Implement based on available transitions
    true
}

// Target implementation
fn validate_transition(&self, from_state: &str, to_state: &str) -> Result<(), TransitionError> {
    // Check if target state exists
    if !self.states.contains_key(to_state) {
        return Err(TransitionError::InvalidState(to_state.to_string()));
    }
    
    // Check if transition is allowed from current state
    if let Some(state) = self.states.get(from_state) {
        if state.transitions.contains_key(to_state) {
            Ok(())
        } else {
            Err(TransitionError::TransitionNotAllowed {
                from: from_state.to_string(),
                to: to_state.to_string(),
            })
        }
    } else {
        Err(TransitionError::InvalidState(from_state.to_string()))
    }
}
```

**Priority:** 🟡 MEDIUM
**Time Estimate:** 3 hours

## Implementation Priority Matrix

### Critical Path (Week 1)
1. **Async Store Resource API** - BLOCKS async functionality
2. **Machine Clone Implementation** - BLOCKS visualization
3. **LocalStorage Basic Implementation** - BLOCKS persistence

### Core Features (Week 2)
1. **Effects Implementation** - Improves utility
2. **Machine State Validation** - Improves safety
3. **Persistence Error Handling** - Improves robustness

### Extended Features (Week 3)
1. **Advanced persistence backends**
2. **Comprehensive validation**
3. **Performance optimizations**

## Implementation Strategy

### 1. Research Phase (Day 1)
- Study Leptos 0.7/0.8 create_resource API changes
- Review web-sys localStorage API patterns
- Analyze Machine Clone architectural options

### 2. Architecture Decisions (Day 2)
- Choose Machine Clone strategy (custom impl vs. separation)
- Design persistence error hierarchy
- Plan effects implementation approach

### 3. Implementation Phase (Days 3-10)
- Implement features in priority order
- Add comprehensive tests for each feature
- Update documentation and examples

### 4. Integration Phase (Days 11-14)
- Integration testing with real examples
- Performance benchmarking
- API stability validation

## Feature Implementation Templates

### Async Store Template
```rust
#[cfg(feature = "async")]
pub fn create_async_store<T, F, Fut>(fetcher: F) -> AsyncStore<T> 
where 
    T: Clone + 'static,
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = T> + 'static,
{
    // Research current Leptos resource API
    // Implement based on current patterns
}
```

### Persistence Template
```rust
pub trait PersistenceBackend {
    fn save(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError>;
    fn load(&self, key: &str) -> Result<Vec<u8>, PersistenceError>;
    fn delete(&self, key: &str) -> Result<(), PersistenceError>;
}

pub struct LocalStorageBackend;
pub struct IndexedDBBackend;
pub struct FileSystemBackend;
```

### Error Handling Template
```rust
#[derive(Debug, thiserror::Error)]
pub enum ImplementationError {
    #[error("Feature not yet implemented: {feature}")]
    NotImplemented { feature: String },
    
    #[error("Async operation failed: {source}")]
    AsyncError { #[from] source: JsValue },
    
    #[error("Persistence error: {source}")]
    PersistenceError { #[from] source: PersistenceError },
}
```

## Testing Requirements

### For Each Implemented Feature
1. **Unit Tests:** Test individual functions
2. **Integration Tests:** Test with other components
3. **WASM Tests:** Test browser-specific functionality
4. **Error Tests:** Test error conditions

### Test Coverage Goals
- [ ] 90%+ line coverage for new implementations
- [ ] All error paths tested
- [ ] Browser compatibility verified
- [ ] Performance benchmarks established

## API Stability Guarantees

### Implemented Features Must Provide
1. **Backward Compatibility:** No breaking changes to existing API
2. **Error Handling:** Comprehensive error types and messages
3. **Documentation:** Inline docs and examples
4. **Testing:** Full test coverage

### Versioning Strategy
- Patch version for bug fixes and stub implementations
- Minor version for new API additions
- Major version for breaking changes

## Success Metrics

- [ ] Zero TODO/unimplemented! in critical paths
- [ ] All advertised features work as documented
- [ ] Comprehensive test coverage (>90%)
- [ ] Performance benchmarks meet targets
- [ ] Examples demonstrate all implemented features

**Next Steps:** After stub implementation, proceed to TEST_COVERAGE.md
