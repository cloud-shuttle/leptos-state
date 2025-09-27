# 🔄 API Alignment - README & Examples Synchronization

## Overview
Fix mismatches between documented APIs in README and actual implementation.

## Current API Mismatches

### 1. Store API Inconsistencies

**README Claims:**
```rust
use leptos_state::{create_store, use_store};

let (store, actions) = use_store::<CounterStore>();
```

**Actual Implementation:**
```rust
// use_store function doesn't exist in current codebase
// Store creation uses different pattern
```

**Fix Required:**
- Implement `use_store<T>()` function or update README
- Create store creation utilities that match documented API
- Ensure return types match (store, actions) tuple

### 2. Machine Builder API Mismatch

**README Claims:**
```rust
let machine = MachineBuilder::new()
    .state("red")
        .on(TrafficLightEvent::Next, "green")
    .state("green")
        .on(TrafficLightEvent::Next, "yellow")
    .state("yellow")
        .on(TrafficLightEvent::Next, "red")
    .initial("red")
    .build();

let (state, send) = use_machine(machine);
```

**Actual Implementation Issues:**
- `MachineBuilder::new()` may have different signature
- Fluent API chain might not work as shown
- `use_machine()` function needs verification
- Event handling pattern may differ

### 3. Middleware API Documentation

**README Claims:**
```rust
let store = create_store::<MyStore>()
    .with_middleware(
        MiddlewareChain::new()
            .add(LoggerMiddleware::new())
            .add(ValidationMiddleware::new())
    );
```

**Issues:**
- `create_store` function pattern not found
- `with_middleware` method may not exist
- Middleware types need verification

### 4. Persistence API Mismatch

**README Claims:**
```rust
let machine = MachineBuilder::new()
    .state("idle")
    .build_with_persistence(PersistenceConfig {
        enabled: true,
        storage_key: "my_machine".to_string(),
        auto_save: true,
        ..Default::default()
    });
```

**Issues:**
- `build_with_persistence` method needs verification
- `PersistenceConfig` struct alignment
- Default implementation completeness

## Solution Strategies

### Strategy 1: Update Implementation to Match README
**Pros:** Users get the API they expect
**Cons:** May require significant implementation work

**Priority APIs to Implement:**
1. `use_store<T>()` - Core store hook
2. `create_store<T>()` - Store creation utility
3. `use_machine(machine)` - Machine hook
4. Fluent builder APIs as documented

### Strategy 2: Update README to Match Implementation
**Pros:** Faster to implement, accurate documentation
**Cons:** May disappoint users expecting documented API

**Required Changes:**
1. Audit current public API
2. Rewrite all examples using actual functions
3. Update feature documentation to match capabilities

### Strategy 3: Hybrid Approach (Recommended)
**Core APIs:** Implement to match README (high user impact)
**Advanced APIs:** Update README to match implementation

## Implementation Priority

### Phase 1: Core API Alignment (Week 1)
```rust
// Must implement these functions to match README:
pub fn use_store<T>() -> (ReadSignal<T>, WriteSignal<T>) { }
pub fn create_store<T>() -> Store<T> { }
pub fn use_machine<M>(machine: M) -> (MachineState, SendEvent) { }
```

### Phase 2: Builder API Fix (Week 2)
- Ensure `MachineBuilder::new()` works as documented
- Fix fluent API chains (`.state().on().state()`)
- Verify `build()` method return types

### Phase 3: Advanced Feature Alignment (Week 3)
- Middleware system API verification
- Persistence configuration matching
- Code generation API updates

## Specific File Updates Required

### README.md Updates
```markdown
<!-- Add this section at the top -->
## ⚠️ API Status
- ✅ Core store APIs - Working
- ✅ Basic state machine - Working  
- 🚧 Middleware system - In development
- 🚧 Persistence - Basic implementation
- 📋 DevTools integration - Planned
```

### New API Implementation Files
```
leptos-state/src/
├── hooks/
│   ├── mod.rs           # use_store, use_machine
│   ├── store_hooks.rs   # Store-specific hooks
│   └── machine_hooks.rs # Machine-specific hooks
├── builders/
│   ├── mod.rs           # Re-export builders
│   └── store_builder.rs # create_store utilities
```

### Example Updates
1. `examples/counter/` - Update to use documented API
2. `examples/traffic-light/` - Verify against README
3. `examples/todo-app/` - Use create_store pattern

## API Validation Tests

Create integration tests that match README exactly:

```rust
#[test]
fn readme_counter_example_works() {
    // Copy exact code from README
    use leptos_state::{create_store, use_store};
    
    #[derive(Clone, Debug)]
    struct CounterStore {
        count: i32,
        name: String,
    }
    
    // This must compile and work
    let (store, actions) = use_store::<CounterStore>();
}
```

## Documentation Generation

After API alignment:
1. Generate API docs: `cargo doc --no-deps --open`
2. Validate all README examples compile
3. Create API compatibility matrix
4. Add API stability guarantees

## Rollout Plan

### Week 1: Critical Path
- [ ] Audit current public API surface
- [ ] Implement `use_store` and `use_machine` hooks
- [ ] Fix counter example to match README

### Week 2: Builder APIs
- [ ] Fix MachineBuilder fluent API
- [ ] Implement create_store utilities
- [ ] Update traffic-light example

### Week 3: Advanced Features
- [ ] Align middleware APIs
- [ ] Fix persistence configuration
- [ ] Update all remaining examples

### Week 4: Documentation Polish
- [ ] Comprehensive README update
- [ ] API reference completion
- [ ] Migration guide for API changes

## Success Metrics

- [ ] All README examples compile without modification
- [ ] API documentation matches README claims
- [ ] Examples run successfully with README code
- [ ] User API surface is intuitive and consistent

**Next Steps:** After API alignment, proceed to FILE_REFACTOR.md
