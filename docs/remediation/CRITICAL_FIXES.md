# 🚨 Critical Fixes - Immediate Action Required

## Overview
These are compilation-blocking issues that must be fixed before any development can proceed.

## 1. Serde Dependency Missing

**Issue:** Code doesn't compile due to missing serde dependency in async_store.rs
```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `serde`
--> leptos-state/src/store/async_store.rs:185:14
```

**Fix:** Update leptos-state/Cargo.toml to include serde in default features
```toml
[features]
default = ["serde", "serde_json"]  # Add serde to default
```

**Alternative Fix:** Make async_store conditional on serialization feature
```rust
#[cfg(feature = "serialization")]
mod async_store;
```

**Priority:** 🔴 CRITICAL - Blocks all compilation
**Time Estimate:** 5 minutes
**Files:** `leptos-state/Cargo.toml`, `leptos-state/src/store/async_store.rs`

## 2. Legacy Test Compilation Failures

**Issue:** Test files reference non-existent APIs and imports
- Tests use outdated function signatures
- Import paths don't match current module structure
- Mock implementations reference removed traits

**Fix:** Update test files to match current API
```bash
# Run to identify all failing tests
cargo test --workspace 2>&1 | grep "error\|failed"
```

**Priority:** 🔴 CRITICAL - Prevents testing
**Time Estimate:** 2 hours
**Files:** `tests/rust/`, examples test files

## 3. README API Mismatches

**Issue:** README examples use non-existent APIs
- `use_store::<CounterStore>()` - function doesn't exist
- `MachineBuilder::new()` - different signature than actual
- `use_machine(machine)` - incorrect parameter types

**Immediate Fix:** Add disclaimer to README
```markdown
⚠️ **NOTICE**: This library is under active development. 
API examples in this README may not match current implementation.
See examples/ directory for working code.
```

**Priority:** 🟡 HIGH - Misleads users
**Time Estimate:** 30 minutes
**Files:** `README.md`

## 4. Example Compilation Issues

**Issue:** Examples don't compile due to API changes
```bash
# Test all examples
cargo check --examples --workspace
```

**Fix Priority Order:**
1. `counter` example - simplest, fix first
2. `todo-app` - most referenced
3. `traffic-light` - used in README
4. Other examples

**Priority:** 🟡 HIGH - Blocks development workflow
**Time Estimate:** 4 hours
**Files:** `examples/*/`

## 5. Feature Flag Dependencies

**Issue:** Optional features cause compilation failures when disabled
- async_store.rs uses serde without feature guard
- visualization.rs uses serde without proper conditionals

**Fix:** Add proper feature guards
```rust
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "serialization"))]
compile_error!("async_store requires 'serialization' feature");
```

**Priority:** 🟡 HIGH - Breaks optional features
**Time Estimate:** 1 hour
**Files:** All modules using optional dependencies

## Implementation Order

### Phase 1: Immediate (Today)
1. ✅ Fix serde dependency in Cargo.toml
2. ✅ Add README disclaimer
3. ✅ Create minimal working counter example

### Phase 2: Critical (This Week)
1. 🔄 Fix all compilation errors
2. 🔄 Update test files to compile
3. 🔄 Fix at least 3 core examples

### Phase 3: Stabilization (Next Week)
1. 📋 Fix all examples
2. 📋 Remove all TODO/unimplemented! from critical paths
3. 📋 Update README with working examples

## Success Criteria

- [ ] `cargo check --workspace` passes without errors
- [ ] `cargo test --workspace` compiles (tests may fail, but must compile)
- [ ] At least 3 examples compile and run
- [ ] README has accurate disclaimer or fixed examples

## Quick Validation Commands

```bash
# Must pass before any other work
cargo check --workspace

# Must compile (can have test failures)
cargo test --workspace --no-run

# Must work for core examples
cargo run --example counter
cargo run --example todo-app
cargo run --example traffic-light
```

## Emergency Contacts

If these fixes reveal deeper architectural issues:
1. Consider feature-flagging entire modules temporarily
2. Create minimal stub implementations for missing functions
3. Document all breaking changes for future API alignment

**Next Steps:** Once compilation is fixed, proceed to API_ALIGNMENT.md
