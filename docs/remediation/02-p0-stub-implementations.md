# P0: Fix Stub Implementations (Data Loss Issues)

**Priority**: P0 (Production Blocker)  
**Timeline**: 1 week  
**Assignee**: TBD

## Problem Statement

Critical stub implementations that silently lose data in production:

1. **MemoryBackend**: `save/remove` are NO-OPs, nothing persists
2. **Clone Semantics**: `Transition` and `StateNode` clones drop guards/actions  
3. **API Generator**: Produces skeletal OpenAPI specs

## Impact Analysis

- **MemoryBackend**: Applications think data is persisted but it's lost on restart
- **Clone Issues**: Performance caching and visualization lose behavioral logic
- **API Specs**: Integration testing and documentation generation fail

## Solution Design

### Fix 1: MemoryBackend Implementation

**Current** (BROKEN):
```rust
impl PersistenceBackend for MemoryBackend {
    fn save(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError> {
        // TODO: simplified for testing
        Ok(())
    }
}
```

**Fixed**:
```rust
use parking_lot::RwLock;
use std::collections::HashMap;

pub struct MemoryBackend {
    storage: RwLock<HashMap<String, Vec<u8>>>,
}

impl PersistenceBackend for MemoryBackend {
    fn save(&self, key: &str, data: &[u8]) -> Result<(), PersistenceError> {
        self.storage.write().insert(key.to_string(), data.to_vec());
        Ok(())
    }
    
    fn load(&self, key: &str) -> Result<Option<Vec<u8>>, PersistenceError> {
        Ok(self.storage.read().get(key).cloned())
    }
    
    fn remove(&self, key: &str) -> Result<(), PersistenceError> {
        self.storage.write().remove(key);
        Ok(())
    }
}
```

### Fix 2: Clone Semantics

**Current** (BROKEN):
```rust
impl Clone for Transition {
    fn clone(&self) -> Self {
        Self {
            from: self.from.clone(),
            to: self.to.clone(),
            event: self.event.clone(),
            guards: vec![], // ⚠️ LOST!
            actions: vec![], // ⚠️ LOST!
        }
    }
}
```

**Options**:

A. **Forbid cloning with trait objects** (Recommended):
```rust
#[derive(Clone)]
pub struct Transition<G: Guard + Clone, A: Action + Clone> {
    guards: Vec<G>,
    actions: Vec<A>,
    // ...
}
```

B. **Deep clone with dyn_clone**:
```rust
use dyn_clone::DynClone;

pub trait Guard: DynClone + Send + Sync {
    fn evaluate(&self, context: &StateContext) -> bool;
}

dyn_clone::clone_trait_object!(Guard);
```

### Fix 3: API Generator

Move incomplete OpenAPI generation behind feature flag:
```rust
#[cfg(feature = "openapi")]
pub mod openapi {
    // Complete implementation or return Err
}

#[cfg(not(feature = "openapi"))]  
compile_error!("OpenAPI generation requires 'openapi' feature");
```

## Implementation Plan

### Day 1-2: MemoryBackend
- [ ] Add `parking_lot` dependency  
- [ ] Implement proper storage with RwLock<HashMap>
- [ ] Add unit tests for save/load/remove operations
- [ ] Test persistence round-trips

### Day 3-4: Clone Semantics  
- [ ] Analyze clone usage patterns in codebase
- [ ] Choose solution approach (A or B above)
- [ ] Implement new clone semantics
- [ ] Update dependent code and tests

### Day 5: API Generator
- [ ] Feature-gate incomplete implementations
- [ ] Add TODO tracking for full implementation
- [ ] Document API generation roadmap

## Testing Requirements

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    // Persistence round-trip property test
    quickcheck! {
        fn memory_backend_roundtrip(key: String, data: Vec<u8>) -> bool {
            let backend = MemoryBackend::new();
            backend.save(&key, &data).is_ok() &&
            backend.load(&key).unwrap() == Some(data)
        }
    }

    // Clone preserves behavior test
    #[test]
    fn clone_preserves_guards_and_actions() {
        let transition = create_test_transition_with_guards();
        let cloned = transition.clone();
        assert_eq!(transition.guards.len(), cloned.guards.len());
        assert_eq!(transition.actions.len(), cloned.actions.len());
    }
}
```

## Acceptance Criteria

- [ ] MemoryBackend actually persists data
- [ ] Clone operations preserve all behavioral logic
- [ ] Property tests verify round-trip consistency  
- [ ] No silent data loss in any persistence operation
- [ ] Incomplete features are clearly marked as unimplemented

## Dependencies

- `parking_lot` crate for RwLock
- `quickcheck` for property-based tests
- Decision on clone strategy (affects API design)

## Risks

- Clone strategy change may break existing API consumers
- MemoryBackend thread safety may need additional consideration
- Performance impact of deep cloning needs measurement
