# Main leptos-state Library Emergency Fixes

## Phase 1: Emergency Compilation Fixes

**Goal**: Make the main leptos-state library compile with minimal functionality
**Priority**: Critical
**Timeline**: 1-2 days

## Step 1: Add Missing Dependencies

### Add libc crate
```toml
[dependencies]
libc = "0.2"
```

### Add missing imports
- Fix `use yansi::paint::Paint` imports
- Add missing `serde` derive imports
- Fix symbol resolution issues

## Step 2: Fix Critical Trait Bounds

### Core Machine Trait Bounds
**File**: `leptos-state/src/machine/core/core.rs`

**Before (Broken)**:
```rust
pub struct Machine<
    C: Send + Sync + Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static,
> { /* fields */ }
```

**After (Working)**:
```rust
pub struct Machine<
    C: Send + Sync + Clone + std::fmt::Debug + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
> { /* fields */ }
```

### Remove Default Requirements
**File**: `leptos-state/src/machine/core/core.rs`
- Remove `Default` bound from context type `C`
- Remove `Eq + Hash` bounds from event type `E`
- Update all method implementations accordingly

## Step 3: Fix Dyn Compatibility Issues

### Store Trait Refactor
**File**: `leptos-state/src/store/store_core.rs`

**Before (Broken)**:
```rust
pub trait Store: Send + Sync + 'static {
    type State;
    fn update<F>(&self, f: F)
    where
        F: FnOnce(Self::State) -> Self::State + Send + Sync + 'static;
}
```

**After (Working)**:
```rust
pub trait Store: Send + Sync + 'static {
    type State;
    fn update(&self, updater: Box<dyn FnOnce(Self::State) -> Self::State + Send + 'static>);
    fn get(&self) -> Self::State;
    fn set(&self, state: Self::State);
}
```

### Remove Dyn Store Usage
**File**: `leptos-state/src/store/store_core.rs`
```rust
// Before (Broken)
#[derive(Clone, Debug)]
pub struct StoreWrapper<T: Send + Sync + 'static> {
    pub store: std::rc::Rc<dyn Store<State = T>>,
}

// After (Working)
#[derive(Clone, Debug)]
pub struct StoreWrapper<T: Send + Sync + 'static> {
    pub store: std::rc::Rc<ConcreteStore<T>>,
}
```

## Step 4: Fix Lifetime Issues

### Add Static Bounds
**File**: `leptos-state/src/machine/action_core.rs`
```rust
// Add 'static bounds to generic parameters
impl<C: Send + Sync + std::fmt::Debug + 'static, E: Send + Sync + std::fmt::Debug + PartialEq + 'static, T: Send + Sync + 'static, F> Action<C, E>
```

**File**: `leptos-state/src/machine/guard_composite.rs`
```rust
// Add 'static bounds
impl<C: 'static, E: 'static> GuardEvaluator<C, E> for CompositeGuard<C, E>
```

## Step 5: Implement Missing Methods

### Add Constructor Methods
**File**: `leptos-state/src/machine/visualization/monitor/state_info.rs`
```rust
impl<C, E> CollectionStats {
    pub fn new() -> Self {
        Self::default()
    }
}
```

**File**: `leptos-state/src/machine/visualization/monitor/health.rs`
```rust
impl<C, E> HealthChecker<C, E> {
    pub fn new() -> Self {
        Self {
            machine: None,
            last_result: None,
            config: HealthCheckConfig::default(),
        }
    }
}
```

### Add Missing Trait Methods
**File**: `leptos-state/src/machine/visualization/monitor/health.rs`
```rust
impl HealthStatus {
    pub fn is_error(&self) -> bool {
        matches!(self, HealthStatus::Error(_))
    }
}
```

## Step 6: Fix Borrow Checker Issues

### Fix Moved Values
**File**: `leptos-state/src/machine/doc_builder.rs`
```rust
pub fn build_and_save(self, path: Option<&std::path::Path>) -> StateResult<GeneratedDocument> {
    let config = self.config.clone(); // Clone config before moving self
    let document = self.build()?;
    let file_path = path.map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::Path::new(&config.output_dir).join(document.full_filename()));
    // ... rest of implementation
}
```

**File**: `leptos-state/src/machine/persistence/ext/monitoring.rs`
```rust
pub fn generate_report(&self) -> PersistenceReport {
    let stats = self.stats();
    PersistenceReport {
        stats: stats.clone(), // Clone stats before moving
        recommendations: self.generate_recommendations(&stats),
    }
}
```

## Step 7: Fix Type Inference Issues

### Add Explicit Type Annotations
**File**: `leptos-state/src/machine/visualization/monitor/health.rs`
```rust
pub fn check_health(&mut self) -> HealthCheckResult {
    // ... implementation
    if let Some(machine) = &self.machine {
        return self.perform_machine_check::<C, E>(machine);
    }
    // ... rest
}
```

## Step 8: Fix Import and Symbol Resolution

### Fix Yansi Paint Imports
Replace incorrect `use yansi::paint::Paint` with proper imports or remove unused imports.

### Fix Serde Derives
Add missing `#[derive(Serialize, Deserialize)]` where needed.

## Step 9: Remove Recursion Issues

### Fix Infinite Recursion
**File**: `leptos-state/src/machine/test_data_generation.rs`
```rust
impl<C, E> TestDataGenerator<C, E> {
    fn generate_context_for_state(&self, _state: &str) -> C {
        // TODO: Implement actual generation logic
        unimplemented!("Test data generation not implemented")
    }

    fn generate_event_for_transition(&self, _from: &str, _to: &str) -> E {
        // TODO: Implement actual generation logic
        unimplemented!("Test data generation not implemented")
    }
}
```

## Step 10: Validation

### Test Compilation
```bash
cd leptos-state
cargo check --quiet
cargo check --features serde --quiet
cargo check --features "serde,persist" --quiet
```

### Expected Outcome
- Zero compilation errors
- Library compiles with basic functionality
- Core traits and structs available
- Foundation for Phase 2 established

## Success Criteria

### Compilation ✅
- `cargo check` passes without errors
- All core modules compile
- No panic-inducing code

### API Availability ✅
- Core `Machine` and `Store` types available
- Basic trait implementations working
- Constructor methods functional

### Foundation for Migration ✅
- Trait bounds simplified for usability
- Dyn compatibility issues resolved
- Lifetime requirements relaxed

## Next Steps

After Phase 1 completion:
1. **Phase 2**: Implement core functionality (basic state machines and stores)
2. **Phase 3**: Migrate advanced features from leptos-state-minimal
3. **Phase 4**: Restore ecosystem integrations

## Risk Mitigation

- **Backup strategy**: Keep leptos-state-minimal as working reference
- **Incremental testing**: Validate each fix doesn't break working code
- **Documentation**: Track all changes for user migration guide

## Timeline Estimate

- **Step 1-3**: 4-6 hours (dependency and trait bound fixes)
- **Step 4-6**: 6-8 hours (lifetime and borrow checker fixes)
- **Step 7-9**: 4-6 hours (implementation and recursion fixes)
- **Step 10**: 2-4 hours (validation and testing)

**Total: 16-24 hours for Phase 1 completion**
